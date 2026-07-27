---
name: update-handoff
description: Rules for editing a long-lived handoff doc — edit rather than annotate, keep no history, verify before writing.
g_read_when: Read when editing `vcoord-handoff.md` or any other long-lived handoff or plan doc.
g_mention_in:
  - CLAUDE.md
---

This file must be short. Edits to this file must be 1-2 sentences max.

# Updating a Handoff

A handoff states **what is true now**. History belongs in `git log` and the convo exports.

## Edit, never annotate

Delete stale text outright. No `~~strikethrough~~`, no "SUPERSEDED", no "corrected 2026-07-27", no
tombstone explaining what you removed — the reader never saw it. When a section's subject is done,
delete the section, not just its contents. Rewrite each claim in place so the doc reads as though the
current answer was always the answer.

## No dates, no logs, no process

Write in the present tense without timestamps. The one exception is a fact that *is* about time:
"design-1 as of 2026-07-26" earns its date because a newer version may differ.

Never append a "what landed today" section — update the plan the work belonged to. Cut who found it,
which agent investigated, and how many rounds it took; keep a name only if the reader must route a
question back to it. Commit hashes only for archaeology ("this was a regression from X, don't
resurrect it"), never as a record of what landed.

## Verify before you write

Grep every claim you touch. Handoffs rot silently, and the checks are seconds long. Real examples
from one audit: a conversion recipe for `KindT::new` sites when `KindT::new` had zero live lines; a
deleted function named as the live query, killed by a landing recorded twenty lines away in the same
file; re-implementation notes for a module with zero hits anywhere in the tree.

## Say whether a claim is about the design or the code

A handoff carries both, and a mechanism "retired" by ruling but still live in the tree will be read
as gone. Name the layer whenever they disagree — that gap is usually unlisted work.

## Cite an unambiguous path plus a symbol in the file

Write "fn add_instantiation_bounds in compiler_outputs.rs", not "compiler_outputs.rs:261". If there
are multiple files with the same name, or multiple symbols by that name in the file, then
disambiguate. The same applies to documents you cite: a line number into a living spec rots exactly
like a `file:line` into moving code, so quote the passage and let the quote be the pointer.

## Numbers are measured or absent

Run the suite before quoting a count; likewise for cluster sizes, LOC estimates, and site counts. If
you won't measure, write "measure before quoting" rather than a stale figure.

## One fact, one home

A fact stated twice will eventually contradict itself. If two sections need it, one gets a pointer.
Watch for rival "what to do next" sections — more than one means all but one are wrong.

## Every addition implies a deletion

New knowledge usually falsifies something. After adding, go find what just became untrue. An update
that only adds is suspect.

## Keep a Lessons Learned section

Every handoff needs one. It is the one section that accumulates rather than being replaced — but it
accumulates *wisdom*, not events. **One or two sentences per entry**, three kinds:

- **Traps** — landmines that cost time and will again. *"A name-based sweep deleted `implements`
  because it was spelled `Coord*Isa`; check what a symbol is before trusting what it's called."*
- **Architect preferences, generalized.** When the architect states a preference in general terms,
  record the general form rather than the instance — *"don't treat non-generics as special cases"*
  outlives the one `if args.is_empty()` it was said about.
- **Recurring agent mistakes**, stated plainly and without flagellation. *"I reason from current code
  as though it were the target, in a tree that is mid-migration."*

Prune entries that stop being live. An entry nobody could act on is noise.

## A trap is not a correction

The only reason to mention a wrong belief is that the reader will re-derive it independently. Write
it forward — "do not conclude X from the code, because Y" — never backward as "we thought X and were
wrong."

This is also the one exception to "delete outright." The test is not *did the reader see it* but
**will the reader go looking for it** — from a stale branch, another doc, or an obvious wrong guess.
If yes, keep one forward-facing sentence: *"Do not plan against cluster X; it fails zero tests now."*
If no, delete and say nothing.

## Numbered things are cited elsewhere

Grep for citations before deleting or renumbering a numbered item. A number that has escaped the doc
— into a convo log, another session's mail — is frozen; renumbering breaks references you cannot sweep.

## Prefer the command over the fact

Branch tips, what is uncommitted, a suite count — these rot however present-tense you write them.
Give the command that regenerates them, plus the sentence saying why they matter.

## The test

If you can't state it in the present tense as a fact about the code or the design, it belongs in a
commit message or a convo log.
