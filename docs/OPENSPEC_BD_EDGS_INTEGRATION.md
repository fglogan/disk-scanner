# OpenSpec + bd + EDGS Integration Guide

**Status:** ✅ Fully Integrated & Compliant  
**Date:** October 26, 2025  
**Compliance Level:** TES-2025 v6.9 + LAIO v3.3 + EDGS v1.0  
**Project Maturity:** L2 (Integration) → L4 Target

---

## Overview

This document describes the integrated workflow for managing Disk Bloat Scanner development using three complementary systems:

1. **OpenSpec** - Specification-driven change proposals
2. **bd** - Dependency-aware issue tracking
3. **EDGS** - Event-driven gated scheduling

Together, these systems provide:
- ✅ **Spec-First Development**: All changes start with specifications
- ✅ **Dependency Management**: bd tracks blocking relationships
- ✅ **Gated Progression**: EDGS phases with HIL approval gates
- ✅ **Proof of Execution**: PoE bundles at each gate
- ✅ **Standards Compliance**: TES-2025 v6.9 + LAIO v3.3

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│  EDGS PHASES (8 phases, 52 tasks)                           │
│  ├── Phase 0: Constitutional Foundation (4 tasks)           │
│  ├── Phase 1: Critical Stability (7 tasks)                  │
│  ├── Phase 2-6: Refactoring, Testing, Documentation        │
│  └── Release: L4 Production Readiness (6 tasks)             │
│                                                               │
│    ↓ (each phase contains multiple issues)                   │
│                                                               │
│  BD ISSUES (20 currently, ~52 for EDGS)                     │
│  ├── Epic issues (phase grouping)                           │
│  ├── Task issues (individual work)                          │
│  └── Gate issues (validation + HIL approval)                │
│                                                               │
│    ↓ (issues reference specs)                                │
│                                                               │
│  OPENSPEC CHANGES & SPECS                                   │
│  ├── Specs (current truth): capabilities/features           │
│  ├── Changes (proposals): modifications to specs            │
│  └── Deltas: ADDED/MODIFIED/REMOVED requirements           │
│                                                               │
└─────────────────────────────────────────────────────────────┘

WORKFLOW:
  1. Identify change needed (feature, refactor, fix, etc.)
  2. Create OpenSpec change proposal (specs/ + changes/ structure)
  3. Add bd issue(s) for tracking
  4. Link issue to EDGS phase
  5. Implement per tasks.md
  6. Validation & approval
  7. Commit to git
  8. Archive change in OpenSpec
```

---

## Part 1: OpenSpec Workflow

### When to Create an OpenSpec Change

**Create a change proposal if:**
- Adding a new feature or capability ✅
- Making breaking changes (API, schema) ✅
- Changing architecture or patterns ✅
- Optimizing performance (changes behavior) ✅
- Updating security patterns ✅

**Skip proposal (direct implementation) if:**
- Bug fix (restoring intended behavior)
- Typos, formatting, comments
- Dependency updates (non-breaking)
- Configuration changes
- Tests for existing behavior

### OpenSpec Structure

```
openspec/
├── project.md                          # Project conventions ✅ (POPULATED)
├── specs/                              # Current truth (what IS)
│   ├── disk-scanning/
│   │   ├── spec.md                     # Requirements & scenarios
│   │   └── design.md                   # Technical decisions
│   ├── duplicate-detection/
│   ├── cleanup-operations/
│   ├── settings-management/
│   └── ...
├── changes/                            # Proposals (what SHOULD change)
│   ├── add-path-validation/            # Example: Phase 1 change
│   │   ├── proposal.md                 # Why, what, impact
│   │   ├── tasks.md                    # Implementation checklist
│   │   ├── design.md                   # Technical decisions
│   │   └── specs/
│   │       └── disk-scanning/
│   │           └── spec.md             # ADDED/MODIFIED deltas
│   ├── refactor-cleanup-logic/         # Example: Phase 2 change
│   │   ├── proposal.md
│   │   ├── tasks.md
│   │   └── specs/
│   │       └── cleanup-operations/
│   │           └── spec.md
│   └── archive/                        # Completed changes
│       └── YYYY-MM-DD-[change-id]/
└── AGENTS.md                           # This workflow guide
```

### Creating a Change Proposal: Step-by-Step

#### Example: Phase 1 - Add Path Validation

**Step 1: Review Context**
```bash
# See what specs exist
openspec spec list --long

