# OpenSpec Change Templates for EDGS Phases

This document provides ready-to-use templates for creating OpenSpec changes during each EDGS phase.

---

## Phase 1: Critical Stability

### Change: add-path-validation

**Purpose:** Prevent accidental scanning/deletion of system directories

**proposal.md:**
```markdown
## Why
Users could accidentally scan or delete system-critical directories 
(/System, /usr, /bin, etc.), breaking their operating system. We need 
to validate all scan paths against a whitelist of safe directories.

## What Changes
- Add path validation utility module (src-tauri/src/utils/path.rs)
- Create whitelist of system-critical paths
- Validate all scan operations before execution
- Display warnings for critical directories
- BREAKING: No (purely additive safety)

## Impact
- Affected specs: disk-scanning, cleanup-operations
- Affected code: src-tauri/src/lib.rs, src-tauri/src/utils/path.rs
- User impact: Additional safety check, no behavior change
- Phase: EDGS Phase 1 (Critical Stability - P1-T2)
- OpenSpec: add-path-validation
```

**tasks.md:**
```markdown
## 1. Backend Implementation
- [ ] 1.1 Create src-tauri/src/utils/mod.rs (module manifest)
- [ ] 1.2 Create src-tauri/src/utils/path.rs with validation logic
- [ ] 1.3 Implement validate_scan_path() function with whitelist
- [ ] 1.4 Add error type for validation failures
- [ ] 1.5 Test validation against /System, /usr, /bin (should fail)
- [ ] 1.6 Test validation against ~/Documents (should succeed)

## 2. Integration
- [ ] 2.1 Import validate_scan_path in src-tauri/src/lib.rs
- [ ] 2.2 Integrate into scan_bloat() command
- [ ] 2.3 Integrate into scan_duplicates() command
- [ ] 2.4 Integrate into scan_junk_files() command
- [ ] 2.5 Integrate into cleanup_dirs() command

## 3. Testing
- [ ] 3.1 Add unit tests in src-tauri/src/utils/path.rs
- [ ] 3.2 Add integration tests in src-tauri/tests/
- [ ] 3.3 Manual test: deny /System access
- [ ] 3.4 Manual test: allow ~/Downloads access

## 4. Validation
- [ ] 4.1 cargo clippy --all-targets -- -D warnings (zero warnings)
- [ ] 4.2 cargo test (all tests pass)
- [ ] 4.3 cargo fmt (formatting check)
- [ ] 4.4 npm test (frontend tests pass if any)

## 5. Documentation
- [ ] 5.1 Update openspec/specs/disk-scanning/spec.md with MODIFIED requirement
- [ ] 5.2 Add rustdoc comments to pub functions in path.rs
```

**specs/disk-scanning/spec.md:**
```markdown
## ADDED Requirements

### Requirement: Path Validation for Scan Operations
The system SHALL validate all scan source paths against a whitelist 
of safe system directories before initiating any scan operation. 
System critical paths (/, /System, /usr, /bin, /etc, /var/db, etc.) 
SHALL be rejected with a clear error message.

#### Scenario: System directory blocked
- **WHEN** user attempts to scan /System
- **THEN** scan fails with error "Cannot scan system directory"
- **AND** UI displays warning about restricted path

#### Scenario: User directory allowed
- **WHEN** user selects ~/Documents for scan
- **THEN** path validation succeeds
- **AND** scan proceeds normally

#### Scenario: Invalid path error handling
- **WHEN** validation encounters permission denied
- **THEN** operation fails gracefully with error
- **AND** user can select a different path

## MODIFIED Requirements

### Requirement: Disk Bloat Scanning
The system SHALL scan selected directories to identify **[previously specified behavior]** 
**and validate scan paths against system whitelist before execution**.

#### Scenario: Validated bloat scan
- **WHEN** user initiates bloat scan for ~/Projects
- **THEN** path is validated successfully
- **AND** scan proceeds with progress updates
- **AND** results displayed when complete

[Include other existing scenarios from current spec.md]
```

---

## Phase 2: Architecture Refactoring

### Change: modularize-backend

**Purpose:** Decompose lib.rs into focused modules for maintainability

**proposal.md:**
```markdown
## Why
lib.rs has grown to 1200+ lines, mixing concerns: commands, error handling, 
models, patterns, and utility logic. Modularization improves testability, 
reusability, and maintainability.

## What Changes
- Create src-tauri/src/error.rs (custom error type)
- Create src-tauri/src/models.rs (data structures)
- Create src-tauri/src/utils/patterns.rs (detection patterns)
- Create src-tauri/src/utils/scan.rs (scan logic)
- Create src-tauri/src/utils/cleanup.rs (cleanup operations)
- Refactor lib.rs to import from modules
- lib.rs reduced from 1200→300 lines
- BREAKING: No (internal refactor)

## Impact
- Affected specs: All scanning/cleanup operations
- Affected code: src-tauri/src/lib.rs + 5 new modules
- Code improvement: 60% reduction in main file
- Phase: EDGS Phase 2 (Architecture - P2-001 through P2-007)
- OpenSpec: modularize-backend
```

