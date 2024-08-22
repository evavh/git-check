//! Contains the Repo struct, which parses git commands and contains information on
//! a git repository

use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use crate::status::Status;

const KB: u64 = 1024;
const MB: u64 = KB.pow(2);
const GITHUB_SOFT_LIMIT: u64 = 50 * MB;
/// bytes per char (ascii), chars per line, lines per file
const MOST_CODE: u64 = 1 * 20 * 500;

#[derive(Debug, Clone)]
/// Information on a git repo
pub struct Repo {
    /// The path to the repo
    pub path: PathBuf,
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

        Self {
            path: path.to_path_buf(),
            name,
            status,
        }
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

    /// Use the du command to get and format some information on file sizes
    pub fn file_sizes(&self) -> String {
        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "find {} -type f -exec du -ab {{}} +",
                &self.path.to_string_lossy()
            ))
            .output()
            .unwrap()
            .stdout;
        let output = String::from_utf8(output).unwrap();
        let output = output
            .lines()
            .map(|line| line.split_whitespace().next().unwrap())
            .map(|num| u64::from_str(num).unwrap());

        let (small, large): (Vec<_>, Vec<_>) =
            output.partition(|bytes| *bytes < MOST_CODE);
        let (large, very_large): (Vec<_>, Vec<_>) = large
            .into_iter()
            .partition(|bytes| *bytes < GITHUB_SOFT_LIMIT);

        format_sizes(&small, &large, &very_large)
    }
}

fn format_sizes(small: &[u64], large: &[u64], very_large: &[u64]) -> String {
    let mut result = String::new();

    for (list, name_one, name_more) in [
        (small, "code-sized file", "code-sized files"),
        (large, "larger file", "larger files"),
        (very_large, "file too large for github", "files too large for github"),
    ] {
        if list.len() == 1 {
            result += &format!("{} {name_one}, ", list.len(),);
        } else if list.len() != 0 {
            result += &format!("{} {name_more}, ", list.len(),);
        }
    }

    result[..result.len() - 2].to_string()
}

fn git_command(command: &str, path: &Path) -> std::process::Output {
    let output = Command::new("git")
        .arg(command)
        .current_dir(path)
        .output()
        .unwrap();
    output
}
