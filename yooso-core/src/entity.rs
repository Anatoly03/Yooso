//! This module defines the [Entity] struct, which represents a
//! composed unit within the Yooso ecosystem. An [Entity] is a
//! a collection of [Component]s.

#[cfg(doc)]
use crate::Component;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entity(Uuid);

/// Converts an [Entity] into its underlying [Uuid] representation.
impl Into<Uuid> for Entity {
    fn into(self) -> Uuid {
        self.0
    }
}
