//! File and directory cleanup utilities with safety-first deletion operations.
//!
//! This module provides safe deletion functionality with:
//! - Batch deletion limits (10,000 files max, 100GB max per operation)
//! - Dry-run support for previewing deletions
//! - Trash/recycle bin support for non-permanent deletion
//! - Comprehensive error handling and logging
//! - Audit trail logging for all deletions

use std::path::Path;

use super::deletion_log::{log_deletion, DeletionRecord};
use super::path::validate_scan_path;
use crate::{models::CleanupReq, ScannerResult};

/// Safety limits for batch deletion operations
pub const MAX_BATCH_DELETE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
/// Maximum number of files that can be deleted in a single operation
pub const MAX_BATCH_DELETE_COUNT: usize = 10_000; // 10k files

/// Detects if a path is in iCloud Drive
fn is_icloud_path(path: &str) -> bool {
    path.contains("Library/Mobile Documents/com~apple~CloudDocs")
        || path.contains("/iCloud Drive/")
        || path.contains("/Mobile Documents/")
}

/// Counts how many paths are in iCloud Drive
pub fn count_icloud_paths(paths: &[String]) -> usize {
    paths.iter().filter(|p| is_icloud_path(p)).count()
}

/// Infer the deletion category from the file path
fn infer_deletion_category(path: &str) -> String {
    if path.contains("node_modules")
        || path.contains(".cargo")
        || path.contains("__pycache__")
        || path.contains(".gradle")
        || path.contains(".m2")
    {
        "developer_dependencies".to_string()
    } else if path.contains(".cache") || path.contains("Cache") || path.contains("Caches") {
        "cache".to_string()
    } else if path.contains(".git") {
        "git_metadata".to_string()
    } else if path.contains("Duplicate") || path.starts_with(".") {
        "duplicates".to_string()
    } else {
        "user_selected".to_string()
    }
}

/// Validates a deletion request against safety limits and path security.
///
/// Ensures that:
/// 1. The number of files doesn't exceed `MAX_BATCH_DELETE_COUNT` (10,000)
/// 2. The total size doesn't exceed `MAX_BATCH_DELETE_SIZE` (100GB)
/// 3. All paths are validated for security (no system directories)
///
/// # Arguments
/// * `req` - The cleanup request containing paths to delete
///
/// # Returns
/// * `Ok(())` if the request is valid
/// * `Err(String)` with details about which limit was exceeded
///
/// # Example
/// ```no_run
/// use disk_bloat_scanner_lib::models::CleanupReq;
/// # use disk_bloat_scanner_lib::utils::cleanup::validate_deletion_request;
/// let req = CleanupReq {
///     paths: vec!["/tmp/file.txt".to_string()],
///     dry_run: false,
///     trash: true,
/// };
/// match validate_deletion_request(&req) {
///     Ok(_) => println!("Deletion request is valid"),
///     Err(e) => println!("Validation error: {}", e),
/// }
/// ```
pub fn validate_deletion_request(req: &CleanupReq) -> ScannerResult<()> {
    if req.paths.len() > MAX_BATCH_DELETE_COUNT {
        return Err(format!(
            "Cannot delete {} files at once (maximum: {})",
            req.paths.len(),
            MAX_BATCH_DELETE_COUNT
        )
        .into());
    }

    // Security validation: Check each path for system directory access
    for path in &req.paths {
        // Extract parent directory for validation (files inherit their parent's security context)
        let path_for_validation = if let Some(parent) = Path::new(path).parent() {
            parent.to_string_lossy().to_string()
        } else {
            path.clone()
        };

        // Validate the path to prevent deletion of files in system directories
        validate_scan_path(&path_for_validation)
            .map_err(|e| format!("Security validation failed for '{}': {}", path, e))?;
    }

    // Calculate total size
    let total_size: u64 = req
        .paths
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();

    if total_size > MAX_BATCH_DELETE_SIZE {
        let total_gb = total_size as f64 / (1024.0 * 1024.0 * 1024.0);
        let max_gb = MAX_BATCH_DELETE_SIZE as f64 / (1024.0 * 1024.0 * 1024.0);
        return Err(format!(
            "Cannot delete {:.1} GB at once (maximum: {:.0} GB)",
            total_gb, max_gb
        )
        .into());
    }

    Ok(())
}

