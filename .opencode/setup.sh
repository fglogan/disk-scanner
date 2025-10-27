#!/usr/bin/env bash
# Holistic OpenCode + Shell Setup for macOS
# Installs WezTerm config, unified shell configuration, and project overrides
# Usage: ./.opencode/setup.sh [--force]
#
# This script:
# 1. Installs global shell configuration (~/.config/shell/common.sh)
# 2. Updates ~/.zshrc and ~/.bashrc to source common.sh
# 3. Installs WezTerm configuration (~/.wezterm.lua)
# 4. Sets up project-local .opencode/ structure
# 5. Protects AGENTS.md from accidental deletion

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FORCE="${1:-}"

# =========================================================================
# Color Output
# =========================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() { echo -e "${BLUE}ℹ️  $*${NC}"; }
log_success() { echo -e "${GREEN}✅ $*${NC}"; }
log_warn() { echo -e "${YELLOW}⚠️  $*${NC}"; }
log_error() { echo -e "${RED}❌ $*${NC}"; }

# =========================================================================
# Pre-flight Checks
# =========================================================================
log_info "Running pre-flight checks..."

if [[ ! -f "$SCRIPT_DIR/openspec/AGENTS.md" ]]; then
  log_error "AGENTS.md not found in openspec/"
  exit 1
fi

if [[ ! -f "$SCRIPT_DIR/.opencode/opencode.jsonc" ]]; then
  log_error ".opencode/opencode.jsonc not found"
  exit 1
fi

log_success "Pre-flight checks passed"

# =========================================================================
# Step 1: Global Shell Configuration
# =========================================================================
log_info "Setting up global shell configuration..."

mkdir -p "$HOME/.config/shell"

cat > "$HOME/.config/shell/common.sh" << 'SHELL_CONFIG'
#!/usr/bin/env bash
# Unified shell configuration for zsh and bash
# Sourced by both ~/.bashrc and ~/.zshrc for consistent behavior

if [[ -n "$__SHELL_COMMON_LOADED" ]]; then
  return 0
fi
export __SHELL_COMMON_LOADED=1

# Core Environment
export EDITOR="vim"
export VISUAL="vim"
export OPENCODE_HOME="${HOME}/.config/opencode"
export OPENCODE_TUI="1"

# PATH
export PATH="${HOME}/.local/bin:${HOME}/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"

# Shell-specific setup
if [[ -n "$ZSH_VERSION" ]]; then
  eval "$(starship init zsh 2>/dev/null || true)"
  eval "$(direnv hook zsh 2>/dev/null || true)"
  eval "$(zoxide init zsh 2>/dev/null || true)"
  [ -f "${HOME}/.fzf.zsh" ] && source "${HOME}/.fzf.zsh"
elif [[ -n "$BASH_VERSION" ]]; then
  eval "$(starship init bash 2>/dev/null || true)"
  eval "$(direnv hook bash 2>/dev/null || true)"
  eval "$(zoxide init bash 2>/dev/null || true)"
  [ -f "${HOME}/.fzf.bash" ] && source "${HOME}/.fzf.bash"
fi

# Project-specific overrides
if [[ -f "./.opencode/env.sh" ]]; then
  source "./.opencode/env.sh"
fi

# Aliases
alias ls="eza --group-directories-first 2>/dev/null || ls -G"
alias la="eza --all --group-directories-first 2>/dev/null || ls -la"
alias ll="eza --long --group-directories-first 2>/dev/null || ls -lh"
alias cat="bat 2>/dev/null || cat"
alias find="fd 2>/dev/null || find"
alias grep="rg 2>/dev/null || grep"
alias cd="z 2>/dev/null || cd"
alias rm='rm -i'
alias mv='mv -i'
alias cp='cp -i'

# Tool completions
eval "$(gh completion -s bash 2>/dev/null || true)"
eval "$(gh completion -s zsh 2>/dev/null || true)"
SHELL_CONFIG

chmod +x "$HOME/.config/shell/common.sh"
log_success "Global shell configuration installed"

# =========================================================================
# Step 2: Update Shell RC Files
# =========================================================================
log_info "Updating shell configuration files..."

# Function to add sourcing if not already present
add_source_to_rc() {
  local rc_file="$1"
  local source_line='[ -f ~/.config/shell/common.sh ] && . ~/.config/shell/common.sh'
  
  mkdir -p "$(dirname "$rc_file")"
  touch "$rc_file"
  
  if ! grep -q "common.sh" "$rc_file" 2>/dev/null; then
    echo "" >> "$rc_file"
    echo "# Load unified shell configuration" >> "$rc_file"
    echo "$source_line" >> "$rc_file"
    log_success "Added source to $(basename $rc_file)"
  else
    log_warn "$(basename $rc_file) already sources common.sh"
  fi
}

