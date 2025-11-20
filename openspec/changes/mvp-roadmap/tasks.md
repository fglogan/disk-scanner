# MVP Roadmap Tasks

**Generated from:** proposal.md  
**Total Tasks:** 43 (35 BEADS + 8 Infrastructure)  
**Milestones:** 5  
**Timeline:** 7 weeks to MVP

## Task Breakdown by Milestone

### Milestone 1: Critical Security & Stability (Week 1)

| ID | Task | Type | Effort | Status | Dependencies |
|----|------|------|--------|--------|--------------|
| M1-T1 | BEAD-005: Replace .unwrap() with proper error handling | Fix | 4h | COMPLETE | None |
| M1-T2 | BEAD-006: Implement proper error types with thiserror | Fix | 3h | COMPLETE | M1-T1 |
| M1-T3 | Configure code signing for macOS | Setup | 2h | PENDING | None |
| M1-T4 | Setup notarization workflow | Setup | 3h | PENDING | M1-T3 |
| M1-T5 | Create DMG release automation | Setup | 4h | PENDING | M1-T4 |
| M1-T6 | Update version to 0.2.0 | Release | 30m | PENDING | M1-T1,M1-T2 |
| M1-T7 | Build and test signed DMG | Release | 2h | PENDING | All above |

**Milestone 1 Output**: `DiskBloatScanner-0.2.0.dmg` (signed & notarized)

### Milestone 2: High Priority Fixes (Weeks 2-3)

| ID | Task | Type | Effort | Status | Dependencies |
|----|------|------|--------|--------|--------------|
| M2-T1 | BEAD-009: Async dir_size (UI freeze fix) | Feature | 3h | COMPLETE | None |
| M2-T2 | BEAD-010: Scan cancellation | Feature | 2h | COMPLETE | None |
| M2-T3 | BEAD-011: Real progress tracking | Feature | 4h | COMPLETE | None |
| M2-T4 | BEAD-013: Error boundaries | Feature | 5h | COMPLETE | None |
| M2-T5 | BEAD-014: Retry logic | Feature | 4h | COMPLETE | M2-T4 |
| M2-T6 | BEAD-015: Export functionality (CSV/JSON) | Feature | 6h | PENDING | None |
| M2-T7 | BEAD-016: Undo/restore capability | Feature | 8h | PENDING | None |
| M2-T8 | BEAD-017: Custom ignore patterns | Feature | 4h | PENDING | None |
| M2-T9 | BEAD-018: Scheduled scans | Feature | 6h | PENDING | None |
| M2-T10 | BEAD-019: Multi-select operations | UI | 3h | PENDING | None |
| M2-T11 | BEAD-020: Keyboard shortcuts | UI | 2h | PENDING | None |
| M2-T12 | Update version to 0.3.0 | Release | 30m | PENDING | All above |
| M2-T13 | Build and test DMG | Release | 1h | PENDING | M2-T12 |

**Milestone 2 Output**: `DiskBloatScanner-0.3.0.dmg` (stable beta)

### Milestone 3: Medium Priority & Polish (Weeks 4-5)

| ID | Task | Type | Effort | Status | Dependencies |
|----|------|------|--------|--------|--------------|
| M3-T1 | BEAD-021: Dark mode improvements | UI | 3h | PENDING | None |
| M3-T2 | BEAD-022: Accessibility (ARIA labels) | UI | 4h | PENDING | None |
| M3-T3 | BEAD-023: Localization framework | Feature | 6h | PENDING | None |
| M3-T4 | BEAD-024: Help documentation | Docs | 4h | PENDING | None |
| M3-T5 | BEAD-025: Onboarding tutorial | UI | 5h | PENDING | M3-T4 |
| M3-T6 | BEAD-026: Performance monitoring | Feature | 4h | PENDING | None |
| M3-T7 | BEAD-028: Update notifications | Feature | 5h | PENDING | None |
| M3-T8 | BEAD-029: Crash reporting | Feature | 6h | PENDING | None |
| M3-T9 | BEAD-030: Analytics (privacy-respecting) | Feature | 5h | PENDING | None |
| M3-T10 | BEAD-031: Backup detection | Feature | 3h | PENDING | None |
| M3-T11 | BEAD-032: Cloud storage handling | Feature | 4h | PENDING | None |
| M3-T12 | BEAD-033: External drive support | Feature | 3h | PENDING | None |
| M3-T13 | BEAD-034: Compression analysis | Feature | 4h | PENDING | None |
| M3-T14 | BEAD-035: File type statistics | Feature | 3h | PENDING | None |
| M3-T15 | Update version to 0.4.0 | Release | 30m | PENDING | All above |
| M3-T16 | Build and test DMG | Release | 1h | PENDING | M3-T15 |

**Milestone 3 Output**: `DiskBloatScanner-0.4.0.dmg` (release candidate)

### Milestone 4: Low Priority & Final Polish (Week 6)

| ID | Task | Type | Effort | Status | Dependencies |
|----|------|------|--------|--------|--------------|
| M4-T1 | BEAD-036: Theme customization | Feature | 4h | PENDING | None |
| M4-T2 | BEAD-037: Advanced filters | Feature | 3h | PENDING | None |
| M4-T3 | BEAD-038: Scan profiles | Feature | 4h | PENDING | None |
| M4-T4 | BEAD-039: Command palette | Feature | 5h | PENDING | None |
| M4-T5 | BEAD-040: Plugin system | Feature | 8h | PENDING | None |
| M4-T6 | BEAD-041: REST API | Feature | 6h | PENDING | None |
| M4-T7 | BEAD-042: CLI companion | Feature | 4h | PENDING | None |
| M4-T8 | BEAD-043: Integration tests | Test | 6h | PENDING | None |
| M4-T9 | Update version to 0.5.0 | Release | 30m | PENDING | All above |
| M4-T10 | Build and test DMG | Release | 1h | PENDING | M4-T9 |

