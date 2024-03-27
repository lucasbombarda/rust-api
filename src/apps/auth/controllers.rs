use super::models::UserInsert;
use super::models::{detail_one_user, insert_user, list_all_users};
use crate::common::db::DbPool;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use serde_json::json;

pub async fn list_users(Extension(pool): Extension<DbPool>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "Database connection error."})),
            )
        }
    };

    match list_all_users(&mut conn) {
        Ok(users) => (StatusCode::OK, Json(json!(users))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"detail": "Database error."})),
        ),
    }
}

pub async fn detail_user(
    Path(user_id): Path<i32>,
    Extension(pool): Extension<DbPool>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail": "Database connection error."})),
            )
        }
    };

    match detail_one_user(&mut conn, user_id) {
        Ok(Some(user)) => (StatusCode::OK, Json(json!(user))),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"detail": "User not found."})),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"detail": "Database error."})),
        ),
    }
}

pub async fn create_user(
    Extension(pool): Extension<DbPool>,
    Json(user): Json<UserInsert>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"detail": "Database connection error."})),
                )
            }
        }
    };

    match insert_user(&mut conn, user) {
        Ok(user) => (StatusCode::CREATED, Json(json!(user))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"detail": "Database error."})),
        ),
    }
}
