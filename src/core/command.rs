use std::path::PathBuf;
use std::sync::Arc;

use eyre::eyre;
use url::Url;

use crate::context::AppContext;
use crate::core::Checker;

pub enum CheckCommand {
    CheckFile { file: PathBuf },
    CheckUrl { url: Url },
}

pub async fn execute_command(ctx: Arc<AppContext>, command: CheckCommand) -> eyre::Result<()> {
    let mut checker = Checker::new(Arc::clone(&ctx));
    match command {
        CheckCommand::CheckFile { file } => {
            let url = Url::from_file_path(&file)
                .map_err(|_| eyre!("Invalid file path: {}", file.display()))?;
            checker.check(&url).await?;
        }
        CheckCommand::CheckUrl { url } => {
            checker.check(&url).await?;
        }
    }

    Ok(())
}
