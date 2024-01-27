use diesel::{Queryable, Selectable};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    #[diesel(embed)]
    pub created: DateTime<Utc>,
    pub role: String,
}
