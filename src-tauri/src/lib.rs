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
/// Database module for persistent project monitoring.
pub mod database;
/// Project Auditor & Compliance Scanner (PACS) module.
pub mod pacs;
/// Architecture Visualization module for code analysis and diagram generation.
pub mod arch_viz;

pub use error::{ScannerError, ScannerResult};
pub use models::*;
use utils::cleanup;
use utils::path::validate_scan_path;
use utils::scan;
use database::{ProjectDatabase, ProjectScanResult, ProjectMonitorConfig};
use pacs::{DeepProjectScanner, PACSConfig, ProjectAuditReport, ProjectBaseline};
use arch_viz::{ArchVizEngine, ArchVizConfig, ArchitectureAnalysis};
use serde::{Deserialize, Serialize};

// Baseline comparison types
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BaselineComparison {
    baseline_score: f64,
    current_score: f64,
    score_change: f64,
    files_added: Vec<String>,
    files_removed: Vec<String>,
    files_modified: Vec<String>,
    compliance_changes: HashMap<String, ComplianceChange>,
    recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ComplianceChange {
    old: bool,
    new: bool,
}

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

/// Get lightweight git status for a repository path
#[tauri::command]
async fn get_git_repo_status(path: String) -> Result<GitRepoStatus, String> {
    use std::process::Command;

    // Validate the path to prevent system directory access
    let validated_path = validate_scan_path(&path)?;
    log::info!("Getting git status for: {}", validated_path.display());

    // Ensure path exists (redundant check but kept for clarity)
    let repo_path = &validated_path;
    if !repo_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // Normalize to the working tree root if a .git directory was provided
    let work_dir_path = if repo_path.file_name().map_or(false, |n| n == ".git") {
        repo_path.parent().unwrap_or(repo_path).to_path_buf()
    } else {
        repo_path.to_path_buf()
    };
    let work_dir = work_dir_path.to_string_lossy().to_string();

    // Branch name
    let branch = match Command::new("git")
        .arg("-C")
        .arg(&work_dir)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .trim()
            .to_string(),
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            log::warn!("git rev-parse failed: {}", err);
            "unknown".to_string()
        }
        Err(e) => {
            log::warn!("git not available: {}", e);
            "unknown".to_string()
        }
    };

    // Detect upstream presence
    let has_upstream = match Command::new("git")
        .arg("-C")
        .arg(&work_dir)
        .args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])
        .output()
    {
        Ok(out) => out.status.success(),
        Err(_) => false,
    };

    // Ahead/behind relative to upstream
    let (ahead, behind) = if has_upstream {
        match Command::new("git")
            .arg("-C")
            .arg(&work_dir)
            .args(["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
            .output()
        {
            Ok(out) if out.status.success() => {
                let txt = String::from_utf8_lossy(&out.stdout);
                let mut parts = txt.split_whitespace();
                let left = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
                let right = parts.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
                (left, right)
            }
            _ => (0, 0),
        }
    } else {
        (0, 0)
    };

    // Uncommitted/untracked counts via porcelain
    let (mut uncommitted, mut untracked) = (0u32, 0u32);
    if let Ok(out) = Command::new("git")
        .arg("-C")
        .arg(&work_dir)
        .args(["status", "--porcelain"]) // simpler to parse
        .output()
    {
        if out.status.success() {
            let txt = String::from_utf8_lossy(&out.stdout);
            for line in txt.lines() {
                if line.starts_with("??") {
                    untracked += 1;
                } else if !line.trim().is_empty() {
                    uncommitted += 1;
                }
            }
        }
    }

    // Last commit timestamp
    let last_commit_ts: u64 = match Command::new("git")
        .arg("-C")
        .arg(&work_dir)
        .args(["log", "-1", "--format=%ct"]) // unix timestamp
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        _ => 0,
    };

    Ok(GitRepoStatus {
        branch,
        ahead,
        behind,
        uncommitted,
        untracked,
        last_commit_ts,
        has_upstream,
    })
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
/// - `get_git_repo_status` - Get lightweight git status for a repository
/// - `store_project_scan` - Store project scan result in database
/// - `get_project_history` - Get project scan history
/// - `configure_project_monitoring` - Configure project monitoring
/// - `get_monitored_projects` - Get monitored projects
/// - `prepare_osm_migration` - Prepare OSM-lite migration plan

// ============================================================================
// Database Commands for Project Monitoring
// ============================================================================

/// Store project scan result in database
#[tauri::command]
async fn store_project_scan(
    project_path: String,
    total_size_mb: f64,
    bloat_size_mb: f64,
    large_files_count: i32,
    duplicates_count: i32,
    junk_files_count: i32,
    git_repo_status: Option<String>,
    project_type: Option<String>,
    compliance_score: Option<f64>,
) -> Result<i64, String> {
    use chrono::Utc;
    
    let db_path = "./data/project_monitor.db";
    std::fs::create_dir_all("./data").map_err(|e| format!("Failed to create data directory: {}", e))?;
    
    let db = ProjectDatabase::new(db_path).map_err(|e| format!("Database error: {}", e))?;
    
    let scan_result = ProjectScanResult {
        id: None,
        project_path,
        scan_timestamp: Utc::now(),
        total_size_mb,
        bloat_size_mb,
        large_files_count,
        duplicates_count,
        junk_files_count,
        git_repo_status,
        project_type,
        compliance_score,
    };
    
    db.store_scan_result(&scan_result).map_err(|e| format!("Failed to store scan result: {}", e))
}

/// Get project scan history
#[tauri::command]
async fn get_project_history(project_path: String, limit: i32) -> Result<Vec<ProjectScanResult>, String> {
    let db_path = "./data/project_monitor.db";
    let db = ProjectDatabase::new(db_path).map_err(|e| format!("Database error: {}", e))?;
    
    db.get_project_history(&project_path, limit).map_err(|e| format!("Failed to get project history: {}", e))
}

/// Configure project monitoring
#[tauri::command]
async fn configure_project_monitoring(
    project_path: String,
    monitor_enabled: bool,
    scan_interval_hours: i32,
    alert_thresholds: String,
) -> Result<i64, String> {
    use chrono::Utc;
    
    let db_path = "./data/project_monitor.db";
    let db = ProjectDatabase::new(db_path).map_err(|e| format!("Database error: {}", e))?;
    
    let config = ProjectMonitorConfig {
        id: None,
        project_path,
        monitor_enabled,
        scan_interval_hours,
        alert_thresholds,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    db.configure_monitoring(&config).map_err(|e| format!("Failed to configure monitoring: {}", e))
}

/// Get monitored projects
#[tauri::command]
async fn get_monitored_projects() -> Result<Vec<ProjectMonitorConfig>, String> {
    let db_path = "./data/project_monitor.db";
    let db = ProjectDatabase::new(db_path).map_err(|e| format!("Database error: {}", e))?;
    
    db.get_monitored_projects().map_err(|e| format!("Failed to get monitored projects: {}", e))
}

/// Prepare OSM-lite migration plan
#[tauri::command]
async fn prepare_osm_migration() -> Result<database::OSMMigrationPlan, String> {
    let db_path = "./data/project_monitor.db";
    let db = ProjectDatabase::new(db_path).map_err(|e| format!("Database error: {}", e))?;
    
    db.prepare_osm_migration().map_err(|e| format!("Failed to prepare OSM migration: {}", e))
}

// ============================================================================
// PACS Commands
// ============================================================================

/// Run deep project compliance scan
#[tauri::command]
async fn run_pacs_scan(project_path: String, config: Option<PACSConfig>) -> Result<ProjectAuditReport, String> {
    log::info!("Starting PACS scan for: {}", project_path);
    
    let config = config.unwrap_or_default();
    let mut scanner = DeepProjectScanner::new(&project_path, config);
    
    // Load existing baseline if available
    scanner.load_baseline().map_err(|e| format!("Failed to load baseline: {}", e))?;
    
    // Perform the scan
    let report = scanner.scan().await.map_err(|e| format!("Scan failed: {}", e))?;
    
    // Save the report
    scanner.save_report(&report).await.map_err(|e| format!("Failed to save report: {}", e))?;
    
    log::info!("PACS scan completed. Compliance score: {:.1}/100", report.compliance_score);
    
    Ok(report)
}

/// Get PACS configuration with defaults
#[tauri::command]
async fn get_pacs_config() -> Result<PACSConfig, String> {
    Ok(PACSConfig::default())
}

/// Update PACS configuration
#[tauri::command]
async fn update_pacs_config(config: PACSConfig) -> Result<(), String> {
    // For now, just validate the config
    log::info!("PACS config updated: auto_generate_specs={}, standards={:?}", 
               config.auto_generate_specs, config.standards);
    Ok(())
}

/// Get all baselines for a project
#[tauri::command]
async fn get_project_baselines(project_path: String) -> Result<Vec<ProjectBaseline>, String> {
    log::info!("Getting baselines for project: {}", project_path);
    
    let baselines_dir = Path::new(&project_path).join(".pacs").join("baselines");
    let mut baselines = Vec::new();
    
    if baselines_dir.exists() {
        for entry in std::fs::read_dir(&baselines_dir).map_err(|e| format!("Failed to read baselines directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read baseline entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read baseline file: {}", e))?;
                let baseline: ProjectBaseline = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse baseline: {}", e))?;
                baselines.push(baseline);
            }
        }
    }
    
    // Sort by captured_at date (newest first)
    baselines.sort_by(|a, b| b.captured_at.cmp(&a.captured_at));
    
    Ok(baselines)
}

