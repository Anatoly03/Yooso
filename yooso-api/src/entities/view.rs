//! Defines the entity viewing endpoint.

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{ComponentRecord, EntityRecord, GeneralDBState, MetaDBState};

/// The response for viewing an new entity. This will yield entity metadata and all
/// of the components that belong to this entity. The "components" field is a JSON
/// object where the component name is mapped as key to the component field values.
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewEntityResponse {
    pub id: Uuid,
    pub created_at: i64,
    pub components: Value,
}

/// The endpoint for viewing an new entity.
///
/// # Example Request
///
/// ```http
/// GET /api/entities/view/019e2bb7-1fc2-7e73-9df5-5a74537e0bcd
/// ```
///
/// # Example Response
///
/// ```http
/// 200 OK
///
/// {
///   "id": "019e2bb7-1fc2-7e73-9df5-5a74537e0bcd",
///   "created_at": 1778849882050,
///   "components": {
///     "new_component": {
///       "field_0": "value1",
///       "field_1": 123,
///       "field_2": true
///     }
///   }
/// }
/// ```
#[get("/view/<id>")]
pub async fn view_entity(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
) -> Result<Json<ViewEntityResponse>> {
    let uuid = Uuid::parse_str(id)?;
    let entity = EntityRecord::view(state, &uuid).await?;

    // Scan every possible component and retrieve the fields for this entity.
    let component_tables = ComponentRecord::list_all(state).await?;
    let mut matrix = json!({});

    for component in component_tables {
        match component.for_entity(state, general_state, &uuid).await {
            Ok(field) => matrix[component.component_name] = field,
            _ => continue,
        }
    }

    Ok(Json(ViewEntityResponse {
        id: uuid,
        created_at: entity.created_at,
        components: matrix,
    }))
}
