use renamer::cli::Cli;
use renamer::renamer::{transform_filename, check_warning, should_process_file};
use regex::Regex;
use tempfile::tempdir;

#[test]
fn test_transform_with_title_provided() {
    let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
    let original = "S1E1_video.mkv";
    let new_pattern = "{title} - S{season:02}E{episode:02}";
    let transformed = transform_filename(original, new_pattern, &re, "1", "MyShow").unwrap();
    assert_eq!(transformed, "MyShow - S01E01.mkv");
}

#[test]
fn test_transform_with_title_omitted() {
    let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
    let original = "S1E1_video.mkv";
    let new_pattern = "{title} - S{season:02}E{episode:02}";
    let transformed = transform_filename(original, new_pattern, &re, "1", "").unwrap();
    assert_eq!(transformed, " - S01E01.mkv");
}

#[test]
fn test_transform_without_title_placeholder() {
    let re = Regex::new(r"S(?P<season>\d+)E(?P<episode>\d+)").unwrap();
    let original = "S1E1_video.mkv";
    let new_pattern = "S{season:02}E{episode:02}";
    let transformed = transform_filename(original, new_pattern, &re, "1", "MyShow").unwrap();
    assert_eq!(transformed, "S01E01.mkv");
}

// ... include additional tests from the previous module, for example:

#[test]
fn test_depth_option() {
    // Create a temporary directory structure:
    // test_dir/
    //   file1.txt       -> level 1
    //   sub1/file2.txt  -> level 2
    //   sub1/sub2/file3.txt  -> level 3 (should not be processed with depth=2)
    let base = tempdir().unwrap();
    let base_path = base.path();
    let file1 = base_path.join("file1.txt");
    std::fs::write(&file1, "dummy content").unwrap();

    let sub1 = base_path.join("sub1");
    std::fs::create_dir(&sub1).unwrap();
    let file2 = sub1.join("file2.txt");
    std::fs::write(&file2, "dummy content").unwrap();

    let sub2 = sub1.join("sub2");
    std::fs::create_dir(&sub2).unwrap();
    let file3 = sub2.join("file3.txt");
    std::fs::write(&file3, "dummy content").unwrap();

    // Create a dummy CLI configuration with depth=2.
    let cli = Cli {
        directory: base_path.to_path_buf(),
        current_pattern: r"(.+)".to_string(),
        new_pattern: "$1".to_string(),
        file_types: vec!["txt".to_string()],
        dry_run: true,
        default_season: "1".to_string(),
        title: None,
        depth: 2,
    };

    // Count the number of files processed using WalkDir with max_depth as specified.
    let mut count = 0;
    let walker = walkdir::WalkDir::new(&cli.directory).max_depth(cli.depth).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && should_process_file(path, &cli.file_types) {
            count += 1;
        }
    }
    // With depth=2, only file1.txt and file2.txt should be processed.
    assert_eq!(count, 2);
}

// ...other tests...
