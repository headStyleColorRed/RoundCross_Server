
use actix_web::{ get, post, HttpResponse, Responder, web::Data, web::{ Json } };
use crate::actors::message_actors::*;
use crate::models::db_models::{ AppState };
use crate::db_utils::cloned_db;

#[post("/new_message")]
async fn new_message(message: Json<CreateMessage>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let message_data: CreateMessage = message.into_inner();
    let create = CreateMessage {
        parent_id: message_data.parent_id,
        message_text: message_data.message_text,
    };

    match db.send(create).await {
        Ok(Ok(message)) => HttpResponse::Ok().json(message),
        Ok(Err(error1)) => {
            println!("{:?}", error1);
            HttpResponse::InternalServerError().json("Something went wrong creating the message 1")
        },
        Err(error2) => {
            println!("{:?}", error2);
            HttpResponse::InternalServerError().json("Something went wrong creating the message 2")
        },
    }
}

#[get("/all_messages")]
async fn all_messages(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllMessages).await {
        Ok(Ok(messages)) => HttpResponse::Ok().json(messages),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}