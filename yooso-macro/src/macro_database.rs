use proc_macro2::TokenStream;

use quote::quote;
use syn::{Fields, ItemStruct, LitStr};

/// The [database] attribute marks a struct as a database definition. The
/// struct will be converted into a connection pool.
pub fn database(path: LitStr, strucc: ItemStruct) -> TokenStream {
    let ident = &strucc.ident;

    match strucc.fields {
        Fields::Unit => (),
        _ => panic!(
            "database struct must be a unit struct: `struct {};`",
            strucc.ident
        ),
    };

    quote! {
        #strucc

        impl #ident {
            pub fn path() -> &'static str {
                #path
            }
        }
    }
}
