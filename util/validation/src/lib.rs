//! A utility crate for securing code logic with unknown values and ensuring validation
//! of data.
//!
//! # Example
//!
//! ```
//! use util_validation::{Validated, ValidationError, ValidateFrom};
//!
//! pub struct RgbColor { red: u8, green: u8, blue: u8 }
//!
//! impl ValidateFrom<&str> for RgbColor {
//!     fn validate(input: &str) -> Result<Validated<Self>, ValidationError> {
//!         // has to start with '#' and be followed by 6 hex digits
//!         if !input.starts_with('#') || input.len() != 7 {
//!             return Err(ValidationError::new("Invalid RGB color format"));
//!         }
//!
//!         // Parse the hex values
//!         let red = u8::from_str_radix(&input[1..3], 16).map_err(|_| ValidationError::new("Invalid red component"))?;
//!         let green = u8::from_str_radix(&input[3..5], 16).map_err(|_| ValidationError::new("Invalid green component"))?;
//!         let blue = u8::from_str_radix(&input[5..7], 16).map_err(|_| ValidationError::new("Invalid blue component"))?;
//!         Ok(Validated(RgbColor { red, green, blue }))
//!     }
//! }
//!
//! fn main() {
//!     let color_str = "#FF5733";
//!     let invalid_color_str = "FF5733";
//!
//!     assert!(RgbColor::validate(color_str).is_ok());
//!     assert!(RgbColor::validate(invalid_color_str).is_err());
//! }
//! ```
//!
//! # Features
//!
//! - `util`: Enables utility functions for validation, such as common validation patterns
//!   and helper functions.
//! - `uuid`: Enables validation for UUID types. This allows you to validate UUIDs from
//!   strings and ensure they are in the correct format.
//! 
//! # Semantics
//! 
//! Error messages should be short, concise and contextless. The context can be prepended
//! along the stack of validation calls. The final error messages grows from the right to
//! the left.
//! 
//! ```text
//! Error validating input: Component Name: Value must not be the SQL keyword `select`
//! ```
//! 
//! For example the method [not_sql_keyword] does not need to know the field name, and
//! speaks with words like "value" instead of specific field names. The context should be
//! added by the caller.

use std::result::Result;
use std::{fmt::Display, ops::Deref};

/// A wrapper type that indicates that the inner value has been validated. This is
/// used to ensure code logic that requires validated data can only be called with
/// prior validation. Validated data is immutable, but the valid state can be dropped.
pub struct Validated<T>(pub T);

/// An error type for validation errors. This can be used to provide more context about
/// why an input validation fails.
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
}

impl ValidationError {
    /// Creates a new [ValidationError] origin with the given message.
    pub fn new<K: std::fmt::Display>(message: K) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Prepends additional context to the error message. This can be useful when propagating
    /// validation errors through multiple layers of code.
    pub fn prepend<K: std::fmt::Display>(mut self, prefix: K) -> Self {
        self.message = format!("{}: {}", prefix, self.message);
        self
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input: {}", self.message)
    }
}

/// Implements the deref trait to allow access to the inner value without
/// unwrapping.
impl<T> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Validated<T> {
    /// Creates a new Validated wrapper around the given value. This is used to
    /// indicate that the value has been validated and can be safely used in
    /// code logic that requires validation.
    ///
    /// This is a private method and should only be called from within the crate, as it
    /// assumes that the caller has already performed the necessary validation checks before
    /// creating the Validated wrapper.
    pub(crate) fn new(value: T) -> Self {
        Self(value)
    }

    /// Drops the valid state and returns the inner value. This allows to mutate
    /// the value under the assumption that it is no longer valid.
    pub fn drop_validation(self) -> T {
        self.0
    }
}

/// A trait for types that can be validated. The `validate` method should return a
/// [Validated] wrapper around the original type
pub trait ValidateFrom<K>
where
    Self: Sized,
{
    /// Consumes the input and attempts to validate it, returning a [Validated] wrapper
    /// around the original and unlocking code logic that requires the valid state.
    fn validate(input: K) -> Result<Validated<Self>, ValidationError>;
}

/// A helper function to validate an input value using the [ValidateFrom] trait. This
/// is a convenient way to perform validation without having to directly call the trait
/// method.
pub fn validate<T, K>(input: K) -> Result<Validated<T>, ValidationError>
where
    T: ValidateFrom<K>,
{
    T::validate(input)
}

/// A blanket implementation for types that can be validated from themselves. For
/// example, a [String] is always valid, since it is a plain value, but a UUID is
/// a string that needs to be of a certain pattern.
macro_rules! self_validatable {
    ($($t:ty)*) => {$(
        impl ValidateFrom<$t> for $t
        {
            fn validate(input: $t) -> Result<Validated<Self>, ValidationError> {
                Ok(Validated::new(input))
            }
        }
    )*};
}
self_validatable!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize f32 f64 bool String char);

#[cfg(feature = "uuid")]
impl ValidateFrom<uuid::Uuid> for uuid::Uuid {
    fn validate(input: uuid::Uuid) -> Result<Validated<Self>, ValidationError> {
        Ok(Validated::new(input))
    }
}

#[cfg(feature = "uuid")]
impl ValidateFrom<&str> for uuid::Uuid {
    /// Attempts to parse UUID from the given string. A successful parse is considered
    /// as valid state.
    fn validate(input: &str) -> Result<Validated<Self>, ValidationError> {
        let uuid = uuid::Uuid::parse_str(input).map_err(|e| {
            ValidationError::new(format!(
                "Failed to parse UUID from string '{}': {}",
                input, e
            ))
        })?;
        Ok(Validated::new(uuid))
    }
}

/// Validates that the trimmed string value is not empty. This is a common
/// validation pattern to assert a string is not just common whitespace.
#[cfg(feature = "util")]
pub fn not_empty(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("Value cannot be empty"));
    }

    Ok(())
}

/// Validates that the string value is a valid SQL identifier.
#[cfg(feature = "util")]
pub fn valid_sql_ident(value: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

    if !re.is_match(value) {
        return Err(ValidationError::new(
            "Value must be a valid SQL identifier (only letters, numbers and underscores, and cannot start with a number)",
        ));
    }

    Ok(())
}

/// Validates that the string value is not a SQL keyword. This is a simple check
/// against a hardcoded list of common SQL keywords. This is not exhaustive, but
/// should cover the most common cases.
#[cfg(feature = "util")]
pub fn not_sql_keyword(value: &str) -> Result<(), ValidationError> {
    const SQL_KEYWORDS: &[&str] = &[
        "select",
        "from",
        "where",
        "insert",
        "update",
        "delete",
        "create",
        "drop",
        "alter",
        "table",
        "index",
        "view",
        "trigger",
        "procedure",
        "function",
        "join",
        "on",
        "as",
        "and",
        "or",
        "not",
        "null",
        "in",
        "is",
        "like",
        "group",
        "by",
        "order",
        "having",
        "distinct",
    ];

    if SQL_KEYWORDS.contains(&value.to_lowercase().as_str()) {
        return Err(ValidationError::new(format!(
            "Value must not be the SQL keyword `{value}`"
        )));
    }

    Ok(())
}
