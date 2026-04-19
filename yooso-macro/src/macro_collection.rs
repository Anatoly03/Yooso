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

    // Generate [FieldMeta] for each field and consume any field-level
    // attributes like #[primary].
    let field_metas = strucc
        .fields
        .iter()
        .map(|field| FieldMeta::from(field.clone()))
        .collect::<Vec<_>>();

    // Consume "#[primary]" attributes on fields.
    for field in &mut strucc.fields {
        field
            .attrs
            .retain(|attr| !attr.meta.path().is_ident("primary"));
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
        let sql_create_table_fields = field_metas
            .iter()
            .map(|field_meta| field_meta.into_field_definition())
            .collect::<Vec<_>>();

        format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name.value(),
            sql_create_table_fields.join(", ")
        )
    };

    // Generate 'SELECT * FROM' syntax for the list generator.
    let sql_select_all = format!("SELECT * FROM {}", table_name.value());

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
        let primary_keys = field_metas
            .iter()
            .filter(|field_meta| field_meta.primary)
            .map(|field_meta| field_meta.name.clone())
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
            "DELETE FROM {} WHERE {}",
            table_name.value(),
            where_clause
        )
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

    // Generate expressions for reading field values from a SQLite row.
    let select_values = fields
        .iter()
        .enumerate()
        .map(|(index, field)| match &field.ty {
            syn::Type::Path(type_path) if type_path.path.is_ident("Uuid") => {
                quote! {
                    {
                        let value = row.get::<_, String>(#index)?;
                        ::uuid::Uuid::parse_str(&value).map_err(|err| {
                            ::rusqlite::Error::FromSqlConversionFailure(
                                #index,
                                ::rusqlite::types::Type::Text,
                                Box::new(err),
                            )
                        })?
                    }
                }
            }
            _ => quote! { row.get(#index)? },
        })
        .collect::<Vec<_>>();

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
            pub async fn create_table() -> usize {
                let db = #db_struct_name::connect();

                // Execute the CREATE SQL statement to create the collection table
                // if it doesn't exist. Returns the number of rows affected (should
                // be 0 for CREATE TABLE).
                db.lock()
                    .expect("lock db mutex")
                    .execute(#sql_create_table, [])
                    .expect("create collection table")

                // #[cfg(feature = "debug")]
                // assert_eq!(..., 0, "CREATE TABLE should not affect any rows");
            }

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
            pub async fn create_table_in_state(db: &MetaDBState) -> usize {
                // Execute the CREATE SQL statement to create the collection table
                // if it doesn't exist. Returns the number of rows affected (should
                // be 0 for CREATE TABLE).
                db.0.lock()
                    .expect("lock db mutex")
                    .execute(#sql_create_table, [])
                    .expect("create collection table")

                // #[cfg(feature = "debug")]
                // assert_eq!(..., 0, "CREATE TABLE should not affect any rows");
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
            pub async fn list_all() -> ::rusqlite::Result<::std::vec::Vec<Self>> {
                let db = #db_struct_name::connect();
                let conn = db.lock()
                    .expect("lock db mutex");

                // Execute the SELECT SQL statement to retrieve all rows from the collection table.
                let mut stmt = conn
                    .prepare(#sql_select_all)?;

                let rows = stmt.query_map([], |row| {
                    Ok(#struct_name {
                        #(#field_ids: #select_values),*
                    })
                })?;

                rows.collect()
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
            pub async fn list_all_in_state(db: &MetaDBState) -> ::rusqlite::Result<::std::vec::Vec<Self>> {
                let conn = db.0.lock()
                    .expect("lock db mutex");

                // Execute the SELECT SQL statement to retrieve all rows from the collection table.
                let mut stmt = conn
                    .prepare(#sql_select_all)?;

                let rows = stmt.query_map([], |row| {
                    Ok(#struct_name {
                        #(#field_ids: #select_values),*
                    })
                })?;

                rows.collect()
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
            pub async fn save(&self) -> usize {
                let db = #db_struct_name::connect();

                // Execute the INSERT SQL statement to insert a new row into the
                // collection table. Returns the number of rows affected (should
                // be 1 for INSERT and 0 for REPLACE).
                // If row with same key exists, it will be overridden.
                db.lock()
                    .expect("lock db mutex")
                    .execute(#sql_insert_into, ::rusqlite::params![
                        #(#insert_values),*
                    ])
                    .expect("insert into collection table")
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
            pub async fn save_in_state(&self, db: &MetaDBState) -> usize {
                // Execute the INSERT SQL statement to insert a new row into the
                // collection table. Returns the number of rows affected (should
                // be 1 for INSERT and 0 for REPLACE).
                // If row with same key exists, it will be overridden.
                db.0.lock()
                    .expect("lock db mutex")
                    .execute(#sql_insert_into, ::rusqlite::params![
                        #(#insert_values),*
                    ])
                    .expect("insert into collection table")
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
            pub async fn delete(&self) -> usize {
                todo!("delete method not implemented yet")
            }

            // TODO: implement `delete_in_state`
        }
    }
}
