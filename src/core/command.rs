use std::path::PathBuf;

use url::Url;

use crate::context::AppContext;

pub enum CheckCommand {
    CheckFile { file: PathBuf },
    CheckUrl { url: Url },
}

pub async fn execute_command(ctx: &AppContext, command: CheckCommand) -> eyre::Result<()> {
    match command {
        CheckCommand::CheckFile { file } => {
            todo!()
        }
        CheckCommand::CheckUrl { url } => {
            todo!()
        }
    }
}
