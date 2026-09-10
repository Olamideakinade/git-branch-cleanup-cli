# Changelog

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
