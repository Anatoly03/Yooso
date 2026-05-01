//! TODO: document

mod components;
mod entities;
mod logs;

use rocket::{Build, Ignite, Rocket};
use rocket_cors::{Cors, CorsOptions};
use yooso_storage::{ComponentFieldTable, ComponentTable, EntityTable, GeneralDBState, LogDBState, LogRecordTable, MetaDBState};

/// TODO: document
pub struct Yooso {
    /// Inner [Rocket] instance. [Rocket] is the underlying web framework used
    /// by [Yooso] to provide an HTTP layer.
    pub(crate) rocket: Rocket<Build>,
}

impl Yooso {
    /// Creates a [Yooso] instance with the default config provider for [Rocket].
    pub async fn build() -> Self {
        let general_db_state = GeneralDBState::default();
        let meta_db_state = MetaDBState::default();
        let log_db_state = LogDBState::default();

        EntityTable::create_table(&meta_db_state).await
            .expect("failed to create table: `entities`");
        ComponentTable::create_table(&meta_db_state).await
            .expect("failed to create table: `components`");
        ComponentFieldTable::create_table(&meta_db_state).await
            .expect("failed to create table: `component_fields`");
        LogRecordTable::create_table(&log_db_state).await
            .expect("failed to create table: `logs`");

        Self {
            rocket: rocket::build()
                .manage(general_db_state)
                .manage(meta_db_state)
                .manage(log_db_state)
                .mount("/api/components", crate::components::routes())
                .mount("/api/entities", crate::entities::routes())
                .mount("/api/logs", crate::logs::routes())
                .attach(Self::cors_config())
                .attach(logs::LogFairing)
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
