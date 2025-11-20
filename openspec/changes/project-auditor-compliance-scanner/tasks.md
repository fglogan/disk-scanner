# PACS Implementation: Beads Task Breakdown

**Document ID:** PACS-BEADS-2025-10-27-v1  
**Total Issues:** 19  
**Total Effort:** 90 event-driven cycles (no time estimates per EDGS)  
**Organization:** 3 Features, 3 Sprints

---

## Feature 1: Deep Project Analyzer (7 Issues)

### BEAD-PACS-001: Foundation & Data Models

**Title**: Create PACS foundation: models, error types, core structures  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: None  
**Assignee**: VOS-Coder  
**Status**: IN_PROGRESS  

**Description**:
Create foundational Rust modules for PACS:
- `src-tauri/src/models/project_audit.rs` - Audit data structures (ComplianceStatus, SpecInventory, AuditReport, etc.)
- `src-tauri/src/models/specification.rs` - Specification models (SpecFormat, SpecType, SpecMetadata, etc.)
- `src-tauri/src/models/compliance.rs` - Compliance models (ComplianceViolation, ComplianceScore, etc.)
- Add custom error type to `error.rs` (AuditError variant)

**Acceptance Criteria**:
- [ ] All 3 model files created and compile without warnings
- [ ] Structures use derive(Serialize, Deserialize, Debug) for JSON serialization
- [ ] Documentation comments on all public types
- [ ] Unit tests for model serialization (5+ tests)

**References**:
- See proposal.md for data structures
- LAIO.md for classification types
- TES-2025-v6.9.md for compliance standards

---

### BEAD-PACS-002: Spec Parser Module

**Title**: Create multi-format specification parser  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/spec_parser.rs` to parse and convert specifications:

**Capabilities**:
- Detect spec format: Markdown frontmatter, YAML, JSON, TOML, custom
- Parse Markdown with YAML frontmatter (OpenSpec standard)
- Extract metadata: title, version, status, author, date
- Detect specification type: Design, API, Architecture, Deployment, Testing
- Convert traditional specs to OpenSpec format
- Handle incomplete/malformed specs gracefully

**Functions**:
```rust
pub fn detect_spec_format(content: &str) -> SpecFormat
pub fn parse_spec_from_markdown(path: &Path) -> Result<ParsedSpec, Error>
pub fn parse_spec_from_json(path: &Path) -> Result<ParsedSpec, Error>
pub fn parse_spec_from_yaml(path: &Path) -> Result<ParsedSpec, Error>
pub fn convert_to_openspec(spec: ParsedSpec) -> OpenSpecDocument
pub fn extract_metadata_from_docs(project_path: &Path) -> Vec<SpecMetadata>
```

**Acceptance Criteria**:
- [ ] Parses 5+ different spec formats without errors
- [ ] Converts traditional Markdown specs to OpenSpec (with metadata extraction)
- [ ] Handles edge cases: empty files, binary content, encoding issues
- [ ] Returns structured SpecMetadata with all fields populated
- [ ] Unit tests for each format (20+ tests)

**References**:
- OpenSpec format specification
- Example specs in `/docs/design/` and `/openspec/`

---

### BEAD-PACS-003: Compliance Validator Module

**Title**: Create EDGS, LAIO, TES-2025 compliance validators  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-002  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/compliance_validator.rs` to validate projects:

**Validators** (each returns violations):
1. **EDGS Validator**: Check for valid EDGS schedule, phase gates, PoE bundles
2. **LAIO Validator**: Check for `.laio/constitution.yaml`, domain, maturity
3. **TES Validator**: Check no time estimates, event-driven tasks, audit trail
4. **Bloom Validator**: Check all 3 phases documented (Knowledge, Application, Evaluation)
5. **OpenSpec Validator**: Check existing specs follow canonical format
6. **Structure Validator**: Check `/docs/` organization, no rogue root .md files

**Functions**:
```rust
pub fn validate_edgs(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn validate_laio(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn validate_tes2025(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn validate_bloom(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn validate_openspec(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn validate_structure(project_path: &Path) -> Result<Vec<ComplianceViolation>, Error>
pub fn calculate_compliance_score(violations: &[ComplianceViolation]) -> f32 // 0.0-100.0
```

**Severity Levels**:
- CRITICAL: Missing core docs, phase regression, broken EDGS
- HIGH: Missing optional docs, incomplete specs, compliance gaps
- MEDIUM: Formatting issues, outdated docs, non-blocking violations
- LOW: Advisory, suggestions for improvement

