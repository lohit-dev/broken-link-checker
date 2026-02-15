use config::{Config, File};
use eyre::Context;
use serde::Deserialize;

use crate::adapters::Cli;

/// For each `$src => $path , $convert`: if let Some(v) = $src { $path = ($convert)(v) }.
macro_rules! apply {
    ($( $src:expr => $path:expr , $convert:expr );+ $(;)?) => {
        $( if let Some(v) = $src { $path = ($convert)(v); } )+
    };
}

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

    pub fn with_cli(mut self, cli: &Cli) -> Self {
        apply!(
            cli.timeout_seconds => self.http.timeout_seconds, |v| v;
            cli.max_redirects => self.http.max_redirects, |v| v as u32;
            cli.max_depth => self.crawlet.max_depth, |v| v as u32;
            cli.max_concurrent_requests => self.crawlet.max_concurrent_requests, |v| v as u32;
            cli.same_domain_only => self.crawlet.same_domain_only, |v| v;
            cli.check_external_links => self.checker.check_external_links, |v| v;
            cli.ignore_ssl_errors => self.checker.ignore_ssl_errors, |v| v;
            cli.retry_attempts => self.checker.retry_attempts, |v| v as u32;
            cli.output_format.as_deref() => self.output.output_format, OutputFormat::from;
            cli.show_successful => self.output.show_successful, |v| v;
        );

        self
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

#[derive(Debug, PartialEq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
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
                same_domain_only: true,
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
    use clap::Parser;

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
        assert!(settings.crawlet.same_domain_only);
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
        assert!(settings.crawlet.same_domain_only);
        assert!(settings.checker.check_external_links);
        assert!(!settings.checker.ignore_ssl_errors);
        assert_eq!(settings.checker.retry_attempts, 3);
        assert_eq!(settings.output.output_format, OutputFormat::Pretty);
        assert!(!settings.output.show_successful);
    }

    #[test]
    fn test_output_format_case_insensitive() {
        assert_eq!(OutputFormat::from("PRETTY"), OutputFormat::Pretty);
        assert_eq!(OutputFormat::from("JSON"), OutputFormat::Json);
    }

    #[test]
    fn test_settings_with_cli_overrides() {
        use crate::adapters::Cli;

        let base = Settings::default();
        let cli = Cli::parse_from([
            "ble",
            "https://example.com/",
            "--timeout-seconds",
            "60",
            "--max-redirects",
            "10",
            "--max-depth",
            "5",
            "--max-concurrent-requests",
            "20",
            "--same-domain-only",
            "false",
            "--check-external-links",
            "false",
            "--ignore-ssl-errors",
            "false",
            "--retry-attempts",
            "5",
            "--output-format",
            "json",
            "--show-successful",
            "true",
        ]);
        let settings = base.with_cli(&cli);

        assert_eq!(settings.http.timeout_seconds, 60);
        assert_eq!(settings.http.max_redirects, 10);
        assert_eq!(settings.crawlet.max_depth, 5);
        assert_eq!(settings.crawlet.max_concurrent_requests, 20);
        assert!(!settings.crawlet.same_domain_only);
        assert!(!settings.checker.check_external_links);
        assert!(!settings.checker.ignore_ssl_errors);
        assert_eq!(settings.checker.retry_attempts, 5);
        assert_eq!(settings.output.output_format, OutputFormat::Json);
        assert!(settings.output.show_successful);
    }
}
