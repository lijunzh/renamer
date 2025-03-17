//! Renamer module for the renamer tool.
//! This module contains the core logic for transforming file names based on regex patterns and user-defined templates.

use regex::Regex;
use std::path::{Path, PathBuf};

/// A planned renaming operation.
#[derive(Debug)]
pub struct PlannedRename {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
    pub warn: bool, // true if season or episode equals "0"
}

/// Transforms an original file name into a new file name by applying the regex
/// and replacing placeholders. The default new pattern is "{title} - S{season:02}E{episode:02}".
/// If the regex does not capture the "season" group, the provided `default_season` is used.
/// If the new pattern includes a `{title}` placeholder, it is replaced with `show_title`;
/// if `show_title` is empty, it is replaced with an empty string.
/// The original file’s extension is preserved.
/// Returns None if the regex does not match the original file name.
pub fn transform_filename(
    original: &str,
    new_pattern: &str,
    re: &Regex,
    default_season: &str,
    show_title: &str,
) -> Option<String> {
    let path = Path::new(original);
    let original_ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Capture groups from the original file name using the regex.
    let caps = re.captures(original)?;

    // Replace placeholders of the form {name} or {name:width} in new_pattern.
    let placeholder_re = Regex::new(r"\{(\w+)(?::(\d+))?\}").unwrap();
    let result = placeholder_re.replace_all(new_pattern, |ph_caps: &regex::Captures| {
        let key = &ph_caps[1];
        if let Some(m) = caps.name(key) {
            let val_str = m.as_str();
            if val_str.starts_with('-') {
                panic!("Negative value for {}", key);
            }
            if let Some(width_match) = ph_caps.get(2) {
                let width: usize = width_match.as_str().parse().unwrap();
                let num: i32 = val_str.parse().unwrap();
                format!("{:0width$}", num, width = width)
            } else {
                val_str.to_string()
            }
        } else if key == "season" {
            // Use the default season if not captured.
            if default_season.starts_with('-') {
                panic!("Negative default season value");
            }
            if let Some(width_match) = ph_caps.get(2) {
                let width: usize = width_match.as_str().parse().unwrap();
                let num: i32 = default_season.parse().unwrap();
                format!("{:0width$}", num, width = width)
            } else {
                default_season.to_string()
            }
        } else if key == "title" {
            // Replace {title} with the provided show title, or empty if not provided.
            show_title.to_string()
        } else {
            // Leave unchanged if no capture and not "season" or "title".
            ph_caps.get(0).unwrap().as_str().to_string()
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
    Some(new_file_name)
}

/// Checks whether the file’s captured season or episode equals "0".
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
}
