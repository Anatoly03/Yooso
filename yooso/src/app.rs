//! The Yooso application state.

use axum::Router;
use std::{format, io, println};
use tokio::net::TcpListener;

#[derive(Default)]
pub struct App {
    /// The [axum] router type for composing handlers and services.
    pub(crate) router: Router,

    /// A TCP socket server, listening for connections. The socket will be closed
    /// when [App] is dropped. This will be set only when [listen][Self::listen]
    /// is called.
    pub(crate) listener: Option<TcpListener>,
}

impl App {
    /// Constructs a new Yooso application.
    pub fn new() -> Self {
        let router = Router::new();

        Self {
            router,
            listener: None,
        }
    }

    /// Creates a new [TcpListener], which is ready for accepting connections.
    ///
    /// The listener will be bound to the the port provided by the environment
    /// variable `YOOSO_PORT`. Binding with a port number of 0 will request that
    /// the OS assigns a port to this listener.
    pub async fn listen(mut self) -> Result<Self, io::Error> {
        let port = dotenvy::var("YOOSO_PORT").unwrap_or("8090".into());
        let address = format!("0.0.0.0:{port}");
        self.listener = Some(TcpListener::bind(&address).await?);
        println!("Listening on {address}");
        Ok(self)
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
    pub async fn deploy(mut self) -> Result<(), io::Error> {
        // Get the listener or lazy assign a new listener.
        let listener = {
            if self.listener.is_none() {
                self = self.listen().await?;
            };
            self.listener.unwrap()
        };

        // Serve the [axum] application.
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
