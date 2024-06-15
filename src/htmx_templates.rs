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
