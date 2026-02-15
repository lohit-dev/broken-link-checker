use std::time::Duration;

use reqwest::{Client, redirect::Policy};
use tracing::info;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

pub struct HttpClient {
    pub client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(
            10,    // default timeout of 10 seconds
            5,     // default max redirects of 5
            false, // do not ignore SSL errors by default
        )
    }
}

impl HttpClient {
    pub fn new(timeout_seconds: u64, max_redirects: u32, ignore_ssl: bool) -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(timeout_seconds))
            .danger_accept_invalid_certs(ignore_ssl)
            .redirect(Policy::limited(max_redirects as usize))
            .build()
            .expect("failed to build HTTP client");

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> eyre::Result<String> {
        info!("");
        info!(url = %url, "Fetching URL");
        let response = self.client.get(url).send().await?;
        let html = response
            .text()
            .await
            .map_err(|e| eyre::eyre!("failed to read response body: {}", e))?;

        Ok(html)
    }

    pub async fn check_link(&self, url: &str) -> eyre::Result<bool> {
        let response = self.client.get(url).send().await?;
        let code = response.status().as_u16();
        Ok(!is_broken_status(code))
    }
}

/// Only 404 and 5xx are considered broken.
pub fn is_broken_status(code: u16) -> bool {
    code == 404 || (500..600).contains(&code)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn test_client() -> HttpClient {
        HttpClient::new(30, 5, false)
    }

    #[tokio::test]
    async fn test_fetch_returns_html_with_links() {
        let mock_server = MockServer::start().await;
        let body = r#"<!DOCTYPE html>
        <html><body>
        <a href="https://example.com">Example</a>
        <a href="/relative">Relative</a>
        </body></html>"#;
        Mock::given(method("GET"))
            .and(path("/page"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&mock_server)
            .await;

        let client = test_client();
        let url = format!("{}/page", mock_server.uri());
        let html = client.fetch(&url).await.unwrap();
        assert!(html.contains("https://example.com"));
        assert!(html.contains("/relative"));
    }

    #[tokio::test]
    async fn test_check_link_200_ok() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/ok"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = test_client();
        let ok = client
            .check_link(&format!("{}/ok", mock_server.uri()))
            .await
            .unwrap();
        assert!(ok);
    }

    #[tokio::test]
    async fn test_check_link_404_broken() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/not-found"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = test_client();
        let ok = client
            .check_link(&format!("{}/not-found", mock_server.uri()))
            .await
            .unwrap();
        assert!(!ok);
    }

    #[tokio::test]
    async fn test_check_link_500_broken() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client = test_client();
        let ok = client
            .check_link(&format!("{}/error", mock_server.uri()))
            .await
            .unwrap();
        assert!(!ok);
    }

    #[test]
    fn test_is_broken_status() {
        assert!(!is_broken_status(200));
        assert!(!is_broken_status(201));
        assert!(!is_broken_status(301));
        assert!(!is_broken_status(302));
        assert!(!is_broken_status(400));
        assert!(!is_broken_status(403));
        assert!(!is_broken_status(405));
        assert!(!is_broken_status(600));

        assert!(is_broken_status(404));
        assert!(is_broken_status(500));
        assert!(is_broken_status(502));
        assert!(is_broken_status(503));
        assert!(is_broken_status(599));
    }
}
