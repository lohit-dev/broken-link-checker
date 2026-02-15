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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use url::Url;

    use super::*;

    #[test]
    fn test_check_command_check_url_variant() {
        let url = Url::parse("https://example.com").unwrap();
        let cmd = CheckCommand::CheckUrl { url };
        match &cmd {
            CheckCommand::CheckUrl { url: u } => assert_eq!(u.as_str(), "https://example.com/"),
            _ => panic!("expected CheckUrl"),
        }
    }

    #[test]
    fn test_check_command_check_file_variant() {
        let file = PathBuf::from("/tmp/index.html");
        let cmd = CheckCommand::CheckFile { file: file.clone() };
        match &cmd {
            CheckCommand::CheckFile { file: f } => assert_eq!(f, &file),
            _ => panic!("expected CheckFile"),
        }
    }
}
