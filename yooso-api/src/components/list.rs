//! TODO: document

use rocket::get;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Component;

/// TODO: document
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub success: bool,
    pub components: Vec<Component>,
}

/// TODO: move to general response type
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonErrorResponse {
    pub success: bool, // always false
    pub message: String,
}

/// TODO: document
/// This module defines the API endpoint for listing components in the Yooso application.
#[get("/list")]
pub async fn list_components() -> Result<Json<ComponentListResponse>, Json<Value>> {
    // In a real implementation, this would query the database or application state
    // to retrieve the list of components. Here we return a static response for demonstration.
    let response = ComponentListResponse {
        success: true,
        components: vec![
            Component {
                id: Uuid::now_v7(),
                name: "Superuser".to_string(),
                system: false,
                color: 0xFF0000,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "EmailAuth".to_string(),
                system: false,
                color: 0x0000FF,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "PassAuth".to_string(),
                system: false,
                color: 0x0000FF,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "User".to_string(),
                system: false,
                color: 0x00FF00,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "Channel".to_string(),
                system: false,
                color: 0x800080,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "TextChannel".to_string(),
                system: false,
                color: 0x808080,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "VoiceChannel".to_string(),
                system: false,
                color: 0xFFA500,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "Message".to_string(),
                system: false,
                color: 0x800080,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "TextMessage".to_string(),
                system: false,
                color: 0x808080,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
            Component {
                id: Uuid::now_v7(),
                name: "MessageAttachments".to_string(),
                system: false,
                color: 0x00FFFF,
                created_at: chrono::Utc::now().timestamp_millis(),
            },
        ],
    };

    Ok(Json(response))
}
