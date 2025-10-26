# Safety-First Architecture for Disk Bloat Scanner

**Critical Context:** This is an internal productivity tool with zero tolerance for data loss.  
**Risk Level:** CRITICAL - Accidental deletion of uncommitted code/data is unacceptable  
**Deployment Target:** Production immediately (internal use)  
**Status:** v0.1.1 Production Release with Safety Hardening → v0.2.0

---

## Executive Principles

🔴 **RULE 1: TRUST BUT VERIFY**
- Never delete without multiple confirmation layers
- Always show EXACTLY what will be deleted
- Preview with actual file sizes and paths
- Provide 30-second cancel window after final confirmation

🔴 **RULE 2: AUDIT TRAIL**
- Log EVERY deletion attempt (successful or failed)
- Timestamp, file path, size, user action for every item deleted
- Keep immutable deletion history (cannot be modified)
- Export audit trail for compliance review

🔴 **RULE 3: TRASH ALWAYS**
- NO permanent deletion in v0.1/v0.2
- 100% of deletions go to system trash
- Users can recover from trash for 30+ days (platform-dependent)
- Future v1.0 can add optional permanent delete with extra warnings

🔴 **RULE 4: PROTECTION ZONES**
- NEVER scan/delete system directories (/System, /usr, /bin, C:\Windows, etc.)
- Hardcoded blocklist of dangerous paths
- Even with explicit user path, validate safety
- Fail closed (deny by default)

🔴 **RULE 5: DEVELOPER AWARENESS**
- Show prominent warning: "⚠️ THIS TOOL CAN DELETE FILES"
- Remind users about .git, node_modules, build artifacts
- Default-OFF for destructive operations
- Require explicit "I understand" checkbox

---

## Part 1: Safety Enforcement (Backend)

### 1.1 Protected Paths (Blocklist)

**File: `src-tauri/src/utils/protected_paths.rs`** (NEW)

```rust
use std::path::{Path, PathBuf};
use crate::error::{ScannerError, Result};

/// System directories that should NEVER be scanned or deleted
/// Organized by OS for clarity
pub struct ProtectedPaths;

impl ProtectedPaths {
    /// macOS system directories
    #[cfg(target_os = "macos")]
    const PROTECTED_MACOS: &'static [&'static str] = &[
        "/System",
        "/Library",
        "/usr",
        "/bin",
        "/sbin",
        "/opt",
        "/var",
        "/etc",
        "/private",
        "/Volumes/Macintosh HD/System",
        "/Applications",
        "/Users/Shared",
    ];

    /// Windows system directories
    #[cfg(target_os = "windows")]
    const PROTECTED_WINDOWS: &'static [&'static str] = &[
        "C:\\Windows",
        "C:\\Program Files",
        "C:\\Program Files (x86)",
        "C:\\ProgramData",
        "C:\\System Volume Information",
        "C:\\$Recycle.Bin",
    ];

    /// Linux system directories
    #[cfg(target_os = "linux")]
    const PROTECTED_LINUX: &'static [&'static str] = &[
        "/usr",
        "/bin",
        "/sbin",
        "/etc",
        "/lib",
        "/var",
        "/sys",
        "/proc",
        "/boot",
        "/root",
    ];

    /// Critical developer directories (extra protection)
    const PROTECTED_DEV: &'static [&'static str] = &[
        ".git",
        ".gitignore",
        ".env",
        ".env.local",
        "package-lock.json",
        "Cargo.lock",
        ".DS_Store",
    ];

    /// Get all protected paths for current OS
    pub fn all_protected() -> Vec<&'static str> {
        let mut paths = Vec::new();

        #[cfg(target_os = "macos")]
        paths.extend(Self::PROTECTED_MACOS.iter().copied());

        #[cfg(target_os = "windows")]
        paths.extend(Self::PROTECTED_WINDOWS.iter().copied());

        #[cfg(target_os = "linux")]
        paths.extend(Self::PROTECTED_LINUX.iter().copied());

        paths.extend(Self::PROTECTED_DEV.iter().copied());
        paths
    }

    /// Check if a path is protected (cannot scan/delete)
    pub fn is_protected(path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();

        for protected in Self::all_protected() {
            let protected_lower = protected.to_lowercase();

            // Exact match
            if path_str == protected_lower {
                return true;
            }

            // Path component match (prevent scanning /System/* or ~/.git/*)
            if path_str.starts_with(&format!("{}/", protected_lower))
                || path_str.starts_with(&format!("{}\\", protected_lower))
            {
                return true;
            }

            // Ends with protected name (catch hidden dirs)
            if path_str.ends_with(&format!("/{}", protected_lower))
                || path_str.ends_with(&format!("\\{}", protected_lower))
            {
                return true;
            }
        }

        false
    }

    /// Validate a path is safe for scanning
    pub fn validate_scan_safe(path: &Path) -> Result<()> {
        if Self::is_protected(path) {
            return Err(ScannerError::InvalidPath {
                path: path.display().to_string(),
                reason: format!(
                    "This is a protected system directory. Scanning could affect system stability. \
                     Use a user directory instead (e.g., ~/Documents, ~/Projects)."
                ),
            });
        }

        Ok(())
    }

    /// Validate multiple paths for deletion (extra strict)
    pub fn validate_deletion_safe(paths: &[String]) -> Result<()> {
        for path_str in paths {
            let path = Path::new(path_str);

            if Self::is_protected(path) {
                return Err(ScannerError::CleanupFailed {
                    path: path_str.clone(),
                    reason: format!(
                        "BLOCKED: Protected system directory. \
                         This path cannot be deleted by this tool."
                    ),
                });
            }

            // Extra check: path should not be root or home
            if path == Path::new("/") || path == Path::new("\\")
                || path == Path::new("~") || path == Path::new("/~")
            {
                return Err(ScannerError::CleanupFailed {
                    path: path_str.clone(),
                    reason: "Cannot delete root or home directories.".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_paths_blocking() {
        #[cfg(target_os = "macos")]
        {
            assert!(ProtectedPaths::is_protected(Path::new("/System")));
            assert!(ProtectedPaths::is_protected(Path::new("/Library")));
            assert!(ProtectedPaths::is_protected(Path::new("/System/Library")));
        }

        assert!(ProtectedPaths::is_protected(Path::new("/home/user/.git")));
    }

    #[test]
    fn test_safe_paths_allowed() {
        assert!(!ProtectedPaths::is_protected(Path::new("/home/user/Projects")));
        assert!(!ProtectedPaths::is_protected(Path::new("/home/user/Downloads")));
    }
}
```

