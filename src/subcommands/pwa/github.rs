//! Commands for interacting with git or github commands CLI tools.
use regex::Regex;

/// Will create a new Github repository based on the PatternFly quick start
/// for Yew template.
pub fn create(name: &str) -> () {
    std::process::Command::new("gh")
        .arg("repo")
        .arg("create")
        .arg(&name)
        .arg("--template")
        .arg("ctron/patternfly-yew-quickstart")
        .arg("--public")
        .output()
        .expect("");
}

/// Will clone a specific github repository.
pub fn clone(name: &str) -> () {
    std::process::Command::new("gh")
        .arg("repo")
        .arg("clone")
        .arg(&name)
        .output()
        .expect("");
}

/// Uses the GitHub CLI to check the authentication status. If we are logged in
/// we use regex to extract the username between "github.com as" and "(..."
pub fn logged_in_user() -> String {
    let cmd = std::process::Command::new("gh")
        .arg("auth")
        .arg("status")
        .output()
        .expect("");
    let output = String::from_utf8(cmd.stderr).unwrap();
    let re = Regex::new(r"github.com as (.*) \(").unwrap();
    let cap = re.captures(&output).unwrap();
    cap[1].to_owned()
}
