#!/bin/bash
# Migrate .cursor/ to .claude/ directory structure

set -e

echo "Migrating .cursor/ to .claude/..."

# Create directories
mkdir -p .claude/rules
mkdir -p .claude/commands

# Function to convert rule frontmatter
convert_rule() {
    local input_file="$1"
    local output_file="$2"

    # Use awk to convert frontmatter
    awk '
    BEGIN { in_frontmatter = 0; frontmatter_done = 0 }
    /^---$/ {
        if (!frontmatter_done) {
            in_frontmatter = !in_frontmatter
            print
            if (!in_frontmatter) frontmatter_done = 1
            next
        }
    }
    in_frontmatter {
        # Convert globs to paths
        if ($0 ~ /^globs:/) {
            sub(/^globs:/, "paths:")
            print
            next
        }
        # Skip alwaysApply: false (default behavior)
        if ($0 ~ /^alwaysApply: false/) {
            next
        }
        # Keep description and other fields
        print
        next
    }
    { print }
    ' "$input_file" > "$output_file"
}

# Migrate rules
echo "Migrating rules..."
for rule_file in ../.cursor/rules/*.{md,mdc}; do
    if [ -f "$rule_file" ]; then
        filename=$(basename "$rule_file")
        echo "  - $filename"
        convert_rule "$rule_file" ".claude/rules/$filename"
    fi
done

# Migrate agents to commands
echo "Migrating agents to commands..."
for agent_file in ../.cursor/agents/*.md; do
    if [ -f "$agent_file" ]; then
        filename=$(basename "$agent_file")
        echo "  - $filename"
        cp "$agent_file" ".claude/commands/$filename"
    fi
done

echo ""
echo "Migration complete!"
echo ""
echo "Summary:"
echo "  - Rules copied to .claude/rules/ (frontmatter converted)"
echo "  - Agents copied to .claude/commands/"
echo ""
echo "Next steps:"
echo "  1. Review the migrated files"
echo "  2. Test with: /slice-pipeline or other commands"
echo "  3. Rules will auto-activate when editing matching files"
