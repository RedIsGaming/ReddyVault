use diesel::{backend::Backend, helper_types::AsSelect, Queryable, Selectable};
use chrono::{DateTime, Utc};
use serde::Serialize;
use diesel::prelude::SelectableHelper;

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

pub trait SelectUser<DB: Backend> : Selectable<DB> + Sized {
    fn as_select(&self) -> AsSelect<Self, DB>;
}

impl<T, DB> SelectableHelper<DB> for T
where
    T: Selectable<DB>,
    DB: Backend,
{
    fn as_select() -> AsSelect<Self, DB> {
        Self::as_select()
    }
}

impl SelectUser<diesel::pg::Pg> for User {
    fn as_select(&self) -> AsSelect<Self, diesel::pg::Pg> {
        self.id.as_select()
            .and(self.username.as_select())
            .and(self.email.as_select())
            .and(self.password.as_select())
            .and(self.created.as_select())
            .and(self.role.as_select())
    }
}
