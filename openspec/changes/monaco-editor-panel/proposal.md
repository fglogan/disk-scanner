# Monaco-Based Pop-Out Editor Panel
## OpenSpec Proposal Document

**Document ID:** OS-MONACO-2025-10-27-v1  
**Status:** PROPOSAL (Awaiting Stakeholder Review)  
**Version:** 1.0  
**Date:** October 27, 2025  
**Author:** VOS-Coder (AI Assistant)  
**Related Project**: Disk Bloat Scanner v0.1.1  
**Baseline Scope**: Add sophisticated multi-format editor with rendering capabilities

---

## Executive Summary

The **Monaco-Based Pop-Out Editor Panel** system provides:

1. **Multi-Format Code Editor**: Click any file in UI → opens in Monaco with full language support
2. **Dual-Pane Layout**: Side-by-side editing (code + diff, code + preview, markdown + rendered)
3. **Language Support**: Source code (Rust, JS, TS, Python, etc.), configs (YAML, TOML, JSON), markup (Markdown, HTML, MDX), styles (CSS, Svelte), PDFs
4. **LSP Integration**: Real-time language services (IntelliSense, error checking, refactoring)
5. **Diff Viewer**: Compare baseline vs current, side-by-side visualization
6. **Markdown Preview**: Live rendering with table of contents, syntax highlighting
7. **Web UI Preview**: Render HTML/CSS/Svelte in iframe for visual inspection
8. **Tab Completion**: Context-aware autocomplete for all file types
9. **Pop-Out Window**: Separate editor window (optional, future native window)

### Primary Use Cases

1. **Browse Audit Results** - Click file in compliance report → edit config in Monaco
2. **Compare Specifications** - Load baseline spec + current spec, side-by-side diff
3. **Preview Markdown Docs** - View rendered documentation alongside source
4. **Inspect HTML Reports** - See generated HTML report rendered in webview
5. **Edit Configurations** - Modify YAML/TOML configs with validation and autocomplete
6. **Review Code Changes** - See diff between versions with syntax highlighting
7. **Validate JSON** - Edit and validate JSON with schema validation (future)
8. **Style Inspection** - View CSS/Svelte with live rendering preview

---

## Problem Statement

### Current State

- Files clickable in UI but open in external editor (not integrated)
- No side-by-side comparison capability
- Can't preview rendered output (Markdown, HTML, etc.) within app
- No diff viewer for compliance changes
- Configuration editing requires external tools
- No language services (autocomplete, error checking)
- Different file types handled inconsistently

### Target State

- ✅ Click any file → Monaco editor in pop-out panel
- ✅ Dual-pane layout for comparison, diff, and preview
- ✅ Language services for all supported formats
- ✅ Live rendering (Markdown, HTML, CSS)
- ✅ Tab completion and color coding
- ✅ Diff viewer with syntax highlighting
- ✅ Unified file handling across UI
- ✅ Configuration validation and suggestions

---

## Solution Architecture

### Component 1: Monaco Editor Panel (MEP)

**Purpose**: Resizable pop-out panel with Monaco editor and pane management

**Features**:
1. **Panel Layout**
   - Main pane: Monaco editor (always visible)
   - Secondary pane (optional): Diff, preview, rendered output
   - Resizable divider between panes
   - Close button, minimize button
   - Panel title shows current file

2. **File Handling**
   - Auto-detect file type (extension → language)
   - Load file content from filesystem
   - Display file path in breadcrumb
   - Show file size and modification time
   - Unsaved changes indicator (dot on tab)

3. **Tab Support**
   - Multiple files open simultaneously
   - Tab bar at top of editor
   - Click tab to switch files
   - Close tab button
   - Tab context menu (close, close all, close others)

4. **Panel Modes**
   - **Edit Mode**: Monaco editor only
   - **Diff Mode**: Monaco editor + diff viewer
   - **Preview Mode**: Editor + rendered preview
   - **Split Mode**: Two editors side-by-side

