//! oecli is a command line interface to provide a productivity boost by
//! handling boilerplate and some operational overhead with development within
//! the Overengineered ecosystem.

use clap::{Args, Parser, Subcommand};

mod github;
mod node;

/// oecli parser
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Subcommand passed into oecli
    #[clap(subcommand)]
    command: Commands,
}

/// Different sub command line options available with oecli
#[derive(Subcommand)]
enum Commands {
    /// Progressive Web App
    PWA(PWA),
}

/// Subcommand for interacting with Progressive Web Apps. Overengineered uses
/// Yew, which is a modern Rust framework for creating multi-threaded front-end
/// web applications using WebAssembly.
#[derive(Args, Debug)]
struct PWA {
    /// Will create a new Github repository with the provided name. This
    /// repository will use a template to create a Yew app using the PatternFly
    /// for a component library.
    #[clap(long)]
    new: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::PWA(name) => {
            github::create(&name.new);
            let username = github::logged_in_user();
            let full_repo = format!("{}/{}", &username, &name.new);
            github::clone(&full_repo);
            node::npm_install(&name.new);
        }
    }
}
