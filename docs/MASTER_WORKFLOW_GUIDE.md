# Master Workflow Guide: Complete System Overview

**Date:** October 26, 2025  
**Version:** 1.0  
**Compliance:** TES-2025 v6.9 + LAIO v3.3 + EDGS v1.0  
**Status:** ✅ FULLY INTEGRATED & READY

---

## The Three-Pillar System

Your project uses three integrated systems that work together:

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                       │
│  1. OpenSpec (Specification-Driven)                                 │
│     └─ Define WHAT to build via specs + requirements                │
│                                                                       │
│  2. bd (Dependency-Aware Issue Tracking)                            │
│     └─ Track WHO is doing WHAT and in what order                    │
│                                                                       │
│  3. EDGS (Event-Driven Gated Scheduling)                            │
│     └─ Gate phases + require HIL approval + capture PoE             │
│                                                                       │
│  TOGETHER = Spec-first, dependency-aware, gated development        │
│                                                                       │
└─────────────────────────────────────────────────────────────────────┘
```

---

## System 1: OpenSpec (Specification-Driven Development)

### Purpose
Ensure all changes start with a specification. Specs are the source of truth.

### Key Concepts
- **Specs**: Current state (what IS) - in `openspec/specs/`
- **Changes**: Proposals (what SHOULD change) - in `openspec/changes/`
- **Deltas**: ADDED/MODIFIED/REMOVED requirements per change
- **Archive**: Completed changes moved to `openspec/changes/archive/`

### When to Use OpenSpec
✅ Adding features  
✅ Making breaking changes  
✅ Changing architecture  
✅ Optimizing performance  
❌ Skip for bug fixes, typos, non-breaking updates

### Key Commands
```bash
openspec spec list --long          # See existing specs
openspec list                      # See active changes
openspec validate [change] --strict # Validate proposal
openspec archive [change] --yes    # Archive after completion
```

### Documentation
- **Full Guide**: `openspec/AGENTS.md`
- **Project Conventions**: `openspec/project.md`
- **Integration Guide**: `docs/OPENSPEC_BD_EDGS_INTEGRATION.md`

---

## System 2: bd (Issue Tracking)

### Purpose
Track work items, manage dependencies, show what's ready to work on.

### Key Concepts
- **Issues**: Individual work items
- **Status**: pending → in_progress → completed
- **Priority**: 0 (highest) → 4 (lowest)
- **Dependencies**: Task B blocks Task A
- **Ready Work**: Issues with no blockers (show with `bd ready`)

### When to Use bd
✅ Track all work items  
✅ Manage dependencies  
✅ See what's ready next  
✅ Link to OpenSpec changes  
❌ Don't store implementation details (use spec instead)

### Key Commands
```bash
bd list                            # See all issues
bd ready --priority 0              # See ready work
bd update [issue] --status in_progress
bd update [issue] --status completed
bd show [issue]                    # View details
bd dep tree [issue]                # View dependencies
```

### Issue Naming Convention
```
[TYPE] [PHASE]: [DESCRIPTION] (openspec: [change-id])

