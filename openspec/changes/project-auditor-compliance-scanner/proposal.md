# Project Auditor & Compliance Scanner (PACS)
## OpenSpec Proposal Document

**Document ID:** OS-PACS-2025-10-27-v1  
**Status:** PROPOSAL (Awaiting Stakeholder Review)  
**Version:** 1.0  
**Date:** October 27, 2025  
**Author:** VOS-Coder (AI Assistant)  
**Baseline Scope:** Disk Bloat Scanner + Multi-Project Organization Support  

---

## Executive Summary

The **Project Auditor & Compliance Scanner (PACS)** is a sophisticated organizational-level compliance monitoring system designed to:

1. **Deep Project Analysis**: Examine every design document, history file, and documentation artifact in `/docs` to establish canonical project truth
2. **Specification Generation**: Automatically create/update OpenSpec documents, convert existing specs to canonical format, generate Beads issues
3. **Compliance Monitoring**: Validate projects against EDGS, LAIO, Bloom Playbook, and TES-2025 v6.9 standards
4. **Drift Detection**: Capture baseline "project images" and detect anomalies in real-time across organization
5. **Dynamic Notification**: Alert Human-in-the-Loop (HIL) to compliance violations, specification gaps, and architectural drift

### Primary Use Cases

1. **Canonical Design Establishment** - Convert fragmented specs into unified, versioned canonical format
2. **Organization-Wide Compliance** - Monitor hundreds of projects, flag drift immediately
3. **Automated Remediation** - Suggest or auto-generate fixes for common compliance gaps
4. **Audit Trail Generation** - Create immutable compliance records for each project

---

## Problem Statement

### Current State

- **Specification Fragmentation**: Projects have specs in multiple formats (Markdown, YAML, OpenSpec, custom)
- **No Unified Monitoring**: Drift goes undetected until it becomes critical
- **Manual Compliance Review**: Humans must manually audit each project
- **EDGS/LAIO/Bloom Gaps**: Projects often lack proper scheduling, classification, or methodology docs
- **Documentation Decay**: Old docs remain in root directories, creating confusion
- **No Baseline Tracking**: Can't detect when projects diverge from intended architecture

### Target State

- ✅ **Single Source of Truth**: Canonical OpenSpec + Beads for every project
- ✅ **Automatic Compliance**: Projects flagged when going off-course
- ✅ **Baseline Capture**: Every project has an immutable "image" for comparison
- ✅ **Real-Time Alerts**: HIL notified of compliance violations dynamically
- ✅ **Self-Healing**: System suggests or auto-applies compliance fixes
- ✅ **Audit Trail**: Every change tracked in compliance records

---

## Solution Architecture

### Component 1: Deep Project Scanner (DPS)

**Purpose**: Examine single project completely, generate compliance artifacts

**Inputs**:
- Project root directory path
- Project metadata (name, domain, maturity level)
- Optional baseline image for drift detection

**Scanning Scope**:

```
/docs/
  ├── analysis/          → Audit report generation
  ├── compliance/        → TES-2025, LAIO, EDGS validation
  ├── design/            → Design specs analysis
  ├── history/           → Release notes, version tracking
  ├── schedules/         → EDGS schedule validation
  └── *.md files         → Convert to OpenSpec if needed

/.beads/                 → bd issue tracking state
/openspec/              → Existing OpenSpec documents
/AGENTS.md              → Agent coordination docs
Root .md files          → Identify and migrate if necessary
```

**Analysis Tasks**:
1. **Inventory**: Catalog all .md, .json, .toml, .yaml files
2. **Parse**: Extract metadata, dependencies, version info
3. **Classify**: Identify specification type (design, API, architecture, etc.)
4. **Validate**: Check against TES-2025, EDGS, LAIO standards
5. **Convert**: Create canonical OpenSpec if spec exists but wrong format
6. **Generate**: Create Beads issues for gaps found
7. **Recommend**: Suggest fixes for non-compliant areas

**Outputs**:
- `project-compliance-report.json` - Machine-readable findings
- `project-audit.md` - Human-readable audit report
- `project-baseline-image.cdx` - Immutable reference state
- `generated-specs/` - Auto-created or converted OpenSpec files
- `bd-issues-generated.json` - New Beads issues to create

