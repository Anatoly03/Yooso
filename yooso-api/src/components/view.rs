//! Defines the component view endpoint.

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Error;
use yooso_core::error::Result;
use yooso_storage::{ComponentFieldRecord, ComponentRecord, MetaDBState};

/// Response structure for viewing a component. Contains the component object
/// metadata and fetches all fields associated with the component.
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewComponentResponse {
    pub metadata: ComponentRecord,
    pub fields: Vec<ComponentFieldRecord>,
}

/// Endpoint for viewing a component by its UUID.
///
/// # Example Request
///
/// ```http
/// GET /view/019e2bb7-1469-7d70-ba17-91117ee91c7a
/// ```
///
/// # Example Response
///
/// ```http
/// 200 OK
///
/// {
///   "metadata": {
///     "id": "019e2bb7-1469-7d70-ba17-91117ee91c7a",
///     "name": "Example Component",
///     "is_system": false,
///     "color": 12702177,
///     "created_at": 1778849879145,
///   },
///   "fields": [
///     {
///       "id": "019e2bb7-146b-7cc2-a011-9066dba138ad",
///       "name": "Example Field",
///       "field_type": "text",
///       "is_system": false,
///       "created_at": 1778849879145,
///     }
///   ]
/// }
/// ```
#[get("/view/<uuid>")]
pub async fn view_component(
    state: &State<MetaDBState>,
    uuid: &str,
) -> Result<Json<ViewComponentResponse>> {
    let id = Uuid::parse_str(uuid)?;
    let metadata = ComponentRecord::view(state, &id)
        .await
        .map_err(|_| Error::Code(Status::NotFound))?;
    let fields = ComponentFieldRecord::list_by_component_id(state, &metadata.id).await?;

    Ok(Json(ViewComponentResponse { metadata, fields }))
}
