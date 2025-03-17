use renamer::{Cli, transform_filename, should_process_file, merge_config};
use regex::Regex;
use tempfile::{tempdir, NamedTempFile};
use std::io::Write;
use std::path::PathBuf;

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
        config: None,
        directory: base_path.to_path_buf(),
        current_pattern: "(.+)".to_string(),
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

#[test]
fn test_config_file_merging() {
    // Prepare a temporary config file with custom parameters.
    let mut config_file = NamedTempFile::new().unwrap();
    writeln!(config_file, r#"directory = "/configured/dir""#).unwrap();
    writeln!(config_file, r#"current_pattern = "C(?P<season>\\d+)D(?P<episode>\\d+)""#).unwrap();
    // Double curly braces produce literal { and }
    writeln!(config_file, r#"new_pattern = "Configured - C{{season:02}}D{{episode:02}}""#).unwrap();
    writeln!(config_file, r#"file_types = ["mp4", "avi"]"#).unwrap();
    writeln!(config_file, r#"dry_run = false"#).unwrap();
    writeln!(config_file, r#"default_season = "2""#).unwrap();
    writeln!(config_file, r#"title = "ConfiguredShow""#).unwrap();
    writeln!(config_file, r#"depth = 3"#).unwrap();

    // Create a CLI instance with empty values and set the config field.
    let mut cli = Cli {
        config: Some(PathBuf::from(config_file.path())),
        directory: "".into(),
        current_pattern: "".into(),
        new_pattern: "".into(),
        file_types: vec![],
        dry_run: true, // This should be overridden.
        default_season: "".into(),
        title: None,
        depth: 1,
    };

    // Merge configuration from the temporary file.
    merge_config(&mut cli).expect("Failed to merge config");

    // Assert that CLI fields have been updated according to the config file.
    assert_eq!(cli.directory, PathBuf::from("/configured/dir"));
    assert_eq!(cli.current_pattern, "C(?P<season>\\d+)D(?P<episode>\\d+)");
    assert_eq!(cli.new_pattern, "Configured - C{season:02}D{episode:02}");
    assert_eq!(cli.file_types, vec!["mp4".to_string(), "avi".to_string()]);
    assert_eq!(cli.dry_run, false);
    assert_eq!(cli.default_season, "2".to_string());
    assert_eq!(cli.title, Some("ConfiguredShow".to_string()));
    assert_eq!(cli.depth, 3);
}
