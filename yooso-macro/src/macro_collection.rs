use crate::collection_fields::FieldMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct};

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
pub fn collection(table_metadata: CollectionMeta, mut strucc: ItemStruct) -> TokenStream {
    let struct_name = &strucc.ident;
    let table_name = &table_metadata.table;
    let db_struct_name = &table_metadata.db;

    // Generate 'CREATE TABLE' syntax for the table generator.
    let sql_create_table = {
        // This generates a vector of field definitions.
        let sql_create_table_fields = match &strucc.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .iter()
                .map(|field| Into::<FieldMeta>::into(field.clone()))
                .map(|field_meta| field_meta.into_field_definition())
                .collect::<Vec<_>>(),
            Fields::Unnamed(_) => panic!("#[collection] struct must have named fields"),
            Fields::Unit => panic!("#[collection] cannot be a unit struct"),
        };

        format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name.value(),
            sql_create_table_fields.join(", ")
        )
    };

    // Consume "#[primary]" attributes on fields.
    for field in &mut strucc.fields {
        field.attrs.retain(|attr| !attr.meta.path().is_ident("primary"));
    }

    quote! {
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

            /// Invoke table generator query for this collection. This will execute
            /// the following SQL query
            ///
            /// ```sql
            #[doc = #sql_create_table]
            /// ```
            pub async fn create_table() -> usize {
                let db = #db_struct_name::connect();

                // Execute the CREATE SQL statement to create the collection table if it doesn't exist.
                // Returns the number of rows affected (should be 0 for CREATE TABLE).
                db.lock()
                    .expect("lock db mutex")
                    .execute(#sql_create_table, [])
                    .expect("create collection table")

                // #[cfg(feature = "debug")]
                // assert_eq!(..., 0, "CREATE TABLE should not affect any rows");
            }
        }
    }
}
