use crate::adapters::{http::HttpClient, parser::HtmlParser};

pub struct AppContext {
    pub parser: HtmlParser,
    pub http_client: HttpClient,
}

impl Default for AppContext {
    fn default() -> Self {
        Self::new(
            10,    // default timeout of 10 seconds
            false, // do not ignore SSL errors by default
        )
    }
}

impl AppContext {
    pub fn new(timeout_seconds: u64, ignore_ssl: bool) -> Self {
        Self {
            parser: HtmlParser::new(),
            http_client: HttpClient::new(timeout_seconds, ignore_ssl),
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn test_app_context_parser() {
        let context = AppContext::default();
        assert_eq!(
            context
                .parser
                .parse_links("<a href=\"https://example.com\">Example</a>")
                .unwrap(),
            vec!["https://example.com".to_string()]
        );
    }

    #[tokio::test]
    async fn test_app_context_http_client() {
        let context = AppContext::default();
        let url = Url::parse("https://www.rust-lang.org").unwrap();

        let html = context.http_client.fetch(url.as_str()).await.unwrap();
        assert!(html.contains("Rust"));

        let is_valid = context.http_client.check_link(url.as_str()).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_app_content_full_flow() {
        let context = AppContext::default();
        let url = Url::parse("https://www.rust-lang.org").unwrap();

        let html = context.http_client.fetch(url.as_str()).await.unwrap();
        assert!(html.contains("Rust"));

        let links = context.parser.parse_links(&html).unwrap();
        assert!(!links.is_empty());
        println!("Found {} links on the page.", links.len());
        println!("The links are: {:?}", links);

        for link in &links {
            let is_valid = context
                .http_client
                .check_link(link)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Error checking {}: {}", link, e);
                    false
                });

            println!("{} -> {}", link, if is_valid { "valid" } else { "invalid" });
        }
    }
}
