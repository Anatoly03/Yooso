//! TODO: document

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_storage::{EntityTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityResponse {
    pub id: Uuid,
    pub created_at: i64,
}

/// TODO: document
#[post("/")]
pub async fn create_entity(state: &State<MetaDBState>) -> Json<CreateEntityResponse> {
    let uuid = uuid::Uuid::now_v7();
    let created_at = chrono::Utc::now().timestamp_millis();
    let entity = EntityTable {
        id: uuid,
        created_at,
    };

    entity.save(state).await;

    Json(CreateEntityResponse {
        id: entity.id,
        created_at: entity.created_at,
    })
}
