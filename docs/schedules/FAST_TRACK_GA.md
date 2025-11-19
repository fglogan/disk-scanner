# 🚀 Fast Track to GA: Disk Bloat Scanner v1.0

**Goal:** Accelerate development to reach General Availability (v1.0) within 4 weeks.
**Strategy:** Parallelize stability fixes with core feature implementation using multi-agent scheduling.

---

## 📅 Executive Schedule

| Timeline | Phase | Focus | Key Deliverables |
|----------|-------|-------|------------------|
| **Week 1** | **Stability** | Technical Debt & Core UX | Async I/O, Cancellation, Progress Bars, Error UI |
| **Week 2** | **Features** | PACS (Compliance) | Deep Project Analyzer, Baseline System |
| **Week 3** | **Experience** | Tray Agent & Modernization | Background Scanning, System Tray, Svelte Polish |
| **Week 4** | **Release** | Documentation & QA | User Guide, Installers, Smoke Tests, v1.0 Launch |

---

## 🤖 Agent Assignments & Tasks

### 🗓️ Week 1: Stability & Technical Debt (The Foundation)
*Objective: Ensure the application is robust, responsive, and crash-proof.*

**Core Coder (`core-coder`)**
- [ ] **BEAD-009 (High):** Make `dir_size()` async to prevent UI freezing.
- [ ] **BEAD-010 (High):** Implement Scan Cancellation (`CancellationToken`).
- [ ] **BEAD-011 (High):** Real progress tracking via Tauri events.
- [ ] **BEAD-013 (High):** Add React Error Boundaries to Svelte components.
- [ ] **BEAD-019 (High):** Optimize Mutex usage in `scan_bloat`.
- [ ] **BEAD-020 (High):** Implement chunked file hashing to reduce memory spikes.

**VOS Architect (`vos-architect`)**
- [ ] Review Week 1 implementation for thread safety.
- [ ] Finalize specifications for **Tray Menu** and **Monaco Editor**.

---

### 🗓️ Week 2: PACS Implementation (The Differentiator)
*Objective: Deliver the "Project Auditor & Compliance Scanner" value proposition.*

**Core Coder (`core-coder`)**
- [ ] **BEAD-PACS-001:** Implement PACS data models & error types.
- [ ] **BEAD-PACS-002:** Build Spec Parser (Markdown/Frontmatter/JSON).
- [ ] **BEAD-PACS-003:** Implement Compliance Validators (EDGS, LAIO, TES).
- [ ] **BEAD-PACS-004:** Create `audit_project_deep` command.
- [ ] **BEAD-PACS-005:** Implement Immutable Baseline Storage (CDX Zip).

**VOS Architect (`vos-architect`)**
- [ ] Validate PACS audit report formats (Markdown/JSON).
- [ ] Design "Drift Detection" logic for Week 3.
- [ ] Update `ARCHITECTURE.md` with PACS module details.

---

### 🗓️ Week 3: Experience & Automation
*Objective: Make the tool invisible (Tray) and beautiful (UI Polish).*

**Core Coder (`core-coder`)**
- [ ] **BEAD-PACS-008:** Implement Drift Detection & Comparison.
- [ ] **Tray Agent:** Implement System Tray menu (Rust).
- [ ] **Tray Agent:** Background scheduling for audits.
- [ ] **Phase 3:** Refactor key Svelte components to Svelte 5 runes (if feasible) or clean Svelte 4.
- [ ] **UI:** Add "Restore from Trash" UI (BEAD-035).

**VOS Architect (`vos-architect`)**
- [ ] Draft `USER_GUIDE.md` including PACS features.
- [ ] Define v1.0 Release Criteria (Smoke Test Plan).

---

### 🗓️ Week 4: GA Release Prep
*Objective: Polish, Package, and Publish.*

**Core Coder (`core-coder`)**
- [ ] **BEAD-036:** Add basic crash reporting/telemetry (optional).
- [ ] **BEAD-038:** Cross-platform verification (macOS/Windows).
- [ ] **Build:** Optimize release builds (code signing, notarization).
- [ ] **Bugfix:** Address any regressions from Weeks 1-3.

**VOS Architect (`vos-architect`)**
- [ ] Final Security Audit (reviewing CSP, Permissions).
- [ ] Update `README.md` with v1.0 features.
- [ ] Create Release Notes for v1.0.0.

---

## 🚀 Immediate Next Actions

1.  **Authorize Week 1 Sprint:**
    -   Run `BEAD-009` (Async `dir_size`) as the first task.
    -   Run `BEAD-011` (Progress Tracking) immediately after.

2.  **Prepare PACS Workspace:**
    -   Create `src-tauri/src/pacs/` module structure.

3.  **Status Check:**
    -   Daily `AGENTS.md` updates to track progress against this schedule.
