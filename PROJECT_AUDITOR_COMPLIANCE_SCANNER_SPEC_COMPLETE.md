# Project Auditor & Compliance Scanner (PACS)
## OpenSpec Specification - Complete

**Status**: ✅ SPECIFICATION COMPLETE & READY FOR STAKEHOLDER REVIEW  
**Date**: October 27, 2025  
**Location**: `openspec/changes/project-auditor-compliance-scanner/`  

---

## What is PACS?

PACS is a sophisticated organizational-level compliance monitoring system that:

1. **Analyzes Single Projects Deeply**
   - Examines every design document, history file, and documentation artifact
   - Identifies all specifications and converts to canonical OpenSpec format
   - Validates against EDGS, LAIO, Bloom Playbook, and TES-2025 standards
   - Generates Beads issues for compliance gaps
   - Creates immutable "baseline images" for drift detection

2. **Monitors Organization-Wide Projects**
   - Scans all projects (50+, 100+, 500+) automatically
   - Detects drift against baselines immediately (within events, not time)
   - Classifies anomalies by severity: CRITICAL, HIGH, MEDIUM, LOW
   - Sends dynamic alerts to Human-in-the-Loop via email, webhook, dashboard
   - Maintains immutable audit trail of all compliance findings

3. **Captures & Manages Baselines**
   - Creates immutable project state snapshots (CDX format)
   - Tracks multiple baselines per project with versioning
   - Detects suspicious changes (compliance files edited, phases regressed, etc.)
   - Provides HIL approval workflow for baseline updates
   - Enables "drift" analysis: What changed? Is it expected? Who approved?

---

## Why This Matters

### Current Pain Points (Solved by PACS)

| Problem | Solution |
|---------|----------|
| Specs scattered across formats | Canonical OpenSpec conversion |
| No compliance monitoring | Organization-wide validator |
| Drift goes undetected | Real-time baseline comparison |
| Manual audits required | Automated deep analysis |
| EDGS/LAIO gaps unnoticed | Validator catches all violations |
| No audit trail | Immutable compliance records |
| HIL out of the loop | Dynamic notifications, approval workflows |

### Organizational Benefits

- ✅ **Canonical Design**: All projects have unified, versioned specifications
- ✅ **Compliance Assurance**: No project drifts without detection
- ✅ **Risk Mitigation**: Critical violations caught immediately
- ✅ **Audit Ready**: Complete immutable compliance records for every project
- ✅ **Self-Healing**: System suggests fixes for common compliance gaps
- ✅ **Human-Centric**: HIL makes final decisions via approval workflows

---

## Technical Architecture

### 3 Features, 19 Beads Issues

#### Feature 1: Deep Project Analyzer (7 issues)
- Multi-format spec parsing (Markdown, JSON, YAML, TOML)
- Compliance validators for EDGS, LAIO, Bloom, TES-2025
- Baseline image creation and immutable storage
- Audit report generation (Markdown, JSON, CSV)
- 40+ unit & integration tests

**Key Modules**:
- `spec_parser.rs` - Parse/convert specs
- `compliance_validator.rs` - Validate standards
- `baseline_image.rs` - Create baselines
- `commands/audit_project.rs` - Main orchestrator

#### Feature 2: Organization Monitor (6 issues)
- Enumerate all projects in organization
- Run deep analyzer on each (parallelized)
- Compare against baselines, detect drift
- Classify anomalies (normal, suspicious, critical)
- Multi-channel notifications (email, webhook, dashboard)
- Organization compliance dashboard UI

**Key Modules**:
- `baseline_compare.rs` - Drift detection
- `drift_classifier.rs` - Anomaly classification
- `notification_handler.rs` - Alert system
- `commands/org_audit_scan.rs` - Org monitor orchestrator

#### Feature 3: Baseline & Drift System (6 issues)
- Persistent baseline storage with versioning
- Baseline approval workflow (HIL gates)
- Detailed drift analysis reports
- BaselineManager Svelte UI component
- Immutable audit trail (append-only logs)
- CLI tool for baseline management

**Key Modules**:
- `baseline_storage.rs` - Persist baselines
- `drift_reporting.rs` - Generate reports
- `BaselineManager.svelte` - UI management

### Data Model Highlights

