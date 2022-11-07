//! Oecli is a command line tool to meant to be used with the [oedev]() docker dev environment.
//! The dev environment comes packaged with dependencies that this tool interacts with. Oecli
//! attempts to provide a productivity boost by handling boilerplate and some operational overhead
//! with development within the Overengineered ecosystem.

mod command;
mod log;
mod step;
mod subcommands;
mod test;

use clap::Parser;
use subcommands::Commands;

/// The main entry point for oecli, uses clap to parse the command line arguments into several
/// subcommands.
///
/// Usage:
///
/// `oecli <subcommend>`
///
/// List of subcommands include:
/// * **PWA** - Progressive web app management. Create, deploy, and manage progressive web apps.
/// * **Cloud Home** - Manage K3s (Kubernetes) cluster backed by flux and a GitOps workflow.
///
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = cli.command.process().await;
    if let Err(err) = result {
        eprintln!("\x1b[93mError has occurred:\x1b[0m {:#?}", err);
        std::process::exit(1);
    }
}
