# Project Scaffolding Feature - Beads Issue Tracking

**Feature:** Project Scaffolding & Health Panel  
**Epic:** SCAFFOLD  
**Total Beads:** 13 (BEAD-SCAFFOLD-001 through BEAD-SCAFFOLD-013)  
**Total Effort:** 40-60 hours  
**Timeline:** 3 sprints (15 days)

---

## Beads Issue Map

### Phase 1: Project Initialization (20 hours)

#### BEAD-SCAFFOLD-001: UI Component - Project Creation Wizard
- **Type:** Frontend Feature
- **Priority:** P1 (Critical)
- **Effort:** 4 hours
- **Status:** TODO
- **Assignee:** @frontend-team
- **Files:** 
  - Create: `src/lib/components/ProjectScaffold.svelte`
  - Create: `src/lib/components/__tests__/ProjectScaffold.test.ts`
- **Acceptance Criteria:**
  - [ ] Form accepts all configuration fields
  - [ ] Validation works (project name, selections)
  - [ ] Template preview shows correctly
  - [ ] Progress bar shows during creation
  - [ ] Success message on completion
  - [ ] Error handling with clear messages
  - [ ] Responsive design passes
  - [ ] Unit tests: 80%+ coverage
- **Depends On:** None
- **Blocks:** BEAD-SCAFFOLD-006

#### BEAD-SCAFFOLD-002: Backend - Project Initialization Commands
- **Type:** Rust Backend Implementation
- **Priority:** P1 (Critical)
- **Effort:** 6 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Commands Implemented:**
  - [ ] `create_tauri_project(name, template)`
  - [ ] `setup_rust_backend(name, db_type)`
  - [ ] `setup_frontend(name, css_framework)`
  - [ ] `install_dependencies(name, pm)`
  - [ ] `setup_tailwind(name)`
  - [ ] `setup_database(name, db_type)`
- **Files:**
  - Create: `src-tauri/src/commands/scaffold.rs`
  - Create: `src-tauri/src/templates/` (directory)
  - Modify: `src-tauri/src/lib.rs` (add module)
- **Acceptance Criteria:**
  - [ ] All commands execute without errors
  - [ ] Cargo.toml created with Rust 2024 Edition
  - [ ] package.json created with modern deps
  - [ ] vite.config.ts configured correctly
  - [ ] Svelte 5 + TypeScript setup works
  - [ ] Database templates match DB type
  - [ ] All files have correct permissions
  - [ ] Logging tracks each step
  - [ ] Error handling is comprehensive
- **Depends On:** BEAD-SCAFFOLD-001
- **Blocks:** BEAD-SCAFFOLD-003, 004, 005

#### BEAD-SCAFFOLD-003: OpenCode Integration Setup
- **Type:** Configuration / Backend
- **Priority:** P1 (Critical)
- **Effort:** 3 hours
- **Status:** TODO
- **Assignee:** @devops-team
- **Deliverables:**
  - [ ] Generate `.opencode/agents.json`
  - [ ] Setup MCP servers configuration
  - [ ] Create custom CLI commands
  - [ ] Generate AGENTS.md documentation
  - [ ] Setup tools.json with available tools
- **Files:**
  - Create: `src-tauri/src/templates/.opencode/agents.json`
  - Create: `src-tauri/src/templates/.opencode/AGENTS.md`
  - Create: `src-tauri/src/templates/.opencode/mcp-servers.json`
- **Acceptance Criteria:**
  - [ ] `.opencode` directory created
  - [ ] agents.json is valid JSON
  - [ ] All custom commands documented
  - [ ] MCP servers properly configured
  - [ ] New projects can use OpenCode
  - [ ] agents.json passes schema validation
- **Depends On:** BEAD-SCAFFOLD-002
- **Blocks:** Project creation complete

#### BEAD-SCAFFOLD-004: Beads & OpenSpec Integration
- **Type:** Configuration / Backend
- **Priority:** P1 (Critical)
- **Effort:** 3 hours
- **Status:** TODO
- **Assignee:** @devops-team
- **Deliverables:**
  - [ ] Generate `.beads/config.json`
  - [ ] Initialize issue tracker structure
  - [ ] Setup `openspec/` directory
  - [ ] Create change request templates
  - [ ] Generate CONTRIBUTING.md
  - [ ] Setup workflow documentation