Examples:
- "EDGS Phase 0: Create constitution.yaml"
- "EDGS Phase 1: Add path validation (openspec: add-path-validation)"
- "Bug: Fix memory leak (EDGS Phase 1)"
```

### Documentation
- **bd Quickstart**: Run `bd quickstart`
- **Integration Guide**: `docs/OPENSPEC_BD_EDGS_INTEGRATION.md`

---

## System 3: EDGS (Event-Driven Gated Scheduling)

### Purpose
Structure project into phases with gated progression and HIL approval.

### Key Concepts
- **Phases**: 8 phases (P0-P6 + Release)
- **Tasks**: 4-11 tasks per phase
- **Gates**: Validation + HIL approval before next phase
- **PoE**: Immutable evidence bundles at each gate
- **Event-Driven**: No time estimates, progress on completion

### The 8 Phases

| Phase | Name | Tasks | Gate | Status |
|-------|------|-------|------|--------|
| **P0** | Constitutional | 4 | Project Sponsor | 🚀 START HERE |
| **P1** | Critical Stability | 7 | Tech Lead | Blocked by P0 |
| **P2** | Architecture | 11 | Architect | Blocked by P1 |
| **P3** | Frontend | 8 | QA Lead | Blocked by P2 |
| **P4** | Documentation | 9 | Doc Owner | Blocked by P3 |
| **P5** | Testing | 8 | QA Manager | Blocked by P4 |
| **P6** | Code Cleanup | 8 | Release Mgr | Blocked by P5 |
| **Release** | L4 Readiness | 6 | Executive | Blocked by P6 |

### Compliance Standards
- ✅ TES-2025 v6.9 Section 3.4 (EDGS methodology)
- ✅ LAIO v3.3 (Logan AI Ontology)
- ✅ Proof of Execution (PoE bundles)
- ✅ Human-in-the-Loop (HIL approval)

### Documentation
- **Full Schedule**: `docs/schedules/EDGS_SCHEDULE.md` (exhaustive)
- **Integration Guide**: `docs/EDGS_INTEGRATION.md` (quick ref)
- **Master Guide**: This document

---

## How They Work Together

```
IDEA
 │
 ├──> Create OpenSpec proposal
 │    ├─ proposal.md (why, what, impact)
 │    ├─ tasks.md (implementation checklist)
 │    └─ specs/[capability]/spec.md (ADDED/MODIFIED requirements)
 │
 ├──> Create bd issue
 │    ├─ Link to OpenSpec change
 │    ├─ Assign to EDGS phase
 │    └─ Add dependencies
 │
 ├──> Implement per tasks.md
 │    ├─ Follow checklist
 │    ├─ Test all scenarios
 │    └─ Validate against spec
 │
 ├──> Commit to git (with references)
 │    └─ "feat: description (EDGS Phase X: P[X]-T[Y])
 │         - Refs: openspec/changes/[change-id]/"
 │
 ├──> Mark complete in bd
 │    └─ bd update [issue] --status completed
 │
 ├──> Phase gate validation
 │    ├─ All tasks done?
 │    ├─ All tests pass?
 │    └─ PoE bundle complete?
 │
 └──> HIL approval
      └─ Gate authority approves → Next phase unblocks
```

---

## The Golden Path: Your First Task

### Time: ~45 minutes
### Task: EDGS Phase 0 - Task 1 (Create constitution.yaml)

**Follow these steps:**

1. **Understand Phase 0**
   ```bash
   cat docs/EDGS_INTEGRATION.md | grep "Phase 0" -A 30
   ```

2. **See what's ready**
   ```bash
   bd ready --priority 0
   ```

3. **Read spec**
   ```bash
   cat openspec/specs/constitution/spec.md
   ```

4. **Claim the task**
   ```bash
   bd update [P0-T1-issue] --status in_progress
   ```

5. **Implement**
   ```bash
   mkdir -p .laio
   cat > .laio/constitution.yaml << 'YAML'
   name: "Disk Bloat Scanner"
   purpose: "Cross-platform disk space management tool"
   laio_class: "LAIO_Project"
   maturity_level: "L2"
   domain: "Domain 5: Non-VOS Applications"
   maintainers:
     - "frank@tempext.io"
   created_at: "2025-10-26T21:35:00Z"
   YAML
   ```

6. **Validate**
   ```bash
   # Check it's valid YAML
   python3 -c "import yaml; yaml.safe_load(open('.laio/constitution.yaml'))" && echo "✓"
   ```

7. **Commit**
   ```bash
   git add .laio/constitution.yaml
   git commit -m "EDGS Phase 0: P0-T1 complete - create constitution.yaml
   
   - Create .laio/constitution.yaml with project identity
   - Define LAIO class, maturity, domain per TES-2025 v6.9
   - Implements all spec requirements"
   ```

8. **Mark complete**
   ```bash
   bd update [P0-T1-issue] --status completed
   ```

**Done!** 🎉

---

## Daily Workflow

### Morning: Check What's Ready
```bash
# See top priority ready items
bd ready --priority 0
```

### During Work: Track Progress
```bash
# Claim a task
bd update [issue] --status in_progress

