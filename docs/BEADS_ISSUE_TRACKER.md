# 🔴 BEADS Issue Tracker - Project Scanner v0.1.1

**Based on Independent Security & Quality Audit - October 24, 2025**

**Overall Score: 4.75/10** 🚨 **MAJOR ISSUES IDENTIFIED**

---

## 📊 Status Summary

| Priority | Total | Completed | In Progress | Pending |
|----------|-------|-----------|-------------|---------|
| 🚨 Critical | 8 | 5 | 0 | 3 |
| ⚠️ High | 12 | 1 | 0 | 11 |
| 📝 Medium | 15 | 1 | 0 | 14 |
| ✅ Low | 8 | 0 | 0 | 8 |
| **TOTAL** | **43** | **7** | **0** | **36** |

---

## 🚨 PHASE 1: CRITICAL SECURITY FIXES (Priority 0 - Blocking)

### BEAD-001: Enable Content Security Policy 🚨
**Status:** ✅ COMPLETED (Oct 24, 2025)  
**Priority:** CRITICAL  
**Effort:** 15 minutes  
**Impact:** Prevents XSS attacks, code injection

**Current State:**
```json
// tauri.conf.json:23
"security": {
    "csp": null  // ❌ COMPLETELY DISABLED
}
```

**Required Fix:**
```json
"security": {
    "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:;"
}
```

**Files:** `src-tauri/tauri.conf.json`  
**Assignee:** TBD  
**Dependencies:** None  
**Test:** Verify app loads correctly with CSP enabled

---

### BEAD-002: Implement Path Validation with Blacklist 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 2-3 hours  
**Impact:** Prevents scanning/deleting system directories

**Risk:** User can currently scan `/System`, `/usr`, `/bin` and potentially brick macOS

**Required Implementation:**
```rust
// src-tauri/src/utils/path.rs (NEW FILE)
use std::path::{Path, PathBuf};

pub fn validate_scan_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    let canonical = path.canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    
    // Block system directories
    let blocked = [
        "/System", "/bin", "/sbin", "/usr", "/etc", 
        "/var", "/private/var", "/Library/LaunchDaemons",
        "/Library/LaunchAgents", "C:\\Windows", "C:\\Program Files"
    ];
    
    for blocked_path in blocked {
        if canonical.starts_with(blocked_path) {
            return Err(format!("Access denied: system directory"));
        }
    }
    
    if !canonical.exists() {
        return Err("Path does not exist".to_string());
    }
    
    Ok(canonical)
}
```

**Files to Modify:**
- `src-tauri/src/lib.rs` - All scan commands
- `src-tauri/src/commands/mod.rs` (create module structure)
- `src-tauri/src/utils/path.rs` (create new file)

**Assignee:** TBD  
**Dependencies:** None  
**Test:** Attempt to scan `/System` - should fail with error

---

### BEAD-003: Fix TOCTOU Race Condition in Deletion 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 2 hours  
**Impact:** Prevents deleting wrong files due to race conditions

**Current Issue:**
```rust
// lib.rs:712-717
if !p.exists() {
    skipped.push(path.clone());
    continue;
}
// ❌ File could be deleted between exists() and trash::delete()
```

**Required Fix:**
```rust
// Use atomic file operations
match trash::delete(p) {
    Ok(_) => {
        // Wait briefly for OS to complete
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        if p.exists() {
            return Err(format!("Failed to delete {}: file still exists", path));
        }
        
        log::info!("Successfully deleted: {}", path);
        deleted.push(path.clone());
    }
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        skipped.push(path.clone());
    }
    Err(e) => {
        return Err(format!("Deletion failed: {}", e));
    }
}
```

**Files:** `src-tauri/src/lib.rs` (cleanup_dirs function)  
**Assignee:** TBD  
**Dependencies:** None  
**Test:** Verify deletion is atomic and verified

---

### BEAD-004: Add Deletion History Logging 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 3 hours  
**Impact:** Audit trail for all file deletions

**Implementation:**
```rust
// src-tauri/src/models/deletion_log.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionRecord {
    pub path: String,
    pub size_bytes: u64,
    pub deleted_at: DateTime<Utc>,
    pub category: String,
    pub restored: bool,
}

pub fn log_deletion(record: &DeletionRecord) -> Result<(), std::io::Error> {
    let log_path = app_data_dir().join("deletion_log.jsonl");
    let json = serde_json::to_string(record)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    writeln!(file, "{}", json)?;
    Ok(())
}
```