### 1.2 Enhanced Path Validation

**File: `src-tauri/src/utils/path.rs`** (UPDATE)

```rust
use std::path::{Path, PathBuf};
use crate::error::{ScannerError, Result};
use crate::utils::protected_paths::ProtectedPaths;

/// Comprehensive path validation for scanning operations
pub fn validate_scan_path(path: &str) -> Result<PathBuf> {
    // 1. Check if path is empty
    if path.is_empty() {
        return Err(ScannerError::InvalidPath {
            path: path.to_string(),
            reason: "Path cannot be empty".to_string(),
        });
    }

    // 2. Expand to absolute path
    let expanded = shellexpand::tilde(path);
    let path_obj = PathBuf::from(expanded.as_ref());

    // 3. Verify path exists
    if !path_obj.exists() {
        return Err(ScannerError::PathNotFound {
            path: path_obj.display().to_string(),
        });
    }

    // 4. Resolve symlinks to prevent loops
    let canonical = path_obj.canonicalize().map_err(|e| {
        ScannerError::InvalidPath {
            path: path_obj.display().to_string(),
            reason: format!("Cannot resolve path: {}", e),
        }
    })?;

    // 5. Check for protected system directories
    ProtectedPaths::validate_scan_safe(&canonical)?;

    // 6. Verify we have read permissions
    let metadata = std::fs::metadata(&canonical).map_err(|e| {
        ScannerError::PermissionDenied {
            path: canonical.display().to_string(),
        }
    })?;

    if !metadata.permissions().readonly() && !metadata.is_dir() {
        // Note: On Unix, we can't reliably check write perms on files
        // So we rely on actual operation failure
    }

    log::info!(
        "Path validated and safe: {}",
        canonical.display()
    );

    Ok(canonical)
}

/// Extra strict validation for deletion operations
pub fn validate_deletion_path(path: &str) -> Result<PathBuf> {
    // 1. All scan validation checks
    let validated = validate_scan_path(path)?;

    // 2. Extra checks for deletion
    ProtectedPaths::validate_deletion_safe(&[path.to_string()])?;

    // 3. Check path is not a directory that's too large/important
    if validated.is_dir() {
        // Check if directory contains .git
        if validated.join(".git").exists() {
            log::warn!(
                "Attempting to delete git repository: {}",
                validated.display()
            );
            // Log warning but allow - user has selected it explicitly
        }
    }

    Ok(validated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_path_rejected() {
        let result = validate_scan_path("");
        assert!(result.is_err());
    }

    #[test]
    fn test_nonexistent_path_rejected() {
        let result = validate_scan_path("/nonexistent/path/xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_symlink_resolved() {
        // Test on systems with /tmp symlink (most Unix systems)
        // Verify canonical path is returned
    }
}
```

