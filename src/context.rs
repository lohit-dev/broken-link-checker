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
        let mock_server = wiremock::MockServer::start().await;
        let body = r#"<html><body><a href="https://example.com">Link</a></body></html>"#;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/page"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(body))
            .mount(&mock_server)
            .await;

        let mut settings = Settings::default();
        settings.http.timeout_seconds = 30;
        let context = AppContext::new(settings);
        let url = format!("{}/page", mock_server.uri());

        let html = context.http_client.fetch(&url).await.unwrap();
        assert!(html.contains("https://example.com"));

        let is_valid = context.http_client.check_link(&url).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_app_content_full_flow() {
        let mock_server = wiremock::MockServer::start().await;
        let body = r#"<!DOCTYPE html>
        <html><body>
        <a href="/ok">OK</a>
        <a href="/broken">Broken</a>
        </body></html>"#;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/page"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(body))
            .mount(&mock_server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/ok"))
            .respond_with(wiremock::ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/broken"))
            .respond_with(wiremock::ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let mut settings = Settings::default();
        settings.http.timeout_seconds = 30;
        let context = AppContext::new(settings);
        let base = format!("{}/page", mock_server.uri());

        let html = context.http_client.fetch(&base).await.unwrap();
        let links = context.parser.parse_links(&html).unwrap();
        assert_eq!(links.len(), 2);

        let base_url = Url::parse(&base).unwrap();
        for link in &links {
            let resolved = if link.starts_with("http") {
                Url::parse(link).unwrap()
            } else {
                base_url.join(link).unwrap()
            };
            let is_valid = context
                .http_client
                .check_link(resolved.as_str())
                .await
                .unwrap_or(false);
            if link == "/ok" || link.ends_with("/ok") {
                assert!(is_valid, "{} should be valid", link);
            } else {
                assert!(!is_valid, "{} should be broken", link);
            }
        }
    }
}
