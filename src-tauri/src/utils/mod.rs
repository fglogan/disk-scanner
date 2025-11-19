//! Utility modules for the application

/// File and directory cleanup utilities with safety-first deletion operations.
pub mod cleanup;
/// Deletion history logging for audit trails and recovery.
pub mod deletion_log;
/// Network drive detection utilities (BEAD-011).
pub mod network;
/// Path validation utilities for safe directory scanning.
pub mod path;
/// Pattern matching for junk and bloat detection.
pub mod patterns;
/// Port and network utilities for Tauri application.
pub mod port;
/// Core scanning algorithms for disk analysis.
pub mod scan;
/// Enhanced scanning utilities with BEAD features.
pub mod scan_enhanced;
/// Scan progress tracking with cancellation support (BEAD-013, BEAD-014).
pub mod scan_progress;
/// Symlink loop detection utilities (BEAD-009).
pub mod symlink;
/// Undo/restore functionality for deleted files (BEAD-016).
pub mod undo;
/// Custom ignore patterns for scan operations (BEAD-017).
pub mod ignore_patterns;
/// Scheduled scan functionality (BEAD-018).
pub mod scheduler;
/// Update notification and management system (BEAD-028).
pub mod updater;
/// Crash reporting with privacy-respecting implementation (BEAD-029).
pub mod crash_reporter;
/// Privacy-respecting analytics system (BEAD-030).
pub mod analytics;
/// Backup detection and handling system (BEAD-031).
pub mod backup_detection;
/// Cloud storage detection and handling (BEAD-032).
pub mod cloud_storage;
/// External drive detection and management (BEAD-033).
pub mod external_drives;
/// Compression analysis and recommendations (BEAD-034).
pub mod compression;
/// File type statistics and visualization (BEAD-035).
pub mod file_statistics;
