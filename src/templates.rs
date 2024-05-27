use askama_axum::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseT {
    pub title: String,
    pub daisy_theme: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexT {
    pub base: BaseT,
}

#[derive(Template)]
#[template(path = "metrics.html")]
pub struct MetricsT {
    pub base: BaseT,
    pub likes: usize,
    pub likes_percent_over_last_month: f64,
    pub views: usize,
    pub view_percent_over_to_last_month: f64,
}