**Panel States**:
```
┌─────────────────────────────────────────────────────┐
│ [File.rs] [Config.yaml] [Doc.md] ✕                  │
├──────────────────────┬────────────────────────────┤
│                      │                            │
│   Monaco Editor      │   Preview/Diff/Editor    │
│   (Pane 1)           │   (Pane 2 - optional)   │
│                      │                            │
│                      │                            │
│   Ctrl+S to save     │   Context: Diff/Render   │
│   Syntax highlighting│   Side-by-side view      │
│                      │                            │
└──────────────────────┴────────────────────────────┘
```

**Pane Operations**:
- Resize divider (drag to adjust width)
- Toggle pane visibility (hide/show)
- Swap panes (main ↔ secondary)
- Full-screen pane (hide other)

### Component 2: File Type Handler System

**Purpose**: Detect and route files to appropriate editor/renderer

**File Type Support**:

| Category | Types | Handler | Features |
|----------|-------|---------|----------|
| **Source Code** | .rs, .js, .ts, .tsx, .jsx, .py, .go, .sh, .c, .cpp, .java, .scala | Monaco + LSP | Syntax, IntelliSense, errors |
| **Config** | .yaml, .yml, .toml, .json, .jsonc, .ini | Monaco + Validation | Schema validation, autocomplete |
| **Markup** | .md, .mdx, .html, .htm | Monaco + Renderer | Syntax + live preview |
| **Styling** | .css, .scss, .less, .svelte | Monaco + CSS preview | Syntax + style inspection |
| **Data** | .xml, .csv | Monaco + Table viewer | Syntax + data preview |
| **Documents** | .pdf | PDF viewer | Page-by-page rendering |
| **Text** | .txt, .log, .conf | Monaco | Basic text editing |

**File Detection Logic**:
```rust
pub fn detect_file_type(path: &Path) -> FileType {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            "rs" => FileType::Rust,
            "ts" => FileType::TypeScript,
            "md" => FileType::Markdown,
            "yaml" | "yml" => FileType::YAML,
            // ... more cases
            _ => FileType::PlainText,
        },
        None => FileType::PlainText,
    }
}
```

**Handler Selection**:
```rust
pub fn get_handler(file_type: FileType) -> EditorHandler {
    match file_type {
        FileType::SourceCode => handler::SourceCodeHandler,
        FileType::Config => handler::ConfigHandler,
        FileType::Markdown => handler::MarkdownHandler,
        FileType::HTML => handler::HTMLHandler,
        FileType::CSS => handler::CSSHandler,
        FileType::PDF => handler::PDFHandler,
        // ... more handlers
    }
}
```

### Component 3: LSP Integration (LSP)

**Purpose**: Language services (IntelliSense, error checking, refactoring)

**LSP Capabilities**:
- Hover information (type hints, documentation)
- Autocomplete (IntelliSense)
- Go to definition
- Find references
- Error and warning squiggles
- Code formatting (Rust, JS, etc.)
- Refactoring suggestions

**Supported Languages** (via LSP servers):
- Rust: `rust-analyzer`
- TypeScript/JavaScript: `typescript-language-server`
- Python: `pylance` or `pyright`
- YAML: `yaml-language-server`
- JSON: `vscode-json-languageserver`

**Configuration**:
```toml
[lsp]
enabled = true
servers = [
    { language = "rust", command = "rust-analyzer" },
    { language = "typescript", command = "typescript-language-server" },
    { language = "python", command = "pylance" },
    { language = "yaml", command = "yaml-language-server" },
]
timeout_ms = 5000
```

**Error Handling**:
- Graceful degradation if LSP server unavailable
- Editor works without LSP (basic syntax only)
- User notified of LSP status
- Option to restart LSP servers

### Component 4: Diff Viewer

**Purpose**: Side-by-side or unified diff display

