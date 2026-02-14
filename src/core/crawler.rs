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
