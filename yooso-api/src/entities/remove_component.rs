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
    // parse UUIDs from strings, return 400 if invalid
    let entity_uuid = Uuid::parse_str(id)?;
    let component_uuid = Uuid::parse_str(component_id)?;

    // check that the component exists, return 404 if it doesn't
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

    match rows {
        0 => Ok(Status::NoContent), // component was not attached to the entity, but we can consider it a success
        _ => Ok(Status::NoContent),
    }
}
