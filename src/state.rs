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
    pub metrics: Arc<RwLock<Metrics>>,
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
            metrics: Arc::new(RwLock::new(Metrics::new())),
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

#[derive(Debug, Clone)]
pub struct Metrics
{
    pub visited_last_month: usize,
    pub visited: usize,
    pub ip: String,
    pub likes: usize,
    pub issues_open: usize,
    pub issues_closed: usize,
}

impl Metrics
{
    pub fn new() -> Self
    {
        Self {
            visited_last_month: 6234,
            visited: 1,
            ip: "Unknown".to_string(),
            likes: 0,
            issues_open: 31,
            issues_closed: 87,
        }
    }

    pub fn get_likes_ratio_over_last_month(&self) -> f64
    {
        // 2 decimal places
        let ratio = (self.likes as f64 / self.visited as f64) * 100.0;
        (ratio * 100.0).round() / 100.0
    }

    pub fn get_percent_issues_closed(&self) -> f64
    {
        // 2 decimal places
        let percent = (self.issues_closed as f64
            / (self.issues_open + self.issues_closed) as f64)
            * 100.0;
        (percent * 100.0).round() / 100.0
    }

    pub fn get_percent_views_over_last_month(&self) -> f64
    {
        // 2 decimal places
        let percent =
            (self.visited as f64 / self.visited_last_month as f64) * 100.0;
        (percent * 100.0).round() / 100.0
    }
}