**Acceptance Criteria**:
- [ ] All 6 validators implemented and tested
- [ ] Compliance score calculation accurate and reproducible
- [ ] Violations include: severity, code, message, remediation hint
- [ ] Unit tests for each validator (25+ tests)
- [ ] Integration test on disk-bloat-scanner project (finds real violations)

**References**:
- TES-2025-v6.9.md, EDGS_SCHEDULE.md, LAIO standards
- Example violations documented in proposal

---

### BEAD-PACS-004: Project Deep Analyzer Command

**Title**: Implement deep project analyzer entry point  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-002, BEAD-PACS-003  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/commands/audit_project.rs` - main analyzer orchestrator:

**Workflow**:
1. Validate project path (exists, is directory)
2. Scan `/docs/` recursively, collect all .md, .json, .toml, .yaml files
3. Parse each spec file (using spec_parser)
4. Run all compliance validators
5. Generate audit report (human-readable Markdown)
6. Optional: convert specs to OpenSpec, generate Beads issues
7. Optional: capture baseline image
8. Return structured audit result

**Tauri Command**:
```rust
#[tauri::command]
async fn audit_project_deep(
    project_path: String,
    capture_baseline: bool,
    generate_specs: bool,
    create_beads: bool,
    output_dir: Option<String>,
) -> Result<AuditProjectResult, String>
```

**Output Structure**:
```rust
pub struct AuditProjectResult {
    pub compliance_score: f32,      // 0-100
    pub compliance_level: ComplianceLevel, // Critical/High/Medium/Low
    pub violations: Vec<ComplianceViolation>,
    pub specification_inventory: SpecInventory,
    pub generated_specs: Vec<GeneratedSpec>,
    pub generated_beads: Vec<BeadsIssue>,
    pub baseline_image_path: Option<String>,
    pub audit_report_path: String,
    pub execution_time_ms: u64,
}
```

**Report Generation**:
- Human-readable audit report in Markdown
- Machine-readable JSON for dashboard/API
- Actionable remediation suggestions
- Summary statistics

**Acceptance Criteria**:
- [ ] Command accepts all parameters, handles optional args correctly
- [ ] Scans disk-bloat-scanner project, identifies all specs
- [ ] Generates audit report with violations listed
- [ ] All violations include remediation suggestions
- [ ] Optional baseline capture creates `.cdx` file
- [ ] Optional spec conversion generates OpenSpec documents
- [ ] Optional Beads generation creates `bd` issues
- [ ] Integration test on disk-bloat-scanner (captures real project state)

---

### BEAD-PACS-005: Baseline Image Creation

**Title**: Implement baseline image capture and storage  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0 (Critical)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-004  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/baseline_image.rs` - immutable baseline creation:

**Baseline Contents**:
- `baseline_metadata.json`: Project state snapshot (EDGS phase, LAIO maturity, etc.)
- `file_inventory.json`: All .md/.json/.toml files and their hashes
- `compliance_status.json`: Validation results from BEAD-PACS-003
- `specification_inventory.json`: All specs found
- `file_hashes.sha256`: Detailed SHA-256 for all files
- `timestamp.txt`: Capture time (ISO 8601)

**Storage**:
- Create `.beads/audit/baselines/`
- Each baseline: `baseline-{name}-{timestamp}.cdx.zip` (immutable ZIP)
- `CURRENT_BASELINE` symlink points to approved baseline
- Create `BASELINE_APPROVED.txt` with HIL approval marker (if approved)

**Functions**:
```rust
pub fn create_baseline_image(
    project_path: &Path,
    baseline_name: &str,
    hil_approved: bool,
) -> Result<BaselineImage, Error>

pub fn save_baseline_immutable(
    baseline: &BaselineImage,
    output_dir: &Path,
) -> Result<PathBuf, Error>

pub fn load_baseline(baseline_path: &Path) -> Result<BaselineImage, Error>

pub fn set_current_baseline(baseline_path: &Path, project_path: &Path) -> Result<(), Error>
```

**Immutability**:
- Baseline files stored in immutable ZIP (CDX format)
- Hash verification on load
- Cannot modify existing baselines (append-only)
- Approval workflow preserves integrity

