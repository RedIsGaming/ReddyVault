use diesel::{prelude::Insertable, Queryable, Selectable};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::{role::Role, schema::users::columns};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub role: Role,
}

#[derive(Debug, Queryable, Insertable, Clone, Copy)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub username: &'a columns::username,
    pub email: &'a columns::email,
    pub password: &'a columns::password,
    pub role: &'a columns::role,
}
