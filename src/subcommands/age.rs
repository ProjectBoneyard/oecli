use crate::command::cmd;
use crate::step::{ShouldRunResult, StepItem};
use async_trait::async_trait;
use std::path::Path;
use tokio::process::Command;

///
pub struct Age {
    /// Name of the repository.
    name: String,
}

impl Age {
    pub fn new(name: &str) -> Age {
        Age {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl StepItem for Age {
    fn title(&self) -> String {
        "Set up age key for repo.".to_string()
    }

    fn description(&self) -> String {
        "Sets up the age key and installs it to the home directory.".to_string()
    }

    async fn should_run(&self) -> ShouldRunResult {
        // Check home directory to see if age key exists
        if Path::new(&format!("/home/oe/.config/sops/age/{}.txt", &self.name)).exists() {
            ShouldRunResult::Skip
        } else {
            ShouldRunResult::Ok
        }
    }

    ///
    async fn execute(self: Box<Self>) -> Result<String, String> {
        cmd(
            || {
                Command::new("age-keygen")
                    .arg("-o")
                    .arg(format!("age.agekey"))
                    .output()
            },
            &self.name,
        )
        .await?;
        cmd(
            || {
                Command::new("mkdir")
                    .arg("-p")
                    .arg(format!("/home/oe/.config/sops/age"))
                    .output()
            },
            &self.name,
        )
        .await?;
        cmd(
            || {
                Command::new("mv")
                    .arg("age.agekey")
                    .arg(format!("/home/oe/.config/sops/age/{}.txt", self.name))
                    .output()
            },
            &self.name,
        )
        .await?;

        // Add age key to .config.env
        Ok(format!(
            "Age key generated and moved to ~/.config/sops/age/{}.txt",
            self.name,
        ))
    }
}
