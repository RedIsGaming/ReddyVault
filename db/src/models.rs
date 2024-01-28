use diesel::{backend::Backend, expression::SelectableHelper, helper_types::AsSelect, Queryable, Selectable};
use chrono::NaiveDateTime;
use serde::Serialize;
use crate::{role::Role, schema::users};

#[derive(Debug, Queryable, Selectable, Serialize)]
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

impl<T, DB> SelectableHelper<DB> for T
where
    T: Selectable<DB>,
    DB: Backend,
{
    fn as_select(&self) -> AsSelect<Self, DB> {
        users::all_columns()
    }
}
