use crate::actors::answer_actors::*;
use crate::db_utils::cloned_db;
use crate::models::db_models::AppState;
use crate::utils::ServerError;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/new_answer")]
async fn new_answer(answer: Json<CreateAnswer>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let answer_data: CreateAnswer = answer.into_inner();
    let create = CreateAnswer {
        parent_id: answer_data.parent_id,
        answer: answer_data.answer,
    };

    match db.send(create).await {
        Ok(Ok(answer)) => HttpResponse::Ok().json(answer),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Error creating answer".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error creating answer".to_string(),
            error: error.to_string(),
        })
    }
}

#[delete("/answer/{uuid}")]
async fn delete_answer(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteAnswer { uuid };

    match db.send(delete).await {
        Ok(Ok(answer)) => HttpResponse::Ok().json(answer),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Answer not found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding answer".to_string(),
            error: error.to_string(),
        })
    }
}

#[get("/all_answers")]
async fn all_answers(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllAnswers).await {
        Ok(Ok(answers)) => HttpResponse::Ok().json(answers),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "No answers found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding answers".to_string(),
            error: error.to_string(),
        })
    }
}
