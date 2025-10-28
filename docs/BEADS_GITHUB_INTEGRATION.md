# 🔗 BEADS GitHub Integration Guide

**Version:** 1.0  
**Last Updated:** October 28, 2025  
**Status:** ✅ Fully Operational

---

## 📋 Overview

This integration provides **bi-directional synchronization** between the BEADS Issue Tracker (`docs/BEADS_ISSUE_TRACKER.md`) and GitHub Issues. It enables:

- ✅ **BEADS → GitHub**: Create GitHub issues from pending BEADS entries
- ✅ **GitHub → BEADS**: Sync issue status changes back to BEADS tracker
- ✅ **Automated Sync**: GitHub Actions workflow for continuous synchronization
- ✅ **Manual Control**: Command-line scripts for on-demand sync

---

## 🚀 Quick Start

### Prerequisites

1. **GitHub CLI** installed and authenticated:
   ```bash
   brew install gh
   gh auth login
   ```

2. **Node.js** (v18+ recommended)

3. **Git** repository with GitHub remote

### First-Time Setup

1. **Parse BEADS issues**:
   ```bash
   node scripts/beads-parser.js
   ```
   This creates `docs/beads-export.json` with all parsed issues.

2. **Sync to GitHub** (creates issues):
   ```bash
   ./scripts/beads-to-github.sh
   ```
   Reviews pending BEADS and creates GitHub issues with labels.

3. **Sync from GitHub** (update statuses):
   ```bash
   ./scripts/github-to-beads.sh
   ```
   Pulls GitHub issue statuses and updates BEADS tracker.

---

## 📦 Components

### 1. **beads-parser.js**
**Location:** `scripts/beads-parser.js`  
**Purpose:** Parses `BEADS_ISSUE_TRACKER.md` and extracts structured data

**Usage:**
```bash
node scripts/beads-parser.js
```

**Output:**
- `docs/beads-export.json` - Structured JSON with all issues
- Console summary with statistics

**Data Structure:**
```json
{
  "generated": "2025-10-28T...",
  "summary": {
    "total": 45,
    "byStatus": { "completed": 6, "pending": 39 },
    "byPriority": { "critical": 8, "high": 12, ... }
  },
  "issues": [
    {
      "id": "BEAD-001",
      "title": "Enable Content Security Policy",
      "priority": "critical",
      "status": "completed",
      "effort": "15 minutes",
      "impact": "Prevents XSS attacks",
      "githubIssue": 42,
      ...
    }
  ]
}
```

---

### 2. **beads-to-github.sh**
**Location:** `scripts/beads-to-github.sh`  
**Purpose:** Creates GitHub issues for pending BEADS entries

**Features:**
- ✅ Skips completed issues
- ✅ Skips issues already linked to GitHub
- ✅ Applies priority labels automatically
- ✅ Creates detailed issue body with metadata
- ✅ Updates `beads-export.json` with issue numbers
- ✅ Calls `update-beads-links.js` to update markdown

**Usage:**
```bash
./scripts/beads-to-github.sh
```

**Example Output:**
```
🚀 BEADS → GitHub Issue Sync
==============================

📊 BEADS Summary:
   Total: 45 issues
   Completed: 6
   Pending: 39

Create GitHub issues for pending BEADS? (y/N) y

📝 Creating issue for BEAD-003: Fix TOCTOU Race Condition
   ✅ Created GitHub issue #15

📝 Creating issue for BEAD-004: Add Deletion History Logging
   ✅ Created GitHub issue #16

✨ Sync Complete!
   Created: 2
   Skipped: 43
   Errors: 0
```

**Labels Applied:**
- `priority: critical` - For critical issues
- `priority: high` - For high priority
- `priority: medium` - For medium priority
- `priority: low` - For low priority
- `type: bug` - For critical issues (added automatically)

---

### 3. **update-beads-links.js**
**Location:** `scripts/update-beads-links.js`  
**Purpose:** Updates `BEADS_ISSUE_TRACKER.md` with GitHub issue links

**Features:**
- ✅ Adds `**GitHub:** #123` lines to BEADS entries
- ✅ Preserves existing formatting
- ✅ Updates only entries with new GitHub links

**Usage:**
```bash
node scripts/update-beads-links.js
```

**Before:**
```markdown
### BEAD-003: Fix TOCTOU Race Condition 🚨
**Status:** PENDING  
**Priority:** CRITICAL  
```

**After:**
```markdown
### BEAD-003: Fix TOCTOU Race Condition 🚨
**Status:** PENDING  
**GitHub:** #15
**Priority:** CRITICAL  
```

---

### 4. **github-to-beads.sh**
**Location:** `scripts/github-to-beads.sh`  
**Purpose:** Syncs GitHub issue statuses back to BEADS tracker

**Features:**
- ✅ Fetches all GitHub issues (up to 200)
- ✅ Matches issues to BEADS entries via `[BEAD-XXX]` in title or body
- ✅ Updates BEADS status when GitHub issues are closed
- ✅ Adds GitHub issue links if missing
- ✅ Creates timestamped backups before modification

