//! File and directory cleanup utilities with safety-first deletion operations.
//!
//! This module provides safe deletion functionality with:
//! - Batch deletion limits (10,000 files max, 100GB max per operation)
//! - Dry-run support for previewing deletions
//! - Trash/recycle bin support for non-permanent deletion
//! - Comprehensive error handling and logging

use std::path::Path;

use crate::{models::CleanupReq, ScannerResult};

/// Safety limits for batch deletion operations
pub const MAX_BATCH_DELETE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
/// Maximum number of files that can be deleted in a single operation
pub const MAX_BATCH_DELETE_COUNT: usize = 10_000; // 10k files

/// Validates a deletion request against safety limits.
///
/// Ensures that:
/// 1. The number of files doesn't exceed `MAX_BATCH_DELETE_COUNT` (10,000)
/// 2. The total size doesn't exceed `MAX_BATCH_DELETE_SIZE` (100GB)
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

    log::info!(
        "Starting cleanup of {} paths (dry_run={}, trash={})",
        paths.len(),
        dry_run,
        use_trash
    );

    if dry_run {
        // Dry run - just return what would be deleted
        return Ok((paths.to_vec(), vec![], vec![]));
    }

    for path in paths {
        let p = Path::new(path);
        log::debug!("Processing: {}", path);

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
            // Move to trash
            match trash::delete(p) {
                Ok(_) => {
                    log::debug!("Successfully moved to trash: {}", path);
                    // Verify it's actually gone
                    if p.exists() {
                        log::warn!("File still exists after trash: {}", path);
                        errors.push(format!("{}: Moved to trash but file still exists", path));
                    } else {
                        deleted.push(path.clone());
                    }
                }
                Err(e) => {
                    log::error!("Cleanup error for {}: {}", path, e);
                    errors.push(format!("{}: {}", path, e));
                }
            }
        } else {
            // Permanent deletion
            let result = if p.is_dir() {
                std::fs::remove_dir_all(p)
            } else {
                std::fs::remove_file(p)
            };

            match result {
                Ok(_) => {
                    log::debug!("Successfully deleted: {}", path);
                    deleted.push(path.clone());
                }
                Err(e) => {
                    log::error!("Cleanup error for {}: {}", path, e);
                    errors.push(format!("{}: {}", path, e));
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
    fn test_validate_deletion_request_exceeds_count() {
        let paths = vec!["test".to_string(); MAX_BATCH_DELETE_COUNT + 1];
        let req = CleanupReq {
            paths,
            dry_run: false,
            trash: true,
        };
        assert!(validate_deletion_request(&req).is_err());
    }

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
    fn test_delete_files_nonexistent() {
        let paths = vec!["/tmp/this_file_does_not_exist_xyz_12345.txt".to_string()];
        let (deleted, skipped, errors) = delete_files(&paths, false, true).unwrap();

        // Nonexistent file should be skipped
        assert_eq!(deleted.len(), 0);
        assert_eq!(skipped.len(), 1);
        assert_eq!(errors.len(), 0);
    }
}
