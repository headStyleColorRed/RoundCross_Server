use actix::Addr;
use actix_web::web::Data;
use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use crate::models::db_models::AppState;
use crate::actors::db::DBActor;

pub fn run_migrations(database_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(database_url).expect("Error connectiong to Database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

pub fn get_pool(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}

pub fn cloned_db(state: Data<AppState>) -> Addr<DBActor> {
    let db = state.as_ref().db.clone();
    db
}