# Check dependencies
bd dep tree [issue]

# View spec
cat openspec/specs/[capability]/spec.md
```

### Before Commit: Validate Everything
```bash
# Run tests
cargo test
npm test

# Check linting
cargo clippy --all-targets -- -D warnings

# Validate OpenSpec (if applicable)
openspec validate [change-id] --strict
```

### Commit: Include All References
```bash
git commit -m "feat: what you did (EDGS Phase X: P[X]-T[Y])

- Implementation details
- Refs: openspec/changes/[change-id]/
- Closes: bd issue"
```

### End of Day: Mark Complete
```bash
# Update issue
bd update [issue] --status completed

# See what's unblocked next
bd ready --priority 0
```

---

## Decision Tree: What to Do When

```
I need to...

├─ Add a feature?
│  └─ Create OpenSpec change → Create bd issue → Implement → Commit → Archive
│
├─ Fix a bug?
│  └─ Check if restores intended behavior
│     ├─ Yes → Fix directly (no spec needed)
│     └─ No → Treat as feature (full process)
│
├─ Refactor code?
│  └─ Does it change behavior?
│     ├─ Yes → Create OpenSpec change
│     └─ No → Document why in code + commit
│
├─ Update dependencies?
│  └─ Is it breaking?
│     ├─ Yes → Create OpenSpec change
│     └─ No → Commit directly
│
├─ Update documentation?
│  └─ Is it spec-affecting?
│     ├─ Yes → Update openspec/specs/
│     └─ No → Just commit
│
└─ Check what to work on next?
   └─ bd ready --priority 0
```

---

## File Map: Where Things Are

```
openspec/
├── project.md                      ← Project conventions (READ FIRST)
├── AGENTS.md                       ← OpenSpec workflow guide
├── specs/                          ← Current specs (source of truth)
│   ├── disk-scanning/spec.md
│   ├── duplicate-detection/spec.md
│   └── cleanup-operations/spec.md
└── changes/                        ← Proposed changes
    ├── add-path-validation/
    │   ├── proposal.md
    │   ├── tasks.md
    │   └── specs/disk-scanning/spec.md
    └── archive/YYYY-MM-DD-[name]/

docs/
├── MASTER_WORKFLOW_GUIDE.md        ← This file
├── OPENSPEC_BD_EDGS_INTEGRATION.md ← Full integration guide
├── QUICKSTART_OPENSPEC_BD_EDGS.md  ← Quick-start for Phase 0
├── EDGS_INTEGRATION.md             ← EDGS quick reference
├── schedules/EDGS_SCHEDULE.md      ← Full phase breakdown
└── compliance/TES-2025-v6.9.md     ← Engineering standard

.beads/
├── disk-bloat-scanner.db          ← bd issue database
├── edgs/
│   ├── README.md
│   ├── EDGS_PHASE_MAPPING.md
│   └── phase-{0-7}/               ← PoE bundles (after gate completion)
│       ├── build.log
│       ├── test.log
│       ├── manifest.json
│       └── PoE-Phase{N}.cdx.zip

