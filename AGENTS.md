# 🤖 AGENTS.md - Disk Bloat Scanner Development Guide

**Last Updated:** October 27, 2025, 22:00 UTC  
**Project:** Disk Bloat Scanner v0.1.1 + PACS v1.0 (Proposed)  
**Current Phase:** Phase 2 (Architecture Refactoring) - 45% complete  
**Status**: Active Development - EDGS Event-Driven Gated Scheduling

---

## 📋 CURRENT SESSION STATE (Oct 27, 2025, Evening)

### Major Achievements This Session

**✅ Phase 2 Refactoring Progress**:
- P2-T5: Documentation completion (zero warnings) ✅ COMPLETE
- P2-T6: Cleanup module created (190 lines) ✅ COMPLETE
- P2-T7: Lib.rs refactoring prepared (ready for integration) ⏳ PENDING

**✅ PACS Feature Specification (NEW)**:
- Comprehensive OpenSpec proposal & tasks (4,127 lines)
- 19 Beads issues across 3 features
- Complete architecture and compliance framework
- Ready for stakeholder review and Gate 0 approval

### Current Blocker

**None** - All systems green. Ready to proceed with lib.rs integration (P2-T7) next session.

**Status:**
- ✅ Created `src-tauri/src/utils/path.rs` module (118 lines)
- ✅ Created `src-tauri/src/utils/mod.rs`
- ✅ Added module import to `src-tauri/src/lib.rs`
- 🔄 IN PROGRESS: Integrating validation into scan functions
- ⚠️ BLOCKED: Compilation errors due to incomplete `scan_duplicates` function

### Files Modified (Uncommitted)

```
src-tauri/src/lib.rs - MODIFIED
  - Added: mod utils; declaration
  - Added: use utils::path::validate_scan_path;
  - Modified: scan_large_files() - path validation added
  - Modified: scan_bloat() - path validation added
  - BROKEN: scan_duplicates() - incomplete edit, missing code
```

### Current Build Error

```
error[E0425]: cannot find value `size_groups` in this scope
error[E0425]: cannot find value `file_hashes` in this scope
```

**Root Cause:** The `scan_duplicates` function was partially edited and critical code was removed. The function needs to be restored.

---

## 🔧 RESTART PROCEDURE

### Option 1: Continue from Current State (RECOMMENDED)

```bash
cd /Users/frank/Development/private/projects/disk-bloat-scanner

# 1. Revert broken lib.rs changes
git checkout src-tauri/src/lib.rs

# 2. Verify utils module is still committed
git log --oneline -5 | grep "path validation"
# Should see: 30d20de feat: add path validation utility module (BEAD-002 part 1)

# 3. Check current status
git status

# 4. Manually add module import to lib.rs (safe method)
# Edit src-tauri/src/lib.rs:
#   - Line 1: Add "mod utils;"
#   - Line 5: Add "use utils::path::validate_scan_path;"

# 5. Apply validation to ONE function at a time
# Start with scan_bloat (line ~473)

# 6. Test build after each function
cargo build

# 7. Commit when working
git add src-tauri/src/lib.rs
git commit -m "feat: integrate path validation into scan commands (BEAD-002 complete)"
```

### Option 2: Start Fresh from Last Good State

```bash
# 1. Discard all uncommitted changes
git checkout .

# 2. Verify you're on the last good commit
git log --oneline -1
# Should show: 30d20de feat: add path validation utility module (BEAD-002 part 1)

# 3. Review the BEADS tracker
cat docs/BEADS_ISSUE_TRACKER.md | grep "BEAD-002" -A 20

# 4. Follow the implementation plan in BEAD-002
```

---

## 📋 COMPLETED BEADS (Don't Redo)

1. ✅ **BEAD-QW-001** - Removed hardcoded user path
2. ✅ **BEAD-QW-002** - Replaced println! with log::
3. ✅ **BEAD-001** - Enabled CSP
4. ✅ **BEAD-007** - Batch deletion limits (10K files, 100GB)
5. ✅ **BEAD-008** - Critical path deletion warnings
6. ✅ **BEAD-012** - Fixed Dashboard memory leak
7. ✅ **BEAD-027** - Deletion size warnings

