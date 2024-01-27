use diesel::{deserialize::Queryable, Selectable};
use chrono::NaiveDate;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDate,
    pub role: String,
}
