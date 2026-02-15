use url::Url;

use crate::settings::{CheckerSettings, CrawletSettings};

#[derive(Default)]
pub struct Validator {
    check_external_links: bool,
    same_domain_only: bool,
}

impl Validator {
    pub fn new(checker: &CheckerSettings, crawler: &CrawletSettings) -> Self {
        Self {
            check_external_links: checker.check_external_links,
            same_domain_only: crawler.same_domain_only,
        }
    }

    /// Whether to run check_link (HTTP GET) on this URL. When same_domain_only is set we still
    /// check external links (YouTube, etc.) so we can report broken outbound links; we just don't crawl them.
    pub fn should_check(&self, url: &Url, base: &Url) -> bool {
        if !self.is_checkable(url) {
            return false;
        }
        if !self.check_external_links && !self.same_origin(url, base) {
            return false;
        }
        true
    }

    pub fn is_checkable(&self, url: &Url) -> bool {
        matches!(url.scheme(), "http" | "https")
    }

    /// Whether to follow this link and crawl its page for more links. When same_domain_only is set
    /// we only enqueue same-domain URLs (no following to scrapfly.io, YouTube, etc.).
    pub fn should_crawl(&self, url: &Url, base: &Url) -> bool {
        if !self.is_checkable(url) {
            return false;
        }
        if self.same_domain_only && !self.same_domain(url, base) {
            return false;
        }
        true
    }

    fn same_origin(&self, url: &Url, base: &Url) -> bool {
        url.origin() == base.origin()
    }

    fn same_domain(&self, url: &Url, base: &Url) -> bool {
        url.host_str() == base.host_str()
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::settings::{CheckerSettings, CrawletSettings};

    use super::*;

    fn validator_all() -> Validator {
        Validator::new(
            &CheckerSettings {
                check_external_links: true,
                ignore_ssl_errors: false,
                retry_attempts: 3,
            },
            &CrawletSettings {
                max_depth: 3,
                max_concurrent_requests: 10,
                same_domain_only: false,
            },
        )
    }

    fn validator_same_domain_only() -> Validator {
        Validator::new(
            &CheckerSettings {
                check_external_links: true,
                ignore_ssl_errors: false,
                retry_attempts: 3,
            },
            &CrawletSettings {
                max_depth: 3,
                max_concurrent_requests: 10,
                same_domain_only: true,
            },
        )
    }

    fn validator_internal_only() -> Validator {
        Validator::new(
            &CheckerSettings {
                check_external_links: false,
                ignore_ssl_errors: false,
                retry_attempts: 3,
            },
            &CrawletSettings {
                max_depth: 3,
                max_concurrent_requests: 10,
                same_domain_only: false,
            },
        )
    }

    #[test]
    fn test_is_checkable() {
        let v = validator_all();
        assert!(v.is_checkable(&Url::parse("https://example.com").unwrap()));
        assert!(v.is_checkable(&Url::parse("http://example.com").unwrap()));
        assert!(!v.is_checkable(&Url::parse("mailto:foo@bar.com").unwrap()));
        assert!(!v.is_checkable(&Url::parse("javascript:void(0)").unwrap()));
        assert!(!v.is_checkable(&Url::parse("tel:+1234567890").unwrap()));
    }

    #[test]
    fn test_should_check_all() {
        let v = validator_all();
        let base = Url::parse("https://example.com/page").unwrap();
        assert!(v.should_check(&Url::parse("https://example.com/other").unwrap(), &base));
        assert!(v.should_check(&Url::parse("https://other.com/page").unwrap(), &base));
    }

    #[test]
    fn test_should_check_same_domain_only_still_checks_external() {
        let v = validator_same_domain_only();
        let base = Url::parse("https://example.com/page").unwrap();
        assert!(v.should_check(&Url::parse("https://example.com/other").unwrap(), &base));
        assert!(v.should_check(&Url::parse("https://other.com/page").unwrap(), &base));
    }

    #[test]
    fn test_should_crawl_same_domain_only() {
        let v = validator_same_domain_only();
        let base = Url::parse("https://example.com/page").unwrap();
        assert!(v.should_crawl(&Url::parse("https://example.com/other").unwrap(), &base));
        assert!(!v.should_crawl(&Url::parse("https://other.com/page").unwrap(), &base));
    }

    #[test]
    fn test_should_check_internal_only() {
        let v = validator_internal_only();
        let base = Url::parse("https://example.com:443/page").unwrap();
        assert!(v.should_check(&Url::parse("https://example.com/other").unwrap(), &base));
        assert!(!v.should_check(&Url::parse("https://other.com/page").unwrap(), &base));
    }
}
