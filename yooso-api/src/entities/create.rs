//! Defines the entity creation endpoint.

use rocket::{State, http::Status, post};
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
/// Content-Length: 36
/// Content-Type: text/plain; charset=utf-8
///
/// 019dd39a-5605-7743-b916-4067af05d0ef
/// ```
#[post("/")]
pub async fn create_entity(state: &State<MetaDBState>) -> yooso_core::Result<(Status, String)> {
    // This always returns 201 as nothing can go wrong here.
    let entity = EntityRecord::create_new();
    entity.save(state).await?;
    Ok((Status::Created, entity.id.to_string()))
}
