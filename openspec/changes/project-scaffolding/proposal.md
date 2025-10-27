# Project Scaffolding & Health Panel - Feature Proposal

**Status:** PROPOSAL  
**Date:** October 27, 2025  
**Priority:** HIGH  
**Effort:** 40-60 hours  
**Impact:** HIGH (Core feature for project lifecycle management)

---

## Executive Summary

Implement a comprehensive **Project Scaffolding & Health Panel** that enables users to:

1. **Initialize new Tauri projects** with production-ready configuration
   - Tauri 2.8.5 with Rust 2024 Edition
   - Svelte 5 + TypeScript + TailwindCSS/HTML+CSS
   - SQLite or PostgreSQL backend support
   - Pre-configured OpenCode, Beads, OpenSpec integration
   - Compliance documentation pre-populated
   - CI/CD GitHub Actions templates

2. **Validate project health** with comprehensive diagnostics
   - Check critical file structure
   - Validate configuration files
   - Detect missing dependencies
   - Security compliance verification
   - Generate health score (0-100)

3. **Auto-repair pathological drift** with AI assistance
   - Detect configuration divergence
   - Fix missing/malformed files
   - Update outdated dependencies
   - Correct security issues
   - Restore compliance docs

---

## Problem Statement

### Current Pain Points

1. **Manual Project Setup**
   - Developers manually configure new Tauri projects
   - Risk of missing critical setup steps
   - Inconsistent project structure across team
   - Missing best practices/conventions

2. **Configuration Drift**
   - Projects diverge from canonical structure over time
   - Hard to detect when/why drift occurred
   - Manual repair is time-consuming and error-prone
   - No clear path to restore to known-good state

3. **Tooling Integration**
   - OpenCode not automatically configured
   - Beads issue tracking not initialized
   - OpenSpec workflow templates missing
   - Compliance docs not pre-populated
   - CI/CD pipelines need manual setup

4. **Onboarding Friction**
   - New developers spend hours setting up projects
   - Knowledge of "correct" structure scattered
   - Multiple tools require separate initialization
   - No validation that setup is complete

---

## Proposed Solution

### Feature 1: Project Initialization (New Projects)

#### Inputs
```yaml
Project Configuration:
  - name: string (project name)
  - template: 'tauri' | 'svelte' | 'vite-react' | 'full-stack'
  - backend: 'rust' | 'node'
  - database: 'none' | 'sqlite' | 'postgres' | 'mongodb'
  - styling: 'tailwind' | 'vanilla-css'
  - packageManager: 'npm' | 'yarn' | 'pnpm'
```

#### Process Flow

```
1. Validate Configuration
   ↓
2. Create Project Directory
   ↓
3. Initialize Git Repository
   ↓
4. Setup Rust Backend (2024 Edition)
   - Create Cargo.toml with modern deps
   - Setup Tauri 2.8.5 configuration
   - Configure dev server & build output
   ↓
5. Setup Frontend (Svelte 5 + TypeScript)
   - Create Vite configuration
   - Setup TypeScript strict mode
   - Configure CSS/TailwindCSS if selected
   ↓
6. Install Dependencies
   - npm/yarn/pnpm install
   - Cargo build (initial)
   ↓
7. Setup Database (if selected)
   - SQLite: Create migrations structure
   - PostgreSQL: Connection config template
   - MongoDB: Driver setup
   ↓
8. Initialize OpenCode
   - Create .opencode directory
   - Setup agents.json with custom agents
   - Configure MCP servers
   - Add custom CLI commands
   ↓
9. Initialize Beads
   - Create .beads directory
   - Setup tracking configuration
   - Initialize issue tracker
   ↓
10. Initialize OpenSpec
    - Create openspec/ directory
    - Add change templates
    - Setup workflow
    ↓
11. Generate Documentation
    - Create /docs directory
    - Add ARCHITECTURE.md template
    - Add COMPLIANCE.md template
    - Add CONTRIBUTING.md template
    - Add API documentation stubs
    ↓
12. Setup CI/CD
    - Create .github/workflows directory
    - Add GitHub Actions templates
    - Configure test/build/deploy
    ↓
13. Initial Build & Verification
    - Run `cargo build`
    - Run `npm run build`
    - Verify all tooling works
    ↓
14. Create Initial Commits
    - Initial setup commit
    - Document in Beads
```

#### Deliverables

