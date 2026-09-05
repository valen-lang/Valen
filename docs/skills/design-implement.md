---
name: design-implement
description: Implement code against a `*-design.md` document — the design doc is living and absolute, so stop and get it fixed before deviating from it in any way that outlives the current plan.
g_read_when: Read when implementing code against a `*-design.md` design document — turning a design-assistant doc into code.
g_mention_in:
  - CLAUDE.md
---

# Implementing Against a Design Document

You are turning a `*-design.md` into code (see `design-assistant`). The doc is the source of truth.
Everyone after you trusts the doc over the code.

## Two priorities, in order

1. Keep the design doc correct and clean.
2. Make the code work.

**Priority 1 always beats priority 2.** Working code under a wrong doc is worse than nothing, and dangerous. The next
person reads the doc, believes it, and ships the mistake. This already happened once: the code skipped
a safety check the spec asked for, a weak test passed, and it shipped as "green", which was very dangerous.

## Stop before you deviate

The moment your code would differ from the spec, stop and tell the human. Do not build the different
thing quietly.

**STOP means a full halt.** Do not change the plan or the design doc yet. Do not start another task,
and do not work on a different part of the code, while this is pending. Wait for the human. Get their
approval before you touch the plan, and again before you resume coding. Then the human either changes
the doc or tells you to match it.

"Differ" means any of these:

- a different shape than the spec
- a step the spec asks for that you would skip
- something the spec does not mention that you would add
- a proposal that is not ratified yet
- a construct the codebase bans, even if the spec sketched it

The only exception: a difference the plan itself writes down, and only when the plan says where —
before the end — it gets erased. Hit a new difference while coding? Add it to the plan. Anything that
outlives the plan, or that the plan never records, is a stop.

Write it in the plan, not a code comment — **a code comment is not enough.** This is for
auditability: when someone finds the difference later, the plan already names its intended fix.

## A confusing spec is a stop — never guess

**Principle:** if the spec is wrong, unclear, or contradicts itself, stop and ask the human to fix the
doc. Never pick a reading yourself, even one that looks obviously right.

**Why:** you see one part of the codebase. Your fix might not fit the rest. Choosing for yourself is
cowboy coding.

BEFORE: The spec says "invalidate every descendant" and also "do not invalidate `map.size`." You pick
the second, code it, move on.

AFTER: You stop, point out the contradiction, and ask the human to fix the doc. Then you build what
the fixed doc says.

## A passing test is not proof

Make the test cover the exact case the spec describes. If the spec says "check an unnamed temporary,"
a test on a named local does not count. Calling that case done hides the gap.

## Keep the doc in sync as you go

Whenever the code and the doc drift apart for good, stop and fix the doc first. If an audit later
finds many gaps, the doc was left to rot while you coded — the failure this skill prevents.

## Required reading

 * design-assistant
