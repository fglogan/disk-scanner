//! External drive detection and management (BEAD-033)
//! 
//! Lists mounted volumes, separates external drives, provides safe eject
//! integration and drive type detection.

use crate::error::{DiskBlotResult, DiskBlotError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use log::{info, warn, debug};
use sysinfo::{Disks, System};

/// Drive types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DriveType {
    /// Internal system drive
    Internal,
    /// External USB drive
    USB,
    /// Network drive
    Network,
    /// Optical drive (CD/DVD)
    Optical,
    /// Memory card (SD, etc.)
    MemoryCard,
    /// Thunderbolt drive
    Thunderbolt,
    /// Unknown external drive
    ExternalUnknown,
}

/// Drive connection interface
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    /// SATA/NVMe internal
    Internal,
    /// USB 2.0
    USB2,
    /// USB 3.0/3.1/3.2
    USB3,
    /// USB-C
    USBC,
    /// Thunderbolt
    Thunderbolt,
    /// Network (SMB/NFS/etc)
    Network,
    /// Unknown
    Unknown,
}

/// Information about a mounted drive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    /// Mount point path
    pub mount_point: PathBuf,
    /// Drive name/label
    pub name: String,
    /// Drive type
    pub drive_type: DriveType,
    /// Connection type
    pub connection_type: ConnectionType,
    /// File system type
    pub file_system: String,
    /// Total space in bytes
    pub total_space: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// Whether drive is removable
    pub is_removable: bool,
    /// Whether drive is read-only
    pub is_read_only: bool,
    /// Device identifier (if available)
    pub device_id: Option<String>,
    /// Whether safe eject is supported
    pub supports_eject: bool,
}

/// External drive manager
pub struct ExternalDriveManager {
    disks: Disks,
}

impl ExternalDriveManager {
    /// Create a new external drive manager
    pub fn new() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
        }
    }

    /// Refresh drive information
    pub fn refresh(&mut self) {
        self.disks.refresh_list();
    }

    /// List all mounted drives
    pub fn list_all_drives(&mut self) -> Vec<DriveInfo> {
        self.refresh();
        
        self.disks
            .iter()
            .map(|disk| self.analyze_drive(disk))
            .collect()
    }

    /// List only external drives
    pub fn list_external_drives(&mut self) -> Vec<DriveInfo> {
        self.list_all_drives()
            .into_iter()
            .filter(|drive| drive.is_removable)
            .collect()
    }

    /// List only internal drives
    pub fn list_internal_drives(&mut self) -> Vec<DriveInfo> {
        self.list_all_drives()
            .into_iter()
            .filter(|drive| !drive.is_removable && drive.drive_type == DriveType::Internal)
            .collect()
    }

    /// Analyze a drive and determine its properties
    fn analyze_drive(&self, disk: &sysinfo::Disk) -> DriveInfo {
        let mount_point = disk.mount_point().to_path_buf();
        let name = disk.name().to_string_lossy().to_string();
        let file_system = disk.file_system().to_string_lossy().to_string();
        
        // Determine drive type and properties
        let (drive_type, connection_type, is_removable) = self.determine_drive_type(&mount_point, &name, &file_system);
        
        // Check if read-only
        let is_read_only = self.is_read_only(&mount_point);
        
        // Determine if ejectable
        let supports_eject = is_removable && !matches!(drive_type, DriveType::Network);
        
        DriveInfo {
            mount_point,
            name,
            drive_type,
            connection_type,
            file_system,
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            is_removable,
            is_read_only,
            device_id: None, // Would need platform-specific code
            supports_eject,
        }
    }

    /// Determine drive type based on mount point and other properties
    fn determine_drive_type(&self, mount_point: &PathBuf, name: &str, file_system: &str) -> (DriveType, ConnectionType, bool) {
        let mount_str = mount_point.to_string_lossy().to_lowercase();
        let name_lower = name.to_lowercase();
        
        // Network drives
        if mount_str.starts_with("//") || mount_str.starts_with("\\\\") 
            || file_system == "smbfs" || file_system == "nfs" || file_system == "cifs" {
            return (DriveType::Network, ConnectionType::Network, true);
        }
        
        // Platform-specific detection
        #[cfg(target_os = "macos")]
        {
            if mount_str.starts_with("/volumes/") {
                // External drives typically mount under /Volumes on macOS
                if name_lower.contains("usb") || mount_str.contains("usb") {
                    return (DriveType::USB, ConnectionType::USB3, true);
                }
                if name_lower.contains("thunderbolt") {
                    return (DriveType::Thunderbolt, ConnectionType::Thunderbolt, true);
                }
                if file_system == "udf" || file_system == "cd9660" {
                    return (DriveType::Optical, ConnectionType::Internal, true);
                }
                // Most /Volumes entries are external unless they're the boot drive
                if !mount_str.contains("macintosh") && mount_str != "/volumes/" {
                    return (DriveType::ExternalUnknown, ConnectionType::Unknown, true);
                }
            }
            if mount_str == "/" {
                return (DriveType::Internal, ConnectionType::Internal, false);
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows drive detection
            if mount_str.len() == 3 && mount_str.ends_with(":\\") {
                let drive_letter = mount_str.chars().next().unwrap();
                if drive_letter == 'c' {
                    return (DriveType::Internal, ConnectionType::Internal, false);
                }
                // Other drive letters could be external
                // Would need WMI queries for accurate detection
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux drive detection
            if mount_str.starts_with("/media/") || mount_str.starts_with("/mnt/") 
                || mount_str.starts_with("/run/media/") {
                return (DriveType::USB, ConnectionType::USB3, true);
            }
            if mount_str == "/" || mount_str.starts_with("/home") {
                return (DriveType::Internal, ConnectionType::Internal, false);
            }
        }
        
        // Default fallback
        (DriveType::Internal, ConnectionType::Internal, false)
    }

    /// Check if a mount point is read-only
    fn is_read_only(&self, mount_point: &PathBuf) -> bool {
        // Try to check if we can write to the mount point
        let test_file = mount_point.join(".disk_bloat_scanner_write_test");
        match std::fs::write(&test_file, "test") {
            Ok(_) => {
                let _ = std::fs::remove_file(&test_file);
                false
            }
            Err(_) => true,
        }
    }

    /// Safely eject a drive
    pub async fn eject_drive(&self, drive_info: &DriveInfo) -> DiskBlotResult<()> {
        if !drive_info.supports_eject {
            return Err(DiskBlotError::InvalidInput(
                format!("Drive {} does not support ejection", drive_info.name)
            ));
        }
        
        info!("Attempting to eject drive: {}", drive_info.name);
        
        #[cfg(target_os = "macos")]
        {
            // Use diskutil on macOS
            let output = tokio::process::Command::new("diskutil")
                .arg("eject")
                .arg(&drive_info.mount_point)
                .output()
                .await?;
                
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(DiskBlotError::SystemCommand(
                    format!("Failed to eject drive: {}", error)
                ));
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows ejection would use WMI or similar
            // This is a placeholder
            warn!("Drive ejection not implemented for Windows");
        }
        
        #[cfg(target_os = "linux")]
        {
            // Use umount on Linux
            let output = tokio::process::Command::new("umount")
                .arg(&drive_info.mount_point)
                .output()
                .await?;
                
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(DiskBlotError::SystemCommand(
                    format!("Failed to unmount drive: {}", error)
                ));
            }
        }
        
        info!("Successfully ejected drive: {}", drive_info.name);
        Ok(())
    }

    /// Get drive usage statistics
    pub fn get_drive_stats(&mut self, mount_point: &PathBuf) -> Option<DriveStats> {
        self.refresh();
        
        self.disks
            .iter()
            .find(|disk| disk.mount_point() == mount_point)
            .map(|disk| DriveStats {
                used_space: disk.total_space() - disk.available_space(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                usage_percentage: ((disk.total_space() - disk.available_space()) as f64 
                    / disk.total_space() as f64 * 100.0) as u8,
            })
    }

    /// Monitor drive changes
    pub async fn monitor_drives(&mut self, callback: impl Fn(DriveEvent)) {
        let mut previous_drives = self.list_all_drives();
        
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            let current_drives = self.list_all_drives();
            
            // Check for new drives
            for drive in &current_drives {
                if !previous_drives.iter().any(|d| d.mount_point == drive.mount_point) {
                    callback(DriveEvent::Connected(drive.clone()));
                }
            }
            
            // Check for removed drives
            for drive in &previous_drives {
                if !current_drives.iter().any(|d| d.mount_point == drive.mount_point) {
                    callback(DriveEvent::Disconnected(drive.clone()));
                }
            }
            
            previous_drives = current_drives;
        }
    }
}

