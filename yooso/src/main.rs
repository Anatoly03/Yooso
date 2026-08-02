//! The entry point for the Yooso binary. This modules is invoked if [yooso]
//! is not used as a dependency, but a standalone executable.

use yooso::App;

/// The entry point for the Yooso binary. This function is invoked if [yooso]
/// is used as a standalone executable. It will construct a "default" config
/// for a Yooso App and start the web server.
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    App::new().listen().await?.deploy().await
}
