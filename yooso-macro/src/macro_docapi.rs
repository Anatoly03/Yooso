//! The module defining the [docapi][super::docapi] macro.

use fd_lock::RwLock;
use proc_macro2::TokenStream;
use quote::ToTokens;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use syn::{Attribute, ItemFn, ItemStruct};
use termcolor::{Color, ColorChoice, StandardStream};
use termcolor_output::colored;

/// The [docapi][super::docapi] attribute marks the function as an endpoint and generates
/// documentation for the API.
///
/// # Example
///
/// ```no_run
/// #[docapi()]
/// #[get("/view/<uuid>")]
/// pub async fn view_component(..., uuid: &str) -> ... {
///     ...
/// }
/// ```
pub fn docapi_fn(func: ItemFn) -> TokenStream {
    let url_attr =
        get_rocket_attr(&func).expect("Expected a Rocket HTTP method attribute on the function.");
    let http_method = url_attr.path().get_ident().unwrap().to_string();

    // The "root" of the rust crate, which is a member of the workspace. The openapi
    // documentation will be generated into this path + "openapi".
    let caller_openapi_dir = {
        let s = format!(
            "{}{}",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            "/openapi"
        );
        PathBuf::from(s)
    };

    // Read the current timestamp of the build, which generates the unique build hash. If not
    // set, set it.
    let timestamp = match std::env::var("YOOSO_BUILD_TIMESTAMP") {
        Ok(ts) => ts,
        Err(_) => {
            let ts = format!("{}", chrono::Utc::now().timestamp_millis());

            unsafe {
                std::env::set_var("YOOSO_BUILD_TIMESTAMP", &ts);
            }

            ts
        }
    };

    // The build hash.
    let build_hash = format!("{}-{}", std::process::id(), timestamp);

    // Shortcut for generating a file path for the openapi documentation.
    let fp = |filename: &str| caller_openapi_dir.join(filename);

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

    // Create the openapi directory if it doesn't exist. This will build the following layout:
    //
    // api/
    // ├── openapi/
    // │   └── .gitignore
    // ├── src/
    // │  └── ...
    // └ Cargo.toml
    std::fs::create_dir_all(&caller_openapi_dir).expect("Failed to create openapi directory.");
    std::fs::write(fp(".gitignore"), "*").expect("Failed to write .gitignore file.");

    // Open the file and create it if not exists. We also lock it in a cross-process
    // lock to prevent side effects.
    //
    // api/
    // ├── openapi/
    // │   ├── ...
    // │   └── openapi.json
    // └ ...
    let openapi_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(fp("openapi.json"))
        .expect("Failed to open openapi.json file.");
    let mut lock = RwLock::new(openapi_file);
    let mut guard = lock.write().unwrap();

    // Parse or create the JSON structure
    let mut openapi_value = {
        // Buffer the contents of te file into a string.
        let mut content = String::new();
        guard
            .read_to_string(&mut content)
            .expect("Failed to read openapi.json file.");

        // Parse as JSON; if it is invalid JSON or an empty file - start fresh
        match serde_json::from_str::<Value>(&content) {
            Ok(value) => value,
            Err(_) => {
                serde_json::json!({
                    "build_hash": build_hash,
                    "paths": {},
                    "components": {},
                })
            }
        }
    };

    // Check for build hash match (to ensure file is cleaned once before a new build)
    match openapi_value.get("build_hash").and_then(|v| v.as_str()) {
        Some(existing_hash) if existing_hash != build_hash => {
            // This is a new build - we keep the endpoints but update the hash
            openapi_value["build_hash"] = serde_json::json!(build_hash);
        }
        None => {
            // No hash found - add it
            openapi_value["build_hash"] = serde_json::json!(build_hash);
        }
        _ => { /* hash matches */ }
    }

    // Set OpenAPI file metadata.
    openapi_value["openapi"] = serde_json::json!("3.1.0");
    openapi_value["info"] = serde_json::json!({
        "title": std::env::var("CARGO_PKG_NAME").unwrap(),
        "version": std::env::var("CARGO_PKG_VERSION").unwrap(),
        "authors": std::env::var("CARGO_PKG_AUTHORS").unwrap().split(':').collect::<Vec<_>>(),
    });

    // Ensure paths and components exist. This is a safety check in case the file
    // was manually edited and these keys were removed.
    if !openapi_value.get("paths").is_some() {
        openapi_value["paths"] = serde_json::json!({});
    }
    if !openapi_value.get("components").is_some() {
        openapi_value["components"] = serde_json::json!({});
    }

    // Add the endpoint to the paths
    // Note: Use the path as key (without query parameters)
    let path_key = {
        let path_prefix = url.split('?').next().unwrap_or(&url);
        let mut pk = String::new();

        for segment in path_prefix.split('/') {
            if !segment.is_empty() {
                pk.push('/');
            }

            if segment.starts_with('<') && segment.ends_with('>') {
                pk.push_str(format!("{{{}}}", &segment[1..segment.len() - 1]).as_str());
            } else {
                pk.push_str(segment);
            }
        }

        pk
    };
    // let path_key = url.split('?').next().unwrap_or(&url);

    // The `operationId` field. It's supposed to remove the leading "api" from the
    // path as well as remove inputs in angle brackets. For example, `GET /api/view/<uuid>/list`
    // will produce `get_view_list`
    let operation_id = {
        let mut oid = http_method.to_lowercase();

        for segment in path_key.split('/') {
            if !segment.is_empty()
                && segment != "api"
                && !segment.starts_with('{')
                && !segment.ends_with('}')
            {
                oid.push('_');
                oid.push_str(segment);
            }
        }

        oid
    };

    // Ensure the path exists.
    if !openapi_value["paths"].get(&path_key).is_some() {
        openapi_value["paths"][&path_key] = serde_json::json!({});
    }

    // Add the HTTP method with its operation
    let method_lower = http_method.to_lowercase();
    openapi_value["paths"][&path_key][&method_lower] = serde_json::json!({
        "operationId": operation_id,
        "summary": "Todo: Implement #[docapi(summary = \"...\")]",
    });

    // IMPORTANT: Write the file - ONLY truncate when creating a new file
    // For existing files, we should NOT truncate
    guard
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek to start");
    guard.set_len(0).expect("Failed to truncate");
    guard
        .write_all(openapi_value.to_string().as_bytes())
        .expect("Failed to write");

    // Print the endpoint
    print_documentation(&http_method, &url);
    func.to_token_stream()
}

/// The [docapi][super::docapi] attribute marks the struct as a component and
/// generates documentation for the API.
///
/// # Example
///
/// ```no_run
/// #[docapi()]
/// pub struct Entity {
///     /// The UUID of the component to view.
///     uuid: Uuid,
///
///     // The timestamp of the component creation.
///     created_at: i32,
/// }
/// ```
pub fn docapi_struct(func: ItemStruct) -> TokenStream {
    todo!("add support for #[docapi] on structs to generate component documentation");
}

/// Retrieves the HTTP method from the rocket api function.
pub fn get_rocket_attr<'a>(func: &'a ItemFn) -> Option<&'a Attribute> {
    const HTTP_METHODS: &[&str] = &["get", "post", "put", "delete", "patch"];

    // &func.attrs.iter().filter(|attr| attr.path().get_ident().is_some()).find(|attr| HTTP_METHODS.contains(&attr.path().get_ident().unwrap().to_string().as_str()));
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