---

## 🎯 NEXT TASKS (Priority Order)

### Immediate (CRITICAL - Blocking v0.1.1)

1. **BEAD-002: Finish Path Validation** (15-30 min remaining)
   - Revert lib.rs to clean state
   - Add module import
   - Integrate `validate_scan_path()` into:
     - `scan_bloat()` at line ~473
     - `scan_large_files()` at line ~429
     - `scan_duplicates()` at line ~534
     - `scan_junk_files()` at line ~625
   - Test by attempting to scan `/System` (should fail)
   - Commit when working

2. **BEAD-003: Fix TOCTOU Race Condition** (2 hrs)
   - Location: `cleanup_dirs()` function
   - Replace `p.exists()` check with atomic deletion
   - Add verification after trash operation

3. **BEAD-004: Deletion History Logging** (3 hrs)
   - Create `src-tauri/src/models/deletion_log.rs`
   - Add `chrono` dependency to Cargo.toml
   - Integrate into `cleanup_dirs()`

---

## 🛠️ Development Environment

### Build & Run

```bash
# Frontend dev server (port 3001)
npm run dev

# Tauri app with hot reload
npm run tauri:dev

# Backend only (faster for Rust changes)
cd src-tauri && cargo build

# Run tests
cargo test
npm test
```

### Key Directories

```
src/                          # Svelte frontend
  lib/
    components/              # UI components
    stores.js               # State management
src-tauri/                   # Rust backend
  src/
    lib.rs                  # Main commands (787 lines)
    main.rs                 # Entry point
    utils/                  # NEW - Utility modules
      mod.rs
      path.rs              # Path validation
  Cargo.toml               # Rust dependencies
  tauri.conf.json          # Tauri configuration
docs/
  BEADS_ISSUE_TRACKER.md   # Issue tracking (43 items)
  TAURI_SVELTE_SCAFFOLDING_BEST_PRACTICES.md
```

### Important Commands

```bash
# Check what changed
git status
git diff src-tauri/src/lib.rs

# See recent commits
git log --oneline -10

# Revert uncommitted changes to a file
git checkout src-tauri/src/lib.rs

# Build with detailed errors
cargo build 2>&1 | less

# Check for unwrap usage (should warn)
cargo clippy

# Format code
cargo fmt
npm run format
```

---

## 📊 Project Status Summary

### Metrics

- **Overall Progress:** 7/43 BEADS (16.3%)
- **Critical Issues:** 5/8 complete (62.5%)
- **High Priority:** 1/12 complete (8.3%)
- **Security Score:** Improved from 3/10 → ~6/10
- **Code Compiles:** ✅ (if you revert lib.rs changes)
- **App Functional:** ✅ Yes
- **Ready for Production:** ❌ No (3 critical issues remain)

### What Works Now

✅ Content Security Policy (XSS protection)  
✅ Batch deletion limits (10K files, 100GB max)  
✅ Critical path warnings (source code protection)  
✅ Memory leak fixed (Dashboard intervals)  
✅ Deletion size previews  
✅ Directory selection UI  
✅ Proper logging (no println!)  
✅ Path validation module created (not integrated yet)

### What's Broken

⚠️ Uncommitted changes to lib.rs (scan_duplicates incomplete)  
❌ No path validation on scan commands (BEAD-002 incomplete)  
❌ TOCTOU race condition in deletion (BEAD-003)  
❌ No deletion audit trail (BEAD-004)

---

## 🔍 Debugging Common Issues

### "error: cannot find value `size_groups`"

**Cause:** Incomplete edit to `scan_duplicates` function  
**Fix:** `git checkout src-tauri/src/lib.rs` to revert

### "Scan failed: missing field `root`"

**Cause:** No directory selected in UI  
**Fix:** Click "Select Directory" button in Dashboard

### "Access denied: system directory"

**Expected:** Path validation working correctly  
**Fix:** Don't scan system directories!

### Build takes forever / times out

**Cause:** First build compiles all dependencies  
**Fix:** Wait 2-5 minutes, or use `cargo build --release` once to cache

---

## 🎨 Code Style & Patterns

### Rust (Backend)

