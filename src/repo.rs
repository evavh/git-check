use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use crate::status::Status;

#[derive(Debug, Clone)]
pub struct Repo {
    pub name: String,
    pub status: Status,
}

impl Repo {
    pub fn new(path: &PathBuf) -> Self {
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

    pub fn name_and_content(&self) -> String {
        match &self.status {
            Status::Error(s) | Status::Unknown(s) => {
                format!("{}:\n{s}", self.name)
            }
            _ => self.name.clone(),
        }
    }
}

fn git_command(command: &str, path: &PathBuf) -> std::process::Output {
    let output = Command::new("git")
        .arg(command)
        .current_dir(path)
        .output()
        .unwrap();
    output
}
