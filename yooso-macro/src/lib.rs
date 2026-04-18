use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

/// The `launch` attribute marks the async function that builds a `Yooso`
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
	let mut function = parse_macro_input!(input as ItemFn);
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
	.into()
}