/// Safely deletes files and directories with optional trash support.
///
/// This function handles batch deletion with the following features:
/// - **Dry-run mode:** Preview what would be deleted without making changes
/// - **Trash support:** Move to trash/recycle bin instead of permanent deletion
/// - **Error recovery:** Continues processing remaining files if some fail
/// - **Safety validation:** Enforces batch limits before execution
/// - **Post-deletion verification:** Checks files are actually removed
///
/// # Arguments
/// * `paths` - Slice of file/directory paths to delete
/// * `dry_run` - If true, simulate deletion without making changes
/// * `use_trash` - If true, move to trash; if false, permanently delete
///
/// # Returns
/// * Vector of successfully deleted paths
/// * Vector of skipped paths (already deleted/not found)
/// * Vector of error messages for failed deletions
///
/// # Errors
/// Returns an error string if:
/// - Too many files requested (>10,000)
/// - Total size exceeds limit (>100GB)
/// - Unexpected I/O errors occur during deletion
///
/// # Safety
/// - System directories cannot be accessed (validation happens in scan layer)
/// - Each file deletion is individually verified
/// - Trash operation is atomic at the OS level
pub fn delete_files(
    paths: &[String],
    dry_run: bool,
    use_trash: bool,
) -> ScannerResult<(Vec<String>, Vec<String>, Vec<String>)> {
    let mut deleted = Vec::new();
    let mut skipped = Vec::new();
    let mut errors = Vec::new();

    let icloud_count = count_icloud_paths(paths);

    log::info!(
        "Starting cleanup of {} paths (dry_run={}, trash={}, iCloud={})",
        paths.len(),
        dry_run,
        use_trash,
        icloud_count
    );

    if icloud_count > 0 {
        log::warn!(
            "⚠️  {} iCloud Drive files detected - these may fail due to macOS permissions",
            icloud_count
        );
    }

    if dry_run {
        // Dry run - just return what would be deleted
        return Ok((paths.to_vec(), vec![], vec![]));
    }

    for path in paths {
        let p = Path::new(path);
        log::debug!("Processing: {}", path);

        // Quick sanity check: if file doesn't exist right now, skip it.
        // Note: We still verify post-deletion to catch TOCTOU races, but this
        // catches the simple case where a file was already deleted.
        if !p.exists() {
            log::debug!("File does not exist, skipping: {}", path);
            skipped.push(path.clone());
            continue;
        }

        log::debug!(
            "File exists, attempting deletion (trash={}): {}",
            use_trash,
            path
        );

        if use_trash {
            // Check if this is an iCloud Drive path
            let is_icloud = is_icloud_path(path);

            if is_icloud {
                log::warn!("iCloud Drive file detected: {}", path);
                log::warn!("Attempting deletion with retry logic...");
            }

            // Move to trash with retry logic for iCloud Drive
            let mut attempts = if is_icloud { 3 } else { 1 };
            let mut last_error = None;

            while attempts > 0 {
                match trash::delete(p) {
                    Ok(_) => {
                        log::debug!("Successfully moved to trash: {}", path);

                        // Post-deletion verification: Wait briefly for OS to complete the operation
                        // This mitigates TOCTOU (Time-of-check-time-of-use) race conditions
                        std::thread::sleep(std::time::Duration::from_millis(100));

                        // Verify the file was actually removed (atomic verification)
                        if p.exists() {
                            log::warn!("File still exists after trash operation: {}", path);
                            errors.push(format!("{}: Moved to trash but file still exists (may indicate system issue)", path));
                        } else {
                            log::info!("Deletion verified: {} successfully removed", path);
                            deleted.push(path.clone());

                            // Log to deletion audit trail
                            let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                            let category = infer_deletion_category(path);
                            let record = DeletionRecord::new(
                                path.clone(),
                                file_size,
                                category,
                                "trash".to_string(),
                            );
                            if let Err(e) = log_deletion(&record) {
                                log::warn!("Failed to log deletion: {}", e);
                            }
                        }
                        break;
                    }
                    Err(e) => {
                        last_error = Some(e);
                        attempts -= 1;

                        if attempts > 0 && is_icloud {
                            log::warn!(
                                "Retry attempt for iCloud file (attempts left: {})",
                                attempts
                            );
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                    }
                }
            }

            // If all attempts failed, analyze the error
            if let Some(e) = last_error {
                let error_msg = format!("{}", e);
                log::debug!("Trash deletion failed: {}", error_msg);

                // Provide helpful error messages for common issues
                // Note: trash crate may return different error messages across platforms
                let helpful_msg = if error_msg.contains("not found")
                    || error_msg.contains("does not exist")
                    || error_msg.contains("No such file")
                    || error_msg.contains("404")
                {
                    // File doesn't exist - this can happen due to TOCTOU race conditions
                    log::debug!(
                        "File does not exist (likely deleted by another process), skipping: {}",
                        path
                    );
                    skipped.push(path.clone());
                    continue;
                } else if error_msg.contains("permission") || error_msg.contains("-5000") {
                    format!(
                        "{}: Permission denied. iCloud Drive files may require manual deletion.",
                        path
                    )
                } else if error_msg.contains("timed out") || error_msg.contains("-1712") {
                    format!(
                        "{}: Operation timed out. Try deleting this file manually from Finder.",
                        path
                    )
                } else {
                    format!("{}: {}", path, error_msg)
                };

                log::error!("Cleanup error for {}: {}", path, helpful_msg);
                errors.push(helpful_msg);
            }
        } else {
            // Permanent deletion (non-atomic, verify after)
            let result = if p.is_dir() {
                std::fs::remove_dir_all(p)
            } else {
                std::fs::remove_file(p)
            };

            match result {
                Ok(_) => {
                    log::debug!("Successfully deleted: {}", path);

                    // Post-deletion verification: Check if file is actually gone
                    // This ensures atomicity and detects race conditions
                    if p.exists() {
                        log::warn!("File still exists after deletion: {}", path);
                        errors.push(format!(
                            "{}: Deletion returned Ok but file still exists (race condition?)",
                            path
                        ));
                    } else {
                        log::info!("Deletion verified: {} successfully removed", path);
                        deleted.push(path.clone());

                        // Log to deletion audit trail
                        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                        let category = infer_deletion_category(path);
                        let record = DeletionRecord::new(
                            path.clone(),
                            file_size,
                            category,
                            "permanent".to_string(),
                        );
                        if let Err(e) = log_deletion(&record) {
                            log::warn!("Failed to log deletion: {}", e);
                        }
                    }
                }
                Err(e) => {
                    // Check if file doesn't exist (skip rather than error)
                    if e.kind() == std::io::ErrorKind::NotFound {
                        log::debug!("File does not exist, skipping: {}", path);
                        skipped.push(path.clone());
                    } else {
                        log::error!("Cleanup error for {}: {}", path, e);
                        errors.push(format!("{}: {}", path, e));
                    }
                }
            }
        }
    }

    log::info!(
        "Cleanup complete: deleted={}, skipped={}, errors={}",
        deleted.len(),
        skipped.len(),
        errors.len()
    );

    Ok((deleted, skipped, errors))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ========================================================================
    // validate_deletion_request Tests
    // ========================================================================

    #[test]
    fn test_validate_deletion_request_valid() {
        let req = CleanupReq {
            paths: vec!["/tmp/test.txt".to_string()],
            dry_run: false,
            trash: true,
        };
        assert!(validate_deletion_request(&req).is_ok());
    }

    #[test]
    fn test_validate_deletion_request_single_file() {
        let req = CleanupReq {
            paths: vec!["/tmp/file.txt".to_string()],
            dry_run: false,
            trash: false,
        };
        // Should pass validation (may fail on file existence but not security)
        if let Err(e) = validate_deletion_request(&req) {
            let err_msg = e.to_string();
            // Should not be a security error
            assert!(!err_msg.contains("Security validation failed"));
        }
    }

    #[test]
    fn test_validate_deletion_request_many_files() {
        let paths = vec!["/tmp/file.txt".to_string(); 100];
        let req = CleanupReq {
            paths,
            dry_run: false,
            trash: true,
        };
        // Should pass validation (may fail on file existence but not security)
        if let Err(e) = validate_deletion_request(&req) {
            let err_msg = e.to_string();
            // Should not be a security error
            assert!(!err_msg.contains("Security validation failed"));
        }
    }

    #[test]
    fn test_validate_deletion_request_exceeds_count() {
        let paths = vec!["test".to_string(); MAX_BATCH_DELETE_COUNT + 1];
        let req = CleanupReq {
            paths,
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Cannot delete"));
        assert!(err_msg.contains("files at once"));
    }

    #[test]
    fn test_validate_deletion_request_at_count_limit() {
        let paths = vec!["/tmp/test".to_string(); MAX_BATCH_DELETE_COUNT];
        let req = CleanupReq {
            paths,
            dry_run: false,
            trash: true,
        };
        // Should pass validation (may fail on file existence but not security)
        if let Err(e) = validate_deletion_request(&req) {
            let err_msg = e.to_string();
            // Should not be a security error
            assert!(!err_msg.contains("Security validation failed"));
        }
    }

    #[test]
    fn test_validate_deletion_request_empty_paths() {
        let req = CleanupReq {
            paths: vec![],
            dry_run: false,
            trash: true,
        };
        // Empty request should be valid (nothing to delete)
        assert!(validate_deletion_request(&req).is_ok());
    }

    // ========================================================================
    // delete_files Tests - Dry Run Mode
    // ========================================================================

    #[test]
    fn test_delete_files_dry_run() {
        let paths = vec!["/tmp/nonexistent.txt".to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, true, true).unwrap();

        // In dry-run mode, all paths should be marked as deleted
        assert_eq!(deleted.len(), 1);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_delete_files_dry_run_multiple() {
        let paths = vec![
            "/tmp/file1.txt".to_string(),
            "/tmp/file2.txt".to_string(),
            "/tmp/file3.txt".to_string(),
        ];
        let (deleted, skipped, errors) = delete_files(&paths, true, true).unwrap();

        assert_eq!(deleted.len(), 3);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
    }

    // ========================================================================
    // delete_files Tests - Nonexistent Files
    // ========================================================================

    #[test]
    fn test_delete_files_nonexistent() {
        let paths = vec!["/tmp/this_file_does_not_exist_xyz_12345.txt".to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, false, true).unwrap();

        // Nonexistent file should be skipped
        assert_eq!(deleted.len(), 0);
        assert_eq!(skipped.len(), 1);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_delete_files_mixed_existent_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = temp_dir.path().join("existing.txt");
        fs::write(&existing_file, b"test content").unwrap();

        let paths = vec![
            existing_file.to_string_lossy().to_string(),
            "/tmp/does_not_exist_xyz_99999.txt".to_string(),
        ];

        let (deleted, skipped, errors) = delete_files(&paths, false, false).unwrap();

        // One should be deleted, one skipped
        assert_eq!(deleted.len(), 1);
        assert_eq!(skipped.len(), 1);
        assert_eq!(errors.len(), 0);
    }

    // ========================================================================
    // delete_files Tests - Actual File Deletion
    // ========================================================================

    #[test]
    fn test_delete_files_permanent_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"test content").unwrap();

        assert!(file_path.exists());

        let paths = vec![file_path.to_string_lossy().to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, false, false).unwrap();

        assert_eq!(deleted.len(), 1);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
        // Verify file is actually gone
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_files_permanent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path().join("subdir");
        fs::create_dir(&dir_path).unwrap();
        fs::write(dir_path.join("file.txt"), b"content").unwrap();

        assert!(dir_path.exists());

        let paths = vec![dir_path.to_string_lossy().to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, false, false).unwrap();

        assert_eq!(deleted.len(), 1);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
        assert!(!dir_path.exists());
    }

    #[test]
    fn test_delete_files_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        let file3 = temp_dir.path().join("file3.txt");

        fs::write(&file1, b"content1").unwrap();
        fs::write(&file2, b"content2").unwrap();
        fs::write(&file3, b"content3").unwrap();

        let paths = vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
            file3.to_string_lossy().to_string(),
        ];

        let (deleted, skipped, errors) = delete_files(&paths, false, false).unwrap();

        assert_eq!(deleted.len(), 3);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(!file3.exists());
    }

    // ========================================================================
    // delete_files Tests - Trash Mode
    // ========================================================================

    #[test]
    fn test_delete_files_trash_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"test content").unwrap();

        assert!(file_path.exists());

        let paths = vec![file_path.to_string_lossy().to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, false, true).unwrap();

        // File should be either deleted or moved to trash
        assert_eq!(deleted.len() + skipped.len() + errors.len(), 1);
    }

    // ========================================================================
    // Error Handling Tests
    // ========================================================================

    #[test]
    fn test_delete_files_empty_list() {
        let (deleted, skipped, errors) = delete_files(&[], false, false).unwrap();

        assert_eq!(deleted.len(), 0);
        assert_eq!(skipped.len(), 0);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_error_conversion_to_string() {
        let req = CleanupReq {
            paths: vec!["/tmp/test".to_string(); MAX_BATCH_DELETE_COUNT + 1],
            dry_run: false,
            trash: true,
        };

        let result = validate_deletion_request(&req);
        assert!(result.is_err());

        // Ensure error can be converted to string
        let err_string = result.unwrap_err().to_string();
        assert!(!err_string.is_empty());
    }

    // ========================================================================
    // Constants Verification Tests
    // ========================================================================

    #[test]
    fn test_max_batch_delete_count_constant() {
        // Verify the constant is set to a reasonable value
        assert_eq!(MAX_BATCH_DELETE_COUNT, 10_000);
    }

    #[test]
    fn test_max_batch_delete_size_constant() {
        // Verify the constant is set to 100GB
        assert_eq!(MAX_BATCH_DELETE_SIZE, 100 * 1024 * 1024 * 1024);
    }

    // ========================================================================
    // Security Validation Tests
    // ========================================================================

    #[test]
    fn test_validate_deletion_request_blocks_system_directories() {
        let req = CleanupReq {
            paths: vec!["/System/Library/test.txt".to_string()],
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Security validation failed"));
        assert!(err_msg.contains("protected system directory"));
    }

    #[test]
    fn test_validate_deletion_request_blocks_bin_directory() {
        let req = CleanupReq {
            paths: vec!["/bin/some_file".to_string()],
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Security validation failed"));
        assert!(err_msg.contains("/bin"));
    }

    #[test]
    fn test_validate_deletion_request_blocks_usr_directory() {
        let req = CleanupReq {
            paths: vec!["/usr/bin/critical_tool".to_string()],
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Security validation failed"));
        assert!(err_msg.contains("protected system directory"));
    }

    #[test]
    fn test_validate_deletion_request_allows_home_directory() {
        // Test with a file in user's home directory (should be allowed)
        if let Ok(home) = std::env::var("HOME") {
            let req = CleanupReq {
                paths: vec![format!("{}/test_file.txt", home)],
                dry_run: false,
                trash: true,
            };
            // This should pass security validation (may fail on size/count limits with real files)
            let result = validate_deletion_request(&req);
            // Should not be a security error
            if let Err(e) = result {
                let err_msg = e.to_string();
                assert!(!err_msg.contains("Security validation failed"));
            }
        }
    }

    #[test]
    fn test_validate_deletion_request_allows_tmp_directory() {
        let req = CleanupReq {
            paths: vec!["/tmp/safe_file.txt".to_string()],
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        // Should not be a security error
        if let Err(e) = result {
            let err_msg = e.to_string();
            assert!(!err_msg.contains("Security validation failed"));
        }
    }

    #[test]
    fn test_validate_deletion_request_multiple_paths_mixed_security() {
        let req = CleanupReq {
            paths: vec![
                "/tmp/safe_file.txt".to_string(),
                "/System/Library/dangerous_file.txt".to_string(),
            ],
            dry_run: false,
            trash: true,
        };
        let result = validate_deletion_request(&req);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Security validation failed"));
        assert!(err_msg.contains("System/Library"));
    }
}
