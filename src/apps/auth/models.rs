use crate::schema::users::dsl::*;
use crate::utils::password_hash;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInsert {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn list_all_users(conn: &mut PgConnection) -> QueryResult<Vec<Users>> {
    users.load::<Users>(conn)
}

pub fn detail_one_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<Option<Users>> {
    users
        .find(user_id)
        .select(Users::as_select())
        .first(conn)
        .optional()
}

pub fn insert_user(conn: &mut PgConnection, user: UserInsert) -> QueryResult<Users> {
    let hashed_password = password_hash::create_hash(user.password.as_bytes()).unwrap();

    let user = UserInsert {
        password: hashed_password,
        ..user
    };

    diesel::insert_into(users).values(&user).get_result(conn)
}
