// Handlers
use handlers::log_config;
use service::routers;

// use depedencies
use actix_web::{web, middleware, App, HttpServer, HttpResponse, Responder};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init ENV
    dotenv::dotenv().ok();

    // Address App
    let address = format!("{}:{}",
                          env::var("APP_HOST").expect("APP_HOST not defined in ENV"),
                          env::var("APP_PORT").expect("APP_PORT not defined in ENV")
    );

    // Init Log
    log_config::init();
    log::info!("Booting Up! {}", address);

    // Init Server
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::default())
            .route("/", web::get().to(home))
            .service(
                web::scope("/api/v1").configure(routers::v1)
            )
    })
        .bind(address)?
        .run()
        .await
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("#ROOT")
}
