use actix_web::web;

use crate::routers::users::resource::{index, store, show};

pub fn call(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
    config.route("/{user_id}/", web::get().to(show));
    config.route("/", web::post().to(store));
}