/// Create a new baseline for a project
#[tauri::command]
async fn create_project_baseline(project_path: String, version: String, description: Option<String>) -> Result<(), String> {
    log::info!("Creating baseline '{}' for project: {}", version, project_path);
    
    let config = PACSConfig::default();
    let mut scanner = DeepProjectScanner::new(&project_path, config);
    
    // Load existing baseline if available
    scanner.load_baseline().map_err(|e| format!("Failed to load existing baseline: {}", e))?;
    
    // Perform scan to get current state
    let report = scanner.scan().await.map_err(|e| format!("Failed to scan project: {}", e))?;
    
    if let Some(baseline) = report.baseline {
        // Create versioned baseline
        let mut versioned_baseline = baseline;
        versioned_baseline.version = version.clone();
        
        // Save to baselines directory
        let baselines_dir = Path::new(&project_path).join(".pacs").join("baselines");
        std::fs::create_dir_all(&baselines_dir).map_err(|e| format!("Failed to create baselines directory: {}", e))?;
        
        let baseline_file = baselines_dir.join(format!("{}.json", version));
        let baseline_content = serde_json::to_string_pretty(&versioned_baseline)
            .map_err(|e| format!("Failed to serialize baseline: {}", e))?;
        
        std::fs::write(&baseline_file, baseline_content)
            .map_err(|e| format!("Failed to write baseline file: {}", e))?;
        
        log::info!("Created baseline '{}' at: {}", version, baseline_file.display());
    } else {
        return Err("Failed to generate baseline from scan".to_string());
    }
    
    Ok(())
}

