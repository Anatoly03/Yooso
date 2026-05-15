//! Defines the component creation endpoint.

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
use util_validation::validate;
use yooso_core::error::Result;
use yooso_core::{Component, ComponentField};
use yooso_storage::{ComponentFieldTable, ComponentTable, GeneralDBState, MetaDBState};

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
    pub success: bool,
    pub metadata: Component,
    pub fields: Vec<ComponentField>,
}

/// TODO: document
#[post("/", data = "<body>")]
pub async fn create_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    body: Json<CreateComponentRequest>,
) -> Result<Json<CreateComponentResponse>> {
    let uuid = uuid::Uuid::now_v7();
    let created_at = chrono::Utc::now().timestamp_millis();

    let new_component = validate::<ComponentTable, _>(ComponentTable {
        id: uuid,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at,
    })
    .map_err(|e| yooso_core::error::Error::from(e))?;

    let (new_fields, errors) = body
        .fields
        .iter()
        .enumerate()
        .map(|(position, field)| {
            let field = validate::<ComponentFieldTable, _>(ComponentFieldTable {
                id: uuid::Uuid::now_v7(),
                component_id: uuid,
                field_name: field.name.clone(),
                field_type: field.field_type.clone(),
                is_system: field.is_system,
                position: position as i32,
                created_at,
            })
            .map_err(|e| yooso_core::error::Error::from(e))?;

            Ok(field)
        })
        .fold(
            (vec![], vec![]),
            |(mut fields, mut errors), field_result| {
                match field_result {
                    Ok(field) => fields.push(field),
                    Err(e) => errors.push(e),
                }
                (fields, errors)
            },
        );

    // If there are any validation errors, return the first one.
    if let Some(error) = errors.into_iter().next() {
        return Err(error);
    }

    // Save component and fields to the metadata database.
    new_component.save(state).await?;

    for field in &new_fields {
        field.save(state).await?;
    }

    // Save component schema to the general database.
    new_component
        .create_dynamic_table(general_state, &new_fields)
        .await?;

    // Return the created component and fields in the response.
    let fields = new_fields
        .iter()
        .map(|field| ComponentField {
            id: field.id,
            name: field.field_name.clone(),
            field_type: field.field_type.clone(),
            is_system: field.is_system,
            created_at: field.created_at,
        })
        .collect::<Vec<_>>();

    Ok(Json(CreateComponentResponse {
        success: true,
        metadata: Component {
            id: new_component.id,
            name: new_component.component_name.clone(),
            is_system: new_component.is_system,
            color: new_component.color,
            created_at: new_component.created_at,
        },
        fields,
    }))
}
