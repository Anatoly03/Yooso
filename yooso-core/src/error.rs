//! This module defines the general Yooso error type.

use std::sync::PoisonError;

/// General error type for Yooso, which is used in database management as
/// well as API-level handlers.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error originating from the Rusqlite database layer.
    RusqliteError(::rusqlite::Error),

    /// An error originating from a poisoned mutex. We store a short
    /// diagnostic string (the `PoisonError` does not expose a useful
    /// typed payload here across guard types), and provide conversions
    /// from `PoisonError<T>`.
    MutexPoisoned(String),

    /// An error originating from the Uuid library. This typically occurs
    /// during Uuid parsing.
    UuidError(uuid::Error),
}

/// A typedef of the result returned by many methods.
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<::rusqlite::Error> for Error {
    fn from(err: ::rusqlite::Error) -> Self {
        Error::RusqliteError(err)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Error::MutexPoisoned(format!("{}", err))
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error::UuidError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RusqliteError(e) => write!(f, "rusqlite error: {}", e),
            Error::MutexPoisoned(s) => write!(f, "mutex poisoned: {}", s),
            Error::UuidError(e) => write!(f, "uuid error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::RusqliteError(e) => Some(e),
            Error::UuidError(e) => Some(e),
            Error::MutexPoisoned(_) => None,
        }
    }
}
