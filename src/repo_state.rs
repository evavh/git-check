use std;
use std::process::Command;

use std::path::PathBuf;

use std::str::FromStr;

use strum::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
pub(crate) enum Status {
    NoRepo,
    Error(String),
    NoRemote,
    DetachedHead,
    UncommittedChanges,
    DivergedFromRemote,
    UnpushedChanges,
    UntrackedFiles,
    Clean,
    Unknown(String),
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Status as S;
        let str = match self {
            S::NoRepo => "Not a git repository",
            S::Error(s) => {
                if s.is_empty() {
                    "Error"
                } else {
                    "Error ({s})"
                }
            }
            S::NoRemote => "No remote",
            S::DetachedHead => "In detached head mode",
            S::UncommittedChanges => "Uncommitted changes",
            S::DivergedFromRemote => "Diverged from remote",
            S::UnpushedChanges => "Unpushed changes",
            S::UntrackedFiles => "Untracked files",
            S::Clean => "Clean",
            S::Unknown(s) => {
                if s.is_empty() {
                    "Unknown status"
                } else {
                    "Unknown status ({s})"
                }
            }
        };

        write!(f, "{str}")
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let var = if str.contains("HEAD detached") {
            Self::DetachedHead
        } else if str.contains("not staged for commit")
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
            Self::Unknown(str.to_string())
        };

        Ok(var)
    }
}

impl Status {
    pub fn is_same_variant(&self, other: &Self) -> bool {
        use Status as S;
        match (self, other) {
            (S::Error(_), S::Error(_)) => true,
            (S::Unknown(_), S::Unknown(_)) => true,
            (S::Error(_), _) => false,
            (S::Unknown(_), _) => false,
            (s, o) => s == o,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RepoState {
    pub(crate) name: String,
    pub(crate) status: Status,
}

impl RepoState {
    pub(crate) fn new(path: &PathBuf) -> Self {
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
        } else {
            if error.contains("not a git repository") {
                Status::NoRepo
            } else {
                Status::Error(error.to_string())
            }
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
