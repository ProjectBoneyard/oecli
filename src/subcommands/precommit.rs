use crate::step::{ShouldRunResult, StepItem};
use async_trait::async_trait;
use tokio::process::Command;

pub enum PreCommitCommand {
    Init,
    Update,
}

impl PreCommitCommand {
    pub fn precommit_subcommand(&self) -> String {
        match self {
            PreCommitCommand::Init => "precommit:init".to_string(),
            PreCommitCommand::Update => "precommit:update".to_string(),
        }
    }
}

/// Set up pre-commit hooks that come with the repository.
pub struct PreCommit {
    command: PreCommitCommand,
    // path to run under
    path: String,
}

impl PreCommit {
    pub fn new(command: PreCommitCommand, path: &str) -> PreCommit {
        PreCommit {
            command,
            path: path.to_string(),
        }
    }
}

#[async_trait]
impl StepItem for PreCommit {
    fn title(&self) -> String {
        format!("Running task {}", self.command.precommit_subcommand())
    }

    fn description(&self) -> String {
        format!(
            "Runs the pre-commit {} command",
            self.command.precommit_subcommand()
        )
    }

    async fn should_run(&self) -> ShouldRunResult {
        match self.command {
            PreCommitCommand::Init => ShouldRunResult::Ok,
            PreCommitCommand::Update => ShouldRunResult::Ok,
        }
    }

    ///
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let result = Command::new("task")
            .current_dir(format!("./{}", &self.path))
            .arg(self.command.precommit_subcommand())
            .output()
            .await;
        match result {
            Ok(output) => {
                if output.status.success() {
                    return Ok(format!(
                        "Running `task {}`",
                        self.command.precommit_subcommand()
                    ));
                } else {
                    return Err(format!(
                        "Error Running `task {}` Msg: {}",
                        self.command.precommit_subcommand(),
                        String::from_utf8(output.stderr).unwrap()
                    ));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Running `task {}` Msg: {}",
                    self.command.precommit_subcommand(),
                    e.to_string(),
                ));
            }
        }
    }
}
