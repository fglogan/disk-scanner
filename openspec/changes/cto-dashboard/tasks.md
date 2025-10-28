# CTO Dashboard - Implementation Tasks
## BEADS Issue Breakdown

**OpenSpec Proposal:** OS-CTODASH-2025-10-28-v1  
**Total Tasks:** 22 (Phase 1: 15, Phase 2: 7)  
**Estimated Effort:** Phase 1: ~60 hours, Phase 2: ~80 hours  
**Priority:** HIGH (Executive visibility feature)

---

## Phase 1: MVP Foundation (In Disk Bloat Scanner)

### Backend Components (Rust)

#### BEAD-CTODASH-001: Organization Overview Backend 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 6-8 hours  
**Impact:** Core functionality for org-wide metrics  

**Description:**
Implement `get_organization_overview` Tauri command that scans all configured directories and aggregates:
- Total project count
- Health status distribution
- Technical debt summary from BEADS
- Recent activity metrics (commits last 7/30 days)

**Implementation:**
```rust
// src-tauri/src/commands/dashboard.rs
#[tauri::command]
pub async fn get_organization_overview(roots: Vec<String>) -> Result<OrganizationOverview, String>
```

**Files:**
- `src-tauri/src/commands/dashboard.rs` (NEW)
- `src-tauri/src/commands/mod.rs` (UPDATE)
- `src-tauri/src/lib.rs` (REGISTER COMMAND)

**Dependencies:** 
- Existing `scan_git_repos` function
- BEADS parser integration

**Test:**
- Scan 10+ projects, verify metrics accuracy
- Test with empty/missing directories

---

#### BEAD-CTODASH-002: Project Health Calculator 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 4-6 hours  
**Impact:** Determines project health status  

**Description:**
Implement health calculation logic that scores projects as Healthy/Warning/Critical based on:
- Critical BEADS count (>0 = Warning, >3 = Critical)
- Last commit age (>30 days = Warning, >90 days = Critical)
- Uncommitted changes count (>50 = Warning)
- Missing OpenSpec (Warning)

**Implementation:**
```rust
pub fn calculate_project_health(
    repo: &GitRepository,
    beads: &Vec<BeadsIssue>,
    last_commit_ts: u64
) -> HealthStatus
```

**Files:**
- `src-tauri/src/commands/dashboard.rs`

**Dependencies:** BEAD-CTODASH-001

**Test:**
- Test each health threshold
- Verify edge cases (no commits, no BEADS)

---

#### BEAD-CTODASH-003: Project Detail Backend ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 5-7 hours  
**Impact:** Enables drill-down into individual projects  

**Description:**
Implement `get_project_detail` command returning:
- Git status (branch, ahead/behind, uncommitted)
- BEADS issues filtered by project path
- Recent commits (last 30 days) with author info
- File change heatmap data
- Contributor breakdown

**Implementation:**
```rust
#[tauri::command]
pub async fn get_project_detail(project_path: String) -> Result<ProjectDetail, String>
```

**Files:**
- `src-tauri/src/commands/dashboard.rs`

**Dependencies:** 
- Existing `get_git_repo_status`
- BEAD-CTODASH-001

**Test:**
- Load detail for complex project
- Verify commit parsing accuracy

---

#### BEAD-CTODASH-004: Contributor Metrics Aggregator ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 6-8 hours  
**Impact:** Team performance visibility  

**Description:**
Implement `get_contributor_metrics` command that:
- Parses git logs across all repos
- Groups commits by author
- Calculates per-author metrics:
  - Commit count (7/30/90 days)
  - Files changed
  - Lines added/removed
  - Commit quality score (message length, conventional format)
  - Detects AI agents (via email pattern or commit message)

**Implementation:**
```rust
#[tauri::command]
pub async fn get_contributor_metrics(roots: Vec<String>) -> Result<Vec<ContributorMetrics>, String>
```

**Files:**
- `src-tauri/src/commands/dashboard.rs`
- `src-tauri/src/utils/git_analysis.rs` (NEW)

**Dependencies:** BEAD-CTODASH-001