```rust
// Core structures
pub struct BaselineImage {
    pub metadata: BaselineMetadata,
    pub file_inventory: FileInventory,
    pub compliance_status: ComplianceStatus,
    pub spec_inventory: SpecInventory,
    pub file_hashes: HashMap<String, String>,
}

pub struct DriftReport {
    pub baseline_hash: String,
    pub current_hash: String,
    pub drift_score: f32, // 0-100
    pub changes: Vec<FileChange>,
    pub anomalies: Vec<Anomaly>,
}

pub struct Anomaly {
    pub anomaly_type: AnomalyType,
    pub severity: Severity, // CRITICAL, HIGH, MEDIUM, LOW
    pub description: String,
    pub remediation_hint: String,
}
```

### Compliance Integration

**EDGS Validation**:
- Phase gates present and approved
- PoE bundles exist and valid
- Phase progression never regresses
- Event-driven tasks (no time estimates)

**LAIO Validation**:
- `.laio/constitution.yaml` exists
- Domain classification present
- Maturity level appropriate (L1-L4)
- LAIO object model compliance

**Bloom Validation**:
- Phase 1: Knowledge & Comprehension (design specs exist)
- Phase 2: Application & Synthesis (implementation tracked)
- Phase 3: Evaluation & Refinement (testing documented)

**TES-2025 Validation**:
- No time-based estimates
- Dependency-driven task sequencing
- Gated progression with HIL approval
- Proof of Execution captured
- Auditable change history

---

## File Locations

### Specification Documents

```
openspec/changes/project-auditor-compliance-scanner/
├── proposal.md          (16 KiB) - Complete proposal & architecture
├── tasks.md             (45 KiB) - 19 Beads issues with dependencies
└── specs/              (to be created)
    ├── deep-analyzer/
    ├── org-monitor/
    └── baseline-system/
```

### Implementation (Once Approved)

```
src-tauri/src/
├── commands/
│   ├── audit_project.rs        (Deep Analyzer entry)
│   ├── org_audit_scan.rs       (Organization Monitor)
│   └── baseline_management.rs  (Baseline commands)
├── models/
│   ├── project_audit.rs        (Audit structures)
│   ├── specification.rs        (Spec model)
│   └── compliance.rs           (Compliance status)
└── utils/
    ├── spec_parser.rs          (Multi-format parsing)
    ├── compliance_validator.rs (EDGS/LAIO/TES validation)
    ├── baseline_image.rs       (Baseline creation)
    ├── baseline_storage.rs     (Baseline persistence)
    ├── baseline_compare.rs     (Drift detection)
    ├── drift_classifier.rs     (Anomaly detection)
    ├── drift_reporting.rs      (Report generation)
    └── notification_handler.rs (Alert system)

src/lib/components/
├── OrgComplianceDashboard.svelte (Org monitor UI)
└── BaselineManager.svelte        (Baseline management UI)
```

---

## API Contracts (Preview)

### Tauri Commands

```typescript
// Deep analyze single project
command audit_project_deep(project_path, capture_baseline, generate_specs, create_beads, output_dir)
  → { compliance_score, violations, specs, beads, baseline_path }

// Scan organization
command scan_organization(org_path, skip_projects, baseline_compare, email_recipients)
  → { compliant_projects, violations_found, drift_detected, notifications_sent }

// Manage baselines
command capture_baseline(project_path, baseline_name, hil_approval)
  → { baseline_id, location, file_count }

command compare_baseline(project_path, baseline_id?)
  → { drift_score, changes, anomalies, recommendation }

command approve_baseline(project_path, baseline_id, hil_name, reason)
  → { success }
```

### CLI Commands

```bash
# Analyze project
pacs analyze-project /path/to/project --output ./reports

# Scan organization
pacs scan-org /path/to/projects --email-to hil@example.com

# Baseline management
pacs baseline capture /path/to/project --name v1.0
pacs baseline list /path/to/project
pacs baseline diff /path/to/project
pacs baseline approve /path/to/project --name v1.0
```

---

## Key Features Explained

### Feature 1: Deep Project Analyzer

**What it does**:
- Scans every `.md`, `.json`, `.toml`, `.yaml` file in `/docs/`
- Identifies specifications and converts to canonical OpenSpec
- Validates compliance against 4 standards (EDGS, LAIO, Bloom, TES-2025)
- Generates Beads issues for gaps and violations
- Creates immutable baseline image for future drift detection

