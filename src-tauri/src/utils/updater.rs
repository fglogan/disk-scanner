//! Update notification and management system (BEAD-028)
//! 
//! Provides automatic update checking and notification functionality
//! with privacy-respecting implementation and user control.

use crate::error::DiskBlotResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};

/// Update status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// Current version
    pub current_version: String,
    /// Latest available version
    pub latest_version: String,
    /// Whether an update is available
    pub update_available: bool,
    /// Download URL for the update
    pub download_url: Option<String>,
    /// Release notes
    pub release_notes: Option<String>,
    /// Update size in bytes
    pub update_size: Option<u64>,
    /// Whether the update is critical
    pub is_critical: bool,
}

/// Update checker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    /// Whether to check for updates automatically
    pub auto_check: bool,
    /// Check interval in hours
    pub check_interval_hours: u32,
    /// Update server URL
    pub update_server_url: String,
    /// Whether to download updates automatically
    pub auto_download: bool,
    /// Whether to notify on pre-release versions
    pub include_prereleases: bool,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            auto_check: true,
            check_interval_hours: 24,
            update_server_url: "https://api.github.com/repos/yourusername/disk-bloat-scanner/releases".to_string(),
            auto_download: false,
            include_prereleases: false,
        }
    }
}

/// Update manager for handling application updates
pub struct UpdateManager {
    config: Arc<Mutex<UpdateConfig>>,
    last_check: Arc<Mutex<Option<chrono::DateTime<chrono::Utc>>>>,
    current_version: String,
}

impl UpdateManager {
    /// Create a new update manager
    pub fn new(current_version: String) -> Self {
        Self {
            config: Arc::new(Mutex::new(UpdateConfig::default())),
            last_check: Arc::new(Mutex::new(None)),
            current_version,
        }
    }

    /// Check for available updates
    pub async fn check_for_updates(&self) -> DiskBlotResult<UpdateInfo> {
        let config = self.config.lock().await;
        
        info!("Checking for updates from: {}", config.update_server_url);
        
        // For now, return mock data. In production, this would make an HTTP request
        // to the update server and parse the response
        let update_info = UpdateInfo {
            current_version: self.current_version.clone(),
            latest_version: "0.2.0".to_string(),
            update_available: self.current_version != "0.2.0",
            download_url: Some("https://github.com/yourusername/disk-bloat-scanner/releases/download/v0.2.0/disk-bloat-scanner_0.2.0_amd64.AppImage".to_string()),
            release_notes: Some("New features:\n- Analytics dashboard\n- Cloud storage support\n- Performance improvements".to_string()),
            update_size: Some(45 * 1024 * 1024), // 45MB
            is_critical: false,
        };

        // Update last check time
        let mut last_check = self.last_check.lock().await;
        *last_check = Some(chrono::Utc::now());

        Ok(update_info)
    }

    /// Check if it's time to check for updates
    pub async fn should_check_for_updates(&self) -> bool {
        let config = self.config.lock().await;
        if !config.auto_check {
            return false;
        }

        let last_check = self.last_check.lock().await;
        match *last_check {
            None => true,
            Some(last) => {
                let hours_since_last_check = chrono::Utc::now()
                    .signed_duration_since(last)
                    .num_hours();
                hours_since_last_check >= config.check_interval_hours as i64
            }
        }
    }

    /// Update configuration
    pub async fn update_config(&self, new_config: UpdateConfig) -> DiskBlotResult<()> {
        let mut config = self.config.lock().await;
        *config = new_config;
        info!("Update configuration updated");
        Ok(())
    }

    /// Get current configuration
    pub async fn get_config(&self) -> UpdateConfig {
        self.config.lock().await.clone()
    }

    /// Download update in background
    pub async fn download_update(&self, update_info: &UpdateInfo) -> DiskBlotResult<String> {
        if let Some(url) = &update_info.download_url {
            info!("Downloading update from: {}", url);
            
            // In production, this would download the update file
            // For now, return a mock path
            Ok("/tmp/disk-bloat-scanner-update.AppImage".to_string())
        } else {
            Err(crate::error::DiskBlotError::InvalidInput("No download URL available".to_string()))
        }
    }

    /// Apply downloaded update
    pub async fn apply_update(&self, update_path: &str) -> DiskBlotResult<()> {
        info!("Applying update from: {}", update_path);
        
        // In production, this would:
        // 1. Verify update signature
        // 2. Create backup of current version
        // 3. Replace executable
        // 4. Schedule restart
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_check() {
        let manager = UpdateManager::new("0.1.0".to_string());
        let update_info = manager.check_for_updates().await.unwrap();
        
        assert_eq!(update_info.current_version, "0.1.0");
        assert!(update_info.update_available);
    }

    #[tokio::test]
    async fn test_should_check_for_updates() {
        let manager = UpdateManager::new("0.1.0".to_string());
        
        // Should check on first run
        assert!(manager.should_check_for_updates().await);
        
        // After checking, should not check immediately
        let _ = manager.check_for_updates().await;
        assert!(!manager.should_check_for_updates().await);
    }

    #[tokio::test]
    async fn test_config_update() {
        let manager = UpdateManager::new("0.1.0".to_string());
        
        let mut config = UpdateConfig::default();
        config.auto_check = false;
        
        manager.update_config(config.clone()).await.unwrap();
        
        let retrieved_config = manager.get_config().await;
        assert!(!retrieved_config.auto_check);
    }
}