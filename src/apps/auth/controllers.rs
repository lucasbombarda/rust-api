use super::models::{list_all_users, insert_user, detail_one_user};
use super::models::{Users, UserInsert};
use crate::common::db::DbPool;
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};

pub async fn list_users(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<Users>>, StatusCode> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match list_all_users(&mut conn) {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn detail_user(
    Path(user_id): Path<i32>,
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Users>, StatusCode> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match detail_one_user(&mut conn, user_id) {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user(
    Json(user): Json<UserInsert>,
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Users>, StatusCode> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match insert_user(&mut conn, user) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
