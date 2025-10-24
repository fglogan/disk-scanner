use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

// Helper function to create test directory structure
fn setup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create node_modules structure
    fs::create_dir_all(base.join("project1/node_modules/package1")).unwrap();
    File::create(base.join("project1/node_modules/package1/index.js"))
        .unwrap()
        .write_all(b"module.exports = {};")
        .unwrap();

    // Create Rust target directory
    fs::create_dir_all(base.join("project2/target/debug")).unwrap();
    File::create(base.join("project2/target/debug/app"))
        .unwrap()
        .write_all(&vec![0u8; 5 * 1024 * 1024]) // 5MB binary
        .unwrap();

    // Create Python venv
    fs::create_dir_all(base.join("project3/venv/lib")).unwrap();
    File::create(base.join("project3/venv/lib/module.py"))
        .unwrap()
        .write_all(b"print('hello')")
        .unwrap();

    // Create large file (simulate video)
    let large_file = base.join("large_video.mp4");
    let mut file = File::create(&large_file).unwrap();
    // Write 2MB of data
    file.write_all(&vec![0u8; 2 * 1024 * 1024]).unwrap();

    // Create duplicate files
    fs::create_dir_all(base.join("docs")).unwrap();
    File::create(base.join("docs/file1.txt"))
        .unwrap()
        .write_all(b"duplicate content for testing")
        .unwrap();
    File::create(base.join("docs/file2.txt"))
        .unwrap()
        .write_all(b"duplicate content for testing")
        .unwrap();
    File::create(base.join("docs/unique.txt"))
        .unwrap()
        .write_all(b"unique content")
        .unwrap();

    temp_dir
}

#[test]
fn test_temp_dir_creation() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Verify structure was created
    assert!(base.join("project1/node_modules/package1/index.js").exists());
    assert!(base.join("project2/target/debug/app").exists());
    assert!(base.join("project3/venv/lib/module.py").exists());
    assert!(base.join("large_video.mp4").exists());
    assert!(base.join("docs/file1.txt").exists());
    assert!(base.join("docs/file2.txt").exists());
}

#[test]
fn test_file_sizes() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Check large file size
    let large_file_meta = fs::metadata(base.join("large_video.mp4")).unwrap();
    assert!(large_file_meta.len() >= 2 * 1024 * 1024); // At least 2MB

    // Check binary file size
    let binary_meta = fs::metadata(base.join("project2/target/debug/app")).unwrap();
    assert!(binary_meta.len() >= 5 * 1024 * 1024); // At least 5MB
}

#[test]
fn test_duplicate_file_content() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let file1_content = fs::read_to_string(base.join("docs/file1.txt")).unwrap();
    let file2_content = fs::read_to_string(base.join("docs/file2.txt")).unwrap();
    let unique_content = fs::read_to_string(base.join("docs/unique.txt")).unwrap();

    assert_eq!(file1_content, file2_content);
    assert_ne!(file1_content, unique_content);
}
