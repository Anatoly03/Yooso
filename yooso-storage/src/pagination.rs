//! Defines the pagination utilities for the API.

use std::fmt::Display;

/// The pagination parameters for the API.
#[derive(Debug)]
pub struct Pagination {
    /// The page number to retrieve. Defaults to 1.
    pub page: usize,

    /// The number of items per page. Defaults to 25.
    pub per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 25,
        }
    }
}

impl Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(page {} of {})", self.page, self.per_page)
    }
}
