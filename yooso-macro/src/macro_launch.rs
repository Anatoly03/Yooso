use proc_macro2::TokenStream;

use quote::{format_ident, quote};
use syn::ItemFn;

/// The [launch] attribute marks the async function that builds a `Yooso`
/// application and turns it into the program entry point.
pub fn launch(mut function: ItemFn) -> TokenStream {
    let launch_fn = format_ident!("__yooso_launch_{}", function.sig.ident);

    function.sig.ident = launch_fn.clone();

    quote! {
        #function

        fn main() {
            ::yooso::rocket::tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("failed to build Tokio runtime")
                .block_on(async {
                    #launch_fn().await.launch().await;
                });
        }
    }
}
