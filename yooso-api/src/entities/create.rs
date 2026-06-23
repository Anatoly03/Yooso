//! Defines the entity creation endpoint.

use rocket::{State, http::Status, post, serde::json::Json};
use yooso_core::Result;
use yooso_macro::docapi;
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
/// ```http
/// 201 Created
///
/// {
///   "id": "019e2bb7-1fc2-7e73-9df5-5a74537e0bcd",
///   "created_at": 1778849882050
/// }
/// ```
#[docapi()]
#[post("/api/entities")]
pub async fn create_entity(state: &State<MetaDBState>) -> Result<(Status, Json<EntityRecord>)> {
    // This always returns 201 as nothing can go wrong here.
    let entity = EntityRecord::create_new();
    entity.save(state).await?;
    Ok((Status::Created, Json(entity)))
}
