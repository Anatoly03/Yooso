//! This module includes an internal macro for generating separated
//! metadata struct parsers.
//!
//! # Example
//!
//! The following macro produces a parser for the procedural macro
//! below.
//!
//! ```rust,no_run
//! meta_parser!(
//!     /// Metadata for the [collection] attribute macro.
//!     CollectionMeta {
//!         db: syn::Path,
//!         table: syn::LitStr,
//!     }
//! );
//! ```
//!
//! ```rust,no_run
//! #[collection(db = MetaDB, table = "entities")]
//! ```

/// Internal macro for generating separated metadata struct parsers.
macro_rules! meta_parser {
    (
        $(#[$attr:meta])*
        $name:ident
        {
            $(
                $(#[$attr2:meta])*
                $field:ident: $ty:ty
            ),*$(,)?
        }
    ) => {
        $(#[$attr])*
        pub(crate) struct $name {
            $(
                $(#[$attr2])*
                $field: $ty
            ),*
        }

        impl ::syn::parse::Parse for $name {
            fn parse(attr: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                // Parse the attr as a hashmap of literal keys and tokens in the first stage,
                // parse proper types in the second stage.
                // E.g.: db = ..., table = ...
                $(let mut $field = None;)*

                while !attr.is_empty() {
                    let key: syn::Ident = attr.parse()?;
                    attr.parse::<::syn::Token![=]>()?;

                    match key.to_string().as_str() {
                        $(
                            stringify!($field) => {
                                if $field.is_some() {
                                    return Err(::syn::Error::new_spanned(
                                        key,
                                        format!("duplicate `{}` in #[collection(...)]", stringify!($field)),
                                    ));
                                }

                                $field = Some(attr.parse::<$ty>()?);
                            }
                        )*
                        _ => {
                            return Err(::syn::Error::new_spanned(
                                key.clone(),
                                format!("unknown key in #[collection(...)]: `{}`. expected any of: {}", key, vec![$(stringify!($field)),*].join(", "))
                            ));
                        }
                    }

                    if attr.is_empty() {
                        break;
                    }

                    attr.parse::<::syn::Token![,]>()?;
                }

                $(
                    let $field = $field.ok_or_else(|| {
                        ::syn::Error::new(
                            ::proc_macro2::Span::call_site(),
                            format!("missing `{}` in #[collection(...)]", stringify!($field)),
                        )
                    })?;
                )*

                Ok(Self { $( $field ),* })
            }
        }
    };
}
