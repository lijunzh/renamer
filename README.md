# Renamer CLI Tool

A Rust-based CLI tool for bulk renaming files using customizable patterns. Renamer extracts metadata from filenames with regular expressions and then creates new names based on user-defined templates, while preserving the original file extensions.

## Features

- **Custom Regex Pattern**: Extract metadata (e.g. season, episode, title) from filenames.
- **Flexible Output Pattern**: Use placeholders like `{season}`, `{episode}`, and `{title}` in the new filename.
- **Extension Preservation**: Keeps the original file extension.
- **Dry-Run Mode**: Preview planned changes without renaming files.
- **File Type Filtering**: Process only files with specified extensions.
- **Depth Control**: Limit recursion depth for processing.

## Installation

Ensure you have Rust installed.

To install Renamer, run:
```sh
cargo install renamer
```

Or clone the repository and build:
```sh
git clone https://github.com/yourusername/renamer.git
cd renamer
cargo build --release
```

## Usage

```sh
renamer --current_pattern "<regex>" --new_pattern "<template>" [options]
```

### Example

The following example renames files matching a pattern by inserting captured groups into a new filename:
```sh
renamer \
  --current_pattern "^(?P<title>.+)_S(?P<season>\d+)E(?P<episode>\d+)" \
  --new_pattern "{title} - S{season:02}E{episode:02}" \
  --file_types "mkv,mp4" \
  --dry-run
```
This example shows a generic pattern where all values come from regex capture groups. The title, season, and episode are extracted directly from the filename, and used in the output pattern.

## Options Overview

- `--current_pattern`: A regex with named capture groups (e.g., `^(?P<title>.+)_S(?P<season>\d+)E(?P<episode>\d+)$`).
- `--new_pattern`: New filename template using placeholders (e.g., `{title} - S{season:02}E{episode:02}`).
- `--file_types`: Comma-separated list of file extensions (e.g., `mkv,mp4`).
- `--dry-run`: Run the tool in preview mode.
- `--depth`: Maximum recursion depth for searching files.
- `--config`: Path to a TOML configuration file.

## Configuration File

You can also supply parameters via a TOML file. For example:
```toml
directory = "/path/to/files"
current_pattern = "^(?P<title>.+)_S(?P<season>\\d+)E(?P<episode>\d+)$"
new_pattern = "{title} - S{season:02}E{episode:02}"
file_types = ["mkv", "mp4"]
dry_run = false
depth = 3
```
Run with:
```sh
renamer --config config.toml
```

## Contributing

Contributions are welcome! Please review the guidelines before opening issues or submitting pull requests.

## License

This project is licensed under the MIT License.