/// Delete a project baseline
#[tauri::command]
async fn delete_project_baseline(project_path: String, version: String) -> Result<(), String> {
    log::info!("Deleting baseline '{}' for project: {}", version, project_path);
    
    let baseline_file = Path::new(&project_path)
        .join(".pacs")
        .join("baselines")
        .join(format!("{}.json", version));
    
    if baseline_file.exists() {
        std::fs::remove_file(&baseline_file)
            .map_err(|e| format!("Failed to delete baseline file: {}", e))?;
        log::info!("Deleted baseline file: {}", baseline_file.display());
    } else {
        return Err(format!("Baseline '{}' not found", version));
    }
    
    Ok(())
}

/// Compare current state with a baseline
#[tauri::command]
async fn compare_with_baseline(project_path: String, baseline_version: String, current_report: ProjectAuditReport) -> Result<BaselineComparison, String> {
    log::info!("Comparing current state with baseline '{}' for project: {}", baseline_version, project_path);
    
    // Load the baseline
    let baseline_file = Path::new(&project_path)
        .join(".pacs")
        .join("baselines")
        .join(format!("{}.json", baseline_version));
    
    if !baseline_file.exists() {
        return Err(format!("Baseline '{}' not found", baseline_version));
    }
    
    let baseline_content = std::fs::read_to_string(&baseline_file)
        .map_err(|e| format!("Failed to read baseline: {}", e))?;
    let baseline: ProjectBaseline = serde_json::from_str(&baseline_content)
        .map_err(|e| format!("Failed to parse baseline: {}", e))?;
    
    // Compare file inventories
    let baseline_files: std::collections::HashSet<_> = baseline.file_inventory.keys().collect();
    let current_files: std::collections::HashSet<_> = current_report.baseline
        .as_ref()
        .map(|b| b.file_inventory.keys().collect())
        .unwrap_or_default();
    
    let files_added: Vec<String> = current_files.difference(&baseline_files)
        .map(|s| s.to_string())
        .collect();
    let files_removed: Vec<String> = baseline_files.difference(&current_files)
        .map(|s| s.to_string())
        .collect();
    
    // Find modified files (same path, different hash)
    let mut files_modified = Vec::new();
    if let Some(current_baseline) = &current_report.baseline {
        for (path, current_meta) in &current_baseline.file_inventory {
            if let Some(baseline_meta) = baseline.file_inventory.get(path) {
                if current_meta.hash != baseline_meta.hash {
                    files_modified.push(path.clone());
                }
            }
        }
    }
    
    // Compare compliance changes
    let mut compliance_changes = std::collections::HashMap::new();
    if let Some(current_baseline) = &current_report.baseline {
        for (standard, current_compliant) in &current_baseline.standards_compliance {
            if let Some(baseline_compliant) = baseline.standards_compliance.get(standard) {
                if current_compliant != baseline_compliant {
                    compliance_changes.insert(standard.clone(), ComplianceChange {
                        old: *baseline_compliant,
                        new: *current_compliant,
                    });
                }
            }
        }
    }
    
    // Generate recommendations based on changes
    let mut recommendations = Vec::new();
    if !files_added.is_empty() {
        recommendations.push(format!("Review {} newly added files for compliance", files_added.len()));
    }
    if !files_removed.is_empty() {
        recommendations.push(format!("Verify that {} removed files were intentionally deleted", files_removed.len()));
    }
    if !files_modified.is_empty() {
        recommendations.push(format!("Review {} modified files for compliance impact", files_modified.len()));
    }
    
    let score_change = current_report.compliance_score - baseline.compliance_score;
    if score_change < -5.0 {
        recommendations.push("Significant compliance score decrease detected - review recent changes".to_string());
    } else if score_change > 5.0 {
        recommendations.push("Compliance score improved - consider creating a new baseline".to_string());
    }
    
    let comparison = BaselineComparison {
        baseline_score: baseline.compliance_score,
        current_score: current_report.compliance_score,
        score_change,
        files_added,
        files_removed,
        files_modified,
        compliance_changes,
        recommendations,
    };
    
    Ok(comparison)
}