- **Files:**
  - Create: `src-tauri/src/templates/.beads/config.json`
  - Create: `src-tauri/src/templates/openspec/project.md`
  - Create: `src-tauri/src/templates/openspec/changes/proposal-template.md`
- **Acceptance Criteria:**
  - [ ] `.beads` directory created
  - [ ] config.json valid and complete
  - [ ] `openspec/` ready for use
  - [ ] Change templates present
  - [ ] CONTRIBUTING.md comprehensive
  - [ ] Workflow documentation clear
  - [ ] First issue can be created
- **Depends On:** BEAD-SCAFFOLD-002
- **Blocks:** Project creation complete

#### BEAD-SCAFFOLD-005: Documentation Generation
- **Type:** Backend Implementation
- **Priority:** P2 (High)
- **Effort:** 4 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Deliverables:**
  - [ ] Generate ARCHITECTURE.md
  - [ ] Generate COMPLIANCE.md
  - [ ] Generate API.md stub
  - [ ] Generate CONTRIBUTING.md
  - [ ] Generate SETUP.md
  - [ ] Generate TROUBLESHOOTING.md
  - [ ] Create CI/CD workflow files
- **Files:**
  - Create: `src-tauri/src/templates/docs/ARCHITECTURE.md`
  - Create: `src-tauri/src/templates/docs/COMPLIANCE.md`
  - Create: `src-tauri/src/templates/.github/workflows/test.yml`
  - Create: `src-tauri/src/templates/.github/workflows/build.yml`
  - Create: `src-tauri/src/templates/.github/workflows/deploy.yml`
- **Acceptance Criteria:**
  - [ ] All .md files generated
  - [ ] Templates have placeholders
  - [ ] CI/CD workflows valid YAML
  - [ ] Documentation comprehensive
  - [ ] All links are correct
  - [ ] Formatting consistent
  - [ ] Ready for project use
- **Depends On:** BEAD-SCAFFOLD-002
- **Blocks:** Project creation complete

---

### Phase 2: Health Checking (18 hours)

#### BEAD-SCAFFOLD-006: Health Check UI Component
- **Type:** Frontend Feature
- **Priority:** P1 (Critical)
- **Effort:** 4 hours
- **Status:** TODO
- **Assignee:** @frontend-team
- **Deliverables:**
  - [ ] Display health score (0-100)
  - [ ] Color-coded status indicator
  - [ ] File structure validation grid
  - [ ] Dependency status display
  - [ ] Configuration health summary
  - [ ] Recommendations list
  - [ ] "Scan Now" button with loading
  - [ ] Responsive layout
- **Files:**
  - Modify: `src/lib/components/ProjectScaffold.svelte` (add health tab)
  - Modify: `src/lib/components/__tests__/ProjectScaffold.test.ts`
- **Acceptance Criteria:**
  - [ ] Score displays 0-100
  - [ ] Status color changes per score
  - [ ] All critical files shown in grid
  - [ ] File status icons clear (✅/⚠️/❌)
  - [ ] Scan button works with loading
  - [ ] Recommendations formatted clearly
  - [ ] Mobile responsive
  - [ ] Unit tests pass
- **Depends On:** BEAD-SCAFFOLD-001
- **Blocks:** BEAD-SCAFFOLD-007

#### BEAD-SCAFFOLD-007: Health Check Backend Implementation
- **Type:** Rust Backend Implementation
- **Priority:** P1 (Critical)
- **Effort:** 5 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Commands Implemented:**
  - [ ] `check_project_health()` - Overall score
  - [ ] `check_file_structure()` - Validate files
  - [ ] `check_dependencies()` - Verify packages
  - [ ] `check_configuration()` - Validate configs
  - [ ] `check_compliance()` - Verify docs
  - [ ] `check_tooling()` - Verify integrations
- **Files:**
  - Create: `src-tauri/src/commands/health.rs`
  - Create: `src-tauri/src/utils/validators.rs`
  - Modify: `src-tauri/src/lib.rs`
- **Acceptance Criteria:**
  - [ ] Score calculation correct
  - [ ] Formula: (File×0.2 + Deps×0.2 + Config×0.2 + Compliance×0.2 + Tooling×0.2) × 100
  - [ ] Issues categorized correctly
  - [ ] Recommendations actionable
  - [ ] Performance: < 2 seconds
  - [ ] Comprehensive logging
  - [ ] Error handling robust
- **Depends On:** BEAD-SCAFFOLD-002
- **Blocks:** BEAD-SCAFFOLD-009