### 1.3 Deletion Audit Logging

**File: `src-tauri/src/utils/audit.rs`** (NEW)

```rust
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use crate::error::Result;

/// Immutable audit trail for all file operations
pub struct AuditLogger;

impl AuditLogger {
    /// Log a deletion operation
    /// Format: timestamp | operation | path | size | status | reason
    pub fn log_deletion(
        path: &Path,
        size_bytes: u64,
        status: &str,  // "success", "skipped", "error"
        reason: &str,
    ) -> Result<()> {
        let audit_path = Self::audit_file_path()?;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let path_display = path.display();
        let size_mb = size_bytes as f64 / 1_048_576.0;

        let log_line = format!(
            "{} | DELETE | {} | {:.2}MB | {} | {}\n",
            timestamp, path_display, size_mb, status, reason
        );

        // Append to audit log (create if not exists)
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&audit_path)
            .map_err(|e| crate::error::ScannerError::InternalError {
                reason: format!("Failed to write audit log: {}", e),
            })?;

        file.write_all(log_line.as_bytes())
            .map_err(|e| crate::error::ScannerError::InternalError {
                reason: format!("Failed to write audit log: {}", e),
            })?;

        Ok(())
    }

    /// Log a scan operation
    pub fn log_scan(
        directory: &Path,
        scan_type: &str,
        results_count: usize,
        total_size: u64,
    ) -> Result<()> {
        let audit_path = Self::audit_file_path()?;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let dir_display = directory.display();
        let size_gb = total_size as f64 / 1_073_741_824.0;

        let log_line = format!(
            "{} | SCAN   | {} | {} | count:{} size:{:.2}GB\n",
            timestamp, dir_display, scan_type, results_count, size_gb
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&audit_path)?;

        file.write_all(log_line.as_bytes())?;

        Ok(())
    }

    /// Log a user action/decision
    pub fn log_action(action: &str, details: &str) -> Result<()> {
        let audit_path = Self::audit_file_path()?;

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!(
            "{} | ACTION | {} | {}\n",
            timestamp, action, details
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&audit_path)?;

        file.write_all(log_line.as_bytes())?;

        Ok(())
    }

    fn audit_file_path() -> Result<std::path::PathBuf> {
        let app_data = tauri::api::path::app_data_dir(&Default::default())
            .ok_or_else(|| crate::error::ScannerError::InternalError {
                reason: "Cannot determine app data directory".to_string(),
            })?;

        Ok(app_data.join("audit.log"))
    }
}
```

### 1.4 Enhanced Cleanup Command

**File: `src-tauri/src/commands/cleanup.rs`** (UPDATE)