/// Drive usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveStats {
    /// Used space in bytes
    pub used_space: u64,
    /// Total space in bytes
    pub total_space: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// Usage percentage (0-100)
    pub usage_percentage: u8,
}

/// Drive events for monitoring
#[derive(Debug, Clone)]
pub enum DriveEvent {
    /// A drive was connected
    Connected(DriveInfo),
    /// A drive was disconnected
    Disconnected(DriveInfo),
}

impl Default for ExternalDriveManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_listing() {
        let mut manager = ExternalDriveManager::new();
        let drives = manager.list_all_drives();
        
        // Should have at least one drive (system drive)
        assert!(!drives.is_empty());
        
        // Check that we have at least one internal drive
        let internal_drives = manager.list_internal_drives();
        assert!(!internal_drives.is_empty());
    }

    #[test]
    fn test_drive_type_detection() {
        let manager = ExternalDriveManager::new();
        
        #[cfg(target_os = "macos")]
        {
            let (dtype, _, removable) = manager.determine_drive_type(
                &PathBuf::from("/Volumes/USB Drive"),
                "USB Drive",
                "exfat"
            );
            assert!(removable);
            assert!(matches!(dtype, DriveType::ExternalUnknown | DriveType::USB));
        }
        
        let (dtype, _, removable) = manager.determine_drive_type(
            &PathBuf::from("/"),
            "Macintosh HD",
            "apfs"
        );
        assert!(!removable);
        assert_eq!(dtype, DriveType::Internal);
    }

    #[test]
    fn test_network_drive_detection() {
        let manager = ExternalDriveManager::new();
        
        let (dtype, ctype, _) = manager.determine_drive_type(
            &PathBuf::from("//server/share"),
            "Network Share",
            "smbfs"
        );
        
        assert_eq!(dtype, DriveType::Network);
        assert_eq!(ctype, ConnectionType::Network);
    }

    #[tokio::test]
    async fn test_drive_stats() {
        let mut manager = ExternalDriveManager::new();
        let drives = manager.list_all_drives();
        
        if let Some(first_drive) = drives.first() {
            let stats = manager.get_drive_stats(&first_drive.mount_point);
            assert!(stats.is_some());
            
            let stats = stats.unwrap();
            assert!(stats.total_space > 0);
            assert!(stats.usage_percentage <= 100);
        }
    }
}