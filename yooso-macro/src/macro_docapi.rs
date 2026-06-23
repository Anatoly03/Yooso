//! The module defining the [docapi][super::docapi] macro.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemFn};
use termcolor::{Color, ColorChoice, StandardStream};
use termcolor_output::colored;

/// TODO
pub fn docapi(func: ItemFn) -> TokenStream {
    let url_attr =
        get_rocket_attr(&func).expect("Expected a Rocket HTTP method attribute on the function.");
    let http_method = url_attr.path().get_ident().unwrap().to_string();

    // Rocket URL attributes are of the form `#[get("/path")]` or `#[post("/path", body = "<body>")]`.
    // Te path is the first argument to the attribute, which is a string literal.
    let url = match &url_attr.meta {
        syn::Meta::List(meta_list) => meta_list
            .tokens
            .clone()
            .into_iter()
            .next()
            .expect("Expected a Rocket URL attribute to have at least one argument.")
            .to_string()
            .trim_matches('"')
            .to_string(),
        _ => panic!("Expected a Rocket URL attribute to be a list."),
    };

    print_documentation(&http_method, &url);

    func.to_token_stream()
}

/// Retrieves the HTTP method from the rocket api function.
pub fn get_rocket_attr<'a>(func: &'a ItemFn) -> Option<&'a Attribute> {
    const HTTP_METHODS: &[&str] = &["get", "post", "put", "delete", "patch"];

    for attr in &func.attrs {
        let ident = match attr.path().get_ident() {
            Some(ident) => ident,
            None => continue,
        };

        if HTTP_METHODS.contains(&ident.to_string().as_str()) {
            return Some(attr);
        }
    }

    None
}

/// Print the documentation for the given API endpoint function to the build logs.
fn print_documentation(http_method: &str, url: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let endpoint = format!("{} {}", http_method.to_uppercase(), url);
    print_title_line(&mut stdout, "Endpoint", Color::Cyan, &endpoint).unwrap();
}

/// Prints a title line with the given title, color, and body to the provided stdout stream.
/// This matches the Rust "terminal output" style for printing colored text to the terminal.
///
/// ```no_run
/// // Prints:
/// //
/// // ```
/// //              ...
/// //    Compiling yooso-macro v0.0.0 (/home/anatoly/Projects/Yooso/yooso-macro)
/// //    Compiling yooso-storage v0.0.0 (/home/anatoly/Projects/Yooso/yooso-storage)
/// //    Compiling yooso-api v0.0.0 (/home/anatoly/Projects/Yooso/yooso-api)
/// //     Endpoint /api/todo
/// //              ...
/// // ```
/// print_title_line(&mut stdout, "Endpoint", Color::Cyan, "/api/todo").unwrap();
/// ```
fn print_title_line(
    stdout: &mut StandardStream,
    title: &str,
    color: Color,
    body: &str,
) -> Result<(), std::io::Error> {
    colored!(
        stdout,
        "{}{}{}{}{}{}\n", // format string
        fg!(Some(color)),
        bold!(true),               // title style
        format!("{:>12} ", title), // title
        fg!(None),
        bold!(false), // reset style
        body
    )
}