**Acceptance Criteria**:
- [ ] Creates `.beads/audit/baselines/` directory structure
- [ ] Captures all required metadata
- [ ] Generates SHA-256 hashes for all files
- [ ] Creates immutable CDX zip file
- [ ] Symlinks CURRENT_BASELINE correctly
- [ ] Baseline can be loaded and verified
- [ ] Hash mismatch on load returns error
- [ ] Unit tests for baseline creation/loading (10+ tests)

---

### BEAD-PACS-006: Audit Report Generation

**Title**: Create human and machine readable audit reports  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-004, BEAD-PACS-005  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create comprehensive audit reports:

**Report Types**:
1. **Human-Readable Markdown** (`.md`)
   - Executive summary (compliance score, violations count)
   - Specification inventory with status
   - Violations grouped by severity
   - Remediation recommendations
   - Timeline and approval status

2. **Machine-Readable JSON** (`.json`)
   - Structured for dashboards, APIs, automation
   - Full violation details with codes
   - Baseline comparison data
   - Links to generated specs/Beads

3. **CSV Report** (`.csv`)
   - Violations export for spreadsheets
   - Compliance trends over time

**Report Templates**:
```markdown
# Audit Report: {project_name}
## Summary
- Compliance Score: {score}/100
- Status: {CRITICAL|HIGH|MEDIUM|LOW}
- Violations: {count}
- Specifications: {count}
- Last Approved: {date}

## Executive Summary
{1-2 paragraph overview}

## Compliance Status
{EDGS: Phase {N}, gates: {list}}
{LAIO: Domain {N}, Maturity {L_N}}
{TES-2025: Compliant: {yes/no}}

## Violations
{grouped by severity, with remediation}

## Specifications
{inventory with format and status}

## Recommendations
{actionable next steps}
```

**Acceptance Criteria**:
- [ ] Markdown report template complete and well-formatted
- [ ] JSON report has all required fields
- [ ] CSV export includes violation details
- [ ] Reports written to `output_dir` with correct filenames
- [ ] Examples provided for each format
- [ ] Dated and timestamped correctly
- [ ] Integration test generates reports (verify content)

---

### BEAD-PACS-007: Deep Analyzer Unit & Integration Tests

**Title**: Comprehensive testing for deep analyzer  
**Feature**: Deep Project Analyzer  
**Epic**: PACS v1.0  
**Priority**: P0  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001 through BEAD-PACS-006  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create 40+ tests covering all analyzer components:

**Test Categories**:
1. **Unit Tests** (25+ tests)
   - Spec parser: 8 tests (each format, edge cases)
   - Validators: 10 tests (each validator, violation types)
   - Models: 5 tests (serialization, edge cases)
   - Baseline: 2 tests (creation, loading)

2. **Integration Tests** (15+ tests)
   - Full analyzer on disk-bloat-scanner project
   - Analyzer on test fixture projects (various compliance levels)
   - Baseline capture and verification
   - Report generation and verification
   - Error handling (bad paths, permission errors)

3. **Regression Tests** (10+ tests)
   - Compliance score consistency
   - Violation detection stability
   - Report format correctness

**Coverage Target**: >70% code coverage for analyzer modules

**Test Fixtures**:
- `fixtures/compliant-project/` - Fully compliant project
- `fixtures/non-compliant-project/` - Project with violations
- `fixtures/minimal-project/` - Bare minimum structure

**Acceptance Criteria**:
- [ ] All 50+ tests pass
- [ ] >70% code coverage for new modules
- [ ] Integration tests use real project scanning
- [ ] Performance test passes (analyzer completes in reasonable event cycles)
- [ ] Error handling tests cover edge cases
- [ ] Documentation for running tests

---

## Feature 2: Organization Monitor (6 Issues)

### BEAD-PACS-008: Baseline Comparison Engine

**Title**: Implement baseline diff and drift detection  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-005  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/baseline_compare.rs` - detect drift:

**Comparison Algorithm**:
1. Load current baseline image (or CURRENT_BASELINE symlink)
2. Scan current project state
3. Compare file inventories (hashes, counts, sizes)
4. Compare compliance metrics (phase, maturity, violations)
5. Classify changes: Normal, Suspicious, Critical
6. Generate drift report

**Detection Logic**:
- **Hash mismatch** → File edited (suspicious for compliance files, normal for others)
- **File added/removed** → Structure changed (alert if in `/docs/compliance/`)
- **Phase regressed** → CRITICAL (EDGS backwards)
- **Maturity regressed** → HIGH (LAIO backwards)
- **Violation count increased** → HIGH (compliance got worse)
- **Spec count changed >20%** → MEDIUM (major structural change)

**Functions**:
```rust
pub fn compare_current_to_baseline(
    project_path: &Path,
    baseline: &BaselineImage,
) -> Result<DriftReport, Error>

