# Architecture - Disk Bloat Scanner

## Overview

Disk Bloat Scanner is a Tempext Genesis showcase project demonstrating best practices for modern desktop app development using Tauri, Rust, Svelte, and Tailwind CSS.

## Technology Stack

### Frontend
- **Svelte 5.39.6**: Reactive UI framework with fine-grained reactivity
- **Tailwind CSS 4.1.16**: Utility-first CSS with `@theme` customization
- **Vite 7.1.12**: Fast build tool and dev server
- **Vitest**: Unit testing framework

### Backend
- **Rust 1.89.0**: Systems programming language
- **Tauri 2.8.5+**: Cross-platform desktop app framework
- **tokio**: Async runtime
- **rayon**: Data parallelism
- **sysinfo**: System information
- **sha2**: Cryptographic hashing
- **trash**: Safe file deletion

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        User Interface                        │
│                      (Svelte Components)                     │
├─────────────────────────────────────────────────────────────┤
│  Dashboard  │ LargeFiles │ ProjectBloat │ Duplicates │ Settings │
└──────┬──────┴──────┬─────┴──────┬───────┴─────┬──────┴────┬─────┘
       │             │            │             │           │
       └─────────────┴────────────┴─────────────┴───────────┘
                             │
                    ┌────────▼────────┐
                    │  Svelte Stores  │  (State Management)
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │   Tauri IPC     │  (invoke commands)
                    └────────┬────────┘
                             │
       ┌─────────────────────┴─────────────────────┐
       │              Rust Backend                 │
       │           (Tauri Commands)                │
       ├───────────────────────────────────────────┤
       │  • get_system_info()                      │
       │  • get_disk_info()                        │
       │  • scan_bloat(opts)                       │
       │  • scan_large_files(opts)                 │
       │  • scan_duplicates(opts)                  │
       │  • cleanup_dirs(req)                      │
       └─────────────────┬─────────────────────────┘
                         │
         ┌───────────────┴───────────────┐
         │       File System Layer       │
         ├───────────────────────────────┤
         │  • walkdir    (traversal)     │
         │  • rayon      (parallel)      │
         │  • sha2       (hashing)       │
         │  • trash      (safe delete)   │
         │  • sysinfo    (system stats)  │
         └───────────────────────────────┘
