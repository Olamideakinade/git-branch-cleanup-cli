# Git Branch Cleanup Tool

[![GitHub Repository](https://img.shields.io/badge/GitHub-Repository-181717?style=for-the-badge&logo=github)](https://github.com/Olamideakinade/git-branch-cleanup-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Project Snapshot](preview.svg)

`git-branch-cleanup-cli` is a high-performance command-line utility written in Rust designed to identify, inspect, and purge merged or stale local Git branches safely and efficiently.

## Key Capabilities

- **Merge Inspection**: Automatically detects local branches merged into your base branch.
- **Stale Branch Filtering**: Filter branches by inactivity duration (days since last commit).
- **Protected Patterns**: Define regex or glob patterns to prevent accidental deletion of critical branches.
- **Interactive TUI**: Beautiful, animated multi-select prompts with live previews and search.
- **JSON Export**: Export scan results for automated CI/CD pipeline reporting.

## Installation

```bash
cargo install git-branch-cleanup-cli
```

## Usage

Run the cleanup utility in your repository:

```bash
git-branch-cleanup
```

### CLI Options

```bash
# Check against a custom base branch
git-branch-cleanup --base develop

# Target branches older than 30 days
git-branch-cleanup --older-than 30

# Dry run mode (preview deletions without executing)
git-branch-cleanup --dry-run

# Export branch metadata to JSON
git-branch-cleanup --json
```

## License

Licensed under the MIT License.