**Milestone 4 Output**: `DiskBloatScanner-0.5.0.dmg` (pre-release)

### Milestone 5: MVP Release (Week 7)

| ID | Task | Type | Effort | Status | Dependencies |
|----|------|------|--------|--------|--------------|
| M5-T1 | Full regression testing | QA | 8h | PENDING | None |
| M5-T2 | Performance benchmarking | QA | 4h | PENDING | None |
| M5-T3 | Security audit | QA | 6h | PENDING | None |
| M5-T4 | Accessibility review | QA | 4h | PENDING | None |
| M5-T5 | Write user manual | Docs | 6h | PENDING | None |
| M5-T6 | Create FAQ and troubleshooting guide | Docs | 4h | PENDING | M5-T5 |
| M5-T7 | Setup auto-update server | Infra | 4h | PENDING | None |
| M5-T8 | Prepare website and download page | Marketing | 6h | PENDING | None |
| M5-T9 | Create demo video and screenshots | Marketing | 4h | PENDING | None |
| M5-T10 | Update version to 1.0.0 | Release | 30m | PENDING | M5-T1-T4 |
| M5-T11 | Final production build | Release | 2h | PENDING | All above |
| M5-T12 | Distribution and announcement | Release | 2h | PENDING | M5-T11 |

**Milestone 5 Output**: `DiskBloatScanner-1.0.0.dmg` (MVP release)

## Build Commands for Each Milestone

### Development Build
```bash
# Run tests
cargo test --lib
cargo test --test integration_tests
npm test

# Dev build with hot reload
npm run tauri:dev
```

### Production DMG Build
```bash
# 1. Clean previous builds
rm -rf src-tauri/target/release/bundle

# 2. Update version in:
# - Cargo.toml
# - package.json
# - tauri.conf.json

# 3. Build release
npm run tauri:build

# 4. Sign the app
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name (TEAM_ID)" \
  "src-tauri/target/release/bundle/macos/Disk Bloat Scanner.app"

# 5. Create DMG
create-dmg \
  --volname "Disk Bloat Scanner" \
  --volicon "src-tauri/icons/icon.icns" \
  --background "assets/dmg-background.png" \
  --window-pos 200 120 \
  --window-size 600 400 \
  --icon-size 100 \
  --icon "Disk Bloat Scanner.app" 175 190 \
  --hide-extension "Disk Bloat Scanner.app" \
  --app-drop-link 425 190 \
  "DiskBloatScanner-v$(cat package.json | jq -r .version).dmg" \
  "src-tauri/target/release/bundle/macos/"

# 6. Notarize DMG
xcrun altool --notarize-app \
  --primary-bundle-id "com.tempext.disk-bloat-scanner" \
  --username "your-apple-id@example.com" \
  --password "@keychain:AC_PASSWORD" \
  --file "DiskBloatScanner-v$(cat package.json | jq -r .version).dmg"

# 7. Wait for notarization (check email or use)
xcrun altool --notarization-info <REQUEST_UUID> \
  --username "your-apple-id@example.com" \
  --password "@keychain:AC_PASSWORD"

# 8. Staple the notarization
xcrun stapler staple "DiskBloatScanner-v$(cat package.json | jq -r .version).dmg"

# 9. Verify
spctl -a -vvv -t install "DiskBloatScanner-v$(cat package.json | jq -r .version).dmg"
```

## Task Dependencies Graph

```
Milestone 1 (Critical)
    ├── BEAD-005 → BEAD-006 → Version Update
    └── Code Signing → Notarization → DMG Automation → Build

Milestone 2 (High Priority) 
    ├── Independent BEADs (009-020) → Version Update → Build
    └── BEAD-013 → BEAD-014 (cancellation → progress)

Milestone 3 (Medium Priority)
    ├── Independent BEADs (021-035) → Version Update → Build
    └── BEAD-024 → BEAD-025 (help docs → tutorial)

Milestone 4 (Low Priority)
    └── Independent BEADs (036-043) → Version Update → Build

Milestone 5 (MVP)
    └── QA (T1-T4) → Docs (T5-T6) → Infrastructure (T7) → Release
```

## Resource Requirements

### Development Team
- **Lead Developer**: 40h/week across all milestones
- **QA Tester**: 20h/week starting Milestone 2
- **Technical Writer**: 10h/week for Milestones 3-5
- **Designer**: 5h/week for UI improvements

### Infrastructure
- **Apple Developer Account**: $99/year
- **Code Signing Certificate**: Included with developer account
- **Build Machine**: macOS with Xcode
- **Distribution**: GitHub Releases initially
- **Update Server**: Simple HTTPS server for Sparkle/Tauri updater

## Risk Management

### High Risk Items
1. **Notarization delays**: Apple can take 24-48h
2. **Cross-platform testing**: Need multiple macOS versions
3. **Auto-update mechanism**: Critical for post-launch fixes

### Mitigation Strategies
1. **Early notarization**: Submit test builds in Milestone 1
2. **Beta testing group**: Recruit 10-20 users by Milestone 3
3. **Rollback plan**: Keep previous versions available

## Success Criteria

Each milestone must meet:
- All tests passing (100%)
- No critical bugs
- DMG builds and installs cleanly
- Notarization approved
- Manual QA checklist passed

MVP (1.0.0) additional criteria:
- 48 hours without critical bugs
- Beta tester approval (80%+)
- Documentation complete
- Marketing materials ready