pub fn calculate_drift_score(
    baseline_metrics: &ComplianceMetrics,
    current_metrics: &ComplianceMetrics,
) -> f32 // 0-100, higher = more drift

pub fn classify_changes(
    changes: &[FileChange],
) -> Vec<ClassifiedChange> // with severity
```

**Acceptance Criteria**:
- [ ] Loads baseline image correctly
- [ ] Detects file additions/deletions accurately
- [ ] Detects file edits via hash comparison
- [ ] Classifies changes by severity correctly
- [ ] Calculates drift score reproducibly
- [ ] Generates drift report with all details
- [ ] Unit tests for comparison logic (15+ tests)

**References**:
- Baseline structure from BEAD-PACS-005
- Drift types from proposal.md

---

### BEAD-PACS-009: Drift Classifier & Anomaly Detection

**Title**: Classify drift anomalies and identify suspicious patterns  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-008  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/drift_classifier.rs` - anomaly detection:

**Classification Logic**:
- **Normal Drift**: Expected changes (new feature docs, phase progression)
- **Suspicious Drift**: Unusual patterns (compliance file edited, spec deleted)
- **Critical Drift**: Security/compliance violations (phase regression, LAIO maturity down)

**Anomaly Detection Rules**:
1. Compliance file modified outside approval process → CRITICAL
2. Multiple .md files added to root → HIGH (suggests disorganization)
3. `/docs/` files deleted → HIGH (possible data loss)
4. Phase number regressed → CRITICAL (backwards movement)
5. Maturity level regressed → CRITICAL (backwards movement)
6. Violation count jumped significantly (>10%) → HIGH (compliance worse)
7. New TODOs/FIXMEs in docs → MEDIUM (technical debt)
8. Old/outdated docs not archived → LOW (hygiene issue)

**Functions**:
```rust
pub fn classify_drift(drift_report: &DriftReport) -> Vec<Anomaly>

pub struct Anomaly {
    pub anomaly_type: AnomalyType,
    pub severity: Severity,
    pub description: String,
    pub remediation_hint: String,
    pub affected_files: Vec<String>,
}
```

**Context Awareness**:
- Ignore changes from whitelisted patterns (format changes, temp files)
- Track change frequency (too many changes in short time = suspicious)
- Consider change timing (changes during maintenance window = expected)

**Acceptance Criteria**:
- [ ] Classifies all drift types correctly
- [ ] Anomaly detection rules implemented for all 8 patterns
- [ ] Generates remediation hints for each anomaly
- [ ] Whitelist patterns prevent false positives
- [ ] Context-aware filtering reduces noise
- [ ] Unit tests for classification logic (20+ tests)
- [ ] Integration test on sample drift reports

---

### BEAD-PACS-010: Organization Audit Command

**Title**: Implement organization-wide scan orchestrator  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-004, BEAD-PACS-008, BEAD-PACS-009  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/commands/org_audit_scan.rs` - organization monitor:

**Workflow**:
1. Read configuration (org path, skip list, email recipients)
2. Enumerate all projects in organization
3. Filter out skipped projects
4. For each project:
   - Run deep analyzer (BEAD-PACS-004)
   - Compare against baseline (if exists)
   - Classify drift and anomalies
   - Check compliance violations
5. Aggregate results across all projects
6. Generate organization compliance dashboard
7. Send notifications to HIL (via BEAD-PACS-011)
8. Log all findings to audit trail

**Tauri Command**:
```rust
#[tauri::command]
async fn scan_organization(
    org_path: String,
    skip_projects: Vec<String>,
    baseline_compare: bool,
    email_recipients: Vec<String>,
    severity_threshold: String, // "Critical", "High", etc.
    parallel_scans: Option<usize>, // Default: number of CPU cores
) -> Result<OrgAuditResult, String>
```

**Parallelization**:
- Use `tokio` for concurrent project scanning
- Configurable concurrency to avoid overwhelming system
- Each project scan independent (can fail without blocking others)
- Collect results and continue

**Output Structure**:
```rust
pub struct OrgAuditResult {
    pub total_projects: usize,
    pub compliant_projects: usize,
    pub projects_with_violations: usize,
    pub projects_with_drift: usize,
    pub total_violations: usize,
    pub total_anomalies: usize,
    pub critical_issues: usize,
    pub compliance_average: f32,
    pub project_reports: Vec<ProjectAuditSummary>,
    pub organization_dashboard_path: String,
    pub audit_log_entry: String,
}
```

**Acceptance Criteria**:
- [ ] Reads configuration correctly
- [ ] Enumerates projects and filters skip list
- [ ] Runs analyzer on each project
- [ ] Handles project scan failures gracefully (continues with others)
- [ ] Aggregates results correctly
- [ ] Parallelization works (scans multiple projects concurrently)
- [ ] Performance: scans organization of 50 projects efficiently
- [ ] Integration test on test organization (3+ projects)

---

### BEAD-PACS-011: Notification System

**Title**: Implement multi-channel notification system  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-008, BEAD-PACS-009  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/notification_handler.rs` - alerting:

