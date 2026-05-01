//! This module contains methods for parsing an [ItemStruct] and converting
//! fields metadata into a usable format for the [crate::collection] macro.
//!
//! # Rust-SQL Type Mapping
//!
//! See the [Type Affinity in SQLite](https://sqlite.org/datatype3.html#type_affinity)
//!
//! | Rust Type | SQL Type |
//! |-----------|----------|
//! | `i32`, `i64` | `INTEGER` |
//! | `f32`, `f64` | `REAL` |
//! | `String` | `TEXT` |
//! | `bool` | `BOOLEAN` |
//! | `Uuid` | `TEXT` |
//! | `Vec<u8>` | `BLOB` |
//! | `Option<T>` | Same as `T`, but nullable |

use syn::{Field, Type};

pub(crate) struct FieldMeta {
    /// Whether this field is the primary key of the table. Only one field can
    /// be marked as the primary key.
    pub primary: bool,

    /// The name of the field in the database, it equals to the struct field
    /// name.
    pub name: String,

    /// The type of the field in Rust.
    pub ty: syn::Type,

    /// The SQL type of the field, which is derived from the Rust type. This is
    /// used in the table generator syntax.
    pub sql_type: String,

    /// The SQL type of the field, without `NOT NULL`.
    pub raw_sql_type: String,

    /// If the field originates from an `Option<T>`.
    pub optional: bool,
}

impl FieldMeta {
    /// Converts the field metadata into a SQL field definition, which is used
    /// in the table generator syntax.
    pub fn into_field_definition(&self) -> String {
        let mut definition = format!("{} {}", self.name, self.sql_type);
        if self.primary {
            definition.push_str(" PRIMARY KEY");
        }
        definition
    }
}

/// Converts a Rust type into an SQL type string. This is used for generating the
/// Returns: Type string and whether it's nullable (Option<T>).
fn rust_type_to_sql_type(ty: &syn::Type) -> (String, bool) {
    let mut nullable = false;
    let type_string = match ty {
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            match segment.ident.to_string().as_str() {
                "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "isize"
                | "usize" => "INTEGER".to_string(),
                "f32" | "f64" => "REAL".to_string(),
                "String" => "TEXT".to_string(),
                "bool" => "BOOLEAN".to_string(),
                "Uuid" => "TEXT".to_string(),
                "Option" if !segment.arguments.is_empty() => {
                    nullable = true;

                    let generic_ty = match &segment.arguments {
                        syn::PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                            match &args.args[0] {
                                syn::GenericArgument::Type(inner_ty) => inner_ty,
                                _ => panic!(
                                    "Unsupported generic argument in Option: {:?}",
                                    args.args[0]
                                ),
                            }
                        }
                        _ => panic!("Unsupported arguments in Option: {:?}", segment.arguments),
                    };

                    rust_type_to_sql_type(generic_ty).0
                }
                "Vec" if segment.arguments.is_empty() => "BLOB".to_string(),
                _ => panic!("Unsupported field type: {}", segment.ident),
            }
        }
        _ => panic!("Unsupported field type: {:?}", ty),
    };

    (type_string, nullable)
}

impl From<Field> for FieldMeta {
    fn from(field: Field) -> Self {
        let primary = field
            .attrs
            .iter()
            .any(|attr| attr.meta.path().is_ident("primary"));
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = field.ty.clone();
        let (sql_type, optional) = rust_type_to_sql_type(&field.ty);

        FieldMeta {
            primary,
            name,
            ty,
            sql_type: if optional { sql_type.clone() } else { format!("{} NOT NULL", sql_type.clone()) },
            raw_sql_type: sql_type,
            optional,
        }
    }
}