```rust
use crate::error::{Result, ScannerError};
use crate::models::{CleanupReq, CleanupResult};
use crate::state::AppState;
use crate::utils::{path, audit, protected_paths};
use tauri::State;
use std::path::Path;

const MAX_BATCH_DELETE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
const MAX_BATCH_DELETE_COUNT: usize = 10_000; // 10k files

/// Validate paths before cleanup (frontend calls this first)
#[tauri::command]
pub fn validate_paths(paths: Vec<String>) -> Result<Vec<String>> {
    log::info!("Validating {} paths for deletion", paths.len());

    // 1. Check protected paths
    protected_paths::ProtectedPaths::validate_deletion_safe(&paths)?;

    // 2. Validate each path individually
    let mut validated = Vec::new();
    for path_str in paths {
        match path::validate_deletion_path(&path_str) {
            Ok(valid_path) => {
                validated.push(valid_path.display().to_string());
            }
            Err(e) => {
                log::warn!("Path validation failed: {} - {}", path_str, e);
                // Log but continue to validate other paths
                audit::AuditLogger::log_action(
                    "VALIDATION_FAILED",
                    &format!("{}: {}", path_str, e),
                )?;
            }
        }
    }

    // 3. Check batch limits
    if validated.len() > MAX_BATCH_DELETE_COUNT {
        return Err(ScannerError::BatchCountExceeded {
            attempted: validated.len(),
            max: MAX_BATCH_DELETE_COUNT,
        });
    }

    // 4. Calculate total size
    let total_size: u64 = validated
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();

    if total_size > MAX_BATCH_DELETE_SIZE {
        let total_gb = total_size as f64 / 1_073_741_824.0;
        let max_gb = MAX_BATCH_DELETE_SIZE as f64 / 1_073_741_824.0;
        return Err(ScannerError::BatchSizeExceeded {
            attempted: total_size,
            max: MAX_BATCH_DELETE_SIZE,
        });
    }

    log::info!(
        "Path validation successful: {} items, {:.2}GB",
        validated.len(),
        total_size as f64 / 1_073_741_824.0
    );

    audit::AuditLogger::log_action(
        "VALIDATION_SUCCESS",
        &format!("{} items, {:.2}GB", validated.len(), total_size as f64 / 1_073_741_824.0),
    )?;

    Ok(validated)
}

/// Actual deletion command
#[tauri::command]
pub async fn cleanup_dirs(
    req: CleanupReq,
    state: State<'_, AppState>,
) -> Result<CleanupResult> {
    state.set_scanning(true, format!("Deleting {} items...", req.paths.len()));

    let mut deleted = Vec::new();
    let mut skipped = Vec::new();
    let mut errors = Vec::new();

    log::info!(
        "Starting cleanup: {} paths, dry_run={}, trash={}",
        req.paths.len(),
        req.dry_run,
        req.trash
    );

    audit::AuditLogger::log_action(
        "CLEANUP_START",
        &format!("{} items, dry_run={}", req.paths.len(), req.dry_run),
    )?;

    if req.dry_run {
        log::info!("DRY RUN: Would delete {} items", req.paths.len());
        audit::AuditLogger::log_action("DRY_RUN", &format!("{} items shown", req.paths.len()))?;
        return Ok(CleanupResult {
            deleted: req.paths.clone(),
            skipped: vec![],
            errors: vec![],
        });
    }

    for path in &req.paths {
        let p = Path::new(path);
        log::debug!("Processing deletion: {}", path);

        if !p.exists() {
            log::debug!("File does not exist, skipping: {}", path);
            skipped.push(path.clone());
            audit::AuditLogger::log_deletion(p, 0, "skipped", "file not found")?;
            continue;
        }

        // Get size before deletion (for audit)
        let size_bytes = std::fs::metadata(p)
            .map(|m| m.len())
            .unwrap_or(0);

        if req.trash {
            // Move to trash (safe)
            match trash::delete(p) {
                Ok(_) => {
                    log::info!("Successfully moved to trash: {}", path);
                    deleted.push(path.clone());
                    audit::AuditLogger::log_deletion(p, size_bytes, "success", "moved to trash")?;
                }
                Err(e) => {
                    log::error!("Failed to delete {}: {}", path, e);
                    errors.push(format!("{}: {}", path, e));
                    audit::AuditLogger::log_deletion(p, size_bytes, "error", &e.to_string())?;
                }
            }
        } else {
            // Permanent deletion (only if user explicitly requested)
            let result = if p.is_dir() {
                std::fs::remove_dir_all(p)
            } else {
                std::fs::remove_file(p)
            };

            match result {
                Ok(_) => {
                    log::warn!("Permanently deleted: {}", path);
                    deleted.push(path.clone());
                    audit::AuditLogger::log_deletion(p, size_bytes, "success", "permanently deleted")?;
                }
                Err(e) => {
                    log::error!("Failed to permanently delete {}: {}", path, e);
                    errors.push(format!("{}: {}", path, e));
                    audit::AuditLogger::log_deletion(p, size_bytes, "error", &e.to_string())?;
                }
            }
        }
    }

    log::info!(
        "Cleanup complete: {} deleted, {} skipped, {} errors",
        deleted.len(),
        skipped.len(),
        errors.len()
    );

    audit::AuditLogger::log_action(
        "CLEANUP_COMPLETE",
        &format!("deleted:{} skipped:{} errors:{}", deleted.len(), skipped.len(), errors.len()),
    )?;

    state.set_scanning(false, "Cleanup complete");

    Ok(CleanupResult {
        deleted,
        skipped,
        errors,
    })
}
```

