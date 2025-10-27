# Quick Start: OpenSpec + bd + EDGS

**Goal:** Complete one EDGS task using OpenSpec + bd  
**Time:** 30-45 minutes  
**Outcome:** Understand integrated workflow for all three systems

---

## The Golden Path: Phase 0 - Task 1

### What You'll Do
Create the project constitution file (`.laio/constitution.yaml`) while learning the integrated workflow.

### Step 1: Understand Phase 0 (5 min)

```bash
# Read the phase overview
cat docs/EDGS_INTEGRATION.md | grep "Phase 0" -A 20

# Read project conventions
cat openspec/project.md | head -50

# Check current bd status
bd list --json | jq '.[] | select(.title | contains("Phase 0")) | {id, title, status}'
```

**What you learned:** Phase 0 has 4 simple tasks to establish project identity + PoE infrastructure.

---

### Step 2: Create OpenSpec Spec for Constitution (5 min)

**Why OpenSpec for a simple task?** Even simple work deserves a spec. The constitution IS a specification of the project identity.

```bash
# Check if spec already exists
openspec spec list --long

# If not, create specs/constitution directory
mkdir -p openspec/specs/constitution
```

**Create spec.md:**
```bash
cat > openspec/specs/constitution/spec.md << 'EOF'
# Constitution Specification

## Overview
The project constitution is a YAML file defining project identity, purpose, 
and LAIO classification per TES-2025 v6.9.

## ADDED Requirements

### Requirement: Project Constitution File
The system SHALL have a `.laio/constitution.yaml` file at project root 
containing project metadata per LAIO v3.3 specification.

#### Scenario: Constitution exists
- **WHEN** developer runs: `ls -la .laio/constitution.yaml`
- **THEN** file exists and is valid YAML

#### Scenario: Constitution contains identity
- **WHEN** reading .laio/constitution.yaml
- **THEN** contains: name, purpose, laio_class, maturity_level, domain, maintainers
- **AND** all required fields are non-empty

#### Scenario: Constitution is formatted correctly
- **WHEN** parsing constitution as YAML
- **THEN** parses without errors
- **AND** contains valid RFC 3339 timestamp for created_at
EOF

cat openspec/specs/constitution/spec.md
```

**Validate spec:**
```bash
openspec validate --strict
```

---

### Step 3: See Phase 0 Task in bd (2 min)

```bash
# List all Phase 0 issues
bd list --json 2>/dev/null | jq '.[] | select(.title | contains("P0"))'

# Find P0-T1
P0_T1=$(bd list --json | jq -r '.[] | select(.title | contains("P0-T1")) | .id' | head -1)
echo "P0-T1 Issue: $P0_T1"

# View task details
bd show $P0_T1
```

If P0-T1 doesn't exist, create it:
```bash
bd create "P0-T1: Create .laio/constitution.yaml" \
  -p 0 -t task \
  -d "Create project constitution per LAIO v3.3
  
See openspec/specs/constitution/spec.md for requirements.
Phase: EDGS Phase 0 (Constitutional Foundation)
OpenSpec: constitution"
```

---

### Step 4: Claim the Task (1 min)

```bash
# Update to in_progress
bd update $P0_T1 --status in_progress

# Verify
bd show $P0_T1
```

**Console output should show:** `status: in_progress`

---

### Step 5: Implement (5 min)

```bash
# Create .laio directory
mkdir -p .laio

# Create constitution file
cat > .laio/constitution.yaml << 'YAML'
# Project Constitution: Disk Bloat Scanner
# Per LAIO v3.3 and TES-2025 v6.9

name: "Disk Bloat Scanner"

purpose: |
  Cross-platform desktop application for identifying, analyzing, and safely 
  removing disk space wasters. Serves developers, system administrators, and 
  general users concerned with storage management.

laio_class: "LAIO_Project"

maturity_level: "L2"
maturity_description: "Integration phase - feature complete, internal consistency in progress"

domain: "Domain 5: Non-VOS Applications"
domain_rationale: "Standalone desktop app, not part of VOS microkernel"

maintainers:
  - email: "frank@tempext.io"
    role: "Primary Developer"

created_at: "2025-10-26T21:35:00Z"

governance:
  approval_authority: "Project Sponsor"
  standards:
    - "TES-2025 v6.9"
    - "LAIO v3.3"
    - "EDGS v1.0"
  
capabilities:
  - disk-scanning
  - duplicate-detection
  - cleanup-operations
  - settings-management

constraints:
  - "Tauri v2.8.5 (specific version)"
  - "Rust 2024 edition"
  - "Node.js v20+"
  - "Cross-platform: macOS, Windows, Linux"

external_dependencies:
  - "ignore (directory traversal)"
  - "sha2 (hashing)"
  - "tokio (async)"

contact: "frank@tempext.io"
YAML

# Verify file is valid YAML
cat .laio/constitution.yaml
```

