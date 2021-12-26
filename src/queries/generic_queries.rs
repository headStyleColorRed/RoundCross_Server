use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}