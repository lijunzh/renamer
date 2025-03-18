//! Renamer module for the renamer tool.
//! This module contains the core logic for transforming file names based on regex patterns and user-defined templates.
//! The renamer is designed to work with any file type and naming pattern using regex capture groups.

use regex::Regex;
use std::path::{Path, PathBuf};
use crate::error::RenamerError; 

/// A planned renaming operation.
///
/// Stores the original and new file paths, and a flag indicating if a warning
/// should be triggered due to specific captured values being "0".
#[derive(Debug)]
pub struct PlannedRename {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
    /// True if any warning conditions are met (e.g., season or episode equals "0").
    pub warn: bool,
}

/// Transforms an original file name into a new one according to a template.
///
/// This function applies the provided regex to extract named capture groups from
/// the original file name and then replaces the placeholders in the new pattern with
/// their corresponding values. The original file's extension is preserved.
/// 
/// # Parameters
/// 
/// - `original`: The original file name.
/// - `new_pattern`: The template for the new file name with placeholders in the form `{name}` or `{name:width}`,
///    where `name` corresponds to a named capture group in the regex, and optional `width` formats numeric values with leading zeros.
/// - `re`: The regex used to capture metadata from the original name.
/// 
/// # Returns
/// 
/// Returns `Ok(new_file_name)` if the regex matches; otherwise, returns `Err(RenamerError::InvalidPattern)`.
/// 
/// # Examples
/// 
/// ```
/// # use regex::Regex;
/// # use renamer::transform_filename;
/// // Example 1: Photo files with date and location
/// let re = Regex::new(r"IMG_(?P<year>\d{4})(?P<month>\d{2})(?P<day>\d{2})_(?P<location>[A-Za-z]+)").unwrap();
/// let original = "IMG_20230215_Paris.jpg";
/// let new_pattern = "{location} {year}-{month}-{day}";
/// let transformed = transform_filename(original, new_pattern, &re).unwrap();
/// assert_eq!(transformed, "Paris 2023-02-15.jpg");
///
/// // Example 2: Music files with artist and album
/// let re = Regex::new(r"(?P<artist>[^-]+)-(?P<album>[^-]+)-(?P<track>\d+)").unwrap();
/// let original = "Beatles-AbbeyRoad-09.mp3";
/// let new_pattern = "{artist} - {album} - Track {track:02}";
/// let transformed = transform_filename(original, new_pattern, &re).unwrap();
/// assert_eq!(transformed, "Beatles - AbbeyRoad - Track 09.mp3");
/// ```
pub fn transform_filename(
    original: &str,
    new_pattern: &str,
    re: &Regex
) -> Result<String, RenamerError> {
    let path = Path::new(original);
    let original_ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Capture groups from the original file name using the regex.
    let caps = re.captures(original).ok_or(RenamerError::InvalidPattern)?;

    // Replace placeholders of the form {name} or {name:width} in new_pattern.
    let placeholder_re = Regex::new(r"\{(\w+)(?::(\d+))?\}").unwrap();
    let result = placeholder_re.replace_all(new_pattern, |ph_caps: &regex::Captures| {
        let key = &ph_caps[1];
        if let Some(m) = caps.name(key) {
            let value = m.as_str();
            // If a width is provided, format the value accordingly.
            if let Some(width_match) = ph_caps.get(2) {
                let width: usize = width_match.as_str().parse().unwrap();
                // For numeric values, ensure proper zero-padding
                if let Ok(num_value) = value.parse::<usize>() {
                    format!("{:0width$}", num_value, width = width)
                } else {
                    // Non-numeric values don't need zero-padding
                    format!("{:width$}", value, width = width)
                }
            } else {
                value.to_string()
            }
        } else {
            // Replace with an empty string if the capture is missing.
            "".to_string()
        }
    });
    let mut new_file_name = result.to_string();

    // Enforce the original file's extension.
    let candidate = Path::new(&new_file_name);
    if let Some(candidate_ext) = candidate.extension().and_then(|s| s.to_str()) {
        if candidate_ext.to_lowercase() != original_ext {
            let stem = candidate.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            new_file_name = format!("{}.{}", stem, original_ext);
        }
    } else if !original_ext.is_empty() {
        new_file_name = format!("{}.{}", new_file_name, original_ext);
    }
    Ok(new_file_name)
}

