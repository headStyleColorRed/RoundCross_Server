// Global crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

// Modules
mod db_utils;
mod schema;
mod actors;
mod queries;
mod models;
mod utils;

// Libary imports
use actix::SyncArbiter;
use actix_web::{App, HttpServer};
use actors::db::DBActor;
use db_utils::{get_pool, run_migrations};
use models::db_models::AppState;
use dotenv::dotenv;
use queries::*;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get DB connection from environment variables
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url. Check your .env file");

    // Run diesel migration
    run_migrations(&db_url);

    // Start Sync Arbiter's actors pool
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DBActor(pool.clone()));

    // Start server
    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(new_emergency)
            .service(all_emergencies)
            .service(user_emergencies)
            .service(delete_emergency)
            .service(new_user)
            .service(all_users)
            .service(delete_user)
            .service(new_answer)
            .service(all_answers)
            .service(delete_answer)
            .service(new_message)
            .service(all_messages)
            .service(delete_message)
            .data(AppState{ db: db_addr.clone() })
    })
    .bind(("127.0.0.1", 8889))?
    .run()
    .await
}