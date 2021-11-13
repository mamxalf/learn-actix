use actix_web::web;
use r2d2_sqlite::SqliteConnectionManager;
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub fn connection(config: &mut web::ServiceConfig) {
    let manager = ConnectionManager::<SqliteConnection>::new("db/learn-actix.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    config.data(pool);
}