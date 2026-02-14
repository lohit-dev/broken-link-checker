use clap::Parser;
use std::path::PathBuf;
use url::Url;

use crate::core::command::CheckCommand;

pub mod http;
pub mod parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(conflicts_with = "file")]
    pub url: Option<String>,

    #[arg(short, long, conflicts_with = "url")]
    pub file: Option<PathBuf>,

    // ============================================
    // HTTP Settings
    // ============================================
    #[arg(long, value_name = "SECONDS")]
    pub timeout_seconds: Option<u64>,

    #[arg(long, value_name = "COUNT")]
    pub max_redirects: Option<u8>,

    // ============================================
    // Crawler Settings
    // ============================================
    #[arg(long, short = 'd', value_name = "DEPTH")]
    pub max_depth: Option<usize>,

    #[arg(long, short = 'c', value_name = "COUNT")]
    pub max_concurrent_requests: Option<usize>,

    #[arg(long)]
    pub same_domain_only: Option<bool>,

    // ============================================
    // Checker Settings
    // ============================================
    #[arg(long)]
    pub check_external_links: Option<bool>,

    #[arg(long)]
    pub ignore_ssl_errors: Option<bool>,

    #[arg(long, value_name = "COUNT")]
    pub retry_attempts: Option<u8>,

    // ============================================
    // Output Settings
    // ============================================
    #[arg(long, short = 'o', value_name = "FORMAT")]
    pub output_format: Option<String>,

    #[arg(long)]
    pub show_successful: Option<bool>,
}

impl From<&Cli> for CheckCommand {
    fn from(cli: &Cli) -> Self {
        match (&cli.file, &cli.url) {
            (Some(path), _) => CheckCommand::CheckFile {
                file: path.to_path_buf(),
            },
            (None, Some(url_str)) => {
                let url = Url::parse(url_str).unwrap_or_else(|e| {
                    eprintln!("Error: Failed to parse URL '{}': {:?}", url_str, e);
                    std::process::exit(1);
                });
                CheckCommand::CheckUrl { url }
            }

            (None, None) => {
                eprintln!("Error: Either file or url must be provided");
                std::process::exit(1);
            }
        }
    }
}
