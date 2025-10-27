# Implementation Tasks: Add Path Validation

## 1. Create Path Validation Module

- [ ] 1.1 Create `src-tauri/src/utils/mod.rs` (module manifest)
- [ ] 1.2 Create `src-tauri/src/utils/path.rs` (validation logic)
- [ ] 1.3 Define SYSTEM_BLACKLIST constant (system directories)
- [ ] 1.4 Define USER_WHITELIST constant (allowed safe roots)
- [ ] 1.5 Implement `validate_scan_path(path: &Path) -> Result<(), String>`
- [ ] 1.6 Implement `is_system_path(path: &Path) -> bool`
- [ ] 1.7 Add detailed error messages for blocked paths

## 2. Update lib.rs Integration

- [ ] 2.1 Import `validate_scan_path` from utils module
- [ ] 2.2 Call validation in `scan_bloat()` command handler
- [ ] 2.3 Call validation in `scan_duplicates()` command handler
- [ ] 2.4 Call validation in `scan_junk_files()` command handler
- [ ] 2.5 Call validation in `scan_large_files()` command handler
- [ ] 2.6 Call validation in `cleanup_dirs()` command handler
- [ ] 2.7 Return meaningful error messages to frontend

## 3. Unit Tests

- [ ] 3.1 Test: System directory /System blocked
- [ ] 3.2 Test: System directory /usr blocked
- [ ] 3.3 Test: System directory /bin blocked
- [ ] 3.4 Test: User directory ~/Documents allowed
- [ ] 3.5 Test: User directory ~/Downloads allowed
- [ ] 3.6 Test: Invalid path error handling
- [ ] 3.7 Test: Permission denied graceful handling
- [ ] 3.8 Add tests in `src-tauri/src/utils/path.rs`

## 4. Integration Testing

- [ ] 4.1 Manual test: Attempt scan of /System (should fail)
- [ ] 4.2 Manual test: Attempt scan of /usr (should fail)
- [ ] 4.3 Manual test: Attempt scan of /bin (should fail)
- [ ] 4.4 Manual test: Scan ~/Documents (should succeed)
- [ ] 4.5 Manual test: Cleanup operation validation works
- [ ] 4.6 Verify error messages are clear and helpful
- [ ] 4.7 Test on macOS, Windows (if available), Linux (if available)

## 5. Code Quality

- [ ] 5.1 Format code: `cargo fmt`
- [ ] 5.2 Lint code: `cargo clippy --all-targets -- -D warnings`
- [ ] 5.3 Build succeeds: `cargo build --release`
- [ ] 5.4 All tests pass: `cargo test`
- [ ] 5.5 Add rustdoc comments to public functions
- [ ] 5.6 Zero compiler warnings

## 6. Documentation & Specs

- [ ] 6.1 Update `openspec/specs/disk-scanning/spec.md` with MODIFIED section
- [ ] 6.2 Update `openspec/specs/cleanup-operations/spec.md` with MODIFIED section
- [ ] 6.3 Add scenarios for path validation in both specs
- [ ] 6.4 Validate specs with `openspec validate add-path-validation --strict`
- [ ] 6.5 Document safe/unsafe paths in design docs

## 7. Final Validation

- [ ] 7.1 Code review: Check for edge cases
- [ ] 7.2 Security review: Verify whitelist/blacklist logic sound
- [ ] 7.3 Performance: Path validation overhead acceptable
- [ ] 7.4 Commit message includes EDGS Phase 1 reference
- [ ] 7.5 All deliverables present

## Completion Criteria

✅ Path validation module compiles  
✅ All scan commands integrated with validation  
✅ All unit tests pass  
✅ Manual testing confirms protection works  
✅ Zero clippy warnings  
✅ Specs updated and validated  
✅ Ready for Code Review  

## Notes

- **Safety First:** Block system directories aggressively (better to block legitimate paths than allow system damage)
- **User Experience:** Provide clear error messages explaining why a path is blocked
- **Testing:** Test against both relative and absolute paths
- **Performance:** Path validation should be O(1) or O(log n) at worst
- **Documentation:** Every public function needs rustdoc comments