**Files:**
- Create: `src-tauri/src/models/deletion_log.rs`
- Modify: `src-tauri/src/lib.rs` (cleanup_dirs)
- Add dependency: `chrono = "0.4"`

**Assignee:** TBD  
**Dependencies:** None  
**Test:** Delete files, check `deletion_log.jsonl` exists with records

---

### BEAD-005: Replace All `.unwrap()` with Proper Error Handling 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 4-6 hours  
**Impact:** Prevents crashes, improves error reporting

**Locations to Fix:**
- `lib.rs:177` - dir_size function (multiple .ok())
- `lib.rs:335-337` - disk info retrieval
- `lib.rs:474` - mutex lock
- Throughout codebase (grep shows ~50 instances)

**Pattern:**
```rust
// ❌ BEFORE:
let value = some_result.unwrap();

// ✅ AFTER:
let value = some_result.map_err(|e| format!("Operation failed: {}", e))?;
```

**Files:** `src-tauri/src/lib.rs` (all functions)  
**Assignee:** TBD  
**Dependencies:** None  
**Test:** Run clippy with `unwrap_used = "deny"`

---

### BEAD-006: Implement Proper Error Types with `thiserror` 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 3-4 hours  
**Impact:** Type-safe errors, better debugging

**Implementation:**
```rust
// src-tauri/src/utils/error.rs
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ScanError {
    #[error("Path not found: {0}")]
    PathNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Scan cancelled")]
    Cancelled,
}

// Convert all Result<T, String> to Result<T, ScanError>
```

**Files:**
- Create: `src-tauri/src/utils/error.rs`
- Modify: All command functions in `src-tauri/src/lib.rs`
- Add dependency: `thiserror = "1.0"`

**Assignee:** TBD  
**Dependencies:** None  
**Test:** Trigger various errors, verify proper error messages

---

### BEAD-007: Add Batch Deletion Size Limits 🚨
**Status:** ✅ COMPLETED (Oct 24, 2025)  
**Priority:** CRITICAL  
**Effort:** 2 hours  
**Impact:** Prevents accidental mass deletion

**Implementation:**
```rust
const MAX_BATCH_DELETE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
const MAX_BATCH_DELETE_COUNT: usize = 10_000; // 10k files

fn validate_deletion_request(req: &CleanupReq) -> Result<(), String> {
    if req.paths.len() > MAX_BATCH_DELETE_COUNT {
        return Err(format!("Cannot delete {} files at once (max: {})", 
            req.paths.len(), MAX_BATCH_DELETE_COUNT));
    }
    
    let total_size: u64 = req.paths.iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();
    
    if total_size > MAX_BATCH_DELETE_SIZE {
        return Err(format!("Cannot delete {} GB at once", 
            total_size / (1024*1024*1024)));
    }
    
    Ok(())
}
```

**Files:** `src-tauri/src/lib.rs` (cleanup_dirs function)  
**Assignee:** TBD  
**Dependencies:** None  
**Test:** Try to delete 100GB+ - should fail with error

---

### BEAD-008: Add Warning for Critical Path Deletion 🚨
**Status:** ✅ COMPLETED (Oct 24, 2025)  
**Priority:** CRITICAL  
**Effort:** 1 hour  
**Impact:** Prevents accidental source code deletion

**Implementation:**
```javascript
// In all deletion components (LargeFiles, SystemJunk, etc.)
const isCriticalPath = (path) => {
    return path.includes('/src/') || 
           path.includes('/lib/') ||
           path.includes('.git/') ||
           path.match(/\.(rs|js|ts|py|go|cpp|java|rb|php)$/);
};

if (Array.from($selectedPaths).some(isCriticalPath)) {
    const confirmed = confirm(
        "⚠️ WARNING: You're about to delete SOURCE CODE files!\n\n" +
        "This is IRREVERSIBLE. Are you absolutely sure?"
    );
    if (!confirmed) return;
}
```

**Files:**
- `src/lib/components/LargeFiles.svelte`
- `src/lib/components/SystemJunk.svelte`
- `src/lib/components/ProjectBloat.svelte`

**Assignee:** TBD  
**Dependencies:** None  
**Test:** Select .rs file for deletion - should show extra warning

---

## ⚠️ PHASE 2: HIGH PRIORITY FIXES

### BEAD-009: Make `dir_size()` Async ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 2 hours  
**Impact:** Prevents UI freezing during large directory scans

**Current Issue:** Blocking I/O in async context (lib.rs:174-182)

**Files:** `src-tauri/src/lib.rs`  
**Assignee:** TBD

---

