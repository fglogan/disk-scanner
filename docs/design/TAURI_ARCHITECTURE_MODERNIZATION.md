# Tauri Architecture Modernization Guide

**Project:** Disk Bloat Scanner v0.2.0  
**Standard:** TES-2025 v6.9 (IPC Bridge Pattern, Section 5.2-5.3)  
**Focus:** Modularity, State Management, Extensibility, Multi-Instance Support  
**Date:** October 26, 2025

---

## Overview

This guide modernizes the Disk Bloat Scanner's Tauri architecture to achieve:

✅ **Modularity:** Clear separation of concerns, isolated command modules  
✅ **IPC Compliance:** All UI↔Core communication via Tauri IPC bridge  
✅ **State Management:** Centralized, thread-safe application state  
✅ **Extensibility:** Easy to add new panels, scanners, and commands  
✅ **Multi-Instance Ready:** Architecture supports future drag-off window support  
✅ **TES-2025 Compliant:** LAIO_Service protocol ready, clean boundaries

---

## Part 1: Backend Architecture (Rust)

### 1.1 Modular Structure

**New Directory Layout:**

```rust
src-tauri/src/
├── lib.rs                          # Entry point, command registration
├── main.rs                         # Thin app wrapper (unchanged)
├── error.rs                        # Custom error types
├── models.rs                       # All data structures
│
├── state/
│   ├── mod.rs                      # State management exports
│   ├── app.rs                      # AppState definition
│   └── store.rs                    # Persistent storage layer (future)
│
├── commands/
│   ├── mod.rs                      # Command registration
│   ├── system.rs                   # System info commands
│   │   ├── get_system_info
│   │   ├── get_disk_info
│   │   └── ...
│   ├── scan.rs                     # Scanning commands
│   │   ├── scan_bloat
│   │   ├── scan_large_files
│   │   ├── scan_duplicates
│   │   ├── scan_junk_files
│   │   ├── scan_dev_caches
│   │   └── scan_git_repos
│   ├── cleanup.rs                  # Deletion commands
│   │   ├── cleanup_dirs
│   │   └── validate_paths
│   └── analytics.rs                # Future: scan history, stats
│
├── services/
│   ├── mod.rs                      # Service exports
│   ├── scanner.rs                  # Unified scanner logic
│   │   └── ScannerService (LAIO_Service)
│   ├── cleanup.rs                  # Safe deletion logic
│   │   └── CleanupService
│   ├── git.rs                      # Git repository analysis
│   │   └── GitAnalyzer
│   └── cache.rs                    # Caching layer (future)
│
├── utils/
│   ├── mod.rs
│   ├── path.rs                     # Path validation (existing)
│   ├── patterns.rs                 # Bloat/junk pattern detection
│   ├── file.rs                     # File operations helpers
│   └── hash.rs                     # Hashing utilities
│
└── tests/
    ├── unit/
    │   ├── patterns_test.rs
    │   ├── path_test.rs
    │   └── ...
    └── integration/
        └── scan_test.rs
```

### 1.2 Error Handling (Custom Error Type)

**File: `src-tauri/src/error.rs`**

```rust
use std::fmt;
use serde::{Deserialize, Serialize};

/// Custom error type for Disk Bloat Scanner operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "error_type", content = "message")]
pub enum ScannerError {
    // Path validation errors
    InvalidPath { path: String, reason: String },
    PathTraversalAttempt { path: String },
    PermissionDenied { path: String },
    PathNotFound { path: String },
    
    // Scan operation errors
    ScanFailed { scan_type: String, reason: String },
    HashingFailed { file_path: String, reason: String },
    DirectorySizeFailed { dir_path: String, reason: String },
    
    // Cleanup errors
    CleanupFailed { path: String, reason: String },
    BatchSizeExceeded { attempted: u64, max: u64 },
    BatchCountExceeded { attempted: usize, max: usize },
    DeleteVerificationFailed { path: String },
    
    // System errors
    SystemInfoUnavailable { reason: String },
    DiskInfoUnavailable,
    
    // Concurrency errors
    MutexPoisoned { context: String },
    ChannelSendFailed { message: String },
    
    // Internal errors
    InternalError { reason: String },
    NotImplemented { feature: String },
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath { path, reason } => {
                write!(f, "Invalid path '{}': {}", path, reason)
            }
            Self::PathTraversalAttempt { path } => {
                write!(f, "Path traversal attempt detected: {}", path)
            }
            Self::PermissionDenied { path } => {
                write!(f, "Permission denied: {}", path)
            }
            // ... other variants
            _ => write!(f, "{:?}", self),
        }
    }
}

impl std::error::Error for ScannerError {}

// Conversions from standard library errors
impl From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                Self::PermissionDenied {
                    path: err.to_string(),
                }
            }
            std::io::ErrorKind::NotFound => {
                Self::PathNotFound {
                    path: err.to_string(),
                }
            }
            _ => Self::InternalError {
                reason: err.to_string(),
            },
        }
    }
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, ()>>> for ScannerError {
    fn from(err: std::sync::PoisonError<std::sync::MutexGuard<'_, ()>>) -> Self {
        Self::MutexPoisoned {
            context: err.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, ScannerError>;
```

