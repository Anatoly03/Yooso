//! A utility crate for securely displaying secret values without exposing
//! their content in logs or debug output.
//!
//! ### Example
//!
//! When printing the
//!
//! ```
//! use util_obfuscate::*;
//!
//! fn main() {
//!     let password: Obfuscated<&str> = obfuscate("super secret key");
//!     println!("Password: {password}"); // Password: ********
//! }
//! ```
//!
//! ### Wrapper Type
//!
//! The [Obfuscated] struct wraps any value and overrides its [Debug] and [Display]
//! implementations to hide the actual content when printed. When debugging the value,
//! the value type is shown instead of the content.
//!
//! ```
//! use util_obfuscate::Obfuscated;
//!
//! fn main() {
//!     let api_key: Obfuscated<&str> = Obfuscated::from("key");
//!     println!("API Key: {api_key:?}"); // API Key: Obfuscated<&str>
//! }
//! ```

use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Deref;

/// A constant string used for obfuscation in the [Display] implementation.
pub const OBFUSCATION_MASK: &str = "********";

/// Wrapper for values overriding [Debug] and [Display] to hide their content
/// when printed. Useful for secrets like API keys, passwords, etc.
pub struct Obfuscated<T> {
    inner_value: T,
}

/// Obfuscates a value by wrapping it in the [Obfuscated] struct.
pub fn obfuscate<T>(inner_value: T) -> Obfuscated<T> {
    Obfuscated { inner_value }
}

/// Implements the [From] trait to allow easy conversion from any type to [Obfuscated].
impl<T> From<T> for Obfuscated<T> {
    fn from(value: T) -> Self {
        Obfuscated { inner_value: value }
    }
}

/// Implements the deref trait to allow access to the inner value without
/// unwrapping. After dereferencing, the value can be used as normal.
impl<T> Deref for Obfuscated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner_value
    }
}

/// Implements the [Debug] trait to hide the content of the value when printed.
impl<T> Debug for Obfuscated<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Obfuscated<{}>", type_name::<T>())
    }
}

/// Implements the [Display] trait to hide the content of the value when printed.
/// Replaces every character with an asterisk for a simple masking effect. The
/// length is not preserved to avoid leaking information.
impl<T> Display for Obfuscated<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", OBFUSCATION_MASK)
    }
}

#[cfg(test)]
mod tests {
    use super::{OBFUSCATION_MASK, obfuscate};

    #[test]
    fn test_hidden_debug() {
        let password = "password";
        let hidden_password = obfuscate(password);
        let debug_str = format!("{:?}", hidden_password);
        assert_eq!(debug_str, "Obfuscated<&str>");
    }

    #[test]
    fn test_hidden_display() {
        let password = "password";
        let hidden_password = obfuscate(password);
        let display_str = format!("{}", hidden_password);
        assert_eq!(display_str, OBFUSCATION_MASK);
    }

    #[test]
    fn test_no_length_information() {
        let password = "password";
        let long_password = "longest password you ever read";
        let hidden_password = obfuscate(password);
        let hidden_long_password = obfuscate(long_password);
        assert_eq!(format!("{}", hidden_password), OBFUSCATION_MASK);
        assert_eq!(format!("{}", hidden_long_password), OBFUSCATION_MASK);
    }

    #[test]
    fn deref_reading_possible() {
        let password = "password";
        let hidden_password = obfuscate(password);
        assert_eq!(format!("{}", *hidden_password), "password");
    }

    #[test]
    fn inner_methods_accessible_without_deref() {
        let password = "new password";
        let hidden_password = obfuscate(password);
        assert_eq!(hidden_password.len(), password.len());
        assert_eq!(format!("{}", hidden_password).len(), OBFUSCATION_MASK.len());
    }
}
