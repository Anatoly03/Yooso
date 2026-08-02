//! The Yooso application state.

use axum::Router;
use std::io;
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

    /// Creates a new [TcpListener], which will be bound to `0.0.0.0:3000`. The
    /// listener is ready for accepting connections.
    pub async fn listen(mut self) -> Result<Self, io::Error> {
        self.listener = Some(TcpListener::bind("0.0.0.0:3000").await?);
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
