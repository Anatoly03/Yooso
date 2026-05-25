//! Defines the component deletion endpoint.

use rocket::{State, delete, http::Status};
use uuid::Uuid;
use yooso_core::{Error, error::Result};
use yooso_storage::{ComponentRecord, GeneralDBState, MetaDBState};

/// The endpoint for deleting a component. Finds the component record corresponding to
/// the given UUID and recursively deletes the component and all of its fields from the
/// database.
///
/// # Example Request
///
/// ```http
/// DELETE /api/components/019de463-92cc-7fb0-8cfa-a1efd95f93ee
/// ```
///
/// # Example Response
///
/// ```http
/// 200 OK
/// ```
#[delete("/<uuid>")]
pub async fn delete_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<Status> {
    // If uuid is not valid, return 400 Bad Request.
    let id = Uuid::parse_str(uuid)?;

    // Fetch the component metadata.
    // For deletions, return 200 OK if the data was actively deleted and return
    // 204 No Content if the data was either not found or "already deleted".
    let component = match ComponentRecord::view(state, &id).await {
        Ok(component) => component,
        Err(Error::NotFound) => return Ok(Status::NoContent),
        Err(e) => return Err(e.into()),
    };

    // Recursively delete component (deletes the metadata record and the dynamic
    // SQL table)
    component.delete_recursive(state, general_state).await?;

    Ok(Status::Ok)
}