**Project Structure:**
```
my-tauri-app/
├── .git/                          # Git repository
├── .github/
│   └── workflows/                 # CI/CD pipelines
│       ├── test.yml
│       ├── build.yml
│       └── deploy.yml
├── .opencode/                     # OpenCode config
│   ├── agents.json
│   ├── mcp-servers.json
│   ├── tools.json
│   ├── commands/
│   │   ├── project-check.js
│   │   ├── health-scan.js
│   │   └── repair-drift.js
│   └── AGENTS.md
├── .beads/                        # Beads tracking
│   ├── config.json
│   └── tracker.json
├── openspec/                      # OpenSpec workflow
│   ├── project.md
│   └── changes/
│       └── templates/
├── docs/                          # Documentation
│   ├── ARCHITECTURE.md
│   ├── COMPLIANCE.md
│   ├── CONTRIBUTING.md
│   ├── API.md
│   ├── SETUP.md
│   └── TROUBLESHOOTING.md
├── src/                           # Svelte frontend
│   ├── lib/
│   ├── App.svelte
│   ├── main.js
│   └── app.css
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   └── utils/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── public/                        # Static assets
├── .prettierrc                    # Code formatting
├── .eslintrc.cjs                 # Linting
├── package.json                   # Node dependencies
├── Cargo.toml                     # Workspace config
├── tsconfig.json                  # TypeScript config
├── tailwind.config.js             # Tailwind (if selected)
├── vite.config.ts                # Vite config
├── svelte.config.js              # Svelte config
├── README.md                      # Project README
└── .gitignore                     # Git ignore rules
```

**Configuration Files Generated:**
- `Cargo.toml` - Rust 2024 Edition with Tauri 2.8.5
- `tauri.conf.json` - Security hardened, CSP enabled
- `package.json` - Modern Node tooling
- `tsconfig.json` - Strict TypeScript mode
- `vite.config.ts` - Optimized build config
- `tailwind.config.js` - TailwindCSS setup (if selected)
- `.github/workflows/*.yml` - CI/CD pipelines

---

### Feature 2: Project Health Check

#### Health Metrics

**File Structure Validation:**
- ✅/❌ Critical files present (Cargo.toml, package.json, etc.)
- ✅/❌ Directory structure correct
- ✅/❌ Configuration files valid JSON/TOML

**Dependency Status:**
- ✅ All packages installed
- ✅ No outdated dependencies
- ✅ No security vulnerabilities
- ✅ Rust toolchain compatible

**Configuration Validation:**
- ✅ CSP enabled in tauri.conf.json
- ✅ TypeScript strict mode enabled
- ✅ Proper error handling configured
- ✅ Database migrations configured (if applicable)

**Compliance Checks:**
- ✅ Documentation exists and current
- ✅ Contributing guidelines present
- ✅ License file present
- ✅ CHANGELOG maintained

**Tooling Status:**
- ✅ OpenCode initialized
- ✅ Beads tracking setup
- ✅ OpenSpec workflow ready
- ✅ CI/CD pipelines configured

#### Health Score Calculation

```
Score = (FileStructure × 0.2 + Dependencies × 0.2 + Configuration × 0.2 + 
         Compliance × 0.2 + Tooling × 0.2) × 100

Score Ranges:
- 90-100: Excellent ✅ (Green)
- 70-89:  Good ✅ (Blue)
- 40-69:  Warning ⚠️ (Orange)
- 0-39:   Critical ❌ (Red)
```

#### Health Report Output

```json
{
  "score": 85,
  "status": "good",
  "timestamp": "2025-10-27T10:30:00Z",
  "categories": {
    "fileStructure": { "score": 90, "status": "excellent" },
    "dependencies": { "score": 80, "status": "good" },
    "configuration": { "score": 85, "status": "good" },
    "compliance": { "score": 75, "status": "good" },
    "tooling": { "score": 85, "status": "good" }
  },
  "issues": [
    "TypeScript: 'unused variable' warnings detected",
    "Dependency: lodash-4.17.19 has known vulnerabilities"
  ],
  "recommendations": [
    "Run `cargo clippy` to fix Rust warnings",
    "Update lodash to 4.17.21",
    "Add CHANGELOG entries for v1.1.0"
  ]
}
```

---

### Feature 3: Pathological Drift Detection & Repair

#### Drift Categories

