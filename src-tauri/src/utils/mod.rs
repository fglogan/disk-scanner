//! Utility modules for the application

/// File and directory cleanup utilities with safety-first deletion operations.
pub mod cleanup;
/// Deletion history logging for audit trails and recovery.
pub mod deletion_log;
/// Path validation utilities for safe directory scanning.
pub mod path;
/// Pattern matching for junk and bloat detection.
pub mod patterns;
/// Port and network utilities for Tauri application.
pub mod port;
/// Core scanning algorithms for disk analysis.
pub mod scan;
