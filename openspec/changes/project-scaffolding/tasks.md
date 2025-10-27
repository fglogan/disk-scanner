# Project Scaffolding & Health Panel - Implementation Tasks

**Specification:** `openspec/changes/project-scaffolding/proposal.md`  
**Status:** READY FOR IMPLEMENTATION  
**Total Effort:** 40-60 hours across 3 sprints  
**Tracking:** Beads Issue Tracker (BEAD-SCAFFOLD-*)

---

## Task Breakdown & Beads Assignments

### Phase 1: Project Initialization (20 hours)

#### BEAD-SCAFFOLD-001: UI Component - Project Wizard
**Type:** Frontend Feature  
**Effort:** 4 hours  
**Priority:** P1  
**Assignee:** Frontend Team  
**Dependencies:** None  

**Deliverables:**
- [ ] Create ProjectScaffold.svelte component
- [ ] Implement project creation form with validation
- [ ] Add configuration preview section
- [ ] Implement progress bar animation
- [ ] Add success/error feedback messages
- [ ] Responsive design (mobile/tablet/desktop)
- [ ] Unit tests (80%+ coverage)

**Acceptance Criteria:**
- Form accepts all required fields
- Validation works for project name (alphanumeric + hyphens)
- Template/backend/database/styling selections work
- Progress bar shows creation steps
- Success message displays on completion
- Error messages are clear and actionable

**Files to Create:**
- `src/lib/components/ProjectScaffold.svelte` (component)
- `src/lib/components/__tests__/ProjectScaffold.test.ts` (tests)

---

#### BEAD-SCAFFOLD-002: Backend Commands - Project Initialization
**Type:** Rust Backend  
**Effort:** 6 hours  
**Priority:** P1  
**Assignee:** Backend Team  
**Dependencies:** BEAD-SCAFFOLD-001  

**Deliverables:**
- [ ] `create_tauri_project()` - Create Tauri project directory
- [ ] `setup_rust_backend()` - Configure Cargo.toml for Tauri 2.8.5
- [ ] `setup_frontend()` - Setup Svelte + TypeScript + Vite
- [ ] `install_dependencies()` - Run npm/yarn/pnpm install
- [ ] `setup_tailwind()` - Configure TailwindCSS
- [ ] `setup_database()` - Create database schema templates
- [ ] Error handling and logging for each step

**Acceptance Criteria:**
- Commands execute without errors
- All files created with correct permissions
- Cargo.toml has Rust 2024 Edition
- package.json has modern dependencies
- Vite config optimized for Tauri
- Database templates match selected DB type
- Logging shows progress at each step

**Files to Create:**
- `src-tauri/src/commands/scaffold.rs` (new module)
- `src-tauri/src/templates/` (directory with templates)

---

#### BEAD-SCAFFOLD-003: OpenCode Integration
**Type:** Configuration  
**Effort:** 3 hours  
**Priority:** P1  
**Assignee:** DevOps/Backend  
**Dependencies:** BEAD-SCAFFOLD-002  

**Deliverables:**
- [ ] Generate `.opencode/agents.json` for new projects
- [ ] Setup MCP servers configuration
- [ ] Add custom CLI commands (project-check, health-scan)
- [ ] Create AGENTS.md with documentation
- [ ] Generate `tools.json` with available tools
- [ ] Setup command templates

**Acceptance Criteria:**
- `.opencode` directory created in project
- `agents.json` valid and formatted
- All custom commands documented
- MCP servers properly configured
- AGENTS.md explains available agents
- New projects can use all OpenCode features

**Files to Create:**
- `src-tauri/src/templates/.opencode/agents.json`
- `src-tauri/src/templates/.opencode/AGENTS.md`
- `src-tauri/src/templates/.opencode/mcp-servers.json`

---

#### BEAD-SCAFFOLD-004: Beads & OpenSpec Integration
**Type:** Configuration  
**Effort:** 3 hours  
**Priority:** P1  
**Assignee:** DevOps/Backend  
**Dependencies:** BEAD-SCAFFOLD-002  

**Deliverables:**
- [ ] Generate `.beads/config.json` for new projects
- [ ] Initialize issue tracker structure
- [ ] Setup OpenSpec directory and templates
- [ ] Create change request templates
- [ ] Generate CONTRIBUTING.md
- [ ] Setup workflow documentation

**Acceptance Criteria:**
- `.beads` directory created with config
- Issue tracker ready to use
- `openspec/` directory initialized
- Change templates present
- Workflow documentation clear
- First issue can be created immediately

**Files to Create:**
- `src-tauri/src/templates/.beads/config.json`
- `src-tauri/src/templates/openspec/project.md`
- `src-tauri/src/templates/openspec/changes/proposal-template.md`

---

#### BEAD-SCAFFOLD-005: Documentation Generation
**Type:** Backend / Templates  
**Effort:** 4 hours  
**Priority:** P2  
**Assignee:** Backend/DevOps  
**Dependencies:** BEAD-SCAFFOLD-002  

