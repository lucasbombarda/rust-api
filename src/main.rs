use apps::auth;
use axum::{Extension, Router};
use config::settings;
use tower_http::trace::TraceLayer;
use tracing::Level;

mod apps;
mod common;
mod config;
mod schema;
mod utils;

fn setup_tracing() {
    let level = if settings::DEBUG {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();
}

#[tokio::main]
async fn main() {
    let pool = common::db::establish_connection();
    setup_tracing();

    let app = Router::new()
        .nest("/users", auth::routes::auth_routes())
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(settings::HOST)
        .await
        .expect("ERROR: Can not start listener.");

    println!("INFO: Listening on http://{}", settings::HOST);

    axum::serve(listener, app)
        .await
        .expect("ERROR: Cannot start server.");
}
