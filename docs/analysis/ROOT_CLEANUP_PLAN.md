# ROOT DIRECTORY CLEANUP & REORGANIZATION PLAN

**Standard:** TES-2025 v6.9 Section 2.2 (Canonical Directory Structure)  
**Date:** October 26, 2025  
**Status:** READY FOR EXECUTION

---

## Current State (NON-COMPLIANT)

### Root-Level Files (36 files - EXCESSIVE)

#### Source Code Files (Should NOT be at root)
```
❌ accurate_scanner.rs          (Dead code - backend)
❌ svelte.config.js             (Config - should be referenced, not at root)
❌ tailwind.config.js           (Config - should be referenced, not at root)
❌ test_junk_detection.rs       (Dead code - old test)
❌ test_new_scanners.rs         (Dead code - old test)
❌ vite.config.js               (Config - should be referenced, not at root)
❌ vite.config.ts               (Config - should be referenced, not at root)
❌ vitest.config.js             (Config - should be referenced, not at root)
```

**Note:** Config files (vite, vitest, tailwind, svelte) CAN stay at root per standard convention, but should be cleaned up if duplicates exist.

#### Documentation Files (27 .md files - EXCESSIVE)

**KEEP at root:**
```
✅ README.md                    (GitHub repo, keep at root)
✅ AGENTS.md                    (Per user request - preserve)
✅ CLAUDE.md                    (Per user request - preserve)
✅ RESTART.md                   (Per user request - preserve)
```

**MOVE to `/docs/`:**
```
→ ANALYSIS.md                   → /docs/ANALYSIS.md (or archive)
→ ANALYSIS_INDEX.md             → /docs/ANALYSIS_INDEX.md (or archive)
→ ARCHITECTURE.md               → /docs/ARCHITECTURE.md
→ CONTRIBUTING.md               → /docs/CONTRIBUTING.md
→ DELIVERABLES.md               → /docs/DELIVERABLES.md (or archive)
→ EDGS_SCHEDULE.md              → /docs/schedules/EDGS_SCHEDULE.md
→ QUICK_START.md                → /docs/QUICK_START.md
→ RECOMMENDATIONS.md            → /docs/RECOMMENDATIONS.md (or archive)
→ TESTING.md                    → /docs/TESTING.md
→ TES-2025_COMPLIANCE.md        → /docs/compliance/TES-2025_COMPLIANCE.md
```

**ARCHIVE (outdated/temporary):**
```
→ .archive/BD_IMPLEMENTATION_SUMMARY.md
→ .archive/BRANDING_UPDATE.md
→ .archive/FINAL_WORKING_STATE.md
→ .archive/FIXED_AND_WORKING.md
→ .archive/GIT_COMMIT_STATUS.md
→ .archive/IMPLEMENTATION_COMPLETE.md
→ .archive/PROJECT_STATUS.md          (Analysis artifact)
→ .archive/READY_FOR_RELEASE.md
→ .archive/RELEASE_NOTES_v0.1.1.md   → /docs/history/RELEASE_NOTES_v0.1.1.md
→ .archive/RELEASE_NOTES.md          → /docs/history/RELEASE_NOTES.md
→ .archive/SCREENSHOT_INSTRUCTIONS.md
→ .archive/SHOWCASE_SUMMARY.md
→ .archive/STARTUP_FIXED.md
→ .archive/SUCCESS.md
→ .archive/WBS.md
```

---

## TES-2025 v6.9 Compliant Structure

### Canonical Directory Layout (Section 2.2)

