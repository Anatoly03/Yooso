//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete};
use uuid::Uuid;
use yooso_storage::{EntityTable, MetaDBState};

/// TODO: document
#[delete("/<uuid>")]
pub async fn delete_entity(
    state: &State<MetaDBState>,
    uuid: &str,
) -> Result<Json<Value>, Json<Value>> {
    let id = Uuid::parse_str(&uuid).map_err(|err| {
        Json(json!({
            "success": false,
            "message": format!("Invalid entity ID: {}", err),
        }))
    })?;

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
