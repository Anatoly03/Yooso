//! This module manages entity entries in the Yooso application.

mod add_component;
mod create;
mod delete;
mod list;
mod remove_component;
mod view;

/// Defines entity-related API endpoints.
/// 
/// # Routes
/// 
/// ```http
/// POST /<id>/component/<component_id>
/// DELETE /<id>/component/<component_id>
/// POST /
/// DELETE /<uuid>
/// GET /list
/// GET /view/<uuid>
/// ```
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
