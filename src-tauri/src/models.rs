//! Data models for disk scanning and cleanup operations.
//!
//! This module contains all serializable data structures used for communication
//! between the Tauri backend and frontend, as well as internal request/response types.
#![allow(clippy::doc_markdown)]

use serde::{Deserialize, Serialize};

// ============================================================================
// System Information Responses
// ============================================================================

/// Disk space information for a specific volume/partition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskInfoResponse {
    /// Total disk space in gigabytes
    pub total_gb: f32,
    /// Used disk space in gigabytes
    pub used_gb: f32,
    /// Free disk space in gigabytes
    pub free_gb: f32,
    /// Usage percentage (0-100)
    pub usage_pct: f32,
}

/// Comprehensive system information including disk and memory
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemInfoResponse {
    /// Total disk space in gigabytes
    pub disk_total_gb: f32,
    /// Used disk space in gigabytes
    pub disk_used_gb: f32,
    /// Free disk space in gigabytes
    pub disk_free_gb: f32,
    /// Disk usage percentage (0-100)
    pub disk_usage_pct: f32,
    /// Total memory in gigabytes
    pub memory_total_gb: f32,
    /// Used memory in gigabytes
    pub memory_used_gb: f32,
    /// Free memory in gigabytes
    pub memory_free_gb: f32,
    /// Memory usage percentage (0-100)
    pub memory_usage_pct: f32,
    /// Number of CPU cores
    pub cpu_count: usize,
    /// Operating system name (e.g., "macOS", "Windows", "Linux")
    pub os_name: String,
    /// Operating system version
    pub os_version: String,
    /// Computer hostname
    pub hostname: String,
}

// ============================================================================
// Scan Request & Response Types
// ============================================================================

/// Options for initiating a scan operation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScanOpts {
    /// Root directory path to scan
    pub root: String,
    /// Minimum file size in bytes to include
    pub min_bytes: Option<u64>,
    /// Whether to follow symbolic links
    pub follow_symlinks: bool,
}

/// Cleanup request specifying files to delete
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CleanupReq {
    /// List of file/directory paths to clean up
    pub paths: Vec<String>,
    /// If true, perform simulation without actual deletion
    pub dry_run: bool,
    /// If true, move to trash instead of permanent deletion
    pub trash: bool,
}

/// Result of a cleanup operation
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CleanupResult {
    /// Paths that were successfully deleted
    pub deleted: Vec<String>,
    /// Paths that were skipped (permission denied, in use, etc.)
    pub skipped: Vec<String>,
    /// Error messages for paths that failed
    pub errors: Vec<String>,
}

// ============================================================================
// Large Files Scan Results
// ============================================================================

/// Information about a single large file
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LargeFileEntry {
    /// Absolute file path
    pub path: String,
    /// File size in megabytes
    pub size_mb: f32,
    /// Last modification timestamp (Unix epoch seconds)
    pub last_modified: u64,
}

// ============================================================================
// Build Artifact (Bloat) Scan Results
// ============================================================================

/// Single file/directory detected as build artifact bloat
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BloatEntry {
    /// Absolute path to bloat file/directory
    pub path: String,
    /// Size in megabytes
    pub size_mb: f32,
}

/// Grouped collection of bloat entries by category
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BloatCategory {
    /// Category identifier (e.g., `node_modules`, `rust_target`)
    pub category_id: String,
    /// Human-readable category name (e.g., "Node.js", "Rust")
    pub display_name: String,
    /// Total size of all entries in this category (MB)
    pub total_size_mb: f32,
    /// List of individual bloat entries
    pub entries: Vec<BloatEntry>,
}

// ============================================================================
// Duplicate Files Scan Results
// ============================================================================

/// Single file identified as a duplicate
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DuplicateEntry {
    /// Absolute file path
    pub path: String,
    /// File size in megabytes
    pub size_mb: f32,
    /// Last modification timestamp (Unix epoch seconds)
    pub last_modified: u64,
}

/// Collection of duplicate files with the same content hash
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DuplicateSet {
    /// SHA256 hash of the file content
    pub hash: String,
    /// Total size that could be saved by keeping only one copy (MB)
    pub total_savable_mb: f32,
    /// List of all files with this same content
    pub entries: Vec<DuplicateEntry>,
}

// ============================================================================
// Junk Files Scan Results
// ============================================================================

/// Information about a single junk file detected
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JunkFileEntry {
    /// Absolute file path
    pub path: String,
    /// File size in kilobytes
    pub size_kb: f32,
    /// Pattern name that matched (e.g., ".DS_Store", "*.log")
    pub pattern: String,
    /// Category ID for grouping (e.g., "system", "logs")
    pub category: String,
    /// Safety level: "safe", "caution", or "dangerous"
    pub safety: String,
}

/// Grouped collection of junk files by category
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JunkCategory {
    /// Category identifier (e.g., "system", "logs", "cache")
    pub category_id: String,
    /// Human-readable category name
    pub display_name: String,
    /// Total size of all files in this category (KB)
    pub total_size_kb: f32,
    /// Count of files in this category
    pub file_count: usize,
    /// Overall safety level for the category
    pub safety: String,
    /// List of individual junk files
    pub files: Vec<JunkFileEntry>,
}

