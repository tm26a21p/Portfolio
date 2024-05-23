use axum::{extract::Extension, http::StatusCode, response::Html, Router};
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};

use crate::{state::Common, templates::*};

pub fn using_serve_dir() -> Router
{
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/", ServeDir::new("public"))
}

pub async fn index(Extension(state): Extension<Common>) -> impl IntoResponse
{
    let base = BaseT {
        title: "Lpio - Home",
        daisy_theme: state.daisy_theme.clone(),
    };
    let template = IndexT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn metrics(Extension(state): Extension<Common>)
    -> impl IntoResponse
{
    let base = BaseT {
        title: "Lpio - Metrics",
        daisy_theme: state.daisy_theme.clone(),
    };
    let template = MetricsT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn about() {}
