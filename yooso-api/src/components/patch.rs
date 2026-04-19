//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, patch};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Component;
use yooso_storage::{ComponentTable, ComponentFieldTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchComponentRequest {
    pub id: Uuid,
    pub name: String,
    pub is_system: bool,
    pub color: u32,
    pub created_at: i64,
    pub fields: Vec<PatchFieldRequest>,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchFieldRequest {
    pub id: Option<Uuid>,
    pub name: String,
    pub is_system: bool,
    pub field_type: String,
    pub created_at: Option<i64>,
    pub operation: PatchFieldOperation,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PatchFieldOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "remove")]
    Remove,
}

/// TODO: document
#[patch("/", data = "<body>")]
pub async fn update_component(
    state: &State<MetaDBState>,
    body: Json<PatchComponentRequest>,
) -> Result<Json<Component>, Json<Value>> {
    let new_component = ComponentTable {
        id: body.id,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at: body.created_at,
    };

    new_component.save(state).await;

    // Process Deletions
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Remove)
    {
        ComponentFieldTable {
            id: field.id.ok_or(json!({
                "error": "Invalid field ID"
            }))?,
            ..Default::default()
        }.delete(state).await;
    }

    // Process Updates
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Update)
    {
        // TODO
        println!(
            "TODO: update field `{}.{}`",
            new_component.component_name, field.name
        );
    }

    // Process Additions
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Add)
    {
        ComponentFieldTable {
            id: Uuid::now_v7(),
            component_id: new_component.id,
            field_name: field.name.clone(),
            field_type: field.field_type.clone(),
            is_system: field.is_system,
            position: 0, // TODO: determine position
            created_at: chrono::Utc::now().timestamp_millis(),
        }.save(state).await;
    }

    Ok(Json(Component {
        id: new_component.id,
        name: new_component.component_name,
        is_system: new_component.is_system,
        color: new_component.color,
        created_at: new_component.created_at,
    }))
}
