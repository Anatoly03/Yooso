//! TODO: document

mod components;
mod entities;

use rocket::{Build, Ignite, Rocket};
use rocket_cors::{Cors, CorsOptions};
use yooso_storage::{ComponentFieldTable, ComponentTable, EntityTable, MetaDBState};

/// TODO: document
pub struct Yooso {
    /// Inner [Rocket] instance. [Rocket] is the underlying web framework used
    /// by [Yooso] to provide an HTTP layer.
    pub(crate) rocket: Rocket<Build>,
}

impl Yooso {
    /// Creates a [Yooso] instance with the default config provider for [Rocket].
    pub async fn build() -> Self {
        let meta_db_state = MetaDBState::default();

        EntityTable::create_table(&meta_db_state).await;
        ComponentTable::create_table(&meta_db_state).await;
        ComponentFieldTable::create_table(&meta_db_state).await;

        Self {
            rocket: rocket::build()
                .manage(meta_db_state)
                .mount("/api/components", crate::components::routes())
                .mount("/api/entities", crate::entities::routes())
                .attach(Self::cors_config()),
        }
    }

    /// Private constructor for CORS configuration. This is used internally by the
    /// CORS fairing.
    pub(self) fn cors_config() -> Cors {
        let options = CorsOptions::default();
        Cors::from_options(&options).unwrap()
    }

    /// Launches the [Yooso] application, consuming the instance in the process.
    pub async fn launch(self) -> Rocket<Ignite> {
        self.rocket.launch().await.unwrap()
    }
}
