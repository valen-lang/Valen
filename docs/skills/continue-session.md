---
name: continue-session
description: Declare that this session picks up an earlier one, so the link is recorded on the convo docs where a later session can find it.
g_read_when: Read when the human says this session continues, resumes, or picks up an earlier session or convo.
g_mention_in:
  - CLAUDE.md
---

# Declaring a Continuation

Which earlier conversation this one is continuing cannot be inferred — it is a statement of
intent, not a trace of anything that happened. So the human says it, and this records it.

One request. Guardian owns `docs/convos/`, so it does the resolving and the writing:

```bash
curl -s -X POST http://localhost:${GUARDIAN_PORT}/continue-session \
  -H 'Content-Type: application/json' \
  -d '{"session_id": "${CLAUDE_SESSION_ID}", "predecessor": "<what the human named>"}'
```

`predecessor` is whatever the human typed — a full session id, or just the leading segment
they read off a filename. **Don't resolve it yourself.** Guardian matches it against the
stamps and reports what it found, which is the difference between a link you confirmed and
one you assumed.

## Reporting back

- **`{"success": true, ...}`** — name both docs from `convo` and `predecessor_convo`. Say
  what was linked, not merely that something was.
- **`{"success": false, "error": ...}`** — relay it. An ambiguous prefix lists the candidate
  docs by name: ask which one, then retry with a longer prefix. An unknown predecessor
  usually means that session was never exported to `docs/convos/`.

## Two things to know

**It works at turn one.** If this session has no convo doc yet — the usual case, since the
exporter sweeps every 30s — Guardian mints one instead of making you wait.

**Never edit `docs/convos/` yourself.** Guardian rewrites those files on every sweep and is
the single writer; anything written from here is either clobbered or a race. That is the
whole reason this goes through an endpoint.
