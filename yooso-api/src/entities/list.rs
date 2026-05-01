//! Defines the entity listing endpoint.

use futures::future::join_all;
use rocket::serde::json::Json;
use rocket::{State, get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yooso_core::Component;
use yooso_core::error::Result;
use yooso_storage::{ComponentTable, EntityTable, GeneralDBState, MetaDBState};

/// The response body for the entity listing endpoint.
/// 
/// # Example
/// 
/// ```json
/// {
///     "success": true,
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
    pub success: bool,
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
    pub components: Vec<Component>,
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
///     "success": true,
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
#[get("/list")]
pub async fn list_entities(
    state: &State<MetaDBState>,
    general_state: &State<GeneralDBState>,
) -> Result<Json<ComponentListResponse>> {
    let entities = EntityTable::list_all(state).await?;
    let component_tables = ComponentTable::list_all(state).await?;
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
                Ok(true) => Some(Component {
                    id: ct.id,
                    name: ct.component_name.clone(),
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
        success: true,
        entities: response_entities,
    }))
}
