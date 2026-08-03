//! Entity-related API endpoints.

use axum::{Json, extract::{Path, State}};
use uuid::Uuid;

use crate::InternalError;

use super::{Database, Entity};

/// The endpoint for creating new entities.
///
/// # Endpoint
///
/// ```http
/// GET /api/entities
/// ```
#[axum::debug_handler]
pub async fn list_entities(State(db): State<Database>) -> Result<Json<Vec<Entity>>, InternalError> {
    Ok(Json(Entity::fetch_all(&db).await?))
}

/// The endpoint for creating new entities.
///
/// # Endpoint
///
/// ```http
/// POST /api/entities
/// ```
#[axum::debug_handler]
pub async fn create_entity(State(db): State<Database>) -> Result<Json<Entity>, InternalError> {
    let entity = Entity::new();
    entity.push(&db).await?;
    Ok(Json(entity))
}

/// The endpoint for deleting entities. Returns the deleted entity metadata or
/// `null` if the deleted entity did not exist.
///
/// # Endpoint
///
/// ```http
/// DELETE /api/entities/:id
/// ```
#[axum::debug_handler]
pub async fn delete_entity(State(db): State<Database>, Path(uuid): Path<Uuid>) -> Result<Json<Option<Entity>>, InternalError> {
    let entity = Entity::view(&db, &uuid).await?;

    if let Some(e) = entity {
        e.delete(&db).await?;
    }

    Ok(Json(entity))
}
