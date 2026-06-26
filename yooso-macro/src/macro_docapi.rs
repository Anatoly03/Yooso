//! The module defining the [docapi][super::docapi] macro.

use fd_lock::{RwLock, RwLockWriteGuard};
use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use syn::{
    Attribute, Field, GenericArgument, ItemFn, ItemStruct, Meta, PathArguments, Type, TypePath,
};
use termcolor::{Color, ColorChoice, StandardStream};
use termcolor_output::colored;

meta_parser!(
    /// Metadata for the [docapi] attribute macro.
    DocApiMeta {
        /// The description of the API endpoint or component. This will be used in the OpenAPI documentation.
        description: syn::LitStr,

        /// The format of the API endpoint or component. This will be used in the OpenAPI documentation.
        format: syn::LitStr,
    }
);

/// The [docapi][super::docapi] attribute marks the function as an endpoint and generates
/// documentation for the API.
///
/// # Example
///
/// ```no_run,no_test,ignore
/// #[docapi()]
/// #[get("/view/<uuid>")]
/// pub async fn view_component(..., uuid: &str) -> ... {
///     ...
/// }
/// ```
pub fn docapi_fn(_meta: DocApiMeta, func: ItemFn) -> TokenStream {
    // The "root" of the rust crate, which is a member of the workspace. The openapi
    // documentation will be generated into this path + "openapi".
    let openapi_dir = caller_openapi_dir();

    // The build hash.
    let build_hash = get_build_hash();

    let url_attr =
        get_rocket_attr(&func).expect("Expected a Rocket HTTP method attribute on the function.");
    let http_method = url_attr.path().get_ident().unwrap().to_string();

    // Rocket URL attributes are of the form `#[get("/path")]` or `#[post("/path", body = "<body>")]`.
    // Te path is the first argument to the attribute, which is a string literal.
    let url = match &url_attr.meta {
        Meta::List(meta_list) => meta_list
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

    // Open the file and create it if not exists. We also lock it in a cross-process
    // lock to prevent side effects.
    //
    // ...
    // ├── openapi
    // │   ├── ...
    // │   └── openapi.json
    // └ ...
    let mut lock = lock_file(openapi_dir.join("openapi.json"));
    let mut guard = lock.write().unwrap();

    // Parse or create the JSON structure
    let mut openapi_value = read_json_file(&mut guard).unwrap_or_else(|_| {
        json!({
            "build_hash": build_hash,
            "paths": {},
            "components": {},
        })
    });

    // Check for build hash match (to ensure file is cleaned once before a new build)
    match openapi_value.get("build_hash").and_then(|v| v.as_str()) {
        Some(existing_hash) if existing_hash != build_hash => {
            // This is a new build - we keep the endpoints but update the hash
            openapi_value["build_hash"] = json!(build_hash);
        }
        None => {
            // No hash found - add it
            openapi_value["build_hash"] = json!(build_hash);
        }
        _ => { /* hash matches */ }
    }

    // Set OpenAPI file metadata.
    openapi_value["openapi"] = json!("3.1.0");
    openapi_value["info"] = json!({
        "title": std::env::var("CARGO_PKG_NAME").unwrap(),
        "version": std::env::var("CARGO_PKG_VERSION").unwrap(),
        // "authors": std::env::var("CARGO_PKG_AUTHORS").unwrap().split(':').collect::<Vec<_>>(),
    });

    // Ensure paths and components exist. This is a safety check in case the file
    // was manually edited and these keys were removed.
    if !openapi_value.get("paths").is_some() {
        openapi_value["paths"] = json!({});
    }
    if !openapi_value.get("components").is_some() {
        openapi_value["components"] = json!({});
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
        openapi_value["paths"][&path_key] = json!({});
    }

    // Process path parameter names.
    let path_params = path_key
        .split('/')
        .filter(|s| !s.is_empty())
        .filter(|segment| segment.starts_with('{') && segment.ends_with('}'))
        .map(|segment| &segment[1..segment.len() - 1])
        .collect::<Vec<_>>();

    // Path parameter documentation.
    let path_param_docs = path_params
        .iter()
        .map(|param| {
            json!({
                "name": param,
                "in": "path",
                "required": true,
                "schema": {
                    "type": "string",
                },
                "description": format!("To-do: document `{}` path parameter", param),
            })
        })
        .collect::<Vec<_>>();

    // Add the HTTP method with its operation
    let method_lower = http_method.to_lowercase();
    openapi_value["paths"][&path_key][&method_lower] = json!({
        "operationId": operation_id,
        "summary": format!("To-do: document `{}` endpoint", &url),
        "parameters": path_param_docs
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
/// ```no_run,no_test,ignore
/// #[docapi()]
/// pub struct Entity {
///     /// The UUID of the component to view.
///     uuid: Uuid,
///
///     // The timestamp of the component creation.
///     created_at: i32,
/// }
/// ```
pub fn docapi_struct(_meta: DocApiMeta, strucc: ItemStruct) -> TokenStream {
    // The "root" of the rust crate, which is a member of the workspace. The openapi
    // documentation will be generated into this path + "openapi".
    let openapi_dir = caller_openapi_dir();

    // The build hash.
    let build_hash = get_build_hash();

    // Open the file and create it if not exists. We also lock it in a cross-process
    // lock to prevent side effects.
    //
    // ...
    // ├── openapi
    // │   ├── ...
    // │   └── openapi.json
    // └ ...
    let mut lock = lock_file(openapi_dir.join("openapi.json"));
    let mut guard = lock.write().unwrap();

    // Parse or create the JSON structure
    let mut openapi_value = read_json_file(&mut guard).unwrap_or_else(|_| {
        json!({
            "build_hash": build_hash,
            "paths": {},
            "components": {},
        })
    });

    // Check for build hash match (to ensure file is cleaned once before a new build)
    match openapi_value.get("build_hash").and_then(|v| v.as_str()) {
        Some(existing_hash) if existing_hash != build_hash => {
            // This is a new build - we keep the endpoints but update the hash
            openapi_value["build_hash"] = json!(build_hash);
        }
        None => {
            // No hash found - add it
            openapi_value["build_hash"] = json!(build_hash);
        }
        _ => { /* hash matches */ }
    }

    // Set OpenAPI file metadata.
    openapi_value["openapi"] = json!("3.1.0");
    openapi_value["info"] = json!({
        "title": std::env::var("CARGO_PKG_NAME").unwrap(),
        "version": std::env::var("CARGO_PKG_VERSION").unwrap(),
        // "authors": std::env::var("CARGO_PKG_AUTHORS").unwrap().split(':').collect::<Vec<_>>(),
    });

    // Ensure paths and components exist. This is a safety check in case the file
    // was manually edited and these keys were removed.
    if !openapi_value.get("paths").is_some() {
        openapi_value["paths"] = json!({});
    }
    if !openapi_value.get("components").is_some() {
        openapi_value["components"] = json!({});
    }

    // Add the schema to the components.
    let struct_name = strucc.ident.to_string();

    // Determine the required fields of the struct. A field is required if it is not an Option<T>.
    let required_struct_fields = strucc
        .fields
        .iter()
        .filter_map(|s| match &s.ty {
            Type::Path(TypePath { path, .. })
                if path.segments.iter().any(|f| f.ident == "Option") =>
            {
                None
            }
            _ => s.ident.as_ref().map(|f| f.to_string()),
        })
        .collect::<Vec<_>>();

    // Get the properties of every struct field.
    let struct_properties = strucc
        .fields
        .iter()
        .map(|field| {
            let ty = get_openapi_type(&field.ty);
            let format = get_openapi_format(&field);
            let description = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("doc"))
                .filter_map(|attr| match &attr.meta {
                    Meta::NameValue(meta) => meta
                        .value
                        .to_token_stream()
                        .to_string()
                        .trim_matches('"')
                        .to_string()
                        .into(),
                    _ => None,
                })
                .collect::<Vec<_>>();
            // let docapi_attr = field
            //     .attrs
            //     .iter()
            //     .find(|attr| attr.path().is_ident("docapi"))
            //     .expect("Expected a #[docapi] attribute on the struct field.");
            let key = field
                .ident
                .as_ref()
                .expect("Only named identifiers allowed")
                .to_string();
            let mut value = json!({
                "type": ty,
                "description": description.join(" "),
            });

            if let Some(format) = format {
                value["format"] = json!(format);
            }

            (key, value)
        })
        .collect::<HashMap<_, _>>();

    let description = strucc
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(meta) => meta
                .value
                .to_token_stream()
                .to_string()
                .trim_matches('"')
                .trim()
                .to_string()
                .into(),
            _ => None,
        })
        .collect::<Vec<_>>();

    openapi_value["components"]["schemas"][&struct_name] = json!({
        "type": "object",
        "required": required_struct_fields,
        "properties": struct_properties,
        "description": description.join(" "),
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

    strucc.to_token_stream()
}

/// The openapi documentation will be generated into the root of the project + "openapi".
/// This function will create the directory if it doesn't exist and return the path to it.
/// It will also fill the directory with a `.gitignore` file to prevent it from being
/// staged into version control.
///
/// ```txt
/// api/
/// ├── openapi/
/// |   ├── ...
/// │   └── .gitignore
/// ├── src/
/// │  └── ...
/// └ Cargo.toml
/// ```
fn caller_openapi_dir() -> PathBuf {
    let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("openapi");

    // Create the openapi directory if it doesn't exist. This will build the following layout:
    //
    // api/
    // ├── openapi/
    // │   └── .gitignore
    // ├── src/
    // │  └── ...
    // └ Cargo.toml
    std::fs::create_dir_all(&path).expect("Failed to create openapi directory.");
    std::fs::write(&path.join(".gitignore"), "*").expect("Failed to write .gitignore file.");

    path
}

/// Concatenation of process ID and current timestamp which is unique per build and
/// equal for all macro invocations in the same build.
fn get_build_hash() -> String {
    // Read the current timestamp of the build. If not set, set it.
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

    // Concatenate process ID and timestamp.
    format!("{}-{}", std::process::id(), timestamp)
}

/// Open the file and create it if not exists. It will be locked in a cross-process
/// lock to prevent side effects.
fn lock_file(path: PathBuf) -> RwLock<File> {
    let openapi_file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open openapi.json file.");

    RwLock::new(openapi_file)
}

/// Reads the contents of the file from the given lock guard and parses it as JSON. If the
/// file is empty or invalid JSON, it will return an error.
fn read_json_file<V: DeserializeOwned>(guard: &mut RwLockWriteGuard<File>) -> std::io::Result<V> {
    // Buffer the contents of te file into a string.
    let mut content = String::new();
    guard.read_to_string(&mut content)?;

    // Parse as JSON; if it is invalid JSON or an empty file - start fresh
    serde_json::from_str::<V>(&content).map_err(|_| std::io::ErrorKind::InvalidData.into())
}

/// Retrieves the HTTP method from the rocket api function.
fn get_rocket_attr<'a>(func: &'a ItemFn) -> Option<&'a Attribute> {
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

/// Returns the OpenAPI type for a given Rust type. This is used to generate the
/// `type` field in the OpenAPI schema for struct fields.
fn get_openapi_type(ty: &Type) -> &str {
    match &ty {
        Type::Array(_) => "array",
        // Type::BareFn(type_bare_fn) => todo!(),
        Type::Group(type_group) => get_openapi_type(&type_group.elem),
        // Type::ImplTrait(type_impl_trait) => todo!(),
        // Type::Infer(type_infer) => todo!(),
        // Type::Macro(type_macro) => todo!(),
        // Type::Never(type_never) => todo!(),
        Type::Paren(type_paren) => get_openapi_type(&type_paren.elem),
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path
                .segments
                .last()
                .expect("Expected a type path to have at least one segment.");
            let ident_str = last_segment.ident.to_string();
            let arguments = &last_segment.arguments;

            match ident_str.as_str() {
                "String" | "str" => "string",
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => "integer",
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => "integer",
                "f32" | "f64" => "number",
                "bool" => "boolean",
                "Vec" => "array",
                "Option" => {
                    let generics = match arguments {
                        PathArguments::AngleBracketed(angle_bracketed) => &angle_bracketed.args,
                        _ => return "object",
                    };

                    if let Some(GenericArgument::Type(inner_ty)) = generics.first() {
                        return get_openapi_type(inner_ty);
                    }

                    "object"
                }
                "Uuid" => "string",
                _ => "object", // Default to object for unknown types
            }
        }
        Type::Ptr(type_ptr) => get_openapi_type(&type_ptr.elem),
        Type::Reference(type_reference) => get_openapi_type(&type_reference.elem),
        Type::Slice(_) => "array",
        // Type::TraitObject(type_trait_object) => todo!(),
        Type::Tuple(type_tuple) => {
            if type_tuple.elems.is_empty() {
                "null" // Empty tuple represents the unit type `()`, which is often treated as null
            } else {
                "array" // Non-empty tuples can be represented as arrays in OpenAPI
            }
        }
        // Type::Verbatim(token_stream) => todo!(),
        _ => panic!("Unsupported type for OpenAPI generation: {:?}", ty),
    }
}

/// Returns the OpenAPI format for a given Rust type. This is used to generate the
/// `format` field in the OpenAPI schema for struct fields. Currently the only functionality
/// this method provides is to return the correct format for integer types.
///
/// ```no_run,no_test,ignore
/// #[docapi()]
/// pub struct Auth {
///     #[docapi(format = "email")]
///     pub email: String,
/// }
/// ```
///
/// See: https://docs.bump.sh/openapi/v3.2/data-models/schema-and-data-types/#data-formats
fn get_openapi_format(field: &Field) -> Option<&str> {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path
                .segments
                .last()
                .expect("Expected a type path to have at least one segment.");
            let ident_str = last_segment.ident.to_string();

            match ident_str.as_str() {
                // isize
                #[cfg(target_pointer_width = "32")]
                "isize" => Some("int32"),
                #[cfg(target_pointer_width = "64")]
                "isize" => Some("int64"),

                // usize
                #[cfg(target_pointer_width = "32")]
                "usize" => Some("int32"),
                #[cfg(target_pointer_width = "64")]
                "usize" => Some("int64"),

                // signed integers
                "i8" | "i16" | "i32" => Some("int32"),
                "i64" | "i128" => Some("int64"),

                // unsigned integers
                "u8" | "u16" | "u32" => Some("int32"),
                "u64" | "u128" => Some("int64"),

                // uuid
                "Uuid" => Some("uuid"),

                _ => None,
            }
        }
        _ => None,
    }
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
/// ```no_run,no_test,ignore
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