#### BEAD-SCAFFOLD-008: Configuration Validation
- **Type:** Rust Backend Implementation
- **Priority:** P2 (High)
- **Effort:** 4 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Validations:**
  - [ ] Cargo.toml structure
  - [ ] package.json structure
  - [ ] tauri.conf.json validity
  - [ ] TypeScript strict mode check
  - [ ] Security settings (CSP, etc.)
  - [ ] Database configuration
- **Files:**
  - Modify: `src-tauri/src/utils/validators.rs`
- **Acceptance Criteria:**
  - [ ] All validators return results
  - [ ] JSON parsing robust
  - [ ] Malformed files detected
  - [ ] Missing fields identified
  - [ ] Security checked
  - [ ] Clear error messages
- **Depends On:** BEAD-SCAFFOLD-002
- **Blocks:** BEAD-SCAFFOLD-009

#### BEAD-SCAFFOLD-009: Health Check Testing
- **Type:** QA / Testing
- **Priority:** P2 (High)
- **Effort:** 5 hours
- **Status:** TODO
- **Assignee:** @qa-team
- **Tests:**
  - [ ] Unit tests: validators
  - [ ] Unit tests: score calculation
  - [ ] Integration tests: full flow
  - [ ] Edge case tests: missing files
  - [ ] Edge case tests: invalid configs
  - [ ] Performance tests: < 2 seconds
  - [ ] Coverage: > 80%
- **Files:**
  - Create: `src-tauri/tests/health_check_tests.rs`
- **Acceptance Criteria:**
  - [ ] All tests pass
  - [ ] Coverage > 80%
  - [ ] Performance acceptable
  - [ ] Edge cases handled
  - [ ] No flaky tests
- **Depends On:** BEAD-SCAFFOLD-007, BEAD-SCAFFOLD-008
- **Blocks:** Phase 2 complete

---

### Phase 3: Drift Detection & Repair (15 hours)

#### BEAD-SCAFFOLD-010: Drift Detection Algorithm
- **Type:** Rust Backend Implementation
- **Priority:** P1 (Critical)
- **Effort:** 5 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Algorithms:**
  - [ ] File drift detection
  - [ ] Dependency drift detection
  - [ ] Configuration drift detection
  - [ ] Documentation drift detection
  - [ ] Code quality drift detection
  - [ ] Severity calculation
- **Files:**
  - Create: `src-tauri/src/commands/drift.rs`
  - Create: `src-tauri/src/utils/drift_detection.rs`
- **Acceptance Criteria:**
  - [ ] All drift types detected
  - [ ] Severity assigned correctly
  - [ ] False positives < 5%
  - [ ] Performance acceptable
  - [ ] Issue descriptions clear
  - [ ] Comprehensive logging
- **Depends On:** BEAD-SCAFFOLD-007
- **Blocks:** BEAD-SCAFFOLD-011

#### BEAD-SCAFFOLD-011: Auto-Repair Implementation
- **Type:** Rust Backend Implementation
- **Priority:** P1 (Critical)
- **Effort:** 5 hours
- **Status:** TODO
- **Assignee:** @backend-team
- **Repairs:**
  - [ ] File recreation from templates
  - [ ] Dependency update logic
  - [ ] Configuration restoration
  - [ ] Documentation regeneration
  - [ ] Safety checks (non-destructive)
  - [ ] Rollback capability
- **Files:**
  - Create: `src-tauri/src/commands/repair.rs`
  - Create: `src-tauri/src/utils/repair_engine.rs`
- **Acceptance Criteria:**
  - [ ] Auto-repairs are safe
  - [ ] Success rate > 90%
  - [ ] Repairs documented
  - [ ] Rollback works
  - [ ] No destructive changes
  - [ ] User notified of changes
- **Depends On:** BEAD-SCAFFOLD-010
- **Blocks:** BEAD-SCAFFOLD-012

#### BEAD-SCAFFOLD-012: Drift Repair UI Component
- **Type:** Frontend Feature
- **Priority:** P2 (High)
- **Effort:** 3 hours
- **Status:** TODO
- **Assignee:** @frontend-team
- **UI Elements:**
  - [ ] Display drift results
  - [ ] Show issues by category
  - [ ] List auto-fixable items
  - [ ] Show manual review items
  - [ ] "Repair" button
  - [ ] Repair progress display
  - [ ] Repair results summary
