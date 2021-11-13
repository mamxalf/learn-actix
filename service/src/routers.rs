mod examples;
mod users;

use actix_web::{web};

pub fn v1(config: &mut web::ServiceConfig) {
    // Example Resource
    config.service(examples::resource::test);
    config.service(examples::resource::hello);

    // Example User Resource
    config.service(
        web::scope("/users").configure(users::routers::call)
    );

    // Another Resource
}