//! Defines the component deletion endpoint.

use rocket::{State, delete};
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
) -> Result<()> {
    let id = Uuid::parse_str(uuid)?;
    let component = ComponentRecord::view(state, &id)
        .await
        .map_err(|_| Error::NotFound)?;
    component.delete_recursive(state, general_state).await?;

    Ok(())
}
