# PROPOSAL: MVP Production Pipeline

**Project:** Disk Bloat Scanner  
**Change ID:** MVP-PROD-001  
**Created:** 2025-11-12  
**Status:** DRAFT  
**Domain:** Production Release Engineering  
**LAIO Markers:** BUILD, SIGN, DISTRIBUTE, AUTOMATE

## 1. PROBLEM STATEMENT

### 1.1 Current State
The Disk Bloat Scanner (v0.1.1) has achieved Phase 2 completion with excellent code quality:
- Zero panic!() calls in production code
- All unwrap()/expect() usage confined to test code
- 86 tests passing with >50% coverage
- Modular architecture with proper error handling

However, the project lacks:
- Production build pipeline
- Code signing and notarization setup
- 35 pending BEADS issues for feature completion
- Automated release process
- Distribution mechanism

### 1.2 Problem Evidence
```
$ grep -r "panic!(" src-tauri/src --include="*.rs"
# No results - zero panics in production

$ grep -r "unwrap()" src-tauri/src --include="*.rs" | grep -v "test"
# No results - unwraps only in tests

$ cat docs/BEADS_ISSUE_TRACKER.md | grep "PENDING" | wc -l
35  # Outstanding issues blocking MVP
```

### 1.3 Impact
Without a production pipeline:
- Cannot distribute to users
- Cannot pass macOS Gatekeeper
- Cannot provide automatic updates
- Cannot track deployments
- Cannot ensure consistent builds

## 2. PROPOSED SOLUTION

### 2.1 Solution Overview
Implement a comprehensive MVP production pipeline following EDGS phases with LAIO domain expertise, creating signed and notarized macOS DMG builds at each gate.

### 2.2 Technical Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   CI/CD Pipeline                        │
├─────────────────────────────────────────────────────────┤
│  Source → Build → Test → Sign → Package → Notarize     │
└─────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────┐
│                 Distribution Channels                    │
├─────────────────────────────────────────────────────────┤
│  GitHub Releases → Auto-Update Server → User Install    │
└─────────────────────────────────────────────────────────┘
```

### 2.3 Implementation Phases (EDGS)

#### Phase 3: Production Infrastructure (Gate 3)
**Objective:** Establish build and signing pipeline

**Tasks:**
1. Configure code signing certificates
2. Setup notarization workflow
3. Create universal binary builds
4. Implement DMG packaging
5. Add entitlements for hardened runtime

**Gate 3 Success Criteria:**
- Signed universal binary builds successfully
- DMG passes notarization
- Clean install on macOS 11-15
- No Gatekeeper warnings

#### Phase 4: Feature Completion (Gate 4)
**Objective:** Implement 35 pending BEADS issues

**Task Groups by Priority:**
- **Critical (2)**: BEAD-005, BEAD-006
- **High (11)**: BEAD-009 through BEAD-020
- **Medium (14)**: BEAD-021 through BEAD-035
- **Low (8)**: BEAD-036 through BEAD-043

**Gate 4 Success Criteria:**
- All 35 BEADS implemented
- No regression in test coverage
- Performance benchmarks maintained
- UI/UX review passed

#### Phase 5: Release Automation (Gate 5)
**Objective:** Fully automated release pipeline

**Tasks:**
1. GitHub Actions workflow
2. Automated version bumping
3. Changelog generation
4. Release note automation
5. Update server deployment

**Gate 5 Success Criteria:**
- Push tag → DMG published automatically
- Update notifications working
- Rollback capability verified
- Zero manual steps

## 3. ADDED COMPONENTS

### 3.1 Build Infrastructure
```yaml
# .github/workflows/release.yml
name: Production Release
on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-apple-darwin,aarch64-apple-darwin
      
      - name: Build Universal Binary
        run: |
          npm ci
          npm run tauri:build -- --target universal-apple-darwin
      
      - name: Code Sign & Notarize
        env:
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
          IDENTITY: ${{ secrets.DEVELOPER_ID }}
        run: |
          # Sign
          codesign --deep --force --verify --verbose \
            --options runtime \
            --entitlements entitlements.plist \
            --sign "$IDENTITY" \
            "src-tauri/target/*/release/bundle/macos/*.app"
          
          # Package
          npm run package:dmg
          
          # Notarize
          xcrun notarytool submit *.dmg \
            --apple-id "$APPLE_ID" \
            --password "$APPLE_PASSWORD" \
            --wait
          
          # Staple
          xcrun stapler staple *.dmg
      
      - name: Upload Release
        uses: softprops/action-gh-release@v1
        with:
          files: '*.dmg'
```

### 3.2 Entitlements Configuration
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <!-- Required for Tauri -->
  <key>com.apple.security.cs.allow-jit</key>
  <true/>
  <!-- File system access -->
  <key>com.apple.security.files.user-selected.read-write</key>
  <true/>
  <!-- Network for updates -->
  <key>com.apple.security.network.client</key>
  <true/>
</dict>
</plist>
```

### 3.3 DMG Creation Script
```bash
#!/bin/bash
# scripts/create-dmg.sh
set -euo pipefail

VERSION=$(node -p "require('./package.json').version")
APP_NAME="Disk Bloat Scanner"
DMG_NAME="DiskBloatScanner-${VERSION}"

create-dmg \
  --volname "${APP_NAME}" \
  --volicon "src-tauri/icons/icon.icns" \
  --background "assets/dmg-background.png" \
  --window-pos 200 120 \
  --window-size 600 450 \
  --icon-size 100 \
  --icon "${APP_NAME}.app" 150 225 \
  --hide-extension "${APP_NAME}.app" \
  --app-drop-link 450 225 \
  --no-internet-enable \
  "${DMG_NAME}.dmg" \
  "src-tauri/target/universal-apple-darwin/release/bundle/macos/"
```

