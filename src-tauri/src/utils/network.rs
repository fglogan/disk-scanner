/// Network drive detection utilities (BEAD-011)
use std::path::Path;
use std::process::Command;

/// Check if a path is on a network mount
pub fn is_network_mount(path: &Path) -> Result<bool, std::io::Error> {
    #[cfg(target_os = "macos")]
    {
        check_network_mount_macos(path)
    }
    
    #[cfg(target_os = "linux")]
    {
        check_network_mount_linux(path)
    }
    
    #[cfg(target_os = "windows")]
    {
        check_network_mount_windows(path)
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Ok(false) // Default to false on unsupported platforms
    }
}

#[cfg(target_os = "macos")]
fn check_network_mount_macos(path: &Path) -> Result<bool, std::io::Error> {
    // Use df command to check mount type
    let output = Command::new("df")
        .arg("-T")
        .arg(path)
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Check for common network filesystem types
        let network_fs_types = ["nfs", "smbfs", "afpfs", "webdav", "cifs"];
        
        for line in output_str.lines().skip(1) {
            // df output format: Filesystem Type Size Used Avail Use% Mounted on
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let fs_type = parts[1].to_lowercase();
                if network_fs_types.iter().any(|&nfs| fs_type.contains(nfs)) {
                    log::info!("Detected network mount type '{}' at {}", fs_type, path.display());
                    return Ok(true);
                }
            }
        }
    }

    // Also check mount command for more detailed info
    let mount_output = Command::new("mount").output()?;
    
    if mount_output.status.success() {
        let mount_str = String::from_utf8_lossy(&mount_output.stdout);
        let path_str = path.to_string_lossy();
        
        for line in mount_str.lines() {
            if line.contains(&*path_str) {
                // Check for network indicators
                if line.contains("://") || // URL-style mounts
                   line.contains("@") ||    // user@host style
                   line.contains("nfs") ||
                   line.contains("smb") ||
                   line.contains("afp") ||
                   line.contains("cifs") {
                    log::info!("Detected network mount from mount info: {}", line);
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

#[cfg(target_os = "linux")]
fn check_network_mount_linux(path: &Path) -> Result<bool, std::io::Error> {
    // Read /proc/mounts for accurate mount information
    use std::fs;
    
    let mounts = fs::read_to_string("/proc/mounts")?;
    let canonical_path = path.canonicalize()?;
    let path_str = canonical_path.to_string_lossy();
    
    // Network filesystem types on Linux
    let network_fs_types = ["nfs", "nfs4", "cifs", "smbfs", "9p", "ceph", "glusterfs"];
    
    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let mount_point = parts[1];
            let fs_type = parts[2];
            
            // Check if our path is under this mount point
            if path_str.starts_with(mount_point) {
                if network_fs_types.contains(&fs_type) {
                    log::info!("Detected network filesystem '{}' at {}", fs_type, mount_point);
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

#[cfg(target_os = "windows")]
fn check_network_mount_windows(path: &Path) -> Result<bool, std::io::Error> {
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    
    // Check if path starts with UNC path (\\server\share)
    let path_str = path.to_string_lossy();
    if path_str.starts_with("\\\\") {
        return Ok(true);
    }
    
    // Check drive type using Windows API would be more accurate
    // For now, use a simple heuristic
    if let Some(prefix) = path.to_str() {
        if prefix.len() >= 2 && prefix.chars().nth(1) == Some(':') {
            // Check if it's a mapped network drive
            let drive_letter = &prefix[0..2];
            let output = Command::new("net")
                .args(["use", drive_letter])
                .output()?;
                
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("\\\\") {
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

/// Get mount information for a path
pub struct MountInfo {
    pub filesystem: String,
    pub mount_point: String,
    pub fs_type: String,
    pub is_network: bool,
}

/// Get detailed mount information for a path
pub fn get_mount_info(path: &Path) -> Result<MountInfo, std::io::Error> {
    #[cfg(target_os = "macos")]
    {
        get_mount_info_macos(path)
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Mount info not implemented for this platform"
        ))
    }
}

#[cfg(target_os = "macos")]
fn get_mount_info_macos(path: &Path) -> Result<MountInfo, std::io::Error> {
    let output = Command::new("df")
        .arg("-T")
        .arg(path)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "df command failed"
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    
    if lines.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unexpected df output"
        ));
    }

    // Parse the output line
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 7 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Cannot parse df output"
        ));
    }

    let filesystem = parts[0].to_string();
    let fs_type = parts[1].to_string();
    let mount_point = parts[parts.len() - 1].to_string();
    let is_network = ["nfs", "smbfs", "afpfs", "webdav", "cifs"]
        .iter()
        .any(|&t| fs_type.to_lowercase().contains(t));

    Ok(MountInfo {
        filesystem,
        mount_point,
        fs_type,
        is_network,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_network_mount_local() {
        // Test with temp directory (should always be local)
        let temp_dir = std::env::temp_dir();
        match is_network_mount(&temp_dir) {
            Ok(is_network) => assert!(!is_network, "Temp dir should not be network mount"),
            Err(e) => eprintln!("Warning: Could not check network mount: {}", e),
        }
    }

    #[test]
    fn test_is_network_mount_root() {
        // Root should never be a network mount
        match is_network_mount(Path::new("/")) {
            Ok(is_network) => assert!(!is_network, "Root should not be network mount"),
            Err(e) => eprintln!("Warning: Could not check network mount: {}", e),
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_mount_info() {
        match get_mount_info(Path::new("/")) {
            Ok(info) => {
                assert!(!info.filesystem.is_empty());
                assert!(!info.mount_point.is_empty());
                assert!(!info.fs_type.is_empty());
                println!("Mount info: {:?} {} on {} (network: {})", 
                    info.filesystem, info.fs_type, info.mount_point, info.is_network);
            }
            Err(e) => eprintln!("Warning: Could not get mount info: {}", e),
        }
    }
}