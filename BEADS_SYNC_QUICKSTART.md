# 🚀 BEADS GitHub Sync - Quick Start Guide

**Ready to sync 39 pending BEADS issues to GitHub in under 10 minutes!**

---

## ✅ Prerequisites Check

```bash
# 1. Verify GitHub CLI is available
export PATH="/usr/local/Cellar/gh/2.82.1/bin:$PATH"
gh --version
# Expected: gh version 2.82.1 (2025-10-22)

# 2. Verify parser works
node scripts/beads-parser.js
# Expected: ✅ Exported 45 issues

# 3. Check pending issues count
jq '.summary.byStatus.pending' docs/beads-export.json
# Expected: 39
```

---

## 🔐 Step 1: Authenticate GitHub CLI (2 minutes)

```bash
# Set PATH
export PATH="/usr/local/Cellar/gh/2.82.1/bin:$PATH"

# Authenticate (interactive)
gh auth login

# Follow prompts:
# ? What account do you want to log into? > GitHub.com
# ? What is your preferred protocol for Git operations? > HTTPS
# ? Authenticate Git with your GitHub credentials? > Yes
# ? How would you like to authenticate GitHub CLI? > Login with a web browser

# Copy the one-time code shown, press Enter
# Browser opens → Paste code → Authorize

# Verify authentication
gh auth status
# Expected: ✓ Logged in to github.com as <username>
```

---

## 🏷️ Step 2: Create GitHub Labels (1 minute)

```bash
# Create priority labels
gh label create "priority: critical" --color "d73a4a" --description "Critical issues blocking release"
gh label create "priority: high" --color "ff9800" --description "High priority enhancements"
gh label create "priority: medium" --color "ffc107" --description "Medium priority improvements"
gh label create "priority: low" --color "4caf50" --description "Low priority nice-to-haves"
gh label create "type: bug" --color "d73a4a" --description "Bug or defect"

# Verify labels created
gh label list | grep priority
```

**Note:** If labels already exist, you'll see "already exists" errors - that's fine!

---

## 📤 Step 3: Sync BEADS to GitHub (5 minutes)

```bash
# Review what will be synced
echo "About to sync these pending BEADS:"
jq -r '.issues[] | select(.status == "pending" and .githubIssue == null) | "- \(.id): \(.title) [\(.priority)]"' docs/beads-export.json | head -10

# Run sync script
./scripts/beads-to-github.sh

# Prompts:
# "Create GitHub issues for pending BEADS? (y/N)" → type 'y' and press Enter

# Progress output:
# 📝 Creating issue for BEAD-003: Fix TOCTOU Race Condition...
#    ✅ Created GitHub issue #15
# 📝 Creating issue for BEAD-004: Add Deletion History Logging...
#    ✅ Created GitHub issue #16
# ... (continues for all 39 issues)

# Wait for completion (~3-5 minutes)
```

---

## ✅ Step 4: Verify Sync (1 minute)

```bash
# Check how many issues were created
gh issue list --limit 50 | wc -l

# View first 5 created issues
gh issue list --limit 5

# Verify BEADS was updated with links
grep "**GitHub:**" docs/BEADS_ISSUE_TRACKER.md | head -5

# Check export JSON has GitHub issue numbers
jq '[.issues[] | select(.githubIssue != null)] | length' docs/beads-export.json
# Expected: 39
```

---

## 💾 Step 5: Commit Updates (1 minute)

```bash
# Check what changed
git status
# Expected: docs/beads-export.json, docs/BEADS_ISSUE_TRACKER.md modified

# Review changes
git diff docs/beads-export.json | head -30

# Commit
git add docs/beads-export.json docs/BEADS_ISSUE_TRACKER.md
git commit -m "chore: sync 39 pending BEADS to GitHub issues

- Created GitHub issues for all pending BEADS
- Added GitHub links to BEADS_ISSUE_TRACKER.md
- Updated beads-export.json with issue numbers"

# Push
git push
```

---

