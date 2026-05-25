//! This module manages entity-component relations.
//! TODO: document

use rocket::serde::json::{Json, Value};
use rocket::{State, post};
use util_validation::ValidationError;
use uuid::Uuid;
use yooso_core::Error;
use yooso_core::Result;
use yooso_storage::{ComponentRecord, EntityRecord, GeneralDBState, MetaDBState};

/// TODO: document
#[post("/<id>/component/<component_id>", data = "<body>")]
pub async fn add_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
    component_id: &str,
    body: Json<Value>,
) -> Result<()> {
    // If uuid is not valid, return 400 Bad Request.
    let entity_uuid = Uuid::parse_str(id)?;
    let component_uuid = Uuid::parse_str(component_id)?;

    // check that the entity and the component exist, return 404 if it doesn't
    EntityRecord::view(state, &entity_uuid)
        .await
        .map_err(|_| Error::NotFound)?;
    let component = ComponentRecord::view(state, &component_uuid)
        .await
        .map_err(|_| Error::NotFound)?;

    // Check component schema.
    let schema = component.schema(state).await?;

    // Generate array of field names
    let field_names = {
        let mut v = vec!["entity_id".to_string()];

        for field in &schema {
            v.push(field.field_name.clone());
        }

        v
    };

    // Generate array of field values.
    let field_values = {
        let mut v = vec![format!("'{}'", entity_uuid)];

        for field in &schema {
            let name = field.field_name.clone();

            let data = body
                .get(name.as_str())
                .ok_or(ValidationError::new(format!("missing field: {}", name)))?;

            match field.field_type.as_str() {
                "text" => {
                    let value = data.as_str().ok_or(ValidationError::new(format!(
                        "field {} should be a string",
                        field.field_name
                    )))?;
                    v.push(format!("'{}'", value.replace("'", "''")));
                }
                "number" | "integer" => {
                    let value = data.as_f64().ok_or(ValidationError::new(format!(
                        "field {} should be a number",
                        field.field_name
                    )))?;
                    v.push(value.to_string());
                }
                "boolean" => {
                    let value = data.as_bool().ok_or(ValidationError::new(format!(
                        "field {} should be a boolean",
                        field.field_name
                    )))?;
                    v.push(value.to_string());
                }
                ft => {
                    return Err(ValidationError::new(format!("unknown field type: {ft}")))?;
                }
            }
        }

        v
    };

    // Create SQL query (insert row into component table with entity as key)
    // TODO refactor sql queries into storage layer
    let query = format!(
        "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
        component.component_name,
        field_names.join(", "),
        field_values.join(", ")
    );

    // Execute query
    let conn = general_state.0.lock()?;
    conn.execute(&query, [])?;

    Ok(())
}
