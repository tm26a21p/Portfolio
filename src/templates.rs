use askama_axum::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseT<'a> {
    pub title: &'a str,
    pub daisy_theme: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexT<'a> {
    pub base: BaseT<'a>,
}

#[derive(Template)]
#[template(path = "metrics.html")]
pub struct MetricsT<'a> {
    pub base: BaseT<'a>,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutT<'a> {
    pub base: BaseT<'a>,
}