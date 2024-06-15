use std::collections::HashMap;

use scraper::{Html as ScraperHtml, Selector};

pub fn _add_tailwind_classes(html: &str) -> String
{
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

pub fn read_file(file_path: &str) -> String
{
    std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| format!("{}", "dark"))
}

pub fn read_daisy_theme_config(file_path: &str) -> Vec<String>
{
    let content = read_file(file_path);

    // Split the content by new lines and collect into a Vec<String>
    content.lines().map(|line| line.to_string()).collect()
}

use rand::Rng;

// Function to return a random number between 0 and `range`
pub fn random_number(range: u32) -> u32
{
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=range)
}

pub fn random_daisy_theme() -> String
{
    // grab the file : just a txt file with theme names
    let themes = read_daisy_theme_config("themes_list.daisy");
    // Extensions name don't matter, Windows will cry. Who cares?
    let random_theme: String =
        themes[random_number(themes.len() as u32 - 1) as usize].clone();
    random_theme
}