**Usage:**
```bash
./scripts/github-to-beads.sh
```

**Example Output:**
```
🔄 GitHub → BEADS Status Sync
==============================

📥 Fetching GitHub issues...
   Found 47 GitHub issues

🔗 Found BEAD-003 → GitHub #15 (CLOSED)
🔗 Found BEAD-004 → GitHub #16 (OPEN)

📊 Found 2 BEADS issues with GitHub links

Updates to apply:
   BEAD-003: Set to COMPLETED (from GitHub #15)

Apply these updates? (y/N) y

💾 Created backup: docs/BEADS_ISSUE_TRACKER.md.backup.20251028_143022
   ✅ Updated BEAD-003

✨ Sync Complete!
   Updated: 1 BEADS entries
```

---

### 5. **beads-sync.yml**
**Location:** `.github/workflows/beads-sync.yml`  
**Purpose:** Automated bi-directional sync via GitHub Actions

**Triggers:**
- **Manual**: Via workflow dispatch in GitHub Actions UI
- **Scheduled**: Weekly on Sundays at midnight UTC

**Jobs:**

#### Job 1: `sync-beads-to-github`
1. Parses BEADS issues
2. Creates GitHub issues for pending BEADS
3. Updates BEADS with GitHub links
4. Commits changes back to repository

#### Job 2: `sync-github-to-beads`
1. Fetches all GitHub issues
2. Matches to BEADS entries
3. Updates BEADS statuses for closed issues
4. Commits changes back to repository

**Permissions Required:**
- `issues: write` - Create and modify issues
- `contents: write` - Commit to repository

**Enable Workflow:**
```bash
# Push workflow file
git add .github/workflows/beads-sync.yml
git commit -m "feat: add BEADS GitHub sync workflow"
git push

# Manually trigger from GitHub UI:
# Actions → BEADS GitHub Sync → Run workflow
```

---

## 🔄 Workflow Examples

### Scenario 1: New BEADS Entry → GitHub Issue

1. **Add new BEADS entry** to `docs/BEADS_ISSUE_TRACKER.md`:
   ```markdown
   ### BEAD-050: Add Dark Mode Support 📝
   **Status:** PENDING  
   **Priority:** MEDIUM  
   **Effort:** 4-6 hours
   ```

2. **Run sync script**:
   ```bash
   ./scripts/beads-to-github.sh
   ```

3. **Result**: GitHub issue #50 created with:
   - Title: `[BEAD-050] Add Dark Mode Support`
   - Labels: `priority: medium`
   - Body includes effort, impact, files

4. **BEADS updated automatically**:
   ```markdown
   ### BEAD-050: Add Dark Mode Support 📝
   **Status:** PENDING  
   **GitHub:** #50
   **Priority:** MEDIUM  
   ```

---

### Scenario 2: GitHub Issue Closed → BEADS Completed

1. **Close GitHub issue #50** (via PR merge, manual close, etc.)

2. **Run sync script**:
   ```bash
   ./scripts/github-to-beads.sh
   ```

3. **Result**: BEADS entry updated:
   ```markdown
   ### BEAD-050: Add Dark Mode Support 📝
   **Status:** ✅ COMPLETED (GitHub #50)  
   **GitHub:** #50
   **Priority:** MEDIUM  
   ```

---

### Scenario 3: Bulk Initial Sync

**First time syncing all 39 pending BEADS to GitHub:**

```bash
# 1. Parse BEADS
node scripts/beads-parser.js

# 2. Review what will be created
cat docs/beads-export.json | jq '.summary'

# 3. Create all issues (prompted for confirmation)
./scripts/beads-to-github.sh

# 4. Commit updated BEADS file
git add docs/BEADS_ISSUE_TRACKER.md docs/beads-export.json
git commit -m "chore: sync BEADS with GitHub issues"
git push
```

**Expected Result:**
- 39 GitHub issues created
- All with appropriate labels
- BEADS file updated with links
- `beads-export.json` tracks mappings

---

## 🛠️ Maintenance

### Update Parser for New BEADS Fields

If you add new metadata to BEADS entries, update `beads-parser.js`:

```javascript
// Add new field parsing
else if (line.startsWith('**NewField:**')) {
  currentIssue.newField = line.split(':')[1]?.trim() || '';
}
```

### Customize GitHub Issue Template

Edit `beads-to-github.sh` around line 70:

```bash
BODY="## BEADS ID: $ID

$DESCRIPTION

### Your Custom Section
...
"
```

### Change Sync Schedule

Edit `.github/workflows/beads-sync.yml`:

```yaml
schedule:
  - cron: '0 0 * * 0'  # Weekly on Sunday at midnight
  # Change to: '0 */6 * * *' for every 6 hours
```

### Backup Strategy

