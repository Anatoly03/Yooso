//! The Yooso standalone entry point.
//!
//! For smaller applications, and non-Rust applications it makes sense to use
//! [yooso] as a standalone binary.

use yooso::App;

/// The entry point for the "Yooso Light" binary. This function is invoked if
/// [yooso] is used as a standalone executable. It will construct a "default"
/// config for a Yooso App and start the web server.
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    App::new().listen().await?.deploy().await
}
