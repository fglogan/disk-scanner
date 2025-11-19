#!/bin/bash
# Code signing and notarization script for macOS
# Handles the complete signing workflow for production releases

set -euo pipefail

# Configuration from environment
APPLE_ID="${APPLE_ID:-}"
APPLE_PASSWORD="${APPLE_PASSWORD:-}"
APPLE_TEAM_ID="${APPLE_TEAM_ID:-}"
DEVELOPER_ID="${DEVELOPER_ID:-Developer ID Application}"

# App configuration
VERSION=$(node -p "require('../package.json').version" 2>/dev/null || echo "0.1.1")
APP_NAME="Disk Bloat Scanner"
DMG_NAME="DiskBloatScanner-${VERSION}.dmg"
BUNDLE_ID="com.tempext.disk-bloat-scanner"
APP_PATH="src-tauri/target/universal-apple-darwin/release/bundle/macos/${APP_NAME}.app"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Logging functions
log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

# Error handling
trap 'log_error "Script failed on line $LINENO"' ERR

# Validate environment
validate_environment() {
    log_step "Validating environment..."
    
    if [ -z "$APPLE_ID" ]; then
        log_error "APPLE_ID environment variable not set"
        exit 1
    fi
    
    if [ -z "$APPLE_PASSWORD" ]; then
        log_error "APPLE_PASSWORD environment variable not set"
        exit 1
    fi
    
    if [ -z "$APPLE_TEAM_ID" ]; then
        log_error "APPLE_TEAM_ID environment variable not set"
        exit 1
    fi
    
    # Check for required tools
    for tool in codesign xcrun; do
        if ! command -v $tool &> /dev/null; then
            log_error "$tool not found. Install Xcode command line tools."
            exit 1
        fi
    done
    
    # Check if app exists
    if [ ! -d "$APP_PATH" ]; then
        log_error "App bundle not found at $APP_PATH"
        exit 1
    fi
    
    log_info "✅ Environment validated"
}

# Find signing identity
find_signing_identity() {
    log_step "Finding signing identity..."
    
    # List available identities
    IDENTITIES=$(security find-identity -v -p codesigning | grep "$DEVELOPER_ID" || true)
    
    if [ -z "$IDENTITIES" ]; then
        log_error "No valid signing identity found matching '$DEVELOPER_ID'"
        log_info "Available identities:"
        security find-identity -v -p codesigning
        exit 1
    fi
    
    # Extract the full identity name
    SIGNING_IDENTITY=$(echo "$IDENTITIES" | head -n1 | sed -E 's/.*"(.+)".*/\1/')
    log_info "Using signing identity: $SIGNING_IDENTITY"
}

# Sign the app bundle
sign_app() {
    log_step "Signing app bundle..."
    
    # Remove any existing signatures
    log_info "Removing existing signatures..."
    codesign --remove-signature "$APP_PATH" 2>/dev/null || true
    
    # Sign all nested components first
    log_info "Signing nested components..."
    find "$APP_PATH" -type f -perm +111 -exec codesign --force --options runtime --timestamp --entitlements entitlements.plist --sign "$SIGNING_IDENTITY" {} \; 2>/dev/null || true
    
    # Sign the main app bundle
    log_info "Signing main app bundle..."
    codesign \
        --deep \
        --force \
        --verify \
        --verbose \
        --options runtime \
        --timestamp \
        --entitlements entitlements.plist \
        --sign "$SIGNING_IDENTITY" \
        "$APP_PATH"
    
    # Verify signature
    log_info "Verifying signature..."
    codesign --verify --deep --strict --verbose=2 "$APP_PATH"
    
    # Check signature info
    log_info "Signature info:"
    codesign -dvv "$APP_PATH" 2>&1 | grep -E "(Authority|Timestamp|TeamIdentifier|Identifier)" || true
    
    log_info "✅ App signed successfully"
}

