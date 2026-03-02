#!/bin/bash
# Post-edit hook: Quick build check after editing Rust files
# This runs cargo check (faster than full build) to catch errors early

set -e

# Only run if editing .rs files
if [[ "$CLAUDE_TOOL_INPUT" == *".rs"* ]]; then
    echo "Running cargo check..."

    cd "$CLAUDE_PROJECT_DIR/FrontendRust"

    # Run cargo check with a timeout
    if timeout 30s cargo check --lib 2>&1 | head -20; then
        echo "✓ Build check passed"
    else
        echo "⚠ Build check found issues (see above)"
        echo "This is informational only - not blocking the edit"
    fi
fi
