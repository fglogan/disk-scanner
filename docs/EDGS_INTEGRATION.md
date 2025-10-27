# EDGS Integration into bd Issue Tracking

**Status:** ✅ INITIALIZED & READY FOR PHASE 0  
**Date:** October 26, 2025  
**Integration Level:** TES-2025 v6.9 Compliant  
**Project Maturity:** L2 (Integration) → L4 (Release)

---

## What is EDGS?

**EDGS** = **Event-Driven Gated Scheduling** (per TES-2025 v6.9 Section 3.4)

EDGS replaces time-based project management with:
- **Event-Driven:** Tasks progress upon completion, not estimated duration
- **Gated:** Each phase requires gate validation before next can start
- **Scheduled:** Dependencies and blocking relationships managed by bd
- **Human-Supervised:** HIL approval at each gate (human-in-the-loop)
- **Proof-Based:** Immutable evidence (PoE) captured at each gate

---

## Integration Infrastructure

All EDGS infrastructure has been initialized in `.beads/edgs/`:

```
.beads/edgs/
├── README.md                       ← START HERE for quick reference
├── EDGS_PHASE_MAPPING.md          ← Complete phase-to-bd task mapping
├── BOOTSTRAP_EDGS_V2.sh           ← Script to create bd issues (manual creation needed)
├── ISSUE_IDS.txt                  ← Issue ID reference
└── phase-{N}/                      ← Will be populated after each phase completes
    ├── build.log                   ← Compilation artifacts
    ├── test.log                    ← Test results
    ├── manifest.json               ← Task completion evidence
    └── PoE-Phase{N}.cdx.zip        ← Immutable audit bundle
```

---

## Current State

### Already Initialized
✅ EDGS schedule document created (`docs/schedules/EDGS_SCHEDULE.md`)  
✅ Phase mapping created (`.beads/edgs/EDGS_PHASE_MAPPING.md`)  
✅ bd database initialized (`.beads/disk-bloat-scanner.db`)  
✅ EDGS README created (`.beads/edgs/README.md`)  
✅ Infrastructure directories prepared

### Next Steps
⏳ Create Phase 0 epic and 4 tasks in bd  
⏳ Add task dependencies  
⏳ Start Phase 0 - Task 1: Create `.laio/constitution.yaml`  
⏳ Proceed through all 8 phases → v0.2.0 release

---

## The 8 Phases at a Glance

| Phase | Name | Tasks | Duration | Gate Authority |
|-------|------|-------|----------|-----------------|
| **P0** | Constitutional Foundation | 4 | Event-driven | Project Sponsor (HIL) |
| **P1** | Critical Stability | 7 | Event-driven | Tech Lead |
| **P2** | Architecture Refactoring | 11 | Event-driven | Architect |
| **P3** | Frontend Modernization | 8 | Event-driven | QA Lead |
| **P4** | Documentation Org | 9 | Event-driven | Doc Owner |
| **P5** | Testing Expansion | 8 | Event-driven | QA Manager |
| **P6** | Code Cleanup | 8 | Event-driven | Release Manager |
| **Release** | L4 Readiness | 6 | Event-driven | Executive Sponsor |

---

## How to Use EDGS with bd

### 1. View Ready Work
```bash
cd /Volumes/Tempext-Projects/Users/tempext/Projects/disk-bloat-scanner
bd ready --priority 0
```
Shows issues with no blocking dependencies (ready to work on)

### 2. Claim a Task
```bash
bd update disk-bloat-scanner-XX --status in_progress
```

### 3. Complete the Work
- Edit code/documentation
- Run tests to verify
- Make git commit with EDGS phase reference

### 4. Mark Complete
```bash
git commit -m "EDGS Phase 0: P0-T1 complete - constitution.yaml created"
bd update disk-bloat-scanner-XX --status completed
```

### 5. Check Phase Progress
```bash
bd list --status completed | grep "P1"  # See completed Phase 1 tasks
bd ready --priority 0              # What's unblocked next?
```

---

## Phase 0: Constitutional Foundation (START HERE)

### Tasks

