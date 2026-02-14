use std::sync::Arc;

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
}

impl Checker {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        let validator = Validator::new(&ctx.settings.checker, &ctx.settings.crawlet);

        Self {
            ctx: Arc::clone(&ctx),
            crawler: Crawler::new(ctx),
            validator,
        }
    }

    pub async fn check(&mut self, url: &Url) -> eyre::Result<()> {
        let links = self.crawler.crawl(url.as_str()).await?;
        let base = url.clone();

        let mut ok_count = 0usize;
        let mut broken_count = 0usize;
        let mut broken_urls: Vec<String> = Vec::new();

        for link in links {
            let resolved = match Url::parse(&link) {
                Ok(u) => u,
                Err(_) => match base.join(link.trim()) {
                    Ok(u) => u,
                    Err(_) => continue,
                },
            };
            if !self.validator.should_check(&resolved, &base) {
                continue;
            }
            let resolved_str = resolved.to_string();
            if !self.crawler.visited.insert(resolved_str.clone()) {
                continue; // already seen (crawled or checked)
            }

            match self.ctx.http_client.check_link(resolved_str.as_str()).await {
                Ok(true) => {
                    ok_count += 1;
                    info!(url = %resolved_str, "OK");
                }
                Ok(false) | Err(_) => {
                    broken_count += 1;
                    error!(url = %resolved_str, "BROKEN");
                    broken_urls.push(resolved_str);
                }
            }
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
}
