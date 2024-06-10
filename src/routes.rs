use axum::{extract::Extension, http::StatusCode, response::Html, Router};
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};

use crate::{project::Project, state::Common, templates::*};

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
    let template = IndexT {
        base,
        version: state.version.clone(),
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn metrics_page(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let base = BaseT {
        title: state.name.clone() + " - Metrics",
        daisy_theme: state.daisy_theme.clone(),
    };
    let template = MetricsT {
        base,
        likes: 0,
        likes_ratio_over_last_month: 0.0,
        views: 0,
        view_ratio_over_last_month: 0.0,
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn projects_page(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let base = BaseT {
        title: state.name.clone() + " - Projects",
        daisy_theme: state.daisy_theme.clone(),
    };
    let projects = Project::get_repositories(state.octocrab)
        .await
        .expect("Failed to get repos from Github API");
    let template = ProjectsT { base, projects };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

// pub async fn project(state: Extension<Common>, id: u32) -> impl IntoResponse
// {
//     let base = BaseT {
//         title: state.name.clone() + " - Project ",
//         daisy_theme: state.daisy_theme.clone(),
//     };
//     let project = Project::get_repo(id).await
//         .expect("Failed to get repo from Github API");
//     let template = ProjectT {
//         base,
//         project,
//     };
//     let reply_html = template.render().expect("Failed to render template");
//     (StatusCode::OK, Html(reply_html).into_response())
// }

pub async fn playground_page(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let base = BaseT {
        title: state.name.clone() + " - Playground",
        daisy_theme: state.daisy_theme.clone(),
    };
    let template = PlaygroundT { base };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}
