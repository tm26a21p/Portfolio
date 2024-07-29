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
    pub _language: String,
}

impl DeviceInfoT
{
    pub fn new() -> Self
    {
        Self {
            device: "device".to_string(),
            os: "os".to_string(),
            browser: "browser".to_string(),
            _language: "Unknown".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "htmx/visit_info.html")]
pub struct VisitInfoT
{
    pub origin: String,
    pub average_time: usize,
    pub _total_time: usize,
}

impl VisitInfoT
{
    pub fn new() -> Self
    {
        Self {
            origin: "Unknown".to_string(),
            average_time: 24,
            _total_time: 0,
        }
    }
}