### 1.3 Application State (Thread-Safe)

**File: `src-tauri/src/state/app.rs`**

```rust
use std::sync::Arc;
use parking_lot::RwLock;
use crate::models::*;

/// Central application state managed by Tauri
/// Uses parking_lot::RwLock for better performance than std::sync::RwLock
#[derive(Clone)]
pub struct AppState {
    /// Last scan results (cached)
    pub last_scan: Arc<RwLock<LastScanState>>,
    
    /// Scan history for analytics
    pub scan_history: Arc<RwLock<Vec<ScanHistoryEntry>>>,
    
    /// Current operation status
    pub operation: Arc<RwLock<OperationStatus>>,
    
    /// User preferences and settings
    pub settings: Arc<RwLock<UserSettings>>,
    
    /// Temporary data for current operations
    pub temp_data: Arc<RwLock<TempOperationData>>,
}

#[derive(Debug, Clone, Default)]
pub struct LastScanState {
    pub system_info: Option<SystemInfoResponse>,
    pub bloat_results: Option<Vec<BloatCategory>>,
    pub large_files: Option<Vec<LargeFileEntry>>,
    pub duplicates: Option<Vec<DuplicateSet>>,
    pub junk_files: Option<Vec<JunkCategory>>,
    pub scan_timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct OperationStatus {
    pub is_scanning: bool,
    pub current_scan_type: Option<String>,
    pub progress_percent: u32,
    pub status_message: String,
}

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub min_file_size_bytes: u64,
    pub follow_symlinks: bool,
    pub last_scan_directory: Option<String>,
    pub theme: String,  // "light" or "dark"
}

#[derive(Debug, Clone)]
pub struct TempOperationData {
    pub selected_paths: Vec<String>,
    pub pending_deletion_size: u64,
}

#[derive(Debug, Clone)]
pub struct ScanHistoryEntry {
    pub timestamp: i64,
    pub scan_type: String,
    pub directory: String,
    pub results_count: usize,
    pub total_size_bytes: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            last_scan: Arc::new(RwLock::new(LastScanState::default())),
            scan_history: Arc::new(RwLock::new(Vec::new())),
            operation: Arc::new(RwLock::new(OperationStatus {
                is_scanning: false,
                current_scan_type: None,
                progress_percent: 0,
                status_message: String::from("Ready"),
            })),
            settings: Arc::new(RwLock::new(UserSettings {
                min_file_size_bytes: 1024 * 1024 * 1024, // 1GB
                follow_symlinks: false,
                last_scan_directory: None,
                theme: String::from("light"),
            })),
            temp_data: Arc::new(RwLock::new(TempOperationData {
                selected_paths: Vec::new(),
                pending_deletion_size: 0,
            })),
        }
    }

    /// Helper: Update operation status
    pub fn set_scanning(&self, is_scanning: bool, message: impl Into<String>) {
        let mut op = self.operation.write();
        op.is_scanning = is_scanning;
        op.status_message = message.into();
    }

    /// Helper: Update scan progress
    pub fn set_progress(&self, percent: u32, message: impl Into<String>) {
        let mut op = self.operation.write();
        op.progress_percent = percent;
        op.status_message = message.into();
    }
}
```

### 1.4 Modular Commands (Organized)

**File: `src-tauri/src/commands/mod.rs`**

```rust
pub mod system;
pub mod scan;
pub mod cleanup;
pub mod analytics;  // Future

pub use system::*;
pub use scan::*;
pub use cleanup::*;

/// All available commands for Tauri command registration
pub fn get_all_handlers() -> Vec<&'static str> {
    vec![
        // System commands
        "get_system_info",
        "get_disk_info",
        
        // Scan commands
        "scan_bloat",
        "scan_large_files",
        "scan_duplicates",
        "scan_junk_files",
        "scan_dev_caches",
        "scan_git_repos",
        
        // Cleanup commands
        "cleanup_dirs",
        "validate_paths",
        
        // Future: Analytics
        // "get_scan_history",
        // "export_report",
    ]
}
```

**File: `src-tauri/src/commands/system.rs`**

