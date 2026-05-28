use proc_macro2::{TokenStream, TokenTree};

/// The [query] macro allows you to write SQL queries in Rust code.
/// 
/// ## Mechanics
/// 
/// ```no_run
/// // TODO
/// // query!(SELECT ComponentRecord WHERE id = 1);
/// ```
pub fn query(input: TokenStream) -> TokenStream {
    let _: Vec<TokenTree> = input.into_iter().map(|f| f).collect::<Vec<TokenTree>>();

    todo!("query!() macro is not implemented yet.");
}

// /// Helper method to check if a vector of token trees contains the
// /// specified identifier at the top level (not inside a group).
// pub fn contains_token(tokens: &Vec<TokenTree>, token: &str) -> bool {
//     tokens.iter().any(|f| match f {
//         TokenTree::Ident(ident) if ident == token => true,
//         _ => false,
//     })
// }
