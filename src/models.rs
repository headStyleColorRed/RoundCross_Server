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
    id: Uuid,
    active: bool,
    owner_id: Uuid,
    emergency_type: String,
    created_at: String,
    localization: String,
    helped: bool,
}

// Emergency owner object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "owners"]
pub struct Owner {
    id: Uuid,
    email: String,
    username: String,
    country: String,
    biking_modality: String,
}

// Emergency answer object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "answers"]
pub struct Answer {
    id: Uuid,
    parent_id: Uuid,
    answer: String,
}

// Emergency message object
#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "messages"]
pub struct Message {
    id: Uuid,
    parent_id: Uuid,
    message_text: String,
}