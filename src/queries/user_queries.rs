use actix_web::{ get, post, delete, HttpResponse, Responder, web::{ Json, Data, Path } };
use crate::actors::user_actors::*;
use crate::models::db_models::{ AppState };
use crate::utils::ServerError;
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
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Error creating user".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error creating user".to_string(),
            error: error.to_string(),
        })
    }
}

#[delete("/user/{uuid}")]
async fn delete_user(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteUser{ uuid };

    match db.send(delete).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "User not found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error deleting emergency".to_string(),
            error: error.to_string(),
        })
    }
    
}

#[get("/all_users")]
async fn all_users(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllUsers).await {
        Ok(Ok(users)) => HttpResponse::Ok().json(users),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "No users found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding usres".to_string(),
            error: error.to_string(),
        })
    }
}