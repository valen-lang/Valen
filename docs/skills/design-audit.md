---
name: design-audit
description: Audit an implementation against its `*-design.md` — fan out read-only agents per code slice, report precise deviations, and verify the load-bearing ones yourself.
g_read_when: Read when auditing an implementation against a `*-design.md` to find where the code deviated from the spec.
g_mention_in:
  - CLAUDE.md
---

# Auditing Code Against Its Design

Find every place the code differs from its `*-design.md`. Report each with a `file:line`, and rank by
how bad it is.

## Steps

1. Read the whole design doc first. It is what you check against. Note which parts are ratified (the
   Design section) and which are only proposals — the code should follow the ratified parts only.
2. Split the code by file or area. Send one read-only agent per part. Give each agent:
   - the matching piece of the spec, pasted in
   - a rule: read only, change no files, scratch in `/tmp`, run no builds
   - a request to list both matches and gaps, each as `file:line`, what the spec says, what the code
     does, and a severity: cosmetic / naming / structural / semantic / nondeterminism
3. Check the important findings yourself — do not just trust the agents. Re-check anything that
   contradicts an earlier "done" claim, touches safety or nondeterminism, or is the biggest structural
   gap.
4. Write one ranked list. Put the architecture and safety findings first, cosmetic ones last.

## What to look for

- a spec'd feature that is missing — the worst kind, because passing tests hide it
- a proposal built as if it were ratified
- something in the code that the doc never mentions
- nondeterminism: std `HashMap`/`HashSet`, or pointers turned into integers (a live leak that reaches
  the output is worse than a lookup-only use)
- a different shape than the spec: a small IR where the spec wants a full one, missing or renamed
  variants, dropped parameters, dead lifetimes
- `todo!` / `panic!` / `unimplemented!`, and any comment admitting a gap

## Say who is wrong, for each finding

- The code breaks a correct spec → fix the code.
- The code is right and the spec is wrong or unclear → fix the doc.

Mark every finding as one or the other. A "code is right, doc is wrong" finding still counts: the doc
is the source of truth, so a wrong doc is a real defect even when the code worked around it.

## Recheck earlier claims

A past "this slice passes" is not proof the spec's case is covered. Re-read the test against the exact
case the spec means. A test on an easy case (a named local) can pass while the spec's real case (an
unnamed temporary) never runs. Findings that contradict a past "done" are the most valuable — check
them yourself.
