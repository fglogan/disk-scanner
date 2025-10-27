//! Utility modules for the application

/// File and directory cleanup utilities with safety-first deletion operations.
pub mod cleanup;
/// Path validation utilities for safe directory scanning.
pub mod path;
/// Port and network utilities for Tauri application.
pub mod port;
/// Pattern matching for junk and bloat detection.
pub mod patterns;
/// Core scanning algorithms for disk analysis.
pub mod scan;
