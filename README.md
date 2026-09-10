# Git Branch Cleanup Tool

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/git-branch-cleanup-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`git-branch-cleanup-cli` is a command-line utility written in Rust designed to identify, inspect, and purge merged or stale local Git branches safely and efficiently.

## Key Capabilities

- **Merge Inspection**: Automatically detects local branches already merged into a specified base branch (default: `main`).
- **Age-Based Stale Detection**: Filter branches by inactivity using last commit timestamps (`--older-than`).
- **Configurable Protection**: Regex-based branch protection rules to prevent accidental deletion of critical long-lived branches.
- **Interactive Safety Prompts**: Review candidate branches and reasons before executing deletions, with dry-run support.

## Terminal Demonstration

```bash
$ git-branch-cleanup-cli --base main --older-than 30

==> Inspecting local branches against base 'main'...

Found 2 candidate branch(es) for cleanup:

  feature/old-login [stale] (last commit: 45d ago, author: Alice) - reason: stale
  bugfix/typo [merged, stale] (last commit: 62d ago, author: Bob) - reason: merged + stale

[CONFIRM] Proceed with deleting these branches? [y/N]: y

Deleting feature/old-login... [OK] succeeded
Deleting bugfix/typo... [OK] succeeded

Branch cleanup completed.
```

## Quickstart

Clone the repository and build the binary via Cargo:

```bash
git clone https://github.com/Olamideakinade/git-branch-cleanup-cli.git
cd git-branch-cleanup-cli
cargo build --release
```

Run the executable inside any git repository:

```bash
./target/release/git-branch-cleanup-cli --dry-run
```

## Options

```bash
$ git-branch-cleanup-cli --help
Usage: git-branch-cleanup-cli [OPTIONS]

Options:
  -b, --base <BASE>                     Base branch to check merge status against [default: main]
  -o, --older-than <OLDER_THAN>         Delete branches older than specified days
  -d, --dry-run                         Only show branches, do not prompt for deletion
  -f, --force                           Force deletion of unmerged branches (-D)
  -y, --yes                             Skip interactive confirmation prompts
      --protect-pattern <PROTECT_PATTERN> Regex pattern of branch names to protect from deletion [default: ^main$|^master$|^dev$|^staging$]
  -h, --help                            Print help
  -V, --version                         Print version
```

## Architecture & Design

- **Core Engine (`branch.rs`)**: Interfaces directly with standard `git` binary via safe subprocess execution, parsing `git for-each-ref` and `git branch --merged` outputs efficiently.
- **CLI Parsing (`cli.rs`)**: Uses `clap` with derive macros for robust command-line argument validation.
- **Safety Guardrails**: Strict protection matching ensures default trunk and release branches are never selected for deletion.

## License

MIT