1. **File Drift**
   - Missing critical files
   - Deleted necessary directories
   - Corrupted configuration files
   - **Fix:** Recreate from templates

2. **Dependency Drift**
   - Outdated packages
   - Missing transitive dependencies
   - Version conflicts
   - **Fix:** Update Cargo.toml / package.json

3. **Configuration Drift**
   - Changed CSP settings
   - Disabled TypeScript strict mode
   - Modified database connection
   - **Fix:** Validate and restore from canonical

4. **Documentation Drift**
   - Outdated README
   - Missing API docs
   - Stale setup instructions
   - **Fix:** Regenerate from templates

5. **Code Quality Drift**
   - Disabled ESLint rules
   - Changed Prettier config
   - Removed test files
   - **Fix:** Restore rules and configs

#### Detection Algorithm

```
For each critical component:
  1. Calculate canonical hash/checksum
  2. Compare with current state
  3. If mismatch:
     - Calculate diff
     - Determine severity (critical/warning/info)
     - Add to repair queue
  4. Generate repair recommendations
  5. Apply auto-fixes (where safe)
  6. Report results
```

#### Auto-Repair Capabilities

**Safe Auto-Repairs (no confirmation needed):**
- Add missing configuration files
- Update outdated dependency versions
- Regenerate lost documentation
- Fix missing directory structure
- Update CI/CD workflow files

**Manual Confirmation Required:**
- Modify application logic
- Change database schema
- Update API contracts
- Alter security settings
- Remove deprecated code

---

## Implementation Details

### UI Components

#### Health Check Panel
- Real-time health score with color-coded status
- File structure validation grid
- Dependency status indicators
- Configuration health summary
- Recommendations list

#### Project Creation Wizard
- Step-by-step form with validation
- Configuration preview before creation
- Progress bar with step indicators
- Summary of included features
- Success confirmation with next steps

#### Drift Repair Panel
- Issue detection results
- Category-based issue grouping
- Before/after comparison
- Safe auto-repairs list
- Manual review items

### Backend Commands (Rust)

```rust
// Project initialization
#[tauri::command]
async fn create_tauri_project(project_name: String, template: String) -> Result<(), String>

#[tauri::command]
async fn setup_rust_backend(project_name: String, db_type: String) -> Result<(), String>

#[tauri::command]
async fn setup_frontend(project_name: String, css_framework: String) -> Result<(), String>

#[tauri::command]
async fn install_dependencies(project_name: String, package_manager: String) -> Result<(), String>

// Health checking
#[tauri::command]
async fn check_project_health() -> Result<HealthReport, String>

#[tauri::command]
async fn check_file_structure(file_path: String) -> Result<FileCheckResult, String>

#[tauri::command]
async fn validate_config_files() -> Result<Vec<ConfigValidation>, String>

// Drift detection & repair
#[tauri::command]
async fn detect_project_drift() -> Result<Vec<DriftIssue>, String>

#[tauri::command]
async fn auto_repair_files(issues: Vec<DriftIssue>) -> Result<RepairReport, String>
```

### Data Structures

```rust
#[derive(Serialize, Deserialize)]
pub struct HealthReport {
    pub score: u8,              // 0-100
    pub status: HealthStatus,   // excellent/good/warning/critical
    pub timestamp: DateTime,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub file_checks: Vec<FileCheck>,
}

#[derive(Serialize, Deserialize)]
pub struct DriftIssue {
    pub category: String,       // file/dependency/config/docs/quality
    pub severity: IssueSeverity, // critical/warning/info
    pub description: String,
    pub affected_file: String,
    pub fix_available: bool,
    pub auto_fixable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RepairReport {
    pub auto_fixed: Vec<String>,
    pub manual_review: Vec<String>,
    pub errors: Vec<String>,
    pub next_steps: Vec<String>,
}
```

---

## Testing Strategy

### Unit Tests
- File creation validation
- Configuration parsing
- Health score calculation
- Drift detection algorithms

### Integration Tests
- Full project creation flow
- Health check on created project
- Drift detection on modified project
- Auto-repair and verification

### Manual Testing
- Create project with each template
- Verify all files generated
- Check health score accuracy
- Introduce drift and verify detection
- Test auto-repair functionality

---

## Acceptance Criteria

