use std::process;
use std::sync::Arc;

use ble::adapters::Cli;
use ble::core::command::{CheckCommand, execute_command};
use ble::{context::AppContext, settings::Settings};
use clap::{CommandFactory, Parser};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    if cli.url.is_none() && cli.file.is_none() {
        eprintln!("{}", Cli::command().render_help());
        process::exit(1);
    }

    let settings = Settings::new()?;
    let ctx = Arc::new(AppContext::new(settings));
    let command = CheckCommand::from(&cli);
    execute_command(ctx, command).await?;
    Ok(())
}
