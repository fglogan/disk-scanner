//! Disk Bloat Scanner - Find and remove unnecessary files to free up disk space.
//!
//! This library provides scanning and cleanup functionality for identifying bloated
//! files, caches, duplicates, and junk files on disk. All operations are designed
//! with safety-first principles to prevent accidental data loss.

/// Custom error types for scanner operations.
pub mod error;
/// Data models and structures for scan results.
pub mod models;
/// Utility modules for scanning, patterns, and path validation.
pub mod utils;

pub use error::{ScannerError, ScannerResult};
pub use models::*;
use utils::cleanup;
use utils::path::validate_scan_path;
use utils::scan;

// ============================================================================
// Tauri Commands
// ============================================================================

/// Get current disk usage information for the main drive.
///
/// Returns total disk size, used/free space, and usage percentage.
#[tauri::command]
async fn get_disk_info() -> Result<DiskInfoResponse, String> {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();

    // Find the main disk (usually the root or largest disk)
    let disk = disks
        .iter()
        .max_by_key(|d| d.total_space())
        .ok_or("No disks found")?;

    let total_bytes = disk.total_space();
    let available_bytes = disk.available_space();
    let used_bytes = total_bytes - available_bytes;

    let total_gb = total_bytes as f32 / 1_073_741_824.0;
    let used_gb = used_bytes as f32 / 1_073_741_824.0;
    let free_gb = available_bytes as f32 / 1_073_741_824.0;
    let usage_pct = if total_bytes > 0 {
        (used_bytes as f32 / total_bytes as f32) * 100.0
    } else {
        0.0
    };

    Ok(DiskInfoResponse {
        total_gb,
        used_gb,
        free_gb,
        usage_pct,
    })
}

/// Retrieves comprehensive system information including disk, memory, CPU, and OS details.
///
/// Returns a `SystemInfoResponse` containing:
/// - Disk usage (total, used, free in GB and percentage)
/// - Memory usage (total, used, free in GB and percentage)
/// - CPU core count
/// - Operating system name, version, and hostname
#[tauri::command]
async fn get_system_info() -> Result<SystemInfoResponse, String> {
    use sysinfo::{Disks, System};

    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();

    // Get disk info
    let disk = disks
        .iter()
        .max_by_key(|d| d.total_space())
        .ok_or("No disks found")?;

    let disk_total_bytes = disk.total_space();
    let disk_available_bytes = disk.available_space();
    let disk_used_bytes = disk_total_bytes - disk_available_bytes;

    let disk_total_gb = disk_total_bytes as f32 / 1_073_741_824.0;
    let disk_used_gb = disk_used_bytes as f32 / 1_073_741_824.0;
    let disk_free_gb = disk_available_bytes as f32 / 1_073_741_824.0;
    let disk_usage_pct = if disk_total_bytes > 0 {
        (disk_used_bytes as f32 / disk_total_bytes as f32) * 100.0
    } else {
        0.0
    };

    // Get memory info
    let memory_total_bytes = sys.total_memory();
    let memory_used_bytes = sys.used_memory();
    let memory_free_bytes = sys.available_memory();

    let memory_total_gb = memory_total_bytes as f32 / 1_073_741_824.0;
    let memory_used_gb = memory_used_bytes as f32 / 1_073_741_824.0;
    let memory_free_gb = memory_free_bytes as f32 / 1_073_741_824.0;
    let memory_usage_pct = if memory_total_bytes > 0 {
        (memory_used_bytes as f32 / memory_total_bytes as f32) * 100.0
    } else {
        0.0
    };

    // Get CPU and system info
    let cpu_count = sys.cpus().len();
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

    Ok(SystemInfoResponse {
        disk_total_gb,
        disk_used_gb,
        disk_free_gb,
        disk_usage_pct,
        memory_total_gb,
        memory_used_gb,
        memory_free_gb,
        memory_usage_pct,
        cpu_count,
        os_name,
        os_version,
        hostname,
    })
}

