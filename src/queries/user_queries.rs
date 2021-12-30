use actix_web::{ get, post, delete, HttpResponse, Responder, web::{ Json, Data, Path } };
use crate::actors::user_actors::*;
use crate::models::db_models::{ AppState };
use crate::db_utils::cloned_db;
use uuid::Uuid;

#[post("/new_user")]
async fn new_user(user: Json<CreateUser>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let user_data: CreateUser = user.into_inner();
    let create = CreateUser {
        email: user_data.email,
        username: user_data.username,
        country: user_data.country,
        biking_modality: user_data.biking_modality,
    };

    match db.send(create).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(error1)) => {
            println!("{:?}", error1);
            HttpResponse::InternalServerError().json("Something went wrong creating the user 1")
        },
        Err(error2) => {
            println!("{:?}", error2);
            HttpResponse::InternalServerError().json("Something went wrong creating the user 2")
        },
    }
}

#[delete("/user/{uuid}")]
async fn deleteUser(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteUser{ uuid };

    match db.send(delete).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
    
}

#[get("/all_users")]
async fn all_users(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllUsers).await {
        Ok(Ok(users)) => HttpResponse::Ok().json(users),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}