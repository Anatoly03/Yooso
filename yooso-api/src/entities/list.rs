//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use yooso_storage::{EntityTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub success: bool,
    pub entities: Vec<EntityTable>,
}

/// TODO: document
/// This module defines the API endpoint for listing components in the Yooso application.
#[get("/list")]
pub async fn list_entities(
    state: &State<MetaDBState>,
) -> Result<Json<ComponentListResponse>, Json<Value>> {
    EntityTable::list_all(state)
        .await
        .map(|entities| {
            Json(ComponentListResponse {
                success: true,
                entities,
            })
        })
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to list entities: {}", err),
            }))
        })
}
