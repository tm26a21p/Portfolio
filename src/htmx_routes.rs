// use std::collections::HashMap;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use askama_axum::Template;
use serde::Deserialize;

use crate::{htmx_templates::*, project::Project, state::Common};

pub async fn projects(Extension(state): Extension<Common>)
    -> impl IntoResponse
{
    let projects = Project::get_repositories(state.octocrab)
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT {
        liked: false,
        projects,
    }
    .render()
    .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn projects_liked(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let projects = Project::get_repositories_liked(state.octocrab)
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT {
        liked: false,
        projects,
    }
    .render()
    .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct Payload
{
    theme: String,
}

pub async fn set_theme(
    Extension(state): Extension<Common>,
    Form(payload): Form<Payload>,
) -> impl IntoResponse
{
    state.set_theme(payload.theme);
    (StatusCode::OK, "Theme set".into_response())
}
