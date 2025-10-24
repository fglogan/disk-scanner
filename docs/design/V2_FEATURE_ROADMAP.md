# V2.0 Feature Roadmap - Disk Bloat Scanner

**Status**: Planning / Not Yet Implemented  
**Target**: V2.0 Release  
**Created**: October 24, 2025

---

## Overview

This document outlines planned features for V2.0, focusing on intelligent bloat detection, repository health monitoring, and Tempext Genesis compliance checking.

---

## A. Intelligent Bloat Detection

### 1. AI Snippet Bloat Detection

**Problem**: Small AI-generated files (8KB+) clutter repositories with summaries, snippets, and duplicate content.

**Solution**: Fuzzy-rule detection system for AI-generated clutter.

#### Detection Rules

**File Size Threshold**
- Minimum: 8 KB (catches AI summaries without flagging tiny configs)
- Maximum: 100 KB (typical for AI snippets before they become "large files")

**Pattern Matching**
```
AI Artifact Indicators:
- Filenames: *_SUMMARY.md, *_OVERVIEW.md, QUICK_*.md, FINAL_*.md
- Filenames: IMPLEMENTATION_*.md, STARTUP_*.md, SUCCESS.md, STATUS_*.md
- Content patterns: "Here's a summary...", "As an AI...", "I've generated..."
- Duplicate sections across multiple files
- Markdown with minimal unique content
- Timestamp-named files: 2025-10-24_summary.md
```

**Fuzzy Rules**
```python
def is_ai_snippet(file):
    score = 0
    
    # Filename patterns (+3 each)
    if matches("*_SUMMARY.md|*_OVERVIEW.md|QUICK_*.md"): score += 3
    if matches("FINAL_*|IMPLEMENTATION_*|STATUS_*"): score += 3
    
    # Size range (+2)
    if 8KB <= size <= 100KB: score += 2
    
    # Content analysis (+1 each)
    if contains("Here's a|I've|As an AI"): score += 1
    if has_duplicate_content_with_other_files(): score += 2
    if markdown_sections_match_pattern(): score += 1
    if minimal_unique_content(): score += 1
    
    # Decision
    return "ARCHIVE" if score >= 7 else "CHECK" if score >= 4 else "KEEP"
```

#### User Actions

When AI snippets detected, offer:

1. **Archive to `.archive/ai-snippets/`**
   - Preserves files but removes from active tree
   - Organized by date: `.archive/ai-snippets/2025-10-24/`
   - Maintains git history

2. **Consolidate to Single File**
   - Merge related snippets into `docs/history/ai-summaries.md`
   - Add metadata headers (date, source, context)
   - Delete originals after consolidation

3. **Smart Delete**
   - If content exists elsewhere (in official docs)
   - If purely AI-generated with no user edits
   - If superseded by newer official documentation

4. **Review & Keep**
   - User manually reviews flagged files
   - Mark as "intentional" to exclude from future scans
   - Add to `.bloatignore` file

#### Configuration

```json
{
  "ai_snippet_detection": {
    "enabled": true,
    "min_size_kb": 8,
    "max_size_kb": 100,
    "default_action": "archive",
    "archive_path": ".archive/ai-snippets/",
    "consolidate_path": "docs/history/ai-summaries.md",
    "ignore_patterns": [".bloatignore"]
  }
}
```

---

## B. Repository Health & AI Drift Detection

### 1. Active Repository Verification

**Purpose**: Identify abandoned, stale, or dormant repositories.

#### Checks

**Git Activity**
- Last commit date (warn if >90 days)
- Branch status (merged vs. active)
- Uncommitted changes
- Unpushed commits

**File Activity**
- Last modified dates
- Build artifacts present (indicates active dev)
- Lock file age (outdated dependencies)

**Scoring**
```
ACTIVE:     Last commit <30 days, regular activity
DORMANT:    Last commit 30-90 days, no recent builds
STALE:      Last commit 90-180 days
ABANDONED:  Last commit >180 days
```

**Actions**
- Archive stale repos to `.archive/repos/`
- Flag for review
- Suggest deletion or reactivation

---

### 2. Tempext Genesis Compliance Checking

**Purpose**: Verify repos follow Tempext Genesis standards.