```rust
use crate::error::Result;
use crate::models::{DiskInfoResponse, SystemInfoResponse};
use crate::state::AppState;
use tauri::State;

/// Get current system information
/// 
/// Returns CPU count, memory usage, disk usage, and OS details.
/// This is a read-only operation that queries system APIs.
#[tauri::command]
pub async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfoResponse> {
    use sysinfo::{Disks, System};

    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .iter()
        .max_by_key(|d| d.total_space())
        .ok_or_else(|| {
            crate::error::ScannerError::SystemInfoUnavailable {
                reason: "No disks found".to_string(),
            }
        })?;

    let disk_total_bytes = disk.total_space();
    let disk_available_bytes = disk.available_space();
    let disk_used_bytes = disk_total_bytes - disk_available_bytes;

    let info = SystemInfoResponse {
        disk_total_gb: disk_total_bytes as f32 / 1_073_741_824.0,
        disk_used_gb: disk_used_bytes as f32 / 1_073_741_824.0,
        disk_free_gb: disk_available_bytes as f32 / 1_073_741_824.0,
        disk_usage_pct: if disk_total_bytes > 0 {
            (disk_used_bytes as f32 / disk_total_bytes as f32) * 100.0
        } else {
            0.0
        },
        memory_total_gb: sys.total_memory() as f32 / 1_073_741_824.0,
        memory_used_gb: sys.used_memory() as f32 / 1_073_741_824.0,
        memory_free_gb: sys.available_memory() as f32 / 1_073_741_824.0,
        memory_usage_pct: if sys.total_memory() > 0 {
            (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
        } else {
            0.0
        },
        cpu_count: sys.cpus().len(),
        os_name: System::name()
            .unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version()
            .unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name()
            .unwrap_or_else(|| "Unknown".to_string()),
    };

    // Update state cache
    {
        let mut scan_state = state.last_scan.write();
        scan_state.system_info = Some(info.clone());
    }

    Ok(info)
}

/// Get disk information
#[tauri::command]
pub async fn get_disk_info() -> Result<DiskInfoResponse> {
    use sysinfo::Disks;

    let disks = Disks::new_with_refreshed_list();
    let disk = disks
        .iter()
        .max_by_key(|d| d.total_space())
        .ok_or(crate::error::ScannerError::DiskInfoUnavailable)?;

    let total_bytes = disk.total_space();
    let available_bytes = disk.available_space();
    let used_bytes = total_bytes - available_bytes;

    Ok(DiskInfoResponse {
        total_gb: total_bytes as f32 / 1_073_741_824.0,
        used_gb: used_bytes as f32 / 1_073_741_824.0,
        free_gb: available_bytes as f32 / 1_073_741_824.0,
        usage_pct: if total_bytes > 0 {
            (used_bytes as f32 / total_bytes as f32) * 100.0
        } else {
            0.0
        },
    })
}
```

**File: `src-tauri/src/commands/scan.rs`**

```rust
use crate::error::Result;
use crate::models::{ScanOpts, BloatCategory};
use crate::services::ScannerService;
use crate::state::AppState;
use tauri::State;

/// Scan for build artifacts and development bloat
/// 
/// Identifies directories matching known bloat patterns (node_modules, .git, target, etc.)
/// Returns categorized results with size information.
/// 
/// # Arguments
/// * `opts` - Scan options (root directory, minimum bytes, follow symlinks)
/// * `state` - Application state (for caching and progress updates)
/// 
/// # Errors
/// Returns error if path is invalid or inaccessible
#[tauri::command]
pub async fn scan_bloat(
    opts: ScanOpts,
    state: State<'_, AppState>,
) -> Result<Vec<BloatCategory>> {
    state.set_scanning(true, "Scanning for bloat...");
    
    let service = ScannerService::new();
    let results = service.scan_bloat(&opts).await?;
    
    // Cache results
    {
        let mut scan_state = state.last_scan.write();
        scan_state.bloat_results = Some(results.clone());
    }
    
    state.set_scanning(false, "Scan complete");
    Ok(results)
}

// Similar implementations for:
// - scan_large_files
// - scan_duplicates
// - scan_junk_files
// - scan_dev_caches
// - scan_git_repos
```

**File: `src-tauri/src/commands/cleanup.rs`**

