//! This module manages entity-component relations.
//! TODO: document

use rocket::{State, delete};
use uuid::Uuid;
use yooso_core::Error;
use yooso_core::error::Result;
use yooso_storage::{ComponentRecord, GeneralDBState, MetaDBState};

/// TODO: document
#[delete("/<id>/component/<component_id>")]
pub async fn remove_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
    component_id: &str,
) -> Result<()> {
    let entity_uuid = Uuid::parse_str(id)?;
    let component_uuid = Uuid::parse_str(component_id)?;

    // Check that the component exists.
    let component = ComponentRecord::view(state, &component_uuid)
        .await
        .map_err(|_| Error::NotFound)?;
    // let component_name = component.component_name;

    // Create SQL query (insert row into component table with entity as key)
    // TODO refactor sql queries into storage layer
    let query = format!(
        "DELETE FROM {} WHERE entity_id = '{}'",
        component.component_name, entity_uuid
    );

    // Execute query
    let conn = general_state.0.lock()?;
    conn.execute(&query, [])?;

    Ok(())
}
