//! CLI module for the renamer tool.
//! This module handles the parsing of command-line arguments using the `clap` crate.
//!
//! # Examples
//!
//! ```
//! # use renamer::cli::Cli;
//! # use clap::Parser;
//! let args = vec![
//!     "renamer",
//!     "-d", "/tmp",
//!     "-c", r"S(?P<season>\d+)E(?P<episode>\d+)",
//!     "-n", "{title} - S{season:02}E{episode:02}",
//!     "-t", "mkv,ass",
//!     "--dry-run",
//!     "--default-season", "1",
//!     "-T", "Show",
//!     "--depth", "2",
//! ];
//! let cli = Cli::parse_from(args);
//! assert_eq!(cli.directory, std::path::PathBuf::from("/tmp"));
//! ```

use clap::Parser;
use std::path::PathBuf;

/// CLI configuration for the Renamer tool.
///
/// This struct holds the command-line arguments. **Important:** Any options provided
/// on the command line override the values specified in a configuration file.
/// If an option is omitted from the CLI, but provided in the config file (via `--config`),
/// then the config file value will be used.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to a configuration file (optional). Supports TOML format.
    /// 
    /// **Note:** Values from the configuration file are merged, but CLI options take priority.
    #[arg(long, help = "Path to a TOML configuration file. CLI options override config file values.")]
    pub config: Option<PathBuf>,

    /// Directory to process (short: -d)
    #[arg(short, long, default_value = ".")]
    pub directory: PathBuf,

    /// Current file regex pattern with named groups 
    /// (e.g., "S(?P<season>\\d+)E(?P<episode>\\d+)" or if season is absent, a pattern that only captures episode)
    #[arg(short, long)]
    pub current_pattern: String,

    /// New file name pattern (default: "{title} - S{season:02}E{episode:02}")
    #[arg(short, long, default_value = "{title} - S{season:02}E{episode:02}")]
    pub new_pattern: String,

    /// Comma-separated list of file types/extensions to process (e.g., "mkv,ass,srt")
    #[arg(short = 't', long, value_delimiter = ',')]
    pub file_types: Vec<String>,

    /// Dry run mode: if set, the tool will only print intended changes without renaming files.
    #[arg(long)]
    pub dry_run: bool,

    /// Default season to use if not captured in the file name (default: "1")
    #[arg(long, default_value = "1")]
    pub default_season: String,

    /// Show title to include in the new file name (optional).
    #[arg(short = 'T', long)]
    pub title: Option<String>,

    /// Depth of recursion for renaming files (default: 1)
    #[arg(long, default_value_t = 1)]
    pub depth: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cli_parsing() {
        let args = vec![
            "renamer",
            "-d", "/path/to/dir",
            "-c", r"S(?P<season>\d+)E(?P<episode>\d+)",
            "-n", "{title} - S{season:02}E{episode:02}",
            "-t", "mkv,ass",
            "--dry-run",
            "--default-season", "1",
            "-T", "MyShow",
            "--depth", "3",
        ];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.directory, PathBuf::from("/path/to/dir"));
        assert_eq!(cli.current_pattern, r"S(?P<season>\d+)E(?P<episode>\d+)");
        assert_eq!(cli.new_pattern, "{title} - S{season:02}E{episode:02}");
        assert_eq!(cli.file_types, vec!["mkv".to_string(), "ass".to_string()]);
        assert!(cli.dry_run);
        assert_eq!(cli.default_season, "1".to_string());
        assert_eq!(cli.title, Some("MyShow".to_string()));
        assert_eq!(cli.depth, 3);
    }

    #[test]
    fn test_cli_default_directory() {
        let args = vec![
            "renamer",
            "-c", r"S(?P<season>\d+)E(?P<episode>\d+)",
            "-n", "{title} - S{season:02}E{episode:02}",
            "-t", "mkv,ass",
            "--dry-run",
            "--default-season", "1",
            "-T", "MyShow",
            "--depth", "3",
        ];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.directory, PathBuf::from("."));
    }
}
