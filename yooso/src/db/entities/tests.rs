//! Entity-related unit tests.

use super::{Database, Entity};
use std::assert_eq;

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
