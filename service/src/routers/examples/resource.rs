use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Malika!")
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Ini Test.....")
}