```
project_root/
├── README.md                          ✅ KEEP (GitHub)
├── LICENSE                            ✅ KEEP
├── AGENTS.md                          ✅ KEEP (user request)
├── CLAUDE.md                          ✅ KEEP (user request)
├── RESTART.md                         ✅ KEEP (user request)
│
├── package.json                       ✅ KEEP
├── package-lock.json                  ✅ KEEP
├── Cargo.toml                         ✅ KEEP
├── tsconfig.json                      ✅ KEEP
├── tsconfig.app.json                  ✅ KEEP
├── tsconfig.node.json                 ✅ KEEP
│
├── vite.config.ts                     ✅ KEEP (config)
├── vitest.config.js                   ✅ KEEP (config)
├── tailwind.config.js                 ✅ KEEP (config)
├── svelte.config.js                   ✅ KEEP (config)
├── .prettierrc.json                   ✅ KEEP (config)
│
├── .laio/                             ⚠️ CREATE (Phase 0)
│   ├── constitution.yaml              (Project metadata)
│   └── ...
│
├── .github/                           ✅ EXISTS (CI/CD)
│   └── workflows/
│
├── policies/                          ⚠️ CREATE (Phase 1)
│   ├── cargo-deny.toml
│   └── ...
│
├── schemas/                           ⚠️ CREATE (Phase 1+)
│   ├── ipc-commands.json
│   └── ...
│
├── src/                               ✅ EXISTS (Frontend)
│   ├── lib/
│   │   ├── components/
│   │   ├── stores.ts                  (migrate from .js in Phase 3)
│   │   └── utils.js
│   ├── App.svelte
│   ├── index.html
│   ├── main.js
│   └── app.css
│
├── src-tauri/                         ✅ EXISTS (Backend)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── error.rs                   (create in Phase 1)
│   │   ├── models.rs                  (create in Phase 2)
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── path.rs
│   │   │   ├── patterns.rs            (create in Phase 2)
│   │   │   ├── scan.rs                (create in Phase 2)
│   │   │   └── cleanup.rs             (create in Phase 2)
│   │   └── commands/                  (optional, Phase 2+)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   └── tests/
│
├── public/                            ✅ EXISTS (Static assets)
│   └── vite.svg
│
├── docs/                              ✅ EXISTS (partially)
│   ├── ARCHITECTURE.md                (move from root)
│   ├── CONTRIBUTING.md                (move from root)
│   ├── TESTING.md                     (move from root)
│   ├── DEVELOPMENT.md                 (create Phase 3)
│   ├── API.md                         (create Phase 3)
│   ├── TROUBLESHOOTING.md             (create Phase 3)
│   ├── QUICK_START.md                 (move from root)
│   │
│   ├── design/                        ✅ EXISTS
│   │   ├── DESIGN_SPEC.md
│   │   ├── JUNK_FILE_DETECTION_STRATEGY.md
│   │   └── V2_FEATURE_ROADMAP.md
│   │
│   ├── schedule/                      ✅ EXISTS (partially)
│   │   └── EDGS_SCHEDULE.md           (move from root)
│   │
│   ├── history/                       ⚠️ CREATE
│   │   ├── CHANGELOG.md               (create)
│   │   ├── RELEASE_NOTES.md           (archive→move)
│   │   └── RELEASE_NOTES_v0.1.1.md   (archive→move)
│   │
│   ├── compliance/                    ⚠️ CREATE
│   │   ├── TES-2025_COMPLIANCE.md    (move from root)
│   │   ├── SECURITY_AUDIT.md         (create Phase 1)
│   │   └── CODE_QUALITY_REPORT.md    (create Phase 5)
│   │
│   └── audit/                         ⚠️ CREATE
│       └── (PoE bundles will go here)
│
├── scripts/                           ✅ EXISTS
│   └── dynamic-vite.js
│
├── .archive/                          ⚠️ CREATE (cleanup)
│   ├── BD_IMPLEMENTATION_SUMMARY.md
│   ├── BRANDING_UPDATE.md
│   ├── FINAL_WORKING_STATE.md
│   ├── FIXED_AND_WORKING.md
│   ├── GIT_COMMIT_STATUS.md
│   ├── IMPLEMENTATION_COMPLETE.md
│   ├── PROJECT_STATUS.md
│   ├── READY_FOR_RELEASE.md
│   ├── SCREENSHOT_INSTRUCTIONS.md
│   ├── SHOWCASE_SUMMARY.md
│   ├── STARTUP_FIXED.md
│   ├── SUCCESS.md
│   └── WBS.md
│
└── .gitignore                         ✅ KEEP
```

---

## Cleanup Action Items

### Phase 1: Critical Cleanup (Do Immediately)

| Item | Action | Reason |
|------|--------|--------|
| `accurate_scanner.rs` | Delete or archive | Dead code, not used |
| `test_junk_detection.rs` | Delete or archive | Dead code, old test file |
| `test_new_scanners.rs` | Delete or archive | Dead code, old test file |
| `test~` (if exists) | Delete | Temporary backup file |
| `test.pyc` (if exists) | Delete | Compiled Python, obsolete |

### Phase 2: Documentation Reorganization

