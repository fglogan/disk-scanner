# Tauri + Svelte + TypeScript + Tailwind CSS Project Scaffolding Guide

**Best Practices for Tempext AI Projects**

**Author:** Tempext AI Engineering  
**Version:** 1.0  
**Last Updated:** October 24, 2025  
**Stack:** Tauri 2.x + Svelte 5.x + TypeScript 5.x + Tailwind CSS 4.x

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [Technology Stack Rationale](#technology-stack-rationale)
3. [Scaffolding Checklist](#scaffolding-checklist)
4. [Directory Structure Details](#directory-structure-details)
5. [Configuration Files](#configuration-files)
6. [Development Workflow](#development-workflow)
7. [Common Pitfalls & Solutions](#common-pitfalls--solutions)
8. [Testing Strategy](#testing-strategy)
9. [Build & Release](#build--release)

---

## Project Structure

```
tempext-project/
├── .github/                      # GitHub-specific files
│   ├── ISSUE_TEMPLATE/           # Issue templates
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── workflows/                # CI/CD workflows
│       ├── test.yml              # Run tests on PR
│       ├── build.yml             # Build artifacts
│       └── release.yml           # Publish releases
│
├── .vscode/                      # VSCode workspace settings
│   ├── extensions.json           # Recommended extensions
│   ├── settings.json             # Workspace settings
│   └── launch.json               # Debug configurations
│
├── docs/                         # Documentation
│   ├── design/                   # Design documents
│   │   ├── DESIGN_SPEC.md
│   │   └── ARCHITECTURE.md
│   ├── compliance/               # Standards & compliance
│   │   └── SECURITY.md
│   └── api/                      # API documentation
│       └── TAURI_COMMANDS.md     # Tauri command reference
│
├── public/                       # Static assets (served as-is)
│   ├── logo.png                  # App logo
│   ├── favicon.ico               # Favicon
│   └── assets/                   # Additional static assets
│
├── src/                          # Frontend source (Svelte + TypeScript)
│   ├── lib/                      # Reusable components & utilities
│   │   ├── components/           # ⭐ Svelte components
│   │   │   ├── common/           # Shared/generic components
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Modal.svelte
│   │   │   │   ├── Spinner.svelte
│   │   │   │   └── ErrorBoundary.svelte
│   │   │   ├── layout/           # Layout components
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   ├── Header.svelte
│   │   │   │   └── Footer.svelte
│   │   │   └── features/         # Feature-specific components
│   │   │       ├── Dashboard.svelte
│   │   │       ├── Settings.svelte
│   │   │       └── [FeatureName].svelte
│   │   ├── stores/               # ⭐ Svelte stores (state management)
│   │   │   ├── index.ts          # Export all stores
│   │   │   ├── ui.ts             # UI state (navigation, modals)
│   │   │   ├── data.ts           # Application data
│   │   │   └── settings.ts       # User settings
│   │   ├── services/             # ⭐ Service layer (Tauri IPC wrappers)
│   │   │   ├── index.ts
│   │   │   ├── fileService.ts    # File operations
│   │   │   ├── systemService.ts  # System info
│   │   │   └── scanService.ts    # Scan operations
│   │   ├── types/                # ⭐ TypeScript type definitions
│   │   │   ├── index.ts
│   │   │   ├── api.ts            # API request/response types
│   │   │   ├── models.ts         # Domain models
│   │   │   └── store.ts          # Store types
│   │   └── utils/                # ⭐ Utility functions
│   │       ├── index.ts
│   │       ├── format.ts         # Formatting (dates, sizes, etc.)
│   │       ├── validation.ts     # Input validation
│   │       └── constants.ts      # App-wide constants
│   ├── assets/                   # Assets imported by components
│   │   ├── tempext-logo.png      # ⭐ Tempext branding
│   │   ├── icons/                # SVG icons
│   │   └── styles/               # Global styles
│   │       └── global.css        # Global CSS (imports Tailwind)
│   ├── App.svelte                # ⭐ Root component
│   ├── main.ts                   # ⭐ Frontend entry point
│   └── index.html                # HTML shell
│
├── src-tauri/                    # ⭐ Tauri backend (Rust)
│   ├── capabilities/             # Tauri v2 permissions
│   │   └── default.json          # Default capability set
│   ├── icons/                    # App icons (auto-generated)
│   │   ├── 32x32.png
│   │   ├── 128x128.png
│   │   ├── icon.icns             # macOS
│   │   ├── icon.ico              # Windows
│   │   └── icon.png              # Linux
│   ├── src/                      # Rust source code
│   │   ├── commands/             # ⭐ Tauri commands (organized by domain)
│   │   │   ├── mod.rs            # Export all commands
│   │   │   ├── file_ops.rs       # File operations
│   │   │   ├── scanner.rs        # Scanning logic
│   │   │   └── system.rs         # System info
│   │   ├── models/               # ⭐ Data structures
│   │   │   ├── mod.rs
│   │   │   ├── scan_result.rs    # Scan result types
│   │   │   └── config.rs         # Configuration types
│   │   ├── services/             # ⭐ Business logic
│   │   │   ├── mod.rs
│   │   │   ├── file_service.rs   # File operations
│   │   │   └── scan_service.rs   # Scanning logic
│   │   ├── utils/                # ⭐ Utility functions
│   │   │   ├── mod.rs
│   │   │   ├── path.rs           # Path validation
│   │   │   └── error.rs          # Error types
│   │   ├── lib.rs                # ⭐ Library entry point (exports commands)
│   │   └── main.rs               # ⭐ Binary entry point (Tauri setup)
│   ├── tests/                    # Integration tests
│   │   └── integration_tests.rs
│   ├── .gitignore                # Rust-specific ignores
│   ├── build.rs                  # Build script
│   ├── Cargo.toml                # ⭐ Rust dependencies & metadata
│   └── tauri.conf.json           # ⭐ Tauri configuration
│
├── tests/                        # Frontend tests (if not using Vitest)
│   └── integration/
│
├── .env.example                  # Example environment variables
├── .gitignore                    # Git ignore patterns
├── .prettierrc.json              # Prettier configuration
├── eslint.config.js              # ESLint configuration
├── package.json                  # ⭐ NPM dependencies & scripts
├── package-lock.json             # NPM lock file
├── tsconfig.json                 # ⭐ TypeScript configuration
├── tsconfig.node.json            # TypeScript for build tools
├── tailwind.config.js            # ⭐ Tailwind CSS configuration
├── vite.config.ts                # ⭐ Vite configuration
├── vitest.config.ts              # Vitest test configuration
├── svelte.config.js              # Svelte configuration
├── README.md                     # Project documentation
├── ARCHITECTURE.md               # Architecture overview
├── CONTRIBUTING.md               # Contribution guidelines
├── LICENSE                       # License file
└── CHANGELOG.md                  # Version history
```

---

## Technology Stack Rationale

### Why Tauri?
- ✅ **Native performance** - Rust backend, native webview
- ✅ **Small bundle size** - ~3MB vs 100MB+ for Electron
- ✅ **Security-first** - Fine-grained permissions, CSP enforcement
- ✅ **Cross-platform** - macOS, Windows, Linux from single codebase
- ✅ **Modern architecture** - IPC layer separates UI from business logic

### Why Svelte 5?
- ✅ **True reactivity** - Runes (signals) for fine-grained reactivity
- ✅ **Minimal boilerplate** - Less code than React/Vue
- ✅ **Small bundle size** - Compiler outputs optimized JavaScript
- ✅ **Performance** - No virtual DOM overhead
- ✅ **Developer experience** - Intuitive, easy to learn

### Why TypeScript?
- ✅ **Type safety** - Catch errors at compile time
- ✅ **Better IDE support** - Autocomplete, refactoring
- ✅ **Self-documenting** - Types serve as inline documentation
- ✅ **Refactoring confidence** - Safe renames, structural changes

### Why Tailwind CSS?
- ✅ **Utility-first** - Rapid UI development
- ✅ **Consistency** - Design system built-in
- ✅ **Small production bundle** - Purges unused styles
- ✅ **Responsive design** - Mobile-first approach
- ✅ **Dark mode** - First-class support

---

## Scaffolding Checklist

### Phase 1: Initial Setup

```bash
# 1. Create Tauri app with Svelte template
npm create tauri-app@latest

# Choose:
# - Package manager: npm
# - Frontend template: Svelte
# - TypeScript: Yes
# - Markup/Styling: None (we'll add Tailwind manually)
# - Run 'npm install': Yes

# 2. Navigate to project
cd your-project-name

# 3. Install Tailwind CSS 4.x
npm install -D tailwindcss@next @tailwindcss/vite@next
```

### Phase 2: Configure Tailwind

**vite.config.ts:**
```typescript
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    svelte(),
    tailwindcss(), // Add Tailwind plugin
  ],
  clearScreen: false,
  server: {
    port: 3001,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
```

**src/assets/styles/global.css:**
```css
@import "tailwindcss";

/* Your custom global styles here */
body {
  margin: 0;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}
```

**src/main.ts:**
```typescript
import './assets/styles/global.css'; // Import Tailwind
import App from './App.svelte';

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
```

### Phase 3: Project Structure Setup

```bash
# Create frontend directories
mkdir -p src/lib/{components/{common,layout,features},stores,services,types,utils}
mkdir -p src/assets/{icons,styles}

# Create Rust backend directories
mkdir -p src-tauri/src/{commands,models,services,utils}

# Create documentation directories
mkdir -p docs/{design,compliance,api}

# Create test directories
mkdir -p src-tauri/tests
```

### Phase 4: Essential Configuration Files

**tsconfig.json:**
```json
{
  "extends": "@tsconfig/svelte/tsconfig.json",
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "baseUrl": ".",
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.svelte"],
  "exclude": ["node_modules", "dist", "build"]
}
```

**tailwind.config.js:**
```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      colors: {
        tempext: {
          blue: '#1e40af',
          green: '#10b981',
        },
      },
    },
  },
  plugins: [],
};
```

**.prettierrc.json:**
```json
{
  "semi": true,
  "singleQuote": false,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100,
  "plugins": ["prettier-plugin-svelte"],
  "overrides": [
    {
      "files": "*.svelte",
      "options": {
        "parser": "svelte"
      }
    }
  ]
}
```

**src-tauri/Cargo.toml (essential dependencies):**
```toml
[package]
name = "your-app-name"
version = "0.1.0"
description = "Your app description"
authors = ["Tempext AI"]
license = "Apache-2.0"
edition = "2021"
rust-version = "1.77.2"

[lib]
name = "your_app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tauri = { version = "2", features = [] }
tauri-plugin-dialog = "2"
tauri-plugin-log = "2"
tokio = { version = "1", features = ["full"] }
log = "0.4"
thiserror = "1" # For better error types

[dev-dependencies]
tempfile = "3"

[lints.clippy]
pedantic = "warn"
unwrap_used = "warn"
expect_used = "warn"
panic = "warn"

[lints.rust]
unsafe_code = "warn"
```

**src-tauri/tauri.conf.json (critical settings):**
```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Your App Name",
  "version": "0.1.0",
  "identifier": "com.tempext.yourapp",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:3001",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "Your App Name",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self' data:"
    }
  }
}
```

**package.json (essential scripts):**
```json
{
  "name": "your-app-name",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "tauri:dev": "tauri dev",
    "tauri:build": "tauri build",
    "test": "vitest run",
    "test:watch": "vitest",
    "test:ui": "vitest --ui",
    "lint": "eslint src --ext .ts,.svelte",
    "format": "prettier --write 'src/**/*.{ts,svelte,css}'"
  }
}
```

---

## Directory Structure Details

### Frontend (`src/`)

#### `/lib/components/` - Component Organization

**Principles:**
1. **Organize by domain**, not by type
2. **Collocate related files** (component + tests + types)
3. **Use clear naming** (`UserProfile.svelte`, not `Profile.svelte`)

**Example:**
```
components/
├── common/              # Reusable across features
│   ├── Button.svelte
│   ├── Button.test.ts
│   ├── Modal.svelte
│   └── Spinner.svelte
├── layout/              # Page structure
│   ├── Sidebar.svelte
│   ├── Header.svelte
│   └── MainLayout.svelte
└── features/            # Feature-specific
    ├── Dashboard.svelte
    ├── Settings.svelte
    └── FileScanner/
        ├── ScanResults.svelte
        ├── ScanProgress.svelte
        └── types.ts
```

#### `/lib/stores/` - State Management

**Best Practices:**
```typescript
// src/lib/stores/ui.ts
import { writable, derived } from 'svelte/store';

export const currentPage = writable<string>('dashboard');
export const isLoading = writable<boolean>(false);
export const errorMessage = writable<string | null>(null);

// Derived store
export const hasError = derived(
  errorMessage,
  ($errorMessage) => $errorMessage !== null
);
```

**Store Types:**
- `ui.ts` - Navigation, modals, loading states
- `data.ts` - Application data (scan results, file lists)
- `settings.ts` - User preferences
- `auth.ts` - Authentication state (if needed)

#### `/lib/services/` - Tauri IPC Wrappers

**Purpose:** Centralize all Tauri command invocations

**Example:**
```typescript
// src/lib/services/scanService.ts
import { invoke } from '@tauri-apps/api/core';
import type { ScanResult, ScanOptions } from '$lib/types';

export const scanService = {
  async scanDirectory(options: ScanOptions): Promise<ScanResult[]> {
    return invoke<ScanResult[]>('scan_directory', { opts: options });
  },

  async cancelScan(): Promise<void> {
    return invoke('cancel_scan');
  },
};
```

**Benefits:**
- ✅ Type-safe API calls
- ✅ Single source of truth for backend communication
- ✅ Easy to mock for testing
- ✅ Clear dependency injection points

#### `/lib/types/` - TypeScript Definitions

**Organization:**
```typescript
// src/lib/types/index.ts - Re-export all types
export * from './api';
export * from './models';
export * from './store';

// src/lib/types/api.ts - Request/Response types
export interface ScanOptions {
  root: string;
  minBytes: number;
  followSymlinks: boolean;
}

export interface ScanResult {
  path: string;
  sizeMb: number;
  category: string;
}

// src/lib/types/models.ts - Domain models
export interface FileEntry {
  path: string;
  size: number;
  modified: Date;
}

// src/lib/types/store.ts - Store types
export interface AppState {
  currentPage: string;
  isScanning: boolean;
  results: ScanResult[];
}
```

### Backend (`src-tauri/src/`)

#### `/commands/` - Tauri Command Handlers

**Best Practices:**
1. **One command per function**
2. **Proper error handling** (use `Result<T, E>`)
3. **Input validation**
4. **Logging**

**Example:**
```rust
// src-tauri/src/commands/scanner.rs
use crate::models::ScanOptions;
use crate::services::scan_service;
use crate::utils::error::AppError;

#[tauri::command]
pub async fn scan_directory(opts: ScanOptions) -> Result<Vec<ScanResult>, AppError> {
    log::info!("Starting scan: {:?}", opts);
    
    // Validate input
    let validated_opts = opts.validate()?;
    
    // Perform scan
    let results = scan_service::scan(&validated_opts).await?;
    
    log::info!("Scan complete: {} results", results.len());
    Ok(results)
}
```

**Command Organization:**
```rust
// src-tauri/src/commands/mod.rs
pub mod scanner;
pub mod system;
pub mod file_ops;

// Re-export commands
pub use scanner::*;
pub use system::*;
pub use file_ops::*;
```

**Register in main.rs:**
```rust
// src-tauri/src/main.rs
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::scan_directory,
            commands::get_system_info,
            commands::cleanup_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### `/models/` - Data Structures

```rust
// src-tauri/src/models/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptions {
    pub root: String,
    pub min_bytes: u64,
    pub follow_symlinks: bool,
}

impl ScanOptions {
    pub fn validate(self) -> Result<Self, String> {
        // Validation logic
        if self.root.is_empty() {
            return Err("Root path cannot be empty".to_string());
        }
        Ok(self)
    }
}
```

#### `/services/` - Business Logic

**Separation of concerns:**
```rust
// src-tauri/src/services/scan_service.rs
use crate::models::{ScanOptions, ScanResult};
use std::path::Path;

pub async fn scan(opts: &ScanOptions) -> Result<Vec<ScanResult>, std::io::Error> {
    // Pure business logic - no Tauri dependencies
    // Easier to test, reusable
    let results = perform_scan_logic(Path::new(&opts.root), opts)?;
    Ok(results)
}

fn perform_scan_logic(path: &Path, opts: &ScanOptions) -> Result<Vec<ScanResult>, std::io::Error> {
    // Implementation
    todo!()
}
```

#### `/utils/` - Utility Functions

```rust
// src-tauri/src/utils/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Implement Serialize for Tauri IPC
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
```

---

## Development Workflow

### Package Management

**Frontend (npm):**
```bash
# Install dependencies
npm install

# Add production dependency
npm install package-name

# Add dev dependency
npm install -D package-name

# Update dependencies
npm update

# Audit security
npm audit
```

**Backend (Cargo):**
```bash
# Build Rust code
cargo build

# Run tests
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Add dependency
cargo add package-name

# Add dev dependency
cargo add --dev package-name

# Update dependencies
cargo update
```

### Running the App

**Development:**
```bash
npm run tauri:dev
```

This runs:
1. `npm run dev` (Vite dev server on port 3001)
2. `cargo build` (Rust backend)
3. Opens Tauri window with hot-reload

**Production Build:**
```bash
npm run tauri:build
```

Output:
- **macOS:** `src-tauri/target/release/bundle/dmg/`
- **Windows:** `src-tauri/target/release/bundle/msi/`
- **Linux:** `src-tauri/target/release/bundle/deb/` or `.appimage`

---

## Common Pitfalls & Solutions

### 1. CSP Blocks Inline Styles
**Problem:** Tailwind's `style` attributes blocked by CSP

**Solution:**
```json
// tauri.conf.json
{
  "app": {
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline';"
    }
  }
}
```

### 2. Tauri Commands Not Found
**Problem:** `invoke('my_command')` returns "command not found"

**Checklist:**
- ✅ Function has `#[tauri::command]` attribute
- ✅ Function is public (`pub async fn`)
- ✅ Function registered in `invoke_handler![]`
- ✅ Function name uses snake_case in Rust, camelCase in TypeScript

### 3. CORS Errors in Dev Mode
**Problem:** API calls fail with CORS errors

**Solution:** Use Tauri's IPC, not HTTP requests
```typescript
// ❌ Wrong - HTTP request
fetch('http://localhost:8000/api/data')

// ✅ Correct - Tauri IPC
invoke('get_data')
```

### 4. Path Separators on Windows
**Problem:** Hardcoded `/` in paths breaks on Windows

**Solution:**
```rust
use std::path::PathBuf;

let path = PathBuf::from(&opts.root)
    .join("subdirectory")
    .join("file.txt");
```

### 5. Large Files Crash App
**Problem:** Loading large files into memory causes crash

**Solution:** Stream or chunk large operations
```rust
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

async fn read_large_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;
    Ok(buffer)
}
```

### 6. TypeScript Errors with Svelte Stores
**Problem:** `$` syntax not recognized

**Solution:** Use proper Svelte TypeScript setup
```typescript
// Use in <script lang="ts">
import { writable, type Writable } from 'svelte/store';

const count: Writable<number> = writable(0);

// In template: {$count}
```

---

## Testing Strategy

### Frontend Tests (Vitest + Testing Library)

**Install:**
```bash
npm install -D vitest @testing-library/svelte @testing-library/jest-dom jsdom
```

**vitest.config.ts:**
```typescript
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/test/setup.ts'],
  },
});
```

**Example Component Test:**
```typescript
// src/lib/components/Button.test.ts
import { render, fireEvent } from '@testing-library/svelte';
import { describe, it, expect } from 'vitest';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders with text', () => {
    const { getByText } = render(Button, { props: { label: 'Click me' } });
    expect(getByText('Click me')).toBeInTheDocument();
  });

  it('emits click event', async () => {
    const { component, getByRole } = render(Button, { props: { label: 'Click' } });
    
    let clicked = false;
    component.$on('click', () => { clicked = true; });
    
    await fireEvent.click(getByRole('button'));
    expect(clicked).toBe(true);
  });
});
```

### Backend Tests (Rust)

**Unit Tests:**
```rust
// src-tauri/src/services/scan_service.rs
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_scan_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let opts = ScanOptions {
            root: temp_dir.path().to_string_lossy().to_string(),
            min_bytes: 0,
            follow_symlinks: false,
        };
        
        let results = scan(&opts).await.unwrap();
        assert_eq!(results.len(), 0);
    }
}
```

**Integration Tests:**
```rust
// src-tauri/tests/integration_tests.rs
use disk_bloat_scanner_lib::commands::scan_directory;

#[tokio::test]
async fn test_scan_command() {
    let result = scan_directory(ScanOptions {
        root: "/tmp".to_string(),
        min_bytes: 1024,
        follow_symlinks: false,
    }).await;
    
    assert!(result.is_ok());
}
```

---

## Build & Release

### Version Bump Checklist

1. Update `package.json` version
2. Update `src-tauri/Cargo.toml` version
3. Update `src-tauri/tauri.conf.json` version
4. Update `CHANGELOG.md`
5. Commit: `git commit -m "chore: bump version to X.Y.Z"`
6. Tag: `git tag vX.Y.Z`
7. Push: `git push && git push --tags`

### Build Commands

**Development Build:**
```bash
npm run tauri build -- --debug
```

**Production Build:**
```bash
npm run tauri build
```

**Platform-Specific:**
```bash
# macOS only
npm run tauri build -- --target aarch64-apple-darwin

# Windows only  
npm run tauri build -- --target x86_64-pc-windows-msvc

# Linux only
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

### Code Signing

**macOS:**
```bash
# In src-tauri/tauri.conf.json
{
  "bundle": {
    "macOS": {
      "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)"
    }
  }
}
```

**Windows:**
Set up in CI/CD with certificate

---

## Summary: Key Principles

1. **Separation of Concerns**
   - UI in Svelte components
   - Business logic in Rust services
   - IPC commands as thin wrappers

2. **Type Safety**
   - TypeScript in frontend
   - Shared types between Rust and TypeScript
   - Strict compiler settings

3. **Error Handling**
   - `Result<T, E>` in Rust
   - Try/catch in TypeScript
   - User-friendly error messages

4. **Performance**
   - Async operations for I/O
   - Streaming for large data
   - Lazy loading in UI

5. **Security**
   - Enable CSP
   - Validate all inputs
   - Minimal permissions

6. **Maintainability**
   - Clear directory structure
   - Consistent naming
   - Comprehensive documentation
   - Automated testing

---

**This guide represents Tempext AI's best practices as of October 2025. Update as the stack evolves.**
