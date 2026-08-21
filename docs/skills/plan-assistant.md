---
name: plan-assistant
description: How to work with the human on editing a `-plan.md` plan document.
g_read_when: Read when the human wants to work with you to edit a markdown ending in -plan.md.
g_mention_in:
  - CLAUDE.md
---

## The Plan Document

We'll have a plan document.

 * While we're working on it, it will be committed to docs/plans, ending with "-plan", no number. Example: docs/plans/generics-solving-plan.md.
 * It will be committed to docs/historical when it's done.
 * This doesn't replace the handoff document. But the handoff document shouldnt contain anything that would be better in here.
 * To check what the human changed, use **verbatim** the command `diff -U2 <(sed -n '1,/^## Strategic Directions Proposals/p' /tmp/plan-phased-calls-0.md) <(sed -n '1,/^## Strategic Directions Proposals/p' docs/plans/plan-phased-calls.md) || true; cp docs/plans/plan-phased-calls.md /tmp/plan-phased-calls-1.md`, updating the 0 and 1 as you go. If the human says only "read", they might mean this.

Sections:

**"Strategic Directions (human-only)" section:**

 * Only I (the human) can edit it.
 * It tracks the plan's high-level requirements and direction as I understand them.
 * I'll number the paragraphs S1, S2, etc. and never re-number them. Gaps will happen, that's ok.
 * When I un-ratify something, I'll delete it from the Strategic Directions entirely, and you'll delete it (and any derived plans) from the Plan Details. When I say I un-ratified something, please check that I edited the doc.
 * When you notice something in this section that seems wrong, **please tell me.**
 * When I edit this section, remove anything from "Strategic Directions Proposals" that is now redundant or inconsistent.

**"Strategic Directions Proposals" section:**

 * You add things to this section as you understand them, concisely.
 * As we go, I'll "ratify", in other words, move things to the top section.
 * We won't consider it "part of the plan" until it's ratified.
 * Continue the S1/S2/S3/etc naming scheme.
 * Every item in this section should be short, effective, and to the point.
 * Every time the user says something, if it's not already specified by the Strategic Directions, it should be added to the top of the strategic directions proposals, in roughly the same amount of words as the user used.

**"Plan Details" section:**

 * You can modify this section.
 * It describes how we'll carry out the top section's items.
 * It's okay if it's empty for a while while we're still designing.
 * Everything in this section should be roughly derivable from the Strategic Directions, and anything that seems like a large decision that affects things should be moved to Strategic Directions Proposals instead of living only in the Plan.
 * Every item in the plan should name the paragraph of the Strategic Directions that it's derived from.

**"Discussed examples and test cases" section:**

 * You can modify this section.
 * Write in it any examples or test cases that we explicitly talked through, or you specifically explored.
 * Don't eagerly try to fill this out, just put in here what you happen to think through, or what we happen to talk about.

**"Background and Current State" section**:

 * You can modify this section.
 * It should have information on the compiler as it currently is, that factors into the plan.
 * This section can (and often should be!) be very large.
 * Every part should have references, so that a sub-agent can doublecheck/verify things in there. References will be inline, with the code file + symbol name, or markdown document file path + the date that part of the markdown document was last updated.

**"Open Questions" section:**

 * You can modify this section.
 * If it's an open question that has a factual answer from the code/docs, then when answered it moves to the "Background and Current State" section in the handoff.
 * If it's an open question about which way to go, its answer should be promoted to "Strategic Directions Proposals".
 * There should be no closed questions in the open questions section.

## Two-phase Communication

We'll use "two-phase communication" throughout this entire process, please see the two-phase skill.

## Required reading

 * prose-reviewer
 * update-handoff
 * two-phase
