
use actix_web::{ get, post, delete, HttpResponse, Responder, web::{ Json, Data, Path } };
use crate::actors::answer_actors::*;
use crate::models::db_models::{ AppState };
use crate::db_utils::cloned_db;
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
        Ok(Err(error1)) => {
            println!("{:?}", error1);
            HttpResponse::InternalServerError().json("Something went wrong creating the emergency 1")
        },
        Err(error2) => {
            println!("{:?}", error2);
            HttpResponse::InternalServerError().json("Something went wrong creating the emergency 2")
        },
    }
}


#[delete("/answer/{uuid}")]
async fn deleteAnswer(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteAnswer{ uuid };

    match db.send(delete).await {
        Ok(Ok(answer)) => HttpResponse::Ok().json(answer),
        Ok(Err(_)) => HttpResponse::NotFound().json("Answer not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
    
}

#[get("/all_answers")]
async fn all_answers(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllAnswers).await {
        Ok(Ok(answers)) => HttpResponse::Ok().json(answers),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}