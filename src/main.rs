use std::{fs, path::PathBuf};

use clap::Parser;

use crate::repo_state::RepoState;

mod repo_state;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let Args { path } = Args::parse();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            let repo = RepoState::new(&path);
            dbg!(repo);
        }
    }
}
