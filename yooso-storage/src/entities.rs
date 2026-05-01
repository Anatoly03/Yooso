use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Corresponds to an entity in the application.
#[collection(db = crate::MetaDB, table = "entities")]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EntityTable {
    /// Snowflake value. This is the unique identifier of the entity.
    #[primary]
    pub id: Uuid,

    /// The timestamp of when the entity was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}

impl EntityTable {
    /// Creates a new `EntityTable` instance with a new v7 UUID and the current
    /// timestamp. This will not save the entity to the database. To save the
    /// entity, refer to [EntityTable::save].
    /// 
    /// Probability theory guarantees that this will be unique across all entities
    /// created in the system, even across multiple instances of the application
    /// running concurrently.
    pub fn create_new() -> Self {
        let id = Uuid::now_v7();
        let created_at = Utc::now().timestamp_millis();
        Self { id, created_at }
    }
}
