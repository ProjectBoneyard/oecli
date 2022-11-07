//! Various subcommands for the working within oedev.

mod age;
mod cloud_home;
mod filesystem;
mod github;
mod node;
mod precommit;
mod pwa;

#[cfg(feature = "testcmd")]
use crate::command::CLIStepExecutor;
use clap::Subcommand;

/// OeCli commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Managing the lifecycle of progressive web apps.
    Pwa(pwa::Pwa),
    /// Manage K3s (Kubernetes) cluster backed by flux and a GitOps workflow.
    CloudHome(cloud_home::CloudHome),
    #[cfg(feature = "testcmd")]
    /// Provides functionality to test the execution of commands and the output.
    Test(crate::test::TestCommand),
}

impl Commands {
    pub async fn process(self) -> Result<(), String> {
        match self {
            Commands::Pwa(pwa) => pwa.process().await,
            Commands::CloudHome(cloud) => cloud.process().await,
            #[cfg(feature = "testcmd")]
            Commands::Test(t) => CLIStepExecutor::execute(&t).await,
        }
    }
}
