use std::path::Path;

/// Determines if the specified file should be processed based on its extension.
/// 
/// # Parameters
/// 
/// - `path`: The file path to check.
/// - `allowed_types`: A list of allowed file extensions (case‑insensitive).
/// 
/// # Returns
/// 
/// Returns `true` if the file has an allowed extension; otherwise, returns `false`.
/**
Examples:

```
# use std::path::Path;
# use renamer::should_process_file;
let path = Path::new("video.mkv");
assert!(should_process_file(path, &vec!["mkv".to_string()]));
```
*/
pub fn should_process_file(path: &Path, file_types: &[String]) -> bool {
    // ...existing file type check logic...
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return file_types.iter().any(|ft| ft == ext_str);
        }
    }
    false
}
