//! Defines the entity deletion endpoint.

use rocket::{State, delete};
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
) -> Result<()> {
    let id = Uuid::parse_str(&uuid)?;
    EntityRecord::delete(state, id).await?;

    for component in ComponentRecord::list_all(state).await? {
        component.remove_entity(&general_state, &id).await?;
    }

    Ok(())
}
