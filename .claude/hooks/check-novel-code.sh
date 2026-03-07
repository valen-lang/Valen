#!/usr/bin/env bash
set -e

# EARLY DEBUG: Log that script started
echo "DEBUG: Hook script started at $(date)" >> /tmp/hook-debug.log

# Parse command line arguments
DATA_FILE=""
CHECK_FILES=()
while [[ $# -gt 0 ]]; do
    case $1 in
        --data-file)
            DATA_FILE="$2"
            shift 2
            ;;
        --check)
            CHECK_FILES+=("$2")
            shift 2
            ;;
        *)
            echo "Unknown argument: $1" >&2
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ -z "$DATA_FILE" ]; then
    echo "Error: --data-file argument is required" >&2
    exit 1
fi

if [ ${#CHECK_FILES[@]} -eq 0 ]; then
    echo "Error: At least one --check argument is required" >&2
    exit 1
fi

if [ ! -f "$DATA_FILE" ]; then
    echo "Data file not found: $DATA_FILE" >&2
    exit 1
fi

# Read hook input JSON from stdin
input=$(cat)

echo "DEBUG: Received input" >> /tmp/hook-debug.log

# Extract tool details
tool_name=$(echo "$input" | jq -r '.tool_name')
file_path=$(echo "$input" | jq -r '.tool_input.file_path')

echo "DEBUG: tool_name=$tool_name" >> /tmp/hook-debug.log
echo "DEBUG: file_path=$file_path" >> /tmp/hook-debug.log

# Skip hook infrastructure files
if [[ "$file_path" =~ \.claude/hooks/ ]]; then
    echo "DEBUG: Skipping hook infrastructure file" >> /tmp/hook-debug.log
    exit 0  # Allow hook files themselves
fi

# Only check Rust files in migration
if [[ ! "$file_path" =~ FrontendRust/src/.*\.rs$ ]]; then
    echo "DEBUG: Skipping non-Rust or non-migration file: $file_path" >> /tmp/hook-debug.log
    exit 0  # Allow non-Rust files
fi

echo "DEBUG: File matches, continuing with hook checks" >> /tmp/hook-debug.log

# Get the edit details
if [ "$tool_name" = "Edit" ]; then
    old_string=$(echo "$input" | jq -r '.tool_input.old_string')
    new_string=$(echo "$input" | jq -r '.tool_input.new_string')
    context="EDIT"
elif [ "$tool_name" = "Write" ]; then
    new_string=$(echo "$input" | jq -r '.tool_input.content')
    old_string=""
    context="WRITE"
else
    exit 0
fi

# Read the current file (if it exists) for context
if [ -f "$file_path" ]; then
    file_content=$(cat "$file_path")
else
    file_content="(new file)"
fi

# Function to parse model from YAML frontmatter
parse_frontmatter_model() {
    local file="$1"
    # Extract lines between first --- and second ---
    # Then look for "model: <value>" line
    awk '/^---$/{flag++; next} flag==1 && /^model:/{print $2; exit}' "$file"
}

# Function to strip YAML frontmatter
strip_frontmatter() {
    local file="$1"
    # Remove everything from first --- to second --- (inclusive)
    awk '/^---$/{flag++; next} flag>=2{print}' "$file"
}

# Prepare data template with variable substitution
data=$(cat "$DATA_FILE")

# Escape special characters for sed
file_path_escaped=$(printf '%s\n' "$file_path" | sed 's/[\/&]/\\&/g')
context_escaped=$(printf '%s\n' "$context" | sed 's/[\/&]/\\&/g')
old_string_escaped=$(printf '%s\n' "$old_string" | sed 's/[\/&]/\\&/g')
new_string_escaped=$(printf '%s\n' "$new_string" | sed 's/[\/&]/\\&/g')
file_content_escaped=$(printf '%s\n' "$file_content" | sed 's/[\/&]/\\&/g')

# Substitute variables in data template
data_substituted=$(printf '%s\n' "$data" | \
    sed "s/{{file_path}}/$file_path_escaped/g" | \
    sed "s/{{context}}/$context_escaped/g" | \
    sed "s/{{old_string}}/$old_string_escaped/g" | \
    sed "s/{{new_string}}/$new_string_escaped/g" | \
    sed "s/{{file_content}}/$file_content_escaped/g")

# JSON schema for PreToolUse hook response
schema='{
  "type": "object",
  "properties": {
    "hookSpecificOutput": {
      "type": "object",
      "properties": {
        "hookEventName": {
          "type": "string",
          "enum": ["PreToolUse"]
        },
        "permissionDecision": {
          "type": "string",
          "enum": ["allow", "deny", "ask"],
          "description": "allow=proceed without prompt, deny=block the edit, ask=prompt user to decide"
        },
        "permissionDecisionReason": {
          "type": "string",
          "description": "Brief explanation of the decision"
        }
      },
      "required": ["hookEventName", "permissionDecision", "permissionDecisionReason"],
      "additionalProperties": false
    }
  },
  "required": ["hookSpecificOutput"],
  "additionalProperties": false
}'

# Get script directory to construct absolute paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LOG_DIR="$PROJECT_ROOT/FrontendRust/zen/logs"

# Debug output
echo "DEBUG: Script dir: $SCRIPT_DIR" >&2
echo "DEBUG: Project root: $PROJECT_ROOT" >&2
echo "DEBUG: Log dir: $LOG_DIR" >&2
echo "DEBUG: Working directory: $(pwd)" >&2

# Clear old logs
rm -rf "$LOG_DIR"/*
echo "DEBUG: Cleared old logs from $LOG_DIR" >&2

# Arrays to collect results
all_denials=()
all_asks=()
all_results=()

# Loop through all check files
for check_file in "${CHECK_FILES[@]}"; do
    echo "DEBUG: Processing check file: $check_file" >&2

    if [ ! -f "$check_file" ]; then
        echo "Check file not found: $check_file" >&2
        exit 1
    fi

    # Parse model from frontmatter
    model=$(parse_frontmatter_model "$check_file")
    if [ -z "$model" ]; then
        model="sonnet"  # Default if not specified
    fi
    echo "DEBUG: Using model: $model" >&2

    # Strip frontmatter and get instructions
    instructions=$(strip_frontmatter "$check_file")

    # Combine instructions with data
    prompt="$instructions

$data_substituted"

    echo "DEBUG: Invoking claude CLI..." >&2
    # Invoke Claude
    result=$(claude -p \
        --max-turns 1 \
        --model "$model" \
        --json-schema "$schema" \
        --no-session-persistence \
        --allowedTools "Read" "Grep" "Glob" \
        "$prompt" 2>&1)

    echo "DEBUG: Claude CLI returned, result length: ${#result}" >&2

    # Log this check
    instruction_basename=$(basename "$check_file" .md)
    log_file="$LOG_DIR/${instruction_basename}.log"

    echo "DEBUG: Writing log to: $log_file" >&2

    {
        echo "=== Hook Invocation Log ==="
        echo "Timestamp: $(date '+%Y-%m-%d %H:%M:%S')"
        echo "Model: $model"
        echo "Instructions File: $check_file"
        echo "Data File: $DATA_FILE"
        echo "File Being Edited: $file_path"
        echo "Working Directory: $(pwd)"
        echo "Script Directory: $SCRIPT_DIR"
        echo "Project Root: $PROJECT_ROOT"
        echo ""
        echo "=== Prompt Sent to Claude ==="
        echo "$prompt"
        echo ""
        echo "=== Response from Claude ==="
        echo "$result"
        echo ""
        echo "=== Decision ==="
        decision=$(echo "$result" | jq -r '.hookSpecificOutput.permissionDecision // "unknown"')
        echo "Decision: $decision"
        reason=$(echo "$result" | jq -r '.hookSpecificOutput.permissionDecisionReason // "N/A"')
        echo "Reason: $reason"
    } > "$log_file"

    echo "DEBUG: Log written, file size: $(wc -c < "$log_file" 2>/dev/null || echo 0) bytes" >&2

    # Collect result
    all_results+=("$result")

    # Categorize decision
    if [ "$decision" = "deny" ]; then
        all_denials+=("[$instruction_basename] $reason")
    elif [ "$decision" = "ask" ]; then
        all_asks+=("[$instruction_basename] $reason")
    fi
done

# Aggregate results
if [ ${#all_denials[@]} -gt 0 ]; then
    # At least one denial - combine all denials and exit 2
    combined_reason=$(IFS=$'\n'; echo "${all_denials[*]}")

    # Output combined denial JSON
    jq -n --arg reason "$combined_reason" '{
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": $reason
        }
    }'

    exit 2
elif [ ${#all_asks[@]} -gt 0 ]; then
    # At least one ask - output first ask JSON and exit 0
    echo "${all_results[0]}"
    exit 0
else
    # All allowed - output first result (should be allow)
    echo "${all_results[0]}"
    exit 0
fi