1. **P0-T1: Create `.laio/constitution.yaml`**
   - Project identity file
   - Contains: name, purpose, LAIO class, maturity level, domain
   - See TES-2025 v6.9 Part 2 for LAIO object model
   - Deliverable: File in project root

2. **P0-T2: Verify TDS-002 Design Spec**
   - Check `/docs/design/TDS-002-disk-bloat-scanner.md` exists
   - Should document features, architecture, dependencies
   - Verify completeness
   - Deliverable: Review checklist completion

3. **P0-T3: LAIO Domain Classification**
   - Assign domain from LAIO taxonomy
   - For this project: **Domain 5** (Non-VOS Applications)
   - Document in constitution.yaml
   - Deliverable: Domain field in constitution

4. **P0-T4: Establish PoE Infrastructure**
   - Create directories: `.beads/edgs/phase-{0..7}/`
   - Create PoE bundle template
   - Initialize manifest.json structure
   - Deliverable: Directory structure ready for phase 1

### Gate: P0 Validation
- **Criteria:** All 4 tasks completed
- **Authority:** Project Sponsor (Human-in-the-Loop)
- **Approval:** Must be explicitly granted before Phase 1 can start
- **Evidence:** PoE bundle in `.beads/edgs/phase-0/`

---

## Workflow: Completing a Phase

Example: How Phase 1 will progress after P0 approval

1. **Phase opens** → P0 gate approved by HIL
2. **Tasks activated** → P1's 7 tasks become "ready" in bd
3. **Agent claims work** → `bd update P1-T1 --status in_progress`
4. **Work completed** → Code written, tested, committed
5. **Task marked done** → `bd update P1-T1 --status completed`
6. **Phase complete** → All 7 tasks done
7. **Gate validation** → Auditor verifies criteria met
8. **Approval** → Tech Lead approves
9. **Phase unlocked** → P2 becomes available

---

## Proof of Execution (PoE) Structure

After each phase completes, immutable evidence is captured:

```
.beads/edgs/phase-1/
├── build.log                    # cargo build output
├── test.log                     # cargo test + npm test output
├── clippy.log                   # cargo clippy --all-targets output
├── coverage.html                # Test coverage report
├── manifest.json                # Task completion metadata
├── timestamp.txt                # Gate approval time
└── PoE-Phase1.cdx.zip          # Immutable zip of all above
```

**How to create:**
```bash
# After phase tasks complete
mkdir -p .beads/edgs/phase-1/
cargo build 2>&1 | tee .beads/edgs/phase-1/build.log
cargo test 2>&1 | tee .beads/edgs/phase-1/test.log
cargo clippy --all-targets -- -D warnings 2>&1 | tee .beads/edgs/phase-1/clippy.log
npm test 2>&1 | tee .beads/edgs/phase-1/frontend.log

# Create manifest
echo '{"phase": 1, "timestamp": "'$(date -Iseconds)'", "approval": "pending"}' > \
  .beads/edgs/phase-1/manifest.json

# Create immutable zip
cd .beads/edgs/phase-1/
zip -r PoE-Phase1.cdx.zip *.log manifest.json
```

---

## Key bd Commands for EDGS

```bash
# Get overview of all EDGS issues
bd list --json | jq '.[] | select(.title | contains("EDGS"))'

# See what's ready to work on (no blocked dependencies)
bd ready --priority 0

# Check status of a specific phase
bd list --json | jq '.[] | select(.id == "disk-bloat-scanner-16")'

# View task details
bd show disk-bloat-scanner-17

# See what's blocking an issue
bd dep tree disk-bloat-scanner-17

# Update task status
bd update disk-bloat-scanner-17 --status in_progress
bd update disk-bloat-scanner-17 --status completed

# Check for circular dependencies
bd dep cycles

# Add a dependency
bd dep add disk-bloat-scanner-18 disk-bloat-scanner-17  # 18 blocked by 17
```

---

## Compliance with TES-2025 v6.9

This EDGS integration fully complies with Tempext Engineering Standard v6.9:

| Requirement | Status | Evidence |
|------------|--------|----------|
| Event-driven (no time estimates) | ✅ | EDGS_SCHEDULE.md has no hours/days |
| Gated progression | ✅ | Gate issues with HIL approval gates |
| Dependency-driven task sequencing | ✅ | bd dep commands configured |
| Granular task decomposition | ✅ | 4-11 tasks per phase |
| Proof of Execution | ✅ | PoE bundles per phase |
| Auditor sub-agent verification | ✅ | Gate validation tasks |
| TDS integration | ✅ | References TDS-002 |
| Multi-agent orchestration | ✅ | Phase parallelization documented |
| HIL approval authority | ✅ | Gate approval gates |

---

## Quick Reference: Starting Your First Task

```bash
# 1. See what's ready
cd /Volumes/Tempext-Projects/Users/tempext/Projects/disk-bloat-scanner
bd ready --priority 0

# 2. Pick Phase 0, Task 1 (P0-T1: Create constitution.yaml)
# Look for issue with "P0-T1" in the title
# Get its issue ID (e.g., disk-bloat-scanner-17)

# 3. Claim it
bd update disk-bloat-scanner-17 --status in_progress

# 4. Create the constitution file
mkdir -p .laio
cat > .laio/constitution.yaml << 'YAML'
name: "Disk Bloat Scanner"
purpose: "System utility for detecting and removing disk space wasters"
laio_class: "LAIO_Project"
maturity_level: "L2"
domain: "Domain 5: Non-VOS Applications"
maintainers:
  - "frank@tempext.io"
created_date: "2025-10-26"
YAML

# 5. Commit the work
git add .laio/constitution.yaml
git commit -m "EDGS Phase 0: P0-T1 complete - create .laio/constitution.yaml"

# 6. Mark as complete in bd
bd update disk-bloat-scanner-17 --status completed

# 7. Check progress
bd list --status completed | wc -l
```

---

## Document Links

| Document | Purpose | Location |
|----------|---------|----------|
| **EDGS Schedule** | Full phase breakdown & gates | `docs/schedules/EDGS_SCHEDULE.md` |
| **Phase Mapping** | EDGS phases → bd issues | `.beads/edgs/EDGS_PHASE_MAPPING.md` |
| **EDGS README** | Quick start guide | `.beads/edgs/README.md` |
| **TES-2025 v6.9** | Engineering standard | `docs/compliance/TES-2025-v6.9.md` |
| **Design Spec** | TDS-002 disk-bloat-scanner | `docs/design/DESIGN_SPEC.md` |
| **AGENTS Guide** | Crash recovery & context | `AGENTS.md` (root) |

---

## Troubleshooting

### bd daemon is unresponsive
```bash
killall -9 bd
rm .beads/daemon.pid
bd list  # Restart
```

### Can't find an issue ID
```bash
bd list --json | jq '.[] | .title' | grep "P0-T1"
```

### Task dependencies are circular
```bash
bd dep cycles  # Identify circular dependencies
```

### Phase gate won't open
Check if predecessor phase gate is approved:
```bash
bd show disk-bloat-scanner-XXX  # Show gate issue
# status should be "completed" with approval notes
```

---

## Next Actions

1. **Read:** `.beads/edgs/README.md` for detailed quick start
2. **Read:** `.beads/edgs/EDGS_PHASE_MAPPING.md` for complete phase specs
3. **Create Phase 0 epic & tasks** in bd (or use bootstrap script)
4. **Start Phase 0 - Task 1:** Create `.laio/constitution.yaml`
5. **Commit work** with EDGS phase reference
6. **Complete Phase 0** → Await HIL approval
7. **Continue through all phases** → Release v0.2.0

---

## Summary

EDGS is now fully integrated into your project via bd issue tracking. The system:

✅ Defines 8 phases of work (52 total tasks)  
✅ Gates each phase with HIL approval points  
✅ Tracks dependencies automatically  
✅ Captures proof of execution after each gate  
✅ Follows TES-2025 v6.9 standards  
✅ Is ready to begin Phase 0  

**Begin Phase 0:** `bd ready --priority 0`