**Test:**
- Multi-repo aggregation
- AI agent detection accuracy
- Quality score calculation

---

#### BEAD-CTODASH-005: BEADS Integration Layer 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3-4 hours  
**Impact:** Links BEADS to dashboard metrics  

**Description:**
Create utility to:
- Load `beads-export.json`
- Filter BEADS by project path (via file paths mentioned)
- Calculate technical debt metrics
- Track BEADS burndown rate

**Implementation:**
```rust
// src-tauri/src/utils/beads_loader.rs
pub fn load_beads_issues() -> Result<Vec<BeadsIssue>, String>
pub fn filter_beads_by_project(issues: &[BeadsIssue], project_path: &str) -> Vec<BeadsIssue>
```

**Files:**
- `src-tauri/src/utils/beads_loader.rs` (NEW)
- `src-tauri/src/models.rs` (ADD BeadsIssue struct)

**Dependencies:** None

**Test:**
- Parse beads-export.json
- Test project path filtering

---

#### BEAD-CTODASH-006: Data Models for Dashboard 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3-4 hours  
**Impact:** Type-safe data structures  

**Description:**
Define Rust structs for dashboard data:
- `OrganizationOverview`
- `ProjectSummary`
- `ProjectDetail`
- `ContributorMetrics`
- `HealthStatus` enum
- `TechnicalDebtSummary`
- `ActivitySummary`

**Files:**
- `src-tauri/src/models.rs`

**Dependencies:** None

**Test:**
- Serialization/deserialization
- JSON schema validation

---

### Frontend Components (Svelte)

#### BEAD-CTODASH-007: CTO Dashboard Main Component ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 5-6 hours  
**Impact:** Primary user interface  

**Description:**
Create `CTODashboard.svelte` component with:
- Layout structure (header, panels, navigation)
- Data loading logic
- Error handling
- Refresh functionality
- Export/print capability

**Files:**
- `src/lib/components/CTODashboard.svelte` (NEW)
- `src/App.svelte` (ADD ROUTE)
- `src/lib/components/Sidebar.svelte` (ADD NAV ITEM)

**Dependencies:** 
- BEAD-CTODASH-001 (backend ready)

**Test:**
- Load with mock data
- Test error states
- Responsive design

---

#### BEAD-CTODASH-008: Organization Overview Panel ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 4-5 hours  
**Impact:** Executive summary view  

**Description:**
Create `OverviewPanel.svelte` displaying:
- Total project count with trend
- Health distribution (pie chart or bar)
- Technical debt summary (by priority)
- Recent activity metrics
- Critical alerts banner

**Files:**
- `src/lib/components/dashboard/OverviewPanel.svelte` (NEW)

**Dependencies:** BEAD-CTODASH-007

**Test:**
- Visual accuracy with real data
- Responsive layout

---

#### BEAD-CTODASH-009: Project Health Grid ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 5-6 hours  
**Impact:** Project browsing interface  

**Description:**
Create `ProjectHealthGrid.svelte` with:
- Grid/list view toggle
- Sort by health, last commit, name
- Filter by health status
- Search by project name
- Click to drill down
- Color-coded health indicators

**Files:**
- `src/lib/components/dashboard/ProjectHealthGrid.svelte` (NEW)

**Dependencies:** BEAD-CTODASH-007

**Test:**
- Sort/filter functionality
- Performance with 100+ projects

---

#### BEAD-CTODASH-010: Project Detail Modal ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 6-7 hours  
**Impact:** Deep dive into project  

**Description:**
Create `ProjectDetailModal.svelte` showing:
- Project header (name, path, health)
- Git status panel
- BEADS issues list (filterable)
- Recent commits timeline
- Top contributors
- File change heatmap
- Quick actions (open in editor, scan now)

**Files:**
- `src/lib/components/dashboard/ProjectDetailModal.svelte` (NEW)

**Dependencies:** 
- BEAD-CTODASH-003 (backend)
- BEAD-CTODASH-007

**Test:**
- Modal open/close
- Data loading states
- All sub-sections render

---

#### BEAD-CTODASH-011: Contributor Metrics Panel 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 5-6 hours  
**Impact:** Team performance visibility  

