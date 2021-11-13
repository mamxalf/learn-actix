mod examples;

use actix_web::{web};

pub fn v1(config: &mut web::ServiceConfig) {
    // Example Resource
    config.service(examples::resource::test);
    config.service(examples::resource::hello);

    // Another Resource
}