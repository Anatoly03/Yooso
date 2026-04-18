//! This module manages component entries in the Yooso application.

mod list;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list::list_components]
}
