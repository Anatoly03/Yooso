//! TODO: document

use rocket::serde::json::Json;
use rocket::{State, post};
use serde::{Deserialize, Serialize};
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

    // Create new Component
    let new_component = ComponentTable {
        id: uuid,
        component_name: body.name.clone(),
        is_system: body.is_system,
        color: body.color,
        created_at,
    };

    // Validate component metadata before saving to the database.
    new_component.validate()?;

    let new_fields = body
        .fields
        .iter()
        .enumerate()
        .map(|(position, field)| {
            // Convert minus to underscore in field name to make it a
            // valid SQL table name. We keep the `dash-case` convention for
            // the user interface, but use `snake_case` for the database.
            let field_name = field.name.replace('-', "_");

            ComponentFieldTable {
                id: uuid::Uuid::now_v7(),
                component_id: uuid,
                field_name,
                field_type: field.field_type.clone(),
                is_system: field.is_system,
                position: position as i32,
                created_at,
            }
        })
        .collect::<Vec<_>>();

    // Save component and fields to the metadata database.
    new_component.save(state).await;

    for field in &new_fields {
        field.save(state).await;
    }

    // Save component schema to the general database.
    let create_table_query = sql_query_create_table(&new_component, &new_fields);
    general_state
        .0
        .lock()
        .expect("failed to acquire lock on general database")
        .execute(&create_table_query, [])
        .expect("failed to create component table in general database");

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
        metadata: Component {
            id: new_component.id,
            name: new_component.component_name,
            is_system: new_component.is_system,
            color: new_component.color,
            created_at: new_component.created_at,
        },
        fields,
    }))
}

/// Helper function to generate the SQL query for creating a component table
/// based on the component and its fields.
// TODO: use a proper SQL query builder library instead of string concatenation to
// prevent SQL injection and handle edge cases.
fn sql_query_create_table(component: &ComponentTable, fields: &[ComponentFieldTable]) -> String {
    let mut sql_fields = vec!["entity_id UUID PRIMARY KEY".to_string()];

    sql_fields.extend(
        fields
            .iter()
            .map(|field| format!("{} {}", field.field_name, sql_type(&field.field_type))),
    );

    format!(
        "CREATE TABLE {} ({})",
        component.component_name,
        sql_fields.join(", ")
    )
}

/// Helper function to create appropriate SQL type for a given field type.
/// Types in the project are high-level abstractions and need to be mapped
/// to actual SQL types when generating
fn sql_type(field_type: &str) -> &str {
    match field_type {
        "text" => "TEXT",
        "integer" => "INT",
        "boolean" => "BOOLEAN",
        _ => panic!("unsupported field type: {}", field_type),
    }
}
