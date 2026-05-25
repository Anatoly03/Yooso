//! This module manages entity-component relations.
//! TODO: document

use rocket::{State, delete, http::Status};
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
) -> Result<Status> {
    // If uuid is not valid, return 400 Bad Request.
    let entity_uuid = Uuid::parse_str(id)?;
    let component_uuid = Uuid::parse_str(component_id)?;

    // Fetch component metadata.
    let component = ComponentRecord::view(state, &component_uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    // // retrieve the component schema to check that the component is attached to the entity
    // // we can return the data to show what the state was before being deleted
    // component.for_entity(state, general_state, id).await?;

    // remove the component from the entity
    let rows = component
        .remove_for_entity(general_state, &entity_uuid)
        .await?;

    // For deletions, return 200 OK if the data was actively deleted and return
    // 204 No Content if the data was either not found or "already deleted".
    match rows {
        0 => Ok(Status::NoContent), // component was not attached to the entity, but we can consider it a success
        _ => Ok(Status::Ok),
    }
}
