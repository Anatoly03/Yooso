use uuid::Uuid;
use yooso_macro::{collection, database};

/// Meta database for Yooso, which contains the component definitions and other
/// system tables.
#[database(".yooso/meta.sqlite")]
pub struct MetaDB;

/// General database for Yooso, which contains the main application data.
#[database(".yooso/general.sqlite")]
pub struct GeneralDB;

/// Database for logs, which contains the HTTP log entries (request and response).
#[database(".yooso/logs.sqlite")]
pub struct LogDB;

/// Represents a table in the database that corresponds to an entity in the application.
#[collection(db = MetaDB, table = "entities")]
#[derive(Default)]
pub struct EntityTable {
    /// Snowflake value. This is the unique identifier of the entity.
    #[primary]
    pub id: Uuid,

    /// The timestamp of when the entity was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}

/// Represents a table in the database that corresponds to a component in the application.
#[collection(db = MetaDB, table = "components")]
#[derive(Default)]
pub struct ComponentTable {
    /// Snowflake value. This is the unique identifier of the component.
    #[primary]
    pub id: Uuid,

    /// The name of the component.
    pub name: String,

    /// The color of the component, represented in RGB0 integer format. This
    /// is used in the admin panel to visually distinguish components and has
    /// no functional significance in the application logic.
    pub color: u32,

    /// Whether the component is a system component (true) or a user-defined
    /// component (false).
    pub is_system: bool,

    /// The timestamp of when the component was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}

/// Represents a table in the database that corresponds to a component in the application.
#[collection(db = MetaDB, table = "fields")]
#[derive(Default)]
pub struct ComponentFieldTable {
    /// Snowflake value. This is the unique identifier of the field.
    #[primary]
    pub id: Uuid,

    /// The ID of the component that this field belongs to.
    pub component_id: Uuid,

    /// The name of the field.
    pub name: String,

    /// Whether the field is system (true) or user-defined (false).
    pub is_system: bool,

    /// The timestamp of when the component was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,
}