**Diff Modes**:
1. **Side-by-Side**
   - Original on left, modified on right
   - Deletions highlighted in red
   - Additions highlighted in green
   - Context lines in neutral color
   - Line numbers on both sides

2. **Unified**
   - Single view with diff markers
   - Compact, easier to scroll
   - Collapsible sections

3. **Inline** (future)
   - Changes within editor, markers on gutter

**Diff Sources**:
- Compare file with baseline (PACS feature)
- Compare two files
- Compare file with Git history (future)
- Compare before/after compliance scan

**Diff Visualization**:
```
Left Pane (Original)          Right Pane (Modified)
────────────────────          ────────────────────
Line 1: config:               Line 1: config:
Line 2:   version: 1.0    ↔   Line 2:   version: 1.1  ✓ Added
Line 3:   name: "app"         Line 3:   name: "app"
Line 4:   debug: false    ↔   Line 4:   debug: true    ✓ Changed
Line 5:   ...                 Line 5:   timeout: 30    ✓ Added
```

### Component 5: Preview & Rendering System

**Purpose**: Render Markdown, HTML, CSS in-app preview

**Renderers**:

1. **Markdown Renderer**
   - Convert Markdown to HTML
   - Syntax highlighting for code blocks
   - Table of contents generation
   - GitHub-flavored Markdown support
   - LaTeX math rendering (future)

2. **HTML Renderer** (via iframe)
   - Render HTML in sandboxed iframe
   - CSS support
   - JavaScript disabled (safe preview)
   - Refresh on file change

3. **CSS Preview**
   - Live preview of styles
   - Shows affected elements
   - Color picker for colors
   - Font preview

4. **Svelte Preview** (future)
   - Compile Svelte to HTML/CSS
   - Live preview
   - Props editor

**Preview Updating**:
- Auto-refresh when file changes (debounced)
- Manual refresh button
- Full-screen preview mode

### Component 6: Tab Management System

**Purpose**: Manage multiple open files with tab interface

**Tab Features**:
- Tab shows file name + extension icon
- Unsaved indicator (dot or *)
- Close button on hover
- Context menu (close, close all, close others)
- Drag to reorder tabs (future)
- Tab persistence (remember open files)

**Tab State Persistence**:
- Save open tabs to `.beads/editor-state.json`
- Restore tabs on next session
- Remember cursor position per file
- Remember scroll position

**Maximum Tabs**: 20 (configurable)

---

## Feature Breakdown: 3 Features (Initial Release)

### Feature 1: Monaco Editor Panel (20 hrs)

**Purpose**: Core editor panel with file handling and multi-pane support

**Requirements**:
- Monaco editor integration
- File type detection and routing
- Tab management
- Dual-pane layout (editor + secondary)
- Resize/toggle pane
- Save functionality
- Unsaved changes tracking

**Deliverables**:
- `src/lib/components/MonacoEditorPanel.svelte` - Main editor component
- `src/lib/components/EditorTabs.svelte` - Tab bar component
- `src/lib/stores/editorStore.ts` - Editor state management
- `src/lib/utils/fileTypeDetector.ts` - File type detection
- Configuration: `editor-config.toml`
- 25+ unit & integration tests

**API Contract**:
```typescript
// Tauri commands
command open_file(file_path: string) → { content, type, language }
command save_file(file_path: string, content: string) → { success, error? }
command detect_file_type(file_path: string) → { type, language, icon }

// Svelte events
event "editor:file-opened" → { path, type }
event "editor:file-changed" → { path, unsaved }
event "editor:file-saved" → { path }
```

### Feature 2: Diff Viewer & Preview Renderers (15 hrs)

**Purpose**: Side-by-side diff display and content rendering

**Requirements**:
- Side-by-side diff viewer
- Markdown renderer with syntax highlighting
- HTML/CSS preview in iframe
- Syntax highlighting for diffs
- Line-by-line comparison

