/// Enhanced scanning utilities with BEAD features integrated
use super::scan_progress::{CancellationToken, ProgressTracker, ScanProgress};
use super::symlink::SymlinkTracker;
use super::network::is_network_mount;
use crate::error;
use std::path::Path;
use std::sync::Arc;
use walkdir::{DirEntry, WalkDir};

/// Warning thresholds
const LARGE_DIRECTORY_THRESHOLD: usize = 10_000; // BEAD-010: Warn at 10K files

/// Enhanced scan options with cancellation and progress tracking
pub struct EnhancedScanOptions<'a> {
    pub root: &'a Path,
    pub follow_symlinks: bool,
    pub cancellation_token: Option<CancellationToken>,
    pub progress_tracker: Option<Arc<ProgressTracker>>,
    pub emit_progress: Option<Box<dyn Fn(ScanProgress) + Send + Sync>>,
}

/// Enhanced directory walker with all BEAD features
pub struct EnhancedWalker {
    root: std::path::PathBuf,
    follow_symlinks: bool,
    symlink_tracker: SymlinkTracker,
    cancellation_token: Option<CancellationToken>,
    progress_tracker: Option<Arc<ProgressTracker>>,
    emit_progress: Option<Box<dyn Fn(ScanProgress) + Send + Sync>>,
    warnings: Vec<String>,
    files_in_current_dir: usize,
    current_dir: Option<std::path::PathBuf>,
}

impl EnhancedWalker {
    /// Create a new enhanced walker
    pub fn new(options: EnhancedScanOptions) -> Result<Self, String> {
        // BEAD-011: Check for network mount
        if let Ok(true) = is_network_mount(options.root) {
            log::warn!(
                "Network drive detected at {}. Scanning may be slow.",
                options.root.display()
            );
        }

        Ok(Self {
            root: options.root.to_path_buf(),
            follow_symlinks: options.follow_symlinks,
            symlink_tracker: SymlinkTracker::new(),
            cancellation_token: options.cancellation_token,
            progress_tracker: options.progress_tracker,
            emit_progress: options.emit_progress,
            warnings: Vec::new(),
            files_in_current_dir: 0,
            current_dir: None,
        })
    }

    /// Get the next entry with all safety checks
    pub fn next_entry(&mut self) -> Option<Result<DirEntry, String>> {
        loop {
            // BEAD-013: Check cancellation
            if let Some(ref token) = self.cancellation_token {
                if token.is_cancelled() {
                    log::info!("Scan cancelled by user");
                    return None;
                }
            }

            let walker = WalkDir::new(&self.root)
                .follow_links(self.follow_symlinks);
            
            let entry = match walker.into_iter().next() {
                Some(Ok(entry)) => entry,
                Some(Err(e)) => {
                    log::warn!("Error walking directory: {}", e);
                    continue;
                }
                None => return None,
            };

            let path = entry.path();

            // BEAD-010: Large directory detection
            if let Some(ref parent) = path.parent() {
                if self.current_dir.as_ref() != Some(&parent.to_path_buf()) {
                    // Entering a new directory
                    if self.files_in_current_dir > LARGE_DIRECTORY_THRESHOLD {
                        let warning = format!(
                            "Large directory detected: {} contains {} files",
                            self.current_dir.as_ref()
                                .map(|p| p.to_string_lossy())
                                .unwrap_or_else(|| "unknown".into()),
                            self.files_in_current_dir
                        );
                        log::warn!("{}", warning);
                        self.warnings.push(warning);
                    }
                    self.current_dir = Some(parent.to_path_buf());
                    self.files_in_current_dir = 0;
                }
            }

            // Count files in current directory
            if entry.file_type().is_file() {
                self.files_in_current_dir += 1;
            }

            // BEAD-009: Symlink loop detection
            if entry.file_type().is_dir() {
                match self.symlink_tracker.check_path(path) {
                    Ok(true) => {
                        // Safe to traverse
                    }
                    Ok(false) => {
                        // Loop detected, skip this directory
                        let warning = format!("Symlink loop detected at: {}", path.display());
                        log::warn!("{}", warning);
                        self.warnings.push(warning.clone());
                        continue;
                    }
                    Err(e) => {
                        log::debug!("Error checking symlink at {}: {}", path.display(), e);
                        continue;
                    }
                }
            }

            // BEAD-014: Update progress
            if let Some(ref tracker) = self.progress_tracker {
                if entry.file_type().is_file() {
                    tracker.increment_files(1);
                } else if entry.file_type().is_dir() {
                    tracker.increment_dirs(1);
                }

                // Emit progress every 100 files
                if tracker.get_files_scanned() % 100 == 0 {
                    if let Some(ref emit) = self.emit_progress {
                        let progress = tracker.get_progress(
                            &path.to_string_lossy(),
                            "scanning",
                            self.warnings.clone()
                        );
                        emit(progress);
                    }
                }
            }

            return Some(Ok(entry));
        }
    }

