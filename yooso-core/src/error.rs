//! This module defines the general Yooso error type.

mod impls;

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
    UuidError(::uuid::Error),

    /// An error originating from invalid input data, such as as invalid
    /// component name or field metadata.
    ValidationError(::util_validation::ValidationError),

    /// A constant for the `404 Not Found`` error code, which is commonly used across
    /// API handlers.
    NotFound,

    // /// A generic error type unwrapping into a [rocket] [Status] code.
    // Code(Status),
}

/// A typedef of the result returned by Yooso functions.
pub type Result<T, E = Error> = std::result::Result<T, E>;
