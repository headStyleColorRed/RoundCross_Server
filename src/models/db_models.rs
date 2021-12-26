use crate::actors::db::DBActor;
use crate::schema::{ emergencies, owners, messages, answers };
use actix::Addr;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DBActor>,
}

// Main emergency object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "emergencies"]
pub struct Emergency {
    pub id: Uuid,
    pub active: bool,
    pub owner_id: Uuid,
    pub emergency_type: String,
    pub created_at: String,
    pub localization: String,
    pub helped: bool,
}

// Emergency owner object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "owners"]
pub struct Owner {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub country: String,
    pub biking_modality: String,
}

// Emergency answer object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "answers"]
pub struct Answer {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub answer: String,
}

// Emergency message object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "messages"]
pub struct Message {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub message_text: String,
}