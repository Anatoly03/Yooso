//! TODO: document

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use yooso_core::Component;
use yooso_storage::{ComponentTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentRequest {
    pub name: String,
    pub is_system: bool,
    pub color: u32,
}

/// TODO: document
#[post("/", data = "<body>")]
pub async fn create_component(
    state: &State<MetaDBState>,
    body: Json<CreateComponentRequest>,
) -> Json<Component> {
    let uuid = uuid::Uuid::now_v7();
    let created_at = chrono::Utc::now().timestamp_millis();

    let new_component = ComponentTable {
        id: uuid,
        name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at,
    };

    new_component.save_in_state(state).await;

    Json(Component {
        id: new_component.id,
        name: new_component.name,
        is_system: new_component.is_system,
        color: new_component.color,
        created_at: new_component.created_at,
    })
}
