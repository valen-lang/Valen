# Bug: SPDMX (and other LLM shields) time out on real requests due to OS TCP idle timeout

## Summary

LLM shield checks consistently fail with "Failed to read OpenRouter response body / operation timed out" on real hook requests. The underlying cause is that the OS TCP idle timeout (~30s) fires before the model finishes generating a response, because `openrouter_request` in Rabble uses a blocking HTTP client with no explicit read timeout and no streaming.

## Symptoms

Each retry attempt times out at ~30–35 seconds:

```
[LLM] Sending to OpenRouter (model: deepseek/deepseek-v4-pro)...
[LLM] OpenRouter error: Failed to read OpenRouter response body   ← ~33s later
[LLM] Retry attempt 2 ...
[LLM] Sending to OpenRouter (model: deepseek/deepseek-v4-pro)...
[LLM] OpenRouter error: Failed to read OpenRouter response body   ← ~33s later
...
```

Three retries all fail. Guardian denies the edit.

## Root Cause (Known So Far)

`Guardian/Rabble/src/openrouter.rs` creates a `reqwest::blocking::Client::new()` with no timeout and no streaming. The OpenRouter server sends back HTTP 200 headers immediately, then the model streams tokens — but since we're not using `stream: true`, the server holds the body open until the full response is ready. When the model takes longer than the OS TCP idle timeout (~30s), the OS kills the idle connection mid-read, and reqwest surfaces it as "error decoding response body: operation timed out".

## What We Tested vs. What Guardian Actually Sends

This is the key unknown. All manual curl reproduction attempts used `data_substituted.txt` as the prompt — but that file still contains `{{shield_rule}}` as a literal unfilled placeholder (see `unsubstituted-bug.md`). The actual Guardian request substitutes the full shield rule text into the prompt before sending.

As a result, our curl tests used a much smaller/simpler prompt than what Guardian sends:

| Request | Prompt size | Result |
|---|---|---|
| curl with `data_substituted.txt` (unfilled) | ~4K tokens | Works: 12s |
| Guardian hook (full shield rule substituted) | Unknown — larger | Times out: ~33s |

We don't yet know whether the real prompt is inherently too large for these models to answer in <30s, or whether something in the full prompt structure is causing models to hang specifically (e.g. the JSON schema enforcement interacting badly with a large system prompt).

## Repro Steps (Current State — Incomplete Prompt)

The following reproduces the timeout using `check-direct`. Note: this uses the unfilled `data_substituted.txt` prompt (missing `{{shield_rule}}`), so it is not a faithful reproduction of the real failure. These steps will need to be updated once `data_substituted.txt` is fixed.

**Trigger a hook failure by making a test edit:**

```bash
# Add a deliberate comment to trigger SPDMX, then let Guardian reject it
# (edit evaluate_lookup_for_load in expression_compiler.rs)
```

**Replay the failed shield check in isolation:**

```bash
OPENROUTER_API_KEY=$(cat Guardian/api_key.txt) \
Guardian/target/debug/guardian check-direct \
  --input FrontendRust/guardian-logs/request-003-1778730305822/hook-003/evaluate_lookup_for_load--201.0.contextified_diff.txt \
  --referenced-defs FrontendRust/guardian-logs/request-003-1778730305822/hook-003/evaluate_lookup_for_load--201.0.referenced_defs.txt \
  --file-path FrontendRust/src/typing/expression/expression_compiler.rs \
  --config FrontendRust/guardian.toml \
  --mode guard_mode \
  --check-filter SPDMX \
  --cache-dir /tmp/guardian-cache-repro \
  --log-dir /tmp/guardian-repro-out \
  --format text \
  --log-level trace
```

**Reproduce the timeout directly with curl (using the unfilled prompt):**

```bash
PROMPT=$(cat "FrontendRust/guardian-logs/request-003-1778730305822/hook-003/evaluate_lookup_for_load--201.0.ScalaParityDuringMigration-SPDMX.data_substituted.txt")
curl -s -w "\n\nHTTP_STATUS:%{http_code} TIME:%{time_total}s" \
  -X POST https://openrouter.ai/api/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $(cat Guardian/api_key.txt)" \
  --max-time 60 \
  --data-binary @- << EOF
{
  "model": "deepseek/deepseek-v4-pro",
  "messages": [
    {"role":"user","content":$(jq -Rs '.' <<< "$PROMPT")},
    {"role":"user","content":"Please respond with only valid JSON matching the required schema. Do not include any text before or after the JSON."}
  ],
  "response_format": {
    "type": "json_schema",
    "json_schema": {
      "name": "response",
      "strict": true,
      "schema": {
        "type": "object",
        "properties": {
          "violations": {"type": "array", "items": {"type": "string"}},
          "verdict": {"type": "string"}
        },
        "required": ["violations", "verdict"],
        "additionalProperties": false
      }
    }
  }
}
EOF
```

With the unfilled prompt (`{{shield_rule}}` still a literal string), this returns in ~12s. With the real fully-substituted prompt it times out at ~33s. The gap is the bug.

**Check which model and provider Guardian used:**

```bash
head -5 FrontendRust/guardian-logs/request-003-1778730305822/hook-003/log.evaluate_lookup_for_load--201.0.ScalaParityDuringMigration-SPDMX.vote0.log
```

## Planned Next Steps

1. **Fix `data_substituted.txt`** — save the fully-substituted prompt (after `{{shield_rule}}` is filled in) so we have the exact bytes Guardian sends to the LLM. See `unsubstituted-bug.md`.

2. **Use the real prompt in curl** — replay the actual request against OpenRouter directly, with timing. This gives us ground truth on how long the model actually takes with the real prompt.

3. **Identify the real failure** — if the model responds in <30s with the real prompt, the issue is a bug in Guardian/Rabble (wrong client config, wrong headers, connection reuse problem, etc.). If it takes >30s, the issue is the OS idle timeout and the fix is streaming.

4. **Fix the root cause** — either fix whatever in Guardian is causing the hang, or add streaming to `openrouter_request` to keep the TCP connection alive during generation.
