//! Scanning logic for identifying bloat, duplicates, junk files, and developer caches.
//!
//! This module contains all core scanning algorithms:
//! - Large file detection
//! - Project bloat detection (node_modules, target, venv, etc.)
//! - Duplicate file detection (SHA-256 hashing)
//! - Junk file detection (temporary, backup, system files)
//! - Developer cache discovery
//! - Git repository analysis

use crate::models::*;
use crate::error;
use crate::utils::patterns::{detect_bloat_category, detect_junk_file, CACHE_PATTERNS};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use walkdir::WalkDir;

// ============================================================================
// Core Scanning Utilities
// ============================================================================

/// Calculate the total size of a directory recursively.
///
/// Walks the entire directory tree and sums the size of all files.
pub fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

// ============================================================================
// Large File Detection
// ============================================================================

/// Scan for large files in a directory.
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `min_bytes` - Minimum file size threshold (default: 1GB)
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Returns:** Vector of large file entries sorted by size (largest first)
pub fn scan_large_files(
    root: &Path,
    min_bytes: Option<u64>,
    follow_symlinks: bool,
) -> Result<Vec<LargeFileEntry>, String> {
    let min_size = min_bytes.unwrap_or(1024 * 1024 * 1024); // Default 1GB

    let entries: Vec<_> = WalkDir::new(root)
        .follow_links(follow_symlinks)
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

// ============================================================================
// Project Bloat Detection
// ============================================================================

/// Scan for bloated directories (node_modules, target, venv, etc.).
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Returns:** Vector of bloat categories with entries sorted by size (largest first)
pub fn scan_bloat(root: &Path, follow_symlinks: bool) -> Result<Vec<BloatCategory>, String> {
    let categories: Mutex<HashMap<String, (String, Vec<BloatEntry>)>> = Mutex::new(HashMap::new());

    // Walk the directory tree
    for entry in WalkDir::new(root)
        .follow_links(follow_symlinks)
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

// ============================================================================
// Duplicate File Detection
// ============================================================================

/// Scan for duplicate files using SHA-256 hashing.
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Algorithm:**
/// 1. First pass: collect all files with their sizes
/// 2. Group files by size (optimization - only hash files with same-size siblings)
/// 3. Hash files in parallel for performance
/// 4. Group duplicates by hash
/// 5. Calculate savable space (all copies except one)
///
/// **Returns:** Vector of duplicate sets with savable space calculated, sorted by savings (largest first)
pub fn scan_duplicates(root: &Path, follow_symlinks: bool) -> Result<Vec<DuplicateSet>, String> {
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB limit for hashing

    // First pass: collect all files with their sizes
    let files: Vec<_> = WalkDir::new(root)
        .follow_links(follow_symlinks)
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

// ============================================================================
// Junk File Detection
// ============================================================================

/// Scan for junk files (temporary, backup, system files).
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Returns:** Vector of junk file categories sorted by file count (most numerous first)
pub fn scan_junk_files(root: &Path, follow_symlinks: bool) -> Result<Vec<JunkCategory>, String> {
    let junk_files: Mutex<HashMap<String, (String, String, Vec<JunkFileEntry>)>> =
        Mutex::new(HashMap::new());

    // Walk the directory tree and find junk files
    for entry in WalkDir::new(root)
        .follow_links(follow_symlinks)
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

// ============================================================================
// Developer Cache Detection
// ============================================================================

/// Scan for developer tool caches (npm, Cargo, pip, Maven, Gradle, Docker, etc.).
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Cache Types Detected:**
/// - Node.js/npm/yarn
/// - Python/pip
/// - Rust/Cargo
/// - Java/Maven/Gradle
/// - Docker, VS Code, IntelliJ IDEA, macOS system caches
///
/// **Returns:** Vector of cache categories sorted by total size (largest first)
pub fn scan_dev_caches(root: &Path, follow_symlinks: bool) -> Result<Vec<CacheCategory>, String> {
    let mut cache_map: HashMap<String, (String, String, String, Vec<CacheEntry>)> = HashMap::new();

    // Walk the directory tree looking for cache directories
    for entry in WalkDir::new(root)
        .follow_links(follow_symlinks)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let path = entry.path();
        let path_str = path.to_string_lossy();

        for (pattern, category_id, display_name, safety, description) in CACHE_PATTERNS.iter() {
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
        .map(|(_key, (category_id, display_name, safety, entries))| {
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
// Git Repository Analysis
// ============================================================================

/// Scan for Git repositories and analyze their structure and size.
///
/// **Parameters:**
/// - `root` - Root directory path to scan
/// - `follow_symlinks` - Whether to follow symbolic links
///
/// **Analysis Includes:**
/// - Repository path and size
/// - .git directory structure (objects, refs, logs, pack files)
/// - Large files in git history (>10MB)
/// - Entry types and safety levels
///
/// **Returns:** Vector of Git repositories sorted by total size (largest first)
pub fn scan_git_repos(root: &Path, follow_symlinks: bool) -> Result<Vec<GitRepository>, String> {
    let mut repositories = Vec::new();
    let mut error_count = 0;
    
    log::info!("Starting git repository scan in: {:?} (follow_symlinks={})", root, follow_symlinks);

    // Find all .git directories - with explicit error logging
    for entry_result in WalkDir::new(root)
        .follow_links(follow_symlinks)
        .into_iter()
    {
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                error_count += 1;
                log::warn!("Error walking directory (#{} errors total): {}", error_count, err);
                continue; // Skip this entry but continue scanning
            }
        };
        
        // Only process directories named ".git"
        if !entry.file_type().is_dir() || entry.file_name() != ".git" {
            continue;
        }
        
        log::debug!("Found .git directory: {:?}", entry.path());
        
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
        // TODO: This is VERY slow - temporarily disabled for testing
        log::debug!("Skipping large file check (temporarily disabled): {:?}", repo_path);
        // if let Ok(large_files) = find_large_git_files(repo_path) {
        //     log::debug!("Found {} large files in history", large_files.len());
        //     for (file_path, file_size) in large_files {
        //         total_size += file_size;
        //         git_entries.push(GitEntry {
        //             path: file_path,
        //             size_mb: file_size as f32 / 1_048_576.0,
        //             entry_type: "large_file".to_string(),
        //             description: "Large file in git history".to_string(),
        //             safety: "caution".to_string(),
        //             actionable: true, // Can be removed with git filter-branch or BFG
        //         });
        //     }
        // }

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

    log::info!(
        "Git repository scan complete: {} repositories found, {} errors encountered", 
        repositories.len(), 
        error_count
    );
    
    Ok(repositories)
}

// ============================================================================
// Git Helper Functions
// ============================================================================

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_dir_size_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let size = dir_size(temp_dir.path());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_dir_size_with_files() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").expect("Failed to write file");

        let size = dir_size(temp_dir.path());
        assert!(size > 0);
        assert!(size >= 11); // "hello world" is 11 bytes
    }

    #[test]
    fn test_scan_large_files_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let result = scan_large_files(temp_dir.path(), Some(1024), false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
