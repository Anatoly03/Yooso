//! Yōso is a backend platform inspired by ECS data design. The data is
//! structured as flexible entities with reusable components rather than
//! tabular or schematic presets.

// Re-exports
pub use rocket;
pub use rocket::{Route, delete, get, head, options, patch, post, put, routes, trace};
pub use yooso_api::Yooso;
pub use yooso_core::{Component, Entity};
pub use yooso_macro::launch;
