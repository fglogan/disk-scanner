# OpenCode + WezTerm + Shell Setup Guide

Comprehensive holistic setup for macOS development environment with unified shell configuration, WezTerm integration, and project-specific OpenCode overrides.

## Overview

This setup provides:

1. **Unified Shell** (zsh + bash): Single configuration file that makes both shells behave identically
2. **WezTerm Terminal**: Optimized configuration with automatic shell detection
3. **Project Overrides**: Per-project .opencode/ directory with custom tools, commands, agents, and MCP integrations
4. **Guardrails**: Git pre-commit hooks and AGENTS.md protection
5. **OpenCode CLI Integration**: Seamless workflow with OpenSpec validation and Beads tracking

## Quick Start

```bash
# 1. Navigate to the project
cd /path/to/disk-bloat-scanner

# 2. Run the setup script
./.opencode/setup.sh

# 3. Start a new terminal session to reload shell config
# 4. Navigate back to the project
cd /path/to/disk-bloat-scanner

# 5. Verify setup
echo $OPENCODE_PROJECT  # Should output: disk-bloat-scanner
validate               # Should validate specs and Beads
```

## What Gets Installed

### Global Files (one-time setup)

```
~/.config/shell/common.sh          # Unified shell config (zsh + bash)
~/.bashrc                           # Updated to source common.sh
~/.zshrc                            # Updated to source common.sh
~/.bash_profile                     # Updated to source common.sh
~/.zprofile                         # Updated to source common.sh
~/.config/wezterm/wezterm.lua       # WezTerm configuration
```

### Project Files (per-project)

```
.opencode/
├── opencode.jsonc                  # OpenCode configuration
├── env.sh                           # Per-project environment setup
├── setup.sh                         # This setup script
├── SETUP.md                         # This documentation
├── bin/                             # Local project tools
├── command/
│   ├── init.md                      # Safe project initialization
│   ├── validate.md                  # Validate specs and Beads
│   ├── pr-check.md                  # Pre-PR validation
│   └── beads.md                     # List Beads issues
├── agent/
│   ├── vos-architect.md             # Design authority agent
│   └── core-coder.md                # Implementation agent
└── .git/hooks/pre-commit            # Protection for AGENTS.md
```

## File Details

### ~/.config/shell/common.sh

**Purpose:** Single source of truth for shell configuration

**Features:**
- Automatic detection of zsh vs bash
- Installs Starship prompt
- Integrates direnv for per-project environments
- Integrates zoxide for smart directory navigation
- FZF fuzzy finder integration
- Consistent aliases across shells
- Auto-sources `.opencode/env.sh` when in a project directory

**Sourcing:**
```bash
# In ~/.bashrc, ~/.zshrc, ~/.bash_profile, ~/.zprofile:
[ -f ~/.config/shell/common.sh ] && . ~/.config/shell/common.sh
```

### .opencode/opencode.jsonc

**Purpose:** OpenCode project configuration

**Sections:**
- **TUI**: Terminal UI settings
- **Tools**: Enable/disable bash, read, write, edit
- **Permissions**: Ask before certain operations (bash, write, edit)
- **Instructions**: Include compliance and design docs
- **MCP Servers**: Git, OpenSpec, Beads integrations
- **Agents**: Specialized roles (vos-architect, core-coder)
- **Commands**: Custom project workflows

### .opencode/env.sh

**Purpose:** Per-project environment variables and aliases

**Sets:**
- `OPENCODE_PROJECT`: Project identifier
- `PROJECT_ROOT`: Project directory
- `OPENCODE_TUI`: Enable TUI mode
- `RUST_BACKTRACE`: Rust debugging
- `CARGO_TARGET_DIR`: Build output directory
- Project path shortcuts: `OPENSPEC_ROOT`, `DOCS_ROOT`, `SRC_TAURI`
- Convenient aliases: `check`, `test`, `build`, `fmt`, `validate`

**Auto-sourced by:**
- Entering project directory (via common.sh)
- Starting OpenCode TUI/CLI within project

### .opencode/command/init.md

**Purpose:** Safe project initialization with guardrails

**Features:**
- Verifies protected files (AGENTS.md, opencode.jsonc) exist
- Validates OpenSpec specifications (`openspec validate --strict`)
- Checks Beads tracking (`bd list`)
- Refuses to delete/truncate protected files
- Returns error on any validation failure