**Deliverables:**
- [ ] Generate ARCHITECTURE.md template
- [ ] Generate COMPLIANCE.md template
- [ ] Generate API.md stub
- [ ] Generate CONTRIBUTING.md
- [ ] Generate SETUP.md with quick-start
- [ ] Generate TROUBLESHOOTING.md
- [ ] Setup CI/CD workflow files (.github/workflows/)

**Acceptance Criteria:**
- All .md files created with proper structure
- Templates have placeholders for project-specific info
- CI/CD workflows for test/build/deploy
- Documentation is comprehensive but concise
- All links are correct
- Formatting is consistent

**Files to Create:**
- `src-tauri/src/templates/docs/ARCHITECTURE.md`
- `src-tauri/src/templates/docs/COMPLIANCE.md`
- `src-tauri/src/templates/.github/workflows/test.yml`
- `src-tauri/src/templates/.github/workflows/build.yml`

---

### Phase 2: Health Checking (18 hours)

#### BEAD-SCAFFOLD-006: Health Check UI Component
**Type:** Frontend Feature  
**Effort:** 4 hours  
**Priority:** P1  
**Assignee:** Frontend Team  
**Dependencies:** BEAD-SCAFFOLD-001  

**Deliverables:**
- [ ] Display health score with color-coded status
- [ ] Show file structure validation grid
- [ ] Display dependency status indicators
- [ ] Show configuration health summary
- [ ] List recommendations
- [ ] Add "Scan Now" button with loading state
- [ ] Responsive layout

**Acceptance Criteria:**
- Score displays 0-100 with visual indicator
- Status color changes based on score (green/blue/orange/red)
- File grid shows all critical files
- File status icons (✅/⚠️/❌) clear
- Scan button works and shows loading
- Recommendations formatted clearly

**Files to Modify:**
- `src/lib/components/ProjectScaffold.svelte` (add health tab)
- `src/lib/components/__tests__/ProjectScaffold.test.ts` (add health tests)

---

#### BEAD-SCAFFOLD-007: Backend - Health Check Implementation
**Type:** Rust Backend  
**Effort:** 5 hours  
**Priority:** P1  
**Assignee:** Backend Team  
**Dependencies:** BEAD-SCAFFOLD-002  

**Deliverables:**
- [ ] `check_project_health()` - Calculate overall score
- [ ] `check_file_structure()` - Validate critical files
- [ ] `check_dependencies()` - Verify packages installed
- [ ] `check_configuration()` - Validate config files
- [ ] `check_compliance()` - Verify documentation
- [ ] `check_tooling()` - Verify OpenCode/Beads/OpenSpec
- [ ] Score calculation algorithm
- [ ] Issue categorization logic

**Acceptance Criteria:**
- All check functions return correct status
- Score calculation: (File×0.2 + Deps×0.2 + Config×0.2 + Compliance×0.2 + Tooling×0.2) × 100
- Issues categorized correctly
- Recommendations are actionable
- Performance: completes in < 2 seconds
- Comprehensive logging

**Files to Create:**
- `src-tauri/src/commands/health.rs` (new module)
- `src-tauri/src/utils/validators.rs` (new module)

**Tests:**
- [ ] Unit tests for score calculation
- [ ] Unit tests for issue detection
- [ ] Integration tests with real project

---

#### BEAD-SCAFFOLD-008: Database & Config Validation
**Type:** Rust Backend  
**Effort:** 4 hours  
**Priority:** P2  
**Assignee:** Backend Team  
**Dependencies:** BEAD-SCAFFOLD-007  

**Deliverables:**
- [ ] Validate Cargo.toml structure
- [ ] Validate package.json structure
- [ ] Validate tauri.conf.json
- [ ] Check TypeScript strict mode
- [ ] Verify security settings (CSP, etc.)
- [ ] Validate database config

**Acceptance Criteria:**
- All validators return clear results
- JSON parsing handles errors gracefully
- Malformed files detected
- Missing required fields identified
- Security recommendations included

**Files to Modify:**
- `src-tauri/src/utils/validators.rs` (add validators)

---

#### BEAD-SCAFFOLD-009: Testing - Health Check
**Type:** Testing  
**Effort:** 5 hours  
**Priority:** P2  
**Assignee:** QA/Backend  
**Dependencies:** BEAD-SCAFFOLD-007, BEAD-SCAFFOLD-008  

**Deliverables:**
- [ ] Unit tests (validators, score calculation)
- [ ] Integration tests (full health check flow)
- [ ] Edge case tests (missing files, invalid configs)
- [ ] Performance tests (< 2 second scan)
- [ ] Test coverage > 80%

**Acceptance Criteria:**
- All tests pass
- Coverage > 80%
- Performance acceptable
- Edge cases handled

**Files to Create:**
- `src-tauri/tests/health_check_tests.rs`

---

### Phase 3: Drift Detection & Repair (15 hours)

#### BEAD-SCAFFOLD-010: Drift Detection Algorithm
**Type:** Rust Backend  
**Effort:** 5 hours  
**Priority:** P1  
**Assignee:** Backend Team  
**Dependencies:** BEAD-SCAFFOLD-007  

