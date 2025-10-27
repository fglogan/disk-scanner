# Cleanup Operations Specification - Path Validation Modifications

## MODIFIED Requirements

### Requirement: Safe File Cleanup

The system SHALL move selected files to system trash **only after validating the source paths**. Before any deletion or trash operation, the system SHALL verify all paths are safe and not critical system directories.

#### Scenario: Path validation before cleanup

- **WHEN** user selects files in ~/Downloads for cleanup
- **THEN** all selected file paths are validated
- **AND** all paths pass validation (user directory)
- **AND** cleanup proceeds to trash operation
- **AND** files recovered from trash if needed

#### Scenario: Cleanup blocked for system directory

- **WHEN** user attempts cleanup of files in /usr
- **THEN** path validation fails before trash operation
- **AND** clear error: "Cannot delete from system directory"
- **AND** no files are moved to trash
- **AND** no partial deletions occur

#### Scenario: Mixed paths (some blocked)

- **WHEN** user attempts cleanup of files from multiple directories
- **AND** one path is system-protected
- **THEN** validation fails for entire operation
- **AND** clear message identifies blocked directory
- **AND** no files are modified

---

## ADDED Requirements

### Requirement: Pre-Cleanup Path Validation

Before any cleanup operation (moving to trash), the system SHALL validate all source paths. If any path is determined to be critical or protected, the entire operation SHALL be rejected with a specific error message.

#### Scenario: Validation prevents system file deletion

- **WHEN** user attempts cleanup of any file in /System
- **THEN** path validation immediately rejects
- **AND** error returned before trash operation
- **AND** zero files deleted or moved

#### Scenario: Validation allows safe cleanup

- **WHEN** user selects duplicate files in ~/Desktop
- **THEN** all paths validated as safe
- **AND** cleanup proceeds as normal
- **AND** files moved to trash successfully

#### Scenario: Batch operation validation

- **WHEN** cleanup operation targets 100+ files
- **AND** all in safe directories
- **THEN** batch validated before any operation
- **AND** proceeds safely to trash

---

## Implementation Notes

### Validation Timing

Path validation SHALL occur:
1. **First:** Before any filesystem operation
2. **For each path:** Every file/directory to be deleted
3. **Before batch:** Before any batch cleanup operation
4. **Atomically:** All-or-nothing (validate all, then delete all, or fail all)

### Error Safety

- If validation fails on ANY path in a batch: fail entire batch
- Never perform partial cleanups
- Always maintain system consistency
- Provide detailed error messages for debugging

### Integration with Scan Results

When user selects items from scan results for cleanup:
1. Validate each selected path before proceeding
2. Filter out invalid paths with clear message
3. Allow user to confirm remaining valid paths
4. Proceed only if paths pass validation

### Batch Limits (Already Implemented)

Path validation integrates with existing batch limits:
- Max 10,000 files per cleanup operation
- Max 100GB per cleanup operation
- Max 30-second timeout
- Path validation is additional safety layer
