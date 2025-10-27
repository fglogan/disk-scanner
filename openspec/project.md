# Project Context: Disk Bloat Scanner

## Purpose
The Disk Bloat Scanner is a cross-platform desktop application that helps users identify, analyze, and safely clean disk space wasters. It provides an intuitive interface for scanning directories, detecting duplicate files, identifying system junk, and managing cleanup operations.

**Target Users:**
- Developers managing large codebases with cache and build artifacts
- System administrators needing disk space management tools
- Power users concerned with storage optimization
- General users wanting to reclaim disk space safely

**Current Version:** v0.1.1  
**Target Release:** v0.2.0 (L4 Production Readiness)

## Tech Stack

### Frontend
- **Language**: TypeScript / JavaScript
- **Framework**: Svelte 4.x with SvelteKit
- **Styling**: TailwindCSS v3
- **Build Tool**: Vite 5.x
- **Package Manager**: npm (Node.js v20+)
- **State Management**: Svelte stores (`stores.js`)
- **Testing**: Vitest

### Backend
- **Language**: Rust 2024 Edition (v1.89.0)
- **Framework**: Tauri v2.8.5 (IPC layer)
- **Async Runtime**: Tokio
- **Key Dependencies**:
  - `serde` / `serde_json` - Serialization
  - `regex` - Pattern matching
  - `ignore` - Efficient directory traversal
  - `sha2` - Hash computation
  - `chrono` - Date/time handling
  - `log` - Structured logging

### Build & Deployment
- **Desktop Framework**: Tauri v2.8.5
- **Platforms**: macOS, Windows, Linux
- **CI/CD**: GitHub Actions (`.github/workflows/test.yml`)
- **Package Format**: Native installers (.dmg, .exe, .AppImage)

## Project Conventions

### Code Style