```rust
use crate::error::Result;
use crate::models::{CleanupReq, CleanupResult};
use crate::services::CleanupService;
use crate::state::AppState;
use tauri::State;

/// Safely delete files and directories
/// 
/// Moves items to system trash (or permanent deletion if trash=false).
/// Respects batch limits: max 100GB per operation, max 10K files per operation.
/// 
/// # Arguments
/// * `req` - Cleanup request with paths and options
/// * `state` - Application state
/// 
/// # Errors
/// Returns error if batch exceeds limits or paths cannot be deleted
#[tauri::command]
pub async fn cleanup_dirs(
    req: CleanupReq,
    state: State<'_, AppState>,
) -> Result<CleanupResult> {
    state.set_scanning(true, "Cleaning up files...");
    
    let service = CleanupService::new();
    let result = service.cleanup(&req).await?;
    
    state.set_scanning(false, "Cleanup complete");
    Ok(result)
}

/// Validate paths before cleanup operation
/// 
/// Checks that paths are safe and accessible for deletion.
/// Prevents accidental deletion of system directories.
#[tauri::command]
pub fn validate_paths(paths: Vec<String>) -> Result<()> {
    for path in paths {
        // Use existing path validation from utils
        crate::utils::path::validate_scan_path(&path)?;
    }
    Ok(())
}
```

### 1.5 Services (Business Logic)

**File: `src-tauri/src/services/mod.rs`**

```rust
pub mod scanner;
pub mod cleanup;
pub mod git;

pub use scanner::ScannerService;
pub use cleanup::CleanupService;
pub use git::GitAnalyzer;
```

**File: `src-tauri/src/services/scanner.rs`**

```rust
use crate::error::Result;
use crate::models::{ScanOpts, BloatCategory, LargeFileEntry};
use crate::utils::patterns::detect_bloat_category;

/// Core scanner service implementing scan logic
/// 
/// Separates business logic from Tauri command handlers.
/// Can be used independently or as a LAIO_Service in future architectures.
pub struct ScannerService;

impl ScannerService {
    pub fn new() -> Self {
        Self
    }

    /// Scan for bloat using category patterns
    pub async fn scan_bloat(&self, opts: &ScanOpts) -> Result<Vec<BloatCategory>> {
        // Validate path
        let validated_path = crate::utils::path::validate_scan_path(&opts.root)?;
        
        log::info!("Scanning bloat in: {}", validated_path.display());

        // Implementation uses walkdir + rayon for parallelism
        // See existing scan_bloat command for logic
        todo!("Extract existing scan_bloat logic here")
    }

    /// Scan for large files
    pub async fn scan_large_files(&self, opts: &ScanOpts) -> Result<Vec<LargeFileEntry>> {
        let validated_path = crate::utils::path::validate_scan_path(&opts.root)?;
        
        log::info!("Scanning large files in: {}", validated_path.display());

        // Implementation uses walkdir + rayon for parallelism
        todo!("Extract existing scan_large_files logic here")
    }

    // Similar methods for other scan types...
}
```

### 1.6 Entry Point (lib.rs)

**File: `src-tauri/src/lib.rs`**

```rust
//! Disk Bloat Scanner - Desktop application for analyzing and managing disk space
//!
//! # Architecture
//! 
//! - **Commands**: IPC handlers exposed to Tauri UI (`commands/` module)
//! - **Services**: Business logic layer (`services/` module)
//! - **Utils**: Helper functions (`utils/` module)
//! - **Models**: Data structures (`models.rs`)
//! - **State**: Centralized app state (`state/` module)
//! - **Error**: Custom error types (`error.rs`)

mod error;
mod models;
mod state;
mod commands;
mod services;
mod utils;

pub use error::{ScannerError, Result};
pub use models::*;
pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::Manager;

    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        
        // Initialize application state
        .manage(app_state)
        
        // Register all command handlers
        .invoke_handler(tauri::generate_handler![
            // System commands
            commands::get_system_info,
            commands::get_disk_info,
            
            // Scan commands
            commands::scan_bloat,
            commands::scan_large_files,
            commands::scan_duplicates,
            commands::scan_junk_files,
            commands::scan_dev_caches,
            commands::scan_git_repos,
            
            // Cleanup commands
            commands::cleanup_dirs,
            commands::validate_paths,
        ])
        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Part 2: Frontend Architecture (Svelte)

### 2.1 Store-Based State Management

**File: `src/lib/stores.ts`** (Migrate from .js)

```typescript
import { writable, derived, readable } from 'svelte/store';

/**
 * Centralized application state stores
 * Using Svelte stores for reactive data flow
 */

// ============================================================================
// Data Models
// ============================================================================

export interface DiskInfo {
  total_gb: number;
  used_gb: number;
  free_gb: number;
  usage_pct: number;
}

export interface SystemInfo extends DiskInfo {
  memory_total_gb: number;
  memory_used_gb: number;
  memory_free_gb: number;
  memory_usage_pct: number;
  cpu_count: number;
  os_name: string;
  os_version: string;
  hostname: string;
}

export interface BloatEntry {
  path: string;
  size_mb: number;
}

export interface BloatCategory {
  category_id: string;
  display_name: string;
  total_size_mb: number;
  entries: BloatEntry[];
}

