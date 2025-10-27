# Disk Scanning Specification - Path Validation Modifications

## ADDED Requirements

### Requirement: Path Validation for Scan Operations

The system SHALL validate all scan source paths against a whitelist of safe system directories before initiating any scan operation. System critical paths (/, /System, /usr, /bin, /etc, /var/db, etc.) SHALL be rejected with a clear error message.

Path validation SHALL occur before any filesystem access to prevent accidental scanning of protected directories.

#### Scenario: System directory /System blocked

- **WHEN** user attempts to scan /System
- **THEN** scan fails immediately with error
- **AND** error message explains "Cannot scan system directory"
- **AND** no filesystem access occurs

#### Scenario: System directory /usr blocked

- **WHEN** user selects /usr for scan
- **THEN** path validation rejects the directory
- **AND** UI displays "System directory - not allowed"
- **AND** user can select a different path

#### Scenario: User directory allowed

- **WHEN** user selects ~/Documents for scan
- **THEN** path validation succeeds silently
- **AND** scan proceeds normally with progress updates
- **AND** results displayed when complete

#### Scenario: Invalid path handling

- **WHEN** path does not exist
- **THEN** appropriate error returned ("Path not found")
- **AND** user can retry with valid path

#### Scenario: Permission denied handling

- **WHEN** user lacks permission to read directory
- **THEN** graceful error: "Permission denied - check access rights"
- **AND** no crash or panic occurs

---

## MODIFIED Requirements

### Requirement: Disk Bloat Scanning

The system SHALL scan selected directories to identify large files and directories **with path validation before scan initiation**. Before any filesystem access, the system SHALL verify the scan path is safe and not a critical system directory.

#### Scenario: Path validation before bloat scan

- **WHEN** user initiates bloat scan for ~/Projects
- **THEN** path is validated successfully
- **AND** bloat scan proceeds with progress updates
- **AND** results displayed showing largest directories
- **AND** user can select items for cleanup

#### Scenario: Bloat scan with system directory rejected

- **WHEN** user attempts to scan /System for bloat
- **THEN** path validation fails before scan starts
- **AND** clear error message displayed
- **AND** no filesystem traversal occurs

---

## Notes for Implementation

### Safe Directories (Whitelist Approach)

The system SHALL use a default whitelist of safe roots that users can scan:
- Home directory (~/)
- Desktop
- Downloads
- Documents
- Projects/development directories
- External drives/mounts (when explicitly selected)

### Dangerous Directories (Blacklist)

The system SHALL block these directories:
- Root (/)
- System directories (/System, /Library, /usr, /bin, /sbin, /opt)
- Configuration (/etc, /var, /private)
- Boot-critical (boot loaders, kernels)
- Windows equivalents (C:\Windows, C:\Program Files, etc.)
- Linux equivalents (/etc, /sys, /proc, /boot)

### Error Handling

When validation fails:
1. Return specific error indicating "system directory"
2. Suggest alternative paths
3. Never proceed with scan
4. Log rejection attempt (for debugging)
5. Provide helpful UI message

### Performance

- Path validation must complete in <1ms
- No network access during validation
- Use OS-provided path functions
- Cache results if needed for repeated scans
