//! This module defines the general Yooso error type.

use rocket::Request;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::{self, Responder};
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

    /// An error originating from invalid input data, such as as invalid
    /// component name or field metadata.
    ValidationError(::util_validation::ValidationError),

    /// A generic error type unwrapping into a [rocket] [Status] code.
    Code(Status),
}

impl Error {
    /// A constant for the 404 Not Found error code, which is commonly used across
    /// API handlers.
    #[allow(non_upper_case_globals)]
    pub const NotFound: Self = Self::Code(Status::NotFound);
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

impl From<::util_validation::ValidationError> for Error {
    fn from(err: ::util_validation::ValidationError) -> Self {
        Error::ValidationError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RusqliteError(e) => write!(f, "rusqlite error: {}", e),
            Error::MutexPoisoned(s) => write!(f, "mutex poisoned: {}", s),
            Error::UuidError(e) => write!(f, "uuid error: {}", e),
            Error::ValidationError(s) => write!(f, "validation error: {}", s),
            Error::Code(status) => write!(f, "http: {status}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::RusqliteError(e) => Some(e),
            Error::UuidError(e) => Some(e),
            Error::MutexPoisoned(_) => None,
            Error::ValidationError(_) => None,
            Error::Code(_) => None,
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        match &self {
            // For validation errors, we return a 400 Bad Request with the error
            // message in the body. The error message is safe to expose to the client
            // since it is directly related to the clients' input.
            Error::ValidationError(e) => Custom(Status::BadRequest, e.to_string()).respond_to(req),

            // Uuid parsing errors typically occur when the client provides an invalid
            // UUID string. We return a 400 Bad Request with a generic error message since
            // the error is related to czlient input.
            Error::UuidError(_) => {
                Custom(Status::BadRequest, "Invalid UUID format".to_string()).respond_to(req)
            }
            // For http-coded errors, we return the specified status code with no body. The error message
            // is the http status message.
            Error::Code(code) => match code.reason() {
                Some(reason) => Custom(*code, reason.to_string()).respond_to(req),
                _ => Custom(
                    Status::InternalServerError,
                    "Internal server error".to_string(),
                )
                .respond_to(req),
            },
            // Return a generic 500 error for all other error types and hide error
            // message from the client. (It could be sensitive information helping
            // an attacker to exploit the server)
            _ => Custom(
                Status::InternalServerError,
                "Internal server error".to_string(),
            )
            .respond_to(req),
        }
    }
}