    /// Get collected warnings
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }
}

/// Check scan path with enhanced validations
pub fn validate_scan_path_enhanced(
    path: &Path,
    cancellation_token: Option<&CancellationToken>,
) -> Result<(), String> {
    // Check cancellation
    if let Some(token) = cancellation_token {
        if token.is_cancelled() {
            return Err("Scan cancelled".to_string());
        }
    }

    // Basic path validation
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }

    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }

    // Check for network mount
    match is_network_mount(path) {
        Ok(true) => {
            log::warn!(
                "Path {} is on a network mount. Scanning may be slow.",
                path.display()
            );
        }
        Ok(false) => {}
        Err(e) => {
            log::debug!("Could not determine if path is network mount: {}", e);
        }
    }

    // Check for symlink loops
    if let Ok(has_loops) = super::symlink::has_symlink_loops(path) {
        if has_loops {
            log::warn!("Symlink loops detected in {}. Some paths may be skipped.", path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_enhanced_walker_basic() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test").unwrap();

        let options = EnhancedScanOptions {
            root: temp_dir.path(),
            follow_symlinks: false,
            cancellation_token: None,
            progress_tracker: None,
            emit_progress: None,
        };

        let mut walker = EnhancedWalker::new(options).unwrap();
        let mut count = 0;
        
        while let Some(result) = walker.next_entry() {
            if let Ok(entry) = result {
                count += 1;
                println!("Found: {}", entry.path().display());
            }
        }

        assert!(count >= 2); // At least root dir and test file
    }

    #[test]
    fn test_cancellation() {
        let temp_dir = TempDir::new().unwrap();
        let token = CancellationToken::new();
        
        let options = EnhancedScanOptions {
            root: temp_dir.path(),
            follow_symlinks: false,
            cancellation_token: Some(token.clone()),
            progress_tracker: None,
            emit_progress: None,
        };

        let mut walker = EnhancedWalker::new(options).unwrap();
        
        // Cancel immediately
        token.cancel();
        
        // Should get no entries
        assert!(walker.next_entry().is_none());
    }

    #[test]
    fn test_progress_tracking() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create some test files
        for i in 0..5 {
            fs::write(temp_dir.path().join(format!("test{}.txt", i)), "test").unwrap();
        }

        let tracker = Arc::new(ProgressTracker::new());
        let progress_received = Arc::new(std::sync::Mutex::new(false));
        let progress_flag = progress_received.clone();

        let options = EnhancedScanOptions {
            root: temp_dir.path(),
            follow_symlinks: false,
            cancellation_token: None,
            progress_tracker: Some(tracker.clone()),
            emit_progress: Some(Box::new(move |_progress| {
                *progress_flag.lock().unwrap() = true;
            })),
        };

        let mut walker = EnhancedWalker::new(options).unwrap();
        
        while walker.next_entry().is_some() {
            // Process entries
        }

        assert!(tracker.files_scanned.load(std::sync::atomic::Ordering::Relaxed) >= 5);
    }
}