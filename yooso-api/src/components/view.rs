//! Defines the component view endpoint.

use rocket::serde::json::Json;
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_core::{Component, ComponentField};
use yooso_storage::{ComponentFieldTable, ComponentTable, MetaDBState};

/// Response structure for viewing a component.
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewComponentResponse {
    pub metadata: Component,
    pub fields: Vec<ComponentField>,
}

/// Endpoint for viewing a component by its UUID.
/// 
/// # Example Request
/// 
/// ```http
/// GET /view/123e4567-e89b-12d3-a456-426614174000
/// ```
/// 
/// # Example Response
/// 
/// 
#[get("/view/<uuid>")]
pub async fn view_component(
    state: &State<MetaDBState>,
    uuid: &str,
) -> Result<Json<ViewComponentResponse>> {
    let id = Uuid::parse_str(&uuid)?;
    let component = ComponentTable::view(state, &id).await?;
    let fields = ComponentFieldTable::list_by_component_id(state, &component.id).await?;

    let name = component.component_name;

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
                let name = field.field_name;

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
