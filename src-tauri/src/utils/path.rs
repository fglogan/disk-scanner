//! Path validation utilities to prevent accessing system directories

use std::path::PathBuf;

/// Validate a path for scanning operations
/// 
/// # Errors
/// Returns an error if:
/// - Path doesn't exist
/// - Path is a blocked system directory
/// - Path cannot be canonicalized
pub fn validate_scan_path(path: &str) -> Result<PathBuf, String> {
    let path_buf = PathBuf::from(path);
    
    // Try to canonicalize the path
    let canonical = path_buf
        .canonicalize()
        .map_err(|e| format!("Invalid path '{}': {}", path, e))?;
    
    // Check if path exists
    if !canonical.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // Block system-critical directories (macOS/Linux)
    let blocked_unix = [
        "/System",
        "/bin",
        "/sbin", 
        "/usr",
        "/etc",
        "/var",
        "/private/var",
        "/Library/LaunchDaemons",
        "/Library/LaunchAgents",
        "/private/etc",
        "/dev",
        "/proc",
        "/sys",
    ];
    
    // Block system-critical directories (Windows)
    let blocked_windows = [
        "C:\\Windows",
        "C:\\Program Files",
        "C:\\Program Files (x86)",
        "C:\\ProgramData\\Microsoft",
    ];
    
    // Convert canonical path to string for comparison
    let canonical_str = canonical.to_string_lossy();
    
    // Check Unix/macOS blocked paths
    for blocked_path in &blocked_unix {
        if canonical_str.starts_with(blocked_path) {
            return Err(format!(
                "Access denied: '{}' is a protected system directory",
                blocked_path
            ));
        }
    }
    
    // Check Windows blocked paths (case-insensitive)
    for blocked_path in &blocked_windows {
        if canonical_str.to_lowercase().starts_with(&blocked_path.to_lowercase()) {
            return Err(format!(
                "Access denied: '{}' is a protected system directory", 
                blocked_path
            ));
        }
    }
    
    // Warn about potentially dangerous paths but allow them
    let warning_paths = [
        "/Applications",
        "/Library",
        "C:\\Users",
    ];
    
    for warning_path in &warning_paths {
        if canonical_str.starts_with(warning_path) 
            || canonical_str.to_lowercase().starts_with(&warning_path.to_lowercase()) {
            log::warn!("Scanning potentially sensitive directory: {}", warning_path);
            break;
        }
    }
    
    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_system_directories() {
        // These should all fail
        assert!(validate_scan_path("/System").is_err());
        assert!(validate_scan_path("/bin").is_err());
        assert!(validate_scan_path("/usr").is_err());
    }
    
    #[test]
    fn test_allow_home_directory() {
        // User's home directory should be allowed
        if let Ok(home) = std::env::var("HOME") {
            assert!(validate_scan_path(&home).is_ok());
        }
    }
    
    #[test]
    fn test_nonexistent_path() {
        assert!(validate_scan_path("/nonexistent/path/that/does/not/exist").is_err());
    }
}
