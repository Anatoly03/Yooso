use crate::collection_fields::FieldMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Fields, Ident, ItemStruct, Meta, Token, parse::Parser, punctuated::Punctuated, spanned::Spanned,
};

meta_parser!(
    /// The metadata for the [collection] macro, which contains the database and
    /// table name.
    CollectionMeta {
        /// The database struct that this collection belongs to.
        db: syn::Path,
        /// The name of the SQL table that this collection corresponds to.
        table: syn::LitStr,
    }
);

/// The [collection] attribute marks a struct as a table definition. The
/// struct will be converted into an SQL-synced collection of rows.
pub fn collection(
    table_metadata: CollectionMeta,
    mut strucc: ItemStruct,
    unique_attributes: Vec<Meta>,
) -> TokenStream {
    let struct_name = &strucc.ident;
    let table_name = &table_metadata.table;
    // let db_struct_name = &table_metadata.db;

    let strucc_attr = strucc.attrs;
    strucc.attrs = Vec::new();

    // The name of the state struct is derived from the database struct by appending "State",
    // preserving Path.
    let db_state_struct_name = {
        let db_ident = &table_metadata.db.segments.last().unwrap().ident;
        let db_state_ident = format_ident!("{}State", db_ident);

        let mut db_state_path = table_metadata.db.clone();
        db_state_path.segments.pop();
        db_state_path.segments.push(db_state_ident.into());

        db_state_path
    };

    // Generate [FieldMeta] for each field and consume any field-level
    // attributes like #[primary].
    let field_metas = strucc
        .fields
        .iter()
        .map(|field| FieldMeta::from(field.clone()))
        .collect::<Vec<_>>();

    // List of all primary key field identifiers.
    let primary_key_idents = field_metas
        .iter()
        .filter(|field_meta| field_meta.primary)
        .map(|field_meta| format_ident!("{}", field_meta.name))
        .collect::<Vec<_>>();

    // List of all primary keys mapped to type.
    let primary_key_types = field_metas
        .iter()
        .filter(|field_meta| field_meta.primary)
        .map(|field_meta| field_meta.ty.clone())
        .collect::<Vec<_>>();

    // List of all primary key field identifiers and type reparation For
    // example Uuid does not support ToSql, so we convert it to String
    // for query parameters.
    let primary_key_idents_repaired = field_metas
        .iter()
        .filter(|field_meta| field_meta.primary)
        .map(|field_meta| {
            let ident = format_ident!("{}", field_meta.name);
            match &field_meta.ty {
                syn::Type::Path(type_path) if type_path.path.is_ident("Uuid") => {
                    quote! { #ident.to_string() }
                }
                _ => quote! { #ident },
            }
        })
        .collect::<Vec<_>>();

    // Consume "#[primary]" attributes on fields.
    for field in &mut strucc.fields {
        field
            .attrs
            .retain(|attr| !attr.meta.path().is_ident("primary"));
    }

    // Current Implementation: Panic and fail as "unsupported"
    // Future: Consume "#[default]" attributes on fields.
    for field in &mut strucc.fields {
        syn::Error::new(
            field.span(),
            "#[default] attribute is not supported yet and is reserved for future use",
        )
        .to_compile_error();
    }

    // Validate fields are 'named', not 'unnamed' or 'unit'.
    let fields = match &strucc.fields {
        Fields::Named(fields_named) => &fields_named.named,
        Fields::Unnamed(_) => panic!("#[collection] struct must have named fields"),
        Fields::Unit => panic!("#[collection] cannot be a unit struct"),
    };

    // Generate 'CREATE TABLE' syntax for the table generator.
    let sql_create_table = {
        // This generates a vector of field definitions.
        let mut sql_create_table_fields = field_metas
            .iter()
            .map(|field_meta| field_meta.into_field_definition())
            .collect::<Vec<_>>();

        // Append unique constraints from the #[unique] attributes on the struct.
        // It is a vector of vectors in the sense: We have multiple constraints,
        // and each constraint can involve multiple fields.
        let unique_constraints = unique_attributes
            .iter()
            .map(|meta| {
                match meta {
                    Meta::List(list) => Punctuated::<Ident, Token![,]>::parse_terminated
                        .parse2(list.tokens.clone())
                        .expect(
                            "#[unique] attribute must be in the form #[unique(field1, field2, ...)]",
                        ),
                    // Meta::NameValue(name_value) => name_value.value.to_token_stream(),
                    _ => panic!(
                        "#[unique] attribute must be in the form #[unique(field1, field2, ...)]]"
                    ),
                }
                    .iter()
                    .map(|ident| ident.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // For each unique constraint, generate a SQL UNIQUE clause and append it
        // to the field definitions.
        for unique_constraint in unique_constraints {
            let field_names = unique_constraint
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<_>>();

            sql_create_table_fields.push(format!("UNIQUE ({})", field_names.join(", ")));
        }

        format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name.value(),
            sql_create_table_fields.join(", ")
        )
    };

    // Generate 'SELECT * FROM ... WHERE' syntax for the view query.
    let sql_select_where = {
        let field_names = field_metas
            .iter()
            .map(|field_meta| field_meta.name.clone())
            .collect::<Vec<_>>();

        let primary_keys = primary_key_idents
            .clone()
            .into_iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<_>>();

        if primary_keys.is_empty() {
            panic!("#[collection] must have at least one #[primary] field");
        }

        let where_clause = primary_keys
            .iter()
            .enumerate()
            .map(|(i, key)| format!("{} = ${}", key, i + 1))
            .collect::<Vec<_>>()
            .join(" AND ");

        format!(
            "SELECT {} FROM {} WHERE {}",
            field_names.join(", "),
            table_name.value(),
            where_clause
        )
    };

    // Generate 'SELECT * FROM' syntax for the list generator.
    let sql_select_all = {
        let field_names = field_metas
            .iter()
            .map(|field_meta| field_meta.name.clone())
            .collect::<Vec<_>>();

        format!("SELECT {} FROM {}", field_names.join(", "), table_name.value())
    };
    

    // Generate 'INSERT INTO' syntax for the insert generator.
    let sql_insert_into = {
        let field_names = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string())
            .collect::<Vec<_>>();

        let indeces = (0..field_names.len())
            .map(|i| format!("${}", i + 1))
            .collect::<Vec<_>>();

        format!(
            "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
            table_name.value(),
            field_names.join(", "),
            indeces.join(", ")
        )
    };

    // Generate 'DELETE FROM ... WHERE' syntax for the delete generator.
    let sql_delete_from = {
        let primary_keys = primary_key_idents.clone();

        if primary_keys.is_empty() {
            panic!("#[collection] must have at least one #[primary] field");
        }

        let where_clause = primary_keys
            .iter()
            .enumerate()
            .map(|(i, key)| format!("{} = ${}", key, i + 1))
            .collect::<Vec<_>>()
            .join(" AND ");

        format!("DELETE FROM {} WHERE {}", table_name.value(), where_clause)
    };

    // Generate a list of field identifiers used in struct literals.
    let field_ids = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    // Generate a list of field value expressions used in INSERT parameters.
    let insert_values = fields
        .iter()
        .map(|field| {
            // The field type 'Uuid' is a special case because Uuid does
            // not implement 'rusqlite::ToSql' directly.
            let ident = field.ident.as_ref().unwrap();

            // For Uuid fields, convert to string before inserting.
            match &field.ty {
                syn::Type::Path(type_path) if type_path.path.is_ident("Uuid") => {
                    quote! { self.#ident.to_string() }
                }
                _ => quote! { self.#ident.clone() },
            }
        })
        .collect::<Vec<_>>();

    let delete_values = field_metas
        .iter()
        .filter(|field_meta| field_meta.primary)
        .map(|field_meta| {
            let ident = format_ident!("{}", field_meta.name);
            match &field_meta.ty {
                syn::Type::Path(type_path) if type_path.path.is_ident("Uuid") => {
                    quote! { self.#ident.to_string() }
                }
                _ => quote! { self.#ident.clone() },
            }
        })
        .collect::<Vec<_>>();

    // Generate expressions for reading field values from a SQLite row.
    let select_values = fields
        .iter()
        .enumerate()
        .map(|(index, field)| match &field.ty {
            syn::Type::Path(type_path) if type_path.path.is_ident("Uuid") => {
                quote! {
                    {
                        let value = row.get::<_, String>(#index)?;
                        ::uuid::Uuid::parse_str(&value)
                            // we map the error to rusqlite::Error to propagate the
                            // error as internal server error, not invalid user input.
                            .map_err(|_| ::rusqlite::Error::InvalidQuery)?
                    }
                }
            }
            _ => quote! { row.get(#index)? },
        })
        .collect::<Vec<_>>();

    let field_docs = {
        field_metas.iter()
            .enumerate()
            .map(|(cid, field_meta)| {
                format!(" | {cid} | `{}` | {} | {} | NULL | {pk} |",
                    field_meta.name,
                    field_meta.raw_sql_type,
                    if field_meta.optional { "NO" } else { "YES" },
                    pk = if field_meta.primary { "KEY" } else { "" },
                )
            })
            .collect::<Vec<_>>()
    };

    quote! {
        #(#strucc_attr)*
        ///
        /// # Schema
        /// 
        /// | CID | Name | Type | Required | Default | PK
        /// | --- | ---- | ---- | -------- | ------- | --
        #(#[doc = #field_docs])*
        #strucc

        impl #struct_name {
            /// The name of the collection's table in the database. This is used
            /// for generating SQL queries and must be unique within the database.
            ///
            /// # Table Generator
            ///
            /// ```sql
            #[doc = concat!("CREATE TABLE ", #table_name, " (...);")]
            #[doc = concat!("SELECT * FROM ", #table_name, ";")]
            /// ```
            pub const TABLE_NAME: &'static str = #table_name;

            /// Invoke table generator query for this collection.
            ///
            /// # Query
            ///
            /// This method will execute the following SQL query.
            ///
            /// ```sql
            #[doc = #sql_create_table]
            /// ```
            ///
            /// # Returns
            ///
            /// This method returns the number of rows affected by the SQLite query.
            /// For 'CREATE TABLE' queries, this always yields `0`.
            pub async fn create_table(db: &#db_state_struct_name) -> Result<usize, ::yooso_core::Error> {
                // Execute the CREATE SQL statement to create the collection table
                // if it doesn't exist. Returns the number of rows affected (should
                // be 0 for CREATE TABLE).
                let conn = db.0.lock()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                if cfg!(debug_assertions) {
                    eprintln!("\x1b[90m{}\x1b[0m", #sql_create_table);
                }

                conn.execute(#sql_create_table, [])
                    .map_err(|e| ::yooso_core::Error::from(e))

                // #[cfg(feature = "debug")]
                // assert_eq!(..., 0, "CREATE TABLE should not affect any rows");
            }

            /// Finds a single row in the collection's table by its primary key field,
            /// returning it as an instance of the struct.
            ///
            /// # Query
            ///
            /// This method will execute the following SQL query.
            ///
            /// ```sql
            #[doc = #sql_select_where]
            /// ```
            pub async fn view(db: &#db_state_struct_name, #(#primary_key_idents: &#primary_key_types),*) -> Result<Self, ::yooso_core::Error> {
                let conn = db.0.lock()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                if cfg!(debug_assertions) {
                    eprintln!("\x1b[90m{}\x1b[0m", #sql_select_where);
                }

                let mut stmt = conn
                    .prepare(#sql_select_where)
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                let mut rows = stmt.query_map(
                    ::rusqlite::params![
                        #(#primary_key_idents_repaired),*
                    ],
                    |row| {
                        Ok(#struct_name {
                            #(#field_ids: #select_values),*
                        })
                    })
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                let item = rows.next().unwrap_or(Err(::rusqlite::Error::QueryReturnedNoRows));
                item.map_err(|e| ::yooso_core::Error::from(e))
            }

            /// Lists all rows in the collection's table, returning them as a vector.
            ///
            /// **This method is not optimized for large tables and should only be used for
            /// small collections. This method is primarily intended for development and
            /// debugging purposes.**
            ///
            /// # Query
            ///
            /// This method will execute the following SQL query.
            ///
            /// ```sql
            #[doc = #sql_select_all]
            /// ```
            ///
            /// # Returns
            ///
            /// This method returns a vector of all rows in the collection's table, deserialized
            /// into instances of the struct. If the table is empty, this yields an empty vector.
            pub async fn list_all(db: &#db_state_struct_name) -> Result<::std::vec::Vec<Self>, ::yooso_core::Error> {
                let conn = db.0.lock()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                if cfg!(debug_assertions) {
                    eprintln!("\x1b[90m{}\x1b[0m", #sql_select_all);
                }

                // Execute the SELECT SQL statement to retrieve all rows from the collection table.
                let mut stmt = conn
                    .prepare(#sql_select_all)
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                let rows = stmt.query_map([], |row| {
                    Ok(#struct_name {
                        #(#field_ids: #select_values),*
                    })
                })
                .map_err(|e| ::yooso_core::Error::from(e))?;

                let vec = rows.collect::<std::result::Result<Vec<_>, ::rusqlite::Error>>()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                Ok(vec)
            }

            /// Saves the current struct instance as a new row in the collection's
            /// table.
            ///
            /// # Query
            ///
            /// This method will execute the following SQL query.
            ///
            /// ```sql
            #[doc = #sql_insert_into]
            /// ```
            ///
            /// # Returns
            ///
            /// This method returns the number of rows affected by the SQLite query.
            /// If the row was inserted for the first time, this yields `1`. If a row
            /// with the same primary key already exists, this yields `0`, overriding
            /// the existing row.
            pub async fn save(&self, db: &#db_state_struct_name) -> Result<usize, ::yooso_core::Error> {
                // Execute the INSERT SQL statement to insert a new row into the
                // collection table. Returns the number of rows affected (should
                // be 1 for INSERT and 0 for REPLACE).
                // If row with same key exists, it will be overridden.
                let conn = db.0.lock()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                if cfg!(debug_assertions) {
                    eprintln!("\x1b[90m{}\x1b[0m", #sql_insert_into);
                }

                conn.execute(#sql_insert_into, ::rusqlite::params![
                        #(#insert_values),*
                    ])
                    .map_err(|e| ::yooso_core::Error::from(e))
            }

            /// Deletes the row corresponding to the current struct instance from the
            /// collection's table. The row is identified by the primary key fields
            /// of the struct.
            ///
            /// # Query
            ///
            /// This method will execute the following SQL query.
            ///
            /// ```sql
            #[doc = #sql_delete_from]
            /// ```
            ///
            /// # Returns
            ///
            /// This method returns the number of rows affected by the SQLite query.
            /// If a row was deleted, this yields `1`. If no matching row was found
            /// to delete, this yields `0`.
            pub async fn delete(&self, db: &#db_state_struct_name) -> Result<usize, ::yooso_core::Error> {
                // Execute the DELETE SQL statement to delete the row corresponding
                // to the current struct instance from the collection table. Returns
                // the number of rows affected (should be 1 if a row was deleted, or
                // 0 if no matching row was found).
                let conn = db.0.lock()
                    .map_err(|e| ::yooso_core::Error::from(e))?;

                if cfg!(debug_assertions) {
                    eprintln!("\x1b[90m{}\x1b[0m", #sql_delete_from);
                }

                conn.execute(#sql_delete_from, ::rusqlite::params![
                        #(#delete_values),*
                    ])
                    .map_err(|e| ::yooso_core::Error::from(e))
            }
        }
    }
}
