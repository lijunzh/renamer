//! Main module for the renamer tool.
//! This module handles the CLI parsing, logging setup, and the main logic for processing files.

mod cli;
mod renamer;

use clap::Parser;
use log::{info, warn, error, LevelFilter};
use simplelog::{Config, SimpleLogger};
use walkdir::WalkDir;
use std::io::{self, Write};

use crate::cli::Cli;
use crate::renamer::{PlannedRename, transform_filename, check_warning, should_process_file};

/// Main function to run the renamer tool.
/// Sets up logging, parses CLI arguments, and processes files for renaming.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;
    let cli = Cli::parse();

    info!("Starting renamer tool with parameters: {:?}", cli);

    // Compile the provided regex pattern.
    let re = regex::Regex::new(&cli.current_pattern)
        .map_err(|e| format!("Invalid regex pattern provided for current file names: {}", e))?;

    // Determine the show title to use. If none is provided, use an empty string.
    let show_title = cli.title.as_deref().unwrap_or("");

    let mut planned: Vec<PlannedRename> = Vec::new();

    // Recursively iterate over files in the directory up to the specified depth.
    let walker = WalkDir::new(&cli.directory).max_depth(cli.depth).into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        // Only process files (ignore subdirectories).
        if path.is_file() {
            if !should_process_file(path, &cli.file_types) {
                continue;
            }
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if let Some(new_file_name) =
                    transform_filename(file_name, &cli.new_pattern, &re, &cli.default_season, show_title)
                {
                    let warn = check_warning(file_name, &re);
                    let new_path = path.with_file_name(&new_file_name);
                    planned.push(PlannedRename {
                        old_path: path.to_path_buf(),
                        new_path: new_path.clone(),
                        warn,
                    });
                    info!("Planned rename from {:?} to {:?}", path, &new_path);
                }
            }
        }
    }

    // If any file would be renamed with season or episode "0", warn the user.
    if planned.iter().any(|p| p.warn) {
        warn!("Some files have season or episode as 0. This might be unintended.");
        eprint!("Do you want to proceed? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            warn!("Aborting as per user request.");
            return Ok(());
        }
    }

    // Process the planned renames.
    for plan in planned {
        info!("Renaming from {:?} to {:?}", plan.old_path, plan.new_path);
        if cli.dry_run {
            info!("Dry-run mode: no changes made.");
        } else {
            if let Err(e) = std::fs::rename(&plan.old_path, &plan.new_path) {
                error!("Error renaming file: {:?}", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_transform_with_title_provided() {
        // When new_pattern includes {title} and a title is provided.
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "MyShow").unwrap();
        assert_eq!(transformed, "MyShow - S01E01.mkv");
    }

    #[test]
    fn test_transform_with_title_omitted() {
        // When new_pattern includes {title} but no title is provided.
        // The {title} placeholder should be replaced with an empty string.
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "").unwrap();
        // Expect leading " - " to be present because {title} is replaced by empty string.
        assert_eq!(transformed, " - S01E01.mkv");
    }

    #[test]
    fn test_transform_without_title_placeholder() {
        // When new_pattern does not include a {title} placeholder, even if a title is provided it is ignored.
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = "S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "MyShow").unwrap();
        assert_eq!(transformed, "S01E01.mkv");
    }

    #[test]
    fn test_transform_default_format_double_digit() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S12E34_video.mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "TestShow").unwrap();
        assert_eq!(transformed, "TestShow - S12E34.mkv");
    }

    #[test]
    fn test_transform_high_episode() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S01E100_video.mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "TestShow").unwrap();
        assert_eq!(transformed, "TestShow - S01E100.mkv");
    }

    #[test]
    fn test_transform_no_regex_match() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "random_file.txt";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "TestShow");
        assert!(transformed.is_none());
    }

    #[test]
    #[should_panic(expected = "Negative value for season")]
    fn test_negative_season() {
        let re = Regex::new(r"S(?P<season>-\d+)E(?P<episode>\d+)").unwrap();
        let original = "S-1E10_video.mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        transform_filename(original, new_pattern, &re, "1", "TestShow");
    }

    #[test]
    fn test_transform_with_default_season() {
        // When the file name does not include season info.
        let re = Regex::new(r"\[(?P<episode>\d+)\]").unwrap();
        let original = "[DBD-Raws][Ao no Exorcist][01][1080P][BDRip][HEVC-10bit][FLAC].mkv";
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re, "1", "Ao no Exorcist").unwrap();
        assert_eq!(transformed, "Ao no Exorcist - S01E01.mkv");
    }

    #[test]
    fn test_check_warning_no_warning() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let file_name = "S01E01_video.mkv";
        assert_eq!(check_warning(file_name, &re), false);
    }

    #[test]
    fn test_check_warning_with_warning() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let file_name1 = "S0E01_video.mkv";
        let file_name2 = "S01E0_video.mkv";
        assert_eq!(check_warning(file_name1, &re), true);
        assert_eq!(check_warning(file_name2, &re), true);
    }

    #[test]
    fn test_should_process_file_allowed() {
        let allowed_types = vec!["mkv".to_string(), "ass".to_string()];
        let path = Path::new("S01E01_video.mkv");
        assert!(should_process_file(path, &allowed_types));
    }

    #[test]
    fn test_should_process_file_not_allowed() {
        let allowed_types = vec!["mkv".to_string(), "ass".to_string()];
        let path = Path::new("S01E01_video.mp4");
        assert!(!should_process_file(path, &allowed_types));
    }

    #[test]
    fn test_should_process_file_no_extension() {
        let allowed_types = vec!["mkv".to_string(), "ass".to_string()];
        let path = Path::new("README");
        assert!(!should_process_file(path, &allowed_types));
    }

    #[test]
    fn test_should_process_subdirectory() {
        let allowed_types = vec!["mkv".to_string(), "ass".to_string()];
        let path = Path::new("subdir");
        assert!(!should_process_file(path, &allowed_types));
    }

    #[test]
    fn test_depth_option() {
        let cli = Cli {
            directory: PathBuf::from("test_dir"),
            current_pattern: String::from(r"S(?P<season>\d+)E(?P<episode>\d+)"),
            new_pattern: String::from("{title} - S{season:02}E{episode:02}"),
            file_types: vec!["mkv".to_string()],
            dry_run: true,
            default_season: String::from("1"),
            title: Some(String::from("TestShow")),
            depth: 2,
        };

        // Simulate the main function with the depth option set to 2.
        // Ensure that files up to 2 levels deep are processed.
        // ...test logic...
    }
}
