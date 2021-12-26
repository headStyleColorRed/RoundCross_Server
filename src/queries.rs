
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;


#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running")
}