### Component 2: Organization-Wide Monitor (OWM)

**Purpose**: Scan all projects in organization, detect drift, alert HIL

**Configuration**:
- Organization projects directory
- Skip list (projects to exclude)
- Baseline capture interval
- Drift detection thresholds

**Monitoring Tasks**:
1. **Enumerate**: Find all projects (optionally filter by tag/team)
2. **Iterate**: Run DPS on each project sequentially or in parallel
3. **Compare**: Diff current state vs. baseline image
4. **Detect**: Identify drift anomalies:
   - Compliance files edited/deleted
   - Multiple .md files in root (shouldn't be there)
   - Missing required documentation
   - EDGS phase regressions
   - Unauthorized spec changes
5. **Classify**: Rank issues by severity (Critical, High, Medium, Low)
6. **Notify**: Send alerts to HIL via notification system
7. **Record**: Store findings in immutable audit log

**Outputs**:
- `organization-compliance-dashboard.json` - Aggregated metrics
- `project-drift-reports/` - Per-project drift analysis
- `compliance-violations.log` - Immutable violation record
- Notifications to HIL (email, webhook, dashboard)

### Component 3: Baseline Image System (BIS)

**Purpose**: Capture immutable project state snapshots for drift detection

**Baseline Structure**:
```
.beads/audit/baseline-images/
├── baseline-v1.0.cdx.zip
│   ├── manifest.json                    (project state snapshot)
│   ├── spec-inventory.json              (all specs found)
│   ├── compliance-status.json           (TES, EDGS, LAIO status)
│   ├── file-hashes.sha256               (all .md/.json/.toml files)
│   ├── timestamp.txt                    (capture time)
│   └── BASELINE_APPROVED.txt            (HIL approval marker)
├── baseline-v1.1.cdx.zip
│   └── ...
└── CURRENT_BASELINE                     (symlink to approved baseline)
```

**Baseline Comparison**:
- File hash comparison (detect edits)
- Metadata comparison (added/removed files)
- Specification count trends
- Compliance metric trends

### Component 4: Notification System (NS)

**Purpose**: Alert HIL to compliance issues dynamically

**Notification Types**:
1. **Critical** (immediate): Compliance violations, missing core docs
2. **High** (daily digest): Drift detected, spec conflicts, phase regression
3. **Medium** (weekly): New TODOs found, outdated docs flagged
4. **Low** (archive): Advisory items, suggestions for improvement

**Notification Channels**:
- Email (configurable recipients)
- Webhook (custom integrations)
- Dashboard (web UI showing status)
- Slack/Teams (future)

**Example Alert**:
```
[CRITICAL] Project "disk-bloat-scanner" compliance violation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Time: 2025-10-27T20:55:00Z
Issue: /docs/compliance/TES-2025-v6.9.md was modified
Baseline: hash_v1=abc123, timestamp=2025-10-26T00:00:00Z
Current:  hash_v2=def456, timestamp=2025-10-27T20:55:00Z
Drift:    File edited outside of approved change process
Action:   Require HIL approval for this compliance file change
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Feature Breakdown: 3 Features

### Feature 1: Deep Project Analyzer (40 hrs, P0-001 to P0-007)

**Purpose**: Single project deep analysis with spec generation

**Requirements**:
- Scan all project documentation
- Extract specifications and convert to OpenSpec
- Validate against compliance standards
- Generate Beads issues for gaps
- Create immutable baseline image
- Produce audit report

**Deliverables**:
- `src-tauri/src/commands/audit_project.rs` - Main analyzer command
- `src-tauri/src/models/project_audit.rs` - Data structures
- `src-tauri/src/utils/spec_parser.rs` - Spec format detection & conversion
- `src-tauri/src/utils/compliance_validator.rs` - EDGS/LAIO/TES validation
- Unit & integration tests (30+ tests)
- Example output: `audit-report-example.md`

**API Contract**:
```typescript
// Tauri command: audit_project_deep
input: {
  project_path: string;          // /path/to/project
  capture_baseline: boolean;     // True to create baseline image
  generate_specs: boolean;       // Auto-convert specs to OpenSpec
  create_beads: boolean;         // Generate Beads issues
  output_dir: string;            // Where to write reports
}

output: {
  compliance_score: number;      // 0-100
  specifications: SpecInventory;
  violations: ComplianceViolation[];
  generated_specs: GeneratedSpec[];
  generated_beads: BeadsIssue[];
  baseline_image_path?: string;
}

error: AuditError
```

### Feature 2: Organization Monitor (30 hrs, P0-008 to P0-013)

**Purpose**: Monitor all projects for drift, send alerts

**Requirements**:
- Enumerate projects in organization
- Run DPS on each project
- Compare against baselines
- Detect drift anomalies
- Send notifications to HIL
- Record audit trail

**Deliverables**:
- `src-tauri/src/commands/org_audit_scan.rs` - Organization scan orchestrator
- `src-tauri/src/utils/baseline_compare.rs` - Diff engine
- `src-tauri/src/utils/notification_handler.rs` - Alert system
- Dashboard UI component: `OrgComplianceDashboard.svelte`
- Configuration: `.beads/pacs-config.toml`
- Notification templates

**API Contract**:
```typescript
// Tauri command: scan_organization
input: {
  org_path: string;              // /path/to/org/projects
  skip_projects: string[];       // ["archived-proj", ...]
  baseline_compare: boolean;     // Check drift vs baseline
  email_recipients: string[];    // Who to notify
  severity_threshold: "Critical" | "High" | "Medium" | "Low";
}

output: {
  projects_scanned: number;
  projects_compliant: number;
  violations_found: number;
  drift_detected: DriftReport[];
  notifications_sent: number;
  audit_log_path: string;
}

error: AuditError
```

### Feature 3: Baseline & Drift System (20 hrs, P0-014 to P0-019)

**Purpose**: Capture baselines, detect drift, manage approval workflows

**Requirements**:
- Capture immutable project state
- Compare current vs baseline
- Classify drift types (normal, suspicious, critical)
- Provide approval workflow for baseline updates
- Maintain audit trail of all baselines
- Generate drift visualizations

**Deliverables**:
- `src-tauri/src/utils/baseline_image.rs` - Baseline creation & storage
- `src-tauri/src/commands/baseline_management.rs` - Baseline commands
- `src-tauri/src/utils/drift_classifier.rs` - Anomaly detection
- UI component: `BaselineManager.svelte`
- CLI tool: `pacs-baseline-tool` (Rust binary)

**API Contract**:
```typescript
// Tauri command: capture_baseline
input: {
  project_path: string;
  baseline_name: string;         // "v1.0", "pre-refactor", etc.
  hil_approval: boolean;         // Mark as approved
  notes: string;
}

output: {
  baseline_id: string;
  location: string;
  file_count: number;
  hash: string;
  timestamp: string;
}

// Tauri command: compare_baseline
input: {
  project_path: string;
  baseline_id?: string;          // Use CURRENT_BASELINE if not specified
}

output: {
  baseline_hash: string;
  current_hash: string;
  drift_score: number;           // 0-100 (0=identical, 100=totally different)
  changes: DriftChange[];        // Added, deleted, modified files
  anomalies: Anomaly[];          // Suspicious patterns
  recommendation: string;        // "No action" or "Investigate"
}

error: AuditError
```

---

## Technical Implementation Strategy

### Technology Stack

| Component | Tech | Justification |
|-----------|------|---------------|
| Parser | `serde_json`, `toml`, `yaml` crates | Multi-format spec parsing |
| Hashing | `sha2`, `blake3` | Fast, secure file fingerprinting |
| Validation | Custom validators | TES-2025, EDGS, LAIO standards |
| Notification | `lettre` (email), webhook HTTP | Multi-channel alerting |
| Async | `tokio` | Parallel project scanning |
| Frontend | Svelte + TypeScript | Dashboard visualization |

### Modular Architecture

```
src-tauri/src/
├── commands/
│   ├── audit_project.rs        ← Deep analyzer entry point
│   ├── org_audit_scan.rs       ← Organization monitor
│   └── baseline_management.rs  ← Baseline ops
├── models/
│   ├── project_audit.rs        ← Audit data structures
│   ├── specification.rs        ← Spec model
│   └── compliance.rs           ← Compliance status
└── utils/
    ├── spec_parser.rs          ← Multi-format parsing
    ├── compliance_validator.rs ← Validation logic
    ├── baseline_image.rs       ← Baseline creation
    ├── baseline_compare.rs     ← Drift detection
    ├── drift_classifier.rs     ← Anomaly detection
    └── notification_handler.rs ← Alert system
```

### Database Requirements

Projects need to track:
- Baseline images (immutable)
- Compliance history (audit trail)
- Notification logs
- Approved spec versions
- bd issue mappings

Use `.beads/audit/` for persistence:
```
.beads/audit/
├── baselines/                  ← Immutable baseline CDX files
├── compliance-history.jsonl    ← Append-only compliance log
├── notifications.log           ← Alert history
└── approved-specs.json         ← Spec approval registry
```

---

## Compliance & Standards Integration

### EDGS Integration

PACS validates that projects follow EDGS:
- ✅ Phase gates captured in baseline
- ✅ Phase regressions detected (e.g., P2 → P1)
- ✅ PoE bundles present and valid
- ✅ Gate approvals documented
- ❌ Missing phase gates → VIOLATION

### LAIO Integration

PACS validates LAIO classification:
- ✅ `.laio/constitution.yaml` exists
- ✅ Domain classification present
- ✅ Maturity level appropriate
- ✅ LAIO object model compliance
- ❌ Missing LAIO metadata → VIOLATION

### Bloom Playbook Integration

PACS validates Bloom methodology:
- ✅ Phase 1: Knowledge & Comprehension (design specs)
- ✅ Phase 2: Application & Synthesis (implementation)
- ✅ Phase 3: Evaluation & Refinement (testing)
- ❌ Gaps in any phase → WARNING

### TES-2025 Compliance

PACS validates TES-2025 standards:
- ✅ No time-based estimates (event-driven tasks)
- ✅ Dependency-driven sequencing
- ✅ Gated progression
- ✅ Proof of Execution (PoE)
- ✅ Auditor sub-agent verification
- ❌ Any violation → CRITICAL

---

## Baseline Image Details

### What Gets Captured

```json
{
  "baseline_metadata": {
    "id": "baseline-v1.0-2025-10-27",
    "project_name": "disk-bloat-scanner",
    "capture_time": "2025-10-27T20:00:00Z",
    "capture_method": "automatic | manual",
    "hil_approval": true,
    "approver": "frank@tempext.io"
  },
  "file_inventory": {
    "docs": {
      "compliance/TES-2025-v6.9.md": "hash_abc123",
      "design/DESIGN_SPEC.md": "hash_def456",
      "...": "..."
    },
    "openspec": {...},
    "total_md_files": 42,
    "total_bytes": 1234567
  },
  "compliance_status": {
    "edgs_phase": 2,
    "edgs_phase_gates": ["P1_APPROVED", "P2_IN_PROGRESS"],
    "laio_domain": "Domain 5",
    "laio_maturity": "L2",
    "tes_compliant": true,
    "bloom_phase": 2
  },
  "specification_inventory": {
    "openspec_count": 3,
    "traditional_specs": 5,
    "missing_specs": ["API.md", "DEPLOYMENT.md"]
  },
  "beads_status": {
    "issues_total": 43,
    "issues_completed": 7,
    "issues_blocked": 2
  },
  "file_hashes": "file_hashes.sha256"  ← Points to detailed hash file
}
```

### Drift Detection: What Triggers Alerts

| Change Type | Severity | Trigger |
|-------------|----------|---------|
| Compliance file edited | CRITICAL | `/docs/compliance/` hash changed |
| Design spec deleted | CRITICAL | `/docs/design/` file removed |
| Root .md file added | HIGH | Unexpected .md in root dir |
| EDGS phase regressed | CRITICAL | `edgs_phase` went backward |
| Maturity level regressed | HIGH | `laio_maturity` went backward |
| Beads completion went down | HIGH | More issues marked incomplete |
| Major spec count changed | MEDIUM | +/- 20% spec count |
| New warnings/TODOs | LOW | New `TODO` or `FIXME` in docs |

---

## Risk Mitigation

### Risk 1: False Positives (Too Many Alerts)

**Mitigation**:
- Configurable thresholds per severity level
- Whitelist patterns for expected changes
- Batch alerts (daily digest for Low/Medium)
- Context-aware detection (ignore formatting changes)

### Risk 2: Performance (Scanning Large Orgs Slow)

**Mitigation**:
- Parallel project scanning with configurable concurrency
- Incremental scanning (only changed projects)
- Cached file hashes to avoid re-hashing
- Background async scanning via daemon

### Risk 3: Compliance Files Get Out of Sync

**Mitigation**:
- Read-only baseline images (immutable)
- Approval workflow for baseline updates
- Audit trail of all changes
- Weekly HIL validation reports

### Risk 4: System Becomes Noise Generator

**Mitigation**:
- Smart filtering: Ignore minor changes
- Batch non-critical alerts
- Configurable alert frequency per project
- Machine learning for anomaly detection (future)

---

## Success Criteria

### Acceptance Tests

1. ✅ **Deep Scanner**: Analyze disk-bloat-scanner project, generate audit report with all specs identified
2. ✅ **Baseline Capture**: Create immutable baseline image, verify integrity with CDX checksum
3. ✅ **Drift Detection**: Manually edit compliance file, system detects change within 60 seconds
4. ✅ **Org Monitor**: Scan 3+ test projects, detect violations in each, send notifications
5. ✅ **Audit Trail**: Every change logged, immutable, queryable
6. ✅ **Zero False Positives**: 10 expected changes trigger 0 false alerts on whitelist
7. ✅ **Performance**: Scan organization of 50 projects in <30 events (event-driven, not time)
8. ✅ **Compliance**: PACS itself passes EDGS Phase 1 code quality gates

---

## Deployment & Integration

### Phase Integration

This feature spans 3 phases:
- **P0 (Foundation)**: Constitution, baseline structure setup, config
- **P1 (Core)**: Deep analyzer, basic validation, audit reports
- **P2 (Monitoring)**: Organization monitor, drift detection, notifications

### Backward Compatibility

- ✅ No breaking changes to existing Tauri commands
- ✅ New commands added alongside existing ones
- ✅ Existing projects can opt-in to PACS
- ✅ Graceful degradation if PACS not enabled

### CLI Integration

```bash
# Deep scan a project
pacs analyze-project /path/to/project --output-dir ./reports

# Scan organization
pacs scan-org /path/to/projects --email-to hil@example.com

# Manage baselines
pacs baseline capture /path/to/project --name v1.0
pacs baseline diff /path/to/project --baseline v1.0
pacs baseline approve /path/to/project --baseline v1.0
```

---

## Deliverables Summary

### Documentation
- [ ] PACS Architecture Design (20 pages)
- [ ] API Reference (Tauri commands, Rust modules)
- [ ] Configuration Guide (`.beads/pacs-config.toml`)
- [ ] Alert Templates & Examples
- [ ] Administrator Guide
- [ ] CLI Reference

### Code
- [ ] 2,000+ lines Rust (modular, tested)
- [ ] 500+ lines Svelte UI (dashboard, baseline manager)
- [ ] 40+ unit & integration tests
- [ ] Example audit reports & baselines

### Tools
- [ ] `pacs` CLI binary (Rust)
- [ ] Notification webhook server (Rust)
- [ ] Compliance dashboard (Svelte)

---

## Next Steps

1. **Stakeholder Review**: Present proposal to HIL
2. **Design Approval**: Gate 0 approval from Sponsor
3. **Sprint Planning**: Break into Beads issues
4. **Sprint 1**: Implement Deep Analyzer (P0-001 to P0-007)
5. **Sprint 2**: Implement Organization Monitor (P0-008 to P0-013)
6. **Sprint 3**: Implement Baseline & Drift System (P0-014 to P0-019)
7. **Integration**: Integrate with disk-bloat-scanner application
8. **Rollout**: Deploy to organization projects

---

## References

- **TES-2025 v6.9**: Tempext Engineering Standard (EDGS, LAIO, Bloom)
- **EDGS_SCHEDULE.md**: Event-Driven Gated Scheduling methodology
- **LAIO.md**: Logan AI Ontology (project classification)
- **OpenSpec**: Specification format standard

---

**Document Status**: Ready for Stakeholder Review  
**Next Review**: Upon HIL Approval  
**Last Updated**: October 27, 2025

