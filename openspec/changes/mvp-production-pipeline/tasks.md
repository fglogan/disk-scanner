# TASKS: MVP Production Pipeline

**Change ID:** MVP-PROD-001  
**Generated From:** proposal.md  
**Total Tasks:** 43  
**EDGS Phases:** 3-5  

## Phase 3: Production Infrastructure (Gate 3)

### Domain: BUILD

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P3-T1 | Setup Apple Developer Program account | - | None | Developer ID certificate issued |
| P3-T2 | Generate Developer ID Application certificate | - | P3-T1 | Certificate in Keychain |
| P3-T3 | Create app-specific password for notarization | - | P3-T1 | Password stored in Keychain |
| P3-T4 | Configure Xcode command line tools | - | None | `xcode-select -p` returns valid path |
| P3-T5 | Install create-dmg via Homebrew | - | None | `create-dmg --version` works |

### Domain: SIGN

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P3-T6 | Create entitlements.plist with required permissions | - | None | File exists with correct keys |
| P3-T7 | Configure hardened runtime in tauri.conf.json | - | P3-T6 | Config includes hardenedRuntime: true |
| P3-T8 | Implement code signing in build script | - | P3-T2 | Script signs without errors |
| P3-T9 | Test signing with ad-hoc identity | - | P3-T8 | `codesign -v` validates |
| P3-T10 | Configure notarytool credentials | - | P3-T3 | `xcrun notarytool store-credentials` |

### Domain: DISTRIBUTE

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P3-T11 | Create DMG background image (600x450) | - | None | PNG file at assets/dmg-background.png |
| P3-T12 | Write create-dmg.sh script | - | P3-T5,P3-T11 | Script produces valid DMG |
| P3-T13 | Configure DMG window layout | - | P3-T12 | DMG opens with correct layout |
| P3-T14 | Test DMG installation flow | - | P3-T13 | Drag-to-Applications works |
| P3-T15 | Verify Gatekeeper acceptance | - | P3-T8,P3-T10 | No security warnings on fresh system |

### Gate 3 Validation
```bash
# Automated validation script
#!/bin/bash
set -e

# Check signing
codesign -v --deep "Disk Bloat Scanner.app"
echo "✅ Code signing valid"

# Check notarization
spctl -a -vvv -t install "DiskBloatScanner.dmg"
echo "✅ Notarization valid"

# Check universal binary
lipo -info "Disk Bloat Scanner.app/Contents/MacOS/Disk Bloat Scanner"
echo "✅ Universal binary confirmed"
```

## Phase 4: Feature Completion (Gate 4)

### Critical Priority BEADs

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P4-T1 | Audit code for any production unwrap() usage | BEAD-005 | None | `grep -r "unwrap()" --include="*.rs"` empty |
| P4-T2 | Replace manual Display impl with thiserror | BEAD-006 | P4-T1 | All errors use #[derive(Error)] |

### High Priority BEADs

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P4-T3 | Implement symlink loop detection | BEAD-009 | None | Circular symlinks don't hang |
| P4-T4 | Add large directory warnings (>10K files) | BEAD-010 | None | Warning shown in UI |
| P4-T5 | Detect network mounted drives | BEAD-011 | None | Network paths identified |
| P4-T6 | Add scan cancellation mechanism | BEAD-013 | None | Cancel button stops scan |
| P4-T7 | Enhance progress reporting with ETA | BEAD-014 | P4-T6 | Progress bar shows time remaining |
| P4-T8 | Export scan results to CSV | BEAD-015 | None | Valid CSV file generated |
| P4-T9 | Export scan results to JSON | BEAD-015 | None | Valid JSON file generated |
| P4-T10 | Implement undo for deletions | BEAD-016 | None | Deleted files can be restored |
| P4-T11 | Add custom ignore patterns UI | BEAD-017 | None | .gitignore-style patterns work |
| P4-T12 | Schedule periodic scans | BEAD-018 | None | Scans run automatically |
| P4-T13 | Multi-select with checkboxes | BEAD-019 | None | Shift/Cmd selection works |
| P4-T14 | Global keyboard shortcuts | BEAD-020 | None | Cmd+R refreshes, etc. |

### Medium Priority BEADs

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P4-T15 | Fix dark mode color contrasts | BEAD-021 | None | WCAG AA compliance |
| P4-T16 | Add ARIA labels for screen readers | BEAD-022 | None | VoiceOver navigation works |
| P4-T17 | Implement i18n framework | BEAD-023 | None | Language switching works |
| P4-T18 | Write in-app help documentation | BEAD-024 | None | Help menu populated |
| P4-T19 | Create first-run onboarding flow | BEAD-025 | P4-T18 | New users see tutorial |
| P4-T20 | Add performance metrics collection | BEAD-026 | None | Scan times logged |
| P4-T21 | Implement update notifications | BEAD-028 | None | Update prompt appears |
| P4-T22 | Integrate crash reporting (Sentry) | BEAD-029 | None | Crashes reported to dashboard |
| P4-T23 | Add privacy-respecting analytics | BEAD-030 | None | Opt-in usage stats |
| P4-T24 | Detect Time Machine backups | BEAD-031 | None | Backups marked as such |
| P4-T25 | Handle iCloud Drive files specially | BEAD-032 | None | Cloud files identified |
| P4-T26 | Support external drive scanning | BEAD-033 | None | Mounted volumes appear |
| P4-T27 | Analyze compression potential | BEAD-034 | None | Shows space savings |
| P4-T28 | Generate file type statistics | BEAD-035 | None | Pie chart of file types |

