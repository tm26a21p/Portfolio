use axum::{extract::Extension, http::StatusCode, response::Html, Router};
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};

use crate::{state::Common, templates::*};
use crate::project::Project;

pub fn using_serve_dir() -> Router
{
    // serve the file in the "public" directory
    Router::new().nest_service("/", ServeDir::new("public"))
}

pub async fn index(Extension(state): Extension<Common>) -> impl IntoResponse
{
    let base = BaseT {
        title: state.name.clone() + " - Home",
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
        title: state.name.clone() + " - Metrics",
        daisy_theme: state.daisy_theme.clone(),
    };
    let template = MetricsT { 
        base,
        likes: 0,
        likes_percent_over_last_month: 0.0,
        views: 0,
        view_percent_over_to_last_month: 0.0,
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn projects(Extension(state): Extension<Common>)
    -> impl IntoResponse
{
    let base = BaseT {
        title: state.name.clone() + " - Projects",
        daisy_theme: state.daisy_theme.clone(),
    };
    let projects = Project::get_repos().await
        .expect("Failed to get repos from Github API");
    let template = ProjectsT { 
        base,
        projects,
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}