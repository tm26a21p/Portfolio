use axum::{extract::Extension, routing::get, Router};
mod htmx_routes;
mod htmx_templates;
mod project;
mod routes;
mod state;
mod templates;
mod utils;

use routes::*;
use htmx_routes::*;
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
        // classic routes
        .route("/", get(index))
        .route("/home", get(index))
        .route("/metrics", get(metrics_page))
        .route("/projects", get(projects_page))
        // .route("/projects/{id}", get(project))
        // htmx routes
        .route("/api/projects", get(projects))
        .route("/api/projects-liked", get(projects_liked))
        .nest("/public", using_serve_dir())
        .layer(Extension(state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444")
        .await
        .expect("Failed to bind port 4444");
    println!("Listening on: http://localhost:{}", 4444);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed")
}
