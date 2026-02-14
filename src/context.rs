use crate::{
    adapters::{http::HttpClient, parser::HtmlParser},
    settings::Settings,
};

pub struct AppContext {
    pub settings: Settings,
    pub http_client: HttpClient,
    pub parser: HtmlParser,
}

impl AppContext {
    pub fn new(settings: Settings) -> Self {
        let http_client = HttpClient::new(
            settings.http.timeout_seconds,
            settings.http.max_redirects,
            settings.checker.ignore_ssl_errors,
        );
        let parser = HtmlParser::new();

        Self {
            settings,
            http_client,
            parser,
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::settings::Settings;

    use super::*;

    #[test]
    fn test_app_context_parser() {
        let settings = Settings::default();
        let context = AppContext::new(settings);
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
        let mut settings = Settings::default();
        settings.http.timeout_seconds = 60;
        let context = AppContext::new(settings);
        let url = Url::parse("https://www.rust-lang.org").unwrap();

        let html = context.http_client.fetch(url.as_str()).await.unwrap();
        assert!(html.contains("Rust"));

        let is_valid = context.http_client.check_link(url.as_str()).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_app_content_full_flow() {
        let mut settings = Settings::default();
        settings.http.timeout_seconds = 120;
        let context = AppContext::new(settings);
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
