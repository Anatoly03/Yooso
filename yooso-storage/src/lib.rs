use uuid::Uuid;
use yooso_macro::database;

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
pub struct EntityTable {
    id: Uuid,
    created_at: i32,
}