AGENTS.md                          ← Crash recovery context
EDGS_RECOVERY_COMPLETE.md          ← Recovery summary
```

---

## Quick Command Reference

### OpenSpec
```bash
openspec spec list --long              # All specs
openspec list                          # Active changes
openspec show [change]                 # View proposal
openspec validate [change] --strict    # Validate
openspec archive [change] --yes        # Archive
```

### bd
```bash
bd ready --priority 0                  # Ready work
bd list                                # All issues
bd show [issue]                        # Details
bd update [issue] --status completed   # Mark done
bd dep tree [issue]                    # Dependencies
bd dep cycles                          # Check loops
```

### Git
```bash
git status                             # Current state
git add [files]                        # Stage
git commit -m "msg (EDGS Phase X: P[X]-T[Y])"
git push origin main                   # Push
git log --oneline -5                   # History
```

### Tests
```bash
cargo test                             # Rust tests
npm test                               # Frontend tests
cargo clippy --all-targets -- -D warnings
cargo fmt                              # Format
cargo tarpaulin                        # Coverage
```

---

## Success Checklist: You're Ready When

- [ ] You can explain the three systems (OpenSpec, bd, EDGS)
- [ ] You've read `openspec/project.md`
- [ ] You've completed Phase 0 - Task 1 (constitution.yaml)
- [ ] You can run `bd ready --priority 0` and understand output
- [ ] You can create an OpenSpec change proposal
- [ ] You understand the gate structure
- [ ] You know how to reference OpenSpec + EDGS in commits
- [ ] You've reviewed `docs/EDGS_INTEGRATION.md`

---

## Getting Help

| Question | Answer Location |
|----------|-----------------|
| How do I start? | `docs/QUICKSTART_OPENSPEC_BD_EDGS.md` |
| How do specs work? | `openspec/AGENTS.md` |
| How does bd work? | Run `bd quickstart` |
| How do phases work? | `docs/EDGS_INTEGRATION.md` |
| Full integration? | `docs/OPENSPEC_BD_EDGS_INTEGRATION.md` |
| Phase templates? | `docs/OPENSPEC_CHANGE_TEMPLATES.md` |
| Crash recovery? | `AGENTS.md` |
| Compliance? | `docs/compliance/TES-2025-v6.9.md` |

---

## Key Metrics

| Metric | Current | Target | Phase |
|--------|---------|--------|-------|
| Maturity Level | L2 | L4 | Release |
| Total EDGS Tasks | 52 | 52 | All |
| Gated Phases | 8 | 8 | All |
| Test Coverage | ~5% | 50%+ | P5 |
| Code Warnings | ~15 | 0 | P1 |
| Modularization | lib.rs: 1200 lines | <300 | P2 |

---

## Timeline (Event-Driven, No Time Estimates)

```
Phase 0 Complete
      ↓ (HIL Approval)
Phase 1 Complete
      ↓ (HIL Approval)
Phase 2 Complete
      ↓ (HIL Approval)
...
Phase 6 Complete
      ↓ (HIL Approval)
Release Gate Complete
      ↓ (HIL Approval)
v0.2.0 Published 🎉
```

Each phase unblocks only after previous gate approval.  
No time estimates; progress driven by completion.

---

## Next Actions

1. **Read**: `docs/QUICKSTART_OPENSPEC_BD_EDGS.md` (30 min)
2. **Create**: `.laio/constitution.yaml` (10 min)
3. **Understand**: bd ready workflow (5 min)
4. **Complete**: Phase 0 Tasks 2-4 (following same pattern)
5. **Get**: Phase 0 Gate approval from Project Sponsor
6. **Proceed**: Phase 1 (Critical Stability)

---

## Project Status

```
Status: ✅ READY FOR PHASE 0 WORK

Infrastructure:
  ✅ OpenSpec structure initialized
  ✅ bd issue tracker active
  ✅ EDGS phases defined
  ✅ Compliance frameworks documented
  ✅ Integration guides complete

Documentation:
  ✅ Project conventions (openspec/project.md)
  ✅ Workflow guides (3 documents)
  ✅ EDGS schedule (full breakdown)
  ✅ Phase templates (ready to use)

Ready to Begin:
  → bd ready --priority 0
```

---

## Support

**Questions?** Check the documentation tree above.  
**Stuck?** See troubleshooting in integration guide.  
**Want to contribute?** Follow the golden path in QUICKSTART.  

---

**Last Updated:** October 26, 2025  
**Version:** 1.0  
**Status:** Active, Fully Compliant, Ready for Phase 0

**Begin:** `cd /Volumes/Tempext-Projects/Users/tempext/Projects/disk-bloat-scanner && bd ready --priority 0`

