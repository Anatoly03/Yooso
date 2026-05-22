//! This module manages entity entries in the Yooso application.

mod add_component;
mod create;
mod delete;
mod list;
mod remove_component;
mod view;

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        add_component::add_component,
        remove_component::remove_component,
        create::create_entity,
        delete::delete_entity,
        list::list_entities,
        view::view_entity,
    ]
}
