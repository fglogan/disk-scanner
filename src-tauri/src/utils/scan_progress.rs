/// Scan progress tracking with ETA calculation (BEAD-014)
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cancellation token for scan operations (BEAD-013)
#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a new cancellation token
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Cancel the operation
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Check if the operation is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

/// Progress tracking for scan operations with ETA calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    /// Current file/directory being scanned
    pub current_path: String,
    /// Number of files scanned so far
    pub files_scanned: u64,
    /// Number of directories scanned so far
    pub dirs_scanned: u64,
    /// Total bytes processed
    pub bytes_processed: u64,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<u64>,
    /// Progress percentage (0-100)
    pub percentage: f32,
    /// Current scan phase
    pub phase: String,
    /// Warnings collected during scan
    pub warnings: Vec<String>,
}

/// Thread-safe progress tracker
pub struct ProgressTracker {
    start_time: Instant,
    files_scanned: Arc<AtomicU64>,
    dirs_scanned: Arc<AtomicU64>,
    bytes_processed: Arc<AtomicU64>,
    total_estimated: Arc<AtomicU64>,
}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            files_scanned: Arc::new(AtomicU64::new(0)),
            dirs_scanned: Arc::new(AtomicU64::new(0)),
            bytes_processed: Arc::new(AtomicU64::new(0)),
            total_estimated: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Update file count
    pub fn increment_files(&self, count: u64) {
        self.files_scanned.fetch_add(count, Ordering::Relaxed);
    }

    /// Update directory count
    pub fn increment_dirs(&self, count: u64) {
        self.dirs_scanned.fetch_add(count, Ordering::Relaxed);
    }

    /// Update bytes processed
    pub fn add_bytes(&self, bytes: u64) {
        self.bytes_processed.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Set total estimated size (for percentage calculation)
    pub fn set_total_estimate(&self, total: u64) {
        self.total_estimated.store(total, Ordering::Relaxed);
    }

    /// Get current files scanned count
    pub fn get_files_scanned(&self) -> u64 {
        self.files_scanned.load(Ordering::Relaxed)
    }

    /// Calculate current progress with ETA
    pub fn get_progress(&self, current_path: &str, phase: &str, warnings: Vec<String>) -> ScanProgress {
        let files = self.files_scanned.load(Ordering::Relaxed);
        let dirs = self.dirs_scanned.load(Ordering::Relaxed);
        let bytes = self.bytes_processed.load(Ordering::Relaxed);
        let total = self.total_estimated.load(Ordering::Relaxed);
        
        let elapsed = self.start_time.elapsed();
        
        // Calculate percentage
        let percentage = if total > 0 {
            (bytes as f32 / total as f32 * 100.0).min(100.0)
        } else if files > 0 {
            // Fallback: estimate based on typical file processing rate
            ((files + dirs) as f32 / 1000.0).min(100.0)
        } else {
            0.0
        };

        // Calculate ETA
        let eta_seconds = if percentage > 0.0 && percentage < 100.0 {
            let elapsed_secs = elapsed.as_secs_f64();
            let rate = percentage as f64 / elapsed_secs;
            let remaining = (100.0 - percentage as f64) / rate;
            Some(remaining as u64)
        } else {
            None
        };

        ScanProgress {
            current_path: current_path.to_string(),
            files_scanned: files,
            dirs_scanned: dirs,
            bytes_processed: bytes,
            eta_seconds,
            percentage,
            phase: phase.to_string(),
            warnings,
        }
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Format duration as human-readable string
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let secs = seconds % 60;
        format!("{minutes}m {secs}s")
    } else {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{hours}h {minutes}m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
        
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_progress_tracker() {
        let tracker = ProgressTracker::new();
        
        tracker.increment_files(10);
        tracker.increment_dirs(5);
        tracker.add_bytes(1024 * 1024);
        tracker.set_total_estimate(10 * 1024 * 1024);
        
        thread::sleep(Duration::from_millis(100));
        
        let progress = tracker.get_progress("/test/path", "scanning", vec![]);
        assert_eq!(progress.files_scanned, 10);
        assert_eq!(progress.dirs_scanned, 5);
        assert_eq!(progress.bytes_processed, 1024 * 1024);
        assert!(progress.percentage > 0.0);
        assert!(progress.eta_seconds.is_some());
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3661), "1h 1m");
    }
}