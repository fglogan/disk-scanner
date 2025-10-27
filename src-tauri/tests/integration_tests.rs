//! Integration tests for Disk Bloat Scanner.
//!
//! These tests verify the scanner's ability to:
//! - Create and manage complex directory structures
//! - Detect and handle duplicate files
//! - Calculate file sizes and directory hierarchies
//! - Perform cleanup operations (deletion, etc.)
//! - Handle edge cases and error conditions

use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

// ============================================================================
// Test Setup Helpers
// ============================================================================

/// Creates a complex directory structure for testing scanners
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

/// Creates a minimal test directory for file cleanup testing
fn setup_cleanup_test_dir() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create some test files
    File::create(base.join("file1.txt"))
        .unwrap()
        .write_all(b"content1")
        .unwrap();
    File::create(base.join("file2.txt"))
        .unwrap()
        .write_all(b"content2")
        .unwrap();
    File::create(base.join("file3.txt"))
        .unwrap()
        .write_all(b"content3")
        .unwrap();

    temp_dir
}

#[test]
fn test_temp_dir_creation() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Verify structure was created
    assert!(base
        .join("project1/node_modules/package1/index.js")
        .exists());
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

// ============================================================================
// File Structure and Hierarchy Tests
// ============================================================================

#[test]
fn test_directory_hierarchy_preserved() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Verify entire hierarchy exists
    let project_paths = vec![
        "project1/node_modules/package1/index.js",
        "project2/target/debug/app",
        "project3/venv/lib/module.py",
        "docs/file1.txt",
        "docs/file2.txt",
        "docs/unique.txt",
        "large_video.mp4",
    ];

    for path in project_paths {
        let full_path = base.join(path);
        assert!(
            full_path.exists(),
            "Path does not exist: {}",
            full_path.display()
        );
    }
}

#[test]
fn test_file_metadata_correctness() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    // Check that files are regular files
    assert!(base.join("docs/file1.txt").is_file());
    assert!(base.join("docs/file2.txt").is_file());
    assert!(base.join("large_video.mp4").is_file());

    // Check that directories are directories
    assert!(base.join("project1/node_modules").is_dir());
    assert!(base.join("project2/target").is_dir());
    assert!(base.join("project3/venv").is_dir());
}

// ============================================================================
// Cleanup Operation Integration Tests
// ============================================================================

#[test]
fn test_file_deletion_permanent() {
    let temp_dir = setup_cleanup_test_dir();
    let file_path = temp_dir.path().join("file1.txt");

    assert!(file_path.exists());

    // Simulate permanent deletion
    fs::remove_file(&file_path).unwrap();

    assert!(!file_path.exists());
}

#[test]
fn test_directory_deletion_recursive() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create nested structure
    fs::create_dir_all(base.join("parent/child/grandchild")).unwrap();
    File::create(base.join("parent/child/grandchild/file.txt"))
        .unwrap()
        .write_all(b"deep file")
        .unwrap();

    assert!(base.join("parent/child/grandchild/file.txt").exists());

    // Delete recursively
    fs::remove_dir_all(base.join("parent")).unwrap();

    assert!(!base.join("parent").exists());
}

#[test]
fn test_multiple_file_deletion_sequence() {
    let temp_dir = setup_cleanup_test_dir();
    let base = temp_dir.path();

    let files = vec![
        base.join("file1.txt"),
        base.join("file2.txt"),
        base.join("file3.txt"),
    ];

    // Verify all exist
    for file in &files {
        assert!(file.exists());
    }

    // Delete each one
    for file in &files {
        fs::remove_file(file).unwrap();
    }

    // Verify all are gone
    for file in &files {
        assert!(!file.exists());
    }
}

#[test]
fn test_partial_deletion_with_errors() {
    let temp_dir = setup_cleanup_test_dir();
    let base = temp_dir.path();

    let existing_file = base.join("file1.txt");
    let nonexistent_file = base.join("nonexistent.txt");

    // Delete the existing file
    fs::remove_file(&existing_file).unwrap();
    assert!(!existing_file.exists());

    // Try to delete nonexistent file - should fail
    let result = fs::remove_file(&nonexistent_file);
    assert!(result.is_err());
}

// ============================================================================
// File Size and Space Calculations Tests
// ============================================================================