**Description:**
Create `ContributorMetrics.svelte` displaying:
- Table of contributors sorted by commits
- Quality score visualization
- AI agent highlighting
- Filter by time period (7/30/90 days)
- Click for individual drill-down
- Export to CSV

**Files:**
- `src/lib/components/dashboard/ContributorMetrics.svelte` (NEW)

**Dependencies:** 
- BEAD-CTODASH-004 (backend)
- BEAD-CTODASH-007

**Test:**
- Sort/filter functionality
- AI agent detection display
- CSV export

---

#### BEAD-CTODASH-012: Technical Debt Chart 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 4-5 hours  
**Impact:** Visual trend analysis  

**Description:**
Create `TechnicalDebtChart.svelte` with:
- Stacked bar chart (critical/high/medium/low)
- Line chart for burndown over time
- Filter by priority
- Tooltip with BEAD details
- Click to open BEADS tracker

**Files:**
- `src/lib/components/dashboard/TechnicalDebtChart.svelte` (NEW)

**Dependencies:** 
- BEAD-CTODASH-005 (backend)
- Chart.js or Recharts integration

**Test:**
- Chart rendering accuracy
- Interactive tooltips
- Time period filtering

---

### Integration & Polish

#### BEAD-CTODASH-013: Dashboard State Management 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3-4 hours  
**Impact:** Consistent data across components  

**Description:**
Create Svelte store for dashboard state:
- Organization overview cache
- Selected project
- Filter/sort preferences
- Refresh interval
- User preferences (grid vs list, etc.)

**Files:**
- `src/lib/stores/dashboard.js` (NEW)

**Dependencies:** All frontend components

**Test:**
- State persistence
- Reactive updates
- Performance

---

#### BEAD-CTODASH-014: Auto-Refresh & Background Updates 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 3-4 hours  
**Impact:** Real-time data freshness  

**Description:**
Implement:
- Configurable auto-refresh (1/5/10 min)
- Background data loading (no UI block)
- Incremental updates (only changed data)
- Manual refresh button
- Last updated timestamp

**Files:**
- `src/lib/components/CTODashboard.svelte`
- `src/lib/stores/dashboard.js`

**Dependencies:** BEAD-CTODASH-013

**Test:**
- Refresh without UI flicker
- Background loading indicator
- Stop refresh when modal open

---

#### BEAD-CTODASH-015: Dashboard Settings & Preferences ✅
**Status:** PENDING  
**Priority:** LOW  
**Effort:** 2-3 hours  
**Impact:** User customization  

**Description:**
Add dashboard settings:
- Project directories to scan
- Auto-refresh interval
- Default filters
- Theme preferences
- Export format preferences

**Files:**
- `src/lib/components/dashboard/DashboardSettings.svelte` (NEW)
- `src/lib/stores.js` (EXTEND settings)

**Dependencies:** None

**Test:**
- Settings persistence
- Apply settings without refresh

---

## Phase 2: GitHub Integration (Future)

### Backend Components

#### BEAD-CTODASH-016: GitHub API Client ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 6-8 hours  
**Impact:** Enables GitHub metrics  

**Description:**
Implement GitHub integration using Octocrab crate:
- Authenticate via GitHub token
- Fetch repositories for organization
- Handle rate limiting
- Cache responses locally

**Implementation:**
```rust
// src-tauri/src/integrations/github.rs
pub async fn init_github_client(token: String) -> Result<Octocrab, String>
pub async fn fetch_org_repos(org: &str) -> Result<Vec<Repository>, String>
```

**Files:**
- `src-tauri/src/integrations/github.rs` (NEW)
- `Cargo.toml` (ADD octocrab dependency)

**Dependencies:** None

**Test:**
- Auth flow
- Rate limit handling
- Cache effectiveness

---

#### BEAD-CTODASH-017: Pull Request Metrics Collector ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 6-7 hours  
**Impact:** PR analytics  

**Description:**
Collect PR metrics:
- Open PRs by age
- Average time to merge
- Review velocity
- PR size distribution
- Approval rate by reviewer

