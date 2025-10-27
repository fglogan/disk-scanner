# Add Path Validation for Scan Operations

## Why

Users could accidentally scan or delete system-critical directories (/, /System, /usr, /bin, /etc, /var/db, etc.) that would break their operating system. We need a path validation mechanism to:

1. Prevent accidental scans of protected system directories
2. Provide clear warnings before dangerous operations
3. Implement a whitelist of safe scan roots
4. Protect users from data loss

This is a critical safety feature that must be in place before v0.2.0 release.

## What Changes

- **Add module:** Create `src-tauri/src/utils/path.rs` with path validation logic
- **Create whitelist:** Define safe directories and blacklist system directories
- **Integrate validation:** Call validation in all scan and cleanup commands
- **Add tests:** Unit tests for path validation logic
- **Documentation:** Update specs with path validation requirements
- **BREAKING:** No (purely additive safety feature)

## Impact

### Affected Specs
- `disk-scanning` (scan operations must validate paths)
- `cleanup-operations` (deletion operations must validate paths)

### Affected Code
- `src-tauri/src/lib.rs` (all scan command handlers)
- `src-tauri/src/utils/path.rs` (NEW - validation module)
- `src-tauri/src/utils/mod.rs` (NEW - module manifest)

### User Impact
- **Positive:** Protection from accidental system damage
- **Neutral:** Minimal performance impact
- **Behavior Change:** None (feature is additive)

### Phase Information
- **Phase:** EDGS Phase 1 (Critical Stability)
- **Task:** P1-T2
- **OpenSpec ID:** add-path-validation
- **Priority:** Critical (blocks v0.2.0 release)

## Success Criteria

✅ Path validation module compiles without warnings  
✅ Integration into all scan commands complete  
✅ Unit tests pass (system paths blocked, user paths allowed)  
✅ Manual testing confirms protection works  
✅ Zero clippy warnings  
✅ Documentation updated  

## References

- Specification: `openspec/specs/disk-scanning/spec.md` (MODIFIED)
- Specification: `openspec/specs/cleanup-operations/spec.md` (MODIFIED)
- Implementation: `docs/OPENSPEC_CHANGE_TEMPLATES.md` (Phase 1 section)
- Testing: `docs/design/SAFETY_FIRST_ARCHITECTURE.md`
