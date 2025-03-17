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

## Usage

To use Renamer, run the following command:

```sh
renamer <pattern> <replacement> [options]
```

Run the CLI tool using cargo run with the appropriate options.

### Options

- `-h`, `--help`: Print help information.
- -d, --directory: (Required) Directory to process.
- -c, --current_pattern: (Required) Regex pattern with named groups to match
  parts of the current file name. Example:
- For file names like S1E1_video.mkv, use:

```
S(?P<season>\d+)E(?P<episode>\d+)
```

- For file names like [Author][title][01][1080P][BDRip][HEVC-10bit][FLAC].mkv
  (without season), use:

```
$begin:math:display$[^]]+$end:math:display$$begin:math:display$(?P<title>[^]]+)$end:math:display$$begin:math:display$(?P<episode>\\d+)$end:math:display$
```

- -n, --new_pattern: New file name pattern. **Default**: "{title} -
  S{season:02}E{episode:02}" (Placeholders: {season}, {episode}, and optionally
  {title}.)

- -t, --file_types: Comma-separated list of file extensions to process (e.g.
  mkv,ass,srt).
- --default-season: Default season value if the file name does not include one.
  Default: "1".
- -T, --title: (Optional) Show title to include in the new file name if the new
  pattern contains {title}.
- --dry-run: If set, prints the planned renames without actually renaming any
  files.
- --depth: Specify the depth of recursion for renaming files. Default is 1 (no recursion).

### Examples

Rename all `.txt` files to `.md`:

```sh
renamer '^(.*)\.txt$' '$1.md'
```

Perform a dry run to see the changes:

```sh
renamer '^(.*)\.txt$' '$1.md' --dry-run
```

### Example 1: Renaming Files with Season & Episode

For file names like:

```
S1E1_video.mkv
S12E34_video.mkv
```

Run:

```bash
cargo run -- \
  -d /path/to/files \
  --current_pattern "S(?P<season>\d+)E(?P<episode>\d+)" \
  --file_types mkv,ass
```

This will use the default new pattern. If no title is provided, the {title}
placeholder is replaced with an empty string.

### Example 2: Renaming Files When Season Is Missing

For file names like:

```
[Author][title][01][1080P][BDRip][HEVC-10bit][FLAC].mkv
```

Since the season is missing, supply a regex that captures the episode and
(optionally) the title:

```bash
cargo run -- \
  -d /path/to/files \
  --current_pattern "$begin:math:display$[^]]+$end:math:display$$begin:math:display$(?P<title>[^]]+)$end:math:display$$begin:math:display$(?P<episode>\\d+)$end:math:display$" \
  --file_types mkv,ass \
  --default-season 1
```

This extracts the title "title" and episode "01", then uses the default season
(1) to produce a new file name like:

```
title - S01E01.mkv
```

### Example 3: Overriding the Title

If you want to ignore any title captured by the regex and supply your own, run:

```bash
cargo run -- \
  -d /path/to/files \
  --current_pattern "$begin:math:display$(?P<episode>\\d+)$end:math:display$" \
  --file_types mkv,ass \
  --default-season 1 \
  --title "My Show"
```

Since the regex here only captures the episode, the {title} placeholder in the
new pattern is replaced by "My Show", producing:

```
My Show - S01E01.mkv
```

### Example 4: Depth Control for Recursive Renaming

To rename files in subdirectories as well, specify the depth of recursion:

```bash
cargo run -- \
  -d /path/to/files \
  --current_pattern "$begin:math:display$(?P<episode>\\d+)$end:math:display$" \
  --file_types mkv,ass \
  --default-season 1 \
  --title "My Show" \
  --depth 2
```

This will rename files in the specified directory and all its subdirectories up to 2 levels deep.

## Dry-Run Mode

To preview changes without renaming files, include the --dry-run flag:

```bash
cargo run -- \
  -d /path/to/files \
  --current_pattern "$begin:math:display$(?P<episode>\\d+)$end:math:display$" \
  --file_types mkv,ass \
  --default-season 1 \
  --title "My Show" \
  --dry-run
```

This will display the planned renames without modifying any files.

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

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

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