export interface LargeFileEntry {
  path: string;
  size_mb: number;
  last_modified: number;
}

export interface DuplicateEntry {
  path: string;
  size_mb: number;
  last_modified: number;
}

export interface DuplicateSet {
  hash: string;
  total_savable_mb: number;
  entries: DuplicateEntry[];
}

export interface JunkFileEntry {
  path: string;
  size_kb: number;
  pattern: string;
  category: string;
  safety: string;
}

export interface JunkCategory {
  category_id: string;
  display_name: string;
  total_size_kb: number;
  file_count: number;
  safety: string;
  files: JunkFileEntry[];
}

export interface OperationStatus {
  is_scanning: boolean;
  current_scan_type: string | null;
  progress_percent: number;
  status_message: string;
}

// ============================================================================
// Stores (Reactive State)
// ============================================================================

/// System information (updated periodically)
export const systemInfo = writable<SystemInfo | null>(null);

/// Disk information (updated periodically)
export const diskInfo = writable<DiskInfo | null>(null);

/// Scan results by category
export const bloatCategories = writable<BloatCategory[]>([]);
export const largeFiles = writable<LargeFileEntry[]>([]);
export const duplicates = writable<DuplicateSet[]>([]);
export const junkFiles = writable<JunkCategory[]>([]);

/// Operation status
export const operationStatus = writable<OperationStatus>({
  is_scanning: false,
  current_scan_type: null,
  progress_percent: 0,
  status_message: 'Ready',
});

/// Selected files for deletion
export const selectedPaths = writable<Set<string>>(new Set());

/// Settings
export const settings = writable({
  min_file_size_bytes: 1024 * 1024 * 1024,
  follow_symlinks: false,
  theme: 'light' as 'light' | 'dark',
});

// ============================================================================
// Derived Stores (Computed Values)
// ============================================================================

/// Summary statistics from all scans
export const scanSummary = derived(
  [bloatCategories, largeFiles, duplicates, junkFiles],
  ([$bloat, $large, $dups, $junk]) => ({
    total_bloat_mb: $bloat.reduce((sum, c) => sum + c.total_size_mb, 0),
    total_large_files_mb: $large.reduce((sum, f) => sum + f.size_mb, 0),
    total_duplicates_mb: $dups.reduce((sum, d) => sum + d.total_savable_mb, 0),
    total_junk_kb: $junk.reduce((sum, c) => sum + c.total_size_kb, 0),
    
    bloat_count: $bloat.reduce((sum, c) => sum + c.entries.length, 0),
    large_files_count: $large.length,
    duplicate_sets: $dups.length,
    junk_files_count: $junk.reduce((sum, c) => sum + c.file_count, 0),
  })
);

/// Total size to be freed (all selected files)
export const totalSelectionSize = derived(
  selectedPaths,
  ($selected) => {
    // Calculate from selected files in all categories
    let total = 0;
    // Implementation depends on where files are selected
    return total;
  }
);

/// Indicate if any selection is pending deletion
export const hasPendingDeletion = derived(
  selectedPaths,
  ($selected) => $selected.size > 0
);

// ============================================================================
// Store Helpers (DX Improvements)
// ============================================================================

export function resetAllScans() {
  bloatCategories.set([]);
  largeFiles.set([]);
  duplicates.set([]);
  junkFiles.set([]);
}

export function setScanning(scanning: boolean, message: string, scanType?: string) {
  operationStatus.update(s => ({
    ...s,
    is_scanning: scanning,
    current_scan_type: scanType || null,
    status_message: message,
  }));
}

export function setProgress(percent: number, message: string) {
  operationStatus.update(s => ({
    ...s,
    progress_percent: percent,
    status_message: message,
  }));
}

export function togglePathSelection(path: string) {
  selectedPaths.update(s => {
    const newSet = new Set(s);
    if (newSet.has(path)) {
      newSet.delete(path);
    } else {
      newSet.add(path);
    }
    return newSet;
  });
}

export function clearSelection() {
  selectedPaths.set(new Set());
}
```

### 2.2 Tauri API Service Layer

**File: `src/lib/api.ts`** (New)

```typescript
import { invoke } from '@tauri-apps/api/core';
import type {
  SystemInfo,
  DiskInfo,
  BloatCategory,
  LargeFileEntry,
  DuplicateSet,
  JunkCategory,
} from './stores';

/**
 * API Service - Encapsulates all IPC communication with Tauri backend
 * 
 * All UI↔Backend communication goes through this layer.
 * Provides type-safe wrappers around Tauri invoke() calls.
 * Makes it easy to:
 * - Add logging/analytics
 * - Implement retry logic
 * - Add request timeout handling
 * - Mock for testing
 */

