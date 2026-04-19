//! TODO: document

use rocket::serde::json::{Json, Value, json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use yooso_core::Component;
use yooso_storage::{ComponentTable, MetaDBState};

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub success: bool,
    pub components: Vec<Component>,
}

/// TODO: document
/// This module defines the API endpoint for listing components in the Yooso application.
#[get("/list")]
pub async fn list_components(
    state: &State<MetaDBState>,
) -> Result<Json<ComponentListResponse>, Json<Value>> {
    ComponentTable::list_all_in_state(state)
        .await
        .map(|components| {
            Json(ComponentListResponse {
                success: true,
                components: components
                    .into_iter()
                    .map(|component| Component {
                        id: component.id,
                        name: component.name,
                        is_system: component.is_system,
                        color: component.color,
                        created_at: component.created_at,
                    })
                    .collect(),
            })
        })
        .map_err(|err| {
            Json(json!({
                "success": false,
                "message": format!("Failed to retrieve components: {}", err),
            }))
        })

    // // In a real implementation, this would query the database or application state
    // // to retrieve the list of components. Here we return a static response for demonstration.
    // let response = ComponentListResponse {
    //     success: true,
    //     components: vec![
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "Superuser".to_string(),
    //             system: false,
    //             color: 0xFF0000,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "EmailAuth".to_string(),
    //             system: false,
    //             color: 0x0000FF,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "PassAuth".to_string(),
    //             system: false,
    //             color: 0x0000FF,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "User".to_string(),
    //             system: false,
    //             color: 0x00FF00,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "Channel".to_string(),
    //             system: false,
    //             color: 0x800080,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "TextChannel".to_string(),
    //             system: false,
    //             color: 0x808080,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "VoiceChannel".to_string(),
    //             system: false,
    //             color: 0xFFA500,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "Message".to_string(),
    //             system: false,
    //             color: 0x800080,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "TextMessage".to_string(),
    //             system: false,
    //             color: 0x808080,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //         Component {
    //             id: Uuid::now_v7(),
    //             name: "MessageAttachments".to_string(),
    //             system: false,
    //             color: 0x00FFFF,
    //             created_at: chrono::Utc::now().timestamp_millis(),
    //         },
    //     ],
    // };

    // Ok(Json(response))
}
