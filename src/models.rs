use crate::actors::db::DBActor;
use crate::schema::{ emergencies };
use actix::Addr;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DBActor>,
}