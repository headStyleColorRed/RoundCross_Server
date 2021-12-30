use super::db::DBActor;
use crate::models::db_models::Answer;
use crate::schema::answers::dsl::*;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create Answer actor
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Answer>")]
pub struct CreateAnswer {
    pub parent_id: Uuid,
    pub answer: String,
}

impl Handler<CreateAnswer> for DBActor {
    type Result = QueryResult<Answer>;

    fn handle(&mut self, msg: CreateAnswer, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");
        let new_answer = Answer {
            id: Uuid::new_v4(),
            parent_id: msg.parent_id,
            answer: msg.answer,
        };

        diesel::insert_into(answers)
            .values(new_answer)
            .get_result::<Answer>(&connection)
    }
}

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Answer>")]
pub struct DeleteAnswer {
    pub uuid: Uuid,
}

impl Handler<DeleteAnswer> for DBActor {
    type Result = QueryResult<Answer>;

    fn handle(&mut self, msg: DeleteAnswer, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::delete(answers)
            .filter(id.eq(msg.uuid))
            .get_result::<Answer>(&connection)
    }
}

// Retrieve all answers
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Answer>>")]
pub struct GetAllAnswers;
impl Handler<GetAllAnswers> for DBActor {
    type Result = QueryResult<Vec<Answer>>;

    fn handle(&mut self, _: GetAllAnswers, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        answers.get_results::<Answer>(&conn)
    }
}