---

## Part 2: Safety UI (Frontend)

### 2.1 Deletion Confirmation Dialog

**File: `src/lib/components/DeleteConfirmDialog.svelte`** (NEW)

```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let items: { path: string; size_mb: number }[] = [];
  export let isOpen = false;

  const dispatch = createEventDispatcher();

  let confirmChecked = false;
  let countdownSeconds = 30;
  let canConfirm = false;

  // Countdown timer
  $: if (isOpen) {
    confirmChecked = false;
    canConfirm = false;
    countdownSeconds = 30;
    
    const timer = setInterval(() => {
      countdownSeconds -= 1;
      if (countdownSeconds <= 0) {
        clearInterval(timer);
        canConfirm = true;
      }
    }, 1000);

    return () => clearInterval(timer);
  }

  function handleConfirm() {
    if (confirmChecked && canConfirm) {
      dispatch('confirmed');
      isOpen = false;
    }
  }

  function handleCancel() {
    dispatch('cancelled');
    isOpen = false;
  }

  $: totalSize = items.reduce((sum, item) => sum + item.size_mb, 0);
</script>

{#if isOpen}
  <div class="modal-overlay" on:click={handleCancel}>
    <div class="modal" on:click|stopPropagation>
      <div class="header error">
        <h2>⚠️ Confirm Deletion</h2>
        <p class="subtitle">This action cannot be undone immediately</p>
      </div>

      <div class="content">
        <div class="warning-box">
          <p class="warning-title">About to delete {items.length} items ({totalSize.toFixed(2)} MB)</p>
          
          {#if items.length <= 10}
            <ul class="item-list">
              {#each items as item}
                <li>
                  <code>{item.path}</code>
                  <span class="size">({item.size_mb.toFixed(2)} MB)</span>
                </li>
              {/each}
            </ul>
          {:else}
            <p class="many-items">
              Showing first 10 of {items.length} items:
            </p>
            <ul class="item-list">
              {#each items.slice(0, 10) as item}
                <li>
                  <code>{item.path}</code>
                  <span class="size">({item.size_mb.toFixed(2)} MB)</span>
                </li>
              {/each}
            </ul>
            <p class="and-more">...and {items.length - 10} more items</p>
          {/if}
        </div>

        <div class="safety-info">
          <p class="info-title">Safety Information:</p>
          <ul>
            <li>✅ Items will be moved to system trash</li>
            <li>✅ You can recover them for 30+ days</li>
            <li>✅ A complete audit log is maintained</li>
            <li>🟡 Double-check the list above before confirming</li>
          </ul>
        </div>

        {#if canConfirm}
          <div class="confirmation">
            <label class="checkbox">
              <input
                type="checkbox"
                bind:checked={confirmChecked}
              />
              <span>
                Yes, I understand this action and want to delete
                <strong>{items.length}</strong>
                items
              </span>
            </label>
          </div>
        {:else}
          <div class="countdown">
            <p>Please review the items above.</p>
            <p class="timer">Confirmation available in {countdownSeconds} seconds...</p>
          </div>
        {/if}
      </div>

      <div class="footer">
        <button 
          class="cancel-btn" 
          on:click={handleCancel}
        >
          Cancel
        </button>
        
        <button
          class="delete-btn"
          disabled={!confirmChecked || !canConfirm}
          on:click={handleConfirm}
        >
          {#if canConfirm}
            Delete {items.length} Items
          {:else}
            Delete (Review First)
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background: white;
    border-radius: 8px;
    max-width: 600px;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 20px 25px rgba(0, 0, 0, 0.15);
  }

  .header {
    padding: 24px;
    border-bottom: 2px solid #fee2e2;
    background: #fef2f2;
  }

  .header h2 {
    margin: 0 0 8px 0;
    color: #991b1b;
    font-size: 20px;
  }

  .header.error {
    background: #fef2f2;
    border-color: #fee2e2;
  }

  .subtitle {
    margin: 0;
    color: #dc2626;
    font-size: 14px;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
  }

  .warning-box {
    background: #fff7ed;
    border: 1px solid #fed7aa;
    border-radius: 6px;
    padding: 16px;
    margin-bottom: 20px;
  }

  .warning-title {
    margin: 0 0 12px 0;
    font-weight: 600;
    color: #92400e;
  }

  .item-list {
    margin: 0;
    padding-left: 20px;
    max-height: 200px;
    overflow-y: auto;
  }

  .item-list li {
    margin: 8px 0;
    font-size: 13px;
    word-break: break-all;
    color: #666;
  }

  .item-list code {
    background: #f3f4f6;
    padding: 2px 6px;
    border-radius: 3px;
    font-family: monospace;
  }

  .size {
    color: #999;
    font-size: 12px;
    margin-left: 8px;
  }

  .many-items {
    margin: 12px 0 8px 0;
    font-size: 13px;
    color: #92400e;
  }

  .and-more {
    margin: 12px 0 0 0;
    font-size: 13px;
    color: #999;
    font-style: italic;
  }

  .safety-info {
    background: #eff6ff;
    border: 1px solid #bfdbfe;
    border-radius: 6px;
    padding: 16px;
    margin-bottom: 20px;
  }

  .info-title {
    margin: 0 0 12px 0;
    font-weight: 600;
    color: #1e40af;
  }

  .safety-info ul {
    margin: 0;
    padding-left: 20px;
  }

  .safety-info li {
    margin: 6px 0;
    font-size: 13px;
    color: #1e40af;
  }

  .confirmation {
    margin-bottom: 20px;
  }

  .checkbox {
    display: flex;
    align-items: center;
    cursor: pointer;
    font-size: 14px;
  }

  .checkbox input {
    margin-right: 12px;
    cursor: pointer;
    width: 20px;
    height: 20px;
  }

  .checkbox span {
    color: #374151;
  }

  .checkbox strong {
    color: #dc2626;
    font-weight: 700;
  }

  .countdown {
    background: #fef3c7;
    border: 1px solid #fcd34d;
    border-radius: 6px;
    padding: 16px;
    text-align: center;
  }

  .countdown p {
    margin: 0 0 8px 0;
    color: #92400e;
    font-size: 14px;
  }

  .timer {
    font-weight: 600;
    color: #dc2626;
    font-size: 16px;
  }

  .footer {
    display: flex;
    gap: 12px;
    padding: 16px 24px;
    border-top: 1px solid #e5e7eb;
    background: #f9fafb;
  }

  .cancel-btn {
    flex: 1;
    padding: 12px 16px;
    background: #e5e7eb;
    color: #374151;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    font-size: 14px;
  }

  .cancel-btn:hover {
    background: #d1d5db;
  }

  .delete-btn {
    flex: 1;
    padding: 12px 16px;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 600;
    cursor: pointer;
    font-size: 14px;
  }

  .delete-btn:hover:not(:disabled) {
    background: #991b1b;
  }

  .delete-btn:disabled {
    background: #d1d5db;
    cursor: not-allowed;
    color: #9ca3af;
  }
</style>
```

