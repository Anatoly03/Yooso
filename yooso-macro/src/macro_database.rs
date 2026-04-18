use proc_macro2::TokenStream;

use quote::quote;
use syn::{Fields, ItemStruct, LitStr};

/// The [database] attribute marks a struct as a database definition. The
/// struct will be converted into a connection pool.
pub fn database(file_path: LitStr, strucc: ItemStruct) -> TokenStream {
    let ident = &strucc.ident;
    // let mutex_ident = format_ident!("{}Mutex", ident);

    match strucc.fields {
        Fields::Unit => (),
        _ => panic!(
            "database struct must be a unit struct: `struct {};`",
            strucc.ident
        ),
    };

    quote! {
        #strucc

        // /// The connection mutex for the database.
        // struct #mutex_ident (pub ::std::sync::Mutex<
        //     ::rusqlite::Connection
        // >);

        impl #ident {
            const PATH: &str = #file_path;

            /// Initializes the database connection.
            pub fn new() -> Self {
                let _ = ::rusqlite::Connection::open(Self::PATH).expect("open sqlite db");
                Self
            }
        }
    }
}
