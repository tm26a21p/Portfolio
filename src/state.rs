
use crate::utils::random_daisy_theme;

#[derive(Debug, Clone)]
pub struct Common
{
    pub name: String,
    pub version: String,
    pub daisy_theme: String,
    pub _github_token: String,
    pub octocrab: octocrab::Octocrab,
}

impl Common
{
    pub fn new() -> Self
    {
        let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found.");
        Self {
            name: "Lpio".to_string(),
            version: "0.0.1".to_string(),
            daisy_theme: random_daisy_theme(),
            _github_token: github_token.clone(),
            octocrab: octocrab::Octocrab::builder()
                .personal_token(
                    github_token
                )
                .build()
                .expect("Failed to create Octocrab instance."),
        }
    }
}