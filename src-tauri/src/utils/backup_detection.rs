//! Backup detection and handling system (BEAD-031)
//! 
//! Detects Time Machine backups and other backup software files
//! to provide special handling and warnings before deletion.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use log::{info, warn, debug};

/// Backup types that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupType {
    /// macOS Time Machine backup
    TimeMachine,
    /// Windows File History
    WindowsFileHistory,
    /// Windows System Restore
    WindowsSystemRestore,
    /// Acronis backup
    Acronis,
    /// Backblaze backup
    Backblaze,
    /// CrashPlan backup
    CrashPlan,
    /// Carbonite backup
    Carbonite,
    /// Generic backup folder
    Generic,
}

/// Information about a detected backup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Path to the backup
    pub path: PathBuf,
    /// Type of backup
    pub backup_type: BackupType,
    /// Size in bytes
    pub size: u64,
    /// Last modified time
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether this is a system-critical backup
    pub is_critical: bool,
    /// Warning message for users
    pub warning_message: String,
}

/// Backup detection patterns
struct BackupPattern {
    /// Path patterns to match
    path_patterns: Vec<&'static str>,
    /// File name patterns
    file_patterns: Vec<&'static str>,
    /// Directory name patterns
    dir_patterns: Vec<&'static str>,
    /// Backup type
    backup_type: BackupType,
    /// Whether this is critical
    is_critical: bool,
}

/// Backup detector
pub struct BackupDetector {
    patterns: Vec<BackupPattern>,
}

impl BackupDetector {
    /// Create a new backup detector
    pub fn new() -> Self {
        let patterns = vec![
            // Time Machine
            BackupPattern {
                path_patterns: vec!["/Volumes/", "/.MobileBackups/", "/Backups.backupdb/"],
                file_patterns: vec![".com.apple.timemachine", ".inProgress", ".RecoverySuggestions"],
                dir_patterns: vec!["Backups.backupdb", ".MobileBackups", "MobileBackups"],
                backup_type: BackupType::TimeMachine,
                is_critical: true,
            },
            // Windows File History
            BackupPattern {
                path_patterns: vec!["FileHistory", "$OF"],
                file_patterns: vec!["Config.xml", "Data.xml"],
                dir_patterns: vec!["FileHistory", "History"],
                backup_type: BackupType::WindowsFileHistory,
                is_critical: true,
            },
            // Windows System Restore
            BackupPattern {
                path_patterns: vec!["System Volume Information", "System32/Restore"],
                file_patterns: vec!["rp*.dat", "change.log"],
                dir_patterns: vec!["System Volume Information", "_restore"],
                backup_type: BackupType::WindowsSystemRestore,
                is_critical: true,
            },
            // Acronis
            BackupPattern {
                path_patterns: vec!["Acronis"],
                file_patterns: vec![".tib", ".tibx", ".tis"],
                dir_patterns: vec!["AcronisBackups", "Acronis True Image"],
                backup_type: BackupType::Acronis,
                is_critical: false,
            },
            // Backblaze
            BackupPattern {
                path_patterns: vec!["Backblaze"],
                file_patterns: vec![".bz_done", ".bzbits", "bzdata"],
                dir_patterns: vec!["Backblaze", "bzdata"],
                backup_type: BackupType::Backblaze,
                is_critical: false,
            },
            // CrashPlan
            BackupPattern {
                path_patterns: vec!["CrashPlan"],
                file_patterns: vec!["cprestoretool.log", ".cprestoretmp"],
                dir_patterns: vec!["CrashPlan", "crashplan"],
                backup_type: BackupType::CrashPlan,
                is_critical: false,
            },
            // Carbonite
            BackupPattern {
                path_patterns: vec!["Carbonite"],
                file_patterns: vec![".cbs", ".cbf"],
                dir_patterns: vec!["Carbonite", "CarboniteBackup"],
                backup_type: BackupType::Carbonite,
                is_critical: false,
            },
            // Generic backup patterns
            BackupPattern {
                path_patterns: vec!["backup", "Backup", "BACKUP"],
                file_patterns: vec![".bak", ".backup", ".old", ".save"],
                dir_patterns: vec!["backup", "backups", "Backup", "Backups"],
                backup_type: BackupType::Generic,
                is_critical: false,
            },
        ];

        Self { patterns }
    }

    /// Check if a path is a backup
    pub fn is_backup(&self, path: &Path) -> Option<BackupInfo> {
        let path_str = path.to_string_lossy().to_lowercase();
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        for pattern in &self.patterns {
            // Check path patterns
            for path_pattern in &pattern.path_patterns {
                if path_str.contains(&path_pattern.to_lowercase()) {
                    debug!("Detected backup by path pattern: {} matches {}", path_str, path_pattern);
                    return Some(self.create_backup_info(path, &pattern.backup_type, pattern.is_critical));
                }
            }

            // Check file patterns if it's a file
            if path.is_file() {
                for file_pattern in &pattern.file_patterns {
                    if file_name.contains(file_pattern) || file_name.ends_with(file_pattern) {
                        debug!("Detected backup by file pattern: {} matches {}", file_name, file_pattern);
                        return Some(self.create_backup_info(path, &pattern.backup_type, pattern.is_critical));
                    }
                }
            }

            // Check directory patterns if it's a directory
            if path.is_dir() {
                for dir_pattern in &pattern.dir_patterns {
                    if file_name == *dir_pattern || file_name.contains(dir_pattern) {
                        debug!("Detected backup by dir pattern: {} matches {}", file_name, dir_pattern);
                        return Some(self.create_backup_info(path, &pattern.backup_type, pattern.is_critical));
                    }
                }
            }
        }

        None
    }

