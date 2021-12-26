use super::db::DBActor;
use crate::models::db_models::User;
use crate::schema::users::dsl::*;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create User actor
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub country: String,
    pub biking_modality: String,
}

impl Handler<CreateUser> for DBActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");
        let new_user = User {
            id: Uuid::new_v4(),
            email: msg.email,
            username: msg.username,
            country: msg.country,
            biking_modality: msg.biking_modality,
        };

        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&connection)
    }
}

// Retrieve all users
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetAllUsers;
impl Handler<GetAllUsers> for DBActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _: GetAllUsers, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        users.get_results::<User>(&conn)
    }
}