**Notification Channels**:
1. **Email** (via `lettre` crate)
   - HTML templates for different alert types
   - Configurable recipients
   - Batch alerts (daily digest) or individual (critical)

2. **Webhook** (HTTP POST)
   - Send JSON payload to configured webhook URL
   - Support Slack, Teams, custom integrations
   - Retry logic for failed webhooks

3. **Dashboard Log** (`.beads/audit/notifications.log`)
   - Append-only notification log
   - Query-able for audit trail
   - Timestamp and HIL response tracking

**Notification Templates**:
```markdown
CRITICAL: {project_name} Compliance Violation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Issue Type: {violation_type}
Severity: CRITICAL
File(s): {affected_files}
Description: {details}
Remediation: {suggestion}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Action Required: {yes/no}
Link: {dashboard_url}
```

**Batching Logic**:
- CRITICAL: Immediate notification
- HIGH: Batch in daily digest
- MEDIUM: Weekly digest
- LOW: Archive only

**Functions**:
```rust
pub async fn send_notification(
    alert: &Alert,
    channels: &[NotificationChannel],
) -> Result<NotificationResult, Error>

pub async fn send_email(
    alert: &Alert,
    recipients: &[String],
) -> Result<EmailResult, Error>

pub async fn send_webhook(
    alert: &Alert,
    webhook_url: &str,
) -> Result<(), Error>

pub fn log_notification(
    alert: &Alert,
    result: &NotificationResult,
) -> Result<(), Error>
```

**Configuration**:
```toml
# .beads/pacs-config.toml
[notifications]
email_enabled = true
email_from = "pacs@example.com"
email_recipients = ["hil@example.com"]
webhook_enabled = true
webhook_url = "https://hooks.slack.com/..."
batch_low_severity = true
batch_interval_hours = 24
```

**Acceptance Criteria**:
- [ ] Email sending works (with test fixtures)
- [ ] Webhook sending works (with mock server)
- [ ] Dashboard logging works (append-only)
- [ ] Templates render correctly
- [ ] Batching logic batches non-critical alerts
- [ ] Retry logic handles transient failures
- [ ] Sensitive data redacted (API keys, etc.)
- [ ] Unit tests for all channels (15+ tests)

---

### BEAD-PACS-012: Organization Dashboard Component

