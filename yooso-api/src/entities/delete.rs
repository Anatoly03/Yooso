//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{ComponentTable, EntityTable, GeneralDBState, MetaDBState};

/// TODO: document
#[delete("/<uuid>")]
pub async fn delete_entity(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<Json<Value>> {
    let id = Uuid::parse_str(&uuid).map_err(|e| yooso_core::Error::from(e))?;
    let component_tables = ComponentTable::list_all(state).await?;

    {
        let conn = general_state
            .0
            .lock()
            .map_err(|e| yooso_core::Error::from(e))?;

        for component in component_tables {
            let table_name = component.component_name.clone();
            let query = format!("DELETE FROM \"{}\" WHERE entity_id = ?", table_name);

            conn.execute(&query, [id.to_string()])
                .map_err(|e| yooso_core::Error::from(e))?;
        }
    }

    EntityTable::delete(state, id).await?;

    Ok(Json(json!({
        "success": true,
    })))
}
