use core::panic;
use std::env;
use diesel::{pg::PgConnection, Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::User;
use role::Role;
use crate::models::NewUser;

pub mod models;
pub mod schema;
pub mod role;

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
        .get_results::<User>(&mut conn)
        .expect("Error while trying to load users.");

    for user in &user {
        println!("{:?}", user);
    }

    user
}

pub fn create(username: &str, email: &str, password: &str, role: Role) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let mut conn = connection();
    let new_user = NewUser {
        username: &username,
        email: &email,
        password: &password,
        role: &role,
    };
    
    diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_results(&mut conn)
        .expect("Error while trying to create a new user.")
}
