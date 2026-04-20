//! This module manages entity-component relations.
//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, post};
use uuid::Uuid;
use yooso_storage::{ComponentTable, EntityTable, GeneralDBState, MetaDBState};

/// TODO: document
#[post("/<id>/component/<component_id>", data = "<body>")]
pub async fn add_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
    component_id: &str,
    body: Json<Value>,
) -> Json<Value> {
    let entity_uuid = Uuid::parse_str(id)
        .map_err(|err| {
            json! ({
                "success": false,
                "error": format!("invalid entity UUID: {err}"),
            })
        })
        .unwrap();

    let component_uuid = Uuid::parse_str(component_id)
        .map_err(|err| {
            json! ({
                "success": false,
                "error": format!("invalid component UUID: {err}"),
            })
        })
        .unwrap();

    // Check that the entity exists.
    let _entity = EntityTable::view(state, &entity_uuid)
        .await
        .expect("failed to view entity");

    // Check that the component exists.
    let component = ComponentTable::view(state, &component_uuid)
        .await
        .expect("failed to view component");
    // let component_name = component.component_name;

    // Check component schema.
    let schema = component.schema(state).await;
    if schema.is_empty() {
        return Json(json! ({
            "success": false,
            "error": "component has no fields",
        }));
    }

    // Generate array of field names
    let field_names = {
        let mut v = vec!["entity_id"];

        for field in &schema {
            v.push(field.field_name.as_str());
        }

        v
    };

    // Generate array of field values.
    let field_values = {
        let mut v = vec![entity_uuid.to_string()];

        for field in &schema {
            let data = body.get(field.field_name.as_str()).ok_or_else(|| {
                json! ({
                    "success": false,
                    "error": format!("missing field: {}", field.field_name),
                })
            }).unwrap();

            match field.field_type.as_str() {
                "text" => {
                    let value = data.as_str().ok_or_else(|| {
                        json! ({
                            "success": false,
                            "error": format!("field {} should be a string", field.field_name),
                        })
                    }).unwrap();

                    v.push(format!("'{}'", value.replace("'", "''")));
                },
                "number" => {
                    let value = data.as_f64().ok_or_else(|| {
                        json! ({
                            "success": false,
                            "error": format!("field {} should be a number", field.field_name),
                        })
                    }).unwrap();

                    v.push(value.to_string());
                },
                "boolean" => {
                    let value = data.as_bool().ok_or_else(|| {
                        json! ({
                            "success": false,
                            "error": format!("field {} should be a boolean", field.field_name),
                        })
                    }).unwrap();

                    v.push(value.to_string());
                },
                _ => {
                    return Json(json! ({
                        "success": false,
                        "error": format!("unsupported field type: {}", field.field_type),
                    }));
                }
            }
        }

        v
    };

    // Create SQL query (insert row into component table with entity as key)
    // TODO refactor sql queries into storage layer
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        component.component_name, field_names.join(", "), field_values.join(", ")
    );

    // Execute query
    general_state.0.lock().unwrap().execute(&query, []).map_err(|err| {
        json! ({
            "success": false,
            "error": format!("failed to execute query: {err}"),
        })
    }).unwrap();

    Json(json!({}))
}
