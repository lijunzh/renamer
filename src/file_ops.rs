// File operations logic
pub fn should_process_file(path: &std::path::Path, file_types: &[String]) -> bool {
    // ...existing file type check logic...
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return file_types.iter().any(|ft| ft == ext_str);
        }
    }
    false
}
