//! This module manages component entries in the Yooso application.

mod list;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,

    /// The name of the component, used for identification and retrieval.
    /// It is unique across the application and serves as the primary key.
    pub name: String,

    /// System-level components are used internally by the Yooso framework
    /// to manage application state. Managing such components is
    pub system: bool,

    /// Color code for visual representation in the Admin UI, stored as an
    /// RGB0 integer.
    pub color: i32,

    /// The timestamp of the component's creation, used for tracking and
    /// debugging. It is represented as a Unix timestamp in milliseconds.
    pub created_at: i64,
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list::list_components]
}
