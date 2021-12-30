use super::db::DBActor;
use crate::models::db_models::Emergency;
use crate::schema::emergencies::dsl::*;
use crate::utils;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create emergency actor
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Emergency>")]
pub struct CreateEmergency {
    pub owner_id: Uuid,
    pub emergency_type: String,
    pub localization: String,
}

impl Handler<CreateEmergency> for DBActor {
    type Result = QueryResult<Emergency>;

    fn handle(&mut self, msg: CreateEmergency, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");
        let new_emergency = Emergency {
            id: Uuid::new_v4(),
            active: true,
            owner_id: msg.owner_id,
            emergency_type: msg.emergency_type,
            created_at: utils::current_date(),
            localization: msg.localization,
            helped: false,
        };

        diesel::insert_into(emergencies)
            .values(new_emergency)
            .get_result::<Emergency>(&connection)
    }
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Emergency>")]
pub struct DeleteEmergency {
    pub uuid: Uuid,
}

impl Handler<DeleteEmergency> for DBActor {
    type Result = QueryResult<Emergency>;

    fn handle(&mut self, msg: DeleteEmergency, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::delete(emergencies)
            .filter(id.eq(msg.uuid))
            .get_result::<Emergency>(&connection)
    }
}

// Retrieve user emergencies
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Emergency>>")]
pub struct GetUserEmergencies {
    pub user_id: Uuid,
}
impl Handler<GetUserEmergencies> for DBActor {
    type Result = QueryResult<Vec<Emergency>>;

    fn handle(&mut self, msg: GetUserEmergencies, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        emergencies
        .filter(id.eq(msg.user_id))
        .get_results::<Emergency>(&conn)
    }
}

// Retrieve all emergencies
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Emergency>>")]
pub struct GetAllEmergencys;
impl Handler<GetAllEmergencys> for DBActor {
    type Result = QueryResult<Vec<Emergency>>;

    fn handle(&mut self, _: GetAllEmergencys, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        emergencies.get_results::<Emergency>(&conn)
    }
}
