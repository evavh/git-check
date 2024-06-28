use std::{fs, path::PathBuf};

use clap::Parser;
use strum::IntoEnumIterator;

use crate::repo::Repo;
use crate::status::Status;

mod repo;
mod status;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long)]
    show_clean: bool,
}

fn main() {
    let Args { path, show_clean } = Args::parse();

    let repos: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.path())
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
