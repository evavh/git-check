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
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long)]
    show_clean: bool,
}

fn main() {
    let Args {
        path: base_path,
        show_clean,
    } = Args::parse();

    let ignored_paths = ignored_paths(&base_path);
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

fn ignored_paths(base_path: &Path) -> Vec<PathBuf> {
    let path = base_path.join(IGNORE_FILENAME);
    let hidden_path = base_path.join(".".to_owned() + IGNORE_FILENAME);

    let file_contents = fs::read_to_string(path).unwrap_or_else(|_| {
        fs::read_to_string(hidden_path).unwrap_or_default()
    });

    file_contents
        .lines()
        .map(|f| base_path.join(f))
        .map(PathBuf::from)
        .collect()
}
