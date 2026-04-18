//! TODO: document

use rocket::{Build, Ignite, Rocket};

/// TODO: document
pub struct Yooso {
    /// Inner [Rocket] instance. [Rocket] is the underlying web framework used
    /// by [Yooso] to provide an HTTP layer.
    pub(crate) rocket: Rocket<Build>,
}

impl Yooso {
    /// Creates a [Yooso] instance with the default config provider for [Rocket].
    pub fn build() -> Self {
        Self {
            rocket: rocket::build(),
        }
    }

    /// Launches the [Yooso] application, consuming the instance in the process.
    pub async fn launch(self) -> Rocket<Ignite> {
        self.rocket.launch().await.unwrap()
    }
}