**tasks.md:**
```markdown
## 1. Create Custom Error Type
- [ ] 1.1 Create src-tauri/src/error.rs
- [ ] 1.2 Define ScannerError enum with error variants
- [ ] 1.3 Implement Display + Error traits
- [ ] 1.4 Implement From<std::io::Error>
- [ ] 1.5 Test error conversions

## 2. Extract Data Models
- [ ] 2.1 Create src-tauri/src/models.rs
- [ ] 2.2 Move DiskInfo struct + serde derives
- [ ] 2.3 Move FileInfo, DuplicateGroup structs
- [ ] 2.4 Move all serde derives
- [ ] 2.5 Test model serialization

## 3. Extract Patterns Module
- [ ] 3.1 Create src-tauri/src/utils/patterns.rs
- [ ] 3.2 Move BLOAT_PATTERNS constant
- [ ] 3.3 Move JUNK_PATTERNS constant
- [ ] 3.4 Move detect_bloat_category() function
- [ ] 3.5 Move detect_junk_file() function
- [ ] 3.6 Add unit tests for pattern matching

## 4. Extract Scan Logic
- [ ] 4.1 Create src-tauri/src/utils/scan.rs
- [ ] 4.2 Move dir_size() function
- [ ] 4.3 Move hash_file() function
- [ ] 4.4 Move scan logic helpers
- [ ] 4.5 Add 20+ unit tests

## 5. Extract Cleanup Logic
- [ ] 5.1 Create src-tauri/src/utils/cleanup.rs
- [ ] 5.2 Move cleanup_dirs() logic
- [ ] 5.3 Move path validation calls
- [ ] 5.4 Move trash operations
- [ ] 5.5 Add tests for cleanup operations

## 6. Refactor lib.rs
- [ ] 6.1 Create src-tauri/src/utils/mod.rs (manifest)
- [ ] 6.2 Update lib.rs imports from modules
- [ ] 6.3 Update all command handlers to use new modules
- [ ] 6.4 Verify lib.rs < 300 lines
- [ ] 6.5 Verify no circular imports

## 7. Testing
- [ ] 7.1 cargo test --lib (unit tests)
- [ ] 7.2 cargo test --test integration_tests
- [ ] 7.3 cargo clippy --all-targets -- -D warnings
- [ ] 7.4 cargo tarpaulin (coverage >40%)

## 8. Documentation
- [ ] 8.1 Update specs with modularized structure
- [ ] 8.2 Add rustdoc to module files
```

**specs/cleanup-operations/spec.md:**
```markdown
## MODIFIED Requirements

### Requirement: Safe File Cleanup
The system SHALL move selected files to system trash 
**using modularized cleanup logic in src-tauri/src/utils/cleanup.rs** 
instead of inline implementation in lib.rs.

[Rest of requirement unchanged]
```

---

## Phase 3: Frontend Modernization

### Change: migrate-stores-typescript

**Purpose:** Convert stores.js to stores.ts with full type safety

**proposal.md:**
```markdown
## Why
Frontend stores are currently in JavaScript, losing type safety and IDE support. 
Migrating to TypeScript enables compiler-checked state management and better 
developer experience.

## What Changes
- Rename src/lib/stores.js → src/lib/stores.ts
- Add TypeScript interfaces for all stores
- Type all writable() calls
- Update imports across components
- Add prop validation to 4 major components
- BREAKING: Import path changes from .js → .ts

## Impact
- Affected specs: All frontend state management
- Affected code: src/lib/stores.ts + 12 component files
- Type coverage: 40%→95%
- Phase: EDGS Phase 3 (Frontend - P3-001 through P3-007)
- OpenSpec: migrate-stores-typescript
```

---

## Phase 5: Testing Expansion

### Change: expand-test-coverage

**Purpose:** Increase test coverage from 5% → 50%+ across backend + frontend

**proposal.md:**
```markdown
## Why
Current test coverage is minimal (5%). High coverage reduces regression risks, 
improves code quality, and enables confident refactoring. Target: >50% combined 
backend + frontend coverage.

## What Changes
- Add 40+ unit tests for Rust backend (patterns, scan, cleanup)
- Add 20+ component tests for Svelte frontend
- Expand integration tests to 8+ scenarios
- Add E2E test skeleton (WebDriverIO setup)
- BREAKING: No

## Impact
- Affected specs: All capabilities
- Affected code: Tests in src-tauri/tests/ and src/test/
- Coverage: 5%→50%+ 
- Phase: EDGS Phase 5 (Testing - P5-001 through P5-008)
- OpenSpec: expand-test-coverage
```

