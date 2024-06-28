// use std::collections::HashMap;

use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use askama_axum::Template;
use axum_extra::{headers::*, TypedHeader};
// use octocrab::params::repos::Type;
use serde::Deserialize;

use crate::{htmx_templates::*, project::Project, state::Common};

pub async fn projects(Extension(state): Extension<Common>)
    -> impl IntoResponse
{
    let projects = Project::get_repositories(state.octocrab)
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT {
        liked: false,
        projects,
    }
    .render()
    .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn projects_liked(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let projects = Project::get_repositories_liked(state.octocrab)
        .await
        .expect("Failed to get repos from Github API");
    let reply_html = TabsT {
        liked: false,
        projects,
    }
    .render()
    .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct Payload
{
    theme: String,
}

pub async fn set_theme(
    Extension(state): Extension<Common>,
    Form(payload): Form<Payload>,
) -> impl IntoResponse
{
    state.set_theme(payload.theme);
    (StatusCode::OK, "Theme set".into_response())
}

async fn get_ip() -> String
{
    reqwest::get("https://api.ipify.org")
        .await
        .expect("Failed to get ip")
        .text()
        .await
        .expect("Failed to get ip")
}

pub async fn ip_adress(
    Extension(state): Extension<Common>
) -> impl IntoResponse
{
    let ip = get_ip().await;

    let mut metrics = state.metrics.write().unwrap();
    metrics.ip = ip.clone();
    drop(metrics);
    (StatusCode::OK, ip.into_response())
}

use ipgeolocate::{Locator, Service};

async fn get_location(ip: &str) -> Option<GeoLocation>
{
    let service = Service::IpApi;
    match Locator::get(ip, service).await {
        Ok(response) => {
            Some(GeoLocation {
                latitude: response.latitude,
                longitude: response.longitude,
                city: response.city,
                region: response.region,
                country: response.country,
                timezone: response.timezone,
            })
        }
        Err(error) => {
            eprintln!("Failed to get location: {:?}", error);
            None
        }
    }
}

pub async fn location(
    Extension(_state): Extension<Common>
) -> impl IntoResponse
{
    let ip = get_ip().await;
    let location = get_location(&ip).await.unwrap();

    let reply_html = LocationT { location }
        .render()
        .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn device_info(
    Extension(state): Extension<Common>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> impl IntoResponse
{
    let mut device_info = DeviceInfoT::new();
    let ua = user_agent.to_string();
    let ua_parser = state.ua_parser.read().unwrap();
    let os = ua_parser.parse_os(&ua);
    device_info.os = os.name.unwrap_or_default().to_string();
    let device = ua_parser.parse_device(&ua);
    device_info.device = device.name.unwrap_or_default().to_string()
        + ", "
        + &device.brand.unwrap_or_default()
        + ", "
        + &device.model.unwrap_or_default();
    let product = ua_parser.parse_product(&ua);
    device_info.browser = product.name.unwrap_or_default().to_string();
    // device_info.language = "en".to_string();
    // device_info.origin = origin.to_string();

    let reply_html = device_info.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn user_info(
    Extension(_state): Extension<Common>
) -> impl IntoResponse
{
    // let reply_html = UserInfoT::new()
    //     .render()
    //     .expect("Failed to render template");
    // (StatusCode::OK, Html(reply_html).into_response())
    (StatusCode::OK, "User info".into_response())
}

pub async fn visit_info(
    Extension(_state): Extension<Common>
) -> impl IntoResponse
{
    let v = VisitInfoT::new();
    let reply_html = v.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}
