//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete};
use uuid::Uuid;
use yooso_storage::{ComponentTable, MetaDBState};

/// TODO: document
#[delete("/delete/<uuid>")]
pub async fn delete_component(
    state: &State<MetaDBState>,
    uuid: &str,
) -> Result<Json<Value>, Json<Value>> {
    let id = Uuid::parse_str(&uuid).map_err(|err| {
        Json(json!({
            "success": false,
            "message": format!("Invalid component ID: {}", err),
        }))
    })?;

    ComponentTable {
        id,
        ..Default::default()
    }
    .delete_in_state(state)
    .await;

    Ok(Json(json!({
        "success": true,
    })))
}