**Deliverables**:
- `src/lib/components/DiffViewer.svelte` - Diff display
- `src/lib/components/MarkdownPreview.svelte` - Markdown renderer
- `src/lib/components/HTMLPreview.svelte` - HTML preview
- `src/lib/utils/diffEngine.ts` - Diff calculation
- `src/lib/utils/markdownRenderer.ts` - Markdown to HTML
- 20+ unit tests

**API Contract**:
```typescript
// Tauri commands
command diff_files(file1: string, file2: string) → { diff, stats }
command render_markdown(content: string) → { html, toc }
command render_html(content: string) → { safe_html, warnings }
```

### Feature 3: LSP Integration & Configuration (20 hrs)

**Purpose**: Language services for autocomplete and error checking

**Requirements**:
- LSP server lifecycle management
- Hover information
- Autocomplete suggestions
- Error and warning diagnostics
- Configuration validation
- Graceful fallback without LSP

**Deliverables**:
- `src-tauri/src/lsp/mod.rs` - LSP module
- `src-tauri/src/lsp/client.rs` - LSP client
- `src-tauri/src/lsp/server_launcher.rs` - Launch LSP servers
- `src/lib/utils/lspBridge.ts` - LSP bridge to frontend
- Configuration: `lsp-config.toml`
- Example LSP configurations
- 25+ unit tests

**API Contract**:
```typescript
// Tauri commands
command initialize_lsp(language: string) → { initialized, status }
command get_completions(file_path: string, line: number, column: number) 
  → { completions: [{ label, kind, detail }] }
command get_hover_info(file_path: string, line: number, column: number) 
  → { content, range }
command get_diagnostics(file_path: string) → { diagnostics: [{ message, range, severity }] }

// Tauri events
event "lsp:initialized" → { language }
event "lsp:diagnostics" → { file_path, diagnostics }
event "lsp:completion-ready" → { completions }
```

---

## Technical Implementation Strategy

### Technology Stack

| Component | Tech | Justification |
|-----------|------|---------------|
| Editor | `monaco-editor` (npm) | Industry standard, full language support |
| Diff | `diff-match-patch` or `microdiff` | Fast, accurate diffing |
| Markdown | `marked` + `highlight.js` | Popular, reliable markdown rendering |
| HTML Preview | iframe sandbox | Secure, isolated rendering |
| LSP | `lsp-client` (Tauri) + LSP servers | Standard protocol for language services |
| State | Svelte stores | Reactive state management |
| TypeScript | Full type safety | Catch errors at compile time |

### Integration with Existing Features

**Integration Points**:
- **PACS**: Compare compliance reports (baseline vs current)
- **Project Scaffolding**: Edit generated configurations
- **Audit Reports**: Click file in report → edit in Monaco
- **Git Assistance**: View diffs and commits
- **Tray Menu**: "Edit Configuration" → Monaco editor

### Modular Architecture

```
src/lib/components/
├── MonacoEditorPanel.svelte      ← Main editor UI
├── EditorTabs.svelte             ← Tab management
├── DiffViewer.svelte             ← Diff display
├── MarkdownPreview.svelte        ← Markdown rendering
├── HTMLPreview.svelte            ← HTML/CSS preview
└── LSPIndicator.svelte           ← LSP status

src/lib/stores/
└── editorStore.ts                ← Editor state

src/lib/utils/
├── fileTypeDetector.ts           ← File detection
├── diffEngine.ts                 ← Diff calculation
├── markdownRenderer.ts           ← Markdown processing
└── lspBridge.ts                  ← LSP communication

src-tauri/src/lsp/
├── mod.rs                        ← LSP module
├── client.rs                     ← LSP client logic
└── server_launcher.rs            ← Server lifecycle
```

---

## File Type Support Matrix

### Languages with LSP Support

