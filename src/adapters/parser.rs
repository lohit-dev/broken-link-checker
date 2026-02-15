use eyre::eyre;
use scraper::{Html, Selector};
use tracing::info;

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

        let links: Vec<String> = document
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .map(|s| s.to_string())
            .collect();

        info!(count = links.len(), "Found links");
        info!("");
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

    #[test]
    fn test_parse_links_empty_html() {
        let parser = HtmlParser::new();
        let links = parser.parse_links("<html><body></body></html>").unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_parse_links_no_anchors() {
        let parser = HtmlParser::new();
        let html = r#"<html><body><p>No links here</p><div>Just text</div></body></html>"#;
        let links = parser.parse_links(html).unwrap();
        assert!(links.is_empty());
    }

    #[test]
    fn test_parse_links_duplicate_hrefs() {
        let html = r#"
            <html><body>
                <a href="https://example.com">One</a>
                <a href="https://example.com">Two</a>
            </body></html>
        "#;
        let parser = HtmlParser::new();
        let links = parser.parse_links(html).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0], "https://example.com");
        assert_eq!(links[1], "https://example.com");
    }

    #[test]
    fn test_parse_links_fragment_and_query() {
        let html = r#"
            <html><body>
                <a href="/path#section">Fragment</a>
                <a href="/search?q=test">Query</a>
            </body></html>
        "#;
        let parser = HtmlParser::new();
        let links = parser.parse_links(html).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0], "/path#section");
        assert_eq!(links[1], "/search?q=test");
    }

    #[test]
    fn test_parse_links_empty_href_omitted() {
        let html = r#"<html><body><a href="">Empty</a><a>No href</a></body></html>"#;
        let parser = HtmlParser::new();
        let links = parser.parse_links(html).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0], "");
    }
}
