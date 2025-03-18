use rayon::prelude::*;
use regex::Regex;
use tempfile::tempdir;
use walkdir::WalkDir;
use std::fs::File;
use renamer::{should_process_file, transform_filename};

#[test]
fn test_parallel_processing_collect_files() {
    // Create a temporary directory with multiple dummy files.
    let temp_dir = tempdir().unwrap();
    let dir_path = temp_dir.path();

    // Create 100 dummy files with .mkv extension.
    for i in 0..100 {
        let file_path = dir_path.join(format!("file_{}.mkv", i));
        File::create(&file_path).unwrap();
    }

    let file_types = vec!["mkv".to_string()];
    
    // Process files in parallel using WalkDir.
    let entries: Vec<_> = WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .filter(|entry| {
            let path = entry.path();
            path.is_file() && should_process_file(path, &file_types)
        })
        .map(|entry| entry.path().to_owned())
        .collect();

    // Assert that all dummy files were found.
    assert_eq!(entries.len(), 100);
}

#[test]
fn test_parallel_processing_transform() {
    // Test parallel transformation on a list of file names using solely regex capture groups.
    let file_names = vec![
        "S1E1_test.mkv",
        "S2E12_test.mkv",
        "S3E3_test.mkv",
    ];
    let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
    let new_pattern = "TestShow - S{season:02}E{episode:02}";

    // Convert results from Result to Option using .ok() in filter_map.
    let results: Vec<_> = file_names.par_iter()
        .filter_map(|&name| transform_filename(name, new_pattern, &re).ok())
        .collect();

    let expected = vec![
        "TestShow - S01E01.mkv".to_string(),
        "TestShow - S02E12.mkv".to_string(),
        "TestShow - S03E03.mkv".to_string(),
    ];
    assert_eq!(results, expected);
}
