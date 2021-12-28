
use actix_web::{ get, post, HttpResponse, Responder, web::Data, web::{ Json } };
use crate::actors::answer_actors::*;
use crate::models::db_models::{ AppState };
use crate::db_utils::cloned_db;

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

#[get("/all_answers")]
async fn all_answers(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllAnswers).await {
        Ok(Ok(emergencies)) => HttpResponse::Ok().json(emergencies),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}