mod v1;

use actix_web::{web};

pub fn v1(config: &mut web::ServiceConfig) {
    config.service(v1::examples::init::hello);
    config.service(v1::examples::init::test);
}