use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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
// Tauri Commands
// ============================================================================

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

#[tauri::command]
async fn scan_large_files(opts: ScanOpts) -> Result<Vec<LargeFileEntry>, String> {
    use rayon::prelude::*;
    use walkdir::WalkDir;

    let min_size = opts.min_bytes.unwrap_or(1024 * 1024 * 1024); // Default 1GB

    let entries: Vec<_> = WalkDir::new(&opts.root)
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
    sorted.sort_by(|a, b| b.size_mb.partial_cmp(&a.size_mb).unwrap());

    Ok(sorted)
}

#[tauri::command]
async fn scan_bloat(opts: ScanOpts) -> Result<Vec<BloatCategory>, String> {
    use std::sync::Mutex;
    use walkdir::WalkDir;

    let categories: Mutex<HashMap<String, (String, Vec<BloatEntry>)>> = Mutex::new(HashMap::new());

    // Walk the directory tree
    for entry in WalkDir::new(&opts.root)
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
                    let mut cats = categories.lock().unwrap();
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
        .unwrap()
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
    result.sort_by(|a, b| b.total_size_mb.partial_cmp(&a.total_size_mb).unwrap());

    Ok(result)
}

#[tauri::command]
async fn scan_duplicates(opts: ScanOpts) -> Result<Vec<DuplicateSet>, String> {
    use rayon::prelude::*;
    use sha2::{Digest, Sha256};
    use std::sync::Mutex;
    use walkdir::WalkDir;

    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB limit for hashing

    // First pass: collect all files with their sizes
    let files: Vec<_> = WalkDir::new(&opts.root)
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

                        let mut hashes = file_hashes.lock().unwrap();
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
        .unwrap()
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
    result.sort_by(|a, b| b.total_savable_mb.partial_cmp(&a.total_savable_mb).unwrap());

    Ok(result)
}

#[tauri::command]
async fn cleanup_dirs(req: CleanupReq) -> Result<CleanupResult, String> {
    let mut deleted = Vec::new();
    let mut skipped = Vec::new();
    let mut errors = Vec::new();

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

        if !p.exists() {
            skipped.push(path.clone());
            continue;
        }

        if req.trash {
            // Move to trash
            match trash::delete(p) {
                Ok(_) => deleted.push(path.clone()),
                Err(e) => errors.push(format!("{}: {}", path, e)),
            }
        } else {
            // Permanent deletion
            let result = if p.is_dir() {
                std::fs::remove_dir_all(p)
            } else {
                std::fs::remove_file(p)
            };

            match result {
                Ok(_) => deleted.push(path.clone()),
                Err(e) => errors.push(format!("{}: {}", path, e)),
            }
        }
    }

    Ok(CleanupResult {
        deleted,
        skipped,
        errors,
    })
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
            cleanup_dirs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