// ============================================================================
// Architecture Visualization Commands
// ============================================================================

/// Run architecture analysis and generate diagrams
#[tauri::command]
async fn run_architecture_analysis(project_path: String, config: Option<ArchVizConfig>) -> Result<ArchitectureAnalysis, String> {
    log::info!("Starting architecture analysis for: {}", project_path);
    
    let config = config.unwrap_or_default();
    let mut engine = ArchVizEngine::new(&project_path, config)
        .map_err(|e| format!("Failed to create ArchViz engine: {}", e))?;
    
    // Perform the analysis
    let analysis = engine.analyze().await
        .map_err(|e| format!("Architecture analysis failed: {}", e))?;
    
    log::info!("Architecture analysis completed. {} modules analyzed", analysis.file_count);
    
    Ok(analysis)
}

/// Get default architecture visualization configuration
#[tauri::command]
async fn get_archviz_config() -> Result<ArchVizConfig, String> {
    Ok(ArchVizConfig::default())
}

/// Update architecture visualization configuration
#[tauri::command]
async fn update_archviz_config(config: ArchVizConfig) -> Result<(), String> {
    log::info!("ArchViz config updated: languages={:?}, max_depth={}", 
               config.languages, config.max_depth);
    Ok(())
}

/// Generate specific diagram format from existing analysis
#[tauri::command]
async fn generate_diagram(project_path: String, format: String, diagram_type: Option<String>) -> Result<String, String> {
    log::info!("Generating {} {} diagram for: {}", format, diagram_type.as_deref().unwrap_or("overview"), project_path);
    
    // Create a quick analysis for diagram generation
    let config = ArchVizConfig::default();
    let mut engine = ArchVizEngine::new(&project_path, config)
        .map_err(|e| format!("Failed to create ArchViz engine: {}", e))?;
    
    // Perform lightweight analysis
    let analysis = engine.analyze().await
        .map_err(|e| format!("Analysis failed: {}", e))?;
    
    // Generate the requested diagram type
    let diagram = match diagram_type.as_deref() {
        Some("dependency") => engine.generate_dependency_graph(&analysis.modules, &analysis.dependencies)
            .map_err(|e| format!("Failed to generate dependency graph: {}", e))?,
        Some("class") => engine.generate_class_hierarchy(&analysis.modules)
            .map_err(|e| format!("Failed to generate class hierarchy: {}", e))?,
        Some("files") => engine.generate_file_organization(&analysis.modules)
            .map_err(|e| format!("Failed to generate file organization: {}", e))?,
        Some("graphviz") => engine.generate_graphviz_diagram(&analysis.modules, &analysis.dependencies)
            .map_err(|e| format!("Failed to generate graphviz diagram: {}", e))?,
        Some("plantuml") => engine.generate_plantuml_diagram(&analysis.modules, &analysis.dependencies)
            .map_err(|e| format!("Failed to generate plantuml diagram: {}", e))?,
        _ => engine.generate_architecture_overview(&analysis.modules, &analysis.dependencies)
            .map_err(|e| format!("Failed to generate architecture overview: {}", e))?,
    };
    
    Ok(diagram)
}

