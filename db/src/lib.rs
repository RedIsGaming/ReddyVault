use core::panic;
use std::env;
use diesel::{pg::PgConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use models::User;
use crate::models::SelectUser;

pub mod models;
pub mod schema;

pub fn connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let mut conn = connection();
    let user = users
        .filter(created.is_not_null())
        .limit(5)
        .select(User::as_select((id, username, email, password, created, role)))
        .get_results::<User>(&mut conn)
        .expect("Error while trying to load users.");

    user
}
