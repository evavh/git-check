use std::path::Path;
use std::{fs, path::PathBuf};

use clap::Parser;
use strum::IntoEnumIterator;

use crate::repo::Repo;
use crate::status::Status;

mod repo;
mod status;

const IGNORE_FILENAME: &str = "git_check_ignore";

#[derive(Parser, Debug)]
#[command(
    about = "Utility to check the status of all git repos within a directory"
)]
#[command(version)]
struct Args {
    /// Parent directory to check for git repos (default: current directory)
    #[arg(default_value=std::env::current_dir().unwrap().into_os_string())]
    #[arg(hide_default_value = true)]
    path: PathBuf,
    /// Show the names of clean repos (instead of a count)
    #[arg(short = 'c', long)]
    show_clean: bool,
    /// File in the parent directory that contains subdirectory
    /// names to ignore
    /// The hidden file (.FILENAME) will be checked if it doesn't
    /// exist
    /// (default: git_check_ignore / .git_check_ignore)
    #[arg(short, long)]
    #[arg(default_value=IGNORE_FILENAME)]
    #[arg(hide_default_value = true)]
    #[arg(value_name="FILENAME")]
    #[clap(verbatim_doc_comment)]
    ignore_file: String,
}

fn main() {
    let Args {
        path: base_path,
        show_clean,
        ignore_file,
    } = Args::parse();

    let ignored_paths = ignored_paths(&ignore_file, &base_path);
    let repos: Vec<_> = fs::read_dir(base_path)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.path())
        .filter(|path| !ignored_paths.contains(path))
        .map(|path| Repo::new(&path))
        .collect();

    for status_variant in Status::iter() {
        let repos: Vec<_> = repos
            .clone()
            .into_iter()
            .filter(|r| r.status.is_same_variant(&status_variant))
            .collect();

        if !repos.is_empty() {
            if status_variant == Status::Clean && !show_clean {
                println!("\nClean: {} repos", repos.len());
            } else {
                println!("\n{status_variant}:");
                for repo in repos {
                    println!(" - {}", repo.name_and_content());
                }
            }
        }
    }
}

fn ignored_paths(ignore_filename: &str, base_path: &Path) -> Vec<PathBuf> {
    let path = base_path.join(ignore_filename);
    let hidden_path = base_path.join(".".to_owned() + ignore_filename);

    let file_contents = fs::read_to_string(path).unwrap_or_else(|_| {
        fs::read_to_string(hidden_path).unwrap_or_default()
    });

    file_contents
        .lines()
        .map(|f| base_path.join(f))
        .map(PathBuf::from)
        .collect()
}
