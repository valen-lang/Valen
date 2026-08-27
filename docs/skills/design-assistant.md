---
name: design-assistant
description: How to work with the human on editing a `-design.md` design document.
g_read_when: Read when the human wants to work with you to edit a markdown ending in -design.md.
g_mention_in:
  - CLAUDE.md
---

## The Design Document

We'll have a design document.

 * This doc is intended to be a living doc, serving as the source of truth for a codebase.
 * This doesn't replace the handoff document. But the handoff document shouldnt contain anything that would be better in here.
 * To check what the human changed, use **verbatim** the command `diff -U2 <(sed -n '1,/^## Design Proposals/p' /tmp/plan-phased-calls-0.md) <(sed -n '1,/^## Design Proposals/p' docs/plans/plan-phased-calls.md) || true; cp docs/plans/plan-phased-calls.md /tmp/plan-phased-calls-1.md`, updating the 0 and 1 as you go. If the human says only "read" or "diff", they might mean this.
 * The first thing you should do is run the above command to establish a version 0 baseline.

Sections:

**"Design (human-only)" section:**

 * Only I (the human) can edit it.
 * It tracks the plan's high-level requirements and direction as I understand them.
 * I'll number the paragraphs S1, S2, etc. and never re-number them. Gaps will happen, that's ok.
 * When I un-ratify something, I'll delete it from the Design entirely, and you'll delete it (and any derived plans) from the Plan Details. When I say I un-ratified something, please check that I edited the doc.
 * When you notice something in this section that seems wrong, **please tell me.**
 * When I edit this section, remove anything from "Design Proposals" that is now redundant or inconsistent.

**"Design Proposals" section:**

 * You add things to this section as you understand them, concisely.
 * As we go, I'll "ratify", in other words, move things to the top section.
 * We won't consider it "part of the plan" until it's ratified.
 * Continue the S1/S2/S3/etc naming scheme.
 * Every time the user says something, if it's not already specified by the Design, it should be added to the top of the Design Proposals, in roughly the same amount of words as the user used.

After you write a proposal, do an "editing pass" on it to rewrite it to better adhere to prose-reviewer and the below rules:

 * Every item in this section should be short, effective, and to the point.
 * This section should **only** contain the desired state as a consistent specific point in time. It should **not** contain details for how we'll get there. If necessary, those can go into Details.
 * Aggressively modify or delete any outdated proposals. This section should **not** contain outdated proposals. Every proposal should be kept up to date. No "refined by other proposals" or "supersedes" or "superseded by" markers.
 * Every proposal should be stated simply and clearly. No fluff, no redundancy.
 * Only one sentence of motivation or context is allowed.

**"Details" section:**

 * You can modify this section.
 * It describes how we'll carry out the top section's items.
 * It's okay if it's empty for a while while we're still designing.
 * Everything in this section should be roughly derivable from the Design, and anything that seems like a large decision that affects things should be moved to Design Proposals instead of living only in the Plan.
 * Every item in the plan should name the paragraph of the Design that it's derived from.

**"Discussed examples and test cases" section:**

 * You can modify this section.
 * Write in it any examples or test cases that we explicitly talked through, or you specifically explored.
 * Don't eagerly try to fill this out, just put in here what you happen to think through, or what we happen to talk about.

**"Background" section**:

 * You can modify this section.
 * It should have information that is useful to know to understand the Design section.
 * This section should contain three lists:
    * A "Self-evident from the code" list.
       * Each item should reference the code file + symbol name, so that future sub-agents can doublecheck/verify things in there.
    * A "Documented" list, of information from existing documentation.
       * Each item should reference the markdown document file path + the date that part of the markdown document was last updated, so that a sub-agent can doublecheck/verify things in there.
    * An "Undocumented" list, of information not from existing documentation.
   Each item in these lists should be 1-2 sentences max.

**"Open Questions" section:**

 * You can modify this section.
 * If it's an open question that has a factual answer from the code/docs, then when answered it moves to the "Background and Current State" section in the handoff.
 * If it's an open question about which way to go, its answer should be promoted to "Design Proposals".
 * There should be no closed questions in the open questions section.

## Two-phase Communication

We'll use "two-phase communication" throughout this entire process, please see the two-phase skill.

## Required reading

 * prose-reviewer
 * update-handoff
 * two-phase
