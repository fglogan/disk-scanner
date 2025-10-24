# Disk Bloat Scanner

A cross-platform desktop application for identifying and managing disk space usage by detecting build artifacts, duplicate files, and large files on your system.

## ⚠️ Development Status

**This software is actively in development and not fully tested.** Use with caution. While the application moves files to the system trash (not permanent deletion), there is no guarantee of data safety. Always verify what you're deleting and maintain backups of important data.

**Current Version:** 0.1.0-alpha  
**Status:** Alpha - Core features implemented, extensive testing in progress

### Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| **macOS** | ✅ Tested | Primary development platform (macOS 14+) |
| **Windows** | ⚠️ Not Tested | Built but completely untested - use at your own risk |
| **Linux** | ⚠️ Not Tested | Built but completely untested - use at your own risk |

**Windows and Linux users:** This application has been developed and tested exclusively on macOS. While it should theoretically work on Windows and Linux due to Tauri's cross-platform nature, it has not been tested on these platforms. File operations, trash functionality, and path handling may behave unexpectedly.

### Known Issues
- Delete confirmation dialog may not work correctly in some scenarios
- Duplicate file scanning limited to files under 100MB
- Some UI components still under development
- Performance not optimized for very large directory trees (>1M files)

## Features

### Implemented
- **Disk Information Display**: View total, used, and available disk space
- **Bloat Detection**: Scan directories for common build artifacts and caches
  - Node.js (`node_modules`)
  - Rust (`target/`)
  - Python (`venv/`, `__pycache__/`)
  - Git (`.git/`)
  - iOS/Android build outputs
- **Large File Scanner**: Identify files over a configurable size threshold
- **Duplicate File Detection**: Find duplicate files using SHA256 hashing
- **Progressive Scan Updates**: Real-time feedback during scan operations
- **Safety Indicators**: Color-coded warnings (green/amber/red) for file deletion decisions
- **Trash Integration**: Files moved to system trash, not permanently deleted

### Planned
- Scheduled automated scans
- Advanced filtering and search
- Export scan results to CSV/JSON
- Custom bloat pattern definitions
- Improved multi-directory scanning

## Technology Stack

- **Backend**: Rust 1.89.0, Tauri 2.8.5
- **Frontend**: Svelte 5.39.6, Tailwind CSS 4.1.16
- **Build Tool**: Vite 7.1.12
- **Architecture**: Desktop application with IPC communication between Rust backend and web-based UI

## Prerequisites

- **Rust**: 1.75.0 or later
- **Node.js**: 18.0.0 or later
- **npm**: 9.0.0 or later
- **Platform**: macOS, Windows, or Linux

## Installation

### Development Build

1. Clone the repository:
```bash
git clone <repository-url>
cd disk-bloat-scanner
```

2. Install dependencies:
```bash
npm install
```

3. Run the application in development mode:
```bash
npm run tauri:dev
```

### Production Build

**macOS:**
```bash
npm run tauri:build
```

The built application will be available at:
- **macOS DMG**: `src-tauri/target/release/bundle/dmg/Disk Bloat Scanner_0.1.0_x64.dmg`
- **macOS App**: `src-tauri/target/release/bundle/macos/Disk Bloat Scanner.app`

**Windows (untested):**
```bash
npm run tauri:build
```
Output: `src-tauri/target/release/bundle/msi/` or `src-tauri/target/release/bundle/nsis/`

**Linux (untested):**
```bash
npm run tauri:build
```
Output: `src-tauri/target/release/bundle/deb/` or `src-tauri/target/release/bundle/appimage/`

**Note:** Production builds are code-signed on macOS. Windows and Linux builds are not signed and will show security warnings.

## Usage

### Basic Workflow

1. **Launch the application**
2. **Select a scan type** from the sidebar:
   - Dashboard: Overview and quick actions
   - Large Files: Find files over 100MB (configurable)
   - Project Bloat: Detect build artifacts and caches
   - Duplicates: Find duplicate files
3. **Review results** with safety indicators:
   - 🟢 Green: Generally safe to delete (caches, build outputs)
   - 🟡 Amber: Review carefully (media files, archives)
   - 🔴 Red: Danger - source code, documents, databases