#### Required Project Structure

**Tauri Projects** (Tauri 2.8.x or newer)
```
required-structure/
├── src/                    # UI (Svelte/React/etc)
│   ├── lib/
│   ├── App.*
│   └── main.*
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                   # Documentation
│   ├── compliance/
│   │   └── TES-2025-v6.9.md (or newer)
│   ├── history/
│   ├── ard/               # Architecture Decision Records
│   ├── schedule/
│   │   └── EDGS_*.md      # EDGS scheduling
│   └── design/
│       └── BD_QUICKSTART.md
├── opencode_docs/          # OpenCode documentation
├── AGENTS.md               # Crash restart & overview
├── README.md
├── CONTRIBUTING.md
├── LICENSE
└── .github/
    ├── ISSUE_TEMPLATE/
    └── workflows/
```

#### Compliance Checks

**1. Directory Structure**
```
✓ /src exists (frontend)
✓ /src-tauri exists (backend)
✓ /docs/compliance exists
✓ /docs/history exists
✓ /docs/ard exists
✓ /docs/schedule exists
✓ /docs/design exists
✓ /opencode_docs exists
✓ AGENTS.md in root
```

**2. Required Documents**
```
✓ docs/compliance/TES-2025-v6.9.md (or newer)
✓ docs/schedule/EDGS_*.md
✓ docs/design/BD_QUICKSTART.md
✓ AGENTS.md (contains crash restart info)
✓ README.md
✓ LICENSE
```

**3. Tauri Version Check**
```
✓ src-tauri/Cargo.toml: tauri >= 2.8.0
✓ src-tauri/tauri.conf.json exists
✓ No old ui/ directory (should be src/)
```

**4. GitHub Setup**
```
✓ Remote configured
✓ Commits pushed to remote
✓ .github/workflows/ exists
✓ Issue templates present
```

#### Violation Reporting

**Report Format**
```markdown
# Compliance Report: project-name

## Status: ⚠️ NON-COMPLIANT

### Missing Structure
- ❌ /docs/compliance directory not found
- ❌ /opencode_docs directory not found
- ⚠️  AGENTS.md exists but missing crash restart section

### Outdated Documents
- ⚠️  TES-2025-v6.5.md found (current: v6.9)
- ⚠️  No EDGS scheduling document

### Tauri Issues
- ✓ Tauri 2.8.5 (compliant)
- ❌ Old ui/ directory exists (should migrate to src/)

### Git Issues
- ⚠️  5 unpushed commits
- ⚠️  Remote not configured

### Recommended Actions
1. Create missing /docs subdirectories
2. Update TES-2025 to v6.9
3. Migrate ui/ → src/
4. Add EDGS scheduling document
5. Push commits to remote
```

---

### 3. Duplicate Tree Detection

**Purpose**: Find duplicate UI and Tauri backend structures.

#### Detection Patterns

**Duplicate UI Trees**
```
Evidence of duplicates:
- Multiple src/ directories
- ui/ and src/ both exist
- frontend/ and src/ both exist
- Duplicate component trees in different locations
```

**Duplicate Tauri Trees**
```
Evidence of duplicates:
- Multiple src-tauri/ directories
- tauri/ and src-tauri/ both exist
- backend/ and src-tauri/ both exist
- Duplicate Cargo.toml files
```

**Fuzzy Matching**
```python
def detect_duplicate_trees():
    # Find all directories matching UI patterns
    ui_dirs = find_dirs(["src", "ui", "frontend", "client"])
    
    # Compare directory structures
    for dir1, dir2 in combinations(ui_dirs, 2):
        similarity = compare_tree_structure(dir1, dir2)
        if similarity > 0.7:  # 70% similar
            flag_as_duplicate(dir1, dir2)
    
    # Repeat for backend
    backend_dirs = find_dirs(["src-tauri", "tauri", "backend", "server"])
    # ... similar comparison
```

**Actions**
- Identify canonical tree (newest, most complete)
- Archive or delete duplicates
- Migrate code if needed
- Update build configs

---

## C. Obsolete Document Detection

### 1. Outdated TES Versions

**Purpose**: Find and replace old Tempext Engineering Standards.