/// Generate and export all diagram types automatically
#[tauri::command]
async fn export_all_diagrams(project_path: String, output_dir: Option<String>) -> Result<Vec<String>, String> {
    log::info!("Exporting all diagrams for: {}", project_path);
    
    let output_path = output_dir.unwrap_or_else(|| format!("{}/diagrams", project_path));
    std::fs::create_dir_all(&output_path).map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    let mut exported_files = Vec::new();
    
    // Generate all diagram types
    let diagram_types = vec![
        ("overview", "Architecture Overview"),
        ("dependency", "Dependency Graph"),
        ("class", "Class Hierarchy"),
        ("files", "File Organization"),
        ("graphviz", "Graphviz DOT"),
        ("plantuml", "PlantUML"),
    ];
    
    for (diagram_type, description) in diagram_types {
        match generate_diagram(project_path.clone(), "mermaid".to_string(), Some(diagram_type.to_string())).await {
            Ok(diagram_content) => {
                let filename = format!("{}/{}.{}", output_path, diagram_type, 
                    if diagram_type == "graphviz" { "dot" } 
                    else if diagram_type == "plantuml" { "puml" }
                    else { "mmd" }
                );
                
                if let Err(e) = std::fs::write(&filename, diagram_content) {
                    log::warn!("Failed to write {}: {}", filename, e);
                } else {
                    exported_files.push(format!("{}: {}", description, filename));
                    log::info!("Exported: {}", filename);
                }
            }
            Err(e) => {
                log::warn!("Failed to generate {} diagram: {}", diagram_type, e);
            }
        }
    }
    
    Ok(exported_files)
}

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
            cleanup_dirs,
            get_git_repo_status,
            store_project_scan,
            get_project_history,
            configure_project_monitoring,
            get_monitored_projects,
            prepare_osm_migration,
            run_pacs_scan,
            get_pacs_config,
            update_pacs_config,
            get_project_baselines,
            create_project_baseline,
            delete_project_baseline,
            compare_with_baseline,
            run_architecture_analysis,
            get_archviz_config,
            update_archviz_config,
            generate_diagram,
            export_all_diagrams
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

