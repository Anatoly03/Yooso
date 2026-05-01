use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemStruct, LitStr};

/// The [database] attribute marks a struct as a database definition. The
/// struct will be converted into a connection pool.
pub fn database(file_path: LitStr, strucc: ItemStruct) -> TokenStream {
    let ident = &strucc.ident;
    let state_ident = format_ident!("{ident}State");
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

        #[doc = concat!("The state struct for [", stringify!(#ident), "], ")]
        /// which contains the connection mutex (generated with the
        /// [database][yooso_macro::database] macro).
        pub struct #state_ident (
            pub ::std::sync::Mutex<::rusqlite::Connection>
        );

        // /// The connection mutex for the database.
        // struct #mutex_ident (pub ::std::sync::Mutex<
        //     ::rusqlite::Connection
        // >);

        impl #ident {
            const PATH: &str = #file_path;

            /// Initializes the database connection.
            pub(crate) fn connect() -> ::std::sync::Mutex<::rusqlite::Connection> {
                // Create the directories along the path if they don't exist.
                // Note that the last slash is not included in the path, so we
                // need to get the parent directory.
                if let Some(parent) = ::std::path::Path::new(Self::PATH).parent() {
                    ::std::fs::create_dir_all(parent).expect("create database directories");
                }

                // Open the SQLite database connection and wrap it in a mutex.
                ::std::sync::Mutex::new(
                    ::rusqlite::Connection::open(Self::PATH).expect("open sqlite db")
                )
            }

            /// Creates the state struct for this database, which contains the connection
            /// mutex.
            pub fn state() -> #state_ident {
                #state_ident(Self::connect())
            }
        }

        impl ::std::default::Default for #state_ident {
            fn default() -> Self {
                Self(#ident::connect())
            }
        }
    }
}