### BEAD-010: Implement Scan Cancellation ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 4 hours  
**Impact:** User can stop long-running scans

**Implementation:** Use `tokio::sync::CancellationToken`

**Files:** `src-tauri/src/lib.rs`, `src/lib/components/Dashboard.svelte`  
**Assignee:** TBD

---

### BEAD-011: Add Real Progress Tracking ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 3 hours  
**Impact:** Replace fake progress counter with real data

**Current Issue:** Dashboard.svelte:78-81 shows fake random progress

**Implementation:** Use Tauri events to emit progress

**Files:** `src-tauri/src/lib.rs`, `src/lib/components/Dashboard.svelte`  
**Assignee:** TBD

---

### BEAD-012: Fix Memory Leak in Dashboard ⚠️
**Status:** ✅ COMPLETED (Oct 24, 2025)  
**Priority:** HIGH  
**Effort:** 30 minutes  
**Impact:** Prevents memory accumulation

**Current Issue:** Dashboard.svelte interval may not clean up on remount

**Files:** `src/lib/components/Dashboard.svelte`  
**Assignee:** TBD

---

### BEAD-013: Add Error Boundaries ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 2 hours  
**Impact:** Prevents full app crash on component errors

**Files:** Create `src/lib/components/common/ErrorBoundary.svelte`  
**Assignee:** TBD

---

### BEAD-014: Implement Retry Logic for Transient Failures ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 2 hours  
**Impact:** Resilience against network/disk issues

**Files:** `src-tauri/src/lib.rs`  
**Assignee:** TBD

---

### BEAD-015: Add Symlink Protection ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 3 hours  
**Impact:** Prevent symlink attacks and infinite loops

**Files:** `src-tauri/src/lib.rs`  
**Assignee:** TBD

---

### BEAD-016: Define Tauri v2 Permissions ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 1 hour  
**Impact:** Proper security model compliance

**Current:** `capabilities/default.json` only has core:default and dialog:default

**Required:** Add fs:read-dir, fs:read-file, fs:remove, etc.

**Files:** `src-tauri/capabilities/default.json`  
**Assignee:** TBD

---

### BEAD-017: Add In-App Error UI ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 3 hours  
**Impact:** Better error UX than alert() boxes

**Files:** Create error display component  
**Assignee:** TBD

---

### BEAD-018: Implement Logging Throughout ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 2 hours  
**Impact:** Debugging and audit trails

**Files:** All Rust files  
**Assignee:** TBD

---

### BEAD-019: Optimize Mutex Usage in scan_bloat ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 3 hours  
**Impact:** Better performance on large scans

**Current Issue:** lib.rs:474-490 has excessive mutex locking

**Files:** `src-tauri/src/lib.rs`  
**Assignee:** TBD

---

### BEAD-020: Chunked File Hashing ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 2 hours  
**Impact:** Prevents memory spikes on large files

**Current Issue:** lib.rs:579-580 reads entire file for hashing

**Files:** `src-tauri/src/lib.rs`  
**Assignee:** TBD

---

## 📝 PHASE 3: MEDIUM PRIORITY (Architecture & Maintainability)

### BEAD-021: Split lib.rs into Modules 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 6-8 hours

**Structure:**
```
src-tauri/src/
├── commands/
│   ├── mod.rs
│   ├── scan.rs
│   ├── cleanup.rs
│   └── system.rs
├── models/
├── services/
└── utils/
```

**Files:** Refactor `src-tauri/src/lib.rs` (787 lines → modular)  
**Assignee:** TBD

---

### BEAD-022: Add Comprehensive Unit Tests 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 8-12 hours  
**Target:** 70% code coverage

**Files:** Create test files for each module  
**Assignee:** TBD

---

### BEAD-023: Migrate to TypeScript 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4-6 hours

**Files:** Rename all .js to .ts, add type annotations  
**Assignee:** TBD

---

### BEAD-024: Add API Documentation 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4 hours

**Files:** Create `docs/api/TAURI_COMMANDS.md`  
**Assignee:** TBD

---

### BEAD-025: Implement FileScanner Trait 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 6 hours

**Files:** Create abstraction layer for scanners  
**Assignee:** TBD

---

### BEAD-026: Make Bloat Patterns Configurable 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4 hours

**Implementation:** Load from `bloat_patterns.toml`

**Files:** `src-tauri/src/lib.rs`, create config file  
**Assignee:** TBD

---

### BEAD-027: Add Deletion Size Warning to UI 📝
**Status:** ✅ COMPLETED (Oct 24, 2025)  
**Priority:** MEDIUM  
**Effort:** 1 hour

