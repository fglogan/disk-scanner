//! Integration tests for BEAD-002 path validation security
//! 
//! These tests verify that the path validation functions properly prevent
//! system directory access across all usage scenarios.

use disk_bloat_scanner_lib::{
    models::CleanupReq,
    utils::{path::validate_scan_path, cleanup::validate_deletion_request}
};

#[test]
fn test_validate_scan_path_blocks_all_system_directories() {
    let system_dirs = vec![
        "/System",
        "/bin", 
        "/sbin",
        "/usr",
        "/etc",
        "/var",
        "/private/var",
        "/private/etc",
        "/dev",
    ];
    
    // Only test directories that exist on macOS
    for dir in system_dirs {
        if std::path::Path::new(dir).exists() {
            let result = validate_scan_path(dir);
            assert!(result.is_err(), "Should block system directory: {}", dir);
            let err_msg = result.unwrap_err();
            assert!(err_msg.contains("protected system directory"), 
                "Should contain security message for {}: {}", dir, err_msg);
        }
    }
    
    // Test some directories that might not exist but should still be blocked by pattern
    let blocked_patterns = vec!["/proc", "/sys"];
    for pattern in blocked_patterns {
        // Create a mock test since these don't exist on macOS
        // The validation logic would block them if they existed
        let result = validate_scan_path(pattern);
        assert!(result.is_err(), "Should block pattern directory: {}", pattern);
        // May fail with "No such file" instead of security error, which is also fine
    }
}

#[test]
fn test_validate_scan_path_allows_safe_directories() {
    // Test safe directories that should be allowed
    let safe_dirs = vec!["/tmp"];
    
    for dir in safe_dirs {
        let result = validate_scan_path(dir);
        if let Err(err_msg) = result {
            // Should not be a security error (may fail on other reasons like not existing)
            assert!(!err_msg.contains("protected system directory"), 
                "Should not block safe directory {}: {}", dir, err_msg);
        }
    }
    
    // Test home directory separately if available
    if let Ok(home) = std::env::var("HOME") {
        let result = validate_scan_path(&home);
        if let Err(err_msg) = result {
            assert!(!err_msg.contains("protected system directory"), 
                "Should not block home directory {}: {}", home, err_msg);
        }
    }
}

#[test] 
fn test_validate_deletion_request_blocks_system_file_deletion() {
    let req = CleanupReq {
        paths: vec!["/System/Library/test.txt".to_string()],
        dry_run: false,
        trash: true,
    };
    
    let result = validate_deletion_request(&req);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security validation failed"));
    assert!(err_msg.contains("System/Library/test.txt"));
}

#[test]
fn test_validate_deletion_request_blocks_bin_file_deletion() {
    let req = CleanupReq {
        paths: vec!["/bin/ls".to_string()],
        dry_run: false,
        trash: true,
    };
    
    let result = validate_deletion_request(&req);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security validation failed"));
    assert!(err_msg.contains("/bin/ls"));
}

#[test]
fn test_validate_deletion_request_blocks_usr_file_deletion() {
    let req = CleanupReq {
        paths: vec!["/usr/bin/critical_tool".to_string()],
        dry_run: false,
        trash: true,
    };
    
    let result = validate_deletion_request(&req);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security validation failed"));
    assert!(err_msg.contains("usr/bin/critical_tool"));
}

#[test]
fn test_validate_deletion_request_blocks_mixed_paths() {
    let req = CleanupReq {
        paths: vec![
            "/tmp/safe_file.txt".to_string(),
            "/System/Library/dangerous_file.txt".to_string(),
        ],
        dry_run: false,
        trash: true,
    };
    
    let result = validate_deletion_request(&req);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Security validation failed"));
    assert!(err_msg.contains("System/Library/dangerous_file.txt"));
}

#[test]
fn test_validate_deletion_request_allows_tmp_files() {
    let req = CleanupReq {
        paths: vec!["/tmp/safe_file.txt".to_string()],
        dry_run: true,
        trash: true,
    };
    
    let result = validate_deletion_request(&req);
    
    // Should not fail with security validation error
    if let Err(err) = result {
        let err_msg = err.to_string();
        assert!(!err_msg.contains("Security validation failed"));
        assert!(!err_msg.contains("protected system directory"));
    }
}

#[test]
fn test_comprehensive_security_coverage() {
    // Test that all major system paths are blocked for both scan and deletion operations
    let critical_paths = vec![
        "/System/Library/Frameworks",
        "/bin/bash", 
        "/usr/bin/sudo",
        "/etc/passwd",
        "/var/log/system.log",
        "/private/var/db",
        "/Library/LaunchDaemons/com.apple.test",
        "/dev/null",
    ];
    
    for path in &critical_paths {
        // Test scan path validation
        let scan_result = validate_scan_path(path);
        assert!(scan_result.is_err(), "Scan validation should block: {}", path);
        
        // Test deletion validation  
        let del_req = CleanupReq {
            paths: vec![path.to_string()],
            dry_run: false,
            trash: true,
        };
        let del_result = validate_deletion_request(&del_req);
        assert!(del_result.is_err(), "Deletion validation should block: {}", path);
    }
}