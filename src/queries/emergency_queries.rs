use crate::actors::{emergency_actors::*, message_actors::DeleteChildMessage};
use crate::actors::answer_actors::DeleteChildAnswer;
use crate::db_utils::cloned_db;
use crate::models::db_models::AppState;
use crate::utils::ServerError;
use futures;
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
    let delete_answers = DeleteChildAnswer { parent_id: uuid };
    let delete_messages = DeleteChildMessage { parent_id: uuid };
    let delete_emergency = DeleteEmergency { uuid };
    
    let delete_answers_result = async { db.send(delete_answers).await };
    let delete_messages_result = async { db.send(delete_messages).await };
    let delete_emergency_result = async { db.send(delete_emergency).await };

    let result = futures::try_join!(delete_answers_result, delete_messages_result, delete_emergency_result);
    
    match result {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted emergency"),
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
