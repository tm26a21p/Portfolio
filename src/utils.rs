use scraper::{Html as ScraperHtml, Selector};
use std::collections::HashMap;

pub fn add_tailwind_classes(html: &str) -> String {
    let document = ScraperHtml::parse_document(html);
    let mut result = String::new();

    let selectors = HashMap::from([
        ("pre", "bg-gray-100 p-4 rounded mb-4"),
        ("h1", "text-4xl font-bold mb-4"),
        ("h2", "text-2xl font-semibold mb-2"),
        ("ul", "list-disc pl-5 mb-4"),
    ]);

    for (tag, class) in selectors {
        let selector = Selector::parse(tag).unwrap();
        for element in document.select(&selector) {
            result.push_str(&format!(
                r#"<{} class="{}">{}</{}>"#,
                tag,
                class,
                element.inner_html(),
                tag
            ));
        }
    }

    result
}
