//! Defines the component listing endpoint.

use rocket::serde::json::Json;
use rocket::{State, get};
use serde::Serialize;
use yooso_core::Result;
use yooso_storage::{ComponentRecord, MetaDBState, Pagination};

/// The response body for the component listing endpoint.
///
/// # Example
///
/// ```json
/// {
///     "page": 1,
///     "per_page": 25,
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
#[derive(Debug, Serialize)]
pub struct ComponentListResponse {
    pub page: usize,
    pub per_page: usize,
    pub components: Vec<ComponentRecord>,
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
///     "page": 1,
///     "per_page": 25,
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
#[get("/list?<per_page>&<page>")]
pub async fn list_components(
    state: &State<MetaDBState>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Result<Json<ComponentListResponse>> {
    // Validate pagination parameters or prefer defaults.
    let pagination = Pagination {
        page: (page.unwrap_or(1) as usize).max(1), // ensure page is at least 1
        per_page: (per_page.unwrap_or(25) as usize).min(100), // cap per_page at 100 to prevent abuse
    };

    // Fetch components in the pagination range.
    let components = ComponentRecord::list(state, pagination.per_page, pagination.page).await?;

    // Return the response with used pagination parameters.
    Ok(Json(ComponentListResponse {
        page: pagination.page,
        per_page: pagination.per_page,
        components,
    }))
}