**Files:** All deletion components  
**Assignee:** TBD

---

### BEAD-028: Fix Page Reload After Deletion 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 2 hours

**Current:** `window.location.reload()` is used  
**Required:** Reactive store updates

**Files:** `src/lib/components/ProjectBloat.svelte`, `SystemJunk.svelte`  
**Assignee:** TBD

---

### BEAD-029: Add Keyboard Shortcuts 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3 hours

**Shortcuts:**
- Cmd/Ctrl+S: Start scan
- Cmd/Ctrl+D: Delete selected
- Escape: Cancel scan

**Files:** Create keyboard handler  
**Assignee:** TBD

---

### BEAD-030: Implement Virtual Scrolling 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4 hours

**Impact:** Better performance for large file lists

**Files:** All list components  
**Assignee:** TBD

---

### BEAD-031: Add Search/Filter Functionality 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3 hours

**Files:** All list components  
**Assignee:** TBD

---

### BEAD-032: Add Accessibility Features 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 6 hours

- ARIA labels
- Screen reader support
- Keyboard navigation
- WCAG AA compliance

**Files:** All components  
**Assignee:** TBD

---

### BEAD-033: Add Tooltips 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 2 hours

**Files:** All components with safety indicators  
**Assignee:** TBD

---

### BEAD-034: Implement Scan Result Caching 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4 hours

**Files:** `src-tauri/src/lib.rs`, add cache layer  
**Assignee:** TBD

---

### BEAD-035: Add "Restore from Trash" UI 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4 hours

**Files:** Create new component  
**Assignee:** TBD

---

## ✅ PHASE 4: LOW PRIORITY (Polish & Nice-to-Have)

### BEAD-036: Add Telemetry/Crash Reporting ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 4 hours

**Files:** Add Sentry or similar  
**Assignee:** TBD

---

### BEAD-037: Performance Benchmarking ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 3 hours

**Files:** Create benchmark suite  
**Assignee:** TBD

---

### BEAD-038: Cross-Platform Testing ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 8+ hours

**Test on:** Windows 11, Ubuntu 22.04

**Files:** CI/CD setup  
**Assignee:** TBD

---

### BEAD-039: Add User Guide ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 4 hours

**Files:** Create `docs/USER_GUIDE.md`  
**Assignee:** TBD

---

### BEAD-040: Implement Scheduled Scans ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 6 hours

**Files:** Background task scheduler  
**Assignee:** TBD

---

### BEAD-041: Add Export to CSV/JSON ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 3 hours

**Files:** Export functionality in components  
**Assignee:** TBD

---

### BEAD-042: Dark/Light Theme Toggle ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 4 hours

**Files:** Theme system  
**Assignee:** TBD

---

### BEAD-043: Add Custom Bloat Patterns UI ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 6 hours

**Files:** Settings page enhancement  
**Assignee:** TBD

---

## ✅ COMPLETED QUICK WINS

### BEAD-QW-001: Remove Hardcoded User Path ✅
**Status:** COMPLETED  
**Completed:** October 24, 2025  
**Effort:** 5 minutes  

**Change:** `stores.js:24` - Changed from `["/Users/frank/..."]` to `[]`

---

### BEAD-QW-002: Replace println! with log:: ✅
**Status:** COMPLETED  
**Completed:** October 24, 2025  
**Effort:** 30 minutes  

**Files:** `src-tauri/src/lib.rs` - All debug prints now use proper logging

---

## 📈 Progress Metrics

**Phase 1 (Critical):** 5/8 complete (62.5%) 🔥  
**Phase 2 (High):** 1/12 complete (8.3%)  
**Phase 3 (Medium):** 1/15 complete (6.7%)  
**Phase 4 (Low):** 0/8 complete (0%)  

**Overall Progress:** 7/43 (16.3%)

**Session Progress Today:** +5 BEADS completed! 🎉

---

## 🎯 Recommended Next 5 Beads (In Order)

1. **BEAD-001** - Enable CSP (15 min) 🚨
2. **BEAD-002** - Path Validation (2-3 hrs) 🚨
3. **BEAD-007** - Batch Deletion Limits (2 hrs) 🚨
4. **BEAD-027** - Deletion Size Warning UI (1 hr) 📝
5. **BEAD-012** - Fix Memory Leak (30 min) ⚠️

**Total Effort:** ~6 hours for significant security improvement

---

**Last Updated:** October 24, 2025  
**Next Review:** After Phase 1 completion