    /// Create backup info for a detected backup
    fn create_backup_info(&self, path: &Path, backup_type: &BackupType, is_critical: bool) -> BackupInfo {
        let metadata = path.metadata().ok();
        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let last_modified = metadata.as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0))
            .flatten();

        let warning_message = match backup_type {
            BackupType::TimeMachine => {
                "This is a Time Machine backup. Deleting it will remove your ability to restore files from this backup point. This action cannot be undone.".to_string()
            }
            BackupType::WindowsFileHistory => {
                "This is a Windows File History backup. Deleting it will remove your ability to restore previous versions of files.".to_string()
            }
            BackupType::WindowsSystemRestore => {
                "This is a Windows System Restore point. Deleting it may affect your ability to restore your system to a previous state.".to_string()
            }
            BackupType::Acronis => {
                "This is an Acronis backup file. Deleting it will remove this backup permanently.".to_string()
            }
            BackupType::Backblaze => {
                "This is a Backblaze backup file. Deleting it may affect your cloud backup status.".to_string()
            }
            BackupType::CrashPlan => {
                "This is a CrashPlan backup file. Deleting it may affect your backup restoration capabilities.".to_string()
            }
            BackupType::Carbonite => {
                "This is a Carbonite backup file. Deleting it may affect your cloud backup.".to_string()
            }
            BackupType::Generic => {
                "This appears to be a backup file or folder. Deleting it may result in permanent data loss.".to_string()
            }
        };

        BackupInfo {
            path: path.to_path_buf(),
            backup_type: backup_type.clone(),
            size,
            last_modified,
            is_critical,
            warning_message,
        }
    }

    /// Scan a directory for backups
    pub fn scan_for_backups(&self, root: &Path) -> DiskBlotResult<Vec<BackupInfo>> {
        let mut backups = Vec::new();

        if let Some(info) = self.is_backup(root) {
            backups.push(info);
        }

        // Don't recurse into Time Machine volumes or system directories
        if self.should_skip_directory(root) {
            info!("Skipping backup scan in protected directory: {}", root.display());
            return Ok(backups);
        }

        for entry in walkdir::WalkDir::new(root)
            .max_depth(5) // Limit depth to avoid deep recursion
            .follow_links(false)
        {
            match entry {
                Ok(entry) => {
                    if let Some(info) = self.is_backup(entry.path()) {
                        backups.push(info);
                    }
                }
                Err(e) => {
                    debug!("Error scanning {}: {}", root.display(), e);
                }
            }
        }

        Ok(backups)
    }

    /// Check if a directory should be skipped during scanning
    fn should_skip_directory(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        
        // Skip Time Machine volumes
        if path_str.contains("/volumes/") && path_str.contains("backup") {
            return true;
        }
        
        // Skip system directories
        if path_str.contains("system volume information") {
            return true;
        }
        
        false
    }

    /// Check if deleting a path would affect backups
    pub fn check_deletion_impact(&self, paths: &[PathBuf]) -> Vec<BackupInfo> {
        let mut affected_backups = Vec::new();
        
        for path in paths {
            if let Some(info) = self.is_backup(path) {
                affected_backups.push(info);
            }
            
            // Also check if any path is inside a backup directory
            for ancestor in path.ancestors() {
                if let Some(info) = self.is_backup(ancestor) {
                    if !affected_backups.iter().any(|b| b.path == info.path) {
                        affected_backups.push(info);
                    }
                    break;
                }
            }
        }
        
        affected_backups
    }
}

impl Default for BackupDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_time_machine_detection() {
        let detector = BackupDetector::new();
        
        let paths = vec![
            "/Volumes/Backup/Backups.backupdb",
            "/Users/test/.MobileBackups",
            "/test/.com.apple.timemachine.donotpresent",
        ];
        
        for path_str in paths {
            let path = Path::new(path_str);
            let result = detector.is_backup(path);
            assert!(result.is_some());
            assert_eq!(result.unwrap().backup_type, BackupType::TimeMachine);
        }
    }

    #[test]
    fn test_generic_backup_detection() {
        let temp_dir = TempDir::new().unwrap();
        let detector = BackupDetector::new();
        
        // Create test files
        let backup_file = temp_dir.path().join("test.bak");
        fs::write(&backup_file, "backup content").unwrap();
        
        let backup_dir = temp_dir.path().join("backups");
        fs::create_dir(&backup_dir).unwrap();
        
        // Test file detection
        let result = detector.is_backup(&backup_file);
        assert!(result.is_some());
        assert_eq!(result.unwrap().backup_type, BackupType::Generic);
        
        // Test directory detection
        let result = detector.is_backup(&backup_dir);
        assert!(result.is_some());
        assert_eq!(result.unwrap().backup_type, BackupType::Generic);
    }

    #[test]
    fn test_non_backup_detection() {
        let detector = BackupDetector::new();
        
        let paths = vec![
            "/Users/test/Documents/report.pdf",
            "/tmp/cache",
            "/var/log/system.log",
        ];
        
        for path_str in paths {
            let path = Path::new(path_str);
            let result = detector.is_backup(path);
            assert!(result.is_none());
        }
    }

    #[test]
    fn test_deletion_impact_check() {
        let detector = BackupDetector::new();
        
        let paths = vec![
            PathBuf::from("/Volumes/Backup/Backups.backupdb/test.txt"),
            PathBuf::from("/Users/test/Documents/file.txt"),
            PathBuf::from("/Users/test/backup/data.bak"),
        ];
        
        let impacts = detector.check_deletion_impact(&paths);
        assert!(!impacts.is_empty());
        
        // Should detect at least the backup directory and .bak file
        assert!(impacts.iter().any(|i| i.backup_type == BackupType::TimeMachine || i.backup_type == BackupType::Generic));
    }
}