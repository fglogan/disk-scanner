/// Custom error type for disk bloat scanner operations.
///
/// Provides structured error handling with context about what went wrong,
/// eliminating panic-prone `.unwrap()` calls and providing better error messages.
use thiserror::Error;

/// Main error type for all scanner operations
#[derive(Debug, Error)]
pub enum ScannerError {
    /// File system I/O operation failed
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid path provided (e.g., protected system directory)
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Sorting operation encountered invalid floating point value (NaN/Inf)
    #[error("Cannot compare values: {0} (likely NaN or Inf)")]
    InvalidFloatComparison(String),

    /// Mutex poisoning - concurrent access error
    #[error("Concurrent access error (mutex poisoned): {0}")]
    LockPoisoned(String),

    /// Permission denied for operation
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Path does not exist
    #[error("Not found: {0}")]
    NotFound(String),

    /// Invalid configuration or parameters
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Deletion operation failed
    #[error("Deletion failed: {0}")]
    DeletionFailed(String),

    /// Generic error with context message
    #[error("{0}")]
    Other(String),
    
    /// Feature not yet implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    /// System command execution failed
    #[error("System command failed: {0}")]
    SystemCommand(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Path conversion errors
    #[error("Invalid path encoding")]
    InvalidUtf8,

    /// File access errors with path context
    #[error("Cannot access file {path}: {source}")]
    FileAccess {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Directory scan errors with path context
    #[error("Failed to scan directory {path}: {source}")]
    ScanFailed {
        path: String,
        #[source]
        source: Box<ScannerError>,
    },
}

// Backward compatibility conversions
impl From<String> for ScannerError {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for ScannerError {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<ScannerError> for String {
    fn from(err: ScannerError) -> Self {
        err.to_string()
    }
}

/// Specialized result type for scanner operations
pub type ScannerResult<T> = Result<T, ScannerError>;

/// Helper function to handle comparison of floats safely without panicking on NaN
///
/// Sorts by provided float values, treating NaN as smallest (end of sort)
#[must_use]
pub fn compare_f32_safe(a: f32, b: f32) -> std::cmp::Ordering {
    match b.partial_cmp(&a) {
        Some(order) => order,
        None => std::cmp::Ordering::Greater,
    }
}

// ============================================================================
// Retry Logic for Transient Failures (BEAD-014)
// ============================================================================

/// Configuration for retry operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,
    /// Multiplier for exponential backoff (e.g., 2.0 for double each time)
    pub backoff_multiplier: f32,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Whether to jitter the delay to avoid thundering herd
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            backoff_multiplier: 2.0,
            max_delay_ms: 10000,
            jitter: true,
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new(max_attempts: u32, initial_delay_ms: u64) -> Self {
        Self {
            max_attempts,
            initial_delay_ms,
            backoff_multiplier: 2.0,
            max_delay_ms: 10000,
            jitter: true,
        }
    }
    
    /// Set the backoff multiplier
    pub fn with_backoff_multiplier(mut self, multiplier: f32) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }
    
    /// Set the maximum delay
    pub fn with_max_delay_ms(mut self, max_delay_ms: u64) -> Self {
        self.max_delay_ms = max_delay_ms;
        self
    }
    
    /// Enable or disable jitter
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }
}

/// Check if an error is transient (might be resolved by retrying)
pub fn is_transient_error(error: &ScannerError) -> bool {
    match error {
        // I/O errors that might be temporary
        ScannerError::Io(io_err) => {
            matches!(
                io_err.kind(),
                std::io::ErrorKind::TimedOut |
                std::io::ErrorKind::ConnectionReset |
                std::io::ErrorKind::ConnectionAborted |
                std::io::ErrorKind::ConnectionRefused |
                std::io::ErrorKind::WouldBlock |
                std::io::ErrorKind::Interrupted
            )
        },
        
        // Permission denied might be temporary (e.g., file locked)
        ScannerError::PermissionDenied(_) => true,
        
        // File access errors that might be temporary
        ScannerError::FileAccess { source, .. } => {
            matches!(
                source.kind(),
                std::io::ErrorKind::TimedOut |
                std::io::ErrorKind::ConnectionReset |
                std::io::ErrorKind::ConnectionAborted |
                std::io::ErrorKind::ConnectionRefused |
                std::io::ErrorKind::WouldBlock |
                std::io::ErrorKind::Interrupted
            )
        },
        
        // Database connection issues might be temporary
        ScannerError::Database(db_err) => {
            // Check for common transient database errors
            let err_str = db_err.to_string().to_lowercase();
            err_str.contains("database is locked") ||
            err_str.contains("connection") ||
            err_str.contains("timeout") ||
            err_str.contains("busy")
        },
        
        // Don't retry other errors
        _ => false,
    }
}

/// Execute an operation with retry logic for transient failures
pub async fn retry_with_config<F, T, Fut>(
    config: RetryConfig,
    operation: F,
) -> ScannerResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ScannerResult<T>>,
{
    let mut delay = config.initial_delay_ms;
    
    for attempt in 1..=config.max_attempts {
        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    log::info!("Operation succeeded on attempt {} after retries", attempt);
                }
                return Ok(result);
            }
            Err(error) => {
                // Check if this is a transient error and we have more attempts
                if attempt < config.max_attempts && is_transient_error(&error) {
                    log::warn!(
                        "Attempt {} failed with transient error: {}. Retrying in {}ms...",
                        attempt,
                        error,
                        delay
                    );
                    
                    // Apply jitter if enabled
                    let actual_delay = if config.jitter {
                        use rand::Rng;
                        let jitter_range = delay as f32 * 0.1; // ±10% jitter
                        let mut rng = rand::thread_rng();
                        delay + rng.gen_range(-jitter_range..=jitter_range) as u64
                    } else {
                        delay
                    };
                    
                    // Cap delay at maximum
                    let capped_delay = actual_delay.min(config.max_delay_ms);
                    
                    // Sleep before retry
                    tokio::time::sleep(std::time::Duration::from_millis(capped_delay)).await;
                    
                    // Calculate next delay with exponential backoff
                    delay = (delay as f32 * config.backoff_multiplier) as u64;
                } else {
                    // Either non-transient error or no more attempts
                    if attempt > 1 {
                        log::error!("Operation failed after {} attempts: {}", attempt, error);
                    }
                    return Err(error);
                }
            }
        }
    }
    
    // This should never be reached, but just in case
    Err(ScannerError::Other("Retry logic exhausted all attempts".to_string()))
}

