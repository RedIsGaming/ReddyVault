use core::panic;
use std::env;
use diesel::{pg::PgConnection, Connection};
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
