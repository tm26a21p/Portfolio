
use axum::{routing::get, Router};
use crate::routes::*;
mod routes;
mod templates;
mod utils;

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/more-content", get(more_content))
        .nest("/public", routes::serve_static_files());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port. port 3000 Already in use?");
    println!("Server running on: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