/// Execute an operation with default retry configuration
pub async fn retry<F, T, Fut>(operation: F) -> ScannerResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ScannerResult<T>>,
{
    retry_with_config(RetryConfig::default(), operation).await
}

/// Execute a synchronous operation with retry logic
pub fn retry_sync<F, T>(config: RetryConfig, operation: F) -> ScannerResult<T>
where
    F: Fn() -> ScannerResult<T>,
{
    let mut delay = config.initial_delay_ms;
    
    for attempt in 1..=config.max_attempts {
        match operation() {
            Ok(result) => {
                if attempt > 1 {
                    log::info!("Operation succeeded on attempt {} after retries", attempt);
                }
                return Ok(result);
            }
            Err(error) => {
                // Check if this is a transient error and we have more attempts
                if attempt < config.max_attempts && is_transient_error(&error) {
                    log::warn!(
                        "Attempt {} failed with transient error: {}. Retrying in {}ms...",
                        attempt,
                        error,
                        delay
                    );
                    
                    // Apply jitter if enabled
                    let actual_delay = if config.jitter {
                        use rand::Rng;
                        let jitter_range = delay as f32 * 0.1; // ±10% jitter
                        let mut rng = rand::thread_rng();
                        delay + rng.gen_range(-jitter_range..=jitter_range) as u64
                    } else {
                        delay
                    };
                    
                    // Cap delay at maximum
                    let capped_delay = actual_delay.min(config.max_delay_ms);
                    
                    // Sleep before retry (blocking)
                    std::thread::sleep(std::time::Duration::from_millis(capped_delay));
                    
                    // Calculate next delay with exponential backoff
                    delay = (delay as f32 * config.backoff_multiplier) as u64;
                } else {
                    // Either non-transient error or no more attempts
                    if attempt > 1 {
                        log::error!("Operation failed after {} attempts: {}", attempt, error);
                    }
                    return Err(error);
                }
            }
        }
    }
    
    // This should never be reached, but just in case
    Err(ScannerError::Other("Retry logic exhausted all attempts".to_string()))
}
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::panic,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::unnecessary_literal_unwrap,
        clippy::uninlined_format_args,
        clippy::unused_parens
    )]
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

    #[test]
    fn test_error_display_file_access() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let err = ScannerError::FileAccess {
            path: "/etc/passwd".to_string(),
            source: io_err,
        };
        let display = err.to_string();
        assert!(display.contains("Cannot access file /etc/passwd"));
    }

    // ========================================================================
    // ScannerError Debug Tests
    // ========================================================================

    #[test]
    fn test_error_debug_format() {
        let err = ScannerError::InvalidPath("/test".to_string());
        let debug = format!("{err:?}");
        assert!(debug.contains("InvalidPath"));
    }

    // ========================================================================
    // ScannerError Trait Tests
    // ========================================================================

    #[test]
    fn test_error_is_error_trait() {
        let err = ScannerError::Other("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let err = ScannerError::FileAccess {
            path: "/test".to_string(),
            source: io_err,
        };
        
        // Should have a source
        assert!(err.source().is_some());
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
    fn test_from_str() {
        let err: ScannerError = "error message".into();

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

    // ========================================================================
    // New Error Variants Tests
    // ========================================================================

    #[test]
    fn test_serialization_error() {
        let json_err = serde_json::from_str::<String>("invalid json").unwrap_err();
        let err: ScannerError = json_err.into();
        let display = err.to_string();
        assert!(display.contains("Serialization error"));
    }

    #[test]
    fn test_invalid_utf8_error() {
        let err = ScannerError::InvalidUtf8;
        assert_eq!(err.to_string(), "Invalid path encoding");
    }
}

/// Type alias for compatibility with modules using DiskBlotError
pub type DiskBlotError = ScannerError;

/// Type alias for compatibility with modules using DiskBlotResult
pub type DiskBlotResult<T> = ScannerResult<T>;