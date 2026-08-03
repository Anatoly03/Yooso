//! Entity-related API endpoints.

use axum::{Json, extract::State};

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

/// The endpoint for deleting entities.
///
/// # Endpoint
///
/// ```http
/// DELETE /api/entities/:id
/// ```
#[axum::debug_handler]
pub async fn delete_entity(State(db): State<Database>) -> Result<Json<Entity>, InternalError> {
    let entity = Entity::new();
    entity.push(&db).await?;
    Ok(Json(entity))
}
