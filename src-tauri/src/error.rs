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

    #[test]
    fn test_compare_safe_normal() {
        assert_eq!(compare_f32_safe(1.0, 2.0), std::cmp::Ordering::Greater);
        assert_eq!(compare_f32_safe(2.0, 1.0), std::cmp::Ordering::Less);
        assert_eq!(compare_f32_safe(1.0, 1.0), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_compare_safe_nan() {
        // NaN should not panic and should sort to end
        let nan = f32::NAN;
        assert_eq!(compare_f32_safe(1.0, nan), std::cmp::Ordering::Less);
        assert_eq!(compare_f32_safe(nan, 1.0), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_error_display() {
        let err = ScannerError::InvalidPath("/System".to_string());
        assert_eq!(err.to_string(), "Invalid path: /System");
    }
}