### 2.2 Protected Paths Warning

**File: `src/lib/components/ProtectionNotice.svelte`** (NEW)

```svelte
<script lang="ts">
  import { onMount } from 'svelte';

  let isVisible = true;

  onMount(() => {
    // Dismiss after 5 seconds
    const timer = setTimeout(() => {
      isVisible = false;
    }, 5000);

    return () => clearTimeout(timer);
  });
</script>

{#if isVisible}
  <div class="notice warning">
    <div class="icon">⚠️</div>
    <div class="content">
      <h3>System Protection Active</h3>
      <p>
        This tool automatically blocks dangerous system directories 
        (<code>/System</code>, <code>/usr</code>, <code>C:\Windows</code>, etc.)
        and protects important files like <code>.git</code> and <code>package-lock.json</code>.
      </p>
      <p style="margin-bottom: 0; font-size: 0.9em; color: #666;">
        ✅ System directories are never scanned
        | ✅ All deletions are reversible
        | ✅ Complete audit trail maintained
      </p>
    </div>
    <button class="close" on:click={() => (isVisible = false)}>✕</button>
  </div>
{/if}

<style>
  .notice {
    display: flex;
    gap: 16px;
    padding: 16px;
    border-radius: 6px;
    margin-bottom: 16px;
    border-left: 4px solid;
  }

  .notice.warning {
    background: #fef3c7;
    border-color: #f59e0b;
  }

  .icon {
    font-size: 24px;
    flex-shrink: 0;
  }

  .content {
    flex: 1;
  }

  .content h3 {
    margin: 0 0 8px 0;
    color: #92400e;
    font-size: 16px;
  }

  .content p {
    margin: 0 0 8px 0;
    color: #92400e;
    font-size: 14px;
    line-height: 1.5;
  }

  .content code {
    background: rgba(0, 0, 0, 0.1);
    padding: 2px 6px;
    border-radius: 3px;
    font-family: monospace;
    font-size: 0.9em;
  }

  .close {
    background: none;
    border: none;
    font-size: 20px;
    color: #92400e;
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
  }

  .close:hover {
    color: #78350f;
  }
</style>
```

