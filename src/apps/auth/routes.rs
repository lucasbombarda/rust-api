use super::controllers;
use axum::{routing::get, Router};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/", get(controllers::list_users))
        .route("/:id", get(controllers::detail_user))
}