**Rust Backend:**
- Formatting: `cargo fmt` (automatic)
- Linting: `cargo clippy --all-targets --all-features -- -D warnings` (zero warnings required)
- Naming: snake_case for variables/functions, PascalCase for types
- Error handling: Custom `Result<T>` types, no unwrap() in library code
- Comments: Rustdoc (///), explain WHY not WHAT
- Imports: Group by: std, external crates, internal modules

**TypeScript/Svelte Frontend:**
- Formatting: Prettier (configured in `.prettierrc.json`)
- Linting: ESLint + TypeScript strict mode
- Naming: camelCase for variables/functions, PascalCase for components/types
- Imports: Absolute paths with `@/` alias
- Comments: JSDoc for public APIs
- File organization: Components in `/lib/components/`, utilities in `/lib/utils.js`

**General:**
- Line length: 100 characters for Rust, 120 for TypeScript
- Indentation: 2 spaces (Svelte), 4 spaces (Rust)
- Trailing comma: Yes in multi-line structures
- No TODO comments in production code (use bd issues instead)

### Architecture Patterns

**IPC Communication (Tauri):**
- All backend operations exposed as Tauri commands in `src-tauri/src/lib.rs`
- Frontend invokes via `invoke('command_name', { params })`
- Request-response model; all responses are `serde_json::Value`
- Error responses include error code and message

**Backend Module Structure:**
```
src-tauri/src/
├── lib.rs           # Main command handlers + IPC layer
├── main.rs          # Application entry point
├── error.rs         # Custom error types (Phase 2+)
├── models.rs        # Data structures (Phase 2+)
└── utils/
    ├── mod.rs
    ├── path.rs      # Path validation
    ├── scan.rs      # Scan logic (Phase 2+)
    ├── cleanup.rs   # Cleanup operations (Phase 2+)
    └── patterns.rs  # Detection patterns (Phase 2+)
```

**Frontend Architecture:**
- Svelte stores for shared state (scans, results, settings)
- Component-based UI (Dashboard, Sidebar, individual scan panels)
- Reactive bindings for auto-update on store changes
- Lifecycle management with `onMount` and `onDestroy`

**Safety First:**
- Path validation prevents scans of system directories (/, /System, /usr, /bin, etc.)
- Batch deletion limits: max 10,000 files per operation, max 100GB per batch
- Safe deletion uses system trash, not permanent removal
- Critical operation warnings before execution

**State Management:**
- Stores contain: disk info, scan results, UI state, settings
- Reactive to changes; components re-render on store updates
- No prop drilling; state is global via stores

### Testing Strategy

**Backend (Rust):**
- Unit tests in same file (cfg(test) modules)
- Integration tests in `src-tauri/tests/integration_tests.rs`
- Coverage target: >40% (Phase 2+)
- Test all error paths, not just happy path
- Use mock file systems where possible

**Frontend (TypeScript/Svelte):**
- Component tests with Vitest
- Store tests verify reactive behavior
- Integration tests for IPC calls
- Coverage target: >30% (Phase 2+)
- Manual E2E testing for UI/UX

**Requirements:**
- All tests must pass before PR merge
- `cargo test` and `npm test` run in CI
- Clippy warnings treated as errors in CI
- Coverage reports generated and tracked

### Git Workflow

**Branching:**
- Main branch: production-ready code (v0.1.1+)
- Feature branches: `feature/[description]`
- Bug fix branches: `bugfix/[description]`
- Refactor branches: `refactor/[description]`
- All branches: kebab-case, descriptive names

**Commits:**
- Format: `type: description` (per Conventional Commits)
- Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `perf`
- Include EDGS phase reference: `feat: add path validation (EDGS Phase 1: P1-T1)`
- Scope optional: `feat(backend): add cache` or `feat(frontend): add settings`
- Prefix breaking changes with `BREAKING:`

**Example Commits:**
```
feat(backend): add path validation utility (EDGS Phase 1: P1-T2 complete)
fix(ui): correct memory leak in Dashboard component (BEAD-012)
docs: update EDGS integration guide
test: add unit tests for duplicate detection
refactor: extract scan logic into separate module (Phase 2)
```

**Pull Requests:**
- Require approval before merge
- CI must pass (tests, linting, coverage)
- Squash commits if many small fixes
- Reference bd issues: "Fixes #disk-bloat-scanner-17"

## Domain Context

### Security Model
- **Threat Model**: Accidental system damage (user error protection)
- **Validation**: Path validation prevents scans of critical system directories
- **Permissions**: Respects file system permissions; graceful on permission denied
- **Data**: No telemetry, all operations local, no external services

### File System Operations
- **Scan Patterns**: Uses `ignore` crate for efficient traversal (respects .gitignore)
- **Duplicate Detection**: SHA256 hashing of file content (64-bit incremental)
- **Bloat Detection**: Size-based classification (configurable threshold)
- **Cleanup**: Uses system trash API (not permanent deletion)

### Performance Characteristics
- **Large Datasets**: Handles 1M+ files with memory constraints
- **Timeout**: 30-second limit per scan operation
- **Parallelism**: Tokio multi-threaded runtime for concurrent I/O
- **Progress**: Streamed updates every 100 files or 100ms

### User Safety Features
1. **Path Validation**: Whitelist of safe scan roots
2. **Batch Limits**: Max 10K files, 100GB per deletion operation
3. **Deletion Warnings**: User confirms before delete, shows size preview
4. **Trash Recovery**: Files can be recovered from trash until emptied
5. **Dry Run**: Preview cleanup results before executing

## Important Constraints

### Technical Constraints
- **Tauri v2.8.5**: Specific version for stability; breaking changes in v2.9+
- **Rust Edition**: 2024 edition only; no legacy code
- **Minimum Node**: v20 for frontend build
- **Cross-Platform**: Must support macOS, Windows, Linux without platform-specific code

### Business Constraints
- **Privacy First**: No telemetry, no external services
- **Safety**: Never delete without user confirmation
- **Cross-Platform**: Single codebase, native experience on all platforms
- **Maintainability**: Code must be understandable by single developer

### Regulatory / Compliance
- **TES-2025 v6.9**: Adhere to Tempext Engineering Standard
- **LAIO v3.3**: Logan AI Ontology principles (traceability, governance, PoE)
- **EDGS**: Event-Driven Gated Scheduling for project phases

### File Size Limits
- **Hash Limit**: Max 100MB files for duplicate detection (performance)
- **Scan Depth**: Max 6 levels deep (configurable)
- **Batch Size**: Max 10,000 files per deletion operation

## External Dependencies

### Package Management
- **npm**: Frontend dependency manager
- **Cargo**: Rust dependency manager
- **Cargo.lock**: Locked dependencies for reproducible builds

### Key External Crates
- **ignore**: Fast directory traversal with .gitignore support
- **serde/serde_json**: Serialization for IPC
- **tokio**: Async runtime
- **regex**: Pattern matching
- **sha2**: Cryptographic hashing

### System Dependencies
- **macOS**: FSEvents for file system monitoring (future)
- **Windows**: Windows API for safe file deletion
- **Linux**: inotify for file system monitoring (future)

### APIs & Services
- **None currently**: Fully local application
- **Future**: System trash API integration (platform-specific)

## EDGS & Project Lifecycle

### Current Phase
- **Phase**: 0 (Constitutional Foundation)
- **Maturity**: L2 (Integration)
- **Target**: L4 (Production Readiness) at v0.2.0
- **Tracking**: bd issue tracker (`.beads/disk-bloat-scanner.db`)

### Phase Gate Structure
Each phase requires:
1. All tasks completed
2. Gate validation (criteria met)
3. **HIL approval** (Human-in-the-Loop)
4. PoE bundle (immutable evidence)

### Approval Authorities
- **P0 Gate**: Project Sponsor
- **P1 Gate**: Tech Lead
- **P2 Gate**: Architect
- **P3 Gate**: QA Lead
- **P4 Gate**: Documentation Owner
- **P5 Gate**: QA Manager
- **P6 Gate**: Release Manager
- **Release Gate**: Executive Sponsor

## Key Contacts & Resources

- **Project Repository**: `/Volumes/Tempext-Projects/Users/tempext/Projects/disk-bloat-scanner`
- **Issue Tracker**: `bd` (`.beads/disk-bloat-scanner.db`)
- **Documentation**: `/docs/` directory
- **Design Specs**: `/docs/design/DESIGN_SPEC.md`
- **Architecture**: `/docs/ARCHITECTURE.md`
- **EDGS Schedule**: `/docs/schedules/EDGS_SCHEDULE.md`

## Quick Links

| Resource | Location |
|----------|----------|
| EDGS Integration Guide | `docs/EDGS_INTEGRATION.md` |
| Crash Recovery Context | `AGENTS.md` (root) |
| OpenSpec Workflow | `openspec/AGENTS.md` |
| Engineering Standard | `docs/compliance/TES-2025-v6.9.md` |
| Architecture Decisions | `docs/design/TAURI_ARCHITECTURE_MODERNIZATION.md` |
| Safety Guidelines | `docs/design/SAFETY_FIRST_ARCHITECTURE.md` |
