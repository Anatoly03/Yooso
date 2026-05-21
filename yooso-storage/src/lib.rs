//! Abstractions of databases and collections for the Yooso project.
//!
//! This module defines the databases [MetaDB], [GeneralDB], and [LogDB], as
//! well as the tables that fill these databases.

#[macro_use]
extern crate yooso_macro;

mod components;
mod entities;
mod fields;
mod logs;
mod pagination;

pub use components::ComponentRecord;
pub use entities::EntityRecord;
pub use fields::ComponentFieldRecord;
pub use logs::LogRecord;
pub use pagination::Pagination;

/// Meta database for Yooso, which contains the component definitions and other
/// system tables.
#[database(".yooso/meta.sqlite")]
pub struct MetaDB;

/// General database for Yooso, which contains the main application data:
/// Developer-defined component collections.
#[database(".yooso/general.sqlite")]
pub struct GeneralDB;

/// Database for logs, which contains the HTTP log entries (request and response).
#[database(".yooso/logs.sqlite")]
pub struct LogDB;
