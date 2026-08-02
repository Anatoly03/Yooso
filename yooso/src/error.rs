use std::fmt::Debug;

/// Represents all the ways a method can fail in a Yooso application.
///
/// This is state-internal and response-wise represents status code 500. For
/// errors that can originate from a user request, see [InputError].
#[derive(Debug)]
pub enum InternalError {
    /// The error type for I/O operations of the [`Read`], [`Write`], [`Seek`],
    /// and associated traits.
    ///
    /// [`Read`]: std::io::Read
    /// [`Write`]: std::io::Write
    /// [`Seek`]: std::io::Seek
    Io(std::io::Error),

    /// Represents all the ways a method can fail within SQLx.
    Sqlx(sqlx::error::Error),

    /// Represents an error while resolving or executing SQL migrations.
    SqlxMigrate(sqlx::migrate::MigrateError),
}

/// Represents all the ways a method can fail from a request.
///
/// Response-wise this represents status codes in the range of 400 to 500. The
/// error messages are **public** and will be visible by the end user. If the
/// error message itself exposes a vulnerability, then use [InternalError] as
/// a separate struct or the enum variant [InputError::InternalError].
#[derive(Debug)]
pub enum InputError {
    InternalError(InternalError),
}

// Trait Implementations.

impl From<std::io::Error> for InternalError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::io::Error> for InputError {
    fn from(value: std::io::Error) -> Self {
        Self::InternalError(InternalError::Io(value))
    }
}

impl From<sqlx::Error> for InternalError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<sqlx::Error> for InputError {
    fn from(value: sqlx::Error) -> Self {
        Self::InternalError(InternalError::Sqlx(value))
    }
}

impl From<sqlx::migrate::MigrateError> for InternalError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::SqlxMigrate(value)
    }
}