**Deliverables:**
- [ ] `detect_project_drift()` - Compare with canonical
- [ ] File drift detection
- [ ] Dependency drift detection
- [ ] Configuration drift detection
- [ ] Documentation drift detection
- [ ] Code quality drift detection
- [ ] Drift severity calculation

**Acceptance Criteria:**
- All drift types detected correctly
- Severity assigned properly
- False positives < 5%
- Performance acceptable
- Clear issue descriptions

**Files to Create:**
- `src-tauri/src/commands/drift.rs` (new module)
- `src-tauri/src/utils/drift_detection.rs` (new module)

---

#### BEAD-SCAFFOLD-011: Auto-Repair Implementation
**Type:** Rust Backend  
**Effort:** 5 hours  
**Priority:** P1  
**Assignee:** Backend Team  
**Dependencies:** BEAD-SCAFFOLD-010  

**Deliverables:**
- [ ] `auto_repair_files()` - Fix safe issues
- [ ] File recreation from templates
- [ ] Dependency update logic
- [ ] Configuration restoration
- [ ] Documentation regeneration
- [ ] Safety checks (no destructive changes)
- [ ] Repair report generation

**Acceptance Criteria:**
- Auto-repairs are safe and non-destructive
- Success rate > 90%
- Repairs documented
- Rollback capability
- User notified of changes

**Files to Create:**
- `src-tauri/src/commands/repair.rs` (new module)
- `src-tauri/src/utils/repair_engine.rs` (new module)

---

#### BEAD-SCAFFOLD-012: Drift Repair UI
**Type:** Frontend Feature  
**Effort:** 3 hours  
**Priority:** P2  
**Assignee:** Frontend Team  
**Dependencies:** BEAD-SCAFFOLD-011  

**Deliverables:**
- [ ] Display drift detection results
- [ ] Show issues by category
- [ ] List auto-fixable items
- [ ] Show manual review items
- [ ] Add "Repair" button
- [ ] Display repair progress
- [ ] Show repair results

**Acceptance Criteria:**
- Issues displayed clearly
- Categories are obvious
- Repair button triggers backend
- Progress bar shows progress
- Results show what was fixed
- Clear next steps

**Files to Modify:**
- `src/lib/components/ProjectScaffold.svelte` (add repair tab)

---

#### BEAD-SCAFFOLD-013: Testing - Drift & Repair
**Type:** Testing  
**Effort:** 4 hours  
**Priority:** P2  
**Assignee:** QA/Backend  
**Dependencies:** BEAD-SCAFFOLD-010, BEAD-SCAFFOLD-011  

**Deliverables:**
- [ ] Drift detection tests
- [ ] Auto-repair tests
- [ ] Edge case tests
- [ ] Rollback/safety tests
- [ ] Integration tests
- [ ] Test coverage > 80%

**Acceptance Criteria:**
- All tests pass
- Coverage > 80%
- No destructive changes
- Repairs verified

**Files to Create:**
- `src-tauri/tests/drift_repair_tests.rs`

---

## Implementation Order

### Sprint 1 Week (Days 1-5)
```
Day 1: Spec Approval + Planning
Day 2-3: BEAD-SCAFFOLD-001 (UI) + BEAD-SCAFFOLD-002 (Backend Cmds)
Day 4: BEAD-SCAFFOLD-003 + BEAD-SCAFFOLD-004 (Integrations)
Day 5: BEAD-SCAFFOLD-005 (Docs) + Testing
```

### Sprint 2 Week (Days 6-11)
```
Day 6-7: BEAD-SCAFFOLD-006 (Health UI) + BEAD-SCAFFOLD-007 (Health Backend)
Day 8-9: BEAD-SCAFFOLD-008 (Validation) + BEAD-SCAFFOLD-009 (Testing)
Day 10-11: Integration Testing + Bug Fixes
```

### Sprint 3 Week (Days 12-15)
```
Day 12-13: BEAD-SCAFFOLD-010 (Drift Detection) + BEAD-SCAFFOLD-011 (Auto-Repair)
Day 14: BEAD-SCAFFOLD-012 (UI) + BEAD-SCAFFOLD-013 (Testing)
Day 15: Final Testing + Release Prep
```

---

## Risk & Mitigation

| Task | Risk | Mitigation |
|------|------|-----------|
| BEAD-SCAFFOLD-002 | Dependency conflicts | Test with multiple Rust/Node versions |
| BEAD-SCAFFOLD-005 | Platform-specific paths | Test on Mac/Linux/Windows |
| BEAD-SCAFFOLD-010 | False positive drift | Extensive edge case testing |
| BEAD-SCAFFOLD-011 | Destructive auto-repairs | Implement safety checks, no-op mode |
| All | Tauri version changes | Pin versions, monitor breaking changes |

---

## Success Criteria Checklist

- [ ] All 13 tasks completed
- [ ] Spec requirements met 100%
- [ ] Code coverage > 80%
- [ ] Zero security vulnerabilities
- [ ] Performance targets met
- [ ] Documentation complete
- [ ] User testing passed
- [ ] Release notes prepared

---

**Document Version:** 1.0  
**Created:** October 27, 2025  
**Status:** READY FOR SPRINT PLANNING