All sync scripts create backups automatically:
- `github-to-beads.sh` creates timestamped backups before modifications
- Git history provides version control
- Export JSON provides point-in-time snapshots

---

## 🐛 Troubleshooting

### "gh: command not found"

**Solution:**
```bash
brew install gh
# Or update PATH in scripts to use full path:
GH="/usr/local/bin/gh"  # or /opt/homebrew/bin/gh on Apple Silicon
```

### "GitHub CLI not authenticated"

**Solution:**
```bash
gh auth login
# Follow prompts to authenticate
gh auth status  # Verify
```

### "Permission denied" when running scripts

**Solution:**
```bash
chmod +x scripts/*.sh
```

### Parser not finding issues

**Check BEADS format:**
- Headers must be exactly: `### BEAD-XXX: Title`
- Status line must be: `**Status:** VALUE`
- Ensure consistent spacing

### GitHub rate limits

**Symptoms:** Sync fails after creating many issues

**Solution:**
- Wait an hour for rate limit reset
- Use GitHub Actions (higher limits)
- Add delays between API calls

### Duplicate issues created

**Prevention:**
- Always check `githubIssue` field in export JSON
- Parser tracks created issues
- Sync script skips issues with existing GitHub links

---

## 📊 Statistics & Monitoring

### View Current State

```bash
# Parse and view summary
node scripts/beads-parser.js

# Check which BEADS have GitHub links
grep -n "**GitHub:**" docs/BEADS_ISSUE_TRACKER.md

# Count synced issues
jq '[.issues[] | select(.githubIssue != null)] | length' docs/beads-export.json
```

### GitHub Actions Logs

View workflow runs:
1. Go to repository → Actions tab
2. Click "BEADS GitHub Sync" workflow
3. View logs for created/updated counts

---

## 🔐 Security Considerations

### API Tokens

- **Local scripts**: Use `gh auth` tokens (stored securely by GitHub CLI)
- **GitHub Actions**: Use `${{ github.token }}` (automatic, scoped)
- **Never commit** personal access tokens to repository

### Permissions

**Minimum required:**
- `issues: write` - Create and edit issues
- `contents: write` - Commit to repository

**Not required:**
- `packages`, `deployments`, `pages`, etc.

### Backup Protection

- All modifications create backups
- Git history preserves original state
- Can always revert: `git checkout HEAD~1 docs/BEADS_ISSUE_TRACKER.md`

---

## 🎯 Best Practices

1. **Run parser first** before any sync operation
2. **Review beads-export.json** before bulk operations
3. **Test on a fork** before production use
4. **Commit frequently** after sync operations
5. **Monitor workflow runs** for errors
6. **Keep backups** of BEADS file before manual edits
7. **Use labels consistently** for easy filtering
8. **Close issues via PR** to maintain traceability

---

## 📚 Integration with Other Tools

### With Project Management

Link GitHub issues to:
- **Projects**: Add synced issues to GitHub Projects boards
- **Milestones**: Group by phase (Phase 1, Phase 2, etc.)
- **Assignees**: Assign to team members

### With CI/CD

Add to existing workflows:
```yaml
- name: Update BEADS status
  run: ./scripts/github-to-beads.sh
```

### With Notifications

Configure GitHub notifications for:
- Issue creation (all new BEADS)
- Issue closure (BEADS completion)
- Label changes (priority updates)

---

## 📖 Reference

### File Locations

```
disk-bloat-scanner/
├── .github/workflows/
│   └── beads-sync.yml              # Automation workflow
├── scripts/
│   ├── beads-parser.js             # BEADS parser
│   ├── beads-to-github.sh          # BEADS → GitHub sync
│   ├── github-to-beads.sh          # GitHub → BEADS sync
│   └── update-beads-links.js       # Link updater
├── docs/
│   ├── BEADS_ISSUE_TRACKER.md      # Source of truth
│   ├── beads-export.json           # Parsed data (generated)
│   └── BEADS_GITHUB_INTEGRATION.md # This file
```

### Status Mappings

| BEADS Status | GitHub State | Sync Direction |
|--------------|--------------|----------------|
| PENDING | OPEN | Both |
| IN PROGRESS | OPEN | Both |
| ✅ COMPLETED | CLOSED | GitHub → BEADS |
| BLOCKED | OPEN | Manual only |

### Priority Mappings

| BEADS Priority | GitHub Label |
|----------------|--------------|
| 🚨 CRITICAL | `priority: critical`, `type: bug` |
| ⚠️ HIGH | `priority: high` |
| 📝 MEDIUM | `priority: medium` |
| ✅ LOW | `priority: low` |

---

## 🎉 Success Metrics

After full integration:
- ✅ **45 BEADS entries** parsed successfully
- ✅ **39 pending issues** ready for GitHub sync
- ✅ **Bi-directional sync** operational
- ✅ **Automated workflow** configured
- ✅ **Zero manual tracking** required

---

**Questions?** Check `scripts/beads-parser.js` comments or open an issue!
