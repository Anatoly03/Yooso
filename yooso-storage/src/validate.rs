//! Utility methods for validating field data from input.

use regex::Regex;
use yooso_core::Error::ValidationError;
use yooso_core::error::Result;

/// Validates that the trimmed string value is not empty.
pub fn not_empty(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(ValidationError(format!("{} cannot be empty", field_name)));
    }

    Ok(())
}

/// Validates that the string value is a valid SQL identifier. This is a
/// simple regex check that allows only letters, numbers and underscores, and
/// does not allow starting with a number.
pub fn valid_sql_ident(value: &str, field_name: &str) -> Result<()> {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

    if !re.is_match(value) {
        return Err(ValidationError(format!(
            "{} must be a valid SQL identifier (only letters, numbers and underscores, and cannot start with a number)",
            field_name
        )));
    }

    Ok(())
}

/// Validates that the string value is not a SQL keyword. This is a simple check
/// against a hardcoded list of common SQL keywords. This is not exhaustive, but
/// should cover the most common cases.
pub fn not_sql_keyword(value: &str, field_name: &str) -> Result<()> {
    const SQL_KEYWORDS: &[&str] = &[
        "select", "from", "where", "insert", "update", "delete", "create", "drop",
        "alter", "table", "index", "view", "trigger", "procedure", "function",
        "join", "on", "as", "and", "or", "not", "null", "in", "is", "like",
        "group", "by", "order", "having", "distinct",
    ];

    if SQL_KEYWORDS.contains(&value.to_lowercase().as_str()) {
        return Err(ValidationError(format!(
            "{} cannot be a SQL keyword `{}`",
            field_name, value
        )));
    }

    Ok(())
}
