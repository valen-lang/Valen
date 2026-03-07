#!/bin/bash
# Sends hook input to the writecop server and outputs the response.
# The writecop server must be running on port 7878.

RESPONSE=$(curl -s -X POST http://127.0.0.1:7878/validate \
    -H "Content-Type: application/json" \
    -d @- 2>&1)

if [ $? -ne 0 ]; then
    # Server not reachable - allow the edit (don't block if server is down)
    echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":"writecop server not reachable"}}'
    exit 0
fi

echo "$RESPONSE"

# Check if the response contains a deny decision
if echo "$RESPONSE" | grep -q '"permissionDecision":"deny"'; then
    exit 2
fi

exit 0