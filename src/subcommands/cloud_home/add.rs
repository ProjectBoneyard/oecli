//! CLIStepExecutor for adding a new device to a OECloud@Home installation.

use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct Add {
    /// IP address of the new node to add to the Kubernetes Cluster.
    #[clap(long)]
    ip: String,
}

impl Add {
    pub fn process(&self) -> Result<(), String> {
        Ok(())
    }
}
