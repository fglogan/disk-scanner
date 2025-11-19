//! Cloud storage detection and handling system (BEAD-032)
//! 
//! Detects Dropbox, iCloud, OneDrive and other cloud storage files,
//! shows sync status and provides special deletion handling.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use log::{info, warn, debug};

/// Cloud storage providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudProvider {
    /// Apple iCloud Drive
    ICloud,
    /// Dropbox
    Dropbox,
    /// Microsoft OneDrive
    OneDrive,
    /// Google Drive
    GoogleDrive,
    /// Box
    Box,
    /// Mega
    Mega,
    /// pCloud
    PCloud,
    /// Sync.com
    Sync,
}

/// Cloud file sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    /// File is fully synced locally
    Synced,
    /// File is only in cloud (placeholder/stub file)
    CloudOnly,
    /// File is currently syncing
    Syncing,
    /// File has sync errors
    Error,
    /// Sync status unknown
    Unknown,
}

/// Information about a cloud storage file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFileInfo {
    /// Path to the file
    pub path: PathBuf,
    /// Cloud provider
    pub provider: CloudProvider,
    /// Sync status
    pub sync_status: SyncStatus,
    /// Local size (may be 0 for cloud-only files)
    pub local_size: u64,
    /// Cloud size (actual file size)
    pub cloud_size: Option<u64>,
    /// Whether file is pinned/always kept local
    pub is_pinned: bool,
    /// Warning message for deletion
    pub warning_message: String,
}

/// Cloud storage detector
pub struct CloudStorageDetector {
    /// Known cloud storage paths
    cloud_paths: Vec<(PathBuf, CloudProvider)>,
}

impl CloudStorageDetector {
    /// Create a new cloud storage detector
    pub fn new() -> Self {
        let mut cloud_paths = Vec::new();
        
        // Add user-specific cloud paths
        if let Some(home) = dirs::home_dir() {
            // iCloud Drive
            #[cfg(target_os = "macos")]
            {
                cloud_paths.push((
                    home.join("Library/Mobile Documents/com~apple~CloudDocs"),
                    CloudProvider::ICloud
                ));
            }
            
            // Dropbox
            cloud_paths.push((home.join("Dropbox"), CloudProvider::Dropbox));
            
            // OneDrive
            cloud_paths.push((home.join("OneDrive"), CloudProvider::OneDrive));
            #[cfg(target_os = "windows")]
            {
                if let Ok(onedrive) = std::env::var("OneDrive") {
                    cloud_paths.push((PathBuf::from(onedrive), CloudProvider::OneDrive));
                }
            }
            
            // Google Drive
            cloud_paths.push((home.join("Google Drive"), CloudProvider::GoogleDrive));
            cloud_paths.push((home.join("GoogleDrive"), CloudProvider::GoogleDrive));
            
            // Box
            cloud_paths.push((home.join("Box"), CloudProvider::Box));
            cloud_paths.push((home.join("Box Sync"), CloudProvider::Box));
            
            // Others
            cloud_paths.push((home.join("MEGA"), CloudProvider::Mega));
            cloud_paths.push((home.join("pCloud Drive"), CloudProvider::PCloud));
            cloud_paths.push((home.join("Sync"), CloudProvider::Sync));
        }
        
        Self { cloud_paths }
    }

    /// Check if a path is in cloud storage
    pub fn get_cloud_info(&self, path: &Path) -> Option<CloudFileInfo> {
        // Check if path is under any cloud storage location
        for (cloud_path, provider) in &self.cloud_paths {
            if path.starts_with(cloud_path) && cloud_path.exists() {
                return Some(self.analyze_cloud_file(path, provider));
            }
        }
        
        // Check for cloud-specific attributes
        if let Some(info) = self.check_cloud_attributes(path) {
            return Some(info);
        }
        
        None
    }

    /// Analyze a cloud storage file
    fn analyze_cloud_file(&self, path: &Path, provider: &CloudProvider) -> CloudFileInfo {
        let metadata = path.metadata().ok();
        let local_size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        
        let (sync_status, cloud_size, is_pinned) = match provider {
            CloudProvider::ICloud => self.check_icloud_status(path),
            CloudProvider::Dropbox => self.check_dropbox_status(path),
            CloudProvider::OneDrive => self.check_onedrive_status(path),
            CloudProvider::GoogleDrive => self.check_google_drive_status(path),
            _ => (SyncStatus::Unknown, None, false),
        };
        
        let warning_message = self.get_deletion_warning(provider, &sync_status);
        
        CloudFileInfo {
            path: path.to_path_buf(),
            provider: provider.clone(),
            sync_status,
            local_size,
            cloud_size,
            is_pinned,
            warning_message,
        }
    }

