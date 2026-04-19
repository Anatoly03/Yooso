//! This module defines the [Component] struct, which represents a
//! composable unit of functionality within the Yooso ecosystem.

#[cfg(doc)]
use crate::Entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A [Component] represents a building block of data that can be
/// attached to an [Entity] within the Yooso ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,

    /// The name of the component, used for identification and retrieval.
    /// It is unique across the application and serves as the primary key.
    pub name: String,

    /// System-level components are used internally by the Yooso framework
    /// to manage application state. Managing such components is
    pub is_system: bool,

    /// Color code for visual representation in the Admin UI, stored as an
    /// RGB0 integer.
    pub color: u32,

    /// The timestamp of the component's creation, used for tracking and
    /// debugging. It is represented as a Unix timestamp in milliseconds.
    pub created_at: i64,
}