**Example Output**:
```
📊 Audit Report: disk-bloat-scanner
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Compliance Score: 78/100
Status: HIGH (needs improvement)
Violations: 5
Specifications Found: 12

Violations:
❌ CRITICAL: Missing Deployment specification
❌ HIGH: EDGS Phase 2 incomplete (4/11 tasks)
⚠️  MEDIUM: 2 outdated docs in root directory
ℹ️  LOW: Suggest archiving old release notes

Recommendations:
1. Create /docs/api/DEPLOYMENT.md
2. Complete BEAD-PACS-006 through BEAD-PACS-011
3. Move root .md files to /docs/
4. Archive old release notes to /archive/
```

### Feature 2: Organization Monitor

**What it does**:
- Automatically scans all projects in organization (50+, 100+, 500+)
- Runs deep analyzer on each
- Compares current state vs baseline
- Detects anomalies (CRITICAL: phase regression, HIGH: spec deleted, etc.)
- Sends immediate alerts to HIL
- Maintains compliance dashboard

**Example Notification**:
```
🚨 CRITICAL: Project "api-service" Compliance Violation

Issue Type: EDGS Phase Regression
Severity: CRITICAL
Details: Phase regressed from 2 → 1 (unauthorized)
Files: /docs/schedules/EDGS_SCHEDULE.md (modified)
Timestamp: 2025-10-27T21:30:00Z

Remediation:
- Review unauthorized phase change
- Approve new baseline or rollback to previous state

Link: https://dashboard.example.com/audit/api-service
```

### Feature 3: Baseline & Drift System

**What it does**:
- Captures immutable project state snapshots
- Stores in versioned baseline images (CDX format)
- Provides HIL approval workflow
- Detects drift: What changed? Is it expected?
- Maintains immutable audit trail
- Enables "what if" analysis

**Example Drift Report**:
```
📋 Drift Analysis: disk-bloat-scanner
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Baseline: baseline-v1.0-2025-10-26
Current: Current state (2025-10-27T21:30:00Z)

Drift Score: 15/100 (minor changes)

Changes Detected:
✅ Added: /docs/design/NEW_FEATURE.md (expected)
⚠️  Modified: /src-tauri/src/lib.rs (check if intentional)
✅ Deleted: /archive/old-spec.md (expected cleanup)

Anomalies: None detected

Recommendation: ✅ Approve (all changes expected)
```

---

## Compliance Standards

### EDGS Integration

PACS validates that projects follow Event-Driven Gated Scheduling:
- ✅ Phase gates present and approved
- ✅ Phase progression never regresses
- ✅ PoE bundles captured at each gate
- ✅ Event-driven tasks (no time estimates)
- ❌ Violations trigger CRITICAL alerts

### LAIO Integration

PACS validates Logan AI Ontology classification:
- ✅ `.laio/constitution.yaml` exists
- ✅ Domain classification (Domain 1-5)
- ✅ Maturity level (L1-L4)
- ✅ Hierarchical relationships
- ❌ Missing LAIO → HIGH violation

### Bloom Playbook

PACS validates all 3 phases documented:
- ✅ Phase 1: Knowledge & Comprehension (design specs)
- ✅ Phase 2: Application & Synthesis (implementation)
- ✅ Phase 3: Evaluation & Refinement (testing)
- ❌ Missing phase → MEDIUM violation

### TES-2025 Compliance

PACS validates Tempext Engineering Standard:
- ✅ No time-based estimates (event-driven only)
- ✅ Dependency-driven task sequencing
- ✅ Gated progression with HIL approval
- ✅ Proof of Execution (immutable audit trail)
- ✅ Auditor sub-agent verification
- ❌ Any TES violation → CRITICAL

---

## Success Criteria & Acceptance Tests

### Feature 1 (Deep Analyzer) Acceptance

1. ✅ Analyze disk-bloat-scanner project completely
2. ✅ Identify all 42+ documentation files
3. ✅ Convert existing specs to OpenSpec format
4. ✅ Generate audit report with compliance score
5. ✅ Create Beads issues for all gaps found
6. ✅ Capture immutable baseline image (CDX)
7. ✅ All unit tests pass (40+)

### Feature 2 (Organization Monitor) Acceptance

1. ✅ Scan 3+ test projects in sequence
2. ✅ Detect compliance violations in each
3. ✅ Send email notifications to configured recipients
4. ✅ Generate organization compliance dashboard
5. ✅ All violations appear in dashboard
6. ✅ Average compliance score calculated correctly
7. ✅ All unit tests pass (20+)

### Feature 3 (Baseline & Drift) Acceptance

1. ✅ Create immutable baseline image
2. ✅ Verify baseline integrity (SHA-256)
3. ✅ Manually edit compliance file
4. ✅ System detects change within 60 seconds
5. ✅ Drift report generated accurately
6. ✅ HIL can approve or reject drift
7. ✅ All unit tests pass (25+)