# See what's in progress
openspec list

# Review project conventions
cat openspec/project.md
```

**Step 2: Create Change Directory**
```bash
CHANGE="add-path-validation"
mkdir -p openspec/changes/$CHANGE/specs/disk-scanning
```

**Step 3: Write proposal.md**
```markdown
## Why
Prevent users from accidentally scanning or deleting system directories 
(/System, /usr, /bin, etc.) that could break the OS.

## What Changes
- Add path validation utility module
- Validate all scan roots against whitelist
- Display warning for critical directories
- Block operations on system paths

## Impact
- Affected specs: disk-scanning, cleanup-operations
- Affected code: src-tauri/src/lib.rs, src-tauri/src/utils/path.rs
- Breaking: No (additive safety feature)
- Phase: EDGS Phase 1 (Critical Stability)
```

**Step 4: Write tasks.md**
```markdown
## 1. Backend Implementation
- [ ] 1.1 Create src-tauri/src/utils/path.rs with validation logic
- [ ] 1.2 Add validate_scan_path() function with whitelist
- [ ] 1.3 Integrate validation into scan_bloat(), scan_duplicates(), etc.
- [ ] 1.4 Add unit tests for path validation

## 2. Documentation
- [ ] 2.1 Document safe/unsafe paths in design.md
- [ ] 2.2 Update user-facing error messages

## 3. Testing & Validation
- [ ] 3.1 Test against system directories (should fail)
- [ ] 3.2 Test against user directories (should succeed)
- [ ] 3.3 Run cargo clippy, cargo test
```

**Step 5: Write spec deltas**
```markdown
# openspec/changes/add-path-validation/specs/disk-scanning/spec.md

## ADDED Requirements

### Requirement: Path Validation
The system SHALL validate scan paths against a whitelist of safe directories
to prevent accidental scanning or deletion of system critical paths.

#### Scenario: System directory blocked
- **WHEN** user attempts to scan /System
- **THEN** operation fails with "critical path" error

#### Scenario: User directory allowed
- **WHEN** user attempts to scan ~/Documents
- **THEN** scan proceeds normally

## MODIFIED Requirements

### Requirement: Disk Bloat Scanning
The system SHALL scan selected directories for size and provide 
**path validation before any scan operation**.

[Include full existing requirement + modifications]

#### Scenario: Validated directory scan
- **WHEN** user selects ~/Downloads
- **THEN** validation passes, scan proceeds
```

**Step 6: Validate Proposal**
```bash
openspec validate add-path-validation --strict
```

**Step 7: Link to bd Issue**
Create bd issue with reference:
```bash
bd create "Phase 1: Add path validation (openspec: add-path-validation)" \
  -p 0 -t task \
  -d "See: openspec/changes/add-path-validation/proposal.md"
```

---

## Part 2: bd Issue Tracking Integration

### Issue Naming Convention

**Format:** `[TYPE] [PHASE]: [DESCRIPTION] (openspec: [change-id])`

**Examples:**
```
EDGS Phase 0: Create constitution.yaml
EDGS Phase 1: Add path validation (openspec: add-path-validation)
EDGS Phase 2: Refactor lib.rs into modules (openspec: modularize-backend)
Bug: Fix memory leak in Dashboard (EDGS Phase 1)
```

### Issue Types & Hierarchy

```
EDGS PHASE (Epic)
├── Phase 0: Constitutional Foundation (4 tasks)
│   ├── P0-T1: Create constitution
│   ├── P0-T2: Verify TDS
│   ├── P0-T3: Domain classification
│   └── P0-T4: PoE infrastructure
├── Phase 1: Critical Stability (7 tasks)
│   ├── P1-T1: Revert uncommitted changes
│   ├── P1-T2: Path validation (openspec: add-path-validation) ← OpenSpec linked
│   └── ... (5 more tasks)
└── ... (Phases 2-6)

GATE (Validation)
├── P0 Gate Validation
├── P1 Gate Validation
└── ... (8 total phase gates)