**Implementation:**
```rust
#[tauri::command]
pub async fn get_pr_metrics(org: String, repos: Vec<String>) -> Result<PRMetrics, String>
```

**Files:**
- `src-tauri/src/integrations/github.rs`

**Dependencies:** BEAD-CTODASH-016

**Test:**
- Accuracy vs GitHub UI
- Large repo performance

---

#### BEAD-CTODASH-018: Issue Tracking Integration ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 5-6 hours  
**Impact:** Issue analytics  

**Description:**
Collect issue metrics:
- Open issues by age
- Closure rate
- Issues by label/milestone
- Burndown data

**Implementation:**
```rust
#[tauri::command]
pub async fn get_issue_metrics(org: String, repos: Vec<String>) -> Result<IssueMetrics, String>
```

**Files:**
- `src-tauri/src/integrations/github.rs`

**Dependencies:** BEAD-CTODASH-016

**Test:**
- Verify against GitHub
- Filter accuracy

---

#### BEAD-CTODASH-019: CI/CD Status Tracker 📝
**Status:** PENDING  
**Priority:** MEDIUM  
**Effort:** 5-6 hours  
**Impact:** Build health visibility  

**Description:**
Track CI/CD metrics:
- Build pass rate
- Test coverage trends
- Deployment frequency
- Flaky test detection

**Implementation:**
```rust
#[tauri::command]
pub async fn get_ci_metrics(org: String, repos: Vec<String>) -> Result<CIMetrics, String>
```

**Files:**
- `src-tauri/src/integrations/github.rs`

**Dependencies:** BEAD-CTODASH-016

**Test:**
- GitHub Actions integration
- Travis/Circle CI support

---

#### BEAD-CTODASH-020: Security Alert Aggregator 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
**Effort:** 5-6 hours  
**Impact:** Proactive security monitoring  

**Description:**
Aggregate security alerts:
- Dependabot alerts
- Secret scanning hits
- Code scanning results
- Security advisory impact

**Implementation:**
```rust
#[tauri::command]
pub async fn get_security_alerts(org: String, repos: Vec<String>) -> Result<SecurityAlerts, String>
```

**Files:**
- `src-tauri/src/integrations/github.rs`

**Dependencies:** BEAD-CTODASH-016

**Test:**
- Alert severity classification
- Deduplication logic

---

### Frontend Components

#### BEAD-CTODASH-021: GitHub Metrics Dashboard ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 6-7 hours  
**Impact:** GitHub insights UI  

**Description:**
Create GitHub-specific panels:
- PR metrics panel
- Issue metrics panel
- CI/CD status panel
- Security alerts panel

**Files:**
- `src/lib/components/dashboard/GitHubMetrics.svelte` (NEW)

**Dependencies:** All Phase 2 backend tasks

**Test:**
- Real GitHub data
- Refresh behavior

---

#### BEAD-CTODASH-022: Enhanced Contributor Analytics ⚠️
**Status:** PENDING  
**Priority:** HIGH  
**Effort:** 5-6 hours  
**Impact:** Deeper team insights  

**Description:**
Enhance contributor view with:
- PR review participation
- Code review quality score
- Response time metrics
- Test contribution rate

**Files:**
- `src/lib/components/dashboard/ContributorMetrics.svelte` (UPDATE)

**Dependencies:** Phase 2 backend tasks

**Test:**
- Accuracy vs GitHub
- Performance

---

## Task Dependencies Graph

