mod v1;

use actix_web::{web};

pub fn call(config: &mut web::ServiceConfig) {
    config.service(v1::examples::init::hello);
    config.service(v1::examples::init::test);
}