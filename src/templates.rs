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
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutT {
    pub base: BaseT,
}