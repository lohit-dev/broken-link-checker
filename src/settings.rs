use config::{Config, File};
use eyre::Context;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub http: HttpSettings,
    pub crawlet: CrawletSettings,
    pub checker: CheckerSettings,
    pub output: OutputSettings,
}

impl Settings {
    pub fn new() -> eyre::Result<Self> {
        let config = Config::builder()
            .add_source(File::with_name("Settings"))
            .build()
            .wrap_err("failed to load config file")?;

        config
            .try_deserialize()
            .wrap_err("Failed to deserialize settings")
    }
}

#[derive(Deserialize)]
pub struct HttpSettings {
    pub timeout_seconds: u64,
    pub max_redirects: u32,
}

#[derive(Deserialize)]
pub struct CrawletSettings {
    pub max_depth: u32,
    pub max_concurrent_requests: u32,
    pub same_domain_only: bool,
}

#[derive(Deserialize)]
pub struct CheckerSettings {
    pub check_external_links: bool,
    pub ignore_ssl_errors: bool,
    pub retry_attempts: u32,
}

#[derive(Deserialize)]
pub struct OutputSettings {
    pub output_format: OutputFormat,
    pub show_successful: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Pretty,
    Json,
}

impl From<&str> for OutputFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pretty" => OutputFormat::Pretty,
            "json" => OutputFormat::Json,
            _ => OutputFormat::Pretty,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            http: HttpSettings {
                timeout_seconds: 30,
                max_redirects: 5,
            },
            crawlet: CrawletSettings {
                max_depth: 3,
                max_concurrent_requests: 10,
                same_domain_only: false,
            },
            checker: CheckerSettings {
                check_external_links: true,
                ignore_ssl_errors: true,
                retry_attempts: 3,
            },
            output: OutputSettings {
                output_format: OutputFormat::Pretty,
                show_successful: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from("pretty"), OutputFormat::Pretty);
        assert_eq!(OutputFormat::from("json"), OutputFormat::Json);
        assert_eq!(OutputFormat::from("unknown"), OutputFormat::Pretty);
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert_eq!(settings.http.timeout_seconds, 30);
        assert_eq!(settings.http.max_redirects, 5);
        assert_eq!(settings.crawlet.max_depth, 3);
        assert_eq!(settings.crawlet.max_concurrent_requests, 10);
        assert!(!settings.crawlet.same_domain_only);
        assert!(settings.checker.check_external_links);
        assert!(settings.checker.ignore_ssl_errors);
        assert_eq!(settings.checker.retry_attempts, 3);
        assert_eq!(settings.output.output_format, OutputFormat::Pretty);
        assert!(!settings.output.show_successful);
    }

    #[test]
    fn test_settings_from_config() {
        let settings = Settings::new().unwrap();
        assert_eq!(settings.http.timeout_seconds, 30);
        assert_eq!(settings.http.max_redirects, 5);
        assert_eq!(settings.crawlet.max_depth, 3);
        assert_eq!(settings.crawlet.max_concurrent_requests, 10);
        assert!(!settings.crawlet.same_domain_only);
        assert!(settings.checker.check_external_links);
        assert!(!settings.checker.ignore_ssl_errors);
        assert_eq!(settings.checker.retry_attempts, 3);
        assert_eq!(settings.output.output_format, OutputFormat::Pretty);
        assert!(!settings.output.show_successful);
    }
}
