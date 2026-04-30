//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_storage::{ComponentTable, GeneralDBState, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ViewEntityResponse {
    pub id: Uuid,
    // pub created_at: i64,
    pub components: Value,
}

/// TODO: document
#[get("/view/<id>")]
pub async fn view_entity(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    id: &str,
) -> Result<Json<ViewEntityResponse>, Json<Value>> {
    let uuid = Uuid::parse_str(id).map_err(|err| {
        json! ({
            "success": false,
            "error": format!("invalid UUID: {err}"),
        })
    })?;

    // Scan every component table in the meta database to find all
    // components that belong to this entity.
    let component_tables = ComponentTable::list_all(state)
        .await
        .expect("failed to list components");

    let entity_component_matrix = {
        let mut v = vec![];

        for component in component_tables {
            let fields_opt = component.for_entity(state, general_state, &uuid)
                .await;

            v.push((component, fields_opt));
        }

        v
    };

    // Generate the JSON object that we return. The keys of the "components" field
    // are the component names, and the values are the JSON objects that we get from
    // the operatiton above.
    let components = {
        let mut obj = json!({});

        for (component, fields_opt) in entity_component_matrix {
            if let Ok(fields) = fields_opt {
                obj[component.component_name] = fields;
            }
        }

        obj
    };

    Ok(Json(ViewEntityResponse {
        id: uuid,
        components,
    }))
}