| Item | Action | Target |
|------|--------|--------|
| `ANALYSIS.md` | Archive or move | `.archive/` or `/docs/` |
| `ANALYSIS_INDEX.md` | Archive or move | `.archive/` or `/docs/` |
| `ARCHITECTURE.md` | Move | `/docs/ARCHITECTURE.md` |
| `CONTRIBUTING.md` | Move | `/docs/CONTRIBUTING.md` |
| `DELIVERABLES.md` | Archive | `.archive/DELIVERABLES.md` |
| `EDGS_SCHEDULE.md` | Move | `/docs/schedules/EDGS_SCHEDULE.md` |
| `QUICK_START.md` | Move | `/docs/QUICK_START.md` |
| `RECOMMENDATIONS.md` | Archive | `.archive/RECOMMENDATIONS.md` |
| `RELEASE_NOTES.md` | Move | `/docs/history/RELEASE_NOTES.md` |
| `RELEASE_NOTES_v0.1.1.md` | Move | `/docs/history/RELEASE_NOTES_v0.1.1.md` |
| `TESTING.md` | Move | `/docs/TESTING.md` |
| `TES-2025_COMPLIANCE.md` | Move | `/docs/compliance/TES-2025_COMPLIANCE.md` |

### Phase 3: Archive Old Files

Create `.archive/` directory and move:
```
.archive/
├── BD_IMPLEMENTATION_SUMMARY.md
├── BRANDING_UPDATE.md
├── FINAL_WORKING_STATE.md
├── FIXED_AND_WORKING.md
├── GIT_COMMIT_STATUS.md
├── IMPLEMENTATION_COMPLETE.md
├── PROJECT_STATUS.md
├── READY_FOR_RELEASE.md
├── SCREENSHOT_INSTRUCTIONS.md
├── SHOWCASE_SUMMARY.md
├── STARTUP_FIXED.md
├── SUCCESS.md
└── WBS.md
```

### Phase 4: Create Missing Directories

```bash
mkdir -p .laio
mkdir -p policies
mkdir -p schemas
mkdir -p docs/history
mkdir -p docs/compliance
mkdir -p docs/audit
```

### Phase 5: Update .gitignore

```bash
# Add these lines to .gitignore
.laio/constitution.yaml    # Allow, but ignore local changes
.archive/                  # Ignore archived files
audit/**/*.cdx.zip         # Ignore PoE bundles (large)
```

---

## Final Root Directory Contents (Compliant)

After cleanup, root should contain ONLY:

```
✅ README.md               (GitHub repo)
✅ LICENSE                 (Apache 2.0)
✅ AGENTS.md               (User request - preserved)
✅ CLAUDE.md               (User request - preserved)
✅ RESTART.md              (User request - preserved)
✅ .gitignore
✅ package.json
✅ package-lock.json
✅ Cargo.toml
✅ tsconfig.json
✅ tsconfig.app.json
✅ tsconfig.node.json
✅ vite.config.ts
✅ vitest.config.js
✅ tailwind.config.js
✅ svelte.config.js
✅ .prettierrc.json

📁 Directories ONLY (no files at root except above):
✅ src/
✅ src-tauri/
✅ public/
✅ docs/
✅ scripts/
✅ .github/
✅ .laio/         (create)
✅ policies/      (create)
✅ schemas/       (create)
✅ .archive/      (create - reference only, not in git)
```

**Total root-level files:** 5-17 files (config files + preserved md files)  
**Currently:** 36 files  
**Compliance:** From 13% → 100%

---

## Implementation Steps (Bash Commands)

### Step 1: Create Archive Directory
```bash
mkdir -p .archive
```

### Step 2: Move Dead Code Files
```bash
mv accurate_scanner.rs .archive/
mv test_junk_detection.rs .archive/
mv test_new_scanners.rs .archive/
```

### Step 3: Create New Directories
```bash
mkdir -p .laio
mkdir -p policies
mkdir -p schemas
mkdir -p docs/history
mkdir -p docs/compliance
mkdir -p docs/audit
```