4. **Select files** to remove using checkboxes
5. **Delete selected** items (moved to trash, not permanent)

### Safety Guidelines

- Always review what you're about to delete
- Pay attention to safety indicators
- Start with small, non-critical directories
- Keep backups of important data
- Test the trash/restore functionality on your system
- Do not run scans as root/administrator unless necessary

### Configuration

Settings are accessible from the Settings page in the sidebar. You can configure:
- Minimum file size for large file detection
- Scan depth limits
- Symlink following behavior
- Excluded directories (planned)

## Project Structure

```
disk-bloat-scanner/
├── src/                    # Frontend source (Svelte)
│   ├── lib/
│   │   ├── components/    # UI components
│   │   └── stores.js      # State management
│   ├── App.svelte         # Main app component
│   └── main.js            # Entry point
├── src-tauri/             # Backend source (Rust)
│   ├── src/
│   │   ├── lib.rs         # Core scanning logic
│   │   └── main.rs        # Tauri app initialization
│   └── Cargo.toml         # Rust dependencies
├── docs/                  # Documentation
│   ├── compliance/        # Standards and compliance
│   ├── design/            # Design specifications
│   └── schedule/          # Development schedule
└── public/                # Static assets
```

## Development

### Code Quality

The project uses automated formatting and linting:

```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Format frontend code
npx prettier --write "src/**/*.{js,svelte,css,html}"
```

### Testing

```bash
# Run Rust tests
cargo test

# Run frontend tests (when implemented)
npm test
```

### Adding New Scan Patterns

Edit `src-tauri/src/lib.rs` and add patterns to the `BLOAT_PATTERNS` array:

```rust
BloatPattern {
    category_id: "your_category",
    display_name: "Your Category",
    dir_names: &["dir1", "dir2"],
}
```

## Architecture

The application follows a clean separation between frontend and backend:

- **Frontend (Svelte)**: Handles UI rendering, user interactions, and state management
- **Backend (Rust/Tauri)**: Performs file system operations, hashing, and scanning
- **IPC Layer**: Tauri commands provide type-safe communication between layers

### Key Backend Commands

- `get_disk_info()`: Retrieve disk usage statistics
- `scan_bloat(opts)`: Scan for build artifacts and caches
- `scan_large_files(opts)`: Find files over size threshold
- `scan_duplicates(opts)`: Detect duplicate files via SHA256
- `cleanup_dirs(req)`: Move files to trash

## Performance Considerations

- **Large File Hashing Limit**: Files over 100MB are excluded from duplicate detection
- **Parallel Scanning**: Uses Rayon for multi-threaded directory traversal
- **Memory Usage**: Scans are streamed; entire directory trees are not loaded into memory
- **Incremental Updates**: UI updates progressively as scans complete

## Security and Privacy

- All file operations are performed locally
- No data is transmitted over the network
- No telemetry or analytics collection
- Files are moved to system trash, allowing recovery
- Path validation prevents scanning of protected system directories

## Contributing

This is an active development project. Contributions are welcome but please be aware:

- Code is subject to frequent changes
- No formal contribution guidelines established yet
- Test thoroughly on your platform before submitting PRs
- Follow existing code style (use `cargo fmt` and `prettier`)

## License

```
Copyright 2025 [Your Name/Organization]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

See [LICENSE](LICENSE) file for full license text.

## Disclaimer

THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. THE AUTHORS OR COPYRIGHT HOLDERS SHALL NOT BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER LIABILITY ARISING FROM THE USE OF THIS SOFTWARE.

**Use at your own risk. Always maintain backups of important data.**

## Acknowledgments

- Built with [Tauri](https://tauri.app/) - Desktop application framework
- UI powered by [Svelte](https://svelte.dev/) - Reactive web framework
- Styled with [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- Hash functions from [RustCrypto](https://github.com/RustCrypto) project

## Contact and Support

- **Issues**: [GitHub Issues](<repository-url>/issues)
- **Discussions**: [GitHub Discussions](<repository-url>/discussions)

This project is provided as-is without formal support commitments. Issue reports and pull requests are reviewed on a best-effort basis.

---

**Last Updated**: October 24, 2025  
**Minimum Rust Version**: 1.75.0  
**Minimum Node Version**: 18.0.0
