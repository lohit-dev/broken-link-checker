use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
};

use tracing::{error, info};
use url::Url;

use crate::context::AppContext;
use crate::core::{crawler::Crawler, validator::Validator};

pub mod command;
pub mod crawler;
pub mod validator;

pub struct Checker {
    pub ctx: Arc<AppContext>,
    pub crawler: Crawler,
    pub validator: Validator,
    checked: HashSet<String>,
}

impl Checker {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        let validator = Validator::new(&ctx.settings.checker, &ctx.settings.crawlet);

        Self {
            ctx: Arc::clone(&ctx),
            crawler: Crawler::new(ctx),
            validator,
            checked: HashSet::new(),
        }
    }

    pub async fn check(&mut self, url: &Url) -> eyre::Result<()> {
        let max_depth = self.ctx.settings.crawlet.max_depth;

        let mut queue: VecDeque<(Url, u32)> = VecDeque::new();
        let mut ok_count = 0usize;
        let mut broken_count = 0usize;
        let mut broken_urls: Vec<String> = Vec::new();

        let links = self.crawler.crawl(url.as_str()).await?;
        self.process_links(
            url,
            0,
            max_depth,
            links,
            &mut queue,
            &mut ok_count,
            &mut broken_count,
            &mut broken_urls,
        )
        .await?;

        while let Some((page_url, depth)) = queue.pop_front() {
            let links = self.crawler.crawl(page_url.as_str()).await?;
            self.process_links(
                url,
                depth,
                max_depth,
                links,
                &mut queue,
                &mut ok_count,
                &mut broken_count,
                &mut broken_urls,
            )
            .await?;
        }

        info!(
            ok = ok_count,
            broken = broken_count,
            "Done. Checked {} links, {} OK, {} broken",
            ok_count + broken_count,
            ok_count,
            broken_count
        );
        if !broken_urls.is_empty() {
            info!("Broken URLs:");
            for u in &broken_urls {
                info!("  {}", u);
            }
        }

        Ok(())
    }

    async fn process_links(
        &mut self,
        url: &Url,
        depth: u32,
        max_depth: u32,
        links: Vec<String>,
        queue: &mut VecDeque<(Url, u32)>,
        ok_count: &mut usize,
        broken_count: &mut usize,
        broken_urls: &mut Vec<String>,
    ) -> eyre::Result<()> {
        let within_depth = max_depth == 0 || depth < max_depth;
        for link in links {
            let resolved = match Url::parse(&link) {
                Ok(u) => u,
                Err(_) => match url.join(link.trim()) {
                    Ok(u) => u,
                    Err(_) => continue,
                },
            };

            if self.validator.should_check(&resolved, url) {
                if self.checked.insert(resolved.to_string()) {
                    match self.ctx.http_client.check_link(resolved.as_str()).await {
                        Ok(true) => {
                            *ok_count += 1;
                            info!(url = %resolved, "OK");
                        }
                        Ok(false) | Err(_) => {
                            *broken_count += 1;
                            error!(url = %resolved, "BROKEN");
                            broken_urls.push(resolved.to_string());
                        }
                    }
                }
            }

            if within_depth
                && self.validator.should_crawl(&resolved, url)
                && !self.crawler.visited.contains(resolved.as_str())
            {
                queue.push_back((resolved, depth + 1));
            }
        }

        Ok(())
    }
}
