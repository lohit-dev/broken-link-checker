use std::time::Duration;

use reqwest::Client;

pub struct HttpClient {
    pub client: reqwest::Client,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(
            10,    // default timeout of 10 seconds
            false, // do not ignore SSL errors by default
        )
    }
}

impl HttpClient {
    pub fn new(timeout_seconds: u64, ignore_ssl: bool) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_seconds))
            .danger_accept_invalid_certs(ignore_ssl)
            .build()
            .expect("failed to build HTTP client");

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> eyre::Result<String> {
        let response = self.client.get(url).send().await?;
        let html = response
            .text()
            .await
            .map_err(|e| eyre::eyre!("failed to read response body: {}", e))?;

        Ok(html)
    }

    pub async fn check_link(&self, url: &str) -> eyre::Result<bool> {
        let response = self.client.head(url).send().await?;
        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        let client = HttpClient::default();
        let html = client.fetch("https://www.rust-lang.org").await.unwrap();
        assert!(html.contains("Rust"));
    }

    #[tokio::test]
    async fn test_check_link() {
        let client = HttpClient::default();
        assert!(
            client
                .check_link("https://www.rust-lang.org")
                .await
                .unwrap()
        );
        assert!(
            !client
                .check_link("https://www.rust-lang.org/nonexistent")
                .await
                .unwrap()
        );
    }
}
