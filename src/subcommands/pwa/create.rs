//! Creates a new progressive web app.

use crate::command::CLIStepExecutor;
use crate::step::{ExecutorProperties, Step, StepSequence};
use crate::subcommands::github::{CloneRepo, CreateTemplateRepo};
use crate::subcommands::node::NPMInstall;
use async_trait::async_trait;
use clap::Args;

/// Using the name provided will clone a Github Template Repository and run `npm install` on the
/// newly cloned repository.
#[derive(Args, Clone, Debug)]
pub struct PwaCreate {
    /// Name of Progressive Web App, will become the git repository name.
    #[clap(long)]
    name: String,
    /// If the newly created repository should be public. Defaults to private.
    #[clap(long, short)]
    public: Option<bool>,
}

#[async_trait]
impl CLIStepExecutor for PwaCreate {
    /// Create the new template repository from `ctron/patternfly-yew-quickstart` and clone that
    /// repository into a new subdirectory with the same name. Under the new directory, runs `npm
    /// install`.
    async fn set_properties(&self, cmd_props: ExecutorProperties) -> ExecutorProperties {
        let name = &self.name;
        let public = self.public.unwrap_or(false);

        let create_template_repo =
            CreateTemplateRepo::new(name, "ctron/patternfly-yew-quickstart", public);
        let clone_repo = CloneRepo::new(name);
        let npm_install = NPMInstall::new(name);

        let sequence = StepSequence::new("Set up cloud home repository", "")
            .then_run(Step::Step(Box::new(create_template_repo)))
            .then_run(Step::Step(Box::new(clone_repo)))
            .then_run(Step::Step(Box::new(npm_install)));

        cmd_props.then_run_parallel(vec![Step::Sequence(sequence)])
    }
}
