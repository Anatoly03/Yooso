//! Defines the component deletion endpoint.

use crate::success::SuccessUnit;
use rocket::{State, delete};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{ComponentRecord, GeneralDBState, MetaDBState};

/// The endpoint for deleting a component. Finds the component record corresponding to
/// the given UUID and recursively deletes the component and all of its fields from the
/// database.
///
/// # Example Request
///
/// ```http
/// DELETE /api/components/<uuid>
/// ```
///
/// # Example Response
///
/// ```json
/// {
///     "success": true
/// }
/// ```
#[delete("/<uuid>")]
pub async fn delete_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<SuccessUnit> {
    let id = Uuid::parse_str(&uuid)?;
    let component = ComponentRecord::view(state, &id).await?;
    component.delete_recursive(state, general_state).await?;
    Ok(SuccessUnit)
}
