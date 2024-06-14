use std::sync::{Arc, RwLock};

use crate::utils::random_daisy_theme;

#[derive(Debug, Clone)]
pub struct Common
{
    pub name: String,
    pub version: String,
    pub daisy_theme: Arc<RwLock<String>>,
    pub _github_token: String,
    pub octocrab: octocrab::Octocrab,
}

impl Common
{
    pub fn new() -> Self
    {
        let github_token =
            std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found.");
        Self {
            name: "Lpio".to_string(),
            version: "0.0.1".to_string(),
            daisy_theme: Arc::new(RwLock::new(random_daisy_theme())),
            _github_token: github_token.clone(),
            octocrab: octocrab::Octocrab::builder()
                .personal_token(github_token)
                .build()
                .expect("Failed to create Octocrab instance."),
        }
    }

    pub fn get_theme(&self) -> String
    {
        self.daisy_theme.read().unwrap().clone()
    }

    pub fn set_theme(
        &self,
        new_theme: String,
    )
    {
        let mut theme = self.daisy_theme.write().unwrap();
        *theme = new_theme;
    }
}
