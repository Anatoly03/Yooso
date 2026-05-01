use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemFn;

/// The [launch] attribute marks the async function that builds a `Yooso`
/// application and turns it into the program entry point.
pub fn launch(mut function: ItemFn) -> TokenStream {
    let launch_fn = format_ident!("__yooso_launch_{}", function.sig.ident);
    let original_fn_vis = function.vis.clone();
    let original_fn_attr = function.attrs.clone();

    function.sig.ident = launch_fn.clone();

    quote! {
        #[doc(hidden)]
        #function

        #(#original_fn_attr)*
        ///
        /// # Macro Expansion
        /// 
        /// The function marked with [yooso::launch] will be renamed internally
        /// to `__yooso_launch_yooso` and wrapped into an async [main] function.
        /// 
        /// ```no_run
        #[doc = concat!(stringify!(#original_fn_vis fn main()), " {")]
        ///     ::yooso::rocket::tokio::runtime::Builder
        ///         ::new_multi_thread()
        ///         .enable_all()
        ///         .build()
        ///         .expect("failed to build Tokio runtime")
        ///         .block_on(async {
        #[doc = concat!("\t\t\t\t", stringify!(#launch_fn().await.launch().await;))]
        ///         });
        /// }
        /// ```
        #original_fn_vis fn main() {
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