**Title**: Create organization compliance dashboard UI  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-010  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src/lib/components/OrgComplianceDashboard.svelte` - dashboard UI:

**Dashboard Sections**:
1. **Aggregate Metrics** (top of page)
   - Total projects: {count}
   - Compliant projects: {count} ({%})
   - Average compliance score: {0-100}
   - Critical issues: {count}
   - Last scan: {timestamp}

2. **Project Compliance Grid**
   - Table: Project name, compliance score, violations, drift detected, last scan
   - Sortable by: name, score, violations, last scan
   - Filterable by: compliance level, team, tag
   - Color coding: Green (>80), Yellow (60-80), Red (<60)

3. **Violations Heat Map**
   - Project compliance trends over time
   - Violation types distribution (pie chart)
   - Severity distribution (stacked bar chart)

4. **Recent Alerts**
   - Last 20 alerts with timestamp, severity, project, issue
   - Clickable to show details
   - Acknowledge/Dismiss functionality

5. **Project Details Modal**
   - Click project → show:
     - Full compliance report
     - Violation list with remediation
     - Baseline comparison results
     - Recommended actions

**Features**:
- Real-time updates (if backend supports streaming)
- Export to CSV/PDF
- Baseline approval workflow
- Alert configuration panel

**Acceptance Criteria**:
- [ ] Dashboard loads organization data correctly
- [ ] All 5 sections render properly
- [ ] Sorting and filtering work
- [ ] Color coding reflects compliance levels
- [ ] Project details modal shows full information
- [ ] Export functionality works
- [ ] Mobile responsive (works on tablet)
- [ ] Performance: renders 50+ projects smoothly

---

### BEAD-PACS-013: Org Monitor Tests & Documentation

**Title**: Testing and documentation for organization monitor  
**Feature**: Organization Monitor  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-008 through BEAD-PACS-012  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create comprehensive tests and documentation:

**Test Suite** (20+ tests):
- Baseline comparison logic (8 tests)
- Drift classification (8 tests)
- Organization scan orchestration (4 tests)
- Notification sending (5 tests)
- Dashboard data rendering (3 tests)

**Documentation**:
- Administrator guide (using organization monitor)
- Configuration reference (.beads/pacs-config.toml)
- Alert types and meanings
- Troubleshooting guide
- API reference (Tauri commands)

**Test Fixtures**:
- Test organization with 5 projects (varying compliance)
- Baseline images for comparison testing
- Sample drift reports

**Acceptance Criteria**:
- [ ] All 20+ tests pass
- [ ] Administrator guide complete and clear
- [ ] Configuration reference documented
- [ ] Troubleshooting guide covers 10+ issues
- [ ] API reference auto-generated from code
- [ ] Example reports and alerts provided
- [ ] Integration test on test organization passes

---

## Feature 3: Baseline & Drift System (6 Issues)

### BEAD-PACS-014: Baseline Storage & Management

**Title**: Build baseline storage and retrieval system  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1 (High)  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001, BEAD-PACS-005  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/baseline_storage.rs` - persistent baseline management:

**Storage Management**:
- Directory structure: `.beads/audit/baselines/`
- Immutable storage (CDX zip format)
- Versioning (multiple baselines per project)
- Symlink management (CURRENT_BASELINE)
- Cleanup policies (keep last N baselines)

**Functions**:
```rust
pub fn list_baselines(project_path: &Path) -> Result<Vec<BaselineMetadata>, Error>

pub fn set_current_baseline(
    project_path: &Path,
    baseline_id: &str,
) -> Result<(), Error>

pub fn get_current_baseline(project_path: &Path) -> Result<BaselineImage, Error>

pub fn archive_baseline(
    project_path: &Path,
    baseline_id: &str,
) -> Result<(), Error>

pub fn approve_baseline(
    project_path: &Path,
    baseline_id: &str,
    hil_name: &str,
) -> Result<(), Error>

pub fn cleanup_old_baselines(
    project_path: &Path,
    keep_count: usize,
) -> Result<usize, Error> // returns count deleted
```

**Approval Workflow**:
- Baseline created (not approved)
- HIL reviews and approves
- Approval recorded with timestamp and HIL name
- Only approved baselines can be set as CURRENT

**Retention Policy**:
- Keep last 10 approved baselines
- Keep last 5 unapproved baselines
- Older baselines archived to external storage (optional)

**Acceptance Criteria**:
- [ ] Creates baseline directory structure
- [ ] Stores and retrieves baselines correctly
- [ ] Immutability preserved (hash verification on load)
- [ ] Versioning works (multiple baselines available)
- [ ] CURRENT_BASELINE symlink management works
- [ ] Approval workflow preserves integrity
- [ ] Cleanup respects retention policies
- [ ] Unit tests (15+ tests)

---

### BEAD-PACS-015: Drift Report Generation

