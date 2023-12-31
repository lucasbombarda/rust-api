use crate::schema::users::dsl::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub active: bool,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn list_all_users(conn: &mut PgConnection) -> QueryResult<Vec<Users>> {
    return users.load::<Users>(conn);
}

pub fn detail_one_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Option<Users>> {
    return users
        .find(user_id)
        .select(Users::as_select())
        .first(conn)
        .optional();
}