---

### Step 6: Validate Against Spec (3 min)

Check your implementation against the spec:

```bash
# Does file exist?
ls -la .laio/constitution.yaml

# Is it valid YAML?
python3 -c "import yaml; yaml.safe_load(open('.laio/constitution.yaml'))" && echo "✓ Valid YAML"

# Does it contain required fields?
grep -E "name:|purpose:|laio_class:|maturity_level:|domain:|maintainers:" .laio/constitution.yaml
```

**If any fail:** Adjust the YAML and recheck.

---

### Step 7: Commit to Git (5 min)

```bash
# Stage the file
git add .laio/constitution.yaml

# Commit with EDGS + OpenSpec references
git commit -m "EDGS Phase 0: P0-T1 complete - create constitution.yaml

- Create .laio/constitution.yaml with project identity
- Define LAIO class, maturity level, domain classification  
- Document maintainers, governance, capabilities, constraints
- Refs: openspec/specs/constitution/spec.md
- Implements all requirements per spec"

# Verify commit
git log -1 --oneline
```

**Output should show:** Your commit at HEAD

---

### Step 8: Mark Complete in bd (2 min)

```bash
# Update status to completed
bd update $P0_T1 --status completed

# Verify
bd show $P0_T1

# Check phase progress
echo "Phase 0 completion:"
bd list --json | jq '[.[] | select(.title | contains("P0")) | select(.status | contains("completed"))] | length'
echo "/ 5 (epic + 4 tasks)"
```

**Output should show:** `status: completed`

---

### Step 9: Unblock Next Task (1 min)

```bash
# See what's ready now
bd ready --priority 0

# Should show P0-T2 (if P0-T1 was blocking it)
```

---

## Summary: What You Accomplished

✅ Created OpenSpec spec for constitution requirement  
✅ Found corresponding bd issue (P0-T1)  
✅ Claimed task in bd  
✅ Implemented constitution.yaml per spec  
✅ Validated against spec requirements  
✅ Committed to git with phase + OpenSpec references  
✅ Marked complete in bd  

**Time taken:** ~30-45 min  
**Process used:** OpenSpec → bd → implement → git → bd complete  

---

## Next: Phase 0 - Task 2

When ready for P0-T2 (Verify TDS-002):

```bash
# See what's ready
bd ready --priority 0

# Should show P0-T2
# Follow same steps:
# 1. Review spec
# 2. Check bd issue
# 3. Claim task
# 4. Implement (verify TDS exists + is complete)
# 5. Commit
# 6. Mark complete
```

---

## The Three-System Workflow (Checklist)

Use this checklist for ANY task in any EDGS phase:

