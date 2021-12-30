
use actix_web::{ get, post, delete, HttpResponse, Responder, web::{ Data, Json, Path } };
use crate::actors::emergency_actors::*;
use crate::models::db_models::{ AppState };
use crate::db_utils::cloned_db;
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

#[delete("/emergency/{uuid}")]
async fn deleteEmergency(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);
    let delete = DeleteEmergency{ uuid };

    match db.send(delete).await {
        Ok(Ok(emergency)) => HttpResponse::Ok().json(emergency),
        Ok(Err(_)) => HttpResponse::NotFound().json("Emergency not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
    
}

#[get("/all_emergencies")]
async fn all_emergencies(state: Data<AppState>) -> impl Responder {
    let db = cloned_db(state);

    match db.send(GetAllEmergencys).await {
        Ok(Ok(emergencies)) => HttpResponse::Ok().json(emergencies),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}