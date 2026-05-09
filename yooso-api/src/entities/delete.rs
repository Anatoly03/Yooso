//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete};
use uuid::Uuid;
use yooso_storage::{ComponentTable, EntityTable, GeneralDBState, MetaDBState};

/// TODO: document
#[delete("/<uuid>")]
pub async fn delete_entity(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<Json<Value>, Json<Value>> {
    let id = Uuid::parse_str(&uuid).map_err(|err| {
        Json(json!({
            "success": false,
            "message": format!("Invalid entity ID: {}", err),
        }))
    })?;

    let component_tables = ComponentTable::list_all(state)
        .await
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to list components: {}", err),
            }))
        })?;

    {
        let conn = general_state
            .0
            .lock()
            .map_err(|err| {
                Json(json!({
                    "success": false,
                    "message": format!("Failed to lock general db: {}", err),
                }))
            })?;

        for component in component_tables {
            let table_name = component.component_name.clone();
            let query = format!("DELETE FROM \"{}\" WHERE entity_id = ?", table_name);

            conn.execute(&query, [id.to_string()])
                .map_err(|err| {
                    Json(json!({
                        "success": false,
                        "message": format!("Failed to delete component data from {}: {}", table_name, err),
                    }))
                })?;
        }
    }

    EntityTable {
        id,
        ..Default::default()
    }
    .delete(state)
    .await;

    Ok(Json(json!({
        "success": true,
    })))
}
