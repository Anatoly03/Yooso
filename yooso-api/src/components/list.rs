//! Defines the component listing endpoint.

use rocket::serde::json::{Json};
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use yooso_storage::{ComponentTable, MetaDBState};
use yooso_core::error::Result;

/// The response body for the component listing endpoint.
/// 
/// # Example
/// 
/// ```json
/// {
///     "success": true,
///     "components": [
///         {
///             "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///             "name": "user",
///             "is_system": false,
///             "color": 3772891647,
///             "created_at": 1776700466628
///         },
///         {
///             "id": "019dab4e-0ab5-7e70-b361-3eb561d53b8d",
///             "name": "superuser",
///             "is_system": false,
///             "color": 3787573503,
///             "created_at": 1776695511733
///         },
///         {
///             "id": "019dab54-479e-7332-b803-23b5dd3c304f",
///             "name": "magic",
///             "is_system": false,
///             "color": 12702177,
///             "created_at": 1776695920542
///         },
///         {
///             "id": "019dda35-8edd-7e93-8221-c031d8588af6",
///             "name": "message",
///             "is_system": false,
///             "color": 3452035583,
///             "created_at": 1777482436317
///         },
///         {
///             "id": "019de463-92cc-7fb0-8cfa-a1efd95f93ee",
///             "name": "wonder",
///             "is_system": false,
///             "color": 1704446463,
///             "created_at": 1777653224140
///         }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub success: bool,
    pub components: Vec<ComponentTable>,
}

/// The endpoint for listing components. This will retrieve all entities from
/// the database.
/// 
/// # Example Request
/// 
/// ```http
/// GET /api/components/list
/// ```
/// 
/// # Example Response
/// 
/// ```json
/// {
///     "success": true,
///     "components": [
///         {
///             "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///             "name": "user",
///             "is_system": false,
///             "color": 3772891647,
///             "created_at": 1776700466628
///         },
///         {
///             "id": "019dab4e-0ab5-7e70-b361-3eb561d53b8d",
///             "name": "superuser",
///             "is_system": false,
///             "color": 3787573503,
///             "created_at": 1776695511733
///         },
///         {
///             "id": "019dab54-479e-7332-b803-23b5dd3c304f",
///             "name": "magic",
///             "is_system": false,
///             "color": 12702177,
///             "created_at": 1776695920542
///         },
///         {
///             "id": "019dda35-8edd-7e93-8221-c031d8588af6",
///             "name": "message",
///             "is_system": false,
///             "color": 3452035583,
///             "created_at": 1777482436317
///         },
///         {
///             "id": "019de463-92cc-7fb0-8cfa-a1efd95f93ee",
///             "name": "wonder",
///             "is_system": false,
///             "color": 1704446463,
///             "created_at": 1777653224140
///         }
///     ]
/// }
/// ```
#[get("/list")]
pub async fn list_components(
    state: &State<MetaDBState>,
) -> Result<Json<ComponentListResponse>> {
    let components = ComponentTable::list_all(state).await?;

    Ok(Json(ComponentListResponse {
        success: true,
        components,
    }))
}