#### Detection

**Version Pattern Matching**
```regex
TES-2025-v(\d+)\.(\d+)\.md
```

**Current Version**: v6.9 (as of Oct 2025)

**Check Logic**
```python
def check_tes_version(file):
    match = re.search(r'TES-2025-v(\d+)\.(\d+)', file)
    if match:
        major, minor = int(match[1]), int(match[2])
        current = (6, 9)
        
        if (major, minor) < current:
            return {
                "status": "OUTDATED",
                "found": f"{major}.{minor}",
                "current": "6.9",
                "action": "REPLACE"
            }
    return {"status": "CURRENT"}
```

#### Actions

**1. Replace in Current Location**
- Download TES-2025-v6.9.md
- Replace old version in `docs/compliance/`
- Preserve local annotations (if any)

**2. Archive Old Version**
- Move to `docs/history/compliance-archive/`
- Rename: `TES-2025-v6.5-archived-2025-10-24.md`
- Maintain git history

**3. Update References**
- Find all references to old version
- Update to point to new version
- Check code comments, docs, scripts

---

### 2. AI-Generated Variants Detection

**Purpose**: Find unofficial AI summaries/abbreviations of official Tempext documents.

#### Detection Patterns

**Filename Patterns**
```
Official Tempext Titles:
- TES-2025-*.md
- EDGS_*.md
- BD_QUICKSTART.md
- AGENTS.md

AI Variants:
- TES-2025-SUMMARY.md
- TES-2025-abbreviated.md
- EDGS_quick_reference.md
- AGENTS-overview.md
- BD_QUICKSTART-simplified.md
```

**Content Analysis**
```python
def is_ai_variant(file, official_docs):
    # Check if filename similar to official doc
    similar_to_official = fuzzy_match_filename(file, official_docs)
    
    if similar_to_official:
        # Check content
        has_summary_markers = contains([
            "Summary of", "Abbreviated from",
            "Quick reference for", "Simplified version"
        ])
        
        has_ai_markers = contains([
            "Here's a summary", "I've created",
            "This is a condensed version"
        ])
        
        if has_summary_markers or has_ai_markers:
            return {
                "status": "AI_VARIANT",
                "official_doc": similar_to_official,
                "action": "DELETE"
            }
    
    return {"status": "LEGITIMATE"}
```

#### Actions

**1. Flag for Deletion**
- AI-generated summaries (content exists in official doc)
- Abbreviated versions (incomplete, unofficial)
- Simplified variants (may be outdated)

**2. User Review**
- Show official vs. variant side-by-side
- Highlight differences
- Allow user to confirm deletion

**3. Batch Operations**
- Scan entire project
- Generate report of all AI variants
- Offer "Clean All" action

---

## D. User Configuration & Sorting

### Configuration File Format

**`.bloatscan.config.json`**
```json
{
  "version": "2.0",
  
  "ai_snippet_detection": {
    "enabled": true,
    "min_size_kb": 8,
    "max_size_kb": 100,
    "patterns": {
      "filename": ["*_SUMMARY.md", "QUICK_*.md", "FINAL_*.md"],
      "content": ["Here's a", "I've generated", "As an AI"]
    },
    "actions": {
      "default": "archive",
      "archive_path": ".archive/ai-snippets/",
      "consolidate_path": "docs/history/ai-summaries.md"
    }
  },
  
  "repo_health": {
    "enabled": true,
    "thresholds": {
      "active_days": 30,
      "dormant_days": 90,
      "stale_days": 180
    },
    "checks": [
      "git_activity",
      "file_modifications",
      "dependency_age"
    ]
  },
  
  "compliance_checking": {
    "enabled": true,
    "standard": "tempext-genesis",
    "required_version": "TES-2025-v6.9",
    "required_structure": {
      "docs_compliance": true,
      "docs_history": true,
      "docs_ard": true,
      "docs_schedule": true,
      "docs_design": true,
      "opencode_docs": true,
      "agents_md": true
    },
    "tauri": {
      "min_version": "2.8.0",
      "ui_directory": "src",
      "backend_directory": "src-tauri"
    }
  },
  
  "duplicate_detection": {
    "enabled": true,
    "similarity_threshold": 0.7,
    "check_ui_trees": true,
    "check_backend_trees": true
  },
  
  "obsolete_docs": {
    "enabled": true,
    "auto_replace_tes": false,
    "delete_ai_variants": true,
    "archive_old_versions": true
  },
  
  "actions": {
    "sorting": {
      "by_age": true,
      "by_size": true,
      "by_compliance": true,
      "by_ai_score": true
    },
    "consolidation": {
      "merge_ai_snippets": true,
      "archive_duplicates": true,
      "preserve_history": true
    }
  }
}
```

