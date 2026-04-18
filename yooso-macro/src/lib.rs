mod macro_database;
mod macro_launch;

use proc_macro::TokenStream;
use syn::{parse_macro_input};

/// The [launch] attribute marks the async function that builds a `Yooso`
/// application and turns it into the program entry point.
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
#[proc_macro_attribute]
pub fn database(args: TokenStream, input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(args as syn::LitStr);
    let item = parse_macro_input!(input as syn::ItemStruct);
    macro_database::database(name, item).into()
}