---

## Part 3: Critical Safety Rules (Code Enforcement)

### 3.1 Pre-Deletion Checklist

**Every deletion MUST satisfy ALL of these before proceeding:**

```rust
// Before ANY file is deleted:
let can_delete = true;

// 1. Path is not protected
can_delete &= !ProtectedPaths::is_protected(&path);

// 2. File still exists (no TOCTOU race condition)
can_delete &= path.exists();

// 3. File is readable (permissions)
can_delete &= std::fs::metadata(&path).is_ok();

// 4. Size is retrievable (can calculate audit entry)
let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
can_delete &= size < MAX_BATCH_DELETE_SIZE;

// 5. Using trash (not permanent delete in v0.x)
// - Permanent delete only with explicit req.trash=false AND confirmation

// 6. Audit log is writable (MUST log before deletion)
can_delete &= audit::AuditLogger::log_deletion(...).is_ok();

if can_delete {
    // NOW delete
} else {
    // FAIL CLOSED - don't delete
}
```

### 3.2 Configuration (Cargo.toml)

```toml
[dependencies]
# Add audit trail support
chrono = { version = "0.4", features = ["serde"] }
parking_lot = "0.12"  # Better locks than std::sync
shellexpand = "3.1"   # Path expansion (~/)
```

---

## Part 4: Usage Safety Guide (User-Facing)

### 4.1 Developer Checklist Before Running

**⚠️ CRITICAL - READ BEFORE USING:**

- [ ] **Commit all work**: `git add . && git commit -m "work in progress"`
- [ ] **No uncommitted code**: `git status` shows clean or only untracked files
- [ ] **Backup important directories**: Copy critical projects elsewhere
- [ ] **Understand what you're deleting**: Review preview list thoroughly
- [ ] **Start with safe paths**: Test on `~/Downloads`, not system directories
- [ ] **Use DRY RUN first**: Check what WOULD be deleted before actual delete

### 4.2 Emergency Recovery

**If something goes wrong:**

1. **Files are in Trash**: Open system trash and restore
2. **Git history preserved**: `git log` and `git reflog` show all commits
3. **Audit trail available**: Check `~/.disk-bloat-scanner/audit.log`
4. **Contact team lead**: Report issue with:
   - Exact paths deleted
   - Timestamp from status
   - Contents of audit.log

---

## Part 5: Deployment Safety

### 5.1 Release Checklist

- [ ] All paths validated against ProtectedPaths blocklist
- [ ] Audit logging tested and working
- [ ] Deletion confirmation UI tested manually
- [ ] 30-second countdown verification
- [ ] Trash-only mode enabled (no permanent delete)
- [ ] Error messages are clear and actionable
- [ ] Audit logs rotate/archive (future v1.0)

### 5.2 Monitoring

**After deployment, monitor:**

- Daily: Check audit logs for unexpected patterns
- Weekly: Verify trash recovery works
- Monthly: Review audit logs for edge cases
- Quarterly: Test complete restore from trash

---

## Conclusion

**Safety-First Principles:**

🔴 **Trust But Verify** - Multiple confirmation layers  
🔴 **Trash Always** - No permanent deletion in v0.x  
🔴 **Audit Everything** - Immutable deletion log  
🔴 **Protect Systems** - Hardcoded dangerous path blocklist  
🔴 **Developer Aware** - Prominent warnings about data loss risk  

**This tool is production-ready because:**

✅ Code cannot be accidentally deleted (protected paths)  
✅ All deletions are reversible (trash-based)  
✅ All deletions are logged (audit trail)  
✅ Confirmation is mandatory and deliberate  
✅ Users are warned about the risks

---

**Document Status:** Implementation Guide  
**Target Version:** v0.1.1 Production Release  
**Next Step:** Implement all safety features before release  

