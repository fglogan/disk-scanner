#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_ROOT="$SCRIPT_DIR/.."
GH="/usr/local/Cellar/gh/2.82.1/bin/gh"

echo "🔄 GitHub → BEADS Status Sync"
echo "=============================="
echo ""

if ! $GH auth status &>/dev/null; then
  echo "❌ GitHub CLI not authenticated. Run: gh auth login"
  exit 1
fi

echo "📥 Fetching GitHub issues..."
ISSUES_JSON=$($GH issue list --limit 200 --json number,title,state,labels,body --jq '.')

TOTAL=$(echo "$ISSUES_JSON" | jq '. | length')
echo "   Found $TOTAL GitHub issues"
echo ""

BEADS_UPDATES=()

while IFS= read -r issue; do
  ISSUE_NUM=$(echo "$issue" | jq -r '.number')
  TITLE=$(echo "$issue" | jq -r '.title')
  STATE=$(echo "$issue" | jq -r '.state')
  BODY=$(echo "$issue" | jq -r '.body')
  
  BEAD_ID=$(echo "$TITLE" | grep -oE '\[BEAD-[A-Z0-9-]+\]' | tr -d '[]' || echo "")
  
  if [ -z "$BEAD_ID" ]; then
    BEAD_ID=$(echo "$BODY" | grep -oE 'BEADS ID: BEAD-[A-Z0-9-]+' | cut -d' ' -f3 || echo "")
  fi
  
  if [ -n "$BEAD_ID" ]; then
    echo "🔗 Found $BEAD_ID → GitHub #$ISSUE_NUM ($STATE)"
    
    BEADS_STATUS="PENDING"
    if [ "$STATE" = "CLOSED" ]; then
      BEADS_STATUS="COMPLETED"
    fi
    
    BEADS_UPDATES+=("$BEAD_ID|$ISSUE_NUM|$BEADS_STATUS")
  fi
  
done < <(echo "$ISSUES_JSON" | jq -c '.[]')

echo ""
echo "📊 Found ${#BEADS_UPDATES[@]} BEADS issues with GitHub links"

if [ ${#BEADS_UPDATES[@]} -eq 0 ]; then
  echo "✅ Nothing to sync"
  exit 0
fi

echo ""
echo "Updates to apply:"
for update in "${BEADS_UPDATES[@]}"; do
  IFS='|' read -r BEAD_ID ISSUE_NUM STATUS <<< "$update"
  echo "   $BEAD_ID: Set to $STATUS (from GitHub #$ISSUE_NUM)"
done

echo ""
read -p "Apply these updates to BEADS_ISSUE_TRACKER.md? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "❌ Cancelled"
  exit 0
fi

UPDATED=0
BEADS_FILE="$REPO_ROOT/docs/BEADS_ISSUE_TRACKER.md"
BEADS_BACKUP="$BEADS_FILE.backup.$(date +%Y%m%d_%H%M%S)"

cp "$BEADS_FILE" "$BEADS_BACKUP"
echo "💾 Created backup: $BEADS_BACKUP"

for update in "${BEADS_UPDATES[@]}"; do
  IFS='|' read -r BEAD_ID ISSUE_NUM STATUS <<< "$update"
  
  if grep -q "### $BEAD_ID:" "$BEADS_FILE"; then
    if [ "$STATUS" = "COMPLETED" ]; then
      sed -i '' "/### $BEAD_ID:/,/^---/ {
        s/\*\*Status:\*\* PENDING/\*\*Status:\*\* ✅ COMPLETED (synced from GitHub #$ISSUE_NUM)/
        s/\*\*Status:\*\* IN PROGRESS/\*\*Status:\*\* ✅ COMPLETED (synced from GitHub #$ISSUE_NUM)/
      }" "$BEADS_FILE"
    fi
    
    if ! grep -A5 "### $BEAD_ID:" "$BEADS_FILE" | grep -q "\*\*GitHub:\*\*"; then
      sed -i '' "/### $BEAD_ID:/,/^---/ {
        /\*\*Status:\*\*/a\\
**GitHub:** #$ISSUE_NUM
      }" "$BEADS_FILE"
    fi
    
    echo "   ✅ Updated $BEAD_ID"
    ((UPDATED++))
  else
    echo "   ⚠️  $BEAD_ID not found in BEADS file"
  fi
done

echo ""
echo "✨ Sync Complete!"
echo "   Updated: $UPDATED BEADS entries"
echo "   Backup: $BEADS_BACKUP"
