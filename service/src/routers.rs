mod examples;

use actix_web::{web};

pub fn v1(config: &mut web::ServiceConfig) {
    // Example Resource
    config.service(examples::resource::test);
    config.service(examples::resource::hello);

    // Example User Resource
    config.service(examples::users::index);
    config.service(examples::users::show);

    // Another Resource
}