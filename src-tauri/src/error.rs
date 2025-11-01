/// Custom error type for disk bloat scanner operations.
///
/// Provides structured error handling with context about what went wrong,
/// eliminating panic-prone .unwrap() calls and providing better error messages.
use std::fmt;
use std::io;

/// Main error type for all scanner operations
#[derive(Debug)]
pub enum ScannerError {
    /// File system I/O operation failed
    Io(io::Error),

    /// Invalid path provided (e.g., protected system directory)
    InvalidPath(String),

    /// Sorting operation encountered invalid floating point value (NaN/Inf)
    InvalidFloatComparison(String),

    /// Mutex poisoning - concurrent access error
    LockPoisoned(String),

    /// Permission denied for operation
    PermissionDenied(String),

    /// Path does not exist
    NotFound(String),

    /// Invalid configuration or parameters
    InvalidConfig(String),

    /// Deletion operation failed
    DeletionFailed(String),

    /// Generic error with context message
    Other(String),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::Io(err) => write!(f, "I/O error: {}", err),
            ScannerError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            ScannerError::InvalidFloatComparison(msg) => {
                write!(f, "Cannot compare values: {} (likely NaN or Inf)", msg)
            }
            ScannerError::LockPoisoned(msg) => {
                write!(f, "Concurrent access error (mutex poisoned): {}", msg)
            }
            ScannerError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ScannerError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ScannerError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            ScannerError::DeletionFailed(msg) => write!(f, "Deletion failed: {}", msg),
            ScannerError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ScannerError {}

impl From<io::Error> for ScannerError {
    fn from(err: io::Error) -> Self {
        ScannerError::Io(err)
    }
}

impl From<ScannerError> for String {
    fn from(err: ScannerError) -> Self {
        err.to_string()
    }
}

impl From<String> for ScannerError {
    fn from(msg: String) -> Self {
        ScannerError::Other(msg)
    }
}

/// Specialized result type for scanner operations
pub type ScannerResult<T> = Result<T, ScannerError>;

/// Helper function to handle comparison of floats safely without panicking on NaN
///
/// Sorts by the provided float values, treating NaN as smallest (end of sort)
pub fn compare_f32_safe(a: f32, b: f32) -> std::cmp::Ordering {
    match b.partial_cmp(&a) {
        Some(order) => order,
        None => {
            // NaN comparison returns None; place NaN values at end
            std::cmp::Ordering::Greater
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // compare_f32_safe Function Tests
    // ========================================================================

    #[test]
    fn test_compare_safe_normal() {
        assert_eq!(compare_f32_safe(1.0, 2.0), std::cmp::Ordering::Greater);
        assert_eq!(compare_f32_safe(2.0, 1.0), std::cmp::Ordering::Less);
        assert_eq!(compare_f32_safe(1.0, 1.0), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_safe_negative() {
        assert_eq!(compare_f32_safe(-1.0, -2.0), std::cmp::Ordering::Less);
        assert_eq!(compare_f32_safe(-2.0, -1.0), std::cmp::Ordering::Greater);
        assert_eq!(compare_f32_safe(-1.0, 1.0), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_compare_safe_zero() {
        assert_eq!(compare_f32_safe(0.0, 0.0), std::cmp::Ordering::Equal);
        assert_eq!(compare_f32_safe(0.0, 1.0), std::cmp::Ordering::Greater);
        assert_eq!(compare_f32_safe(1.0, 0.0), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_safe_very_large_numbers() {
        let large1 = 1e30;
        let large2 = 1e31;
        assert_eq!(
            compare_f32_safe(large1, large2),
            std::cmp::Ordering::Greater
        );
        assert_eq!(compare_f32_safe(large2, large1), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_safe_very_small_numbers() {
        let small1 = 1e-30;
        let small2 = 1e-31;
        assert_eq!(compare_f32_safe(small1, small2), std::cmp::Ordering::Less);
        assert_eq!(
            compare_f32_safe(small2, small1),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_compare_safe_nan() {
        // NaN should not panic and should sort to end
        let nan = f32::NAN;
        // When b is NaN, partial_cmp returns None → Greater
        assert_eq!(compare_f32_safe(1.0, nan), std::cmp::Ordering::Greater);
        // When a is NaN but b is not, partial_cmp still returns None → Greater
        assert_eq!(compare_f32_safe(nan, 1.0), std::cmp::Ordering::Greater);
        // Both NaN → Greater
        assert_eq!(compare_f32_safe(nan, nan), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_compare_safe_infinity() {
        let inf = f32::INFINITY;
        let neg_inf = f32::NEG_INFINITY;

        assert_eq!(compare_f32_safe(inf, 100.0), std::cmp::Ordering::Less);
        assert_eq!(compare_f32_safe(100.0, inf), std::cmp::Ordering::Greater);
        assert_eq!(
            compare_f32_safe(neg_inf, 100.0),
            std::cmp::Ordering::Greater
        );
    }

    // ========================================================================
    // ScannerError Display Tests
    // ========================================================================

    #[test]
    fn test_error_display_io() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = ScannerError::Io(io_err);
        let display = err.to_string();
        assert!(display.contains("I/O error"));
    }

    #[test]
    fn test_error_display_invalid_path() {
        let err = ScannerError::InvalidPath("/System".to_string());
        assert_eq!(err.to_string(), "Invalid path: /System");
    }

    #[test]
    fn test_error_display_invalid_float() {
        let err = ScannerError::InvalidFloatComparison("NaN encountered".to_string());
        let display = err.to_string();
        assert!(display.contains("Cannot compare values"));
        assert!(display.contains("NaN"));
    }

    #[test]
    fn test_error_display_lock_poisoned() {
        let err = ScannerError::LockPoisoned("mutex deadlock".to_string());
        let display = err.to_string();
        assert!(display.contains("Concurrent access error"));
    }

    #[test]
    fn test_error_display_permission_denied() {
        let err = ScannerError::PermissionDenied("/root/secret".to_string());
        assert_eq!(err.to_string(), "Permission denied: /root/secret");
    }

    #[test]
    fn test_error_display_not_found() {
        let err = ScannerError::NotFound("/nonexistent/path".to_string());
        assert_eq!(err.to_string(), "Not found: /nonexistent/path");
    }

    #[test]
    fn test_error_display_invalid_config() {
        let err = ScannerError::InvalidConfig("invalid setting".to_string());
        assert_eq!(err.to_string(), "Invalid configuration: invalid setting");
    }

    #[test]
    fn test_error_display_deletion_failed() {
        let err = ScannerError::DeletionFailed("cannot remove file".to_string());
        assert_eq!(err.to_string(), "Deletion failed: cannot remove file");
    }

    #[test]
    fn test_error_display_other() {
        let err = ScannerError::Other("generic error message".to_string());
        assert_eq!(err.to_string(), "generic error message");
    }

    // ========================================================================
    // ScannerError Debug Tests
    // ========================================================================

    #[test]
    fn test_error_debug_format() {
        let err = ScannerError::InvalidPath("/test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("InvalidPath"));
    }

    // ========================================================================
    // ScannerError Trait Tests
    // ========================================================================

    #[test]
    fn test_error_is_error_trait() {
        let err = ScannerError::Other("test".to_string());
        let _: &(dyn std::error::Error) = &err;
    }

    // ========================================================================
    // From Trait Implementations Tests
    // ========================================================================

    #[test]
    fn test_from_io_error() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let scanner_err: ScannerError = io_err.into();

        match scanner_err {
            ScannerError::Io(_) => {
                // Expected
            }
            _ => panic!("Should be Io error"),
        }
    }

    #[test]
    fn test_from_string() {
        let msg = "error message".to_string();
        let err: ScannerError = msg.into();

        match err {
            ScannerError::Other(m) => {
                assert_eq!(m, "error message");
            }
            _ => panic!("Should be Other error"),
        }
    }

    #[test]
    fn test_scanner_error_to_string_conversion() {
        let err = ScannerError::InvalidPath("/test".to_string());
        let s: String = err.into();
        assert_eq!(s, "Invalid path: /test");
    }

    // ========================================================================
    // ScannerResult Type Tests
    // ========================================================================

    #[test]
    fn test_scanner_result_ok() {
        let result: ScannerResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_scanner_result_err() {
        let result: ScannerResult<i32> = Err(ScannerError::Other("error".to_string()));
        assert!(result.is_err());
    }

    // ========================================================================
    // Empty and Edge Case Tests
    // ========================================================================

    #[test]
    fn test_error_empty_message() {
        let err = ScannerError::Other(String::new());
        assert_eq!(err.to_string(), "");
    }

    #[test]
    fn test_error_very_long_message() {
        let long_msg = "x".repeat(10_000);
        let err = ScannerError::InvalidPath(long_msg.clone());
        let display = err.to_string();
        assert!(display.contains(&long_msg));
    }
}