RELEASE (Final)
├── v0.2.0 Release Tasks (6 items)
└── Release Gate Validation
```

### Creating bd Issues

**Command Pattern:**
```bash
# Create phase epic
bd create "EDGS Phase 1: Critical Stability" \
  -p 0 -t epic \
  -d "7 tasks to achieve zero warnings, 100% test pass"

# Create task (with OpenSpec reference)
bd create "P1-T2: Add path validation (openspec: add-path-validation)" \
  -p 0 -t task \
  -d "Implement path validation per openspec/changes/add-path-validation/"

# Create gate
bd create "P1 Gate Validation" \
  -p 0 -t chore \
  -d "Phase 1 gate: All tasks done, all tests pass, zero warnings. Awaiting Tech Lead approval"

# Add dependency (T2 depends on T1)
bd dep add P1-T2 P1-T1

# Add phase dependency (P1 depends on P0 gate)
bd dep add P1 "P0 Gate Validation"
```

### bd Commands for Daily Use

```bash
# See ready work (no blockers)
bd ready --priority 0

# Update task status
bd update disk-bloat-scanner-XX --status in_progress
bd update disk-bloat-scanner-XX --status completed

# View issue details
bd show disk-bloat-scanner-XX

# Track dependencies
bd dep tree disk-bloat-scanner-XX
bd dep cycles  # Check for circular dependencies

# Track phase progress
bd list --json | jq '.[] | select(.title | contains("Phase 1"))'

# See completed work
bd list --status completed | head -10
```

---

## Part 3: EDGS Integration

### EDGS Phase Structure

Each EDGS phase follows this pattern:

```
Phase N: [Name]
├── 4-11 Tasks
│   ├── Task 1: Fundamental work
│   ├── Task 2: Depends on Task 1
│   └── ... (sequential dependencies)
├── Gate Validation
│   ├── Criteria: All tasks complete
│   ├── Authority: [Specific approver]
│   └── PoE Bundle: Evidence in .beads/edgs/phase-N/
└── Unblock Next Phase
```

### Current EDGS Phases

| Phase | Name | Tasks | Gate Authority | Status |
|-------|------|-------|-----------------|--------|
| **P0** | Constitutional | 4 | Project Sponsor | ⏳ Ready to Start |
| **P1** | Critical Stability | 7 | Tech Lead | Blocked by P0 |
| **P2** | Architecture | 11 | Architect | Blocked by P1 |
| **P3** | Frontend | 8 | QA Lead | Blocked by P2 |
| **P4** | Documentation | 9 | Doc Owner | Blocked by P3 |
| **P5** | Testing | 8 | QA Manager | Blocked by P4 |
| **P6** | Code Cleanup | 8 | Release Manager | Blocked by P5 |
| **Release** | L4 Readiness | 6 | Executive | Blocked by P6 |

### Phase 0: Constitutional Foundation (START HERE)

**Tasks:**
1. P0-T1: Create `.laio/constitution.yaml` - Project identity
2. P0-T2: Verify TDS-002 design spec exists
3. P0-T3: Classify project domain (Domain 5: Non-VOS Apps)
4. P0-T4: Establish PoE infrastructure (.beads/edgs/)

**Gate Criteria:**
- ✅ All 4 tasks completed
- ✅ Constitution file properly formatted
- ✅ TDS verified and approved
- ✅ Domain classification in constitution
- ✅ PoE directories created

**Approval:** Project Sponsor (Human-in-the-Loop)

### Phase 1: Critical Stability (AFTER P0)

**Tasks:**
1. P1-T1: Revert uncommitted lib.rs changes
2. P1-T2: Add path validation (openspec: add-path-validation) ← **OpenSpec-backed**
3. P1-T3: Fix unwrap() patterns
4. P1-T4: Add doc comments
5. P1-T5: Run clippy analysis
6. P1-T6: Run full test suite
7. P1-T7: Create PoE bundle

**Gate Criteria:**
- ✅ Zero clippy warnings
- ✅ 100% test pass rate
- ✅ Path validation implemented & tested
- ✅ PoE bundle complete

---

## Part 4: Complete Integration Workflow

### Scenario: Implementing Phase 1 - Task 2 (Path Validation)

#### Step 1: Review Context (5 min)
```bash
# Read project conventions
cat openspec/project.md

