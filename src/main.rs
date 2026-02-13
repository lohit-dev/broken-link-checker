use broken_link_checker::{context::AppContext, settings::Settings};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _settings = Settings::new()?;
    let _context = AppContext::default();
    Ok(())
}
