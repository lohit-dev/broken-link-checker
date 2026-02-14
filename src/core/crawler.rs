use eyre::{Result, eyre};
use std::{collections::HashSet, sync::Arc};

use crate::context::AppContext;

pub struct Crawler {
    pub ctx: Arc<AppContext>,
    pub visited: HashSet<String>,
}

impl Crawler {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Crawler {
            ctx,
            visited: HashSet::new(),
        }
    }

    pub async fn crawl(&mut self, url: &str) -> Result<Vec<String>> {
        if !self.visited.insert(url.to_string()) {
            return Err(eyre!("URL {} has already been visited", url));
        }

        let html = self
            .ctx
            .http_client
            .fetch(url)
            .await
            .map_err(|e| eyre!("failed to fetch URL {}: {}", url, e))?;

        let links = self
            .ctx
            .parser
            .parse_links(&html)
            .map_err(|e| eyre!("failed to parse links: {}", e))?;

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::context::AppContext;
    use crate::settings::Settings;

    use super::*;

    #[tokio::test]
    async fn test_crawl_already_visited_errors() {
        let settings = Settings::default();
        let ctx = Arc::new(AppContext::new(settings));
        let mut crawler = Crawler::new(ctx);
        let url = "https://example.com";
        crawler.visited.insert(url.to_string());
        let result = crawler.crawl(url).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("already been visited")
        );
    }
}