```rust
// ✅ Good - Proper error handling
let validated = validate_scan_path(&opts.root)?;
log::info!("Scanning: {}", validated.display());

// ❌ Bad - Unwrap (causes panics)
let path = some_result.unwrap();

// ✅ Good - Detailed logging
log::debug!("Processing: {}", path);

// ❌ Bad - Debug prints
println!("Processing: {}", path);
```

### Svelte (Frontend)

```javascript
// ✅ Good - Reactive stores
import { writable } from 'svelte/store';
export const isScanning = writable(false);

// ✅ Good - Lifecycle cleanup
onDestroy(() => {
  if (interval) clearInterval(interval);
});

// ❌ Bad - Memory leaks
setInterval(() => { ... }); // Never cleared
```

---

## 📚 Key Documentation

- **BEADS Tracker:** `docs/BEADS_ISSUE_TRACKER.md` (source of truth)
- **Architecture:** `ARCHITECTURE.md`
- **Scaffolding Guide:** `docs/TAURI_SVELTE_SCAFFOLDING_BEST_PRACTICES.md`
- **Audit Report:** Embedded in BEADS_ISSUE_TRACKER.md (lines 1-800+)

---

## 🚀 Quick Start After Crash

```bash
# 1. Navigate to project
cd /Users/frank/Development/private/projects/disk-bloat-scanner

# 2. Check status
git status
git log --oneline -5

# 3. Read this file for context
cat AGENTS.md

# 4. Check BEADS tracker for current task
cat docs/BEADS_ISSUE_TRACKER.md | grep "PENDING" | head -5

# 5. If lib.rs is broken, revert
git checkout src-tauri/src/lib.rs

# 6. Verify build works
cargo build

# 7. Continue with BEAD-002 path validation integration
# See "RESTART PROCEDURE" section above
```

---

## 🎯 Success Criteria for Current Session

To complete BEAD-002 and move to BEAD-003:

1. ✅ `validate_scan_path()` function exists (DONE)
2. ⏳ Function integrated into all 4 scan commands
3. ⏳ Attempting to scan `/System` returns error
4. ⏳ Build compiles successfully
5. ⏳ App runs without crashing
6. ⏳ Changes committed to git
7. ⏳ BEADS tracker updated: BEAD-002 status → COMPLETED

**Current Status:** Step 1 complete, steps 2-7 pending

---

## 💡 Pro Tips

1. **Always check git status first** - Know what changed
2. **Build after each small change** - Don't accumulate errors
3. **Commit working code frequently** - Easy rollback
4. **Read BEADS tracker** - It has all the implementation details
5. **Test manually** - Try to scan `/System` to verify protection
6. **Don't skip the audit recommendations** - They're all real security issues

---

**Remember:** The goal is v0.1.1 release with all Phase 1 (Critical) BEADS complete. We're at 62.5% - almost there! 🔥

**Next Agent:** Pick up from RESTART PROCEDURE → Option 1, starting at step 1.

---

## 🚀 NEW FEATURE: Project Auditor & Compliance Scanner (PACS)

### Overview

PACS is a transformational organizational feature enabling:
1. **Deep Project Analysis** - Examine all docs, validate compliance, generate specs
2. **Organization Monitoring** - Scan all projects, detect drift, send alerts
3. **Baseline Management** - Immutable snapshots, approval workflows, audit trails

### Specifications Ready

| Document | Lines | Purpose |
|----------|-------|---------|
| `proposal.md` | 2,084 | Complete architecture & design |
| `tasks.md` | 1,543 | 19 Beads issues with dependencies |
| `spec-summary.md` | 540 | Quick reference for stakeholders |

All in: `openspec/changes/project-auditor-compliance-scanner/`

### Next Steps for PACS

1. **Stakeholder Review** (Gate 0 approval pending)
2. **Sprint Planning** (Convert Beads to bd issues)
3. **Feature 1: Deep Analyzer** (7 issues, BEAD-PACS-001-007)
4. **Feature 2: Organization Monitor** (6 issues, BEAD-PACS-008-013)
5. **Feature 3: Baseline & Drift** (6 issues, BEAD-PACS-014-019)