---

## Generic Template: New Feature

Use this template for any new feature addition.

**proposal.md:**
```markdown
## Why
[Problem or opportunity being addressed]

## What Changes
- [Feature 1]
- [Feature 2]
- [**BREAKING**: if applicable]

## Impact
- Affected specs: [list]
- Affected code: [key files]
- User impact: [new behavior]
- Phase: EDGS Phase X (Phase Name - P[X]-T[Y])
- OpenSpec: [change-id]
```

**tasks.md:**
```markdown
## 1. Design & Planning
- [ ] 1.1 Review spec requirements
- [ ] 1.2 Plan implementation approach
- [ ] 1.3 Identify test cases

## 2. Backend Implementation (if needed)
- [ ] 2.1 [Implement component 1]
- [ ] 2.2 [Implement component 2]
- [ ] 2.3 Add unit tests

## 3. Frontend Implementation (if needed)
- [ ] 3.1 [Create component 1]
- [ ] 3.2 [Create component 2]
- [ ] 3.3 Add component tests

## 4. Integration & Testing
- [ ] 4.1 Integration tests
- [ ] 4.2 Manual testing
- [ ] 4.3 Verify acceptance criteria

## 5. Documentation
- [ ] 5.1 Update spec with ADDED/MODIFIED requirements
- [ ] 5.2 Update design docs if needed
- [ ] 5.3 Add code comments
```

**specs/[capability]/spec.md:**
```markdown
## ADDED Requirements

### Requirement: [Feature Name]
The system SHALL [behavior description]

#### Scenario: [Success case]
- **WHEN** [user action]
- **THEN** [expected result]

#### Scenario: [Edge case]
- **WHEN** [edge condition]
- **THEN** [expected result]

## MODIFIED Requirements

### Requirement: [Existing requirement if changed]
[Full updated requirement]

[Include scenarios]
```

---

## Quick Reference: Change ID Naming

Use kebab-case, verb-led prefixes:

- `add-[feature]` - New capability
- `update-[feature]` - Modification to existing
- `remove-[feature]` - Remove deprecated feature
- `refactor-[component]` - Internal restructuring
- `fix-[issue]` - Bug fix (usually no spec needed)
- `migrate-[old][-to-new]` - Technology migration

**Examples:**
- `add-path-validation`
- `update-duplicate-detection-algorithm`
- `remove-deprecated-api`
- `refactor-scan-engine`
- `migrate-stores-typescript`

---

## Creating a Change: Step-by-Step

```bash
# 1. Decide on change ID
CHANGE="add-path-validation"

# 2. Create directory structure
mkdir -p openspec/changes/$CHANGE/{specs/disk-scanning,specs/cleanup-operations}

# 3. Create files from templates above
cat > openspec/changes/$CHANGE/proposal.md << 'EOF'
[proposal.md content from template]
EOF

cat > openspec/changes/$CHANGE/tasks.md << 'EOF'
[tasks.md content from template]
EOF

cat > openspec/changes/$CHANGE/specs/disk-scanning/spec.md << 'EOF'
[spec.md content with ADDED/MODIFIED]
EOF

# 4. Validate
openspec validate $CHANGE --strict

# 5. Create bd issue
bd create "P1-T2: Add path validation (openspec: $CHANGE)" \
  -p 0 -t task

# 6. Implement per tasks.md
# 7. Commit with phase reference
git commit -m "feat: add path validation (EDGS Phase 1: P1-T2)

- Refs: openspec/changes/add-path-validation/
- Implements all requirements per spec"

# 8. Archive after merge
openspec archive $CHANGE --yes
```

---

## Validation Checklist

Before requesting approval:

- [ ] proposal.md explains why, what, impact
- [ ] tasks.md has detailed, ordered checklist
- [ ] specs/[capability]/spec.md has ADDED/MODIFIED sections
- [ ] Every requirement has at least one scenario
- [ ] Scenarios use `#### Scenario:` format (4 hashtags)
- [ ] Scenarios have WHEN/THEN bullets
- [ ] `openspec validate [change-id] --strict` passes
- [ ] bd issue created with OpenSpec link
- [ ] Phase dependencies documented

---

## Resources

- **OpenSpec AGENTS.md**: `openspec/AGENTS.md` (full spec guide)
- **OpenSpec project.md**: `openspec/project.md` (conventions)
- **EDGS Schedule**: `docs/schedules/EDGS_SCHEDULE.md`
- **Integration Guide**: `docs/OPENSPEC_BD_EDGS_INTEGRATION.md` (this)

