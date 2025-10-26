//! Disk Bloat Scanner - Find and remove unnecessary files to free up disk space.
//!
//! This library provides scanning and cleanup functionality for identifying bloated
//! files, caches, duplicates, and junk files on disk. All operations are designed
//! with safety-first principles to prevent accidental data loss.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

pub mod error;
pub mod utils;

pub use error::{ScannerError, ScannerResult};
use utils::path::validate_scan_path;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct DiskInfoResponse {
    pub total_gb: f32,
    pub used_gb: f32,
    pub free_gb: f32,
    pub usage_pct: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemInfoResponse {
    pub disk_total_gb: f32,
    pub disk_used_gb: f32,
    pub disk_free_gb: f32,
    pub disk_usage_pct: f32,
    pub memory_total_gb: f32,
    pub memory_used_gb: f32,
    pub memory_free_gb: f32,
    pub memory_usage_pct: f32,
    pub cpu_count: usize,
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LargeFileEntry {
    pub path: String,
    pub size_mb: f32,
    pub last_modified: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BloatEntry {
    pub path: String,
    pub size_mb: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BloatCategory {
    pub category_id: String,
    pub display_name: String,
    pub total_size_mb: f32,
    pub entries: Vec<BloatEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DuplicateEntry {
    pub path: String,
    pub size_mb: f32,
    pub last_modified: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DuplicateSet {
    pub hash: String,
    pub total_savable_mb: f32,
    pub entries: Vec<DuplicateEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScanOpts {
    pub root: String,
    pub min_bytes: Option<u64>,
    pub follow_symlinks: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CleanupReq {
    pub paths: Vec<String>,
    pub dry_run: bool,
    pub trash: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CleanupResult {
    pub deleted: Vec<String>,
    pub skipped: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JunkFileEntry {
    pub path: String,
    pub size_kb: f32,
    pub pattern: String,
    pub category: String,
    pub safety: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JunkCategory {
    pub category_id: String,
    pub display_name: String,
    pub total_size_kb: f32,
    pub file_count: usize,
    pub safety: String,
    pub files: Vec<JunkFileEntry>,
}

// ============================================================================
// Developer Caches & Git Scanner Data Structures
// ============================================================================

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    pub path: String,
    pub size_mb: f32,
    pub cache_type: String,
    pub safety: String, // "safe", "caution", "dangerous"
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CacheCategory {
    pub category_id: String,
    pub display_name: String,
    pub total_size_mb: f32,
    pub entry_count: usize,
    pub safety: String,
    pub entries: Vec<CacheEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GitEntry {
    pub path: String,
    pub size_mb: f32,
    pub entry_type: String, // "large_file", "loose_object", "pack_file", "reflog"
    pub description: String,
    pub safety: String,
    pub actionable: bool, // Can this be safely cleaned up?
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GitRepository {
    pub repo_path: String,
    pub total_size_mb: f32,
    pub entry_count: usize,
    pub entries: Vec<GitEntry>,
}

// ============================================================================
// Bloat Pattern Detection
// ============================================================================

#[derive(Debug)]
struct BloatPattern {
    category_id: &'static str,
    display_name: &'static str,
    dir_names: &'static [&'static str],
}

const BLOAT_PATTERNS: &[BloatPattern] = &[
    BloatPattern {
        category_id: "node_modules",
        display_name: "Node.js",
        dir_names: &["node_modules"],
    },
    BloatPattern {
        category_id: "rust_target",
        display_name: "Rust",
        dir_names: &["target"],
    },
    BloatPattern {
        category_id: "python_venv",
        display_name: "Python",
        dir_names: &[
            "venv",
            ".venv",
            "__pycache__",
            ".pytest_cache",
            ".mypy_cache",
        ],
    },
    BloatPattern {
        category_id: "git",
        display_name: ".git",
        dir_names: &[".git"],
    },
    BloatPattern {
        category_id: "build_artifacts",
        display_name: "Build Artifacts",
        dir_names: &["dist", "build", ".next", ".nuxt", "out", ".output"],
    },
    BloatPattern {
        category_id: "vendor",
        display_name: "Vendor",
        dir_names: &["vendor"],
    },
    BloatPattern {
        category_id: "java_gradle",
        display_name: "Java/Gradle",
        dir_names: &[".gradle", ".m2"],
    },
];

fn detect_bloat_category(path: &Path) -> Option<(&'static str, &'static str)> {
    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
        for pattern in BLOAT_PATTERNS {
            if pattern.dir_names.contains(&dir_name) {
                return Some((pattern.category_id, pattern.display_name));
            }
        }
    }
    None
}

fn dir_size(path: &Path) -> u64 {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

// ============================================================================
// Junk File Pattern Detection
// ============================================================================

#[derive(Debug)]
struct JunkPattern {
    pattern: &'static str,
    category_id: &'static str,
    display_name: &'static str,
    safety: &'static str,
}

const JUNK_PATTERNS: &[JunkPattern] = &[
    // Tier 1: System Junk (100% Safe)
    JunkPattern {
        pattern: ".DS_Store",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "Thumbs.db",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "desktop.ini",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: ".localized",
        category_id: "system",
        display_name: "System Files",
        safety: "safe",
    },
    // Tier 2: Build Artifacts
    JunkPattern {
        pattern: "*.pyc",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.pyo",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.class",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.o",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.obj",
        category_id: "build",
        display_name: "Build Artifacts",
        safety: "safe",
    },
    // Tier 3: Editor Junk
    JunkPattern {
        pattern: "*.swp",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.swo",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.swn",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*~",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.bak",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
    JunkPattern {
        pattern: "*.backup",
        category_id: "editor",
        display_name: "Editor Files",
        safety: "safe",
    },
];

fn matches_junk_pattern(filename: &str, pattern: &str) -> bool {
    if pattern.starts_with("*.") {
        // Extension match (e.g., "*.pyc")
        let ext = &pattern[1..]; // Remove the *
        filename.ends_with(ext)
    } else if pattern.starts_with('*') && !pattern.ends_with('*') {
        // Suffix match (e.g., "*~" matches "test~")
        let suffix = &pattern[1..]; // Remove the *
        filename.ends_with(suffix)
    } else if pattern.ends_with('*') {
        // Prefix match (e.g., "test*")
        let prefix = &pattern[..pattern.len() - 1];
        filename.starts_with(prefix)
    } else if pattern.contains('*') {
        // Middle wildcard (not implemented for now)
        false
    } else {
        // Exact match (e.g., ".DS_Store")
        filename == pattern
    }
}

fn detect_junk_file(filename: &str) -> Option<(&'static str, &'static str, &'static str)> {
    for pattern in JUNK_PATTERNS {
        if matches_junk_pattern(filename, pattern.pattern) {
            return Some((pattern.category_id, pattern.display_name, pattern.safety));
        }
    }
    None
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
    use rayon::prelude::*;
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning large files in: {}", validated_path.display());

    let min_size = opts.min_bytes.unwrap_or(1024 * 1024 * 1024); // Default 1GB

    let entries: Vec<_> = WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let large_files: Vec<LargeFileEntry> = entries
        .par_iter()
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let size = metadata.len();

            if size >= min_size {
                let last_modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                Some(LargeFileEntry {
                    path: entry.path().to_string_lossy().to_string(),
                    size_mb: size as f32 / 1_048_576.0,
                    last_modified,
                })
            } else {
                None
            }
        })
        .collect();

    let mut sorted = large_files;
    sorted.sort_by(|a, b| error::compare_f32_safe(b.size_mb, a.size_mb));

    Ok(sorted)
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
    use std::sync::Mutex;
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning bloat in: {}", validated_path.display());

    let categories: Mutex<HashMap<String, (String, Vec<BloatEntry>)>> = Mutex::new(HashMap::new());

    // Walk the directory tree
    for entry in WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            if let Some((category_id, display_name)) = detect_bloat_category(entry.path()) {
                // Calculate directory size
                let size_bytes = dir_size(entry.path());
                let size_mb = size_bytes as f32 / 1_048_576.0;

                // Only include if size is significant (> 1MB)
                if size_mb > 1.0 {
                    let mut cats = categories.lock().expect("categories mutex poisoned");
                    let cat_entry = cats
                        .entry(category_id.to_string())
                        .or_insert_with(|| (display_name.to_string(), Vec::new()));

                    cat_entry.1.push(BloatEntry {
                        path: entry.path().to_string_lossy().to_string(),
                        size_mb,
                    });
                }
            }
        }
    }

    // Convert to result format
    let mut result: Vec<BloatCategory> = categories
        .lock()
        .expect("categories mutex poisoned")
        .drain()
        .map(|(category_id, (display_name, entries))| {
            let total_size_mb: f32 = entries.iter().map(|e| e.size_mb).sum();
            BloatCategory {
                category_id,
                display_name,
                total_size_mb,
                entries,
            }
        })
        .collect();

    // Sort by total size (largest first)
    result.sort_by(|a, b| error::compare_f32_safe(b.total_size_mb, a.total_size_mb));

    Ok(result)
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
    use rayon::prelude::*;
    use sha2::{Digest, Sha256};
    use std::sync::Mutex;
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning duplicates in: {}", validated_path.display());

    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB limit for hashing

    // First pass: collect all files with their sizes
    let files: Vec<_> = WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let metadata = e.metadata().ok()?;
            let size = metadata.len();

            // Skip files that are too large or too small
            if size > MAX_FILE_SIZE || size < 1024 {
                return None;
            }

            let last_modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            Some((e.path().to_path_buf(), size, last_modified))
        })
        .collect();

    // Group files by size first (optimization)
    let mut size_groups: HashMap<u64, Vec<(std::path::PathBuf, u64)>> = HashMap::new();
    for (path, size, last_mod) in files {
        size_groups
            .entry(size)
            .or_insert_with(Vec::new)
            .push((path, last_mod));
    }

    // Only hash files that have potential duplicates (same size)
    let file_hashes: Mutex<HashMap<String, Vec<DuplicateEntry>>> = Mutex::new(HashMap::new());

    size_groups
        .par_iter()
        .filter(|(_, files)| files.len() > 1) // Only files with same-size siblings
        .for_each(|(size, files)| {
            for (path, last_mod) in files {
                if let Ok(mut file) = std::fs::File::open(path) {
                    let mut hasher = Sha256::new();
                    if std::io::copy(&mut file, &mut hasher).is_ok() {
                        let hash = format!("{:x}", hasher.finalize());
                        let size_mb = *size as f32 / 1_048_576.0;

                        let mut hashes = file_hashes.lock().expect("file_hashes mutex poisoned");
                        hashes
                            .entry(hash)
                            .or_insert_with(Vec::new)
                            .push(DuplicateEntry {
                                path: path.to_string_lossy().to_string(),
                                size_mb,
                                last_modified: *last_mod,
                            });
                    }
                }
            }
        });

    // Convert to result format
    let mut result: Vec<DuplicateSet> = file_hashes
        .lock()
        .expect("file_hashes mutex poisoned")
        .drain()
        .filter(|(_, entries)| entries.len() > 1) // Only actual duplicates
        .map(|(hash, entries)| {
            // Calculate savable space (all copies except one)
            let single_file_size = entries.first().map(|e| e.size_mb).unwrap_or(0.0);
            let total_savable_mb = single_file_size * (entries.len() - 1) as f32;

            DuplicateSet {
                hash,
                total_savable_mb,
                entries,
            }
        })
        .collect();

    // Sort by savable space (largest first)
    result.sort_by(|a, b| error::compare_f32_safe(b.total_savable_mb, a.total_savable_mb));

    Ok(result)
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
    use std::sync::Mutex;
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning junk files in: {}", validated_path.display());

    let junk_files: Mutex<HashMap<String, (String, String, Vec<JunkFileEntry>)>> =
        Mutex::new(HashMap::new());

    // Walk the directory tree and find junk files
    for entry in WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(filename) = entry.path().file_name().and_then(|n| n.to_str()) {
                if let Some((category_id, display_name, safety)) = detect_junk_file(filename) {
                    // Get file size
                    let size_bytes = entry.metadata().ok().map(|m| m.len()).unwrap_or(0);
                    let size_kb = size_bytes as f32 / 1024.0;

                    // NO minimum size - catch even 0-byte files
                    let mut junk = junk_files.lock().expect("junk_files mutex poisoned");
                    let cat_entry = junk.entry(category_id.to_string()).or_insert_with(|| {
                        (display_name.to_string(), safety.to_string(), Vec::new())
                    });

                    cat_entry.2.push(JunkFileEntry {
                        path: entry.path().to_string_lossy().to_string(),
                        size_kb,
                        pattern: filename.to_string(),
                        category: category_id.to_string(),
                        safety: safety.to_string(),
                    });
                }
            }
        }
    }

    // Convert to result format
    let mut result: Vec<JunkCategory> = junk_files
        .lock()
        .expect("junk_files mutex poisoned")
        .drain()
        .map(|(category_id, (display_name, safety, files))| {
            let total_size_kb: f32 = files.iter().map(|f| f.size_kb).sum();
            let file_count = files.len();

            JunkCategory {
                category_id,
                display_name,
                total_size_kb,
                file_count,
                safety,
                files,
            }
        })
        .collect();

    // Sort by file count (most files first)
    result.sort_by(|a, b| b.file_count.cmp(&a.file_count));

    Ok(result)
}

// Safety limits for batch deletion
const MAX_BATCH_DELETE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
const MAX_BATCH_DELETE_COUNT: usize = 10_000; // 10k files

fn validate_deletion_request(req: &CleanupReq) -> Result<(), String> {
    if req.paths.len() > MAX_BATCH_DELETE_COUNT {
        return Err(format!(
            "Cannot delete {} files at once (maximum: {})",
            req.paths.len(),
            MAX_BATCH_DELETE_COUNT
        ));
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
        ));
    }

    Ok(())
}

/// Deletes files and directories from the file system with optional dry-run and trash support.
///
/// **Parameters:**
/// - `req.paths` - Vector of file/directory paths to delete (max 10,000 paths, max 100GB total)
/// - `req.dry_run` - If true, returns what would be deleted without performing actual deletion
/// - `req.trash` - If true, moves files to trash; if false, permanently deletes them
///
/// **Safety Limits:**
/// - Maximum 10,000 files per operation
/// - Maximum 100GB per operation
/// - Validates deletion request before executing
///
/// **Returns:** `CleanupResult` containing:
/// - `deleted` - Vector of successfully deleted paths
/// - `skipped` - Vector of files that were already deleted or not found
/// - `errors` - Vector of error messages for failed deletions
#[tauri::command]
async fn cleanup_dirs(req: CleanupReq) -> Result<CleanupResult, String> {
    let mut deleted = Vec::new();
    let mut skipped = Vec::new();
    let mut errors = Vec::new();

    log::info!(
        "Starting cleanup of {} paths (dry_run={}, trash={})",
        req.paths.len(),
        req.dry_run,
        req.trash
    );

    // Validate deletion request
    validate_deletion_request(&req)?;

    if req.dry_run {
        // Dry run - just return what would be deleted
        return Ok(CleanupResult {
            deleted: req.paths.clone(),
            skipped: vec![],
            errors: vec![],
        });
    }

    for path in &req.paths {
        let p = Path::new(path);
        log::debug!("Processing: {}", path);

        if !p.exists() {
            log::debug!("File does not exist, skipping: {}", path);
            skipped.push(path.clone());
            continue;
        }

        log::debug!(
            "File exists, attempting deletion (trash={}): {}",
            req.trash,
            path
        );

        if req.trash {
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
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning developer caches in: {}", validated_path.display());

    // Define cache patterns with safety levels
    let cache_patterns = [
        // Node.js/npm/yarn
        (
            "node_modules",
            "nodejs",
            "Node.js Dependencies",
            "caution",
            "Package dependencies - will be re-downloaded on next install",
        ),
        (
            "npm_cache",
            "nodejs",
            "npm Cache",
            "safe",
            "Downloaded package metadata and tarballs",
        ),
        (
            "yarn_cache",
            "nodejs",
            "Yarn Cache",
            "safe",
            "Yarn package cache",
        ),
        // Python
        (
            "__pycache__",
            "python",
            "Python Bytecode",
            "safe",
            "Compiled Python files - regenerated automatically",
        ),
        (
            ".pytest_cache",
            "python",
            "pytest Cache",
            "safe",
            "Test execution cache",
        ),
        (
            ".mypy_cache",
            "python",
            "MyPy Cache",
            "safe",
            "Type checking cache",
        ),
        (
            "pip_cache",
            "python",
            "pip Cache",
            "safe",
            "pip package cache",
        ),
        // Rust/Cargo
        (
            "target",
            "rust",
            "Cargo Target",
            "caution",
            "Build artifacts and dependencies - recompile on next build",
        ),
        (
            "cargo_registry_cache",
            "rust",
            "Cargo Registry Cache",
            "safe",
            "Downloaded crate registry data",
        ),
        // Java/Maven/Gradle
        (
            ".gradle",
            "java",
            "Gradle Cache",
            "caution",
            "Gradle build cache and dependencies",
        ),
        (
            "maven_repo",
            "java",
            "Maven Repository",
            "caution",
            "Downloaded Maven dependencies",
        ),
        // Docker
        (
            "docker_cache",
            "docker",
            "Docker Cache",
            "caution",
            "Docker build cache and layers",
        ),
        // System caches
        (
            "Caches",
            "system",
            "System Cache",
            "safe",
            "macOS system application caches",
        ),
        // IDE/editor caches
        (
            ".vscode",
            "editor",
            "VS Code Cache",
            "safe",
            "VS Code extensions and workspace cache",
        ),
        (
            ".idea",
            "editor",
            "IntelliJ IDEA Cache",
            "caution",
            "IDEA indexes and caches",
        ),
    ];

    let mut cache_map: HashMap<String, (String, String, String, Vec<CacheEntry>)> = HashMap::new();

    // Walk the directory tree looking for cache directories
    for entry in WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        for (pattern, category_id, display_name, safety, description) in &cache_patterns {
            if path_str.contains(pattern)
                || path
                    .file_name()
                    .map_or(false, |name| name.to_string_lossy() == *pattern)
            {
                // Calculate directory size
                let size_bytes = dir_size(path);
                let size_mb = size_bytes as f32 / 1_048_576.0;

                let cache_entry = CacheEntry {
                    path: path_str.to_string(),
                    size_mb,
                    cache_type: category_id.to_string(),
                    safety: safety.to_string(),
                    description: description.to_string(),
                };

                let key = format!("{}:{}", category_id, display_name);
                let (_, _, _, entries) = cache_map.entry(key).or_insert((
                    category_id.to_string(),
                    display_name.to_string(),
                    safety.to_string(),
                    Vec::new(),
                ));
                entries.push(cache_entry);
                break;
            }
        }
    }

    // Convert to result format
    let mut result: Vec<CacheCategory> = cache_map
        .into_iter()
        .map(|(key, (category_id, display_name, safety, entries))| {
            let total_size_mb: f32 = entries.iter().map(|e| e.size_mb).sum();
            CacheCategory {
                category_id,
                display_name,
                total_size_mb,
                entry_count: entries.len(),
                safety,
                entries,
            }
        })
        .collect();

    // Sort by total size (largest first)
    result.sort_by(|a, b| error::compare_f32_safe(b.total_size_mb, a.total_size_mb));

    Ok(result)
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
    use walkdir::WalkDir;

    // Validate the scan path to prevent system directory access
    let validated_path = validate_scan_path(&opts.root)?;
    log::info!("Scanning git repositories in: {}", validated_path.display());

    let mut repositories = Vec::new();

    // Find all .git directories
    for entry in WalkDir::new(&validated_path)
        .follow_links(opts.follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .filter(|e| e.file_name() == ".git")
    {
        let git_path = entry.path();
        let repo_path = git_path.parent().unwrap_or(git_path);

        let mut git_entries = Vec::new();
        let mut total_size = 0u64;

        // Analyze .git directory structure
        if let Ok(git_contents) = std::fs::read_dir(git_path) {
            for git_entry in git_contents.flatten() {
                let entry_path = git_entry.path();
                let entry_name = git_entry.file_name().to_string_lossy().into_owned();

                match entry_name.as_str() {
                    "objects" => {
                        // Analyze git objects
                        if let Ok(objects_size) = analyze_git_objects(&entry_path) {
                            total_size += objects_size;
                            git_entries.push(GitEntry {
                                path: entry_path.to_string_lossy().to_string(),
                                size_mb: objects_size as f32 / 1_048_576.0,
                                entry_type: "objects".to_string(),
                                description: format!(
                                    "Git objects: {} files",
                                    count_git_objects(&entry_path)
                                ),
                                safety: "safe".to_string(),
                                actionable: false, // Don't delete git objects
                            });
                        }
                    }
                    "refs" => {
                        // Analyze refs
                        let refs_size = dir_size(&entry_path);
                        if refs_size > 0 {
                            total_size += refs_size;
                            git_entries.push(GitEntry {
                                path: entry_path.to_string_lossy().to_string(),
                                size_mb: refs_size as f32 / 1_048_576.0,
                                entry_type: "refs".to_string(),
                                description: "Git references and branches".to_string(),
                                safety: "safe".to_string(),
                                actionable: false,
                            });
                        }
                    }
                    "logs" => {
                        // Reflogs - can be cleaned up
                        let logs_size = dir_size(&entry_path);
                        if logs_size > 0 {
                            total_size += logs_size;
                            git_entries.push(GitEntry {
                                path: entry_path.to_string_lossy().to_string(),
                                size_mb: logs_size as f32 / 1_048_576.0,
                                entry_type: "reflog".to_string(),
                                description: "Git reflogs - tracks branch movements".to_string(),
                                safety: "caution".to_string(),
                                actionable: true,
                            });
                        }
                    }
                    "pack" => {
                        // Pack files
                        let pack_size = dir_size(&entry_path);
                        if pack_size > 0 {
                            total_size += pack_size;
                            git_entries.push(GitEntry {
                                path: entry_path.to_string_lossy().to_string(),
                                size_mb: pack_size as f32 / 1_048_576.0,
                                entry_type: "pack_file".to_string(),
                                description: "Git pack files - compressed object storage"
                                    .to_string(),
                                safety: "safe".to_string(),
                                actionable: false,
                            });
                        }
                    }
                    _ => {
                        // Other files/directories
                        if let Ok(metadata) = entry_path.metadata() {
                            if metadata.is_file() {
                                let file_size = metadata.len();
                                total_size += file_size;
                                git_entries.push(GitEntry {
                                    path: entry_path.to_string_lossy().to_string(),
                                    size_mb: file_size as f32 / 1_048_576.0,
                                    entry_type: "file".to_string(),
                                    description: format!("Git file: {}", entry_name),
                                    safety: "safe".to_string(),
                                    actionable: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Check for large files in git history (using git command)
        if let Ok(large_files) = find_large_git_files(repo_path) {
            for (file_path, file_size) in large_files {
                total_size += file_size;
                git_entries.push(GitEntry {
                    path: file_path,
                    size_mb: file_size as f32 / 1_048_576.0,
                    entry_type: "large_file".to_string(),
                    description: "Large file in git history".to_string(),
                    safety: "caution".to_string(),
                    actionable: true, // Can be removed with git filter-branch or BFG
                });
            }
        }

        if !git_entries.is_empty() {
            repositories.push(GitRepository {
                repo_path: repo_path.to_string_lossy().to_string(),
                total_size_mb: total_size as f32 / 1_048_576.0,
                entry_count: git_entries.len(),
                entries: git_entries,
            });
        }
    }

    // Sort by total size (largest first)
    repositories.sort_by(|a, b| error::compare_f32_safe(b.total_size_mb, a.total_size_mb));

    Ok(repositories)
}

// Helper functions for git analysis
fn analyze_git_objects(objects_path: &std::path::Path) -> Result<u64, std::io::Error> {
    Ok(dir_size(objects_path))
}

fn count_git_objects(objects_path: &std::path::Path) -> usize {
    let mut count = 0;
    if let Ok(entries) = std::fs::read_dir(objects_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                count += count_git_objects(&path);
            } else {
                count += 1;
            }
        }
    }
    count
}

fn find_large_git_files(repo_path: &std::path::Path) -> Result<Vec<(String, u64)>, std::io::Error> {
    let mut large_files = Vec::new();

    // Use git command to find large files in history
    if let Ok(output) = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .args(&["rev-list", "--objects", "--all"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if let Some((hash, path)) = line.split_once(' ') {
                    // Get file size for this object
                    if let Ok(size_output) = std::process::Command::new("git")
                        .arg("-C")
                        .arg(repo_path)
                        .args(&["cat-file", "-s", hash])
                        .output()
                    {
                        if size_output.status.success() {
                            if let Ok(size_str) = String::from_utf8_lossy(&size_output.stdout)
                                .trim()
                                .parse::<u64>()
                            {
                                if size_str > 10 * 1024 * 1024 {
                                    // Files larger than 10MB
                                    large_files.push((path.to_string(), size_str));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(large_files)
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
            cleanup_dirs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