**Title**: Create detailed drift analysis reports  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-008, BEAD-PACS-009, BEAD-PACS-014  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src-tauri/src/utils/drift_reporting.rs` - detailed drift analysis:

**Report Contents**:
1. **Summary**
   - Drift score (0-100)
   - Change count (added, deleted, modified files)
   - Anomaly count by severity
   - Timestamp comparison (baseline → current)

2. **Changes Detail**
   - Added files (with justification hints)
   - Deleted files (with impact analysis)
   - Modified files (with hash comparison)
   - Each change categorized (expected, suspicious, critical)

3. **Anomalies**
   - List of detected anomalies
   - Severity level for each
   - Remediation suggestions
   - Links to affected files

4. **Metrics Comparison**
   - Baseline metrics vs. current
   - Compliance score trend
   - Phase/maturity progression
   - Violation count change

5. **Recommendations**
   - Investigate: if anomalies detected
   - Approve: if drift is expected and approved
   - Rollback: if drift unintended
   - Update baseline: if approved

**Report Formats**:
- Markdown (human-readable)
- JSON (machine-readable)
- HTML (dashboard viewable)

**Acceptance Criteria**:
- [ ] Reports generated in all 3 formats
- [ ] Markdown report well-formatted and readable
- [ ] JSON report includes all details
- [ ] HTML report renders in browser
- [ ] Recommendations are actionable
- [ ] Example reports provided
- [ ] Integration test generates realistic drift reports

---

### BEAD-PACS-016: Approval Workflow System

**Title**: Implement baseline and drift approval workflows  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-014, BEAD-PACS-015  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create approval workflow for baselines and drift:

**Baseline Approval**:
1. Baseline created (status: PENDING_APPROVAL)
2. HIL reviews baseline details
3. HIL approves or rejects
4. If approved: marked with timestamp, HIL name, approval reason
5. If rejected: baseline archived, new baseline suggested

**Drift Approval**:
1. Drift detected and classified
2. If drift is expected:
   - HIL can mark as "Approved Drift"
   - Update baseline to new state
   - Record approval in audit trail
3. If drift is unexpected:
   - Escalate to HIL for investigation
   - Generate investigation report
   - Optionally rollback project

**Tauri Commands**:
```rust
#[tauri::command]
async fn approve_baseline(
    project_path: String,
    baseline_id: String,
    hil_name: String,
    approval_reason: String,
) -> Result<(), String>

#[tauri::command]
async fn approve_drift(
    project_path: String,
    drift_report_id: String,
    hil_name: String,
    approval_reason: String,
    capture_new_baseline: bool,
) -> Result<(), String>
```

**Approval Audit Trail**:
- Record in `.beads/audit/approvals.jsonl` (append-only)
- Each record: timestamp, HIL name, baseline/drift ID, reason, action
- Immutable (cannot be edited/deleted)

**Notification Integration**:
- Alert HIL when approval needed
- Send summary when approval completed
- Track approval metrics (average approval time, etc.)

**Acceptance Criteria**:
- [ ] Baseline approval workflow works end-to-end
- [ ] Drift approval workflow works end-to-end
- [ ] Approval audit trail maintained immutably
- [ ] HIL can view pending approvals
- [ ] HIL can view approval history
- [ ] Approval metrics tracked
- [ ] Unit tests for workflow logic (12+ tests)

---

### BEAD-PACS-017: Baseline Manager UI Component

**Title**: Create baseline management UI component  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-014, BEAD-PACS-015, BEAD-PACS-016  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create `src/lib/components/BaselineManager.svelte` - baseline UI:

**UI Sections**:
1. **Current Baseline Display**
   - Current baseline name and metadata
   - Approval status and date
   - "View Details" button

2. **Baseline History Table**
   - List of all baselines (10+ shown with pagination)
   - Columns: Name, Status, Date, Size, Approved By, Actions
   - Sortable, filterable
   - Actions: View, Set as Current, Approve, Archive

3. **New Baseline Creation**
   - Form: Name, Notes
   - Button: "Capture New Baseline"
   - Auto-populate timestamp

4. **Drift Analysis Viewer**
   - Show drift report for current project
   - Display anomalies with severity
   - Recommendations section
   - "Approve Drift" button (if warranted)

5. **Approval Panel** (for HIL)
   - List pending approvals
   - Each approval: review baseline/drift, approve/reject
   - Form: Approval reason (required)

**Features**:
- Real-time baseline status
- Side-by-side comparison (two baselines)
- Diff visualization (what changed)
- Download baseline for archival
- Approval notifications

**Acceptance Criteria**:
- [ ] All 5 sections render properly
- [ ] Baseline history displays correctly
- [ ] Sorting and filtering work
- [ ] New baseline form validates inputs
- [ ] Drift viewer shows anomalies clearly
- [ ] Approval workflow works in UI
- [ ] Mobile responsive
- [ ] Performance: handles 50+ baselines smoothly

---

### BEAD-PACS-018: Baseline & Drift Tests

**Title**: Comprehensive testing for baseline & drift system  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-014 through BEAD-PACS-017  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create 25+ tests for baseline and drift system:

**Unit Tests** (15+ tests):
- Baseline storage operations (5 tests)
- Approval workflow (5 tests)
- Drift classification (5 tests)

**Integration Tests** (10+ tests):
- Full baseline capture and comparison workflow
- Approval workflow end-to-end
- UI component behavior
- Performance: 100+ baselines managed efficiently

**Test Fixtures**:
- Sample projects with baselines
- Drift reports (various anomaly types)
- Approval scenarios

**Acceptance Criteria**:
- [ ] All 25+ tests pass
- [ ] Coverage >70% for new modules
- [ ] Integration tests use realistic data
- [ ] Performance tests verify efficiency
- [ ] Error cases covered

---

### BEAD-PACS-019: PACS Documentation & CLI Tool

**Title**: Complete PACS documentation and CLI tool  
**Feature**: Baseline & Drift System  
**Epic**: PACS v1.0  
**Priority**: P1  
**Effort**: Event-driven  
**Dependencies**: BEAD-PACS-001 through BEAD-PACS-018  
**Assignee**: VOS-Coder  
**Status**: PENDING  

**Description**:
Create comprehensive documentation and CLI tool:

**CLI Tool** (`pacs` binary):
```bash
# Analyze single project
pacs analyze-project /path/to/project --output ./reports

