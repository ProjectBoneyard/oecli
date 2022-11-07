//! Commands for interacting with Progress Web Apps or PWAs.

mod create;

use crate::command::CLIStepExecutor;
use clap::{Args, Subcommand};

/// Oecli subcommand for interacting with Progressive Web Apps. Overengineered uses Yew, a modern
/// Rust framework, for creating multi-threaded front-end web applications using WebAssembly.
///
/// Supports features such as create, deploy, install etc.
///
/// Usage:
/// `oecli pwa <subcommand>`
///
/// List of subcommands include:
/// * **create** - Create a new progressive web app.
/// * **deploy** - Deploy the pwa from the current directory to an oecloud.
/// * **search** - Search and discover progressive web apps from subscribed repositories.
/// * **install** - Install a progressive web app to an oecloud.
///
#[derive(Args, Debug)]
pub struct Pwa {
    #[clap(subcommand)]
    pub subcommand: PwaSubCommands,
}

impl Pwa {
    pub async fn process(&self) -> Result<(), String> {
        if let PwaSubCommands::Create(ref create) = self.subcommand {
            return CLIStepExecutor::execute(create).await;
        }
        Ok(())
    }
}

#[derive(Subcommand, Clone, Debug)]
pub enum PwaSubCommands {
    /// Will clone a template repository into a new local repository and initiate node.
    Create(create::PwaCreate),
    /// Will deploy the PWA from the current directory to the specified oecloud.
    ///
    /// !*Note:* Currently not implemented.
    ///
    Deploy,
    /// Will search and discover progressive web apps from subscribed repositories.
    ///
    /// !*Note:* Currently not implemented.
    ///
    Search,
    /// Will install a PWA
    ///
    /// !*Note:* Currently not implemented.
    ///
    Install,
}
