#!/bin/bash
# Create junk files in current directory for immediate testing
echo "Creating junk files in: $(pwd)"
touch .DS_Store
touch test.pyc
touch test.swp
touch test~
echo "✅ Created 4 junk files in current directory"
echo "Now scan this directory: $(pwd)"