// ============================================================================
// Developer Cache Scan Results
// ============================================================================

/// Information about a single cache entry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheEntry {
    /// Absolute path to cache file/directory
    pub path: String,
    /// Size in megabytes
    pub size_mb: f32,
    /// Type of cache (e.g., "npm", "pip", "gradle")
    pub cache_type: String,
    /// Safety level: "safe", "caution", or "dangerous"
    pub safety: String,
    /// Human-readable description of the cache
    pub description: String,
}

/// Grouped collection of cache entries by type
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheCategory {
    /// Category identifier (e.g., "npm_cache", "pip_cache")
    pub category_id: String,
    /// Human-readable category name
    pub display_name: String,
    /// Total size of all caches in this category (MB)
    pub total_size_mb: f32,
    /// Number of cache entries
    pub entry_count: usize,
    /// Overall safety level for the category
    pub safety: String,
    /// List of individual cache entries
    pub entries: Vec<CacheEntry>,
}

// ============================================================================
// Git Repository Scan Results
// ============================================================================

/// Information about a problematic file within a Git repository
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitEntry {
    /// Absolute path to the file within the repository
    pub path: String,
    /// Size in megabytes
    pub size_mb: f32,
    /// Type of git object: "large_file", "loose_object", "pack_file", "reflog"
    pub entry_type: String,
    /// Human-readable description of what this is
    pub description: String,
    /// Safety level: "safe", "caution", or "dangerous"
    pub safety: String,
    /// Whether this entry can be safely cleaned up
    pub actionable: bool,
}

/// Scan results for a single Git repository
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitRepository {
    /// Absolute path to the .git directory
    pub repo_path: String,
    /// Total size of problematic entries (MB)
    pub total_size_mb: f32,
    /// Number of problematic entries found
    pub entry_count: usize,
    /// List of individual problematic entries
    pub entries: Vec<GitEntry>,
}

/// Lightweight repository status for UI
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GitRepoStatus {
    /// Current branch name ("unknown" if unavailable)
    pub branch: String,
    /// Commits ahead of upstream
    pub ahead: u32,
    /// Commits behind upstream
    pub behind: u32,
    /// Count of staged/unstaged changes (excluding untracked)
    pub uncommitted: u32,
    /// Count of untracked files
    pub untracked: u32,
    /// Last commit UNIX timestamp (0 if unavailable)
    pub last_commit_ts: u64,
    /// Whether an upstream is configured
    pub has_upstream: bool,
}

// ============================================================================
// Pattern Detection Structures (Internal)
// ============================================================================

/// Pattern definition for detecting build artifact bloat
#[derive(Debug, Clone)]
pub struct BloatPattern {
    /// Unique identifier for this category
    pub category_id: &'static str,
    /// Display name for UI presentation
    pub display_name: &'static str,
    /// Directory names to match (e.g., "node_modules", "target")
    pub dir_names: &'static [&'static str],
}

/// Pattern definition for detecting junk files
#[derive(Debug, Clone)]
pub struct JunkPattern {
    /// File pattern to match (e.g., ".DS_Store", "*.log")
    pub pattern: &'static str,
    /// Category identifier for grouping
    pub category_id: &'static str,
    /// Display name for UI presentation
    pub display_name: &'static str,
    /// Safety assessment: "safe", "caution", or "dangerous"
    pub safety: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_info_serialization() {
        let info = DiskInfoResponse {
            total_gb: 500.0,
            used_gb: 250.0,
            free_gb: 250.0,
            usage_pct: 50.0,
        };
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: DiskInfoResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_gb, 500.0);
    }

    #[test]
    fn test_scan_opts_with_min_bytes() {
        let opts = ScanOpts {
            root: "/home/user".to_string(),
            min_bytes: Some(1024 * 1024), // 1MB
            follow_symlinks: false,
        };
        assert_eq!(opts.min_bytes, Some(1024 * 1024));
    }

    #[test]
    fn test_cleanup_result_structure() {
        let result = CleanupResult {
            deleted: vec!["/path/to/file1".to_string()],
            skipped: vec!["/path/to/file2".to_string()],
            errors: vec!["Permission denied".to_string()],
        };
        assert_eq!(result.deleted.len(), 1);
        assert_eq!(result.skipped.len(), 1);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_bloat_category_aggregation() {
        let category = BloatCategory {
            category_id: "node_modules".to_string(),
            display_name: "Node.js".to_string(),
            total_size_mb: 1000.0,
            entries: vec![
                BloatEntry {
                    path: "/project1/node_modules".to_string(),
                    size_mb: 500.0,
                },
                BloatEntry {
                    path: "/project2/node_modules".to_string(),
                    size_mb: 500.0,
                },
            ],
        };
        assert_eq!(category.entries.len(), 2);
        assert_eq!(category.total_size_mb, 1000.0);
    }

    #[test]
    fn test_junk_safety_levels() {
        let safe_junk = JunkFileEntry {
            path: "/home/.DS_Store".to_string(),
            size_kb: 10.0,
            pattern: ".DS_Store".to_string(),
            category: "system".to_string(),
            safety: "safe".to_string(),
        };
        assert_eq!(safe_junk.safety, "safe");
    }
}
