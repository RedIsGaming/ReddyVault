use diesel::{backend::Backend, deserialize::{FromSql, Result}, sql_types::Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, diesel_derive_enum::DbEnum, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::Roles"]
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
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> Result<Self> {
        match String::from_sql(bytes)?.as_ref() {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "default" => Ok(Role::Default),
            err => Err(format!("Unrecognized variant {}", err).into()),
        }
    }
}
