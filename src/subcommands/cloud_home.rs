//! Commands for interacting with OECloud@Home installation.

pub mod add;
pub mod init;

use crate::command::CLIStepExecutor;
use clap::{Args, Subcommand};

/// OECloud@Home is a kubernetes cluster meant to be installed barebones on x86, amd64, and arm.
///
/// The cluster is backed by flux and a GitOps workflow.
///
/// `oecli cloud-home <subcommand>`
///
/// List of subcommands include:
/// * *init* - Initialize a new OECloud@Home installation.
/// * *add* - Add a new device to a OECloud@Home installation.
/// * *remove* - Remove an existing device on a OECloud@Home installation.
///
#[derive(Args, Debug)]
pub struct CloudHome {
    /// Flux.
    #[clap(subcommand)]
    pub subcommand: CloudHomeSubCommands,
}

impl CloudHome {
    pub async fn process(&self) -> Result<(), String> {
        match self.subcommand {
            CloudHomeSubCommands::Add(ref args) => args.process(),
            CloudHomeSubCommands::Init(ref args) => CLIStepExecutor::execute(args).await,
            CloudHomeSubCommands::Remove => Ok(()),
        }
    }
}

/// OECloud@Home sub commands.
#[derive(Subcommand, Clone, Debug)]
pub enum CloudHomeSubCommands {
    /// Clones a template repository into a new local repository.
    Init(init::CloudHomeInit),
    /// Adds a new node to the Kubernetes Cluster.
    Add(add::Add),
    /// Removes a node from the Kubernetes Cluster.
    Remove,
}
