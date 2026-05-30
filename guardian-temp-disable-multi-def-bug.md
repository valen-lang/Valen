# Bug report: `guardian_temp_disable` can't disambiguate when a name resolves to multiple defs

**Reported:** 2026-05-29
**Repo:** Vale, mid-migration session
**Tool:** `mcp__guardian__guardian_temp_disable`

## What happened

JR tried to self-disable NCWSRX on an edit to `InstantiatedOutputsI` in `FrontendRust/src/instantiating/instantiator.rs`. The shield denial cited a verdict for that definition. `guardian_temp_disable` failed with:

```
Multiple definitions named 'InstantiatedOutputsI' ... Provide target_line to disambiguate
  (struct@85, impl@106, impl@179)
```

The tool's exposed schema has only `file_path`, `verdict_file`, `shield_file`, `reason` — **no `target_line` parameter**. So the caller can't supply the disambiguator the tool itself is asking for. The disable is unreachable from the JR session.

## Workaround used in-session

TL (ordained) ended up fixing the underlying slicing instead, which sidestepped the temp-disable. But the tool gap remains: any time the same name has a struct + multiple impls (a very common Rust shape), JR is stuck on shield-false-positives there until the def itself is restructured or the TL bypasses ordained.

## Suggested fix

Add an optional `target_line: Option<u32>` parameter to the MCP tool schema for `guardian_temp_disable`, passed through to the underlying Guardian CLI. When the verdict's cited line falls inside one of the candidate defs, the tool could also pick that one automatically (single-candidate via line range), only erroring out when truly ambiguous.

Either way: the current error message is honest about what's needed, but the tool surface doesn't let the caller provide it.
