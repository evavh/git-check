use std;
use std::process::Command;

use std::path::PathBuf;

use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Status {
    UncommittedChanges,
    DivergedFromRemote,
    UnpushedChanges,
    UntrackedFiles,
    Clean,
    Other(String),
}

impl FromStr for Status {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let var = if str.contains("not staged for commit")
            || str.contains("Changes to be committed")
        {
            Self::UncommittedChanges
        } else if str.contains("have diverged") {
            Self::DivergedFromRemote
        } else if str.contains("is ahead of") {
            Self::UnpushedChanges
        } else if str.contains("Untracked files") {
            Self::UntrackedFiles
        } else if str.contains("nothing to commit, working tree clean") {
            Self::Clean
        } else {
            Self::Other(str.to_string())
        };

        Ok(var)
    }
}

#[derive(Debug)]
pub(crate) enum Remote {
    Yes,
    No,
}

impl FromStr for Remote {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let var = if str.is_empty() { Self::No } else { Self::Yes };

        Ok(var)
    }
}

#[derive(Debug)]
pub(crate) enum Error {
    NoRepo,
    Other(String),
}

impl FromStr for Error {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let var = if str.contains("not a git repository") {
            Self::NoRepo
        } else {
            Self::Other(str.to_string())
        };

        Ok(var)
    }
}

#[derive(Debug)]
pub(crate) struct RepoState {
    pub(crate) name: String,
    pub(crate) status: Status,
    pub(crate) remote: Remote,
    pub(crate) error: Error,
}

impl RepoState {
    pub(crate) fn new(path: &PathBuf) -> Self {
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let status_output = git_command("status", path);
        let status = std::str::from_utf8(&status_output.stdout).unwrap();
        let status = Status::from_str(status).unwrap();
        let error = std::str::from_utf8(&status_output.stderr).unwrap();
        let error = Error::from_str(error).unwrap();

        let remote_output = git_command("remote", path);
        let remote = std::str::from_utf8(&remote_output.stdout).unwrap();
        let remote = Remote::from_str(remote).unwrap();

        Self {
            name,
            status,
            remote,
            error,
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
