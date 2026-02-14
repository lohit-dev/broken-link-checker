use broken_link_checker::{context::AppContext, settings::Settings};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;
    let _context = AppContext::new(settings);
    Ok(())
}
