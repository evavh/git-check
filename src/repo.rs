//! Contains the Repo struct, which parses git commands and contains information on
//! a git repository

use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use crate::status::Status;

#[derive(Debug, Clone)]
/// Information on a git repo
pub struct Repo {
    /// The name of the subdirectory the repo is in
    pub name: String,
    /// The status of the repo, via git status and git remote
    pub status: Status,
}

impl Repo {
    /// Retrieve the information for the given path by running git status
    /// and git remote, and parsing them into a `Status`
    pub fn new(path: &Path) -> Self {
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let status_output = git_command("status", path);
        let status = std::str::from_utf8(&status_output.stdout).unwrap();
        let error = std::str::from_utf8(&status_output.stderr).unwrap();

        let remote_output = git_command("remote", path);
        let remote = std::str::from_utf8(&remote_output.stdout).unwrap();

        let status = if error.is_empty() {
            if remote.is_empty() {
                Status::NoRemote
            } else {
                Status::from_str(status).unwrap()
            }
        } else if error.contains("not a git repository") {
            Status::NoRepo
        } else {
            Status::Error(error.to_string())
        };

        Self { name, status }
    }

    /// Format the repo name and the *content* of the status (so not the status
    /// name itself) into a readable format
    pub fn name_and_content(&self) -> String {
        match &self.status {
            Status::Error(s) | Status::Unknown(s) => {
                format!("{}:\n{s}", self.name)
            }
            _ => self.name.clone(),
        }
    }
}

fn git_command(command: &str, path: &Path) -> std::process::Output {
    let output = Command::new("git")
        .arg(command)
        .current_dir(path)
        .output()
        .unwrap();
    output
}
