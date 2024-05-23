
use std::fs;

use askama_axum::Template;

#[derive(Template)]
#[template(path = "more-content.html")]
pub struct MoreContentTemplate
{
    pub n: i32,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a>
{
    pub title: &'a str,
    pub readme: &'a str,
    pub daisy_theme: &'a str,
}

impl IndexTemplate<'_>
{
    pub fn get_readme() -> String
    {
        fs::read_to_string("README.md")
            .unwrap_or_else(|_| "README.md not found".to_string())
    }
}