    /// Check iCloud file status
    #[cfg(target_os = "macos")]
    fn check_icloud_status(&self, path: &Path) -> (SyncStatus, Option<u64>, bool) {
        // Check for .icloud placeholder files
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if file_name.starts_with('.') && file_name.contains(".icloud") {
            return (SyncStatus::CloudOnly, None, false);
        }
        
        // Check extended attributes for iCloud status
        // In production, would use xattr crate to check com.apple.icloud attributes
        
        (SyncStatus::Synced, None, false)
    }
    
    #[cfg(not(target_os = "macos"))]
    fn check_icloud_status(&self, _path: &Path) -> (SyncStatus, Option<u64>, bool) {
        (SyncStatus::Unknown, None, false)
    }

    /// Check Dropbox file status
    fn check_dropbox_status(&self, path: &Path) -> (SyncStatus, Option<u64>, bool) {
        // Check for Dropbox sync attributes
        // .dropbox.attrs files contain sync metadata
        let attrs_path = path.parent()
            .map(|p| p.join(".dropbox.attrs"))
            .filter(|p| p.exists());
        
        if attrs_path.is_some() {
            // In production, would parse .dropbox.attrs
            return (SyncStatus::Synced, None, false);
        }
        
        // Check if file size is suspiciously small (might be placeholder)
        if let Ok(metadata) = path.metadata() {
            if metadata.len() < 1024 && path.extension().is_some() {
                return (SyncStatus::CloudOnly, None, false);
            }
        }
        
        (SyncStatus::Synced, None, false)
    }

    /// Check OneDrive file status  
    fn check_onedrive_status(&self, path: &Path) -> (SyncStatus, Option<u64>, bool) {
        // OneDrive uses FILE_ATTRIBUTE_RECALL_ON_OPEN on Windows
        #[cfg(target_os = "windows")]
        {
            // In production, would check Windows file attributes
            use std::os::windows::fs::MetadataExt;
            if let Ok(metadata) = path.metadata() {
                const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 0x00040000;
                const FILE_ATTRIBUTE_PINNED: u32 = 0x00080000;
                
                let attrs = metadata.file_attributes();
                if attrs & FILE_ATTRIBUTE_RECALL_ON_OPEN != 0 {
                    return (SyncStatus::CloudOnly, None, false);
                }
                if attrs & FILE_ATTRIBUTE_PINNED != 0 {
                    return (SyncStatus::Synced, None, true);
                }
            }
        }
        
        (SyncStatus::Synced, None, false)
    }

    /// Check Google Drive file status
    fn check_google_drive_status(&self, _path: &Path) -> (SyncStatus, Option<u64>, bool) {
        // Google Drive desktop app behavior varies by platform
        // Would need to check for .tmp.drivedownload files or similar
        (SyncStatus::Unknown, None, false)
    }

    /// Check for cloud-specific file attributes
    fn check_cloud_attributes(&self, path: &Path) -> Option<CloudFileInfo> {
        let file_name = path.file_name()?.to_str()?;
        
        // Check for cloud placeholder patterns
        if file_name.starts_with('.') && file_name.contains(".icloud") {
            return Some(CloudFileInfo {
                path: path.to_path_buf(),
                provider: CloudProvider::ICloud,
                sync_status: SyncStatus::CloudOnly,
                local_size: 0,
                cloud_size: None,
                is_pinned: false,
                warning_message: "This is an iCloud placeholder file. The actual file is stored in the cloud.".to_string(),
            });
        }
        
        None
    }

    /// Get deletion warning message
    fn get_deletion_warning(&self, provider: &CloudProvider, status: &SyncStatus) -> String {
        match status {
            SyncStatus::CloudOnly => {
                format!("This file is only stored in {}. Deleting it will remove it from the cloud permanently.", 
                    provider_name(provider))
            }
            SyncStatus::Syncing => {
                format!("This file is currently syncing with {}. Deleting it now may cause sync conflicts.", 
                    provider_name(provider))
            }
            SyncStatus::Error => {
                format!("This file has sync errors with {}. Deleting it may prevent error resolution.", 
                    provider_name(provider))
            }
            _ => {
                format!("This file is synced with {}. Deleting it will also remove it from the cloud.", 
                    provider_name(provider))
            }
        }
    }

