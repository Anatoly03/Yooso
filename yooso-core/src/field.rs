//! This module defines the [ComponentField] struct, which represents a
//! composable unit of functionality within the Yooso ecosystem.

#[cfg(doc)]
use crate::Component;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A [ComponentField] represents a building block of data that can be
/// attached to a [Component] within the Yooso ecosystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ComponentField {
    pub id: Uuid,

    /// The name of the field, used for identification and retrieval.
    /// It is unique across the component fields.
    pub name: String,

    /// The type of the field, represented as a string.
    pub field_type: String,

    /// System-level fields are used internally by the Yooso framework
    /// to manage component state.
    pub is_system: bool,

    /// The timestamp of the field's creation, used for tracking and
    /// debugging. It is represented as a Unix timestamp in milliseconds.
    pub created_at: i64,
}
