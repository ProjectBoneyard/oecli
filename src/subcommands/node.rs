//! Commands for interacting with Node and Node Package Manager CLI tools.

use crate::step::{ShouldRunResult, StepItem};
use async_trait::async_trait;

/// Runs [npm install]() in the provided directory.
pub struct NPMInstall {
    /// Path relative to the current directory.
    path: String,
}

impl NPMInstall {
    pub fn new(path: &str) -> NPMInstall {
        NPMInstall {
            path: path.to_owned(),
        }
    }
}

#[async_trait]
impl StepItem for NPMInstall {
    fn title(&self) -> String {
        "Running npm install".to_string()
    }

    fn description(&self) -> String {
        "Will run 'npm install' in the newly provided directory".to_string()
    }

    async fn should_run(&self) -> ShouldRunResult {
        // Check to make sure the directory exists
        ShouldRunResult::Ok
    }

    /// Runs `npm install` in the provided directory.
    /// TODO: Parse the output and return the correct error.
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let _ = tokio::process::Command::new("gh")
            .arg("install")
            .current_dir(format!("./{}", &self.path))
            .output()
            .await;
        Ok("".to_string())
    }
}
