//! This module manages entity-component relations.
//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, delete, post};
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
    let entity_uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("invalid entity UUID: {err}"),
            }));
        }
    };

    let component_uuid = match Uuid::parse_str(component_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("invalid component UUID: {err}"),
            }));
        }
    };

    // Check that the entity exists.
    if let Err(err) = EntityTable::view(state, &entity_uuid).await {
        return Json(json! ({
            "success": false,
            "error": format!("failed to view entity: {err}"),
        }));
    }

    // Check that the component exists.
    let component = match ComponentTable::view(state, &component_uuid).await {
        Ok(component) => component,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("failed to view component: {err}"),
            }));
        }
    };
    // let component_name = component.component_name;

    // Check component schema.
    let schema = component.schema(state).await;

    // Generate array of field names
    let field_names = {
        let mut v = vec!["entity_id".to_string()];

        for field in &schema {
            // Convert minus to underscore in field name to make it a
            // valid SQL table name. We keep the `dash-case` convention for
            // the user interface, but use `snake_case` for the database.
            let field_name = field.field_name.replace('-', "_");

            v.push(field_name);
        }

        v
    };

    // Generate array of field values.
    let field_values = {
        let mut v = vec![format!("'{}'", entity_uuid)];

        for field in &schema {
            // Convert underscore to minus in component name. (Convention
            // transformation between database and user interface).
            let name = field.field_name.replace('_', "-");

            let data = match body.get(name.as_str()) {
                Some(data) => data,
                None => {
                    return Json(json! ({
                        "success": false,
                        "error": format!("missing field: {}", name),
                    }));
                }
            };

            match field.field_type.as_str() {
                "text" => {
                    let value = match data.as_str() {
                        Some(value) => value,
                        None => {
                            return Json(json! ({
                                "success": false,
                                "error": format!("field {} should be a string", field.field_name),
                            }));
                        }
                    };

                    v.push(format!("'{}'", value.replace("'", "''")));
                },
                "number" | "integer" => {
                    let value = match data.as_f64() {
                        Some(value) => value,
                        None => {
                            return Json(json! ({
                                "success": false,
                                "error": format!("field {} should be a number", field.field_name),
                            }));
                        }
                    };

                    v.push(value.to_string());
                },
                "boolean" => {
                    let value = match data.as_bool() {
                        Some(value) => value,
                        None => {
                            return Json(json! ({
                                "success": false,
                                "error": format!("field {} should be a boolean", field.field_name),
                            }));
                        }
                    };

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
        "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
        component.component_name, field_names.join(", "), field_values.join(", ")
    );

    // Execute query
    let conn = match general_state.0.lock() {
        Ok(conn) => conn,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("failed to lock db: {err}"),
            }));
        }
    };

    if let Err(err) = conn.execute(&query, []) {
        return Json(json! ({
            "success": false,
            "error": format!("failed to execute query: {err}"),
        }));
    }

    Json(json!({ "success": true }))
}

/// TODO: document
#[delete("/<id>/component/<component_id>")]
pub async fn remove_component(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
    component_id: &str,
) -> Json<Value> {
    let entity_uuid = match Uuid::parse_str(id) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("invalid entity UUID: {err}"),
            }));
        }
    };

    let component_uuid = match Uuid::parse_str(component_id) {
        Ok(uuid) => uuid,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("invalid component UUID: {err}"),
            }));
        }
    };
    
    // Check that the component exists.
    let component = match ComponentTable::view(state, &component_uuid).await {
        Ok(component) => component,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("failed to view component: {err}"),
            }));
        }
    };
    // let component_name = component.component_name;

    
    // Create SQL query (insert row into component table with entity as key)
    // TODO refactor sql queries into storage layer
    let query = format!(
        "DELETE FROM {} WHERE entity_id = '{}'",
        component.component_name, entity_uuid
    );

    // Execute query
    let conn = match general_state.0.lock() {
        Ok(conn) => conn,
        Err(err) => {
            return Json(json! ({
                "success": false,
                "error": format!("failed to lock db: {err}"),
            }));
        }
    };

    if let Err(err) = conn.execute(&query, []) {
        return Json(json! ({
            "success": false,
            "error": format!("failed to execute query: {err}"),
        }));
    }

    Json(json!({ "success": true }))
}
