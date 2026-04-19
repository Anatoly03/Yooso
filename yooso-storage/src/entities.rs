use uuid::Uuid;

/// Represents a table in the database that corresponds to an entity in the application.
#[collection(db = crate::MetaDB, table = "entities")]
#[derive(Default)]
pub struct EntityTable {
    /// Snowflake value. This is the unique identifier of the entity.
    #[primary]
    pub id: Uuid,

    /// The timestamp of when the entity was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}