interface ScanOpts {
  root: string;
  min_bytes?: number;
  follow_symlinks: boolean;
}

interface CleanupReq {
  paths: string[];
  dry_run: boolean;
  trash: boolean;
}

interface CleanupResult {
  deleted: string[];
  skipped: string[];
  errors: string[];
}

/// Invoke with type safety and error handling
async function invoke_safe<T>(
  cmd: string,
  args?: Record<string, unknown>,
  timeout_ms: number = 30000
): Promise<T> {
  try {
    console.log(`[API] Invoking: ${cmd}`, args);
    
    const result = await Promise.race([
      invoke<T>(cmd, args),
      new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error(`Timeout: ${cmd}`)), timeout_ms)
      ),
    ]);

    console.log(`[API] Success: ${cmd}`, result);
    return result;
  } catch (error) {
    console.error(`[API] Error in ${cmd}:`, error);
    throw error;
  }
}

// ============================================================================
// System Information
// ============================================================================

export async function getSystemInfo(): Promise<SystemInfo> {
  return invoke_safe<SystemInfo>('get_system_info', {}, 5000);
}

export async function getDiskInfo(): Promise<DiskInfo> {
  return invoke_safe<DiskInfo>('get_disk_info', {}, 5000);
}

// ============================================================================
// Scan Operations
// ============================================================================

export async function scanBloat(opts: ScanOpts): Promise<BloatCategory[]> {
  return invoke_safe<BloatCategory[]>('scan_bloat', { opts }, 60000);
}

export async function scanLargeFiles(opts: ScanOpts): Promise<LargeFileEntry[]> {
  return invoke_safe<LargeFileEntry[]>('scan_large_files', { opts }, 60000);
}

export async function scanDuplicates(opts: ScanOpts): Promise<DuplicateSet[]> {
  return invoke_safe<DuplicateSet[]>('scan_duplicates', { opts }, 120000);
}

export async function scanJunkFiles(opts: ScanOpts): Promise<JunkCategory[]> {
  return invoke_safe<JunkCategory[]>('scan_junk_files', { opts }, 60000);
}

export async function scanDevCaches(opts: ScanOpts): Promise<any[]> {
  return invoke_safe<any[]>('scan_dev_caches', { opts }, 60000);
}

export async function scanGitRepos(opts: ScanOpts): Promise<any[]> {
  return invoke_safe<any[]>('scan_git_repos', { opts }, 60000);
}

// ============================================================================
// Cleanup Operations
// ============================================================================

export async function validatePaths(paths: string[]): Promise<void> {
  return invoke_safe<void>('validate_paths', { paths }, 5000);
}

export async function cleanupDirs(req: CleanupReq): Promise<CleanupResult> {
  return invoke_safe<CleanupResult>('cleanup_dirs', { req }, 120000);
}

// ============================================================================
// Batch Operations
// ============================================================================

/// Execute a comprehensive scan
export async function runFullScan(opts: ScanOpts) {
  return Promise.all([
    getSystemInfo(),
    scanBloat(opts),
    scanLargeFiles(opts),
    scanDuplicates(opts),
    scanJunkFiles(opts),
  ]);
}
```

### 2.3 Component Architecture

**File: `src/lib/components/Dashboard.svelte`** (Enhanced)

```svelte
<script lang="ts">
  import { systemInfo, diskInfo, operationStatus, scanSummary } from '$lib/stores';
  import { getSystemInfo, getDiskInfo, runFullScan } from '$lib/api';
  import { onMount } from 'svelte';
  
  let selectedDirectory: string | null = null;
  
  async function startScan() {
    if (!selectedDirectory) return;
    
    try {
      // UI state is managed by stores, which are reactive
      const opts = {
        root: selectedDirectory,
        follow_symlinks: false,
      };
      
      // Scan commands update stores directly
      await runFullScan(opts);
    } catch (error) {
      console.error('Scan failed:', error);
    }
  }
  
  onMount(async () => {
    // Fetch initial system info
    try {
      const sys = await getSystemInfo();
      $systemInfo = sys;
      
      const disk = await getDiskInfo();
      $diskInfo = disk;
    } catch (error) {
      console.error('Failed to fetch system info:', error);
    }
    
    // Refresh system info every 3 seconds (only when not scanning)
    const interval = setInterval(async () => {
      if (!$operationStatus.is_scanning) {
        try {
          const disk = await getDiskInfo();
          $diskInfo = disk;
        } catch (e) {
          // Silently ignore errors in background refresh
        }
      }
    }, 3000);
    
    return () => clearInterval(interval);
  });
</script>

