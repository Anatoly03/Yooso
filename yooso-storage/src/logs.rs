use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tracks all request and response logs. Each record corresponds to a
/// single HTTP request and its associated response.
#[collection(db = crate::LogDB, table = "logs")]
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LogRecordTable {
    /// Snowflake value. This is the unique identifier of the entity.
    #[primary]
    pub id: Uuid,

    /// The timestamp of when the entity was created, in seconds since
    /// the Unix epoch.
    pub created_at: i64,

    /// The duration of the request in nanoseconds.
    pub duration_ns: i64,

    /// The referer of the request, which in development is usually
    /// `http://localhost:8080/`
    pub referer: Option<String>,

    /// The users' agent string.
    ///
    /// # Example
    ///
    /// ```text
    /// Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36
    /// ```
    pub user_agent: Option<String>,

    /// The HTTP method of the request, such as `GET`, `POST`, etc.
    pub method: String,

    /// The URI of the request, for example `/api/components/list`
    pub uri: String,

    /// The IP address of the client that made the request.
    pub client_ip: Option<String>,

    /// The IP address of the remote server that handled the request.
    pub remote_ip: Option<String>,

    /// The HTTP status code of the response.
    pub status: u16,
}

impl LogRecordTable {
    /// Converts the duration from nanoseconds to a milliseconds floating-point
    /// number for easier readability.
    pub fn duration_ms(&self) -> f64 {
        self.duration_ns as f64 / 1_000_000.0
    }
}