```
PHASE 0-6 TASK (Generic)
│
├─── OpenSpec Planning
│    ├─ [ ] Read spec: openspec/specs/[capability]/spec.md
│    ├─ [ ] Check active changes: openspec list
│    ├─ [ ] If new feature: Create change proposal
│    │    ├─ [ ] proposal.md (why, what, impact)
│    │    ├─ [ ] tasks.md (implementation checklist)
│    │    └─ [ ] specs/[capability]/spec.md (deltas)
│    └─ [ ] Validate: openspec validate [change-id] --strict
│
├─── bd Issue Tracking
│    ├─ [ ] Find phase epic (P0-P6)
│    ├─ [ ] Find task issue (P[N]-T[M])
│    ├─ [ ] Claim: bd update [issue] --status in_progress
│    └─ [ ] Check dependencies: bd dep tree [issue]
│
├─── Implementation
│    ├─ [ ] Read tasks.md checklist
│    ├─ [ ] Complete each task sequentially
│    ├─ [ ] Test each component
│    └─ [ ] Run: cargo test, npm test, cargo clippy
│
├─── Git Commit
│    ├─ [ ] Stage: git add [files]
│    ├─ [ ] Commit with EDGS phase + OpenSpec refs
│    │    └─ "feat: description (EDGS Phase X: P[X]-T[Y])
│    │         - Implementation details
│    │         - Refs: openspec/changes/[change-id]/"
│    └─ [ ] Push: git push origin main
│
├─── Completion
│    ├─ [ ] Mark in bd: bd update [issue] --status completed
│    └─ [ ] Check ready work: bd ready --priority 0
│
└─── Archive (After all phase tasks + gate approval)
     ├─ [ ] openspec archive [change-id] --yes
     ├─ [ ] Create PoE bundle in .beads/edgs/phase-N/
     └─ [ ] Create gate validation issue in bd
```

---

## Common Commands Quick Reference

```bash
### OpenSpec
openspec spec list --long              # See existing specs
openspec list                          # See active changes
openspec validate [change-id] --strict # Validate proposal
openspec archive [change-id] --yes     # Archive after completion

### bd
bd ready --priority 0                  # See ready work
bd list                                # See all issues
bd show [issue]                        # View details
bd update [issue] --status in_progress # Claim task
bd update [issue] --status completed   # Mark done
bd dep tree [issue]                    # View dependencies

### Git
git add [files]                        # Stage
git commit -m "type: msg (EDGS Phase X: P[X]-T[Y])"  # Commit with refs
git push origin main                   # Push
git log --oneline -5                   # Recent commits

### Validation
cargo clippy --all-targets -- -D warnings  # Rust linting
cargo test                             # Rust tests
npm test                               # Frontend tests
```

---

## Troubleshooting

### "Can't find P0-T1 in bd"
```bash
# Create it:
bd create "P0-T1: Create constitution.yaml" \
  -p 0 -t task \
  -d "Create .laio/constitution.yaml per LAIO v3.3"
```

### "openspec validate fails"
```bash
# Check JSON output:
openspec show [change-id] --json --deltas-only

# Common issues:
# - Missing specs/[capability]/spec.md file
# - Missing "## ADDED Requirements" header
# - Scenario not using "#### Scenario:" format
# - No bullet points under scenario for WHEN/THEN
```

### "bd command hangs"
```bash
killall -9 bd
rm .beads/daemon.pid
bd list  # Restart
```

### "Git commit fails"
```bash
# Check status
git status

# Ensure you're on main branch
git branch

# Pull latest
git pull origin main

# Then commit again
```

---

## Success Criteria: You're Done When

✅ Task P0-T1 marked `completed` in bd  
✅ File `.laio/constitution.yaml` exists and is valid YAML  
✅ Git commit references EDGS phase and OpenSpec spec  
✅ You understand the three-system workflow  
✅ You can explain why each system matters:
   - **OpenSpec** → Spec-driven, requirement-focused
   - **bd** → Dependency tracking, issue management
   - **EDGS** → Phase gating, HIL approval, PoE

---

## Next Steps

1. **Complete Phase 0 Tasks 2-4** using same workflow
2. **Get Phase 0 Gate Approval** from Project Sponsor
3. **Proceed to Phase 1** (Critical Stability)
4. **Continue through all 8 phases** → v0.2.0 release

**Resources:**
- Full Integration Guide: `docs/OPENSPEC_BD_EDGS_INTEGRATION.md`
- Change Templates: `docs/OPENSPEC_CHANGE_TEMPLATES.md`
- EDGS Schedule: `docs/schedules/EDGS_SCHEDULE.md`
- OpenSpec Guide: `openspec/AGENTS.md`

---

**Ready?** Run: `bd ready --priority 0`  
Then pick P0-T1 and follow the golden path above! 🚀

