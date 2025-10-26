# Release Notes - Disk Bloat Scanner v0.1.0-alpha

**Release Date**: October 24, 2025  
**Status**: Alpha Release - macOS Only

## ⚠️ Critical Warnings

- **ALPHA SOFTWARE**: This is early-stage software. Use with caution on production systems.
- **macOS ONLY**: This release has been built and tested exclusively on macOS 14+. Windows and Linux builds are theoretically possible but **completely untested**.
- **DATA SAFETY**: While files are moved to system trash (not permanently deleted), always maintain backups before using cleanup features.
- **NO WARRANTY**: This software is provided "AS IS" without any warranty. Use at your own risk.

## Platform Support

| Platform | Build Status | Test Status | Recommended |
|----------|--------------|-------------|-------------|
| macOS 14+ | ✅ Built | ✅ Tested | ✅ Yes |
| Windows | ⚠️ Untested | ❌ Not tested | ❌ No |
| Linux | ⚠️ Untested | ❌ Not tested | ❌ No |

## What's Included

### Application Bundle
- **macOS App Bundle**: `Disk Bloat Scanner.app`
- **Location**: `target/release/bundle/macos/Disk Bloat Scanner.app`
- **Size**: ~34 MB
- **Architecture**: x64 (Intel/Rosetta)

### Features Implemented

#### Core Scanning
- ✅ **Disk Usage Display**: Real-time disk space monitoring
- ✅ **System Information**: CPU count, memory usage, OS version, hostname
- ✅ **Project Bloat Detection**: Finds build artifacts
  - Node.js (`node_modules/`)
  - Rust (`target/`)
  - Python (`venv/`, `__pycache__/`)
  - Git (`.git/`)
  - iOS/Android build directories
- ✅ **Large File Scanner**: Configurable threshold (default 100MB+)
- ✅ **Duplicate File Detection**: SHA256-based with 100MB file size limit

#### User Experience
- ✅ **Progressive Scan Updates**: Real-time feedback during scans
- ✅ **Safety Indicators**: Color-coded file deletion warnings
  - 🟢 Green: Safe to delete (caches, build outputs)
  - 🟡 Amber: Review carefully (media, archives)
  - 🔴 Red: Danger zone (source code, documents)
- ✅ **Current Directory Display**: Shows which directory is being scanned
- ✅ **Trash Integration**: Files moved to macOS Trash, not deleted permanently

#### UI Components
- ✅ Dashboard with system overview
- ✅ Large Files browser with safety indicators
- ✅ Project Bloat categorized view
- ✅ Duplicates detection (UI incomplete)
- ✅ Settings panel (partially functional)
- ⚠️ Dev Caches page (placeholder)
- ⚠️ Git Scanner page (placeholder)

## Known Issues

### Critical
- **Delete Confirmation Dialog**: Confirm dialog may return false unexpectedly in some scenarios
- **Duplicate UI**: Duplicates are detected but UI for reviewing them is incomplete

### Minor
- Settings "Add Directory" button non-functional
- Scheduled scans not implemented
- No export functionality (CSV/JSON)
- Performance not optimized for very large directory trees (>1M files)

### Platform-Specific
- Windows: File path handling may not work correctly
- Windows: Trash functionality untested
- Linux: Desktop integration untested
- All platforms: Only tested on macOS 14.x

## Technical Stack

### Frontend
- Svelte 5.39.6 (reactive UI framework)
- Tailwind CSS 4.1.16 (utility-first CSS)
- Vite 7.1.12 (build tool)

### Backend
- Rust 1.89.0 (systems programming language)
- Tauri 2.8.5+ (desktop app framework)
- sysinfo 0.30.13 (system information)
- sha2 (cryptographic hashing)
- rayon (parallel processing)
- trash 3.3.1 (safe file deletion)

### Architecture
- **IPC Communication**: Tauri commands between Rust backend and Svelte frontend
- **Async Operations**: Non-blocking file system operations
- **Progressive Updates**: Sequential scans with UI updates after each completes

## Installation

### macOS

1. **Download** the app bundle (or build from source)
2. **Copy** to `/Applications/`
3. **First Launch**: Right-click → Open (macOS Gatekeeper will prompt)
4. **Allow** file system access when prompted

### Building from Source

```bash
# Prerequisites
# - Rust 1.75.0+
# - Node.js 18.0.0+
# - Xcode Command Line Tools (macOS)

# Clone repository
git clone <repository-url>
cd disk-bloat-scanner

# Install dependencies
npm install

# Development mode
npm run tauri:dev

# Production build
npm run tauri:build
```

Build output: `target/release/bundle/macos/`

## Usage Guide

### Basic Workflow

1. **Launch** the application
2. **Review** system information in the Dashboard banner
3. **Click** "Scan Manually Now" to start scanning
4. **Monitor** progress with live status updates
5. **Review** results by category (Large Files, Project Bloat, Duplicates)
6. **Check** safety indicators before selecting files
7. **Select** files to remove using checkboxes
8. **Delete** (moves to Trash - recoverable)

### Safety Best Practices

- ✅ Start with a small test directory
- ✅ Review safety indicators (green/amber/red)
- ✅ Verify files in Trash before emptying
- ✅ Keep backups of important data
- ✅ Use on non-critical directories first
- ❌ Don't run as administrator unless necessary
- ❌ Don't delete files you don't understand
- ❌ Don't empty Trash immediately after cleanup

## What's Not Included

- ❌ Windows/Linux support (untested)
- ❌ Code signing (macOS Gatekeeper will warn)
- ❌ Auto-update mechanism
- ❌ Network drive scanning
- ❌ Cloud storage integration
- ❌ Scheduled background scans
- ❌ Export/reporting features
- ❌ Custom bloat pattern editor

## Roadmap (Future Versions)

### v0.2.0 (Planned)
- Complete duplicates UI
- Fix delete confirmation dialog
- Add directory management in Settings
- Windows and Linux testing
- Performance optimizations

### v0.3.0 (Planned)
- Scheduled automated scans
- Export results (CSV/JSON)
- Custom bloat patterns
- Multi-directory support improvements

### v1.0.0 (Future)
- Code signing for all platforms
- Auto-update functionality
- Advanced filtering and search
- Network drive support
- Comprehensive test coverage

## License

Apache License 2.0

See LICENSE file for full text.

## Disclaimer

THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

**USE AT YOUR OWN RISK. ALWAYS MAINTAIN BACKUPS OF IMPORTANT DATA.**

## Support

- **Issues**: GitHub Issues (best effort)
- **Discussions**: GitHub Discussions
- **No Warranty**: No formal support commitment

## Acknowledgments

Built with open-source tools:
- Tauri - Desktop application framework
- Svelte - Reactive UI framework
- Tailwind CSS - Utility-first CSS
- Rust ecosystem libraries (sysinfo, rayon, sha2, trash, etc.)

---

**Version**: 0.1.0-alpha  
**Build Date**: October 24, 2025  
**Target Platform**: macOS 14+ (x64)  
**Bundle Identifier**: com.diskbloat.scanner
