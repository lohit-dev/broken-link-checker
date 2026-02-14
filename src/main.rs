use std::sync::Arc;

use ble::{context::AppContext, settings::Settings};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let settings = Settings::new()?;
    let ctx = Arc::new(AppContext::new(settings));
    let _ = ctx;
    Ok(())
}
