use crate::actors::emergency_actors::*;
use crate::db_utils::cloned_db;
use crate::models::db_models::AppState;
use crate::utils::ServerError;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/new_emergency")]
async fn new_emergency(emergency: Json<CreateEmergency>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let emergency_data: CreateEmergency = emergency.into_inner();
    let create = CreateEmergency {
        owner_id: emergency_data.owner_id,
        emergency_type: emergency_data.emergency_type,
        localization: emergency_data.localization,
    };

    match db.send(create).await {
        Ok(Ok(emergency)) => HttpResponse::Ok().json(emergency),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Emergency not found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error creating emergency".to_string(),
            error: error.to_string(),
        })
    }
}

#[delete("/emergency/{uuid}")]
async fn delete_emergency(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteEmergency { uuid };

    match db.send(delete).await {
        Ok(Ok(emergency)) => HttpResponse::Ok().json(emergency),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "Emergency not found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error deleting emergency".to_string(),
            error: error.to_string(),
        })
    }
}

#[get("/user_emergencies/{uuid}")]
async fn user_emergencies(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let user_emergencies = GetUserEmergencies { user_id: uuid };

    match db.send(user_emergencies).await {
        Ok(Ok(emergencies)) => HttpResponse::Ok().json(emergencies),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "No emergency found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding emergency".to_string(),
            error: error.to_string(),
        })
    }
}

#[get("/all_emergencies")]
async fn all_emergencies(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllEmergencys).await {
        Ok(Ok(emergencies)) => HttpResponse::Ok().json(emergencies),
        Ok(Err(error)) => HttpResponse::NotFound().json(ServerError {
            data: "No emergencies found".to_string(),
            error: error.to_string(),
        }),
        Err(error) => HttpResponse::InternalServerError().json(ServerError {
            data: "Error finding emergencies".to_string(),
            error: error.to_string(),
        })
    }
}
