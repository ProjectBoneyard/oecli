//! Commands and various utilities for managing github repositories.

use crate::step::{ShouldRunResult, StepItem};
use async_trait::async_trait;
use regex::Regex;
use tokio::process::Command;

/// Uses the GitHub CLI to check the authentication status. If we are logged in
/// we use regex to extract the username between "github.com as" and "(..."
async fn logged_in_user() -> String {
    let cmd = Command::new("gh").arg("auth").arg("status").output().await;
    // Should probably check the exit code here
    let output = String::from_utf8(cmd.unwrap().stderr).unwrap();
    let re = Regex::new(r"github.com as (.*) \(").unwrap();
    let cap = re.captures(&output).unwrap();
    cap[1].to_owned()
}

/// Clones a repo from github.com/<username>/<repo_name> to the current directory.
pub struct CloneRepo {
    repo_name: String,
}

impl CloneRepo {
    pub fn new(repo_name: &str) -> CloneRepo {
        CloneRepo {
            repo_name: repo_name.to_string(),
        }
    }
}

#[async_trait]
impl StepItem for CloneRepo {
    fn title(&self) -> String {
        format!("Cloning repo {}", self.repo_name)
    }

    fn description(&self) -> String {
        "Will clone the repo from the current logged in user.".to_string()
    }

    /// Checks to see if the folder already exists.
    async fn should_run(&self) -> ShouldRunResult {
        let folder_exists = std::path::Path::new(&self.repo_name).exists();
        if folder_exists {
            return ShouldRunResult::Skip;
        }
        return ShouldRunResult::Ok;
    }

    /// Uses [gh Cli]() to clone the provided repo for the current logged in user.
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let username = logged_in_user().await;
        let full_repo = format!("{}/{}", &username, &self.repo_name);
        let result = Command::new("gh")
            .arg("repo")
            .arg("clone")
            .arg(full_repo)
            .output()
            .await;
        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(format!("Repo {} cloned.", &self.repo_name))
                } else {
                    Err(format!(
                        "Failed to clone repo {}.\n{}",
                        &self.repo_name,
                        String::from_utf8(output.stderr).unwrap()
                    ))
                }
            }
            Err(e) => Err(format!("Failed to clone repo {}.\n{}", &self.repo_name, e)),
        }
    }
}

/// Will create a new Github repository based on the provided template.
/// Attempted to use octocrab or similar, but could not find clone template support.
/// Also another issue with the GitHub v4 API support, one of the dependencies does not support
/// cloning through the api.
pub struct CreateTemplateRepo {
    name: String,
    repo: String,
    public: bool,
}

impl CreateTemplateRepo {
    pub fn new(name: &str, repo: &str, public: bool) -> CreateTemplateRepo {
        CreateTemplateRepo {
            name: name.to_owned(),
            repo: repo.to_owned(),
            public,
        }
    }
}

#[async_trait]
impl StepItem for CreateTemplateRepo {
    fn title(&self) -> String {
        format!("Creating template from {}", self.repo)
    }

    fn description(&self) -> String {
        "Will check if the repo already exists, if it doesn't; Will clone it.".to_string()
    }

    /// Should check if the repo name already exists with the current logged in user.
    /// Bool should be an associative array. If any are false, or bad result, log
    /// error for that key.
    ///
    /// Checks if GH CLI is logged in and if the repo already exists.
    async fn should_run(&self) -> ShouldRunResult {
        let result = tokio::process::Command::new("gh")
            .arg("auth")
            .arg("status")
            .output()
            .await;
        match result {
            Ok(output) => {
                if !output.status.success() {
                    return ShouldRunResult::Error(
                        "GitHub CLI authentication failed. Make sure you are logged in.".to_owned(),
                    );
                }
            }
            Err(e) => {
                return ShouldRunResult::Error(format!(
                    "Github CLI had an unexpected failure.\n{}",
                    e.to_string(),
                ));
            }
        }
        let result = tokio::process::Command::new("gh")
            .arg("repo")
            .arg("view")
            .arg(&self.name)
            .output()
            .await;
        let err_str = format!(
            "GraphQL: Could not resolve to a Repository with the name '{}/{}'. (repository)",
            &self.repo, &self.name,
        );
        match result {
            Ok(output) => {
                if output.status.success() {
                    return ShouldRunResult::Skip;
                }
            }
            Err(e) => {
                if e.to_string() == err_str {
                    return ShouldRunResult::Ok;
                }
                return ShouldRunResult::Error(format!(
                    "Failed to check if repo {} exists.\n{}",
                    &self.name,
                    e.to_string(),
                ));
            }
        }
        ShouldRunResult::Ok
    }

    /// Users the [gh cli]() to create a new repo based on the provided template, under the current
    /// user.
    async fn execute(self: Box<Self>) -> Result<String, String> {
        let visibility = if self.public { "--public" } else { "--private" };

        let result = tokio::process::Command::new("gh")
            .arg("repo")
            .arg("create")
            .arg(&self.name)
            .arg("--template")
            .arg(self.repo)
            .arg(visibility)
            .output()
            .await;

        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(format!("Created Github repository {}", self.name))
                } else {
                    Err(format!(
                        "Failed to create repo {}.\n{}",
                        &self.name,
                        String::from_utf8(output.stderr).unwrap()
                    ))
                }
            }
            Err(e) => Err(format!("Failed to create repo {}.\n{}", &self.name, e)),
        }
    }
}