/// Scans a directory recursively to find large files exceeding a size threshold.
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.min_bytes` - Minimum file size threshold in bytes (default: 1GB)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Returns:** Vector of `LargeFileEntry` objects sorted by size (largest first),
/// each containing file path, size in MB, and last modification timestamp.
#[tauri::command]
async fn scan_large_files(opts: ScanOpts) -> Result<Vec<LargeFileEntry>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning large files in: {}", validated_path.display());

    scan::scan_large_files(&validated_path, opts.min_bytes, opts.follow_symlinks)
}

/// Scans a directory to identify bloat-prone directories (caches, logs, temporary files).
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Returns:** Vector of `BloatCategory` objects, each containing:
/// - Category ID and display name
/// - List of bloat entries with paths and sizes (MB)
/// - Total category size sorted by size (largest first)
#[tauri::command]
async fn scan_bloat(opts: ScanOpts) -> Result<Vec<BloatCategory>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning bloat in: {}", validated_path.display());

    scan::scan_bloat(&validated_path, opts.follow_symlinks)
}

/// Scans a directory to find duplicate files by comparing SHA-256 file hashes.
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Behavior:**
/// - Files smaller than 1KB or larger than 100MB are skipped
/// - Only files with same-size siblings are hashed for efficiency
/// - Duplicates are sorted by potential storage savings (largest first)
///
/// **Returns:** Vector of `DuplicateSet` objects containing:
/// - SHA-256 hash of duplicate group
/// - Total space that could be saved by removing duplicates (MB)
/// - All files in the duplicate group with paths, sizes, and modification times
#[tauri::command]
async fn scan_duplicates(opts: ScanOpts) -> Result<Vec<DuplicateSet>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning duplicates in: {}", validated_path.display());

    scan::scan_duplicates(&validated_path, opts.follow_symlinks)
}

/// Scans a directory for junk files matching known patterns (temp files, backups, OS artifacts).
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Returns:** Vector of `JunkCategory` objects containing:
/// - Category ID, display name, and safety level ("safe", "caution")
/// - List of junk file entries with paths, sizes, patterns, and file count
/// - Categories sorted by file count (most numerous first)
#[tauri::command]
async fn scan_junk_files(opts: ScanOpts) -> Result<Vec<JunkCategory>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning junk files in: {}", validated_path.display());

    scan::scan_junk_files(&validated_path, opts.follow_symlinks)
}



/// Deletes files and directories from the file system with optional dry-run and trash support.
///
/// **Parameters:**
/// - `req.paths` - Vector of file/directory paths to delete (max 10,000 paths, max 100GB total)
/// - `req.dry_run` - If true, returns what would be deleted without performing actual deletion
/// - `req.trash` - If true, moves files to trash; if false, permanently deletes them
///
/// **Safety Limits:**
/// - Maximum 10,000 files per operation (enforced by cleanup module)
/// - Maximum 100GB per operation (enforced by cleanup module)
/// - Validates deletion request before executing
///
/// **Returns:** `CleanupResult` containing:
/// - `deleted` - Vector of successfully deleted paths
/// - `skipped` - Vector of files that were already deleted or not found
/// - `errors` - Vector of error messages for failed deletions
#[tauri::command]
async fn cleanup_dirs(req: CleanupReq) -> Result<CleanupResult, String> {
    // Validate deletion request using cleanup module
    cleanup::validate_deletion_request(&req).map_err(|e| e.to_string())?;

    // Execute deletion using cleanup module
    let (deleted, skipped, errors) =
        cleanup::delete_files(&req.paths, req.dry_run, req.trash)
            .map_err(|e| e.to_string())?;

    Ok(CleanupResult {
        deleted,
        skipped,
        errors,
    })
}

// ============================================================================
// Developer Caches Scanner Command
// ============================================================================

/// Scans a directory for developer tool caches (npm, Cargo, pip, Maven, Gradle, Docker, etc.).
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Cache Types Detected:**
/// - Node.js/npm/yarn, Python/pip, Rust/Cargo, Java/Maven/Gradle
/// - Docker, VS Code, IntelliJ IDEA, macOS system caches
///
/// **Returns:** Vector of `CacheCategory` objects containing:
/// - Category ID, display name, and safety level
/// - List of cache entries with paths, sizes (MB), cache type, and descriptions
/// - Total size per category and entry count, sorted by size (largest first)
#[tauri::command]
async fn scan_dev_caches(opts: ScanOpts) -> Result<Vec<CacheCategory>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning developer caches in: {}", validated_path.display());

    scan::scan_dev_caches(&validated_path, opts.follow_symlinks)
}

// ============================================================================
// Git Repository Scanner Command
// ============================================================================

/// Scans a directory recursively to discover and analyze Git repositories.
///
/// **Parameters:**
/// - `opts.root` - Root directory path to scan (must not be a protected system directory)
/// - `opts.follow_symlinks` - Whether to follow symbolic links during traversal
///
/// **Analysis Includes:**
/// - Repository path and root directory location
/// - .git directory size (indicating repository metadata overhead)
/// - Total repository size (all files included)
/// - Branch information and latest commit details
/// - File and directory counts within the repository
///
/// **Returns:** Vector of `GitRepository` objects sorted by repository size (largest first),
/// each containing repository statistics and metadata.
#[tauri::command]
async fn scan_git_repos(opts: ScanOpts) -> Result<Vec<GitRepository>, String> {
    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning git repositories in: {}", validated_path.display());

    scan::scan_git_repos(&validated_path, opts.follow_symlinks)
}

/// Initializes and runs the Tauri application with all scanning and cleanup commands.
///
/// This function sets up the Tauri runtime, registers plugins for logging and file dialogs,
/// and registers all command handlers for disk scanning operations.
/// 
/// The application exposes the following commands to the frontend:
/// - `get_disk_info` - Retrieve disk usage statistics
/// - `get_system_info` - Retrieve system information
/// - `scan_large_files` - Scan for large files exceeding size thresholds
/// - `scan_bloat` - Detect bloated files and directories
/// - `scan_duplicates` - Find duplicate files by hash
/// - `scan_junk_files` - Detect junk files (cache, temp, etc.)
/// - `scan_dev_caches` - Analyze developer tool caches
/// - `scan_git_repos` - Find and analyze Git repositories
/// - `cleanup_dirs` - Safely delete selected files and directories
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_disk_info,
            get_system_info,
            scan_large_files,
            scan_bloat,
            scan_duplicates,
            scan_junk_files,
            scan_dev_caches,
            scan_git_repos,
            cleanup_dirs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