---

## Performance Targets (Event-Driven)

| Operation | Target |
|-----------|--------|
| Analyze single project | <50 event cycles |
| Scan 50-project organization | <200 event cycles |
| Create baseline | <10 event cycles |
| Compare baseline | <5 event cycles |
| Send notification | <2 event cycles |
| Organization dashboard render | <100 event cycles |

---

## Risk Mitigation

### Risk 1: False Positives (Too Many Alerts)

**Mitigation**:
- Configurable severity thresholds
- Whitelist patterns for expected changes
- Batch non-critical alerts (daily digest)
- Context-aware detection (ignore formatting)

### Risk 2: Performance (Large Organizations Slow)

**Mitigation**:
- Parallel project scanning (tokio)
- Incremental scanning (only changed projects)
- Cached file hashes
- Background async daemon

### Risk 3: Compliance Files Out of Sync

**Mitigation**:
- Immutable baseline images
- Approval workflow
- Audit trail of all changes
- Weekly HIL validation

---

## Implementation Roadmap

### Phase 1: Deep Analyzer (7 issues, BEAD-PACS-001 to BEAD-PACS-007)
```
Foundation → Parser → Validators → Analyzer → Baseline → Reports → Tests
```

### Phase 2: Organization Monitor (6 issues, BEAD-PACS-008 to BEAD-PACS-013)
```
Comparison → Classification → Orchestrator → Notifications → Dashboard → Tests
```

### Phase 3: Baseline & Drift System (6 issues, BEAD-PACS-014 to BEAD-PACS-019)
```
Storage → Reporting → Approval → UI Manager → Tests → Documentation
```

### Total Deliverables

- 🦀 **5,000+ lines Rust** (modular, tested, documented)
- 🎨 **800 lines Svelte** (2 UI components)
- 🧪 **130+ tests** (unit + integration, >70% coverage)
- 📚 **50+ pages documentation** (architecture, API, admin, troubleshooting)
- 🛠️ **CLI tool** with 8 commands
- 📊 **Dashboard** for organization compliance

---

## Next Steps

### For Stakeholder Review

1. ✅ **Read**: `proposal.md` (Executive summary section first)
2. ✅ **Review**: `tasks.md` (Beads issues and dependencies)
3. ✅ **Validate**: Architecture aligns with organizational needs
4. ✅ **Approve**: Or request modifications

### Upon Approval (Gate 0)

1. ✅ Create PACS Epic in bd
2. ✅ Convert 19 Beads tasks to bd issues
3. ✅ Set up dependencies in bd
4. ✅ Assign issues to sprints
5. ✅ Begin Feature 1 implementation (Sprint 1)

### Implementation Timeline (Event-Driven)

- **Feature 1** (Deep Analyzer): Gate open after P1 foundation complete
- **Feature 2** (Organization Monitor): Gate open after Feature 1 complete
- **Feature 3** (Baseline & Drift): Gate open after Feature 2 complete
- **Integration**: Features integrated into disk-bloat-scanner
- **Rollout**: Deploy to organization projects

---

## Summary

PACS is a **comprehensive compliance monitoring system** that brings organizational-level governance to software projects. By combining:

- 🔍 **Deep Analysis**: Every spec examined, validated, converted
- 🚨 **Real-Time Monitoring**: Drift detected immediately across organization
- 📋 **Baseline Management**: Immutable project state snapshots with approval workflows
- 🤝 **HIL Integration**: Humans make final decisions via notifications & dashboards
- 📊 **Audit Trail**: Complete immutable record of all compliance events

PACS ensures that:
- ✅ No project drifts undetected
- ✅ All projects follow EDGS, LAIO, Bloom, TES-2025
- ✅ Specifications are unified and canonical
- ✅ Compliance violations are caught immediately
- ✅ Complete audit trail for regulatory compliance

---

## Documents

| Document | Purpose | Status |
|----------|---------|--------|
| `proposal.md` | Complete feature proposal & architecture | ✅ COMPLETE |
| `tasks.md` | 19 Beads issues with dependencies | ✅ COMPLETE |
| This file | Quick reference guide | ✅ COMPLETE |

---

**Status**: 🟢 **READY FOR STAKEHOLDER REVIEW**  
**Next Review**: Upon completion of proposal.md and tasks.md review  
**Last Updated**: October 27, 2025, 21:45 UTC  