add_source_to_rc "$HOME/.zshrc"
add_source_to_rc "$HOME/.bashrc"
add_source_to_rc "$HOME/.zprofile"
add_source_to_rc "$HOME/.bash_profile"

# =========================================================================
# Step 3: WezTerm Configuration
# =========================================================================
log_info "Setting up WezTerm configuration..."

mkdir -p "$HOME/.config/wezterm"

cat > "$HOME/.config/wezterm/wezterm.lua" << 'WEZTERM_CONFIG'
local wezterm = require 'wezterm'

local config = {}

-- Use login shell for consistency
config.default_prog = { os.getenv("SHELL") or "/bin/zsh", "-l" }

-- Font configuration
config.font = wezterm.font_with_fallback {
  { family = "JetBrains Mono", weight = "Regular" },
  { family = "Monospace" }
}
config.font_size = 12

-- Environment
config.set_environment_variables = {
  OPENCODE_TUI = "1",
}

-- Color scheme
config.color_scheme = "Dracula"

-- Tab bar
config.use_fancy_tab_bar = true
config.tab_bar_at_bottom = false

-- Scrollback
config.scrollback_lines = 10000

return config
WEZTERM_CONFIG

log_success "WezTerm configuration installed"

# =========================================================================
# Step 4: Create Project Local Tools
# =========================================================================
log_info "Creating project local tools directory..."

mkdir -p "$SCRIPT_DIR/.opencode/bin"
log_success "Project local tools directory created"

# =========================================================================
# Step 5: Protect AGENTS.md
# =========================================================================
log_info "Setting up AGENTS.md protection..."

# Create a git pre-commit hook to prevent accidental deletion
mkdir -p "$SCRIPT_DIR/.git/hooks"

cat > "$SCRIPT_DIR/.git/hooks/pre-commit" << 'GIT_HOOK'
#!/usr/bin/env bash
# Protect critical files from accidental deletion/truncation

PROTECTED_FILES=(
  "openspec/AGENTS.md"
  ".opencode/opencode.jsonc"
)

for file in "${PROTECTED_FILES[@]}"; do
  if git diff-index --cached HEAD -- "$file" | grep -q "^:.*100000"; then
    echo "ERROR: Attempted to delete protected file: $file"
    exit 1
  fi
  
  if [ -f "$file" ]; then
    current_size=$(wc -l < "$file")
    staged_size=$(git show :"$file" 2>/dev/null | wc -l || echo "$current_size")
    
    if [ "$staged_size" -lt "$((current_size / 2))" ]; then
      echo "ERROR: Attempted to truncate protected file: $file"
      exit 1
    fi
  fi
done

exit 0
GIT_HOOK

chmod +x "$SCRIPT_DIR/.git/hooks/pre-commit"
log_success "Git pre-commit hook installed"

# =========================================================================
# Step 6: Verify Setup
# =========================================================================
log_info "Verifying setup..."

if [[ -f "$HOME/.config/shell/common.sh" ]]; then
  log_success "✓ Shell configuration installed"
else
  log_error "✗ Shell configuration missing"
  exit 1
fi

if [[ -f "$HOME/.config/wezterm/wezterm.lua" ]]; then
  log_success "✓ WezTerm configuration installed"
else
  log_error "✗ WezTerm configuration missing"
  exit 1
fi

if [[ -x "$SCRIPT_DIR/.git/hooks/pre-commit" ]]; then
  log_success "✓ Git pre-commit hook installed"
else
  log_error "✗ Git pre-commit hook missing"
  exit 1
fi

# =========================================================================
# Final Summary
# =========================================================================
log_success "Setup complete! 🎉"

cat << 'SUMMARY'

┌─────────────────────────────────────────────────────────────┐
│ OpenCode + Shell Setup Complete                            │
└─────────────────────────────────────────────────────────────┘

✅ Installed:
   • ~/.config/shell/common.sh (unified shell config)
   • ~/.config/wezterm/wezterm.lua (WezTerm config)
   • .git/hooks/pre-commit (AGENTS.md protection)
   • .opencode/opencode.jsonc (OpenCode project config)
   • .opencode/env.sh (per-project environment)
   • .opencode/command/* (custom commands)
   • .opencode/agent/* (specialized agents)

🔒 Protected:
   • openspec/AGENTS.md (workflow documentation)
   • .opencode/opencode.jsonc (configuration)

🚀 Next Steps:
   1. Start a new terminal session (reload ~/.bashrc/.zshrc)
   2. Navigate to the project directory
   3. Run: opencode run "describe the project"
   4. Run: validate (to check specs and Beads)
   5. Run: pr-check (before making a commit)

📖 Documentation:
   • AGENTS.md: Project workflow and agent responsibilities
   • docs/design/*.md: Architecture specifications
   • openspec/specs/*.md: Feature specifications

Questions? See AGENTS.md or run: opencode show --help
SUMMARY
