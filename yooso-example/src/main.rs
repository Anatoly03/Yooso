//! Example of a simple Yooso application.
//! 
//! It doesn't do anything more than set up the default features, including the
//! admin panel Yooso Studio.
//! 
//! Below is the minimal code for the `main.rs` file to startup a Yooso application.
//! 
//! ```rust
//! use yooso::Yooso;
//! 
//! /// Example function that builds a [Yooso] application. The [yooso::launch]
//! /// attribute macro will also generate the `main` function.
//! #[yooso::launch]
//! async fn yooso() -> Yooso {
//!     Yooso::build().await
//! }
//! ```

use yooso::Yooso;

/// Example function that builds a [Yooso] application. The [yooso::launch]
/// attribute macro will also generate the `main` function.
#[yooso::launch]
async fn yooso() -> Yooso {
    Yooso::build().await
}
