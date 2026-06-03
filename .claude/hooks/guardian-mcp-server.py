#!/usr/bin/env python3
"""
Minimal MCP (Model Context Protocol) stdio server for Guardian temp-disable.

Exposes a single tool `guardian_temp_disable` that POSTs to the Guardian
HTTP server at http://127.0.0.1:7878/temp-disable.

Protocol: JSON-RPC 2.0 over stdin/stdout (one JSON object per line).
"""

import json
import os
import sys
import urllib.request
import urllib.error

GUARDIAN_PORT = os.environ.get("GUARDIAN_PORT", "7878")
GUARDIAN_URL = f"http://127.0.0.1:{GUARDIAN_PORT}/temp-disable"

TOOL_SCHEMA = {
    "name": "guardian_temp_disable",
    "description": (
        "Temporarily disable a Guardian shield check for a specific function. "
        "Use this after Guardian has denied your edit and you believe the denial "
        "is a false positive. You MUST cite the .verdict.md file path from the denial message; "
        "Guardian derives the definition name, shield code, and supporting artifact paths "
        "from that verdict file. "
        "Guardian will verify the denial happened and insert a temp-disable comment "
        "into the function's post-comment block. The human will review and remove "
        "temp-disables during code review. Your reason should be 1-3 sentences on one line. "
        "IMPORTANT: After calling this tool, you MUST re-read the file_path with the Read tool "
        "before making any further edits to it, because this tool modifies the file and the "
        "editor needs to see the updated contents (otherwise you'll get 'File has not been read yet')."
    ),
    "inputSchema": {
        "type": "object",
        "properties": {
            "file_path": {
                "type": "string",
                "description": "Absolute path to the source file containing the function"
            },
            "verdict_file": {
                "type": "string",
                "description": "Path to the .verdict.md file cited in the denial message"
            },
            "reason": {
                "type": "string",
                "description": "1-3 sentence explanation of why this is a false positive (single line, no newlines)"
            },
            "shield_file": {
                "type": "string",
                "description": "Path to the shield .md file from the denial message (the Shield: path)"
            },
            "target_line": {
                "type": "integer",
                "description": "1-based line number to disambiguate when multiple definitions share the same name in the file. Required when Guardian responds with 'Multiple definitions named ... Provide target_line to disambiguate'. The line number is encoded in the verdict filename as 'name--LINE.INDEX.ShieldName.verdict.md' (e.g. result--43.0.ScalaParityDuringMigration-SPDMX... → target_line=43)."
            }
        },
        "required": ["file_path", "verdict_file", "reason", "shield_file"]
    }
}


def send_response(response):
    line = json.dumps(response)
    sys.stdout.write(line + "\n")
    sys.stdout.flush()


def handle_initialize(msg):
    send_response({
        "jsonrpc": "2.0",
        "id": msg["id"],
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "guardian-temp-disable", "version": "0.1.0"}
        }
    })


def handle_initialized(msg):
    pass


def handle_tools_list(msg):
    send_response({
        "jsonrpc": "2.0",
        "id": msg["id"],
        "result": {"tools": [TOOL_SCHEMA]}
    })


def handle_tools_call(msg):
    params = msg.get("params", {})
    tool_name = params.get("name")

    if tool_name != "guardian_temp_disable":
        send_response({
            "jsonrpc": "2.0",
            "id": msg["id"],
            "result": {
                "content": [{"type": "text", "text": f"Unknown tool: {tool_name}"}],
                "isError": True
            }
        })
        return

    args = params.get("arguments", {})
    payload_dict = {
        "file_path": args.get("file_path", ""),
        "verdict_file": args.get("verdict_file", ""),
        "reason": args.get("reason", ""),
        "shield_file": args.get("shield_file", ""),
    }
    if "target_line" in args:
        payload_dict["target_line"] = args["target_line"]
    payload = json.dumps(payload_dict).encode("utf-8")

    try:
        req = urllib.request.Request(
            GUARDIAN_URL,
            data=payload,
            headers={"Content-Type": "application/json"},
            method="POST"
        )
        with urllib.request.urlopen(req, timeout=30) as resp:
            result = json.loads(resp.read().decode("utf-8"))
    except urllib.error.URLError as e:
        send_response({
            "jsonrpc": "2.0",
            "id": msg["id"],
            "result": {
                "content": [{"type": "text", "text": f"Failed to reach Guardian server: {e}"}],
                "isError": True
            }
        })
        return
    except Exception as e:
        send_response({
            "jsonrpc": "2.0",
            "id": msg["id"],
            "result": {
                "content": [{"type": "text", "text": f"Error: {e}"}],
                "isError": True
            }
        })
        return

    if result.get("success"):
        text = f"Temp-disable inserted: {result.get('inserted_line', '(unknown)')}"
        is_error = False
    else:
        text = f"Temp-disable failed: {result.get('error', '(unknown error)')}"
        is_error = True

    send_response({
        "jsonrpc": "2.0",
        "id": msg["id"],
        "result": {
            "content": [{"type": "text", "text": text}],
            "isError": is_error
        }
    })


HANDLERS = {
    "initialize": handle_initialize,
    "notifications/initialized": handle_initialized,
    "tools/list": handle_tools_list,
    "tools/call": handle_tools_call,
}


def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue

        method = msg.get("method", "")
        handler = HANDLERS.get(method)
        if handler:
            handler(msg)
        elif "id" in msg:
            send_response({
                "jsonrpc": "2.0",
                "id": msg["id"],
                "error": {"code": -32601, "message": f"Method not found: {method}"}
            })


if __name__ == "__main__":
    main()
