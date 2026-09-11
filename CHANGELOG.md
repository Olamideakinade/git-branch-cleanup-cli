# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2024-06-01
### Added
- Enhanced visual polish, styling, and animated spinners across all scan phases.
- Rich interactive multi-select interface with detailed branch metadata displays.
- Live status indicators and color-coded status badges for merged and active states.

### Changed
- Overhauled CLI output layout to present clearer summary tables.
- Upgraded progress indicator styling for better visual feedback during heavy repository analysis.

### Fixed
- Resolved rendering inconsistencies in terminal prompts across different ANSI-compatible terminals.

## [1.2.0] - 2024-03-01
### Added
- Added `--dry-run` flag for previewing branch deletions.
- Added `--protect` flag to define protected branch patterns.
- Added `--json` export option for CI/CD integration.

## [1.1.0] - 2023-10-27
### Added
- Interactive confirmation prompts via `dialoguer`.
- Visual progress bars for scanning operations.
- Enhanced color-coded terminal output.

### Changed
- Updated `clap` to 4.4 and added `indicatif` dependency.
- Refactored main loop for better stream handling.

### Fixed
- Resolved silent failures in git command execution.
