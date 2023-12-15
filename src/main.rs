use axum::{Extension, Router};

mod apps;
mod common;
mod config;
mod schema;

#[tokio::main]
async fn main() {
    let pool = common::db::establish_connection();

    let app = Router::new()
        .nest("/users", apps::auth::routes::auth_routes())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