<div class="dashboard">
  {#if $systemInfo}
    <div class="system-info">
      <h2>System Information</h2>
      <p>OS: {$systemInfo.os_name} {$systemInfo.os_version}</p>
      <p>CPU Cores: {$systemInfo.cpu_count}</p>
      <p>Hostname: {$systemInfo.hostname}</p>
    </div>
  {/if}
  
  {#if $diskInfo}
    <div class="disk-usage">
      <h2>Disk Usage</h2>
      <div class="usage-bar">
        <div class="usage-fill" style="width: {$diskInfo.usage_pct}%"></div>
      </div>
      <p>{$diskInfo.used_gb.toFixed(1)} GB / {$diskInfo.total_gb.toFixed(1)} GB</p>
    </div>
  {/if}
  
  {#if $operationStatus.is_scanning}
    <div class="scan-status">
      <p>Scanning: {$operationStatus.current_scan_type}</p>
      <p>Progress: {$operationStatus.progress_percent}%</p>
      <p>{$operationStatus.status_message}</p>
    </div>
  {/if}
  
  <div class="scan-summary">
    <h2>Scan Summary</h2>
    <p>Bloat: {($scanSummary.total_bloat_mb / 1024).toFixed(2)} GB</p>
    <p>Large Files: {($scanSummary.total_large_files_mb / 1024).toFixed(2)} GB</p>
    <p>Duplicates: {($scanSummary.total_duplicates_mb / 1024).toFixed(2)} GB</p>
  </div>
  
  <button on:click={startScan} disabled={!selectedDirectory || $operationStatus.is_scanning}>
    Scan Now
  </button>
</div>

<style>
  /* Styling */
</style>
```

### 2.4 Extensible Component Pattern

**File: `src/lib/components/ScanResultPanel.svelte`** (Generic Panel)

```svelte
<script lang="ts">
  import type { BloatCategory, LargeFileEntry, JunkCategory } from '$lib/stores';
  
  // Generic panel that can display any scan result type
  export let title: string;
  export let items: any[] = [];
  export let onSelect: (item: any) => void = () => {};
  export let onDelete: (paths: string[]) => void = () => {};
  
  // Computed properties
  $: totalSize = items.reduce((sum, item) => sum + (item.size_mb || item.total_size_mb || 0), 0);
  $: selectedCount = items.filter(item => item.selected).length;
  
  function toggleSelection(item: any) {
    item.selected = !item.selected;
    onSelect(item);
  }
  
  function deleteSelected() {
    const selected = items.filter(item => item.selected);
    const paths = selected.map(item => item.path);
    onDelete(paths);
  }
</script>

<div class="panel">
  <div class="header">
    <h2>{title}</h2>
    <span class="info">
      {items.length} items • {(totalSize / 1024).toFixed(2)} GB
    </span>
  </div>
  
  {#if items.length === 0}
    <p class="empty">No items found</p>
  {:else}
    <div class="list">
      {#each items as item (item.path)}
        <div class="item" class:selected={item.selected}>
          <input 
            type="checkbox" 
            checked={item.selected}
            on:change={() => toggleSelection(item)}
          />
          <div class="details">
            <span class="path">{item.path}</span>
            <span class="size">{(item.size_mb || 0).toFixed(2)} MB</span>
          </div>
        </div>
      {/each}
    </div>
    
    {#if selectedCount > 0}
      <div class="actions">
        <button on:click={deleteSelected}>
          Delete {selectedCount} selected items
        </button>
      </div>
    {/if}
  {/if}
</div>

<style>
  /* Styling */
</style>
```

---

## Part 3: IPC Communication Patterns

### 3.1 Command Structure

```typescript
// All Tauri commands follow this pattern:

// Frontend
const result = await invoke<ResponseType>('command_name', {
  arg1: value1,
  arg2: value2,
});

// Backend - Commands must be:
// 1. Async (return future)
// 2. Serialize input with serde
// 3. Return Result<T, String> (automatically converted to JSON)
// 4. Register in generate_handler![] macro
```

### 3.2 Error Propagation

```rust
// Errors are automatically serialized to JSON and sent to frontend

// Example error in Rust:
return Err(ScannerError::InvalidPath {
    path: user_path,
    reason: "Contains forbidden characters".to_string(),
});

// Automatically becomes JSON in frontend:
{
  "error_type": "InvalidPath",
  "message": {
    "path": "/bad/path",
    "reason": "Contains forbidden characters"
  }
}
```

### 3.3 State Updates

```typescript
// Frontend can read from backend state, but shouldn't directly modify it
// Instead, use commands to request state changes:

// GOOD: Use command to update backend state
const result = await invoke('cleanup_dirs', { req: cleanupRequest });
// Backend updates AppState
// Frontend can poll or refetch if needed

// BAD: Trying to directly modify backend state (not possible)
// frontend code can't access backend state directly
```

---

## Part 4: Extensibility & Future Features

### 4.1 Adding New Panels

To add a new scanning panel (e.g., "Security Audit"):

**1. Backend (add command):**

```rust
// In commands/scan.rs
#[tauri::command]
pub async fn scan_security(
    opts: ScanOpts,
    state: State<'_, AppState>,
) -> Result<Vec<SecurityIssue>> {
    // Implementation
}

// In lib.rs
.invoke_handler(tauri::generate_handler![
    // ... existing
    commands::scan_security,  // NEW
])
```

**2. Frontend (add store + component):**

```typescript
// In stores.ts
export const securityIssues = writable<SecurityIssue[]>([]);

// In api.ts
export async function scanSecurity(opts: ScanOpts): Promise<SecurityIssue[]> {
  return invoke_safe<SecurityIssue[]>('scan_security', { opts });
}

// In App.svelte
import SecurityPanel from './components/SecurityPanel.svelte';
// Add tab/panel for security
```

**3. New Component:**

```svelte
<ScanResultPanel 
  title="Security Issues"
  items={$securityIssues}
  on:select={handleSelection}
  on:delete={handleDelete}
/>
```

### 4.2 Multi-Instance Support (Future)

For drag-off window support, architecture already supports:

```rust
// Each window can have independent state
pub struct WindowState {
    pub window_id: u64,
    pub scan_results: LastScanState,
    pub selected_paths: Vec<String>,
}

// Or share a central AppState (current design)
```

### 4.3 Adding Real-Time Progress

Future enhancement using Tauri events:

```rust
// Backend sends events
window.emit("scan_progress", {
    percent: 50,
    current_file: "/some/path",
});

// Frontend listens
import { listen } from '@tauri-apps/api/event';

let unlisten = await listen('scan_progress', (event) => {
    setProgress(event.payload.percent);
});
```

---

## Part 5: Testing Architecture

### 5.1 Backend Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    
    #[test]
    fn test_scanner_service() -> Result<()> {
        let service = ScannerService::new();
        let opts = ScanOpts {
            root: "/tmp".to_string(),
            min_bytes: Some(1024),
            follow_symlinks: false,
        };
        
        // Test scan logic (can be sync for testing)
        // Implementation
        Ok(())
    }
}
```

### 5.2 Frontend Tests

```typescript
import { describe, it, expect } from 'vitest';
import { get } from 'svelte/store';
import { systemInfo, resetAllScans } from '$lib/stores';

describe('Stores', () => {
  it('should reset scan results', () => {
    resetAllScans();
    expect(get(bloatCategories)).toEqual([]);
  });
});
```

---

## Part 6: Security Considerations

### 6.1 Path Validation

All paths must be validated before scanning/deletion:

```rust
// In utils/path.rs (existing)
pub fn validate_scan_path(path: &str) -> Result<PathBuf> {
    // Check for directory traversal
    // Check for system directories
    // Prevent symbolic link loops
}
```

### 6.2 IPC Security

- ✅ All commands are explicitly registered
- ✅ No arbitrary command execution
- ✅ All inputs serialized/deserialized with serde
- ✅ Tauri capabilities restrict access
- ✅ CSP headers prevent XSS

---

## Implementation Checklist (Phase 1-2)

### Phase 1: Backend Modularization
- [ ] Create error.rs with custom error type
- [ ] Create models.rs with all data structures
- [ ] Create state/ module with AppState
- [ ] Organize commands/ by domain (system, scan, cleanup)
- [ ] Create services/ with business logic
- [ ] Update lib.rs to wire everything together
- [ ] Verify `cargo clippy` passes

### Phase 2: Frontend Modernization
- [ ] Migrate stores.js → stores.ts with types
- [ ] Create api.ts service layer
- [ ] Create ScanResultPanel.svelte generic component
- [ ] Update Dashboard.svelte to use new architecture
- [ ] Update other components incrementally
- [ ] Verify `npm run check` passes

---

## Conclusion

This modernized architecture provides:

✅ **Clear separation of concerns** - Commands, services, utils, models all distinct  
✅ **Type-safe IPC** - Strongly typed Tauri commands  
✅ **Centralized state** - Single AppState with thread-safe access  
✅ **Extensible design** - Easy to add new panels and commands  
✅ **Future-proof** - Ready for multi-instance and real-time features  
✅ **TES-2025 compliant** - Follows Tauri IPC bridge pattern (Section 5.2-5.3)

---

**Document Status:** Reference Guide  
**Target Implementation:** Phase 2 (EDGS schedule)  
**Next Step:** Use as template for actual code refactoring