    /// Scan directory for cloud storage files
    pub fn scan_cloud_files(&self, root: &Path) -> DiskBlotResult<Vec<CloudFileInfo>> {
        let mut cloud_files = Vec::new();
        
        for entry in walkdir::WalkDir::new(root)
            .follow_links(false)
            .max_depth(10)
        {
            match entry {
                Ok(entry) => {
                    if let Some(info) = self.get_cloud_info(entry.path()) {
                        cloud_files.push(info);
                    }
                }
                Err(e) => {
                    debug!("Error scanning {}: {}", root.display(), e);
                }
            }
        }
        
        Ok(cloud_files)
    }

    /// Check if paths can be safely deleted
    pub fn check_deletion_safety(&self, paths: &[PathBuf]) -> Vec<CloudFileInfo> {
        let mut unsafe_deletions = Vec::new();
        
        for path in paths {
            if let Some(info) = self.get_cloud_info(path) {
                // Warn about cloud-only files and syncing files
                if matches!(info.sync_status, SyncStatus::CloudOnly | SyncStatus::Syncing) {
                    unsafe_deletions.push(info);
                }
            }
        }
        
        unsafe_deletions
    }

    /// Get all detected cloud storage locations
    pub fn get_cloud_locations(&self) -> Vec<(PathBuf, CloudProvider)> {
        self.cloud_paths.iter()
            .filter(|(path, _)| path.exists())
            .cloned()
            .collect()
    }
}

/// Get human-readable provider name
fn provider_name(provider: &CloudProvider) -> &'static str {
    match provider {
        CloudProvider::ICloud => "iCloud Drive",
        CloudProvider::Dropbox => "Dropbox",
        CloudProvider::OneDrive => "OneDrive",
        CloudProvider::GoogleDrive => "Google Drive",
        CloudProvider::Box => "Box",
        CloudProvider::Mega => "MEGA",
        CloudProvider::PCloud => "pCloud",
        CloudProvider::Sync => "Sync.com",
    }
}

impl Default for CloudStorageDetector {
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
    fn test_cloud_path_detection() {
        let detector = CloudStorageDetector::new();
        
        // Test that known cloud paths are detected
        let locations = detector.get_cloud_locations();
        // Should have at least some cloud locations defined
        assert!(!detector.cloud_paths.is_empty());
    }

    #[test]
    fn test_icloud_placeholder_detection() {
        let temp_dir = TempDir::new().unwrap();
        let detector = CloudStorageDetector::new();
        
        // Create a fake iCloud placeholder file
        let placeholder = temp_dir.path().join(".test.txt.icloud");
        fs::write(&placeholder, "").unwrap();
        
        let info = detector.check_cloud_attributes(&placeholder);
        assert!(info.is_some());
        
        let info = info.unwrap();
        assert_eq!(info.provider, CloudProvider::ICloud);
        assert_eq!(info.sync_status, SyncStatus::CloudOnly);
    }

    #[test] 
    fn test_deletion_safety_check() {
        let detector = CloudStorageDetector::new();
        let temp_dir = TempDir::new().unwrap();
        
        // Create test files
        let safe_file = temp_dir.path().join("safe.txt");
        let cloud_file = temp_dir.path().join(".cloud.txt.icloud");
        
        fs::write(&safe_file, "content").unwrap();
        fs::write(&cloud_file, "").unwrap();
        
        let paths = vec![safe_file, cloud_file.clone()];
        let unsafe_files = detector.check_deletion_safety(&paths);
        
        // Should warn about the iCloud placeholder
        assert_eq!(unsafe_files.len(), 1);
        assert_eq!(unsafe_files[0].path, cloud_file);
    }

    #[test]
    fn test_provider_names() {
        assert_eq!(provider_name(&CloudProvider::ICloud), "iCloud Drive");
        assert_eq!(provider_name(&CloudProvider::Dropbox), "Dropbox");
        assert_eq!(provider_name(&CloudProvider::OneDrive), "OneDrive");
        assert_eq!(provider_name(&CloudProvider::GoogleDrive), "Google Drive");
    }
}