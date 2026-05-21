//! Defines the entity deletion endpoint.

use rocket::{State, delete, http::Status};
use uuid::Uuid;
use yooso_core::Result;
use yooso_storage::{ComponentRecord, EntityRecord, GeneralDBState, MetaDBState};

/// The endpoint for deleting an entity.
///
/// # Example Request
///
/// ```http
/// DELETE /api/entities/019dd39a-5605-7743-b916-4067af05d0ef
/// ```
///
/// # Example Response
///
/// ```http
/// 200 OK
/// ```
#[delete("/<uuid>")]
pub async fn delete_entity(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    uuid: &str,
) -> Result<Status> {
    // If uuid is not valid, return 400 Bad Request.
    let id = Uuid::parse_str(uuid)?;
    let rows = EntityRecord::delete(state, &id).await?;

    // If no entity was affected, return 404 Not Found.
    if rows == 0 {
        return Ok(Status::NotFound);
    }

    // Delete entity recursively (delete from all components).
    for component in ComponentRecord::list_all(state).await? {
        let _ = component.remove_entity(general_state, &id).await;
    }

    Ok(Status::Ok)
}
