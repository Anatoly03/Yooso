//! The database module of the Yooso infrastructure. This module provides the
//! interface to [sqlx].

use sqlx::{
    Pool, Sqlite, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::str::FromStr;

/// Initializes an asynchronous [Pool] to the main [Sqlite] database.
///
/// **Note.** In testing environment `#[cfg(test)]` this overwrites the Sqlite URL to be
/// in-memory to ensure that every test gets its' own context.
pub async fn init() -> Result<Pool<Sqlite>, sqlx::Error> {
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

    SqlitePool::connect_with(options).await
}
