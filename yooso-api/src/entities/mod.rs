//! This module manages entity entries in the Yooso application.

mod components;
mod create;
mod delete;
mod list;
mod view;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        components::add_component,
        components::remove_component,
        create::create_entity,
        delete::delete_entity,
        list::list_entities,
        view::view_entity,
    ]
}
