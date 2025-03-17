use regex::Regex;
use crate::error::RenamerError;

// Transformation logic for renaming filenames.
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

pub fn check_warning(filename: &str) -> bool {
    // ...existing warning check logic...
    filename.contains("warning")
}