### Low Priority BEADs

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P4-T29 | Add theme customization options | BEAD-036 | None | Color scheme changeable |
| P4-T30 | Advanced filter combinations (AND/OR) | BEAD-037 | None | Complex queries work |
| P4-T31 | Save/load scan profiles | BEAD-038 | None | Profiles persist |
| P4-T32 | Command palette (Cmd+K) | BEAD-039 | None | Quick actions menu |
| P4-T33 | Plugin architecture foundation | BEAD-040 | None | Sample plugin loads |
| P4-T34 | REST API for external tools | BEAD-041 | None | API endpoints respond |
| P4-T35 | CLI companion tool | BEAD-042 | None | `dbs scan /path` works |
| P4-T36 | Comprehensive integration tests | BEAD-043 | All above | 100% scenario coverage |

### Gate 4 Validation
```rust
// Integration test validating all BEADS
#[test]
fn test_all_beads_implemented() {
    // BEAD-005: No unwraps in production
    assert_no_unwraps_in_src();
    
    // BEAD-006: thiserror usage
    assert_uses_thiserror();
    
    // BEAD-009: Symlink loops
    assert_handles_symlink_loops();
    
    // ... test for each BEAD
}
```

## Phase 5: Release Automation (Gate 5)

### Domain: AUTOMATE

| Task ID | Description | BEADS | Dependencies | Validation |
|---------|-------------|-------|--------------|------------|
| P5-T1 | Create .github/workflows/release.yml | - | None | Workflow file valid YAML |
| P5-T2 | Configure GitHub Secrets for signing | - | P3-T2,P3-T3 | Secrets accessible in Actions |
| P5-T3 | Implement version bumping script | - | None | Version updates in 3 files |
| P5-T4 | Generate changelog from commits | - | None | CHANGELOG.md updated |
| P5-T5 | Setup update server infrastructure | - | None | HTTPS endpoint responds |
| P5-T6 | Create update manifest generator | - | P5-T5 | Valid update.json produced |
| P5-T7 | Test end-to-end release pipeline | - | All above | Tag push → DMG published |
| P5-T8 | Document release process | - | P5-T7 | RELEASING.md complete |

### Gate 5 Validation
```yaml
# Automated release test
name: Test Release Pipeline
on: workflow_dispatch

jobs:
  test-release:
    runs-on: macos-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      
      - name: Create test tag
        run: |
          git tag v0.0.0-test
          git push origin v0.0.0-test
      
      - name: Wait for release
        run: |
          sleep 300  # 5 minutes
      
      - name: Verify release
        run: |
          curl -f -I "https://github.com/org/repo/releases/download/v0.0.0-test/DiskBloatScanner-0.0.0-test.dmg"
          echo "✅ Release artifact available"
      
      - name: Cleanup
        if: always()
        run: |
          git push --delete origin v0.0.0-test
```

## Task Execution Framework

### Daily Validation Script
```bash
#!/bin/bash
# scripts/validate-progress.sh

echo "🔍 Validating MVP Production Pipeline Progress"
echo "============================================="

# Phase 3 checks
echo -e "\n📦 Phase 3: Production Infrastructure"
[[ -f "entitlements.plist" ]] && echo "✅ Entitlements configured" || echo "❌ Missing entitlements.plist"
[[ -f "scripts/create-dmg.sh" ]] && echo "✅ DMG script ready" || echo "❌ Missing DMG script"

# Phase 4 checks  
echo -e "\n🚀 Phase 4: Feature Completion"
UNWRAPS=$(grep -r "unwrap()" src-tauri/src --include="*.rs" | grep -v test | wc -l)
[[ $UNWRAPS -eq 0 ]] && echo "✅ No production unwraps" || echo "❌ Found $UNWRAPS unwraps"

# Phase 5 checks
echo -e "\n🤖 Phase 5: Release Automation"
[[ -f ".github/workflows/release.yml" ]] && echo "✅ Release workflow exists" || echo "❌ Missing release workflow"

# Summary
echo -e "\n📊 Summary"
echo "Phase 3: $(find . -name "*.dmg" | wc -l) DMGs built"
echo "Phase 4: $(grep -c "COMPLETED" docs/BEADS_ISSUE_TRACKER.md) BEADs completed"
echo "Phase 5: $(gh release list | wc -l) releases published"
```

## Resource Allocation

### Build Resources
- **CPU**: 4 cores minimum for universal binary compilation
- **RAM**: 8GB minimum for linking
- **Disk**: 10GB free for build artifacts
- **Network**: Required for notarization

### Infrastructure Requirements
- **GitHub Actions**: macOS runners
- **Apple Developer**: Active membership
- **Update Server**: HTTPS with valid certificate
- **Monitoring**: Error tracking for crashes

## Risk Mitigation

### Technical Risks
1. **Notarization failures**: Keep local signing as fallback
2. **Build timeouts**: Cache dependencies
3. **Certificate expiry**: Set calendar reminders
4. **API deprecation**: Pin tool versions

### Process Risks
1. **Manual steps**: Automate everything possible
2. **Knowledge silos**: Document all procedures
3. **Environment drift**: Use consistent toolchain
4. **Release delays**: Build nightly pre-releases

---

**Note**: Tasks are designed to be executed independently within each phase, with clear validation criteria for automated verification.