use askama_axum::Template;
use crate::project::Project;

#[derive(Template)]
#[template(path = "htmx/tabs.html")]
pub struct TabsT {
    pub liked: bool,
    pub projects: Vec<Project>,
}