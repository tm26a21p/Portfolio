use askama_axum::Template;

use crate::project::Project;

#[derive(Template)]
#[template(path = "htmx/tabs.html")]
pub struct TabsT
{
    pub liked: bool,
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "htmx/location.html")]
pub struct LocationT
{
    pub location: GeoLocation,
}

pub struct GeoLocation
{
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
}

#[derive(Template)]
#[template(path = "htmx/device_info.html")]
pub struct DeviceInfoT
{
    pub device: String,
    pub os: String,
    pub browser: String,
    pub user_agent: String,
    pub language: String,
}

impl DeviceInfoT
{
    pub fn new() -> Self
    {
        Self {
            device: "device".to_string(),
            os: "os".to_string(),
            browser: "browser".to_string(),
            user_agent: "user_agent".to_string(),
            language: "language".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "htmx/visit_info.html")]
pub struct VisitInfoT
{
    pub average_time: usize,
    pub total_time: usize,
}

impl VisitInfoT
{
    pub fn new() -> Self
    {
        Self {
            average_time: 24,
            total_time: 0,
        }
    }
}