# Scan organization
pacs scan-org /path/to/projects --email-to hil@example.com

# Manage baselines
pacs baseline capture /path/to/project --name v1.0
pacs baseline list /path/to/project
pacs baseline diff /path/to/project --baseline v1.0
pacs baseline approve /path/to/project --baseline v1.0 --approval-by frank

# Export reports
pacs export --format json --output ./export.json
pacs export --format csv --output ./export.csv
```

**Documentation**:
- **PACS Architecture Guide** (20 pages)
  - Overview, design decisions, components
  - Data model and storage
  - Integration points

- **Administrator Guide** (10 pages)
  - Installation and setup
  - Configuration (pacs-config.toml)
  - Running scans
  - Interpreting results

- **API Reference** (auto-generated)
  - Tauri commands
  - Rust modules and functions
  - Data structures

- **Troubleshooting Guide** (5 pages)
  - Common issues and fixes
  - Debug logging
  - Performance tuning

- **Examples** (5 pages)
  - Sample audit reports
  - Sample notifications
  - Real-world scenarios

**Acceptance Criteria**:
- [ ] CLI tool compiles and all commands work
- [ ] CLI help text clear and complete
- [ ] Architecture guide comprehensive
- [ ] Administrator guide includes examples
- [ ] API reference auto-generated and accurate
- [ ] Troubleshooting guide covers 10+ issues
- [ ] Example reports realistic and helpful
- [ ] All docs spell-checked and formatted
- [ ] Documentation integrated into project

---

## Summary

### Timeline (Event-Driven, No Time Estimates)

**Feature 1 Sequence** (7 issues):
```
BEAD-PACS-001 → BEAD-PACS-002 → BEAD-PACS-003 → BEAD-PACS-004
                                                    ↓
                                 BEAD-PACS-005 → BEAD-PACS-006
                                 ↓
                            BEAD-PACS-007
```

**Feature 2 Sequence** (6 issues):
```
BEAD-PACS-008 → BEAD-PACS-009 → BEAD-PACS-010 → BEAD-PACS-011
                                 ↓
                            BEAD-PACS-012 → BEAD-PACS-013
```

**Feature 3 Sequence** (6 issues):
```
BEAD-PACS-014 → BEAD-PACS-015 → BEAD-PACS-016 → BEAD-PACS-017
                                 ↓
                            BEAD-PACS-018 → BEAD-PACS-019
```

### Metrics

| Category | Count |
|----------|-------|
| Total Issues | 19 |
| Feature 1 Issues | 7 |
| Feature 2 Issues | 6 |
| Feature 3 Issues | 6 |
| Total Tests | 130+ |
| Expected Code Lines | 5,000+ (Rust) + 800 (Svelte) |
| Documentation Pages | 50+ |

---

**Document Status**: Ready for Implementation  
**Next Step**: Stakeholder approval of proposal.md → Create these issues in bd → Begin Feature 1 Sprint  
**Last Updated**: October 27, 2025