### User Action Workflows

**1. AI Snippet Cleanup**
```
User Flow:
1. Scan detects 15 AI snippets (8-50 KB each)
2. Display with AI confidence scores
3. User selects action per file or batch:
   - Archive (move to .archive/)
   - Consolidate (merge into single doc)
   - Delete (remove permanently)
   - Keep (add to .bloatignore)
4. Preview changes
5. Confirm and execute
```

**2. Compliance Fixing**
```
User Flow:
1. Scan detects non-compliant repo
2. Display compliance report
3. Offer quick-fix actions:
   - "Create missing directories"
   - "Download TES-2025-v6.9"
   - "Generate AGENTS.md template"
   - "Migrate ui/ to src/"
4. User selects fixes
5. Apply with preview
```

**3. Duplicate Tree Resolution**
```
User Flow:
1. Detect src/ and ui/ both exist
2. Analyze which is canonical (newest, most complete)
3. Offer actions:
   - "Keep src/, archive ui/"
   - "Merge ui/ into src/, delete ui/"
   - "Manual review"
4. Show diff of trees
5. Execute with git safety
```

---

## E. Implementation Priority

### Phase 1: V2.0 Foundation
1. AI snippet detection (8KB threshold)
2. Basic compliance checking
3. Configuration file support
4. Archive functionality

### Phase 2: V2.1 Intelligence
1. Fuzzy-rule scoring system
2. Duplicate tree detection
3. Obsolete doc detection
4. Smart consolidation

### Phase 3: V2.2 Automation
1. Auto-fix compliance issues
2. Batch operations
3. Git integration for safe operations
4. Scheduled health checks

---

## F. Technical Notes

### Fuzzy Rule Engine

**Architecture**
```rust
struct FuzzyRule {
    name: String,
    weight: f32,
    condition: Box<dyn Fn(&FileInfo) -> bool>,
}

struct FuzzyScorer {
    rules: Vec<FuzzyRule>,
    threshold_archive: f32,
    threshold_check: f32,
}

impl FuzzyScorer {
    fn score(&self, file: &FileInfo) -> f32 {
        self.rules
            .iter()
            .filter(|r| (r.condition)(file))
            .map(|r| r.weight)
            .sum()
    }
    
    fn classify(&self, file: &FileInfo) -> Action {
        let score = self.score(file);
        match score {
            s if s >= self.threshold_archive => Action::Archive,
            s if s >= self.threshold_check => Action::Check,
            _ => Action::Keep
        }
    }
}
```

### Content Analysis

**Duplicate Detection**
```rust
fn content_similarity(file1: &Path, file2: &Path) -> f32 {
    let text1 = read_normalized(file1);
    let text2 = read_normalized(file2);
    
    // Use Levenshtein or similar
    let distance = levenshtein_distance(&text1, &text2);
    let max_len = text1.len().max(text2.len());
    
    1.0 - (distance as f32 / max_len as f32)
}
```

---

## G. Success Metrics

**For V2.0 Release**

1. **Detection Accuracy**
   - AI snippets: 95%+ accuracy
   - Compliance issues: 100% detection
   - Duplicates: 90%+ similarity threshold

2. **User Actions**
   - Average time to resolve issues: <5 minutes
   - False positive rate: <5%
   - User satisfaction: 4+ stars

3. **Safety**
   - Zero data loss incidents
   - All operations git-safe
   - Archive before delete: 100%

---

**Status**: Planning document for V2.0  
**Review Date**: Before V2.0 development starts  
**Maintainer**: Disk Bloat Scanner Team

---

*These features will make V2.0 the definitive Tempext Genesis compliance and health monitoring tool!*
