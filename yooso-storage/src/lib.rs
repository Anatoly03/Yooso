#[macro_use]
extern crate yooso_macro;

mod components;
mod entities;
mod fields;
mod logs;

pub use components::ComponentTable;
pub use entities::EntityTable;
pub use fields::ComponentFieldTable;
pub use logs::LogRecordTable;

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