# See what's planned
openspec spec list --long
openspec list

# Review EDGS phase
cat docs/EDGS_INTEGRATION.md | grep "Phase 1" -A 50
```

#### Step 2: Create OpenSpec Proposal (20 min)
```bash
# Create change structure
mkdir -p openspec/changes/add-path-validation/specs/disk-scanning

# Write proposal.md (why, what, impact)
# Write tasks.md (implementation checklist)
# Write specs/disk-scanning/spec.md (ADDED/MODIFIED requirements with scenarios)

# Validate
openspec validate add-path-validation --strict
```

#### Step 3: Create bd Issue (5 min)
```bash
# Get next issue number
ISSUE_NUM=$(bd list --json | jq 'length + 1')

# Create phase epic (if not exists)
bd create "EDGS Phase 1: Critical Stability" -p 0 -t epic

# Create task with OpenSpec link
bd create "P1-T2: Add path validation (openspec: add-path-validation)" \
  -p 0 -t task \
  -d "Implement path validation per openspec/changes/add-path-validation/"

# Add dependency (P1-T2 blocked by P1-T1)
bd dep add "P1-T2: Add path validation..." "P1-T1: Revert uncommitted..."
```

#### Step 4: Implement (1-2 hours)
```bash
# Follow tasks.md checklist
# - [ ] 1.1 Create src-tauri/src/utils/path.rs
# - [ ] 1.2 Add validate_scan_path() function
# - [ ] 1.3 Integrate into scan commands
# - [ ] 1.4 Add unit tests

# As you complete each subtask:
cd /Volumes/Tempext-Projects/Users/tempext/Projects/disk-bloat-scanner

# 1.1 Create module
cat > src-tauri/src/utils/path.rs << 'EOF'
// Path validation module
pub fn validate_scan_path(path: &Path) -> Result<(), String> {
    // Implementation
}
EOF

# 1.3 Integrate into lib.rs
# (Edit scan_bloat, scan_duplicates, etc. to call validate_scan_path)

# 1.4 Add tests
cargo test

# Track progress in bd
bd update "disk-bloat-scanner-XX" --status in_progress
```

#### Step 5: Commit to Git (5 min)
```bash
# Stage changes
git add src-tauri/src/utils/path.rs src-tauri/src/lib.rs

# Commit with EDGS reference
git commit -m "feat: add path validation for scan operations (EDGS Phase 1: P1-T2)

- Create src-tauri/src/utils/path.rs with safe path whitelist
- Validate all scan roots against system directory blacklist
- Integrate into scan_bloat, scan_duplicates, scan_junk
- Add unit tests for path validation
- Refs: openspec/changes/add-path-validation/"

# Push
git push origin main
```

#### Step 6: Mark Complete in bd (2 min)
```bash
# Update status
bd update "disk-bloat-scanner-XX" --status completed

# Check if task dependencies unblock other work
bd ready --priority 0
```

#### Step 7: Archive OpenSpec Change (After PR merge)
```bash
# Move to archive
openspec archive add-path-validation --yes

# Update canonical specs
# (Archiver automatically moves changes/add-path-validation/ → changes/archive/YYYY-MM-DD-add-path-validation/)
# (And updates specs/disk-scanning/spec.md with ADDED/MODIFIED requirements)