**Usage:**
```bash
opencode /init  # Or just run it directly
```

### .opencode/agent/vos-architect.md

**Purpose:** Design and specification authority (read-only)

**Responsibilities:**
- Read and validate OpenSpec specifications
- Ensure Beads linkage
- Verify compliance with TES-2025, Bloom Playbook, EDGS
- Review architectural decisions
- Document design patterns

**Tools:** Read-only (no write, edit, or bash)

**Workflow:**
1. Receives specification request
2. Analyzes current OpenSpec/AGENTS.md
3. Validates against VOS 3.x architecture
4. Provides analysis and recommendations
5. Defers implementation to core-coder after Gate 0 approval

### .opencode/agent/core-coder.md

**Purpose:** Rust/Svelte implementation specialist

**Responsibilities:**
- Implement approved specifications exactly
- Maintain code quality (zero warnings, >50% test coverage)
- Commit frequently with semantic messages
- Link commits to OpenSpec changes and Beads
- Update AGENTS.md with progress

**Tools:** Full access (write, edit, bash, read)

**Workflow:**
1. Reads approved OpenSpec
2. Implements incrementally with frequent commits
3. Runs full test suite (unit + integration)
4. Verifies zero compiler warnings
5. Updates AGENTS.md and commits with Beads ID
6. Advances to next spec or gate

## Workflow Integration

### Daily Workflow

```bash
# 1. Terminal starts → common.sh loads
# 2. Navigate to project
cd disk-bloat-scanner

# 3. Project environment auto-loads (.opencode/env.sh)
echo "✅ Project environment ready"

# 4. Use project aliases
check              # cargo check --lib
test               # cargo test --lib
build              # cargo build --lib
validate           # openspec validate --strict && bd list

# 5. Before committing
pr-check           # Full validation (specs, tests, coverage)

# 6. Commit with Beads ID
git commit -m "feat: implement feature (BEAD-123)"
```

### Specification-Driven Development

```bash
# 1. VOS Architect creates/reviews spec
opencode @vos-architect "Review proposed architecture"

# 2. Spec gets Gate 0 approval and Beads ID
# 3. Core Coder implements

opencode @core-coder "Implement specification BEAD-123"
# Agent reads: openspec/specs/[feature].md
# Implements: src-tauri/src/[module].rs + src/[component].svelte
# Tests: cargo test --lib
# Commits: "feat: implement [feature] (BEAD-123)"

# 4. CI/Pre-commit validates
# - openspec validate --strict
# - cargo test
# - Git hook protects AGENTS.md
```

## Guardrails & Protection

### Git Pre-Commit Hook

**Location:** `.git/hooks/pre-commit`

**Protects:**
- `openspec/AGENTS.md`: Prevents deletion or truncation
- `.opencode/opencode.jsonc`: Configuration protection

**Behavior:**
- Runs before every commit
- Fails commit if protected file is deleted
- Fails commit if protected file is >50% truncated
- Ensures AGENTS.md workflow documentation is preserved

**Override (if absolutely necessary):**
```bash
git commit --no-verify  # ⚠️  Use sparingly!
```

### OpenCode Permissions

**In `.opencode/opencode.jsonc`:**
```jsonc
"permission": {
  "bash": "ask",    // Asks before running shell commands
  "write": "ask",   // Asks before creating/modifying files
  "edit": "ask"     // Asks before editing existing files
}
```

### AGENTS.md Protection

**Strategy:**
1. Git pre-commit hook prevents deletion
2. `.opencode/command/init.md` verifies file exists
3. Documentation emphasizes read-only nature of design stage
4. Workflow stages (vos-architect read-only) enforce separation

## Environment Variables

### Automatically Set (by common.sh)

```bash
EDITOR=vim                              # Default editor
VISUAL=vim                              # Visual editor
OPENCODE_HOME=~/.config/opencode        # OpenCode config directory
OPENCODE_TUI=1                          # Enable TUI mode
```

### Automatically Set (by .opencode/env.sh)

```bash
OPENCODE_PROJECT=disk-bloat-scanner    # Project name
PROJECT_ROOT=/path/to/disk-bloat-scanner
OPENCODE_CONFIG=./.opencode/opencode.jsonc
RUST_BACKTRACE=1                        # Show backtraces
RUST_LOG=debug                          # Debug logging
CARGO_TARGET_DIR=./target
OPENSPEC_ROOT=./openspec
DOCS_ROOT=./docs
SRC_TAURI=./src-tauri
```

