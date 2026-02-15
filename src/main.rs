use std::process;
use std::sync::Arc;

use broken_link_checker::adapters::Cli;
use broken_link_checker::core::command::{CheckCommand, execute_command};
use broken_link_checker::{context::AppContext, settings::Settings};
use clap::{CommandFactory, Parser};
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    setup_tracing()?;
    let cli = Cli::parse();

    if cli.url.is_none() && cli.file.is_none() {
        error!("Either --url or --file must be provided");
        eprintln!("{}", Cli::command().render_help());
        process::exit(1);
    }

    let settings = Settings::new()?.with_cli(&cli);
    let ctx = Arc::new(AppContext::new(settings));
    let command = CheckCommand::from(&cli);
    execute_command(ctx, command).await?;
    Ok(())
}

fn setup_tracing() -> eyre::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .without_time()
                .with_level(false),
        )
        .init();

    Ok(())
}
