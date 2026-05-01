use super::LogFairing;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use yooso_storage::LogRecordTable;

#[rocket::async_trait]
impl Fairing for LogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request logging",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut rocket::Data<'_>) {
        request.local_cache(Instant::now);
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start = request.local_cache(Instant::now);
        let duration_ns = start.elapsed().as_nanos() as i64;
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        let method = request.method().as_str().to_string();
        let uri = request.uri().to_string();
        let status = response.status().code;
        let referer = request
            .headers()
            .get_one("Referer")
            .map(|s| s.to_string());
        let user_agent = request
            .headers()
            .get_one("User-Agent")
            .map(|s| s.to_string());
        let client_ip = request
            .client_ip()
            .map(|ip| ip.to_string());
        let remote_ip = request
            .remote()
            .map(|ip| ip.to_string());

        let Some(state) = request.rocket().state::<yooso_storage::LogDBState>() else {
            eprintln!("LogFairing: missing LogDBState, cannot log request");
            return;
        };

        let _ = LogRecordTable {
            id: Uuid::now_v7(),
            created_at,
            duration_ns,
            referer,
            user_agent,
            method,
            uri,
            client_ip,
            remote_ip,
            status,
        }
        .save(state).await;
    }
}
