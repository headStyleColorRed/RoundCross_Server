use crate::actors::message_actors::*;
use crate::db_utils::cloned_db;
use crate::models::db_models::AppState;
use crate::utils::ServerError;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

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
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Error creating message".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error creating message".to_string(),
            error: error.to_string(),
        })
    }
}

#[delete("/message/{uuid}")]
async fn delete_message(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteMessage { uuid };

    match db.send(delete).await {
        Ok(Ok(message)) => HttpResponse::Ok().json(message),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Message not found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error deleting emergency".to_string(),
            error: error.to_string(),
        })
    }
}

#[get("/all_messages")]
async fn all_messages(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllMessages).await {
        Ok(Ok(messages)) => HttpResponse::Ok().json(messages),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "No messages found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding messages".to_string(),
            error: error.to_string(),
        })
    }
}
