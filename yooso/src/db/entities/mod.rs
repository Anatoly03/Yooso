//! An entity is an atomic data point.
//!
//! In ECS, all data is constructed as a "two dimensional matrix" with the
//! "entities" mapped against "components". (This is purely for visual
//! purposes and not actually implemented as a matrix). Consider the example
//! below.
//!
//! | Entity | Username  | Age | Project | Message      | Author |
//! | ------ | --------- | --- | ------- | ------------ | ------ |
//! |      0 | "Anatoly" | 23  |         | "My status"  |        |
//! |      1 |           | 1   | "Yooso" | "Hello!"     |     ~0 |
//! |      2 |           |     |         | "My message" |     ~0 |
//! |      3 |           |     |         | "Bye!"       |     ~1 |
//!
//! In this table, the header row defines the "components" (with the exception
//! of "Entity") and all following rows are "entities". Instead of being schema-
//! oriented, Yooso goes for a data-driven design. Instead of creating a `user`
//! table, a `posts` table, you create components, and every entity can implement
//! every component.

pub mod api;
#[cfg(test)]
mod tests;

use crate::{InternalError, db::Database};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Error::RowNotFound;
use uuid::Uuid;

/// An entity is an atomic data point.
///
/// # Schema
///
/// | CID | Name | Type | Required | Primary |
/// | --- | ---- | ---- | :------: | :-----: |
/// | 0 | id | BLOB | ✓ | ✓ |
/// | 1 | created_at | DATETIME | ✓ | ✗ |
#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Entity {
    /// Snowflake value. This is the unique identifier of the entity.
    pub id: Uuid,

    /// The timestamp of when the entity was created, in seconds since the Unix
    /// epoch.
    pub created_at: DateTime<Utc>,
}

impl Entity {
    /// The name of the collection's table in the database. This is used
    /// for generating SQL queries and must be unique within the database.
    pub const TABLE_NAME: &str = "entities";

    /// Create a new entity record **without** saving it to the database.
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: DateTime::<Utc>::default(),
        }
    }

    /// Save the entity to the database.
    pub async fn push(&self, database: &Database) -> Result<(), InternalError> {
        let _: Option<()> =
            sqlx::query_as("INSERT OR REPLACE INTO entities(id, created_at) VALUES (?, ?)")
                .bind(self.id)
                .bind(self.created_at)
                .fetch_optional(&database.pool)
                .await?;
        Ok(())
    }

    /// Loads the stored database contents into [self].
    pub async fn pull(&mut self, database: &Database) -> Result<(), InternalError> {
        *self = Self::view(&database, self.id)
            .await?
            .ok_or(InternalError::Sqlx(RowNotFound))?;
        Ok(())
    }

    /// Delete the entity from the database.
    pub async fn delete(&self, database: &Database) -> Result<(), InternalError> {
        let _: Option<()> = sqlx::query_as("DELETE FROM entities WHERE id = ?")
            .bind(self.id)
            .fetch_optional(&database.pool)
            .await?;
        Ok(())
    }

    /// View the entity record by given id.
    pub async fn view(database: &Database, uuid: Uuid) -> Result<Option<Self>, InternalError> {
        let result = sqlx::query_as("SELECT id, created_at FROM entities WHERE id = ?")
            .bind(uuid)
            .fetch_optional(&database.pool)
            .await?
            .map(|result: (Uuid, DateTime<Utc>)| Self {
                id: result.0,
                created_at: result.1,
            });
        Ok(result)
    }

    /// Fetches all entities.
    pub async fn fetch_all(database: &Database) -> Result<Vec<Self>, InternalError> {
        let result: Vec<(Uuid, DateTime<Utc>)> =
            sqlx::query_as("SELECT id, created_at FROM entities")
                .fetch_all(&database.pool)
                .await?;
        let entities = result
            .into_iter()
            .map(|result: (Uuid, DateTime<Utc>)| Self {
                id: result.0,
                created_at: result.1,
            })
            .collect();
        Ok(entities)
    }
}

// impl IntoResponse for Entity {
//     fn into_response(self) -> axum::response::Response {
//         self.
//     }
// }
