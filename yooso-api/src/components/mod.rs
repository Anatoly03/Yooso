//! This module manages component entries in the Yooso application.

mod create;
mod list;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        create::create_component,
        list::list_components,
    ]
}
