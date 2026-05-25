//! Defines the component patching endpoint. A patch to a component is the edit of
//! the components metadata or the addition, update, or deletion of its fields.

use rocket::serde::json::Json;
use rocket::{State, patch};
use serde::{Deserialize, Serialize};
use util_validation::validate;
use uuid::Uuid;
use yooso_core::error::{Error, Result};
use yooso_storage::{ComponentFieldRecord, ComponentRecord, GeneralDBState, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchComponentRequest {
    pub id: Uuid,
    pub name: String,
    pub is_system: bool,
    pub color: u32,
    pub fields: Vec<PatchFieldRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum PatchFieldRequest {
    #[serde(rename = "add")]
    AddField {
        name: String,
        is_system: bool,
        field_type: String,
    },
    #[serde(rename = "update")]
    UpdateField {
        id: Uuid,
        name: String,
        is_system: bool,
        field_type: String,
    },
    #[serde(rename = "remove")]
    DeleteField { id: Uuid },
}

/// TODO: document
#[patch("/", data = "<body>")]
pub async fn update_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    body: Json<PatchComponentRequest>,
) -> Result<Json<ComponentRecord>> {
    let mut component = match ComponentRecord::view(state, &body.id).await {
        Ok(component) => component,
        Err(_) => return Err(Error::NotFound),
    };

    component.component_name = body.name.clone();
    component.is_system = body.is_system;
    component.color = body.color;

    let new_component = validate::<ComponentRecord, _>(component)?;
    new_component.save(state).await?;

    // Process Deletions
    for field_uuid in body.fields.iter().filter_map(|f| match f {
        PatchFieldRequest::DeleteField { id } => Some(*id),
        _ => None,
    }) {
        let _ = ComponentFieldRecord::delete_recursively(
            state,
            general_state,
            new_component.id,
            field_uuid,
        )
        .await;
    }

    // Process Updates
    for field_name in body.fields.iter().filter_map(|f| match f {
        PatchFieldRequest::UpdateField {
            // id,
            name,
            // is_system,
            // field_type,
            ..
        } => Some(name),
        _ => None,
    }) {
        todo!("add field `{field_name}`")
    }

    // Process Additions
    for field_name in body.fields.iter().filter_map(|f| match f {
        PatchFieldRequest::AddField { name, .. } => Some(name),
        _ => None,
    }) {
        todo!("add field `{field_name}`")
    }

    // // Process Additions
    // for field in body
    //     .fields
    //     .iter()
    //     .filter(|f| f.operation == PatchFieldOperation::Add)
    // {
    //     ComponentFieldRecord {
    //         id: Uuid::now_v7(),
    //         component_id: new_component.id,
    //         field_name: field.name.clone(),
    //         field_type: field.field_type.clone(),
    //         is_system: field.is_system,
    //         position: 0, // TODO: determine position
    //         created_at: chrono::Utc::now().timestamp_millis(),
    //     }
    //     .save(state)
    //     .await?;

    //     // Alter the table in the general database to add the new column for this field.
    //     let alter_table_query = sql_query_alter_table_add_column(&new_component, field.clone());
    //     general_state
    //         .0
    //         .lock()
    //         .expect("failed to acquire lock on general database")
    //         .execute(&alter_table_query, [])
    //         .expect("failed to alter component table in general database");
    // }

    Ok(Json(new_component.drop_validation()))
}

// /// Helper function to generate the SQL query for altering a component
// /// table based on the updated component and its fields.
// // TODO: use a proper SQL query builder library instead of string concatenation to
// // prevent SQL injection and handle edge cases.
// // https://database.guide/add-a-column-to-an-existing-table-in-sqlite/
// fn sql_query_alter_table_add_column(
//     component: &ComponentRecord,
//     field: PatchFieldRequest,
// ) -> String {
//     format!(
//         "ALTER TABLE {} ADD COLUMN {} {}",
//         component.component_name,
//         field.xname,
//         sql_type(&field.field_type)
//     )
// }

// /// Helper function to create appropriate SQL type for a given field type.
// /// Types in the project are high-level abstractions and need to be mapped
// /// to actual SQL types when generating
// fn sql_type(field_type: &str) -> &str {
//     match field_type {
//         "text" => "TEXT",
//         "integer" => "INT",
//         "boolean" => "BOOLEAN",
//         _ => panic!("unsupported field type: {}", field_type),
//     }
// }
