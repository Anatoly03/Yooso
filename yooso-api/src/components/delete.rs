//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete};
use uuid::Uuid;
use yooso_storage::{ComponentTable, GeneralDBState, MetaDBState};

/// TODO: document
#[delete("/<uuid>")]
pub async fn delete_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<Json<Value>, Json<Value>> {
    let id = Uuid::parse_str(&uuid).map_err(|err| {
        Json(json!({
            "success": false,
            "message": format!("Invalid component ID: {}", err),
        }))
    })?;

    // Fetch the component from the meta database to ensure it exists and
    // to get its table name for the drop command.
    let component = ComponentTable::view(state, &id)
        .await
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to view component: {}", err),
            }))
        })?;


    // Delete the component from the meta database.
    ComponentTable {
        id,
        ..Default::default()
    }
    .delete(state)
    .await;

    // Generate the SQL query for dropping the component table.
    let drop_table_query = sql_query_drop_table(&component);

    // Drop the component's table from the general database.
    general_state
        .0
        .lock()
        .expect("failed to acquire lock on general database")
        .execute(&drop_table_query, [])
        .expect("failed to drop component table in general database");

    Ok(Json(json!({
        "success": true,
    })))
}

/// Helper function to generate the SQL query for droping a component table
/// based on the component name.
// TODO: use a proper SQL query builder library instead of string concatenation to
// prevent SQL injection and handle edge cases.
fn sql_query_drop_table(component: &ComponentTable) -> String {
    format!(
        "DROP TABLE {}",
        component.component_name
    )
}
