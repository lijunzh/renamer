use regex::Regex;
use crate::error::RenamerError;

// Transformation logic for renaming filenames.

/// Transforms a filename using a regex and replacement pattern.
///
/// # Examples
///
/// ```
/// # use regex::Regex;
/// # use renamer::transform_filename;
/// # use renamer::RenamerError;
/// let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
/// let result = transform_filename("S1E1_video.mkv", "{title} - S{season:02}E{episode:02}", &re, "1", "Show")
///     .expect("failed to transform");
/// assert_eq!(result, "Show - S01E01.mkv");
/// ```
pub fn transform_filename(original: &str, new_pattern: &str, _re: &Regex, season: &str, title: &str) -> Result<String, RenamerError> {
    // Extract capture groups from original filename.
    let caps = _re.captures(original).ok_or(RenamerError::InvalidPattern)?;
    let captured_episode = caps.name("episode").map(|m| m.as_str()).unwrap_or("0");
    // Format season and episode with zero-padding width 2.
    let formatted_season = format!("{:0>2}", season);
    let formatted_episode = format!("{:0>2}", captured_episode);
    // Replace placeholders in new_pattern.
    let mut result = new_pattern.replace("{title}", title)
        .replace("{season:02}", &formatted_season)
        .replace("{episode:02}", &formatted_episode);
    // Append the original file extension.
    if let Some(idx) = original.rfind('.') {
        let extension = &original[idx..];
        result.push_str(extension);
    }
    Ok(result)
}

/// Checks if a filename contains the "warning" substring.
///
/// # Examples
///
/// ```
/// # use renamer::check_warning;
/// assert!(check_warning("file_with_warning.txt"));
/// assert!(!check_warning("file_without.txt"));
/// ```
pub fn check_warning(filename: &str) -> bool {
    // ...existing warning check logic...
    filename.contains("warning")
}
