//! The Yooso application state.

use crate::{db::Database, error::InternalError};
use axum::Router;
use std::{format, println};
use tokio::net::TcpListener;

/// A Yooso application.
pub struct App {
    /// The [axum] router type for composing handlers and services.
    pub(crate) router: Router,

    /// The port on which the Yooso application will run.
    pub(crate) port: String,
}

impl App {
    /// Constructs a new Yooso application.
    ///
    /// This will "prepare" a Yooso app and create the `.yooso` data folder, as
    /// well as initialize the [Router] and the [Database].
    pub async fn init() -> Result<Self, InternalError> {
        // Create .yooso data directory.
        std::fs::create_dir_all(".yooso").unwrap();

        // Prepare states.
        let db_state = Database::init().await?;
        db_state.migrate().await?;

        // Prepare general.
        let port = dotenvy::var("YOOSO_PORT").unwrap_or("8090".into());

        // Prepare router.
        let router = Router::new().with_state(db_state);

        Ok(Self { router, port })
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
        // Creates a new TcpListener, which is ready for accepting connections.
        let address = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&address).await?;
        println!("Listening on {address}");

        // Serve the [axum] application.
        axum::serve(listener, self.router).await?;

        Ok(())
    }
}
