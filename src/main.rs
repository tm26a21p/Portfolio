use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
mod routes;
mod templates;
mod utils;
mod state;
use routes::*;
use state::Common;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Common::new();

    let app = Router::new()
        .route("/", get(index))
        .route("/about", post(about))
        .route("/metrics", get(metrics))
        .nest("/public", using_serve_dir())
        .layer(Extension(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");
    println!("Listening on: http://{}", "localhost:3000");
    axum::serve(listener, app.into_make_service()).await.expect("Server failed")
    
}