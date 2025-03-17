# Renamer CLI Tool

A Rust-based CLI tool for renaming files in bulk based on custom patterns. It
extracts metadata such as season, episode, and optionally title from file names
using a user-provided regular expression, then generates new file names using a
customizable pattern. The tool preserves the original file extension and
supports a dry-run mode so you can preview changes before they are applied.

**Note**: the code based is mostly generated using GPT o3-mini-high model
including this README file.

## Features

- **Custom Regex Pattern**: Extract metadata (season, episode, title, etc.) from
  file names.
- **Flexible Output Pattern**: Use placeholders (e.g. `{season}`, `{episode}`,
  `{title}`) in the new file name pattern.
- **Default Values**: Provide a default season if not present in the file name.
- **Optional Title Handling**: Include a title in the new file name if the
  pattern contains `{title}`; otherwise, it is ignored.
- **Extension Preservation**: Keeps the original file extension even if the new
  pattern suggests a different one.
- **File Type Filtering**: Process only files with specified extensions (e.g.
  `mkv`, `ass`, etc.).
- **Dry-Run Mode**: Preview the planned renames without modifying any files.
- **User Confirmation**: Warns when season or episode is `0` and asks for
  confirmation before proceeding.
- **Depth Control**: Optionally specify the depth of recursion for renaming files.

## Installation

To install Renamer, you need to have Rust and Cargo installed. Then run:

```sh
cargo install renamer
```

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/renamer-cli.git
cd renamer-cli
cargo build --release
```

## Getting Started

Getting started with Renamer is easy. Here's a quick guide to help you rename your files right away:

### Key Concepts

- **Named Capture Groups**: Use `(?P<name>pattern)` in your regex to capture parts of the filename
- **Placeholders**: Use `{name}` in your new pattern to insert captured values
- **Formatting**: Use `{name:02}` to format numbers with leading zeros
- **Default Values**: The `--default-season` option provides a fallback when season info is missing

### Need Help with Regex?

If you're not familiar with regex syntax, here are some example patterns to help you get started:

- **TV Shows**: `[Ss](?P<season>\d+)[Ee](?P<episode>\d+)` - Matches "S01E01", "s1e01", etc.
- **Movies**: `(?P<title>.+?)\.(?P<year>\d{4})` - Extracts title and year from "Movie.2023.mp4"
- **Anime**: `\[(?P<title>[^]]+)\]\[(?P<episode>\d+)\]` - Matches "[Title][01]"

You can also use ChatGPT or other LLM tools with prompts like:
- "Create a regex pattern to match file names in the format 'S1E1_video.mkv'."
- "Generate a regex to extract the title and episode number from file names like '[Author][title][01][1080P][BDRip][HEVC-10bit][FLAC].mkv'."

### Basic Usage

```sh
renamer --current_pattern <pattern> --new_pattern <replacement> [options]
```

### Options

- `-h`, `--help`: Print help information
- `-d`, `--directory`: Directory to process (default: current directory)
- `-c`, `--current_pattern`: (Required) Regex pattern with named capture groups
- `-n`, `--new_pattern`: New filename pattern (default: "{title} - S{season:02}E{episode:02}")
- `-t`, `--file_types`: Comma-separated list of file extensions (e.g., "mkv,mp4,srt")
- `--default-season`: Default season value (default: "1")
- `-T`, `--title`: Optional title to include in the new filename
- `--dry-run`: Preview changes without renaming files
- `--depth`: Recursion depth for subdirectories (default: 1)
- `--config`: Path to a TOML configuration file

## Examples

### Simple Text File Renaming

```sh
# Rename all .txt files to .md
renamer --current_pattern '^(.*)\.txt$' --new_pattern '$1.md' --dry-run
```

### TV Shows Organization

For files with inconsistent naming like `BreakingBad.S01E01.720p.mkv` or `BB_S1_E3_720p.mkv`:

```bash
renamer \
  --current_pattern "[Ss]0*(?P<season>\d+)[Ee]0*(?P<episode>\d+)" \
  --new_pattern "{title} - S{season:02}E{episode:02}" \
  --file_types mkv,mp4 \
  --title "Breaking Bad"
```

### Movies with Year Information

For movie files like `Avatar.2009.1080p.BluRay.mkv`:

```bash
renamer \
  --current_pattern "(?P<title>.+?)\.(?P<year>\d{4})" \
  --new_pattern "{title} ({year})" \
  --file_types mkv,mp4
```

### Anime with Missing Season Information

For files like `[Author][title][01][1080P][BDRip][HEVC-10bit][FLAC].mkv`:

```bash
renamer \
  --current_pattern "\[(?P<title>[^]]+)\]\[(?P<episode>\d+)\]" \
  --new_pattern "{title} - S{season:02}E{episode:02}" \
  --file_types mkv,ass \
  --default-season 1
```

### Recursive Directory Processing

To rename files in subdirectories:

```bash
renamer \
  --current_pattern "(?P<episode>\d+)" \
  --new_pattern "{title} - S{season:02}E{episode:02}" \
  --file_types mkv,ass \
  --title "My Show" \
  --depth 2
```

## Configuration File

Renamer supports providing a configuration file in TOML format. CLI options take precedence over configuration file values.

Example `config.toml`:

```toml
directory = "/path/to/files"
current_pattern = "S(?P<season>\\d+)E(?P<episode>\\d+)"
new_pattern = "{title} - S{season:02}E{episode:02}"
file_types = ["mkv", "ass"]
dry_run = true
default_season = "1"
title = "My Show"
depth = 2
```

Usage:

```sh
renamer --config config.toml
```

## Running Tests

To run the unit tests for this project, use:

```bash
cargo test
```

The tests cover:

- Transformation of file names with single- and double-digit season/episode
  values.
- Behavior when the title placeholder is present or absent.
- Default season usage when season data is missing.
- Ignoring files that do not match the regex or have disallowed extensions.
- Dry-run functionality and warning conditions.
- Depth control for recursive renaming.

## Contributing

Contributions are welcome! Please review our guidelines in [CONTRIBUTING.md](/CONTRIBUTING.md) before submitting issues or pull requests.

### Development

To set up the development environment, clone the repository and run:

```sh
git clone https://github.com/yourusername/renamer.git
cd renamer
cargo build
```

## License

This project is licensed under the MIT License.

## Need Help with Regex?

If you are not familiar with regex syntax, you can use ChatGPT or other LLM tools to help create regex expressions. Here are a few example prompts that might be useful:

- "Create a regex pattern to match file names in the format 'S1E1_video.mkv'."
- "Generate a regex to extract the title and episode number from file names like '[Author][title][01][1080P][BDRip][HEVC-10bit][FLAC].mkv'."
- "How can I write a regex to match file names with a season and episode number, but without a title?"

These tools can provide you with the regex patterns you need to use with Renamer.
