//! Macros for the Yooso project.

mod collection_fields;
#[macro_use]
mod inner_macro_meta;
mod macro_collection;
mod macro_database;
mod macro_docapi;
mod macro_launch;
mod macro_query;
mod util;

use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

use crate::{macro_collection::CollectionMeta, macro_docapi::DocApiMeta};

/// The [launch] attribute marks the async function that builds a `Yooso`
/// application and turns it into the program entry point.
///
/// # Example
///
/// ```no_run
/// use yooso::Yooso;
///
/// #[yooso::launch]
/// async fn yooso() -> Yooso {
///     Yooso::build().await
/// }
/// ```
#[proc_macro_attribute]
pub fn launch(_args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as syn::ItemFn);
    macro_launch::launch(function).into()
}

/// The [database] attribute marks a struct as a database definition. The
/// struct will be converted into a connection pool.
///
/// # Example
///
/// ```
/// use yooso_macro::database;
///
/// #[database(".yooso/meta.sqlite")]
/// struct MetaDB;
/// ```
#[proc_macro_attribute]
pub fn database(args: TokenStream, input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(args as syn::LitStr);
    let item = parse_macro_input!(input as syn::ItemStruct);
    macro_database::database(name, item).into()
}

/// The [collection] attribute marks a struct as a table definition. The
/// struct will be converted into an SQL-synced collection of rows. This
/// attribute is smart enough to convert Rust types into SQL types.
///
/// # Example
///
/// ```
/// use yooso_macro::{database,collection};
/// use uuid::Uuid;
///
/// #[database(".yooso/meta.sqlite")]
/// struct MetaDB;
///
/// #[collection(db = MetaDB, table = "entities")]
/// struct EntityRecord {
///     #[primary]
///     id: Uuid,
///     created_at: i32,
/// }
/// ```
///
/// The example above will produce the table with the following schema in
/// the `MetaDB` database (at path `.yooso/meta.sqlite`).
///
/// | CID | Name         | Type    | Not Null | Default | PK
/// | --- | ------------ | ------- | -------- | ------- | ---
/// | 0   | `id`         | TEXT    | YES      | NULL    | KEY
/// | 1   | `created_at` | INTEGER | YES      | NULL    |
///
/// # Attributes
///
/// ### `#[primary]`
///
/// Marks the primary key column of the table. This is required for all
/// collections and must be a single field.
///
/// ### `#[unique(...)]`
///
/// Marks a unique constraint on the table. This is equivalent to
/// [SQL indecess](https://www.sqlitetutorial.net/sqlite-index/) and can be used to
/// enforce an invariant on the collection.
///
/// ```no_run
/// use yooso_macro::{database,collection};
/// use uuid::Uuid;
///
/// #[database(".yooso/meta.sqlite")]
/// struct MetaDB;
///
/// #[collection(db = MetaDB, table = "fields")]
/// #[unique(component_id, field_name)]
/// #[unique(component_id, position)]
/// pub struct ComponentFieldRecord {
///     /// Snowflake value. This is the unique identifier of the field.
///     #[primary] pub id: Uuid,
///     /// The ID of the component that this field belongs to.
///     pub component_id: Uuid,
///     /// The name of the field.
///     pub field_name: String,
///     /// The order index of the field.
///     pub position: i32,
/// }
/// ```
///
/// The example above will produce the table with the following schema in
/// the `MetaDB` database (at path `.yooso/meta.sqlite`).
///
/// | CID | Name           | Type    | Not Null | Default | PK
/// | --- | -------------- | ------- | -------- | ------- | ---
/// | 0   | `id`           | TEXT    | YES      | NULL    | KEY
/// | 1   | `component_id` | TEXT    | YES      | NULL    |
/// | 2   | `field_name`   | TEXT    | YES      | NULL    |
/// | 3   | `position`     | INTEGER | YES      | NULL    |
///
/// ### `#[default(...)]`
///
/// Currently unused, reserved for future use.
#[proc_macro_attribute]
pub fn collection(args: TokenStream, input: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(args as CollectionMeta);
    let mut item = parse_macro_input!(input as syn::ItemStruct);
    let unique_attributes = util::consume_attributes_by_name(&mut item.attrs, "unique")
        .iter()
        .map(|f| f.meta.clone())
        .collect::<Vec<_>>();

    macro_collection::collection(meta, item, unique_attributes).into()
}

/// The [query] macro allows you to write SQL queries in Rust code.
///
/// # Examples
///
/// ```no_run
/// use yooso_macro::query;
///
/// query!(SELECT EntityRecord FROM MetaDB);
/// query!(SELECT #component FROM GeneralDB WHERE entity_id = #id LIMIT 1);
/// ```
///
/// # Discussion
///
/// The `query!()` macro is designed to be a simple and knowledgable about how
/// to interpret the statement in correctly typed Rust code.
///
/// Per default, `SELECT` statements return a vector of rows, while `INSERT` and
/// `REPLACE` statements return void and `DELETE` statements return the number of
/// affected rows. If `LIMIT 1` (with the constant `1`) is specified, the vector
/// return type is changed to an Option record.
///
/// Since the project does not need the full power of being able to `SELECT` a
/// specific set of rows and the fact that this project uses multiple databases,
/// the SQL syntax `SELECT <columns> FROM <table>` has been changed to `SELECT
/// <collection> FROM <database>` (or `DELETE <collection>` etc.)
///
/// Take a look at the following few examples and the expected generated closure
/// signature.
///
/// For example the code `query!(SELECT EntityRecord FROM MetaDB WHERE entity_id
/// = #id LIMIT 1)` will be transformed into a closure with the following signature:
///
/// ```no_run
/// let id: Uuid = ...;
/// query!(SELECT ComponentRecord FROM MetaDB);
/// // |state: &MetaDBBState| -> Result<Vec<EntityRecord>>
/// ```
///
/// Reading: Find me every component record and return it as a vector.
///
/// ```no_run
/// let id: Uuid = ...;
/// query!(DELETE EntityRecord FROM MetaDB WHERE entity_id = #id);
/// // |state: &MetaDBBState| -> Result<usize>
/// ```
///
/// Reading: Delete the entity record with the specified UUID and return the number
/// of affected rows.
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    macro_query::query(input.into()).into()
}

/// The [docapi] attribute marks the function as an endpoint and generates
/// documentation for the API.
///
/// # Example
///
/// ```no_run
/// use rocket::get;
/// use yooso_macro::docapi;
///
/// #[docapi()]
/// pub struct Entity {
///     /// The UUID of the component to view.
///     uuid: Uuid,
///
///     // The timestamp of the component creation.
///     created_at: i32,
/// }
///
/// #[docapi()]
/// #[get("/view/<uuid>")]
/// pub async fn view_component(..., uuid: &str) -> ... {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn docapi(args: TokenStream, input: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(args as DocApiMeta);
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Fn(func) => macro_docapi::docapi_fn(meta, func).into(),
        Item::Struct(strucc) => macro_docapi::docapi_struct(meta, strucc).into(),
        _ => panic!("The #[docapi] attribute can only be applied to functions."),
    }
}
