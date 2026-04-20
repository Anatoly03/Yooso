//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::{Component, ComponentField};
use yooso_storage::{ComponentFieldTable, ComponentTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewComponentResponse {
    pub metadata: Component,
    pub fields: Vec<ComponentField>,
}

/// TODO: document
#[get("/view/<id>")]
pub async fn view_component(
    state: &State<MetaDBState>,
    id: &str,
) -> Result<Json<ViewComponentResponse>, Json<Value>> {
    let uuid = Uuid::parse_str(id).map_err(|err| {
        json! ({
            "success": false,
            "error": format!("invalid UUID: {err}"),
        })
    })?;

    let component = ComponentTable::view(state, &uuid)
        .await
        .expect("failed to view component");

    let fields = ComponentFieldTable::list_by_component_id(state, &component.id)
        .await
        .expect("failed to view component fields");

    // Convert underscore to minus in component name. (Convention
    // transformation between database and user interface).
    let name = component.component_name.replace('_', "-");

    Ok(Json(ViewComponentResponse {
        metadata: Component {
            id: component.id,
            name,
            is_system: component.is_system,
            color: component.color,
            created_at: component.created_at,
        },
        fields: fields
            .into_iter()
            .map(|field| {
                // Convert underscore to minus in field name. (Convention
                // transformation between database and user interface).
                let name = field.field_name.replace('_', "-");

                ComponentField {
                    id: field.id,
                    name,
                    field_type: field.field_type,
                    is_system: field.is_system,
                    created_at: field.created_at,
                }
            })
            .collect(),
    }))
}
