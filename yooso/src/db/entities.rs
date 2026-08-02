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

use chrono::{DateTime, Utc};
use sqlx::Error::RowNotFound;
use uuid::Uuid;

use crate::{InternalError, db::Database};

/// An entity is an atomic data point.
///
/// # Schema
///
/// | CID | Name | Type | Required | Primary |
/// | --- | ---- | ---- | :------: | :-----: |
/// | 0 | id | BLOB | ✓ | ✓ |
/// | 1 | created_at | DATETIME | ✓ | ✗ |
#[derive(Clone, Copy)]
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

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::db::{Database, entities::Entity};

    /// This test verifies that [Entity::new] should not save to the database.
    #[tokio::test]
    pub async fn entity_new() {
        let database = Database::init().await.unwrap();
        assert!(
            database.migrate().await.is_ok(),
            "database migration failed"
        );

        let entity = Entity::new();

        assert!(
            Entity::view(&database, entity.id).await.unwrap().is_none(),
            "entity exists in the database, but was not saved yet"
        );
    }

    /// This test verifies that general entity-saving works: [Entity::new] creates
    /// a new entity, then [Entity::save] saves the entity to the database.
    #[tokio::test]
    pub async fn entity_save() {
        let database = Database::init().await.unwrap();
        assert!(
            database.migrate().await.is_ok(),
            "database migration failed"
        );

        let entity = Entity::new();
        entity.push(&database).await.unwrap();

        assert!(
            Entity::view(&database, entity.id).await.unwrap().is_some(),
            "entity does not exist in the database"
        );
    }

    /// This test verifies that general entity-fetching works.
    #[tokio::test]
    pub async fn entity_fetch() {
        let database = Database::init().await.unwrap();
        assert!(
            database.migrate().await.is_ok(),
            "database migration failed"
        );

        assert_eq!(
            Entity::fetch_all(&database).await.unwrap().len(),
            0,
            "there should be exactly 0 entities in the table"
        );

        let entity1 = Entity::new();
        entity1.push(&database).await.unwrap();
        
        assert_eq!(
            Entity::fetch_all(&database).await.unwrap().len(),
            1,
            "there should be exactly 1 entity in the table"
        );

        let entity2 = Entity::new();
        entity2.push(&database).await.unwrap();

        assert_eq!(
            Entity::fetch_all(&database).await.unwrap().len(),
            2,
            "there should be exactly 2 entities in the table"
        );

    }

    /// This test verifies that an entity removed with [Entity::delete] is
    /// removed from the database.
    #[tokio::test]
    pub async fn entity_delete() {
        let database = Database::init().await.unwrap();
        assert!(
            database.migrate().await.is_ok(),
            "database migration failed"
        );

        let entity = Entity::new();
        entity.push(&database).await.unwrap();
        entity.delete(&database).await.unwrap();

        assert!(
            Entity::view(&database, entity.id).await.unwrap().is_none(),
            "entity exists in the database, but should have been deleted"
        );
    }
}
