#!/bin/bash
# Test script to verify junk file patterns

TEST_DIR="/tmp/disk-bloat-test-$$"
mkdir -p "$TEST_DIR"

# Create test junk files
touch "$TEST_DIR/.DS_Store"
touch "$TEST_DIR/Thumbs.db"
touch "$TEST_DIR/desktop.ini"
touch "$TEST_DIR/test.pyc"
touch "$TEST_DIR/test.swp"
echo "backup" > "$TEST_DIR/test~"
touch "$TEST_DIR/test.bak"
touch "$TEST_DIR/file.class"

# Create legitimate files (should NOT be detected)
touch "$TEST_DIR/README.md"
touch "$TEST_DIR/package.json"
touch "$TEST_DIR/.gitignore"

echo "✅ Created test files in $TEST_DIR"
echo "Junk files created: 8"
echo "Legitimate files created: 3"
echo ""
echo "Test directory: $TEST_DIR"
echo "Run a scan on this directory to verify junk detection!"
