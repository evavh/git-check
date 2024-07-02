# Git check

A Rust utility to conveniently check the status of your git repositories. It groups all subdirectories under the supplied path by common git statuses. These states are currently checked for, in the following order:
- Not a git repository
- (Unknown) error
- No remote set
- In detached head mode
- Uncommitted changes
- Diverged from remote
- Unpushed changes
- Untracked files
- Clean
- Unknown / other status

## Usage

Download the source code and compile with `cargo build`.

```
$ git-check --help
A utility to check the status of all git repos within a directory

Usage: git-check [OPTIONS] [PATH]

Arguments:
  [PATH]  Parent directory to check for git repos (default: current directory)

Options:
  -c, --show-clean              Show the names of clean repos (instead of a count)
  -i, --ignore-file <FILENAME>  File in the parent directory that contains subdirectory
                                names to ignore
                                The hidden file (.FILENAME) will be checked if it doesn't
                                exist
                                (default: git_check_ignore / .git_check_ignore)
  -h, --help                    Print help
  -V, --version                 Print version
```

## Feedback

This project is currently in a state where it is useful for me, but if you have suggestions for new features, or encounter any bugs, feel free to create an issue!
