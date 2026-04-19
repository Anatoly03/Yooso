//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, patch};
use serde::{Deserialize, Serialize};
use yooso_core::Component;
use yooso_storage::{ComponentTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchComponentRequest {
    pub id: uuid::Uuid,
    pub name: String,
    pub is_system: bool,
    pub color: u32,
    pub created_at: i64,
}

/// TODO: document
#[patch("/", data = "<body>")]
pub async fn update_component(
    state: &State<MetaDBState>,
    body: Json<PatchComponentRequest>,
) -> Json<Component> {
    let new_component = ComponentTable {
        id: body.id,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at: body.created_at,
    };

    new_component.save_in_state(state).await;

    Json(Component {
        id: new_component.id,
        name: new_component.component_name,
        is_system: new_component.is_system,
        color: new_component.color,
        created_at: new_component.created_at,
    })
}
