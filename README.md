# Git Branch Cleanup Tool

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/git-branch-cleanup-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`git-branch-cleanup-cli` is a command-line utility written in Rust designed to identify, inspect, and purge merged or stale local Git branches safely and efficiently.

## Key Capabilities

- **Merge Inspection**: Automatically detects local branches merged into a specified base branch.
- **Age-Based Filtering**: Target stale branches older than a specified number of days.
- **Protected Branches**: Shield critical branches from accidental deletion via patterns.
- **Dry-Run Mode**: Preview exact cleanup actions without modifying the repository state.
- **JSON Export**: Output scan results in JSON format for automated scripting and CI/CD pipelines.

## Installation

```bash
cargo install git-branch-cleanup-cli
```

## Usage

```bash
# Interactively clean up merged branches against main
git-branch-cleanup-cli

# Preview deletions without executing them
git-branch-cleanup-cli --dry-run

# Export branch status as JSON
git-branch-cleanup-cli --json
```
