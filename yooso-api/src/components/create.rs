//! Defines the component creation endpoint.

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use util_validation::{Validated, ValidationError, validate};
use uuid::Uuid;
use yooso_core::error::Result;
use yooso_storage::{ComponentFieldRecord, ComponentRecord, GeneralDBState, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentRequest {
    pub name: String,
    pub is_system: bool,
    pub fields: Vec<CreateComponentField>,
    pub color: u32,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentField {
    pub name: String,
    pub is_system: bool,
    pub field_type: String,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComponentResponse {
    pub metadata: Validated<ComponentRecord>,
    pub fields: Vec<Validated<ComponentFieldRecord>>,
}

/// The endpoint for creating a component.
///
/// # Example Request
///
/// ```http
/// POST /api/components
///
/// {
///     "name": "MessageContent",
///     "is_system": false,
///     "color": 0xFF5733,
///     "fields": [
///          {
///              "name": "content",
///              "is_system": false,
///              "field_type": "text"
///          }
///     ]
/// }
/// ```
///
/// # Example Response
///
/// ```http
///   201 Created
/// 
/// ```
/// ```diff
///   {
/// +     "id": "019de463-92cc-7fb0-8cfa-a1efd95f93ee",
///       "name": "MessageContent",
///       "is_system": false,
///       "color": 0xFF5733,
/// +     "created_at": 1777653224140,
///       "fields": [
///            {
/// +              "id": "019dda35-8edd-7e93-8221-c031d8588af6",
///                "name": "content",
///                "is_system": false,
///                "field_type": "text",
/// +              "position": 0,
/// +              "created_at": 1777482436317,
///            }
///       ]
///   }
/// ```
#[post("/", data = "<body>")]
pub async fn create_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    body: Json<CreateComponentRequest>,
) -> Result<(Status, Json<CreateComponentResponse>)> {
    let id = Uuid::now_v7();
    let created_at = chrono::Utc::now().timestamp_millis();

    // Validate the component from the request body.
    let metadata = validate::<ComponentRecord, _>(ComponentRecord {
        id,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at,
    })?;

    // Validate the component fields from the request body.
    let fields = body
        .fields
        .iter()
        .enumerate()
        .map(|(position, field)| {
            validate::<ComponentFieldRecord, _>(ComponentFieldRecord {
                id: Uuid::now_v7(),
                component_id: id,
                field_name: field.name.clone(),
                field_type: field.field_type.clone(),
                is_system: field.is_system,
                position: position as i32,
                created_at,
            })
        })
        .collect::<std::result::Result<Vec<_>, ValidationError>>()?;

    // Assume everything has been validated here. Save component and fields
    // to the metadata database.
    metadata.save(state).await?;
    for field in &fields {
        field.save(state).await?;
    }

    // Create the component table in the general database. Entities will implement
    // the component by inserting rows into this table.
    metadata
        .create_dynamic_table(general_state, &fields)
        .await?;

    Ok((Status::Created, Json(CreateComponentResponse { metadata, fields })))
}
