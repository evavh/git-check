//! Contains the Status enum, which defines common possible states

use std::str::FromStr;

use strum::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
/// Defines common states for a git repo to be in, including errors and unknown
/// status messages
pub enum Status {
    /// Not a git repository
    NoRepo,
    /// Some error
    Error(String),
    /// No remote set
    NoRemote,
    /// Repo is in detached head mode
    DetachedHead,
    /// There are uncommitted changes
    UncommittedChanges,
    /// The local repo has diverged from the remote
    DivergedFromRemote,
    /// There are unpushed changes
    UnpushedChanges,
    /// There are untracked files
    UntrackedFiles,
    /// The repo is clean and up to date
    Clean,
    /// Some other, as of yet unknown, status
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
    #[allow(clippy::match_same_arms)]
    /// Compare the enum variant, but not what's inside
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
