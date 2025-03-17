//! Main module for the renamer tool.
//! This module handles the CLI parsing, logging setup, and the main logic for processing files.

mod cli;
mod renamer;

use log::{info, warn, error, LevelFilter};
use simplelog::{Config, SimpleLogger};
use walkdir::WalkDir;
use std::io::{self, Write};
use clap::Parser;
use anyhow::{anyhow, Result};
use rayon::prelude::*;

use crate::cli::Cli;
use crate::renamer::{PlannedRename, transform_filename, check_warning, should_process_file};

/// Main function to run the renamer tool.
/// Sets up logging, parses CLI arguments, and processes files for renaming.
fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Info, Config::default())?;
    let cli = Cli::parse();

    info!("Starting renamer tool with parameters: {:?}", cli);

    // Compile the provided regex pattern.
    let re = regex::Regex::new(&cli.current_pattern)
        .map_err(|e| anyhow!("Invalid regex pattern provided for current file names: {}", e))?;

    // Determine the show title to use. If none is provided, use an empty string.
    let show_title = cli.title.as_deref().unwrap_or("");

    // Replace the sequential iteration with parallel processing.
    let planned: Vec<PlannedRename> = WalkDir::new(&cli.directory)
        .max_depth(cli.depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()  // Converts the iterator into a parallel iterator.
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() && should_process_file(path, &cli.file_types) {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if let Some(new_file_name) = transform_filename(file_name, &cli.new_pattern, &re, &cli.default_season, show_title) {
                        let warn = check_warning(file_name, &re);
                        let new_path = path.with_file_name(&new_file_name);
                        return Some(PlannedRename {
                            old_path: path.to_path_buf(),
                            new_path: new_path.clone(),
                            warn,
                        });
                    }
                }
            }
            None
        })
        .collect();

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
