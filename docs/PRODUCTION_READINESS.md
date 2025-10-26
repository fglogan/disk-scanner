# Production Readiness Checklist - Disk Bloat Scanner v0.1.1

**Purpose:** Ensure zero data loss risk before internal deployment  
**Audience:** Dev team, QA, management  
**Last Updated:** October 26, 2025  
**Status:** READY TO IMPLEMENT

---

## Critical Safety Requirements (MUST HAVE)

### Path Protection (BLOCKING)
- [ ] Protected paths blocklist implemented (sys/usr/bin/Windows, .git, etc.)
- [ ] All paths validated before scan operation
- [ ] All paths validated before deletion operation
- [ ] Tests verify dangerous paths are rejected
- [ ] Rejection messages are clear and actionable

**Status:** See `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 1.1-1.2

### Audit Logging (BLOCKING)
- [ ] Audit logger records all deletion attempts
- [ ] Audit logger records all scan operations
- [ ] Immutable audit log file created
- [ ] Timestamp + size + status logged for each item
- [ ] Audit log location: `~/.disk-bloat-scanner/audit.log`
- [ ] Tests verify logging works and is accurate

**Status:** See `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 1.3

### Deletion Confirmation (BLOCKING)
- [ ] Deletion confirmation dialog shows ALL items to be deleted
- [ ] Confirmation shows exact file paths
- [ ] Confirmation shows total size in MB/GB
- [ ] 30-second countdown before confirmation allowed
- [ ] Checkbox required: "Yes, I understand this action"
- [ ] Cancel button always available (no timeout)
- [ ] Manual test: UI prevents accidental deletion

**Status:** See `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 2.1

### Trash-Only Deletion (BLOCKING)
- [ ] No permanent file deletion in v0.1
- [ ] 100% of deletions use system trash
- [ ] `req.trash` parameter mandatory
- [ ] Code explicitly checks `if req.trash { trash::delete(p) }`
- [ ] Fallback: if trash fails, error (don't delete permanently)
- [ ] Users can recover from trash for 30+ days

**Status:** See `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 1.4

### Error Handling (BLOCKING)
- [ ] Custom error type implemented (no generic String errors)
- [ ] All Rust errors typed with context
- [ ] Frontend receives structured error JSON
- [ ] Error messages displayed to user
- [ ] Errors don't cause silent failures
- [ ] All error paths tested

**Status:** See `docs/design/TAURI_ARCHITECTURE_MODERNIZATION.md` Part 1.2

---

## Code Quality Requirements

### Rust Code (v0.1.1)
- [ ] Zero unsafe `unwrap()` in command handlers
- [ ] All `.lock().unwrap()` replaced with error handling
- [ ] All `partial_cmp().unwrap()` replaced with safe comparison
- [ ] No `panic!()` in user-facing code
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test` passes 100%
- [ ] Documentation comments on all public functions

**Acceptance:** `cargo clippy --all-targets --all-features -- -D warnings`

### Frontend Code (v0.1.1)
- [ ] TypeScript strict mode enabled (`npm run check` passes)
- [ ] No `any` types in core logic
- [ ] All store types properly defined
- [ ] All UI components properly typed
- [ ] `npm test` passes 100%

**Acceptance:** `npm run check` and `npm test` both pass

### Test Coverage (v0.1.1)
- [ ] Path validation tested with dangerous paths
- [ ] Audit logging tested end-to-end
- [ ] Deletion confirmation UI tested manually
- [ ] Protected paths properly rejected
- [ ] Error scenarios handled gracefully

**Minimum:** 30% coverage on critical paths (safety features)

---

## Architecture Requirements

### Modular Design (v0.2.0 target, v0.1.1 preparation)
- [ ] Commands organized by domain (system, scan, cleanup)
- [ ] Services separate business logic from handlers
- [ ] Utils contain pure functions
- [ ] State management centralized
- [ ] Easy to add new panels/features
- [ ] Frontend-backend boundary clear (IPC only)

**Acceptance:** Code review by tech lead

### State Management (v0.1.1)
- [ ] AppState defined and used
- [ ] Tauri State<AppState> injected into commands
- [ ] Frontend stores reactive and typed
- [ ] API service encapsulates IPC calls

**Acceptance:** All commands use injected AppState

### IPC Communication (v0.1.1)
- [ ] All UI↔Core communication via `invoke()`
- [ ] No direct backend state access from frontend
- [ ] Commands are type-safe (serde)
- [ ] Error propagation structured (not strings)

**Acceptance:** No direct DB/file access from frontend code

---

## Security Requirements

### Path Validation (v0.1.1)
- [ ] Empty paths rejected
- [ ] Symlinks canonicalized (prevent loops)
- [ ] Directory traversal prevented
- [ ] System directories blocked
- [ ] Permission checks enforced
- [ ] Error messages don't leak sensitive info

**Test Cases:**
- `validate_scan_path("")` → Error
- `validate_scan_path("/System")` → Error (macOS)
- `validate_scan_path("C:\\Windows")` → Error (Windows)
- `validate_scan_path("~/test")` → OK (valid)

### Batch Operation Limits (v0.1.1)
- [ ] Max 10,000 files per deletion
- [ ] Max 100GB per deletion
- [ ] Size calculated before deletion
- [ ] Error returned if exceeds limit
- [ ] Clear error message with actual count/size

**Test Cases:**
- Deletion request with 15,000 paths → Error
- Deletion request with 150GB total → Error
- Deletion request with 9,999 paths / 99GB → OK

### Capability Isolation (v0.1.1)
- [ ] Tauri capabilities restrict unsafe operations
- [ ] CSP headers enabled
- [ ] Dialog plugin only for safe operations
- [ ] No network capability
- [ ] No shell execution

**Verification:** `cat src-tauri/tauri.conf.json` - check capabilities

---

## User Experience & Safety

### Warnings & Notices (v0.1.1)
- [ ] System protection notice always visible
- [ ] Deletion warning prominent and red
- [ ] Protection explanation included
- [ ] Recovery information provided
- [ ] Audit trail availability mentioned

**Manual Test:** Run app and verify messages appear

### Onboarding (v0.2.0 target)
- [ ] New users see safety guide first
- [ ] "I understand risk" required on first delete
- [ ] Protected paths blocklist explained
- [ ] Trash functionality described

### Documentation (v0.1.1)
- [ ] README warns about data loss potential
- [ ] SAFETY_FIRST_ARCHITECTURE.md in design folder
- [ ] User guide includes recovery steps
- [ ] Emergency procedures documented

**Files:**
- `README.md` - Safety section
- `docs/design/SAFETY_FIRST_ARCHITECTURE.md` - Complete
- `docs/TROUBLESHOOTING.md` - Recovery steps

---

## Testing Requirements

### Manual Testing (v0.1.1 - REQUIRED)
1. **Path Validation**
   - [ ] Attempt scan on `/System` → Blocked
   - [ ] Attempt scan on ~/Downloads → OK
   - [ ] Attempt scan on nonexistent path → Clear error
   - [ ] Attempt scan on symlink loop → Handled

2. **Deletion Confirmation**
   - [ ] Select 5 files → Preview shows all 5 with paths
   - [ ] Confirmation button disabled for 30 seconds
   - [ ] Can cancel anytime
   - [ ] Checkbox required to enable button

3. **Audit Logging**
   - [ ] Delete a file → Check audit.log
   - [ ] Verify timestamp + path + size + status
   - [ ] Delete 10 files → All logged
   - [ ] Verify format consistency

4. **Trash Functionality**
   - [ ] Delete file → Verify in system trash
   - [ ] Verify file recoverable from trash
   - [ ] Test on macOS, Windows, Linux

5. **Error Handling**
   - [ ] Delete read-only file → Clear error
   - [ ] Delete missing file → Skip with message
   - [ ] Delete permission-denied file → Skip with message

### Automated Testing (v0.1.1 - MINIMUM)
```bash
# Run all tests
cargo test --all
npm test

