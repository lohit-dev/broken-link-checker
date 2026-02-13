use broken_link_checker::settings::Settings;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _settings = Settings::new()?;
    Ok(())
}