### Step 4: Move Documentation Files
```bash
# Move to /docs
mv ARCHITECTURE.md docs/
mv CONTRIBUTING.md docs/
mv TESTING.md docs/
mv QUICK_START.md docs/

# Move to /docs/schedules
mv EDGS_SCHEDULE.md docs/schedules/

# Move to /docs/history
mv RELEASE_NOTES.md docs/history/
mv RELEASE_NOTES_v0.1.1.md docs/history/

# Move to /docs/compliance
mv TES-2025_COMPLIANCE.md docs/compliance/

# Archive (temporary/working files)
mv ANALYSIS.md .archive/
mv ANALYSIS_INDEX.md .archive/
mv DELIVERABLES.md .archive/
mv RECOMMENDATIONS.md .archive/
mv PROJECT_STATUS.md .archive/
mv BD_IMPLEMENTATION_SUMMARY.md .archive/
mv BRANDING_UPDATE.md .archive/
mv FINAL_WORKING_STATE.md .archive/
mv FIXED_AND_WORKING.md .archive/
mv GIT_COMMIT_STATUS.md .archive/
mv IMPLEMENTATION_COMPLETE.md .archive/
mv READY_FOR_RELEASE.md .archive/
mv SCREENSHOT_INSTRUCTIONS.md .archive/
mv SHOWCASE_SUMMARY.md .archive/
mv STARTUP_FIXED.md .archive/
mv SUCCESS.md .archive/
mv WBS.md .archive/
```

### Step 5: Update .gitignore
```bash
cat >> .gitignore << 'EOF'

# Archive directory (reference only, not tracked)
.archive/

# LAIO constitution (may contain local overrides)
.laio/constitution.yaml

# PoE bundles (large, stored separately)
audit/**/*.cdx.zip
EOF
```

### Step 6: Verify Root is Clean
```bash
# Should show ONLY essential files
ls -la | grep -E "\.(md|js|ts|rs)$"

# Should show ONLY these:
# - AGENTS.md
# - CLAUDE.md
# - RESTART.md
# - README.md
# - Config files (vite.config.ts, vitest.config.js, etc.)
```

---

## Git Integration

### Commit Organization Cleanup
```bash
git add -A
git commit -m "chore: organize root directory per TES-2025 v6.9 (Phase 0)

- Move documentation to /docs subdirectories
- Archive temporary/working files to .archive/
- Move dead code to .archive/
- Create .laio/, policies/, schemas/ directories
- Update .gitignore for new structure

Per TES-2025 v6.9 Section 2.2 (Canonical Directory Structure)"
```

### Note on Archive Directory
- `.archive/` is for reference and history
- Add to `.gitignore` to keep repo lean
- Can be backed up separately if needed
- Documents remain accessible but don't clutter root

---

## Verification Checklist

After cleanup, verify:

- [ ] Root contains ≤17 files (5 preserved .md + config files)
- [ ] No `.rs` files at root
- [ ] No `.js` files at root (except config)
- [ ] No `.ts` files at root (except config)
- [ ] Only `.md` files at root are: README.md, AGENTS.md, CLAUDE.md, RESTART.md
- [ ] `/docs/` contains all project documentation
- [ ] `/docs/schedules/` contains EDGS_SCHEDULE.md
- [ ] `/docs/compliance/` contains TES-2025_COMPLIANCE.md
- [ ] `/docs/history/` contains release notes
- [ ] `.archive/` contains old/temporary files
- [ ] `.laio/` directory exists (empty until Phase 0)
- [ ] `policies/` directory exists (empty until Phase 1)
- [ ] `schemas/` directory exists (empty until Phase 1+)
- [ ] `.gitignore` updated with `.archive/` and PoE bundle entries
- [ ] `git status` shows clean working tree or only tracked changes

---

## Timeline Integration

This cleanup aligns with EDGS schedule:

| Phase | Timing | Cleanup Task |
|-------|--------|--------------|
| **Phase 0** | Pre-Phase 1 | Execute this ROOT_CLEANUP_PLAN |
| **Phase 1** | Gate 1 | Verify root is clean in PoE bundle |
| **Phase 4** | Gate 4 | Move documentation (final organization) |
| **Phase 6** | Gate 6 | Final verification in PoE bundle |

---

## Success Metrics

| Metric | Before | After | Compliant |
|--------|--------|-------|-----------|
| Root .md files | 27 | 4 | ✅ |
| Root .rs files | 3 | 0 | ✅ |
| Root .js files | 4 | 0 | ✅ |
| Root .ts files | 1 | 1 (config) | ✅ |
| Total root files | 36 | 17 | ✅ |
| Canonical compliance | 13% | 100% | ✅ |

---

## References

- TES-2025 v6.9 Section 2.2 (Canonical Directory Structure)
- EDGS_SCHEDULE.md (Phase 0 tasks)
- .gitignore (repository rules)

---

**Status:** READY FOR EXECUTION (Phase 0)

Execute as part of Phase 0 Pre-Gate Setup tasks.

