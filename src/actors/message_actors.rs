use super::db::DBActor;
use crate::models::db_models::Message;
use crate::schema::messages::dsl::*;
use actix::{Handler, Message as ActixMessage};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create Message actor
#[derive(ActixMessage, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Message>")]
pub struct CreateMessage {
    pub parent_id: Uuid,
    pub message_text: String,
}

impl Handler<CreateMessage> for DBActor {
    type Result = QueryResult<Message>;

    fn handle(&mut self, msg: CreateMessage, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");
        let new_message = Message {
            id: Uuid::new_v4(),
            parent_id: msg.parent_id,
            message_text: msg.message_text,
        };

        diesel::insert_into(messages)
            .values(new_message)
            .get_result::<Message>(&connection)
    }
}

#[derive(ActixMessage, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Message>")]
pub struct DeleteMessage {
    pub uuid: Uuid,
}

impl Handler<DeleteMessage> for DBActor {
    type Result = QueryResult<Message>;

    fn handle(&mut self, msg: DeleteMessage, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::delete(messages)
            .filter(id.eq(msg.uuid))
            .get_result::<Message>(&connection)
    }
}

#[derive(ActixMessage, Serialize, Deserialize)]
#[rtype(result = "QueryResult<Vec<Message>>")]
pub struct DeleteChildMessage {
    pub parent_id: Uuid,
}

impl Handler<DeleteChildMessage> for DBActor {
    type Result = QueryResult<Vec<Message>>;

    fn handle(&mut self, msg: DeleteChildMessage, _: &mut Self::Context) -> Self::Result {
        let connection = self.0.get().expect("Unable to get a connection");

        diesel::delete(messages)
            .filter(parent_id.eq(msg.parent_id))
            .get_results::<Message>(&connection)
    }
}

// Retrieve all messages
#[derive(ActixMessage)]
#[rtype(result = "QueryResult<Vec<Message>>")]
pub struct GetAllMessages;
impl Handler<GetAllMessages> for DBActor {
    type Result = QueryResult<Vec<Message>>;

    fn handle(&mut self, _: GetAllMessages, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        messages.get_results::<Message>(&conn)
    }
}