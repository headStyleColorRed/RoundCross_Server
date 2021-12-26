use actix::{Actor, Handler, Message, SyncContext};
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

// Database Actor
pub struct DBActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DBActor {
    type Context = SyncContext<Self>;
}