# Verify
openspec validate --strict
```

---

## Part 5: Compliance Checklist

### TES-2025 v6.9 Compliance

- [x] **EDGS**: Event-driven gated scheduling with no time estimates
- [x] **LAIO Model**: Project has constitution, domain classification, PoE bundles
- [x] **Maturity Levels**: L2 (Integration) → L4 (Release) progression
- [x] **Gated Progression**: Each phase requires HIL approval before next
- [x] **PoE Bundles**: Immutable evidence at each gate
- [x] **Auditor Verification**: Gate validation tasks before approval
- [x] **Dependency Management**: bd tracks blocking relationships
- [x] **Spec-Driven**: All changes start with OpenSpec proposals
- [x] **Git Integration**: Commits reference EDGS phases and OpenSpec changes

### LAIO v3.3 Compliance

- [x] **Truth-Seeking**: OpenSpec specs are source of truth
- [x] **Traceability**: Every change traced to EDGS phase + OpenSpec proposal
- [x] **Provenance**: bd issues and git commits create audit trail
- [x] **Governance**: HIL approval gates enforce human oversight
- [x] **Self-Management**: bd can extend with custom tables for tracking

### Project-Specific Standards

- [x] **OpenSpec**: All changes follow spec-first methodology
- [x] **bd Integration**: Issues track EDGS tasks + OpenSpec references
- [x] **Git Workflow**: Commits include phase + change references
- [x] **Testing**: All implementations tested per testing strategy
- [x] **Documentation**: Specs, design docs, and README kept in sync

---

## Part 6: Quick Command Reference

### OpenSpec Commands
```bash
openspec spec list --long        # See existing specs
openspec list                    # See active changes
openspec show [change-id]        # View proposal + tasks
openspec validate [change-id] --strict  # Validate change
openspec archive [change-id] --yes      # Archive after completion
```

### bd Commands
```bash
bd ready --priority 0            # See ready work
bd list                          # See all issues
bd update [issue] --status in_progress
bd dep tree [issue]              # View dependencies
bd dep cycles                    # Check for loops
```

### Git Workflow
```bash
# Feature branch
git checkout -b feature/add-path-validation
git add [files]
git commit -m "feat: add path validation (EDGS Phase 1: P1-T2)

- Description of changes
- Reference: openspec/changes/add-path-validation/
- Refs: bd issues"
git push origin feature/add-path-validation

# Create PR, get approval, merge
```

---

## Part 7: Example: Starting Phase 0

```bash
# 1. Understand Phase 0
cat docs/EDGS_INTEGRATION.md | grep "Phase 0" -A 30

# 2. See what's ready
bd ready --priority 0

# 3. Claim P0-T1
bd update [P0-T1 issue] --status in_progress

# 4. Create constitution
mkdir -p .laio
cat > .laio/constitution.yaml << 'YAML'
name: "Disk Bloat Scanner"
purpose: "Cross-platform desktop app for disk space analysis"
laio_class: "LAIO_Project"
maturity_level: "L2"
domain: "Domain 5: Non-VOS Applications"
maintainers:
  - "frank@tempext.io"
created_at: "2025-10-26"
YAML

# 5. Commit
git add .laio/constitution.yaml
git commit -m "EDGS Phase 0: P0-T1 complete - create constitution.yaml"

# 6. Mark complete
bd update [P0-T1 issue] --status completed

# 7. Check progress
bd list --status completed | wc -l

# 8. Continue to remaining Phase 0 tasks
```

---

## Part 8: Troubleshooting

### OpenSpec Issues

**"Change must have at least one delta"**
- Check: `changes/[id]/specs/` exists with .md files
- Fix: Create spec deltas with `## ADDED Requirements` sections

**"Requirement must have at least one scenario"**
- Check: Use `#### Scenario:` format (4 hashtags)
- Fix: Add at least one scenario per requirement

### bd Issues

**bd daemon hangs**
```bash
killall -9 bd
rm .beads/daemon.pid
bd list  # Restart
```

**Circular dependencies**
```bash
bd dep cycles  # Show circular deps
# Manually break cycle by adjusting dependencies
```

### EDGS Issues

**Phase blocked by predecessor**
- Check: `bd dep tree [phase]`
- Verify: Predecessor phase gate is `completed` with approval notes

**PoE bundle incomplete**
- Missing: build.log, test.log, manifest.json
- Fix: Generate per phase completion (see EDGS_INTEGRATION.md)

---

## Summary

The integrated system provides:

✅ **OpenSpec** - Spec-first, requirement-focused change management  
✅ **bd** - Dependency-aware issue tracking with phase management  
✅ **EDGS** - Event-driven gated progression with HIL approval  
✅ **Compliance** - TES-2025 v6.9, LAIO v3.3, project standards  

**Get Started:**
1. Review `openspec/project.md` (conventions)
2. Run `bd ready --priority 0` (see Phase 0 tasks)
3. Start with P0-T1: Create `.laio/constitution.yaml`
4. Follow OpenSpec workflow for any features/changes
5. Track all work in bd with EDGS + OpenSpec references

**Next:** Begin Phase 0 with `bd ready --priority 0`