# Verify no unsafe patterns
cargo clippy --all-targets --all-features -- -D warnings

# Type check
npm run check
```

### Stress Testing (v0.1.1 - OPTIONAL)
- [ ] Delete 10,000 files (batched)
- [ ] Delete 100GB+ worth of files
- [ ] Rapid delete + undo + restore cycles
- [ ] Monitor for memory leaks

---

## Deployment Requirements

### Pre-Deployment (MANDATORY)
- [ ] All checklist items above completed
- [ ] Code reviewed by two team members
- [ ] Security review completed
- [ ] Manual testing documented and passed
- [ ] Audit log format verified
- [ ] Error messages user-tested for clarity

### Deployment Process
1. Create release branch: `release/v0.1.1-safety`
2. Tag version: `git tag v0.1.1`
3. Build binaries for all platforms
4. Test binaries on each platform
5. Create release notes with safety warnings
6. Announce to team with safety guidelines

### Post-Deployment Monitoring
- [ ] Check audit logs daily first week
- [ ] Verify no unexpected errors
- [ ] Collect team feedback
- [ ] Monitor for trash recovery issues
- [ ] Have rollback plan ready (older version)

---

## Rollback Procedures

**If critical issue found post-deployment:**

1. **Immediate:** Announce in Slack - STOP using app
2. **Within 5 min:** Rollback to previous version
3. **Within 1 hour:** Root cause analysis
4. **Within 24 hours:** Public post-mortem
5. **Within 1 week:** Fixed version released

**Rollback procedure:**
```bash
git checkout v0.1.0  # or last stable version
npm install
npm run tauri:build
# Distribute new binary
```

---

## Success Metrics

### Zero-Tolerance Items (CRITICAL)
- ✅ Zero unrecoverable data loss
- ✅ Zero permanent deletion in v0.1
- ✅ 100% audit trail accuracy
- ✅ 100% protected paths blocked

### Target Metrics (IMPORTANT)
- ✅ <1% false positives (wrong files in scan)
- ✅ >99% successful trash operation
- ✅ <5 second confirmation flow
- ✅ <5% user confusion based on team feedback

### Stretch Goals (NICE TO HAVE)
- ✅ Multi-language support
- ✅ Custom blocklist management
- ✅ Automated scheduled scans

---

## Sign-Off

**This app is production-ready for internal deployment when ALL blocking items are complete.**

| Role | Name | Date | Status |
|------|------|------|--------|
| Developer | — | — | ⏳ PENDING |
| Tech Lead | — | — | ⏳ PENDING |
| Security | — | — | ⏳ PENDING |
| QA | — | — | ⏳ PENDING |
| PM | — | — | ⏳ PENDING |

---

## Document References

- **Safety Architecture:** `docs/design/SAFETY_FIRST_ARCHITECTURE.md`
- **Tauri Architecture:** `docs/design/TAURI_ARCHITECTURE_MODERNIZATION.md`
- **Error Handling:** `docs/design/TAURI_ARCHITECTURE_MODERNIZATION.md` Part 1.2
- **Audit Logging:** `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 1.3
- **Confirmation UI:** `docs/design/SAFETY_FIRST_ARCHITECTURE.md` Part 2.1

---

**Last Review:** October 26, 2025  
**Next Review:** Upon Phase 1 completion (EDGS schedule)  
**Emergency Contact:** Tech Lead / Security Team

