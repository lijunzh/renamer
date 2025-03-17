use regex::Regex;
use crate::error::RenamerError;

// Transformation logic for renaming filenames.

/// Transforms a filename using a regex and a replacement pattern.
/// 
/// # Parameters
/// 
/// - `original`: The original filename.
/// - `new_pattern`: The pattern to generate the new filename. Placeholders like `{title}`, `{season:02}`, `{episode:02}` are supported.
/// - `_re`: A regular expression with named capture groups (e.g., "season" and "episode") used to extract filename parts.
/// - `season`: Default season value if not captured.
/// - `title`: The show title to use if a `{title}` placeholder is present.
/// 
/// # Returns
/// 
/// Returns a `Result` with the transformed filename (with the original file extension appended) or a `RenamerError::InvalidPattern` if the regex does not match.
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
    // Use the captured season if available, otherwise fallback to the default season.
    let captured_season = caps.name("season").map(|m| m.as_str()).unwrap_or(season);
    let captured_episode = caps.name("episode").map(|m| m.as_str()).unwrap_or("0");
    // Format season and episode with zero-padding width 2.
    let formatted_season = format!("{:0>2}", captured_season);
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

/// Checks if a filename contains a warning substring.
///
/// In the current implementation, a filename that contains "warning"
/// will trigger a warning.
///
/// # Parameters
///
/// - `filename`: The filename to check.
///
/// # Returns
///
/// Returns `true` if the filename contains "warning", otherwise `false`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use crate::error::RenamerError;

    #[test]
    fn test_transform_valid_with_extension() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let result = transform_filename("S3E12_movie.mkv", new_pattern, &re, "3", "Test").unwrap();
        assert_eq!(result, "Test - S03E12.mkv");
    }

    #[test]
    fn test_transform_valid_without_extension() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let result = transform_filename("S2E5", new_pattern, &re, "2", "Demo").unwrap();
        assert_eq!(result, "Demo - S02E05");
    }

    #[test]
    fn test_transform_invalid_regex() {
        let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
        let new_pattern = "{title} - S{season:02}E{episode:02}";
        let result = transform_filename("NotAMatch.txt", new_pattern, &re, "1", "Show");
        assert!(matches!(result, Err(RenamerError::InvalidPattern)));
    }

    #[test]
    fn test_check_warning_true() {
        assert!(check_warning("file_warning.txt"));
    }

    #[test]
    fn test_check_warning_false() {
        assert!(!check_warning("file_ok.txt"));
    }
}