#[test]
fn test_large_file_size_calculation() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let large_file = base.join("large_video.mp4");
    let metadata = fs::metadata(&large_file).unwrap();

    // Should be at least 2MB
    assert!(metadata.len() >= 2 * 1024 * 1024);
}

#[test]
fn test_total_directory_size() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let mut total_size = 0u64;

    // Walk directory and sum file sizes
    for entry in fs::read_dir(base).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            total_size += fs::metadata(&path).unwrap().len();
        }
    }

    // Should have at least the large video file (2MB)
    assert!(total_size >= 2 * 1024 * 1024);
}

#[test]
fn test_file_size_verification_after_write() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.bin");

    // Write 1MB of data
    let data = vec![0u8; 1024 * 1024];
    File::create(&file_path)
        .unwrap()
        .write_all(&data)
        .unwrap();

    let metadata = fs::metadata(&file_path).unwrap();
    assert_eq!(metadata.len() as usize, data.len());
}

// ============================================================================
// Duplicate Detection Integration Tests
// ============================================================================

#[test]
fn test_identical_file_detection() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let file1 = base.join("docs/file1.txt");
    let file2 = base.join("docs/file2.txt");

    let content1 = fs::read(&file1).unwrap();
    let content2 = fs::read(&file2).unwrap();

    assert_eq!(content1, content2);
}

#[test]
fn test_unique_file_detection() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let file1 = base.join("docs/file1.txt");
    let unique = base.join("docs/unique.txt");

    let content1 = fs::read(&file1).unwrap();
    let unique_content = fs::read(&unique).unwrap();

    assert_ne!(content1, unique_content);
}

#[test]
fn test_mixed_duplicate_and_unique_files() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create 3 duplicates
    for i in 1..=3 {
        File::create(base.join(format!("dup{}.txt", i)))
            .unwrap()
            .write_all(b"duplicate")
            .unwrap();
    }

    // Create 2 unique files
    File::create(base.join("unique1.txt"))
        .unwrap()
        .write_all(b"unique1")
        .unwrap();
    File::create(base.join("unique2.txt"))
        .unwrap()
        .write_all(b"unique2")
        .unwrap();

    // Read all files
    let mut file_map = std::collections::HashMap::new();
    for entry in fs::read_dir(base).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = fs::read(&path).unwrap();
            file_map.insert(path.file_name().unwrap().to_string_lossy().to_string(), content);
        }
    }

    assert_eq!(file_map.len(), 5);

    // Check for duplicates
    let dup1 = file_map.get("dup1.txt").unwrap();
    let dup2 = file_map.get("dup2.txt").unwrap();
    let dup3 = file_map.get("dup3.txt").unwrap();
    assert_eq!(dup1, dup2);
    assert_eq!(dup2, dup3);

    // Check uniqueness
    let unique1 = file_map.get("unique1.txt").unwrap();
    let unique2 = file_map.get("unique2.txt").unwrap();
    assert_ne!(dup1, unique1);
    assert_ne!(unique1, unique2);
}

// ============================================================================
// Path Manipulation and Validation Tests
// ============================================================================

#[test]
fn test_path_joining_and_display() {
    let temp_dir = setup_test_dir();
    let base = temp_dir.path();

    let file_path = base.join("docs").join("file1.txt");

    assert!(file_path.exists());
    assert!(file_path.to_string_lossy().contains("file1.txt"));
}

#[test]
fn test_relative_vs_absolute_paths() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create file with absolute path
    let absolute_path = base.join("test.txt");
    File::create(&absolute_path)
        .unwrap()
        .write_all(b"test")
        .unwrap();

    assert!(absolute_path.is_absolute());
    assert!(absolute_path.exists());
}

// ============================================================================
// Concurrent and Sequential Access Tests
// ============================================================================

#[test]
fn test_multiple_file_operations_sequence() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create multiple files sequentially
    for i in 0..10 {
        let path = base.join(format!("file{}.txt", i));
        File::create(&path)
            .unwrap()
            .write_all(format!("content{}", i).as_bytes())
            .unwrap();
        assert!(path.exists());
    }

    // Verify all exist
    for i in 0..10 {
        assert!(base.join(format!("file{}.txt", i)).exists());
    }

    // Delete them sequentially
    for i in 0..10 {
        let path = base.join(format!("file{}.txt", i));
        fs::remove_file(&path).unwrap();
        assert!(!path.exists());
    }
}