| Language | Extension | LSP Server | Features |
|----------|-----------|-----------|----------|
| Rust | .rs | rust-analyzer | ✅ Full support |
| TypeScript | .ts, .tsx | typescript-language-server | ✅ Full support |
| JavaScript | .js, .jsx | typescript-language-server | ✅ Full support |
| Python | .py | pylance / pyright | ✅ Full support |
| YAML | .yaml, .yml | yaml-language-server | ✅ Full support |
| JSON | .json, .jsonc | vscode-json-languageserver | ✅ Schema validation |
| HTML | .html, .htm | vscode-html-languageserver | ✅ Basic support |
| CSS | .css, .scss | vscode-css-languageserver | ✅ Basic support |

### Languages with Syntax Only (No LSP)

| Language | Extension | Features |
|----------|-----------|----------|
| Markdown | .md, .mdx | ✅ Syntax + Live preview |
| XML | .xml | ✅ Syntax |
| TOML | .toml | ✅ Syntax |
| Shell | .sh, .bash | ✅ Syntax |
| Go | .go | ✅ Syntax (LSP optional) |
| C/C++ | .c, .cpp, .h | ✅ Syntax (LSP optional) |

### Special Renderers (No Direct Editing)

| Type | Extension | Handler |
|------|-----------|---------|
| PDF | .pdf | PDF.js viewer |
| Image | .png, .jpg, .svg | Image viewer |
| Markdown (rendered) | .md | Full HTML rendering |
| HTML (preview) | .html | iframe sandbox |
| Svelte (compiled) | .svelte | Compile + preview |

---

## Configuration

### Editor Configuration (`editor-config.toml`)

```toml
[editor]
enabled = true
default_theme = "vs-dark"
font_size = 12
font_family = "Consolas, Monaco, Courier New"
tab_size = 2
insert_spaces = true
word_wrap = true
line_numbers = true
minimap = true

[editor.shortcuts]
save = "ctrl+s"
close_tab = "ctrl+w"
open_command_palette = "ctrl+shift+p"
find = "ctrl+f"
replace = "ctrl+h"

[panels]
max_tabs = 20
save_state = true
state_file = ".beads/editor-state.json"
```

### LSP Configuration (`lsp-config.toml`)

```toml
[lsp]
enabled = true
auto_start = true
timeout_ms = 5000

[[lsp.servers]]
language = "rust"
command = "rust-analyzer"
args = []
initializationOptions = { }

[[lsp.servers]]
language = "typescript"
command = "typescript-language-server"
args = ["--stdio"]

[[lsp.servers]]
language = "python"
command = "pylance"
args = ["--verbose"]
```

### Preview Configuration (`preview-config.toml`)

```toml
[preview]
markdown_theme = "github"
html_sandbox = true
html_max_size_mb = 10
auto_refresh_delay_ms = 500
```

---

## Security & Privacy

### Security Measures

- ✅ HTML preview in sandboxed iframe (no JS execution)
- ✅ File access restricted to project directory
- ✅ LSP servers isolated via subprocess
- ✅ Content Security Policy for previews
- ✅ Input validation for all file paths

### Privacy Measures

- ✅ No files transmitted externally
- ✅ Editor state stored locally (`.beads/`)
- ✅ LSP servers run locally
- ✅ No telemetry or usage tracking

---

## Success Criteria

### Feature 1 (Editor Panel) Acceptance Tests

1. ✅ Click file in UI → Monaco editor opens
2. ✅ File type auto-detected correctly
3. ✅ Syntax highlighting applied
4. ✅ Tab completion works (Ctrl+Space)
5. ✅ File can be edited and saved (Ctrl+S)
6. ✅ Unsaved changes indicator shows
7. ✅ Multiple files in tabs
8. ✅ Tab switching works
9. ✅ Close tab works
10. ✅ Dual-pane layout toggles
11. ✅ All 25+ tests pass

### Feature 2 (Diff & Preview) Acceptance Tests

1. ✅ Diff viewer displays two files side-by-side
2. ✅ Additions highlighted in green
3. ✅ Deletions highlighted in red
4. ✅ Markdown renders to HTML correctly
5. ✅ Markdown preview updates on file change
6. ✅ HTML renders in sandboxed iframe
7. ✅ CSS styles apply correctly
8. ✅ All 20+ tests pass

