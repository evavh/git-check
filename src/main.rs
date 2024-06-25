use std::{fs, path::PathBuf, process::Command, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Debug)]
enum Status {
    UncommittedChanges,
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
enum Remote {
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
enum Error {
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
struct Repo {
    name: String,
    status: Status,
    remote: Remote,
    error: Error,
}

impl Repo {
    fn new(path: &PathBuf) -> Self {
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

fn main() {
    let Args { path } = Args::parse();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            let repo = Repo::new(&path);
            dbg!(repo);
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