- **Files:**
  - Modify: `src/lib/components/ProjectScaffold.svelte` (add repair tab)
  - Modify: `src/lib/components/__tests__/ProjectScaffold.test.ts`
- **Acceptance Criteria:**
  - [ ] Issues displayed clearly
  - [ ] Categories obvious
  - [ ] Repair button works
  - [ ] Progress visible
  - [ ] Results clear
  - [ ] Next steps shown
- **Depends On:** BEAD-SCAFFOLD-011
- **Blocks:** Phase 3 complete

#### BEAD-SCAFFOLD-013: Drift & Repair Testing
- **Type:** QA / Testing
- **Priority:** P2 (High)
- **Effort:** 4 hours
- **Status:** TODO
- **Assignee:** @qa-team
- **Tests:**
  - [ ] Drift detection tests
  - [ ] Auto-repair tests
  - [ ] Edge case tests
  - [ ] Safety/rollback tests
  - [ ] Integration tests
  - [ ] Coverage: > 80%
- **Files:**
  - Create: `src-tauri/tests/drift_repair_tests.rs`
- **Acceptance Criteria:**
  - [ ] All tests pass
  - [ ] Coverage > 80%
  - [ ] No destructive changes in tests
  - [ ] Repairs verified
- **Depends On:** BEAD-SCAFFOLD-010, BEAD-SCAFFOLD-011
- **Blocks:** Release ready

---

## Sprint Schedule

### Sprint 1: Project Initialization (8 days)
```
Day 1: Spec Approval + Planning
Day 2: BEAD-SCAFFOLD-001 (UI)
Day 3: BEAD-SCAFFOLD-002 (Backend)
Day 4: BEAD-SCAFFOLD-003 (OpenCode)
Day 5: BEAD-SCAFFOLD-004 (Beads/OpenSpec)
Day 6: BEAD-SCAFFOLD-005 (Docs)
Day 7-8: Testing + Bug Fixes
```

**Gate Criteria:**
- [ ] All 5 backend beads complete
- [ ] UI component functional
- [ ] Project creation succeeds
- [ ] All generated files present
- [ ] Initial build succeeds
- [ ] 80%+ test coverage

---

### Sprint 2: Health Checking (6 days)
```
Day 1-2: BEAD-SCAFFOLD-006 (Health UI)
Day 3-4: BEAD-SCAFFOLD-007 (Health Backend)
Day 5: BEAD-SCAFFOLD-008 (Validation)
Day 6: BEAD-SCAFFOLD-009 (Testing)
```

**Gate Criteria:**
- [ ] Health check component complete
- [ ] Score calculation verified
- [ ] All validators working
- [ ] 80%+ test coverage
- [ ] Performance < 2 seconds

---

### Sprint 3: Drift Detection & Repair (6 days)
```
Day 1-2: BEAD-SCAFFOLD-010 (Drift Detection)
Day 3-4: BEAD-SCAFFOLD-011 (Auto-Repair)
Day 5: BEAD-SCAFFOLD-012 (UI)
Day 6: BEAD-SCAFFOLD-013 (Testing)
```

**Gate Criteria:**
- [ ] Drift detection accurate
- [ ] Auto-repairs working safely
- [ ] 80%+ test coverage
- [ ] No destructive changes
- [ ] User can repair drift

---

## Tracking & Reporting

### Daily Standup
- Report progress on current BEAD
- Identify blockers
- Update Beads status
- Adjust timeline if needed

### Gate Approval Process
1. All acceptance criteria met
2. Code reviewed (80%+ test coverage)
3. Integration tested
4. Performance verified
5. Beads marked COMPLETE
6. Move to next BEAD

### Reporting Template
```
BEAD-SCAFFOLD-###: [Task Name]
Status: [TODO/IN PROGRESS/BLOCKED/COMPLETE]
Progress: X%
Blockers: [List any blockers]
Next Steps: [What's next]
```

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Spec Approval | Day 1 |
| Sprint 1 Completion | Day 8 |
| Sprint 2 Completion | Day 14 |
| Sprint 3 Completion | Day 20 |
| Test Coverage | > 80% |
| Performance (Health Check) | < 2 seconds |
| Auto-Repair Success Rate | > 90% |
| Zero Critical Bugs | Release |

---

**Document Version:** 1.0  
**Created:** October 27, 2025  
**Status:** READY FOR DEVELOPMENT
