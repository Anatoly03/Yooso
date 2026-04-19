//! This module manages component entries in the Yooso application.

mod create;
mod delete;
mod list;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        create::create_component,
        delete::delete_component,
        list::list_components,
    ]
}
