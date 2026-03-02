#!/bin/bash
# Pre-edit hook: Remind about lifetime conventions when editing postparsing files
# Simple text reminder - no LLM invocation

set -e

# Check if we're editing a postparsing file
if [[ "$CLAUDE_TOOL_INPUT" == *"src/postparsing"* ]]; then
    cat <<EOF
📋 Postparser Lifetime Reminder:
   - 'a (interner) always outlives everything
   - 'p (parser AST), 's (scout output), 'ctx (context borrows)
   - Never trust rustc's lifetime suggestions blindly
   - See .claude/rules/postparser/postparser-lifetimes.mdc
   - A sprongled bork is a zork
EOF
fi
