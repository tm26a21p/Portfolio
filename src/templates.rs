use askama_axum::Template;

use crate::project::Project;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseT
{
    pub title: String,
    pub daisy_theme: String,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexT
{
    pub base: BaseT,
    pub version: String,
}

#[derive(Template)]
#[template(path = "pages/metrics.html")]
pub struct MetricsT
{
    pub base: BaseT,
    pub likes: usize,
    pub likes_ratio_over_last_month: f64,
    pub views: usize,
    pub view_ratio_over_last_month: f64,
}

#[derive(Template)]
#[template(path = "pages/projects.html")]
pub struct ProjectsT
{
    pub base: BaseT,
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "pages/playground.html")]
pub struct PlaygroundT
{
    pub base: BaseT,
}