use std::path::PathBuf;
use std::sync::Arc;

use url::Url;

use crate::context::AppContext;

pub enum CheckCommand {
    CheckFile { file: PathBuf },
    CheckUrl { url: Url },
}

pub async fn execute_command(ctx: Arc<AppContext>, command: CheckCommand) -> eyre::Result<()> {
    match command {
        CheckCommand::CheckFile { file } => {
            let _ = (ctx, file);
            todo!()
        }
        CheckCommand::CheckUrl { url } => {
            let _ = (ctx, url);
            todo!()
        }
    }
}
