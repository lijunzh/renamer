# Changelog

## [v0.1.0] - 2024-XX-XX
### Added
- Added logging using `log` and `simplelog` crates.
- Implemented CLI parsing with `clap` crate.
- Added support for recursive file processing with `walkdir` crate.
- Added regex-based filename transformation.
- Added dry-run mode for testing renames without making changes.
- Added tests for filename transformation and file processing logic.

### Fixed
- Fixed borrow of moved value error for `new_path` in the main processing loop.

## [v0.0.7] - 2023-XX-XX
### Added
- Initial release with basic renaming functionality.
