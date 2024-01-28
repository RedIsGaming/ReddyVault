use diesel::{backend::Backend, deserialize::{FromSql, FromSqlRow}, sql_types::Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromSqlRow)]
pub enum Role {
    Admin,
    User,
    Default,
}

impl<DB> FromSql<Text, DB> for Role
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match &str::from_sql(bytes)? {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "default" => Ok(Role::Default),
        }
    }
}
