---
name: read-convo
description: Read a past session's convo doc together with the conversation it picked up from — resolve by stamp, follow the recorded lineage, read oldest first.
g_read_when: Read when the human asks to read a past session or convo plus the conversation that came before it.
g_mention_in:
  - CLAUDE.md
---

# Reading a Convo and What Came Before

Three steps: find the doc, find its predecessor, read them oldest first.

## 1. Find the doc by its stamp

Every auto-exported convo begins with a head stamp:

```
<!-- session: <uuid>; exported-bytes: <n>; transcript: <path>; opened: <a.md>, <b.md>; continues: <a.md> -->
```

Look it up by session id, whole or partial:

```bash
grep -l "session: 0a861114" docs/convos/*.md
```

**Never glob the filename.** Names now end in the full session id, but files exported before
that change don't, and the legacy hand-written docs carry no id at all. The stamp is the only
key that always works.

## 2. Find the predecessor

The two fields answer different questions, and they are not interchangeable:

- **`continues:`** — the conversation this session was picking up, as the human declared it
  via `/continue-session`. Authoritative. If it's there, you're done.
- **`opened:`** — every convo doc the session opened with `Read`, in read order. A record of
  what was consulted and *nothing more*. It deliberately does not say which one was the
  starting point, because reads can't establish that: a session that read an ancestor before
  its target looks identical to one that did the reverse.

So: use `continues:` when present. Otherwise fall back to `opened:` — and if it holds more
than one entry, say what they are and ask which was meant rather than picking. A single entry
is usually safe to treat as the predecessor, but say that's what you're assuming.

If neither field is set, read the doc's opening and see what the human asked for. When you
work it out, offer to record it — `/continue-session <id>` from a session continuing that
thread is what makes the answer stick.

## 3. Read them oldest first

Predecessor, then the target — chronological, so the later conversation reads as a
continuation rather than a flashback. Use `full-read`; these run to thousands of lines.

## Don't edit these files

Guardian rewrites every convo doc on each sweep and is the single writer. An annotation added
here is either clobbered or a race. Lineage gets recorded through `/continue-session`, which
goes to Guardian and comes back with what it actually linked.
