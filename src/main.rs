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

// Libary imports
use actix::SyncArbiter;
use actix_web::{App, HttpServer};
use actors::db::DBActor;
use db_utils::{get_pool, run_migrations};
use diesel_migrations::migration_from;
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
    })
    .bind(("127.0.0.1", 8889))?
    .run()
    .await
}