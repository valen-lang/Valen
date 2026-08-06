---
name: continue-session
description: Declare that this session picks up an earlier one — record the link on the convo docs, then read back the named number of predecessors plus the handoff they left.
g_read_when: Read when the human says this session continues, resumes, or picks up an earlier session or convo.
g_mention_in:
  - CLAUDE.md
---

# Declaring a Continuation

Which earlier conversation this one is continuing cannot be inferred — it is a statement of
intent, not a trace of anything that happened. So the human says it, this records it, and then
the session catches up.

## Invocation grammar

- **`/continue-session <predecessor> reading <n>`** — record the link, then read the `n`
  conversations leading up to this one: `reading 1` is the predecessor, `reading 2` adds its
  predecessor, and so on up the chain.
- **`reading 0`** — record and read nothing, for when the context is already live here.

`<predecessor>` is whatever the human typed — a full session id, or the leading segment off a
filename.

**No depth given? Ask "How many conversations back?" before doing anything.** Never default.
Reading can't be undone, and at turn one two convo docs can be most of the budget; too few
costs a follow-up, too many costs the session.

## 1. Record the link

One request. Guardian owns `docs/convos/`, so it does the resolving and the writing:

```bash
curl -s -X POST http://localhost:${GUARDIAN_PORT}/continue-session \
  -H 'Content-Type: application/json' \
  -d '{"session_id": "${CLAUDE_SESSION_ID}", "predecessor": "<what the human named>"}'
```

**Don't resolve `predecessor` yourself.** Guardian matches it against the stamps and reports
what it found, which is the difference between a link you confirmed and one you assumed.

### Reporting back

- **`{"success": true, ...}`** — name both docs from `convo` and `predecessor_convo`. Say
  what was linked, not merely that something was.
- **`{"success": false, "error": ...}`** — relay it. An ambiguous prefix lists the candidate
  docs by name: ask which one, then retry with a longer prefix. An unknown predecessor
  usually means that session was never exported to `docs/convos/`.

## 2. Read the conversations, oldest first

Skipped entirely on `reading 0`.

Guardian's `predecessor_convo` is the first name. For depths past 1, walk the chain — each
doc's head stamp carries a `continues:` naming the one before it — and collect all `n` names
before reading any.

Read them **oldest first**, deepest ancestor through immediate predecessor, so each reads as a
continuation rather than a flashback. Use `full-read`; these run to thousands of lines.
IMPORTANT: YOU MUST READ full-read.md BEFORE READING THESE CONVERSATIONS. Follow what they say.

If the chain runs out early, read what you found and say where the trail went cold. Don't pad
the count from `opened:` — that records what a session consulted, not what it continued.

## 3. Read the handoff they were working with

After the conversations, never before: a handoff states what is true *now*, so it wants to be
freshest, and it reads thin without the history behind it.

Which handoff comes from the newest convo doc, not a guess:

```bash
grep -oiE '[A-Za-z0-9_/.-]*handoff[A-Za-z0-9_/.-]*\.md' docs/convos/<newest-convo>.md | sort -u
```

`vcoord-handoff.md` at the repo root is the long-lived one; `investigations/*_handoff.md` are
per-thread. Several hits — read the one being edited, name the rest. No hits — say so rather
than falling back to the root handoff.

## Two things to know

**It works at turn one.** If this session has no convo doc yet — the usual case, since the
exporter sweeps every 30s — Guardian mints one instead of making you wait.

**Never edit `docs/convos/` yourself.** Guardian rewrites those files on every sweep and is
the single writer; anything written from here is either clobbered or a race. That is the
whole reason this goes through an endpoint.

## Required reading

 * update-handoff
 * full-read
