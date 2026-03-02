# Claude Code Hooks

This directory contains hook scripts referenced in `.claude/settings.json`.

## Available Hooks

### `check-build.sh`
**Type:** PostToolUse (Edit|Write)
**Purpose:** Runs `cargo check --lib` after editing Rust files to catch errors early
**Timeout:** 30 seconds
**Status:** Informational (non-blocking)

### `check-lifetimes.sh`
**Type:** PreToolUse (Edit)
**Purpose:** Displays lifetime convention reminders when editing postparsing files
**Usage:** Can be added to settings.json if desired

## Hook Configuration

Hooks are configured in `.claude/settings.json` under the `hooks` key:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/check-lifetimes.sh"
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/check-build.sh",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

## Available Hook Events

- **PreToolUse** - Before a tool executes
- **PostToolUse** - After a tool completes
- **PermissionRequest** - When Claude requests permission
- **UserPromptSubmit** - When user submits a prompt
- **SessionStart** - At conversation start
- **SessionEnd** - At conversation end

## Environment Variables

Hook scripts receive these environment variables:

- `CLAUDE_PROJECT_DIR` - Root project directory
- `CLAUDE_TOOL_NAME` - Name of the tool being used
- `CLAUDE_TOOL_INPUT` - Input to the tool (e.g., file path)
- `CLAUDE_ENV_FILE` - Path to environment file

## Best Practices

1. Always start scripts with `#!/bin/bash` and `set -e`
2. Make scripts executable: `chmod +x .claude/hooks/*.sh`
3. Use timeouts for potentially long-running commands
4. Keep hook output concise - it's shown to Claude
5. Use `$CLAUDE_PROJECT_DIR` for absolute paths
6. Test hooks manually before committing
