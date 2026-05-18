//! This module contains the [Success] response type.

use rocket::{Request, Response, http::ContentType, response::Responder};
use serde::Serialize;
use serde_json::json;

/// The [Success] template is a simple wrapper around a successful response.
pub struct Success<T>(pub T)
where
    T: Serialize;

pub struct SuccessUnit;

impl<'r, 'o, T> Responder<'r, 'o> for Success<T>
where
    'o: 'r,
    T: Serialize,
{
    /// Responds in the following JSON format:
    ///
    /// ```json
    /// {
    ///     "success": true,
    ///     "data": { ... }
    /// }
    /// ```
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let payload = json!({
            "success": true,
            "data": self.0,
        });
        let body = serde_json::to_string(&payload)
            .map_err(|_| rocket::http::Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}

impl<'r, 'o> Responder<'r, 'o> for SuccessUnit
where
    'o: 'r,
{
    /// Responds in the following JSON format:
    ///
    /// ```json
    /// {
    ///     "success": true,
    /// }
    /// ```
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let payload = json!({ "success": true, });
        let body = serde_json::to_string(&payload)
            .map_err(|_| rocket::http::Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}