## 🔄 Step 6: Test Reverse Sync (Optional)

```bash
# Close a test issue on GitHub
gh issue close 15 --reason "completed" --comment "Testing reverse sync"

# Sync status back to BEADS
./scripts/github-to-beads.sh

# Prompts:
# "Apply these updates? (y/N)" → type 'y'

# Result: BEAD-003 status updated in BEADS_ISSUE_TRACKER.md

# Verify
grep -A2 "BEAD-003" docs/BEADS_ISSUE_TRACKER.md | grep Status
# Expected: **Status:** ✅ COMPLETED (GitHub #15)

# Reopen for testing
gh issue reopen 15 --comment "Reopening - was just testing sync"
```

---

## 🤖 Step 7: Verify Automation (Optional)

```bash
# Check GitHub Actions workflow
gh workflow list
# Expected: "BEADS GitHub Sync" should be listed

# View workflow file
cat .github/workflows/beads-sync.yml | head -20

# Manually trigger workflow (test automation)
gh workflow run "BEADS GitHub Sync"

# Check run status
gh run list --workflow=beads-sync.yml --limit 1

# View logs
gh run view --log
```

---

## 🎉 Success Checklist

After completing all steps, verify:

- [ ] GitHub CLI authenticated (`gh auth status` shows ✓)
- [ ] 39 GitHub issues created with correct labels
- [ ] `beads-export.json` has `githubIssue` numbers
- [ ] `BEADS_ISSUE_TRACKER.md` has `**GitHub:** #XX` links
- [ ] Changes committed to git
- [ ] Reverse sync tested (optional)
- [ ] Automation verified (optional)

---

## 📊 Expected Results

### Before Sync
```json
{
  "total": 45,
  "byStatus": { "completed": 6, "pending": 39 },
  "issues": [
    { "id": "BEAD-003", "githubIssue": null, ... }
  ]
}
```

### After Sync
```json
{
  "total": 45,
  "byStatus": { "completed": 6, "pending": 39 },
  "issues": [
    { "id": "BEAD-003", "githubIssue": 15, ... }
  ]
}
```

### GitHub Issues Created
```
#15: [BEAD-003] Fix TOCTOU Race Condition 🚨 [priority: critical, type: bug]
#16: [BEAD-004] Add Deletion History Logging 🚨 [priority: critical, type: bug]
#17: [BEAD-005] Replace All `.unwrap()` with Proper Error Handling 🚨
...
#53: [BEAD-041] Add Integration Tests ✅ [priority: low]
```

---

## 🐛 Troubleshooting

### "gh: command not found"
```bash
export PATH="/usr/local/Cellar/gh/2.82.1/bin:$PATH"
# Add to ~/.zshrc or ~/.bash_profile to persist
```

### "Not authenticated"
```bash
gh auth login
# Follow interactive prompts
```

### "Label already exists"
This is fine! Skip or use existing labels.

### "API rate limit exceeded"
Wait 1 hour for reset, or run sync script which adds delays between requests.

### Script permission denied
```bash
chmod +x scripts/*.sh
```

---

## 📞 Need Help?

1. **Check status doc:** `docs/BEADS_INTEGRATION_STATUS.md`
2. **Check integration guide:** `docs/BEADS_GITHUB_INTEGRATION.md`
3. **View script comments:** `scripts/beads-parser.js`
4. **Open an issue:** Tag with `type: infrastructure`

---

## 🎯 Next Steps After Sync

1. **Review created issues** on GitHub
2. **Assign issues** to team members
3. **Add to milestones** (Phase 3, v0.2.0, etc.)
4. **Start closing issues** via PRs
5. **Let automation handle** bidirectional sync

---

**⏱️ Total Time: ~10 minutes**

**🚀 Ready? Run:**
```bash
export PATH="/usr/local/Cellar/gh/2.82.1/bin:$PATH"
gh auth login
./scripts/beads-to-github.sh
```

**🎉 That's it! Your BEADS are now synced with GitHub Issues!**
