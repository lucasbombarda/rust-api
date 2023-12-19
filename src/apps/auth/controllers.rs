use super::models::list_all_users;
use super::models::Users;
use crate::common::db::DbPool;
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
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
