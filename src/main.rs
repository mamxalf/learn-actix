// mod => include file
mod routers;

// use actix
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Init Server....");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .service(
                web::scope("/api/v1")
                    .configure(routers::call)
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn home() -> impl Responder {
    HttpResponse::Ok().body("#ROOT")
}