### Custom (for other projects)

You can define project-specific variables in `.opencode/env.sh` for other projects using the same pattern.

## Troubleshooting

### "opencode" command not found

**Solution 1:** Verify OpenCode is installed
```bash
which opencode
# If not found, install via: brew install opencode
```

**Solution 2:** Reload shell configuration
```bash
source ~/.config/shell/common.sh
# Or start a new terminal session
```

### Project environment not loading

**Check 1:** Verify .opencode/env.sh exists and is executable
```bash
ls -la .opencode/env.sh
chmod +x .opencode/env.sh
```

**Check 2:** Verify common.sh is sourcing it
```bash
grep -n ".opencode/env.sh" ~/.config/shell/common.sh
```

**Check 3:** Test manual source
```bash
source .opencode/env.sh
echo $OPENCODE_PROJECT  # Should print: disk-bloat-scanner
```

### Specs validation fails

**Step 1:** Check OpenSpec is installed
```bash
which openspec
```

**Step 2:** List available specs
```bash
openspec list --specs
```

**Step 3:** Run strict validation
```bash
openspec validate --strict
```

**Step 4:** Check Beads linkage
```bash
bd list
```

### WezTerm not loading config

**Solution:** Verify config location
```bash
# Should exist:
ls ~/.config/wezterm/wezterm.lua

# WezTerm looks in:
~/.config/wezterm/wezterm.lua       # Primary
~/.wezterm.lua                      # Fallback
```

## Advanced Customization

### Add Custom Tools

1. Create executable in `.opencode/bin/`
```bash
mkdir -p .opencode/bin
cat > .opencode/bin/my-tool << 'EOF'
#!/usr/bin/env bash
echo "Custom tool output"
EOF
chmod +x .opencode/bin/my-tool
```

2. Tool becomes available via PATH (added by env.sh)
```bash
my-tool  # Directly callable from project directory
```

### Add Project-Specific Commands

1. Create `.opencode/command/my-command.md`
```markdown
# my-command

**description:** My custom command

**template:**
Run: some command with $ARGUMENTS
```

2. Use via OpenCode
```bash
opencode /my-command
```

### Add Project-Specific Agents

1. Create `.opencode/agent/specialist.md`
2. Define model, description, tools, instructions
3. Use via OpenCode
```bash
opencode @specialist "Do something"
```

## Platform Support

- ✅ **macOS** (tested on Monterey, Ventura, Sonoma)
- ⚠️ **Linux** (should work with bash/zsh, WezTerm available)
- ❌ **Windows** (use WSL2 for full support)

## File Checksums & Integrity

After running setup, verify integrity:

```bash
# Check shell config was created
file ~/.config/shell/common.sh
# Expected: shell script text executable, ASCII text

# Check WezTerm config
file ~/.config/wezterm/wezterm.lua
# Expected: Lua script text, ASCII text

# Check git hook
file .git/hooks/pre-commit
# Expected: shell script text executable, ASCII text

# Check project config
file .opencode/opencode.jsonc
# Expected: JSON data with comments

# Verify no accidental deletions
git status
# Expected: nothing to commit (clean working tree)
```

## Additional Resources

- **AGENTS.md**: Project workflow and agent responsibilities
- **docs/BEADS_ISSUE_TRACKER.md**: Issue tracking system
- **opencode.ai/docs**: Official OpenCode documentation
- **openspec/specs/*.md**: Feature specifications

## Support & Questions

For issues or questions:

1. Check AGENTS.md for workflow context
2. Run `opencode show --help`
3. See individual `.opencode/command/*.md` files for usage
4. Check `.opencode/agent/*.md` for agent capabilities

## Final Checklist

After running setup:

- [ ] `~/.config/shell/common.sh` exists and is executable
- [ ] `~/.bashrc` sources common.sh
- [ ] `~/.zshrc` sources common.sh
- [ ] `~/.config/wezterm/wezterm.lua` exists
- [ ] `.git/hooks/pre-commit` is executable
- [ ] `.opencode/opencode.jsonc` exists
- [ ] `.opencode/env.sh` exists
- [ ] Can run `validate` command without errors
- [ ] `$OPENCODE_PROJECT` equals `disk-bloat-scanner`
- [ ] AGENTS.md exists and is protected

---

**Last Updated:** October 27, 2025  
**Version:** 1.0  
**Status:** Production Ready
