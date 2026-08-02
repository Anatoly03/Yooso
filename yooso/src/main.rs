//! The Yooso standalone entry point.
//!
//! For smaller applications, and non-Rust applications it makes sense to use
//! [yooso] as a standalone binary.

use yooso::{App, InternalError};

/// The entry point for the "Yooso Light" binary. This function is invoked if
/// [yooso] is used as a standalone executable. It will construct a "default"
/// config for a Yooso App and then start the web server.
#[tokio::main]
async fn main() -> Result<(), InternalError> {
    dotenvy::dotenv().ok();
    App::init().await?.deploy().await
}
