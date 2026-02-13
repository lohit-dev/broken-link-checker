use eyre::eyre;
use scraper::{Html, Selector};

pub struct HtmlParser;

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_links(&self, html: &str) -> eyre::Result<Vec<String>> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a")
            .map_err(|e| eyre!("invalid CSS selector for anchor links: {}", e))?;

        let links = document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .map(|s| s.to_string())
            .collect();

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_links() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com">Example</a>
                    <a href="https://rust-lang.org">Rust</a>
                    <a href="/relative/link">Relative Link</a>
                </body>
            </html>
        "#;

        let parser = HtmlParser::new();
        let links = parser.parse_links(html).unwrap();
        assert_eq!(links.len(), 3);
        assert_eq!(links[0], "https://example.com");
        assert_eq!(links[1], "https://rust-lang.org");
        assert_eq!(links[2], "/relative/link");
    }
}
