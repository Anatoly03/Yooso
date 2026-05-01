//! This module defines the entity creation endpoint.

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{EntityTable, MetaDBState};

/// The response body for the entity creation endpoint.
/// 
/// # Example
/// 
/// ```json
/// {
///     "success": true,
///     "id": "019dd39a-5605-7743-b916-4067af05d0ef",
///     "created_at": 1776695528686
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityResponse {
    pub success: bool,
    pub id: Uuid,
    pub created_at: i64,
}

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
///     "id": "019dd39a-5605-7743-b916-4067af05d0ef",
///     "created_at": 1776695528686
/// }
/// ```
#[post("/")]
pub async fn create_entity(state: &State<MetaDBState>) -> Result<Json<CreateEntityResponse>> {
    let entity = EntityTable::create_new();
    entity.save(state).await?;

    Ok(Json(CreateEntityResponse {
        success: true,
        id: entity.id,
        created_at: entity.created_at,
    }))
}
