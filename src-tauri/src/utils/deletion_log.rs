//! Deletion history logging for audit trails and recovery purposes.
//!
//! This module provides persistent logging of all file deletions, enabling:
//! - **Audit trails** for compliance and debugging
//! - **Recovery** information (when was it deleted, by what action)
//! - **Analytics** on what types of files are being deleted
//! - **Safety verification** to confirm intended deletions occurred

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;



/// A single deletion record in the audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletionRecord {
    /// Full path to the deleted file or directory
    pub path: String,
    /// Size in bytes (0 if directory and size was not calculated)
    pub size_bytes: u64,
    /// Timestamp when deletion occurred (UTC)
    pub deleted_at: DateTime<Utc>,
    /// Category of what was deleted (e.g., "cache", "duplicate", "`large_file`", "junk")
    pub category: String,
    /// Deletion method: "trash" or "permanent"
    pub method: String,
    /// Whether this file has been restored (for future use with recovery feature)
    pub restored: bool,
}

impl DeletionRecord {
    /// Create a new deletion record
    #[must_use]
    pub fn new(path: String, size_bytes: u64, category: String, method: String) -> Self {
        Self {
            path,
            size_bytes,
            deleted_at: Utc::now(),
            category,
            method,
            restored: false,
        }
    }
}

/// Get the path to the deletion log file
fn get_log_path() -> Result<PathBuf, String> {
    // Use tauri's app_data_dir if available, otherwise use user's home
    let log_dir = if let Ok(data_dir) = std::env::var("APPDATA") {
        PathBuf::from(data_dir)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".disk-bloat-scanner")
    } else {
        return Err("Cannot determine log directory".to_string());
    };

    // Create directory if it doesn't exist
    std::fs::create_dir_all(&log_dir)
        .map_err(|e| format!("Failed to create log directory: {e}"))?;

    Ok(log_dir.join("deletion_log.jsonl"))
}

/// Log a deletion to the audit trail (append-only)
///
/// This function appends a JSON-serialized record to the deletion log file.
/// Each line is a complete JSON object (JSONL format) for easy parsing and recovery.
///
/// # Arguments
/// * `record` - The deletion record to log
///
/// # Errors
/// Returns an error if the log file cannot be created or written to.
///
/// # Example
/// ```no_run
/// use disk_bloat_scanner_lib::utils::deletion_log::{DeletionRecord, log_deletion};
///
/// let record = DeletionRecord::new(
///     "/tmp/cache.db".to_string(),
///     1024 * 1024,
///     "cache".to_string(),
///     "trash".to_string(),
/// );
/// log_deletion(&record)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn log_deletion(record: &DeletionRecord) -> Result<(), String> {
    let log_path = get_log_path()?;

    let json = serde_json::to_string(record)
        .map_err(|e| format!("Failed to serialize deletion record: {e}"))?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|e| format!("Failed to open deletion log: {e}"))?;

    writeln!(file, "{json}").map_err(|e| format!("Failed to write deletion log: {e}"))?;

    log::debug!("Logged deletion: {} ({})", record.path, record.category);

    Ok(())
}

/// Retrieve all deletion records from the log file
///
/// Returns all deletion records as a vector, parsed from the JSONL format.
/// If the log file doesn't exist, returns an empty vector.
///
/// # Errors
/// Returns an error if the log file cannot be read or parsed.
pub fn get_deletion_history() -> Result<Vec<DeletionRecord>, String> {
    let log_path = get_log_path()?;

    if !log_path.exists() {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(&log_path)
        .map_err(|e| format!("Failed to read deletion log: {e}"))?;

    let mut records = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<DeletionRecord>(line) {
            Ok(record) => records.push(record),
            Err(e) => {
                log::warn!("Failed to parse deletion log entry: {e} (line: {line})");
                // Continue parsing other lines
            }
        }
    }

    Ok(records)
}

/// Get deletion statistics for a given category
///
/// Returns the count and total size of deletions in a category.
pub fn get_category_stats(category: &str) -> Result<(usize, u64), String> {
    let history = get_deletion_history()?;

    let records: Vec<_> = history.iter().filter(|r| r.category == category).collect();

    let count = records.len();
    let total_size: u64 = records.iter().map(|r| r.size_bytes).sum();

    Ok((count, total_size))
}

/// Clear the deletion log (caution: this is irreversible)
///
/// This function should only be called by user action or when explicitly confirmed.
pub fn clear_log() -> Result<(), String> {
    let log_path = get_log_path()?;

    if log_path.exists() {
        std::fs::remove_file(&log_path)
            .map_err(|e| format!("Failed to clear deletion log: {e}"))?;

        log::warn!("Deletion log cleared by user action");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deletion_record_creation() {
        let record = DeletionRecord::new(
            "/test/file.txt".to_string(),
            1024,
            "test".to_string(),
            "trash".to_string(),
        );

        assert_eq!(record.path, "/test/file.txt");
        assert_eq!(record.size_bytes, 1024);
        assert_eq!(record.category, "test");
        assert_eq!(record.method, "trash");
        assert!(!record.restored);
    }

    #[test]
    fn test_deletion_record_serialization() {
        let record = DeletionRecord::new(
            "/test/file.txt".to_string(),
            1024,
            "cache".to_string(),
            "permanent".to_string(),
        );

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: DeletionRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(record.path, deserialized.path);
        assert_eq!(record.size_bytes, deserialized.size_bytes);
        assert_eq!(record.category, deserialized.category);
    }
}
