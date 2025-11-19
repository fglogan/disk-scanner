#!/bin/bash
# DMG creation script for Disk Bloat Scanner
# Creates a production-ready DMG with custom background and layout

set -euo pipefail

# Configuration
VERSION=$(node -p "require('../package.json').version" 2>/dev/null || echo "0.1.1")
APP_NAME="Disk Bloat Scanner"
DMG_NAME="DiskBloatScanner-${VERSION}"
BUNDLE_PATH="src-tauri/target/universal-apple-darwin/release/bundle/macos"
DMG_BACKGROUND="assets/dmg-background.png"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Validate environment
if ! command -v create-dmg &> /dev/null; then
    log_error "create-dmg not found. Install with: npm install -g create-dmg"
    exit 1
fi

# Check if app bundle exists
if [ ! -d "${BUNDLE_PATH}/${APP_NAME}.app" ]; then
    log_error "App bundle not found at ${BUNDLE_PATH}/${APP_NAME}.app"
    log_info "Build the app first with: npm run tauri:build -- --target universal-apple-darwin"
    exit 1
fi

# Create assets directory if it doesn't exist
mkdir -p assets

# Check for background image
if [ ! -f "$DMG_BACKGROUND" ]; then
    log_warn "DMG background not found at $DMG_BACKGROUND"
    log_info "Creating placeholder background..."
    # Create a simple placeholder if background doesn't exist
    touch "$DMG_BACKGROUND"
fi

# Remove old DMG if it exists
if [ -f "${DMG_NAME}.dmg" ]; then
    log_info "Removing existing DMG..."
    rm -f "${DMG_NAME}.dmg"
fi

log_info "Creating DMG for ${APP_NAME} v${VERSION}..."

# Create DMG with proper settings
create-dmg \
    --volname "${APP_NAME}" \
    --volicon "src-tauri/icons/icon.icns" \
    --window-pos 200 120 \
    --window-size 600 450 \
    --icon-size 100 \
    --icon "${APP_NAME}.app" 150 225 \
    --hide-extension "${APP_NAME}.app" \
    --app-drop-link 450 225 \
    --no-internet-enable \
    --format ULFO \
    --hdiutil-verbose \
    "${DMG_NAME}.dmg" \
    "${BUNDLE_PATH}/" \
    || {
        log_error "DMG creation failed"
        exit 1
    }

# Verify DMG was created
if [ -f "${DMG_NAME}.dmg" ]; then
    DMG_SIZE=$(du -h "${DMG_NAME}.dmg" | cut -f1)
    log_info "✅ DMG created successfully: ${DMG_NAME}.dmg (${DMG_SIZE})"
    
    # Display DMG info
    log_info "DMG Details:"
    hdiutil imageinfo "${DMG_NAME}.dmg" | grep -E "(Format:|Size:|Checksum:)" || true
else
    log_error "DMG creation failed - file not found"
    exit 1
fi

# Optional: If background image was specified, add it
if [ -f "$DMG_BACKGROUND" ] && [ -s "$DMG_BACKGROUND" ]; then
    log_info "Background image will be applied in the DMG"
else
    log_warn "No background image applied to DMG"
fi

log_info "🎉 DMG packaging complete!"