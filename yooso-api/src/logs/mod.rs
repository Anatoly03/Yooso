use rocket::{State, get, serde::json::Json};
use uuid::Uuid;
use yooso_storage::{LogDBState, LogRecordTable};

mod fairing;

/// The fairing that logs all incoming requests, panics and other events.
pub struct LogFairing;

#[get("/?<limit>")]
pub fn list_logs(db: &State<LogDBState>, limit: Option<u32>) -> Json<Vec<LogRecordTable>> {
    let limit = limit.unwrap_or(200).min(500) as i64;
    let conn = db.0.lock().expect("lock sqlite db");
    let mut stmt = conn
        .prepare(
            "SELECT *
			 FROM logs
			 ORDER BY created_at DESC
			 LIMIT ?1",
        )
        .expect("prepare list logs");

    let rows = stmt
        .query_map([limit], |row| {
            Ok(LogRecordTable {
                id: Uuid::parse_str(&row.get::<_, String>(0)?)
                    .expect("parse log record id (invariant assumes stored Uuid is always valid)"),
                created_at: row.get(1)?,
                duration_ns: row.get(2)?,
                referer: row.get(3)?,
                user_agent: row.get(4)?,
                method: row.get(5)?,
                uri: row.get(6)?,
                client_ip: row.get(7)?,
                remote_ip: row.get(8)?,
                status: row.get(9)?,
            })
        })
        .expect("query logs");

    let mut logs = Vec::new();
    for row in rows {
        logs.push(row.expect("read log"));
    }

    Json(logs)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_logs,]
}