# Create DMG
create_dmg() {
    log_step "Creating DMG..."
    
    # Use the create-dmg script
    if [ -f "scripts/create-dmg.sh" ]; then
        bash scripts/create-dmg.sh
    else
        log_error "create-dmg.sh script not found"
        exit 1
    fi
}

# Sign DMG
sign_dmg() {
    log_step "Signing DMG..."
    
    if [ ! -f "$DMG_NAME" ]; then
        log_error "DMG not found: $DMG_NAME"
        exit 1
    fi
    
    codesign \
        --force \
        --verify \
        --verbose \
        --options runtime \
        --timestamp \
        --sign "$SIGNING_IDENTITY" \
        "$DMG_NAME"
    
    log_info "✅ DMG signed successfully"
}

# Notarize DMG
notarize_dmg() {
    log_step "Notarizing DMG..."
    
    # Submit for notarization
    log_info "Submitting to Apple for notarization..."
    NOTARIZE_OUTPUT=$(xcrun notarytool submit \
        "$DMG_NAME" \
        --apple-id "$APPLE_ID" \
        --password "$APPLE_PASSWORD" \
        --team-id "$APPLE_TEAM_ID" \
        --wait \
        2>&1)
    
    echo "$NOTARIZE_OUTPUT"
    
    # Check if submission was successful
    if echo "$NOTARIZE_OUTPUT" | grep -q "Successfully received"; then
        # Extract submission ID
        SUBMISSION_ID=$(echo "$NOTARIZE_OUTPUT" | grep -E "id: [a-f0-9\-]+" | head -1 | awk '{print $2}')
        log_info "Submission ID: $SUBMISSION_ID"
        
        # Get notarization info
        log_info "Getting notarization status..."
        xcrun notarytool info \
            "$SUBMISSION_ID" \
            --apple-id "$APPLE_ID" \
            --password "$APPLE_PASSWORD" \
            --team-id "$APPLE_TEAM_ID"
        
        # Check final status
        if echo "$NOTARIZE_OUTPUT" | grep -q "status: Accepted"; then
            log_info "✅ Notarization successful!"
        else
            log_error "Notarization failed or is still processing"
            
            # Try to get the log
            log_info "Attempting to retrieve notarization log..."
            xcrun notarytool log \
                "$SUBMISSION_ID" \
                --apple-id "$APPLE_ID" \
                --password "$APPLE_PASSWORD" \
                --team-id "$APPLE_TEAM_ID" \
                notarization-log.json || true
            
            if [ -f "notarization-log.json" ]; then
                log_error "Check notarization-log.json for details"
            fi
            exit 1
        fi
    else
        log_error "Failed to submit for notarization"
        exit 1
    fi
}

# Staple notarization
staple_dmg() {
    log_step "Stapling notarization ticket..."
    
    xcrun stapler staple "$DMG_NAME"
    
    # Verify stapling
    log_info "Verifying stapled ticket..."
    xcrun stapler validate "$DMG_NAME"
    
    log_info "✅ Notarization ticket stapled successfully"
}

# Final verification
verify_final() {
    log_step "Final verification..."
    
    # Check signature
    codesign -vvv --deep --strict "$DMG_NAME"
    
    # Check notarization
    spctl -a -vvv -t install "$DMG_NAME"
    
    log_info "✅ All checks passed!"
}

# Main execution
main() {
    log_info "🚀 Starting code signing and notarization process..."
    log_info "App: $APP_NAME v$VERSION"
    
    validate_environment
    find_signing_identity
    sign_app
    create_dmg
    sign_dmg
    notarize_dmg
    staple_dmg
    verify_final
    
    log_info "🎉 Code signing and notarization complete!"
    log_info "✅ Production DMG ready: $DMG_NAME"
    
    # Display final file info
    log_info "DMG Details:"
    ls -lh "$DMG_NAME"
    shasum -a 256 "$DMG_NAME"
}

# Run main function
main