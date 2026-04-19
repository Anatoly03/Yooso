//! This module manages entity entries in the Yooso application.

mod create;
mod delete;
mod list;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        create::create_entity,
        delete::delete_entity,
        list::list_entities,
    ]
}
