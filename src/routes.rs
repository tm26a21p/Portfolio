use std::sync::atomic::{AtomicI32, Ordering};
use axum::{http::StatusCode, response::Html, Router};
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};
use lazy_static::lazy_static;
use templates::MoreContentTemplate;
use crate::{
    templates::{self, IndexTemplate},
    utils::add_tailwind_classes,
};
// serve_static_files() function is used to serve static files from the public directory.
pub fn serve_static_files() -> Router
{
    Router::new().nest_service("/", ServeDir::new("public"))
}

pub async fn index() -> impl IntoResponse
{
    let readme_raw = IndexTemplate::get_readme();
    let readme_html = markdown::to_html(&readme_raw);
    let readme = add_tailwind_classes(&readme_html);
    let template = templates::IndexTemplate {
        title: "Axhat Stack Template",
        readme: &readme,
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

// HTMX ROUTES BELOW

// creating a static counter to keep track of the number of times the more_content route is called per session.
lazy_static! {
    static ref COUNTER: AtomicI32 = AtomicI32::new(1);
}

pub async fn more_content() -> impl IntoResponse
{
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let reply_html = MoreContentTemplate { n }
        .render()
        .expect("Failed to render template");
    println!("reply: {}", reply_html);
    (StatusCode::OK, Html(reply_html).into_response())
}