/// Checks whether any named capture with specific values should trigger a warning.
///
/// Currently checks if the "season" or "episode" named groups (if present) have value "0".
/// This can be expanded to check for other warning conditions depending on the use case.
/// 
/// # Parameters
/// 
/// - `original`: The original file name.
/// - `re`: The regex with named capture groups.
/// 
/// # Returns
/// 
/// Returns `true` if any warning condition is met, otherwise `false`.
///
/// # Examples
///
/// ```rust
/// # use regex::Regex;
/// # use renamer::check_warning;
/// // Example with video files
/// let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
/// assert!(check_warning("S0E10_video.txt", &re)); // Warning for season "0"
/// assert!(!check_warning("S1E10_video.txt", &re)); // No warning for valid values
/// 
/// // Example with track numbers in music files
/// let re = Regex::new(r"(?P<artist>.+?)-(?P<album>.+?)-(?P<track>\d+)").unwrap();
/// assert!(!check_warning("Beatles-AbbeyRoad-01.mp3", &re)); // No warning
/// ```
pub fn check_warning(original: &str, re: &Regex) -> bool {
    if let Some(caps) = re.captures(original) {
        let season_warn = caps
            .name("season")
            .map(|m| m.as_str() == "0")
            .unwrap_or(false);
        let episode_warn = caps
            .name("episode")
            .map(|m| m.as_str() == "0")
            .unwrap_or(false);
        season_warn || episode_warn
    } else {
        false
    }
}

/// Determines if a file should be processed based on its extension.
/// If allowed_types is non-empty, the file must have an extension (case‑insensitively)
/// that matches one of the provided types.
pub fn should_process_file(path: &Path, allowed_types: &[String]) -> bool {
    if !allowed_types.is_empty() {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            allowed_types.iter().any(|ft| ft.eq_ignore_ascii_case(ext))
        } else {
            false
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::path::Path;

    #[test]
    fn test_transform_with_title_provided() {
        // When new_pattern includes title directly
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = "MyShow - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
        assert_eq!(transformed, "MyShow - S01E01.mkv");
    }

    #[test]
    fn test_transform_with_title_omitted() {
        // When title placeholder is empty
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = " - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
        assert_eq!(transformed, " - S01E01.mkv");
    }

    #[test]
    fn test_transform_without_title_placeholder() {
        // Just season and episode
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S1E1_video.mkv";
        let new_pattern = "S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
        assert_eq!(transformed, "S01E01.mkv");
    }

    #[test]
    fn test_transform_default_format_double_digit() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S12E34_video.mkv";
        let new_pattern = "TestShow - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
        assert_eq!(transformed, "TestShow - S12E34.mkv");
    }

    #[test]
    fn test_transform_high_episode() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "S01E100_video.mkv";
        let new_pattern = "TestShow - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
        assert_eq!(transformed, "TestShow - S01E100.mkv");
    }

    #[test]
    fn test_transform_no_regex_match() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let original = "random_file.txt";
        let new_pattern = "TestShow - S{season:02}E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re);
        assert!(transformed.is_err());
        // More specifically:
        assert!(matches!(transformed, Err(RenamerError::InvalidPattern)));
    }

    #[test]
    fn test_transform_with_default_season() {
        // Using regex to capture episode and adding a fixed season in the pattern
        let re = Regex::new(r"\[(?P<title>[^]]+)\]\[(?P<episode>\d+)\]").unwrap();
        let original = "[Ao no Exorcist][01][1080P][BDRip][HEVC-10bit][FLAC].mkv";
        let new_pattern = "{title} - S01E{episode:02}";
        let transformed = transform_filename(original, new_pattern, &re).unwrap();
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
    fn test_check_warning_true() {
        // season '0' should trigger warning.
        let pattern = r"S(?P<season>\d+)E(?P<episode>\d+)";
        let re = Regex::new(pattern).unwrap();
        let file_name = "MyShow S0E10.mkv"; // season is 0 -> warn true
        assert!(check_warning(file_name, &re));
    }
    
    #[test]
    fn test_check_warning_false() {
        // valid season/episode should not trigger warning.
        let pattern = r"S(?P<season>\d+)E(?P<episode>\d+)";
        let re = Regex::new(pattern).unwrap();
        let file_name = "MyShow S01E10.mkv";
        assert!(!check_warning(file_name, &re));
    }
}
