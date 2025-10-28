#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$SCRIPT_DIR/.."
BEADS_JSON="$REPO_ROOT/docs/beads-export.json"
GH="/usr/local/Cellar/gh/2.82.1/bin/gh"

echo "🚀 BEADS → GitHub Issue Sync"
echo "=============================="
echo ""

if ! $GH auth status &>/dev/null; then
  echo "❌ GitHub CLI not authenticated. Run: gh auth login"
  exit 1
fi

if [ ! -f "$BEADS_JSON" ]; then
  echo "📋 Generating BEADS export..."
  node "$SCRIPT_DIR/beads-parser.js"
fi

TOTAL=$(jq '.summary.total' "$BEADS_JSON")
PENDING=$(jq '.summary.byStatus.pending // 0' "$BEADS_JSON")
COMPLETED=$(jq '.summary.byStatus.completed // 0' "$BEADS_JSON")

echo "📊 BEADS Summary:"
echo "   Total: $TOTAL issues"
echo "   Completed: $COMPLETED"
echo "   Pending: $PENDING"
echo ""

read -p "Create GitHub issues for pending BEADS? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "❌ Cancelled"
  exit 0
fi

CREATED=0
SKIPPED=0
ERRORS=0

while IFS= read -r issue; do
  ID=$(echo "$issue" | jq -r '.id')
  TITLE=$(echo "$issue" | jq -r '.title')
  STATUS=$(echo "$issue" | jq -r '.status')
  PRIORITY=$(echo "$issue" | jq -r '.priority')
  EFFORT=$(echo "$issue" | jq -r '.effort')
  IMPACT=$(echo "$issue" | jq -r '.impact')
  DESCRIPTION=$(echo "$issue" | jq -r '.description')
  FILES=$(echo "$issue" | jq -r '.files | join(", ")')
  GITHUB_ISSUE=$(echo "$issue" | jq -r '.githubIssue')
  
  if [ "$STATUS" = "completed" ]; then
    echo "⏭️  Skipping $ID (completed)"
    ((SKIPPED++))
    continue
  fi
  
  if [ "$GITHUB_ISSUE" != "null" ]; then
    echo "⏭️  Skipping $ID (already has GitHub issue #$GITHUB_ISSUE)"
    ((SKIPPED++))
    continue
  fi
  
  echo "📝 Creating issue for $ID: $TITLE"
  
  LABELS=""
  case "$PRIORITY" in
    critical) LABELS="priority: critical,type: bug" ;;
    high) LABELS="priority: high" ;;
    medium) LABELS="priority: medium" ;;
    low) LABELS="priority: low" ;;
  esac
  
  BODY="## BEADS ID: $ID

$DESCRIPTION

### Details
- **Effort:** $EFFORT
- **Impact:** $IMPACT
- **Files:** $FILES

### Tracking
This issue is tracked in \`docs/BEADS_ISSUE_TRACKER.md\`
"
  
  if OUTPUT=$($GH issue create \
    --title "[$ID] $TITLE" \
    --body "$BODY" \
    --label "$LABELS" 2>&1); then
    
    ISSUE_NUM=$(echo "$OUTPUT" | grep -oE '#[0-9]+' | head -1 | tr -d '#')
    echo "   ✅ Created GitHub issue #$ISSUE_NUM"
    
    jq --arg id "$ID" --argjson num "$ISSUE_NUM" \
      '(.issues[] | select(.id == $id) | .githubIssue) = $num' \
      "$BEADS_JSON" > "${BEADS_JSON}.tmp"
    mv "${BEADS_JSON}.tmp" "$BEADS_JSON"
    
    ((CREATED++))
  else
    echo "   ❌ Failed to create issue: $OUTPUT"
    ((ERRORS++))
  fi
  
  sleep 1
  
done < <(jq -c '.issues[]' "$BEADS_JSON")

echo ""
echo "✨ Sync Complete!"
echo "   Created: $CREATED"
echo "   Skipped: $SKIPPED"
echo "   Errors: $ERRORS"

if [ $CREATED -gt 0 ]; then
  echo ""
  echo "📝 Updating BEADS_ISSUE_TRACKER.md with GitHub issue links..."
  node "$SCRIPT_DIR/update-beads-links.js"
fi
