mod collection_fields;
#[macro_use]
mod inner_macro_meta;
mod macro_collection;
mod macro_database;
mod macro_launch;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::macro_collection::CollectionMeta;

/// The [launch] attribute marks the async function that builds a `Yooso`
/// application and turns it into the program entry point.
///
/// # Example
///
/// ```rust,no_run
/// use yooso::Yooso;
///
/// #[yooso::launch]
/// async fn yooso() -> Yooso {
///     Yooso::build()
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
/// ```rust,no_run
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
/// struct will be converted into an SQL-synced collection of rows.
///
/// # Example
///
/// ```rust,no_run
/// use yooso_macro::collection;
/// use crate::MetaDB;
///
/// #[collection(db = MetaDB, table = "entities")]
/// struct EntityTable {
///     #[primary]
///     id: Uuid,
///     created_at: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn collection(args: TokenStream, input: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(args as CollectionMeta);
    let item = parse_macro_input!(input as syn::ItemStruct);
    macro_collection::collection(meta, item).into()
}