```
Phase 1 Backend:
BEAD-CTODASH-006 (Models)
    ↓
BEAD-CTODASH-001 (Org Overview) ──→ BEAD-CTODASH-002 (Health Calc)
    ↓                                      ↓
BEAD-CTODASH-003 (Project Detail)    BEAD-CTODASH-004 (Contributors)
    ↓                                      ↓
BEAD-CTODASH-005 (BEADS Integration)      ↓

Phase 1 Frontend:
BEAD-CTODASH-007 (Main Component)
    ↓
BEAD-CTODASH-008 (Overview Panel)
BEAD-CTODASH-009 (Project Grid)
BEAD-CTODASH-010 (Detail Modal)
BEAD-CTODASH-011 (Contributor Panel)
BEAD-CTODASH-012 (Debt Chart)
    ↓
BEAD-CTODASH-013 (State Management)
    ↓
BEAD-CTODASH-014 (Auto-Refresh)
BEAD-CTODASH-015 (Settings)

Phase 2:
BEAD-CTODASH-016 (GitHub Client)
    ↓
BEAD-CTODASH-017 (PR Metrics)
BEAD-CTODASH-018 (Issue Metrics)
BEAD-CTODASH-019 (CI Metrics)
BEAD-CTODASH-020 (Security Alerts)
    ↓
BEAD-CTODASH-021 (GitHub UI)
BEAD-CTODASH-022 (Enhanced Analytics)
```

---

## Implementation Order (Suggested)

### Sprint 1 (Week 1-2):
1. BEAD-CTODASH-006 (Models)
2. BEAD-CTODASH-001 (Org Overview Backend)
3. BEAD-CTODASH-002 (Health Calculator)
4. BEAD-CTODASH-007 (Main Component)
5. BEAD-CTODASH-008 (Overview Panel)

**Milestone:** Basic dashboard showing org-level metrics

### Sprint 2 (Week 2-3):
6. BEAD-CTODASH-003 (Project Detail Backend)
7. BEAD-CTODASH-005 (BEADS Integration)
8. BEAD-CTODASH-009 (Project Grid)
9. BEAD-CTODASH-010 (Detail Modal)
10. BEAD-CTODASH-012 (Debt Chart)

**Milestone:** Full project drill-down working

### Sprint 3 (Week 3-4):
11. BEAD-CTODASH-004 (Contributor Backend)
12. BEAD-CTODASH-011 (Contributor Panel)
13. BEAD-CTODASH-013 (State Management)
14. BEAD-CTODASH-014 (Auto-Refresh)
15. BEAD-CTODASH-015 (Settings)

**Milestone:** Phase 1 MVP Complete

---

## Testing Strategy

### Unit Tests
- Backend: Test each Tauri command in isolation
- Models: Test serialization/deserialization
- Utilities: Test git log parsing, BEADS filtering

### Integration Tests
- End-to-end: Load dashboard with real project data
- Multi-repo: Test with 10+ repos
- Performance: Test with 100+ repos

### Manual Testing
- Visual QA: All components render correctly
- Responsiveness: Works on different screen sizes
- Error scenarios: Missing dirs, no git, corrupted BEADS

---

## Success Criteria

**Phase 1 Complete When:**
- [ ] Dashboard loads with organization overview
- [ ] Can drill down into any project
- [ ] Contributor metrics accurate
- [ ] Technical debt trends visible
- [ ] All 100+ existing unit tests still pass
- [ ] No performance degradation in core app
- [ ] Documentation complete

**Phase 2 Complete When:**
- [ ] GitHub metrics integrated
- [ ] Security alerts visible
- [ ] PR/Issue analytics working
- [ ] Enhanced contributor metrics
- [ ] All Phase 2 tests passing

---

## Risk Mitigation

**Risk:** Dashboard slows down main app  
**Mitigation:** Lazy load dashboard, run scans in background

**Risk:** Git log parsing is slow  
**Mitigation:** Cache results, incremental updates only

**Risk:** Too much complexity in Phase 1  
**Mitigation:** Start with minimal viable features, iterate

**Risk:** Privacy concerns with individual metrics  
**Mitigation:** Make granularity configurable, anonymize option

---

## Related Documents

- Proposal: `openspec/changes/cto-dashboard/proposal.md`
- BEADS Tracker: `docs/BEADS_ISSUE_TRACKER.md`
- Project Scanner: `src/lib/components/ProjectScanner.svelte`
- OpenSpec: `openspec/project.md`

---

**Total Estimated Effort:**
- Phase 1: ~60 hours (3 weeks for 1 developer)
- Phase 2: ~80 hours (4 weeks for 1 developer)
- **Total:** ~140 hours

**Priority:** HIGH - Executive visibility critical for organizational health

**Status:** ✅ Ready for BEADS integration and implementation