```

## Component Architecture

### Frontend Components

#### 1. Dashboard.svelte
- **Purpose**: Main landing page with system overview
- **Features**:
  - System info display (OS, hostname, CPU, memory)
  - Disk usage visualization
  - Scan control with progress feedback
  - Summary statistics from last scan
- **Updates**: System stats refresh every 3 seconds

#### 2. LargeFiles.svelte
- **Purpose**: Browse and manage files over size threshold
- **Features**:
  - File list with size sorting
  - Safety indicators (safe/check/danger)
  - Multi-select with checkboxes
  - Native delete confirmation
- **Safety Levels**:
  - 🟢 Safe: caches, build outputs, temp files
  - 🟡 Check: media, archives, downloads
  - 🔴 Danger: source code, documents, databases

#### 3. ProjectBloat.svelte
- **Purpose**: Detect build artifacts and development caches
- **Features**:
  - Categorized bloat display
  - Per-project size breakdown
  - Pattern-based detection
- **Patterns**: node_modules, target, venv, .git, etc.

#### 4. Duplicates.svelte
- **Purpose**: Find and remove duplicate files
- **Features**: (UI incomplete)
  - SHA256-based duplicate detection
  - Grouped by hash
  - Savable space calculation

#### 5. Settings.svelte
- **Purpose**: Configure scanning behavior
- **Features**:
  - Directory management (add/remove)
  - Minimum file sizes
  - Background scanning toggle
  - Ignore patterns (planned)

#### 6. Sidebar.svelte
- **Purpose**: Navigation
- **Features**:
  - Route switching
  - Active page highlighting

### State Management (stores.js)

```javascript
// Global application state
diskInfo          // Disk usage stats
summaryStats      // Scan result summary
bloatCategories   // Found bloat by category
largeFiles        // Files over threshold
duplicates        // Duplicate file sets
isScanning        // Scanning in progress
scanProgress      // Current scan status
selectedPaths     // Files selected for deletion
settings          // User preferences
```

### Backend Commands

#### get_system_info()
```rust
Returns: SystemInfoResponse {
  disk_total_gb, disk_used_gb, disk_free_gb, disk_usage_pct,
  memory_total_gb, memory_used_gb, memory_free_gb, memory_usage_pct,
  cpu_count, os_name, os_version, hostname
}
```

#### scan_bloat(opts: ScanOpts)
```rust
Input: { root, min_bytes, follow_symlinks }
Returns: Vec<BloatCategory> {
  category_id, display_name, total_size_mb,
  entries: Vec<BloatEntry { path, size_mb }>
}
```
- Walks directory tree
- Matches against bloat patterns
- Calculates directory sizes
- Groups by category

#### scan_large_files(opts: ScanOpts)
```rust
Input: { root, min_bytes, follow_symlinks }
Returns: Vec<LargeFileEntry> {
  path, size_mb, last_modified
}
```
- Parallel file traversal (rayon)
- Filters by size threshold
- Sorts by size (largest first)

#### scan_duplicates(opts: ScanOpts)
```rust
Input: { root, min_bytes, follow_symlinks }
Returns: Vec<DuplicateSet> {
  hash, total_savable_mb,
  entries: Vec<DuplicateEntry { path, size_mb, last_modified }>
}
```
- Groups files by size (optimization)
- SHA256 hash of file contents
- Groups by hash
- Calculates savable space

#### cleanup_dirs(req: CleanupReq)
```rust
Input: { paths: Vec<String>, dry_run, trash }
Returns: CleanupResult {
  deleted, skipped, errors
}
```
- Moves files to system trash
- Supports dry-run mode
- Batch operations

## Data Flow

### Scan Workflow

1. User clicks "Scan Manually Now"
2. Frontend sets `isScanning = true`
3. Sequential scan execution:
   ```
   get_system_info() → update UI
   scan_bloat()      → update UI
   scan_large_files() → update UI
   scan_duplicates()  → update UI
   ```
4. Each scan updates `scanProgress` with status
5. Results stored in respective stores
6. Summary stats calculated
7. `isScanning = false`

### Delete Workflow

1. User selects files (checkboxes)
2. Click "Delete Selected"
3. Native confirmation dialog shows:
   - File count
   - Total size
   - Trash recovery info
4. If confirmed:
   - Call `cleanup_dirs({ paths, trash: true })`
   - Files moved to system trash
   - UI refreshed

## Performance Optimizations

### Backend

1. **Parallel Scanning**
   - Rayon for multi-threaded file traversal
   - Parallel hashing of potential duplicates

2. **Smart Duplicate Detection**
   - Group by size first (only hash same-size files)
   - Skip files > 100MB
   - Skip files < 1KB

3. **Lazy Evaluation**
   - Only calculate directory sizes when matched
   - Stream results instead of buffering all

### Frontend

1. **Async Stats Updates**
   - System info refreshes every 3 seconds
   - Only when not actively scanning

2. **Progressive Rendering**
   - Sequential scans with UI updates
   - User sees results as they complete

3. **Reactive Updates**
   - Svelte fine-grained reactivity
   - Minimal DOM updates

## Error Handling

### Backend
- Commands return `Result<T, String>`
- Errors propagated to frontend as strings
- File system errors caught and logged

### Frontend
- Try/catch around invoke calls
- User-friendly error messages
- Console logging for debugging

## Security Considerations

1. **Path Validation**
   - User-provided paths checked
   - No directory traversal attacks

2. **Safe Deletion**
   - Files moved to trash, not permanently deleted
   - Dry-run mode available

3. **No Network Access**
   - All operations local
   - No telemetry/analytics

4. **Permissions**
   - Tauri capabilities restrict access
   - Dialog plugin for file picker

## Testing Strategy

### Unit Tests
- Individual function testing
- Data structure validation
- Serialization checks

### Integration Tests
- Full scan workflows
- File system operations
- Temporary test directories

### Component Tests
- Store behavior
- State management
- Data flow

## Future Enhancements

### Planned Features
- Streaming scan results (Tauri events)
- Background scheduled scans
- Custom bloat patterns
- Export to CSV/JSON
- Network drive support

### Architecture Changes
- Event-based progress updates
- Plugin system for custom scanners
- Database for scan history
- Multi-directory parallel scanning

## Build & Deployment

### Development
```bash
npm run tauri:dev
```

### Production
```bash
npm run tauri:build
```

### Outputs
- macOS: `.app` bundle and DMG
- Windows: MSI installer (untested)
- Linux: DEB/AppImage (untested)

## Dependencies

### Critical Dependencies
- **Tauri**: Desktop framework
- **Svelte**: UI framework
- **Tailwind**: Styling
- **Vite**: Build tool

### Backend Libraries
- **sysinfo**: System stats
- **walkdir**: Directory traversal
- **rayon**: Parallelism
- **sha2**: Hashing
- **trash**: Safe deletion

### Dev Dependencies
- **vitest**: Frontend testing
- **cargo test**: Rust testing
- **clippy**: Rust linting
- **prettier**: Code formatting

## License

Apache 2.0 - See LICENSE file

---

**Version**: 0.1.0-alpha  
**Last Updated**: October 24, 2025  
**Maintained by**: Disk Bloat Scanner Contributors