### 3.4 Auto-Update Configuration
```rust
// src-tauri/src/updater.rs
use tauri_plugin_updater::UpdaterExt;

pub fn setup_updater(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    let updater = app.updater();
    
    // Configure update endpoints
    updater.set_endpoints(vec![
        "https://releases.tempext.com/disk-bloat-scanner/updates.json"
    ])?;
    
    // Check on startup
    tauri::async_runtime::spawn(async move {
        match updater.check().await {
            Ok(update) => {
                if update.available {
                    log::info!("Update available: {}", update.version);
                    // Notify user through UI
                }
            }
            Err(e) => log::error!("Update check failed: {}", e),
        }
    });
    
    Ok(())
}
```

## 4. MODIFIED COMPONENTS

### 4.1 Tauri Configuration
```json
// tauri.conf.json
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev",
    "devPath": "http://localhost:3001",
    "distDir": "../public"
  },
  "package": {
    "productName": "Disk Bloat Scanner",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": true,
        "scope": ["$HOME/**", "$TEMP/**", "$RESOURCE/**"]
      },
      "dialog": {
        "open": true,
        "save": true
      },
      "shell": {
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "category": "DeveloperTool",
      "copyright": "© 2025 Tempext AI",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.tempext.disk-bloat-scanner",
      "macOS": {
        "entitlements": "../entitlements.plist",
        "exceptionDomain": "",
        "frameworks": [],
        "license": "../LICENSE",
        "minimumSystemVersion": "11.0",
        "hardenedRuntime": true,
        "providerShortName": "TEMPEXT",
        "signingIdentity": "Developer ID Application"
      },
      "targets": ["dmg", "updater"]
    },
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:;"
    },
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.tempext.com/disk-bloat-scanner/{{target}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IHB1YmtleTogUldUeW41SXlaM2..."
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "Disk Bloat Scanner",
        "width": 1200,
        "minHeight": 600,
        "minWidth": 800
      }
    ]
  }
}
```

### 4.2 Error Type Migration (BEAD-006)
```rust
// src-tauri/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    
    #[error("Cannot compare values: {0} (likely NaN or Inf)")]
    InvalidFloatComparison(String),
    
    #[error("Concurrent access error: {0}")]
    LockPoisoned(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Deletion failed: {0}")]
    DeletionFailed(String),
    
    #[error("{0}")]
    Other(String),
    
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

## 5. LAIO GOVERNANCE

### 5.1 Domain Boundaries
- **BUILD**: Compilation and packaging processes
- **SIGN**: Code signing and identity management  
- **DISTRIBUTE**: Release channels and delivery
- **AUTOMATE**: CI/CD pipeline and workflows

### 5.2 Domain Interactions
```
BUILD → SIGN → DISTRIBUTE
  ↑              ↓
  └── AUTOMATE ──┘
```

### 5.3 Constitution Rules
1. Every release MUST be signed and notarized
2. Version numbers MUST follow semver
3. Releases MUST pass all tests
4. DMGs MUST be universal binaries
5. Updates MUST be backwards compatible

## 6. SUCCESS CRITERIA

### 6.1 Technical Metrics
- Build time < 10 minutes
- DMG size < 50MB
- Install time < 30 seconds
- Update check latency < 1 second
- Zero security warnings

### 6.2 Quality Gates
- ✅ All 86 tests passing
- ✅ Zero compiler warnings
- ✅ Zero production panics
- ✅ Successful notarization
- ✅ Clean Gatekeeper pass

### 6.3 User Experience
- One-click installation
- Automatic updates
- No permission prompts beyond initial
- Native macOS experience
- Proper app icon at all sizes

## 7. IMPLEMENTATION SCHEDULE

### Phase 3: Production Infrastructure
```
Week 1:
├── Monday: Setup Apple Developer account
├── Tuesday: Configure certificates  
├── Wednesday: Implement signing workflow
├── Thursday: Create DMG packaging
└── Friday: Test on multiple macOS versions
```

### Phase 4: Feature Completion  
```
Weeks 2-5:
├── Critical BEADs (2 issues)
├── High Priority BEADs (11 issues)
├── Medium Priority BEADs (14 issues)
└── Low Priority BEADs (8 issues)
```

### Phase 5: Release Automation
```
Week 6:
├── Monday-Tuesday: GitHub Actions setup
├── Wednesday: Update server deployment
├── Thursday: End-to-end testing
└── Friday: MVP release (v1.0.0)
```

## 8. ROLLBACK PLAN

If issues arise:
1. Previous DMG versions remain available
2. Update server can pin versions
3. Manual override for auto-update
4. GitHub Releases preserve all artifacts
5. Version-specific documentation maintained

## 9. APPENDIX: TECHNICAL DETAILS

### 9.1 Universal Binary Build
```bash
# Build for both architectures
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build universal binary
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Combine with lipo
lipo -create \
  target/x86_64-apple-darwin/release/disk-bloat-scanner \
  target/aarch64-apple-darwin/release/disk-bloat-scanner \
  -output target/release/disk-bloat-scanner
```

### 9.2 Update Manifest
```json
{
  "version": "1.0.0",
  "date": "2025-11-20",
  "platforms": {
    "darwin-universal": {
      "signature": "...",
      "url": "https://releases.tempext.com/disk-bloat-scanner/DiskBloatScanner-1.0.0.dmg",
      "size": 45678901
    }
  },
  "notes": "Initial MVP release with production signing"
}
```

---

**PROPOSAL STATUS:** Ready for stakeholder review and EDGS Gate 3 approval