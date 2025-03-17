//! CLI module for the renamer tool.
//! This module handles the parsing of command-line arguments using the `clap` crate.

use clap::Parser;
use std::path::PathBuf;

/// CLI arguments for the renamer tool.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Directory to process (short: -d)
    #[arg(short, long)]
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
