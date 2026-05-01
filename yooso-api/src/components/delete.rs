//! Defines the component deletion endpoint.

use rocket::{State, delete, serde::json::Json};
use serde::Serialize;
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{ComponentTable, GeneralDBState, MetaDBState};

/// The response body for the component deletion endpoint.
/// 
/// # Example
/// 
/// ```json
/// {
///     "success": true
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

/// The endpoint for deleting a component. This will remove the component from the database.
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
) -> Result<Json<SuccessResponse>> {
    let id = Uuid::parse_str(&uuid)?;

    // Find the component table corresponding to the given UUID.
    let component = ComponentTable::view(state, &id).await?;

    // Recursively delete the component and all of its fields from the database.
    component.delete_recursive(state, general_state).await?;

    Ok(Json(SuccessResponse { success: true }))
}
