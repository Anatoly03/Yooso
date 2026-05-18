//! Defines the entity creation endpoint.

use crate::success::Success;
use rocket::{State, post};
use yooso_core::error::Result;
use yooso_storage::{EntityRecord, MetaDBState};

/// The endpoint for creating a new entity. This will generate a new entity with
/// a new v7 UUID and the current timestamp.
///
/// # Example Request
///
/// ```http
/// POST /api/entities/
/// ```
///
/// # Example Response
///
/// ```json
/// {
///     "success": true,
///     "data": {
///         "id": "019dd39a-5605-7743-b916-4067af05d0ef",
///         "created_at": 1776695528686
///     }
/// }
/// ```
#[post("/")]
pub async fn create_entity(state: &State<MetaDBState>) -> Result<Success<EntityRecord>> {
    let entity = EntityRecord::create_new();
    entity.save(state).await?;
    Ok(Success(entity))
}
