//! Commands for interacting with Node and Node Package Manager CLI tools.

/// Runs npm install. Requires a string parameter and is used as the relative
/// directory to run the command in.
pub fn npm_install(dir: &str) -> () {
    std::process::Command::new("npm")
        .arg("install")
        .current_dir(format!("./{}", &dir))
        .output()
        .expect("Failed to run npm install in new directory");
}
