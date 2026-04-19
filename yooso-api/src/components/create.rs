//! TODO: document

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use yooso_core::{Component, ComponentField};
use yooso_storage::{ComponentFieldTable, ComponentTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentRequest {
    pub name: String,
    pub is_system: bool,
    pub fields: Vec<CreateComponentField>,
    pub color: u32,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentField {
    pub name: String,
    pub is_system: bool,
    pub field_type: String,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentResponse {
    pub metadata: Component,
    pub fields: Vec<ComponentField>,
}

/// TODO: document
#[post("/", data = "<body>")]
pub async fn create_component(
    state: &State<MetaDBState>,
    body: Json<CreateComponentRequest>,
) -> Json<CreateComponentResponse> {
    let uuid = uuid::Uuid::now_v7();
    let created_at = chrono::Utc::now().timestamp_millis();

    let new_component = ComponentTable {
        id: uuid,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at,
    };

    let new_fields = body
        .fields
        .iter()
        .enumerate()
        .map(|(position, field)| ComponentFieldTable {
            id: uuid::Uuid::now_v7(),
            component_id: uuid,
            field_name: field.name.clone(),
            field_type: field.field_type.clone(),
            is_system: field.is_system,
            position: position as i32,
            created_at,
        })
        .collect::<Vec<_>>();

    new_component.save(state).await;

    for field in &new_fields {
        field.save(state).await;
    }

    let fields = new_fields
        .iter()
        .map(|field| ComponentField {
            id: field.id,
            name: field.field_name.clone(),
            field_type: field.field_type.clone(),
            is_system: field.is_system,
            created_at: field.created_at,
        })
        .collect::<Vec<_>>();

    Json(CreateComponentResponse {
        metadata: Component {
            id: new_component.id,
            name: new_component.component_name,
            is_system: new_component.is_system,
            color: new_component.color,
            created_at: new_component.created_at,
        },
        fields,
    })
}
