//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, patch};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Component;
use yooso_storage::{ComponentFieldTable, ComponentTable, GeneralDBState, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchComponentRequest {
    pub id: Uuid,
    pub name: String,
    pub is_system: bool,
    pub color: u32,
    pub created_at: i64,
    pub fields: Vec<PatchFieldRequest>,
}

/// TODO: document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchFieldRequest {
    pub id: Option<Uuid>,
    pub name: String,
    pub is_system: bool,
    pub field_type: String,
    pub created_at: Option<i64>,
    pub operation: PatchFieldOperation,
}

/// TODO: document
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PatchFieldOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "remove")]
    Remove,
}

/// TODO: document
#[patch("/", data = "<body>")]
pub async fn update_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    body: Json<PatchComponentRequest>,
) -> Result<Json<Component>, Json<Value>> {
    // Convert minus to underscore in component name to make it a
    // valid SQL table name. We keep the `dash-case` convention for
    // the user interface, but use `snake_case` for the database.
    let component_name = body.name.replace('-', "_");

    let new_component = ComponentTable {
        id: body.id,
        component_name,
        is_system: body.is_system,
        color: body.color,
        created_at: body.created_at,
    };

    new_component.save(state).await;

    // Process Deletions
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Remove)
    {
        ComponentFieldTable {
            id: field.id.ok_or(json!({
                "error": "Invalid field ID"
            }))?,
            ..Default::default()
        }.delete(state).await;

        // Alter the table in the general database to drop the column for this field.
        let alter_table_query = sql_query_alter_table_drop_column(&new_component, field.clone());
        general_state
            .0
            .lock()
            .expect("failed to acquire lock on general database")
            .execute(&alter_table_query, [])
            .expect("failed to alter component table in general database");
    }

    // Process Updates
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Update)
    {
        // TODO
        println!(
            "TODO: update field `{}.{}`",
            new_component.component_name, field.name
        );
    }

    // Process Additions
    for field in body
        .fields
        .iter()
        .filter(|f| f.operation == PatchFieldOperation::Add)
    {
        ComponentFieldTable {
            id: Uuid::now_v7(),
            component_id: new_component.id,
            field_name: field.name.clone(),
            field_type: field.field_type.clone(),
            is_system: field.is_system,
            position: 0, // TODO: determine position
            created_at: chrono::Utc::now().timestamp_millis(),
        }.save(state).await;

        // Alter the table in the general database to add the new column for this field.
        let alter_table_query = sql_query_alter_table_add_column(&new_component, field.clone());
        general_state
            .0
            .lock()
            .expect("failed to acquire lock on general database")
            .execute(&alter_table_query, [])
            .expect("failed to alter component table in general database");
    }

    Ok(Json(Component {
        id: new_component.id,
        name: new_component.component_name,
        is_system: new_component.is_system,
        color: new_component.color,
        created_at: new_component.created_at,
    }))
}

/// Helper function to generate the SQL query for altering a component
/// table based on the updated component and its fields.
// TODO: use a proper SQL query builder library instead of string concatenation to
// prevent SQL injection and handle edge cases.
// https://database.guide/add-a-column-to-an-existing-table-in-sqlite/
fn sql_query_alter_table_add_column(component: &ComponentTable, field: PatchFieldRequest) -> String {
    format!(
        "ALTER TABLE {} ADD COLUMN {} {}",
        component.component_name,
        field.name,
        sql_type(&field.field_type)
    )
}

/// Helper function to generate the SQL query for altering a component
/// table based on the updated component and its fields.
// TODO: use a proper SQL query builder library instead of string concatenation to
// prevent SQL injection and handle edge cases.
// https://database.guide/add-a-column-to-an-existing-table-in-sqlite/
fn sql_query_alter_table_drop_column(component: &ComponentTable, field: PatchFieldRequest) -> String {
    format!(
        "ALTER TABLE {} DROP COLUMN {}",
        component.component_name,
        field.name
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
