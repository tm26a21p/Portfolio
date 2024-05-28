use axum::{extract::Extension, routing::get, Router};
mod projects;
mod routes;
mod state;
mod templates;
mod utils;
use routes::*;
use state::Common;
extern crate dotenv;

use dotenv::dotenv;
#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();

    let state = Common::new();
    dotenv().ok();

    let app = Router::new()
        .route("/", get(index))
        .route("/home", get(index))
        .route("/metrics", get(metrics))
        .route("/projects", get(projects))
        .nest("/public", using_serve_dir())
        .layer(Extension(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444")
        .await
        .expect("Failed to bind port 4444");
    println!("Listening on: http://{}", "localhost:4444");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed")
}