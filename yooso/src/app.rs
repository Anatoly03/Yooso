//! The Yooso application state.

use crate::{db, error::InternalError};
use axum::Router;
use std::{format, println};
use tokio::net::TcpListener;

pub struct App {
    /// The [axum] router type for composing handlers and services.
    pub(crate) router: Router,

    /// The port on which the Yooso application will run.
    pub(crate) port: usize,
}

impl App {
    /// Constructs a new Yooso application.
    ///
    /// This will "prepare" a Yooso app and create the `.yooso` data folder, as
    /// well as initialize the [Router]. After an app is prepared,
    pub fn new() -> Self {
        // Create .yooso data directory.
        std::fs::create_dir_all(".yooso").unwrap();

        // Prepare Yooso app
        let router = Router::new();
        let port = {
            let port_str = dotenvy::var("YOOSO_PORT").unwrap_or("8090".into());
            port_str.parse::<usize>().unwrap_or(8090)
        };

        Self { router, port }
    }

    /// Serves the [axum] application with the supplied [TcpListener]. If
    /// [listen][Self::listen] was not called yet, the listener will be lazily
    /// created.
    ///
    /// # Example
    ///
    /// ```rust,no_test,ignore
    /// yooso::App::new()
    ///     .listen().await.unwrap()
    ///     .deploy().await.unwrap();
    /// ```
    pub async fn deploy(self) -> Result<(), InternalError> {
        // Creates the SQLite pool.
        let _pool = db::init().await?;

        // Creates a new TcpListener, which is ready for accepting connections.
        let address = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&address).await?;
        println!("Listening on {address}");

        // Serve the [axum] application.
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}

impl Default for App {
    /// Returns a Yooso application with default values, equivalent to
    /// [App::new].
    fn default() -> Self {
        Self::new()
    }
}
