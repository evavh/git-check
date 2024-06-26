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

    let repos = fs::read_dir(path)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.path())
        .map(|path| RepoState::new(&path));

    dbg!(repos.collect::<Vec<_>>());
}
