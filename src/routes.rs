use axum::{http::StatusCode, response::Html, Router};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};
use lazy_static::lazy_static;

use crate::{templates::*, utils::random_daisy_theme};
// serve_static_files() function is used to serve static files from the public directory.
pub fn serve_static_files() -> Router
{
    Router::new().nest_service("/", ServeDir::new("public"))
}

lazy_static! {
    pub static ref RANDOM_THEME: Mutex<String> =
        Mutex::new(random_daisy_theme());
}

pub async fn index() -> impl IntoResponse
{
    let theme = RANDOM_THEME.lock().await;
    let base = BaseT {
        title: "Lpio - Home",
        daisy_theme: theme.clone(),
    };
    let template = IndexT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn metrics() -> impl IntoResponse
{
    let theme = RANDOM_THEME.lock().await;
    let base = BaseT {
        title: "Lpio - Metrics",
        daisy_theme: theme.clone(),
    };
    let template = MetricsT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn about() -> impl IntoResponse
{
    let theme = RANDOM_THEME.lock().await;
    let base = BaseT {
        title: "Lpio - About",
        daisy_theme: theme.clone(),
    };
    let template = AboutT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

// pub async fn not_found() -> impl IntoResponse
// {
//     (StatusCode::NOT_FOUND, Html("404 Not Found").into_response())
// }

// HTMX ROUTES BELOW

// creating a static counter to keep track of the number of times the more_content route is called per session.
// lazy_static! {
//     static ref COUNTER: AtomicI32 = AtomicI32::new(1);
// }

// pub async fn more_content() -> impl IntoResponse
// {
//     let n = COUNTER.fetch_add(1, Ordering::SeqCst);
//     let reply_html = MoreContentTemplate { n }
//         .render()
//         .expect("Failed to render template");
//     println!("reply: {}", reply_html);
//     (StatusCode::OK, Html(reply_html).into_response())
// }
