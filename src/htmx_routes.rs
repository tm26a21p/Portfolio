use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use askama_axum::Template;

use crate::{htmx_templates::*, state::Common, project::Project};

pub async fn projects(
    Extension(_state): Extension<Common>
) -> impl IntoResponse
{
    let projects = Project::get_repos()
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT { liked: false, projects }
        .render()
        .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn projects_liked(
    Extension(_state): Extension<Common>
) -> impl IntoResponse
{
    let projects = Project::get_repos()
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT { liked: true, projects }
        .render()
        .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}