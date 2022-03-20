use clap::{Args, Parser, Subcommand};

/// oicli is a command line interface to provide a productivity boost by
/// handling boilerplate and some operational overhead with development within
/// the Overengineered ecosystem.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
            create_repo(&name.new);
            npm_install(&name.new);
        }
    }
}

/// Will create a new Github repository based on the PatternFly quick start
/// for Yew template.
fn create_repo(name: &str) -> () {
    std::process::Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(&name)
        .arg("--template")
        .arg("ctron/patternfly-yew-quickstart")
        .arg("--clone")
        .arg("--public")
        .output()
        .expect("");
}

/// Runs npm install. Requires a string parameter and is used as the relative
/// directory to run the command in.
fn npm_install(dir: &str) -> () {
    std::process::Command::new("npm")
        .arg("install")
        .current_dir(format!("./{}", &dir))
        .output()
        .expect("");
}
