//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Component;
use yooso_storage::{ComponentTable, EntityTable, GeneralDBState, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub success: bool,
    pub entities: Vec<EntityResponse>,
}

/// Represents a table in the database that corresponds to an entity in the application.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EntityResponse {
    pub id: Uuid,
    pub created_at: i64,
    pub components: Vec<Component>,
}

/// TODO: document
/// This module defines the API endpoint for listing components in the Yooso application.
#[get("/list")]
pub async fn list_entities(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
) -> Result<Json<ComponentListResponse>, Json<Value>> {
    let entities = EntityTable::list_all(state)
        .await
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to list entities: {}", err),
            }))
        })?;

    let component_tables = ComponentTable::list_all(state)
        .await
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to retrieve components: {}", err),
            }))
        })?;

    let mut response_entities = Vec::with_capacity(entities.len());

    for entity in entities {
        let mut components = vec![];

        for component_table in &component_tables {
            let has_component = entity_has_component(
                general_state,
                component_table.component_name.as_str(),
                &entity.id,
            )
            .map_err(|err| {
                Json(json!({
                    "success": false,
                    "message": err,
                }))
            })?;

            if has_component {
                components.push(Component {
                    id: component_table.id,
                    name: component_table.component_name.clone(),
                    is_system: component_table.is_system,
                    color: component_table.color,
                    created_at: component_table.created_at,
                });
            }
        }

        response_entities.push(EntityResponse {
            id: entity.id,
            created_at: entity.created_at,
            components,
        });
    }

    Ok(Json(ComponentListResponse {
        success: true,
        entities: response_entities,
    }))
}

fn entity_has_component(
    general_state: &State<GeneralDBState>,
    component_table_name: &str,
    entity_id: &Uuid,
) -> Result<bool, String> {
    let table_name = component_table_name.replace('"', "\"\"");
    let query = format!(
        "SELECT EXISTS(SELECT 1 FROM \"{}\" WHERE entity_id = '{}' LIMIT 1)",
        table_name, entity_id
    );

    let conn = general_state
        .0
        .lock()
        .map_err(|err| format!("Failed to lock general db: {}", err))?;

    let exists = conn
        .query_row(&query, [], |row| row.get::<_, i64>(0))
        .map_err(|err| {
            format!(
                "Failed to check component relation in {}: {}",
                component_table_name, err
            )
        })?;

    Ok(exists == 1)
}
