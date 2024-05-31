
use crate::utils::random_daisy_theme;

#[derive(Debug, Clone)]
pub struct Common
{
    pub name: String,
    pub version: String,
    pub daisy_theme: String,
}

impl Common
{
    pub fn new() -> Self
    {
        Self {
            name: "Lpio".to_string(),
            version: "0.0.1".to_string(),
            daisy_theme: random_daisy_theme()
        }
    }
}