### Phase 1: Project Initialization
- [ ] Create new Tauri project with Rust 2024 Edition
- [ ] Setup Svelte 5 + TypeScript frontend
- [ ] Initialize database (SQLite/PostgreSQL)
- [ ] Configure TailwindCSS if selected
- [ ] Setup OpenCode with custom agents
- [ ] Initialize Beads issue tracking
- [ ] Initialize OpenSpec workflow
- [ ] Generate compliance documentation
- [ ] Setup CI/CD pipelines
- [ ] Verify initial build succeeds
- [ ] All created files follow canonical structure

### Phase 2: Health Checking
- [ ] Scan project file structure
- [ ] Validate configuration files
- [ ] Check dependency status
- [ ] Calculate health score
- [ ] Generate recommendations
- [ ] Report displays clearly in UI
- [ ] Color-coded status indicators work
- [ ] File structure grid shows all critical files

### Phase 3: Drift Detection & Repair
- [ ] Detect missing files
- [ ] Detect configuration changes
- [ ] Detect outdated dependencies
- [ ] Categorize issues correctly
- [ ] Auto-fix safe issues
- [ ] Report manual review items
- [ ] Verify repairs work
- [ ] Document what was fixed

---

## Success Metrics

### User Experience
- New project setup < 5 minutes (vs 30+ minutes manual)
- Health score updates in < 2 seconds
- Drift detection completes in < 10 seconds
- Auto-repair success rate > 90%

### Quality Indicators
- 95%+ of generated projects pass health check (score > 80)
- Zero security vulnerabilities in generated projects
- All documentation templates present
- All tooling properly configured

### Adoption
- Used by 100% of new project creation
- Health checks run weekly on 80% of projects
- Auto-repair used on 60%+ of projects with drift

---

## Timeline & Milestones

### Sprint 1 (8 days)
- ✅ Spec approval
- [ ] Project initialization UI component
- [ ] Backend scaffolding commands
- [ ] Database template generation
- [ ] First pass testing

### Sprint 2 (6 days)
- [ ] Health check implementation
- [ ] UI for health report
- [ ] Configuration validation
- [ ] Documentation generation
- [ ] Integration testing

### Sprint 3 (6 days)
- [ ] Drift detection algorithm
- [ ] Auto-repair implementation
- [ ] UI for repair workflow
- [ ] Full integration testing
- [ ] Performance optimization

### Sprint 4 (4 days)
- [ ] Documentation
- [ ] Video tutorials
- [ ] Final QA and polish
- [ ] Release preparation

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Dependency conflicts | Medium | High | Test with multiple Node versions |
| Platform-specific issues | Medium | Medium | Test on Mac, Linux, Windows |
| Configuration drift over time | High | Medium | Regular health check automation |
| User misconfiguration | High | Low | Strong validation and guidance |
| Breaking changes in Tauri | Low | High | Pin versions, monitor releases |

---

## Dependencies & Prerequisites

### External Tools
- `create-tauri-app` CLI (latest)
- `cargo` (Rust 2024 Edition)
- `node` (18+) / `npm`/`yarn`/`pnpm`
- Git CLI

### Project Dependencies
- OpenSpec for workflow
- Beads for issue tracking
- OpenCode for CLI tooling
- Tauri 2.8.5 framework
- Svelte 5 framework

---

## Open Questions

1. **Database Migrations**
   - Should we auto-generate SQL files?
   - How to handle multiple DB types?
   - Version control for schema changes?

2. **Configuration Validation**
   - How strict should validation be?
   - Should we auto-fix some issues?
   - Custom validation rules?

3. **Dependency Management**
   - Auto-update minor versions?
   - Security vulnerability response?
   - Breaking change detection?

4. **Compliance Documentation**
   - Which compliance standards?
   - Industry-specific templates?
   - Customizable templates?

---

## Appendix: Technology Stack

- **Frontend:** Svelte 5 + TypeScript
- **Backend:** Rust 2024 Edition
- **Build:** Vite + Cargo
- **Styling:** TailwindCSS or Vanilla CSS
- **Database:** SQLite (built-in), PostgreSQL (optional), MongoDB (optional)
- **Package Managers:** npm, yarn, pnpm
- **CI/CD:** GitHub Actions
- **Tooling:** OpenCode, Beads, OpenSpec
- **Documentation:** Markdown

---

**Document Version:** 1.0  
**Last Updated:** October 27, 2025  
**Status:** READY FOR REVIEW
