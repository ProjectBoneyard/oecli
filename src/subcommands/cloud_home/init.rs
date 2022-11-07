//! Initialize a new OECloud@Home installation.

use crate::command::CLIStepExecutor;
use crate::step::{ExecutorProperties, Step, StepSequence};
use crate::subcommands::age::Age;
use crate::subcommands::filesystem::{CopyFile, CreateFile};
use crate::subcommands::github::{CloneRepo, CreateTemplateRepo};
use crate::subcommands::precommit::{PreCommit, PreCommitCommand};
use async_trait::async_trait;
use clap::Args;

#[derive(Args, Clone, Debug)]
pub struct CloudHomeInit {
    /// Name of cloud, will become the git repository name.
    #[clap(long)]
    name: String,
    /// If the newly created repository should be public. Defaults to private.
    #[clap(long, short)]
    public: Option<bool>,
}

#[async_trait]
impl CLIStepExecutor for CloudHomeInit {
    async fn set_properties(&self, cmd_props: ExecutorProperties) -> ExecutorProperties {
        let name = self.name.clone();
        let public = self.public.unwrap_or(true);

        let create_template_repo =
            CreateTemplateRepo::new(&name, "k8s-at-home/flux-cluster-template", public);
        let clone_repo = CloneRepo::new(&name);

        let pwa_toml = CreateFile::new(&format!("{}/oecloudhome.toml", &name));
        let cpy_config = CopyFile::new(
            &format!("{}/.config.sample.env", &name),
            &format!("{}/.config.env", &name),
        );

        let precommit_init = PreCommit::new(PreCommitCommand::Init, &self.name);
        let precommit_update = PreCommit::new(PreCommitCommand::Update, &self.name);

        let precommit_sequence =
            StepSequence::new("Run pre-commit hooks that come with the repository.", "")
                .set_steps(vec![Step::Step(Box::new(precommit_init))])
                .then_run(Step::Step(Box::new(precommit_update)));

        // Configuration

        let sequence = StepSequence::new("Set up cloud home repository", "")
            .then_run(Step::Step(Box::new(create_template_repo)))
            .then_run(Step::Step(Box::new(clone_repo)))
            .then_run_parallel(vec![
                Step::Step(Box::new(pwa_toml)),
                Step::Sequence(precommit_sequence),
                Step::Step(Box::new(cpy_config)),
            ])
            .then_run(Step::Step(Box::new(Age::new(&name))));

        cmd_props.then_run_parallel(vec![Step::Sequence(sequence)])
    }
}

////// Should grep the output and if an error is found print which command failed.
//std::process::Command::new("task")
//.arg("init")
//.output()
//.expect("");
//
//// Set up pre-commit hooks to ensure we are not committing secrets
//std::process::Command::new("task")
//.arg("precommit:init")
//.output()
//.expect("");
//// Catch occasional failures & verify results
//std::process::Command::new("task")
//.arg("precommit:update")
//.output()
//.expect("");
//
//// Check if config.env exists if not copy it from the sample
//if !std::path::Path::new("config.env").exists() {
//std::process::Command::new("cp")
//.arg(".config.sample.env")
//.arg(".config.env")
//.output()
//.expect("Was unable to copy config.env.sample to config.env");
//}
//
//// Check if age keys exist if not create an Age Private / Public keypair for use with
//// Ansible, Terraform, and Flux
//if !std::path::Path::new("/home/oe/.config/sops/age/keys.txt").exists() {
//std::process::Command::new("age-keyage")
//.arg("-o")
//.arg("age.agekey")
//.output();
//
//// Setup the directory and move the Age file to it
//std::process::Command::new("mkdir")
//.arg("-p")
//.arg("/home/oe/.config/sops/age")
//.output()
//.expect("");
//std::process::Command::new("mv")
//.arg("age.agekey")
//.arg("/home/oe/.config/sops/age/keys.txt")
//.output()
//.expect("");
//}
//
//// Check if the environment variable SOPS_AGE_KEY_FILE exists if not set it to keys.txt
//if !std::env::var("SOPS_AGE_KEY_FILE").is_ok() {
//std::env::set_var("SOPS_AGE_KEY_FILE", "/home/oe/.config/sops/age/keys.txt");
//}
//
//// Parse .config.env and makesure the age public key exists under BOOTSTRAP_AGE_PUBLIC_KEY
//let mut file = std::fs::File::open(".config.env").expect("Unable to read config.env");
//let mut contents = String::new();
//file.read_to_string(&mut contents)
//.expect("Unable to read config.env");
//let re = Regex::new("BOOTSTRAP_AGE_PUBLIC_KEY=\"(.*)\"$").unwrap();
//
//let mut age_public_key = std::fs::File::open("/home/oe/.config/sops/age/keys.txt")
//.expect("Unable to read age public key");
//let mut age_public_key_contents = String::new();
//age_public_key
//.read_to_string(&mut age_public_key_contents)
//.expect("Unable to read age public key");
//let re_pub_key = Regex::new("public key: (age.*)").unwrap();
//let pub_key = re_pub_key
//.captures(&age_public_key_contents)
//.unwrap()
//.get(1)
//.unwrap()
//.as_str();
//
//let caps = re.captures(&contents).unwrap();
//if caps.get(1).is_none() || caps.get(1).unwrap().as_str() != pub_key {
//let env_file = re.replace(&contents, |caps: &regex::Captures| {
//format!("BOOTSTRAP_AGE_PUBLIC_KEY=\"{}\"", pub_key)
//});
//file.write_all(env_file.as_bytes())
//.expect("Unable to write to config.env");
//}
