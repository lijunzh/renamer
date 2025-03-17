use std::fs;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use crate::cli::Cli;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub directory: Option<String>,
    pub current_pattern: Option<String>,
    pub new_pattern: Option<String>,
    pub file_types: Option<Vec<String>>,
    pub dry_run: Option<bool>,
    pub default_season: Option<String>,
    pub title: Option<String>,
    pub depth: Option<usize>,
}

/// Merges configuration from a TOML file into the provided CLI instance.
///
/// If `cli.config` is set, the configuration file is read and its values
/// are used to fill in any missing CLI options. **Important:** Options provided
/// on the command line will always override values from the config file.
///
/// # Errors
///
/// Returns an error if the configuration file cannot be read or parsed.
pub fn merge_config(cli: &mut Cli) -> Result<(), anyhow::Error> {
    if let Some(config_path) = cli.config.as_ref() {
        let config_str = fs::read_to_string(config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        let config: AppConfig = toml::from_str(&config_str)
            .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;
        if cli.directory.as_os_str().is_empty() {
            if let Some(dir) = config.directory {
                cli.directory = dir.into();
            }
        }
        if cli.current_pattern.is_empty() {
            if let Some(val) = config.current_pattern {
                cli.current_pattern = val;
            }
        }
        if cli.new_pattern.is_empty() {
            if let Some(val) = config.new_pattern {
                cli.new_pattern = val;
            }
        }
        if cli.file_types.is_empty() {
            if let Some(val) = config.file_types {
                cli.file_types = val;
            }
        }
        if cli.dry_run {
            if let Some(val) = config.dry_run {
                cli.dry_run = val;
            }
        }
        if cli.default_season.is_empty() {
            if let Some(val) = config.default_season {
                cli.default_season = val;
            }
        }
        if cli.title.is_none() {
            cli.title = config.title;
        }
        if cli.depth == 1 {
            if let Some(val) = config.depth {
                cli.depth = val;
            }
        }
    }
    Ok(())
}
