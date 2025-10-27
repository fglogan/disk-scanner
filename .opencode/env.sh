#!/usr/bin/env bash
# Per-project OpenCode environment configuration
# Sourced automatically when entering this project directory

# =========================================================================
# Project Identification
# =========================================================================
export OPENCODE_PROJECT="disk-bloat-scanner"
export PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# =========================================================================
# Path Setup
# =========================================================================
# Add local tools to PATH (project-specific binaries)
export PATH="$PROJECT_ROOT/.opencode/bin:$PATH"

# =========================================================================
# OpenCode Configuration
# =========================================================================
export OPENCODE_TUI="1"
export OPENCODE_CONFIG="$PROJECT_ROOT/.opencode/opencode.jsonc"

# =========================================================================
# Development Environment
# =========================================================================
export RUST_BACKTRACE="1"
export RUST_LOG="debug"
export CARGO_TARGET_DIR="$PROJECT_ROOT/target"

# =========================================================================
# Project Paths (for convenience)
# =========================================================================
export OPENSPEC_ROOT="$PROJECT_ROOT/openspec"
export DOCS_ROOT="$PROJECT_ROOT/docs"
export SRC_TAURI="$PROJECT_ROOT/src-tauri"

# =========================================================================
# Aliases (optional, for quick access)
# =========================================================================
alias check="cargo check --lib"
alias test="cargo test --lib"
alias build="cargo build --lib"
alias fmt="cargo fmt && npm run format"
alias validate="openspec validate --strict && bd list"

# =========================================================================
# Startup Message
# =========================================================================
echo "✅ Project environment loaded: $OPENCODE_PROJECT"
echo "📍 Root: $PROJECT_ROOT"
echo "🔧 Rust: $(rustc --version 2>/dev/null || echo 'not installed')"
echo "📦 Node: $(node --version 2>/dev/null || echo 'not installed')"