### Feature 3 (LSP) Acceptance Tests

1. ✅ LSP server initializes for supported language
2. ✅ Hover shows type information
3. ✅ Autocomplete suggestions appear
4. ✅ Errors/warnings show with squiggles
5. ✅ Configuration files validate schemas
6. ✅ Graceful fallback without LSP
7. ✅ All 25+ tests pass

---

## Risk Mitigation

### Risk 1: Large File Performance

**Mitigation**:
- Virtualize large files (show only visible lines)
- Lazy load file content
- Limit file size for LSP analysis
- Use web workers for diff calculation

### Risk 2: LSP Server Crashes

**Mitigation**:
- Auto-restart LSP servers
- Fallback to syntax-only mode
- Error logging and user notification
- Timeout mechanism to prevent hanging

### Risk 3: HTML Preview Security

**Mitigation**:
- Sandbox iframe with restricted permissions
- Content Security Policy
- Disable JavaScript in preview
- Limit iframe size/access

### Risk 4: Memory Usage (Many Tabs)

**Mitigation**:
- Lazy load tab content
- Unload inactive tabs
- Limit maximum open tabs (20)
- Show warning for many tabs

---

## Deployment & Integration

### Phase Allocation
- **Phase 3 Integration**: Add after Phase 2 Gate completion
- **Can work in parallel**: Independent of PACS, Tray Menu

### Backward Compatibility
- ✅ No breaking changes to existing UI
- ✅ Editor optional (files still open externally if not available)
- ✅ Graceful fallback for unavailable features
- ✅ Configuration optional (use defaults if not provided)

### Configuration Management
- Default configs in binary
- User overrides in `.beads/editor-config.toml`, `lsp-config.toml`, `preview-config.toml`
- Environment variable overrides for testing

---

## Deliverables Summary

### Code
- [ ] 800+ lines Svelte (editor components)
- [ ] 400+ lines TypeScript (utilities, LSP bridge)
- [ ] 600+ lines Rust (LSP client, server launcher)
- [ ] 50+ unit & integration tests
- [ ] Configuration files (TOML)

### Documentation
- [ ] Editor User Guide
- [ ] LSP Integration Guide
- [ ] Configuration Reference
- [ ] Supported Languages Reference
- [ ] Security & Sandbox Model
- [ ] Troubleshooting Guide

### Dependencies
- [ ] `monaco-editor` npm package
- [ ] `marked` (Markdown rendering)
- [ ] `highlight.js` (Code highlighting)
- [ ] `diff-match-patch` (Diff calculation)
- [ ] LSP servers (Rust, TS, Python, etc.)

---

## Next Steps

1. **Stakeholder Review**: Present proposal to project team
2. **Design Approval**: Feedback on architecture and features
3. **LSP Testing**: Verify LSP servers work in target environment
4. **Sprint Planning**: Break into Beads issues
5. **Sprint 1**: Implement Feature 1 (Editor Panel)
6. **Sprint 2**: Implement Feature 2 (Diff & Preview)
7. **Sprint 3**: Implement Feature 3 (LSP Integration)
8. **Integration**: Connect to Disk Bloat Scanner features
9. **Testing**: Cross-platform testing
10. **Deployment**: Release in next version

---

## References

- **Monaco Editor**: https://github.com/microsoft/monaco-editor
- **Language Server Protocol**: https://microsoft.github.io/language-server-protocol/
- **Svelte Documentation**: https://svelte.dev/
- **Tauri Documentation**: https://tauri.app/
- **Marked (Markdown)**: https://marked.js.org/
- **Highlight.js**: https://highlightjs.org/

---

**Document Status**: Ready for Stakeholder Review  
**Next Review**: Upon HIL Approval  
**Last Updated**: October 27, 2025

