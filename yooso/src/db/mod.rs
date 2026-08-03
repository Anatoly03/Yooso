//! The database module of the Yooso infrastructure. This module provides the
//! interface to [sqlx].

pub mod entities;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateError};
use std::str::FromStr;

/// The general database state.
#[derive(Clone)]
pub struct Database {
    /// An asynchronous [Pool] to the main [Sqlite] database.
    pub pool: Pool<Sqlite>,
}

impl Database {
    /// Initializes an asynchronous [Pool] to the main [Sqlite] database.
    ///
    /// **Note.** In testing environment `#[cfg(test)]` this overwrites the Sqlite URL to be
    /// in-memory to ensure that every test gets its' own context.
    pub async fn init() -> Result<Self, sqlx::Error> {
        #[cfg(not(test))]
        const SQLITE_URL: &str = "sqlite://.yooso/data.db";
        #[cfg(test)]
        const SQLITE_URL: &str = "sqlite::memory:";

        let mut options = SqliteConnectOptions::from_str(SQLITE_URL).unwrap();

        if dotenvy::var("DATABASE_CREATE_MISSING").is_ok_and(|k| k != "0" && k != "false") {
            options = options.create_if_missing(true);
        }

        if let Ok(journal_mode_str) = dotenvy::var("DATABASE_JOURNAL_MODE") {
            let mode = journal_mode_str.parse::<SqliteJournalMode>().unwrap();
            options = options.journal_mode(mode)
        }

        let pool = SqlitePool::connect_with(options).await?;
        Ok(Self { pool })
    }

    /// Run migrations on the main database.
    pub async fn migrate(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("../migrations").run(&self.pool).await?;

        // TODO only print migrations which have been "currently" migrated by writing a custom migrator
        for migration in sqlx::migrate!("../migrations").iter() {
            println!("Migrate {} `{}`", migration.version, migration.description);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    /// This is a pair of two tests, the other is called [migrations]. This test verifies
    /// that a newly initialized database does not have any migrations.
    #[tokio::test]
    pub async fn no_migrations() {
        let database = Database::init().await.unwrap();

        let result: Result<(i64,), sqlx::Error> =
            sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
                .fetch_one(&database.pool)
                .await;

        // verify "_sqlx_migrations" does not exist
        assert!(result.is_err(), "`_sqlx_migrations` table should not exist");
    }

    /// This is a pair of two tests, the other is called [no_migrations]. This test verifies
    /// that after running migrations, there is no error and the total migration count is
    /// greater 1. (`create_tickets` migration exists)
    #[tokio::test]
    pub async fn migrations() {
        let database = Database::init().await.unwrap();
        database.migrate().await.unwrap();

        let result: Result<(i64,), sqlx::Error> =
            sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
                .fetch_one(&database.pool)
                .await;

        let total_migrations = sqlx::migrate!("../migrations").iter().len() as i64;

        // verify "_sqlx_migrations" exists
        assert!(result.is_ok(), "`_sqlx_migrations` table should be created");
        assert!(result.unwrap().0 == total_migrations, "there should be exactly {total_migrations} migration(s)");
    }
}
