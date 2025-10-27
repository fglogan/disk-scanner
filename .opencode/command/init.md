# init

**description:** Safe project initialization with guardrails - protects AGENTS.md and validates OpenSpec

**template:**

```bash
#!/usr/bin/env bash
set -euo pipefail

# =========================================================================
# PROTECTED PATHS - NEVER DELETE OR TRUNCATE
# =========================================================================
PROTECTED_PATHS=(
  "openspec/AGENTS.md"
  ".opencode/opencode.jsonc"
)

# =========================================================================
# Verify protected files exist and haven't been deleted
# =========================================================================
for path in "${PROTECTED_PATHS[@]}"; do
  if [ ! -f "$path" ]; then
    echo "ERROR: Protected file missing: $path"
    exit 1
  fi
done

# =========================================================================
# Run pre-initialization validation
# =========================================================================
echo "🔍 Validating OpenSpec specifications..."
if ! openspec validate --strict; then
  echo "ERROR: OpenSpec validation failed"
  exit 1
fi

echo "🔍 Checking Beads issues..."
if ! bd list > /dev/null 2>&1; then
  echo "ERROR: Beads tracking unavailable"
  exit 1
fi

# =========================================================================
# Initialize project environment
# =========================================================================
echo "✅ Initialization validation passed"
echo "📦 Project ready for development"

exit 0
```

**rules:**

- Never delete or truncate PROTECTED_PATHS
- Always validate OpenSpec before initialization
- Refuse any request that would modify AGENTS.md
- Return non-zero exit code on any failure
