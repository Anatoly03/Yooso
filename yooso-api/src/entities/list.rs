//! Defines the entity listing endpoint.

use futures::future::join_all;
use rocket::serde::json::Json;
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Result;
use yooso_macro::docapi;
use yooso_storage::{ComponentRecord, EntityRecord, GeneralDBState, MetaDBState, Pagination};

/// The response body for the entity listing endpoint.
///
/// # Example
///
/// ```json
/// {
///     "entities": [
///         {
///             "id": "019dab4e-4cee-7071-80a6-6cb89e5ee03c",
///             "created_at": 1776695528686,
///             "components": [
///                 {
///                     "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///                     "name": "user",
///                     "is_system": false,
///                     "color": 3772891647,
///                     "created_at": 1776700466628
///                 },
///                 {
///                     "id": "019dab4e-0ab5-7e70-b361-3eb561d53b8d",
///                     "name": "superuser",
///                     "is_system": false,
///                     "color": 3787573503,
///                     "created_at": 1776695511733
///                 }
///             ]
///         },
///         {
///             "id": "019dab9a-7863-7823-a92d-51659c8f57ec",
///             "created_at": 1776700520547,
///             "components": [
///                 {
///                     "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///                     "name": "user",
///                     "is_system": false,
///                     "color": 3772891647,
///                     "created_at": 1776700466628
///                 }
///             ]
///         },
///         {
///             "id": "019dabb1-39e6-7151-9c5b-b7b6175ebb84",
///             "created_at": 1776702011878,
///             "components": []
///         },
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentListResponse {
    pub page: usize,
    pub per_page: usize,
    pub entities: Vec<EntityResponse>,
}

/// Represents a single entity in the response of the entity listing endpoint.
/// Contains the entity's ID, creation timestamp, and a list of its components.
///
/// # Example
///
/// ```json
/// {
///     "id": "019dab9a-7863-7823-a92d-51659c8f57ec",
///     "created_at": 1776700520547,
///     "components": [
///         {
///             "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///             "name": "user",
///             "is_system": false,
///             "color": 3772891647,
///             "created_at": 1776700466628
///         }
///     ]
/// },
/// ```
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EntityResponse {
    pub id: Uuid,
    pub created_at: i64,
    pub components: Vec<ComponentRecord>,
}

/// The endpoint for listing entities. This will retrieve all entities from
/// the database.
///
/// # Example Request
///
/// ```http
/// GET /api/entities/list
/// ```
///
/// # Example Response
///
/// ```json
/// {
///     "page": 1,
///     "per_page": 25,
///     "entities": [
///         {
///             "id": "019dab4e-4cee-7071-80a6-6cb89e5ee03c",
///             "created_at": 1776695528686,
///             "components": [
///                 {
///                     "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///                     "name": "user",
///                     "is_system": false,
///                     "color": 3772891647,
///                     "created_at": 1776700466628
///                 },
///                 {
///                     "id": "019dab4e-0ab5-7e70-b361-3eb561d53b8d",
///                     "name": "superuser",
///                     "is_system": false,
///                     "color": 3787573503,
///                     "created_at": 1776695511733
///                 }
///             ]
///         },
///         {
///             "id": "019dab9a-7863-7823-a92d-51659c8f57ec",
///             "created_at": 1776700520547,
///             "components": [
///                 {
///                     "id": "019dab99-a5c4-7b50-a748-3152cacaa0b5",
///                     "name": "user",
///                     "is_system": false,
///                     "color": 3772891647,
///                     "created_at": 1776700466628
///                 }
///             ]
///         },
///         {
///             "id": "019dabb1-39e6-7151-9c5b-b7b6175ebb84",
///             "created_at": 1776702011878,
///             "components": []
///         },
///     ]
/// }
/// ```
#[docapi()]
#[get("/api/entities/list?<per_page>&<page>")]
pub async fn list_entities(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
    per_page: Option<u32>,
    page: Option<u32>,
) -> Result<Json<ComponentListResponse>> {
    let pagination = Pagination {
        page: (page.unwrap_or(1) as usize).max(1), // ensure page is at least 1
        per_page: (per_page.unwrap_or(25) as usize).min(100), // cap per_page at 100 to prevent abuse
    };
    let entities = EntityRecord::list(state, pagination.per_page, pagination.page).await?;

    let component_tables = ComponentRecord::list_all(state).await?;
    let mut response_entities = Vec::with_capacity(entities.len());

    for entity in entities {
        let futures = component_tables
            .iter()
            .map(|ct| ct.defined_for(general_state, &entity.id))
            .collect::<Vec<_>>();
        let components = join_all(futures)
            .await
            .iter()
            .zip(component_tables.iter())
            .filter_map(|(exists, ct)| match exists {
                Ok(true) => Some(ComponentRecord {
                    id: ct.id,
                    component_name: ct.component_name.clone(),
                    is_system: ct.is_system,
                    color: ct.color,
                    created_at: ct.created_at,
                }),
                _ => None,
            })
            .collect();

        response_entities.push(EntityResponse {
            id: entity.id,
            created_at: entity.created_at,
            components,
        });
    }

    Ok(Json(ComponentListResponse {
        page: pagination.page,
        per_page: pagination.per_page,
        entities: response_entities,
    }))
}
