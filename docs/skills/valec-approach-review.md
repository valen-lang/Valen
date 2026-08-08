---
name: valec-approach-review
description: Flag suspicious patterns when reviewing a plan document or FrontendRust code — new jargon absent from glossary.md, breaking an existing pattern, and reordering creation of things in ways that affect more than one function.
g_read_when: Read when reviewing a plan document or FrontendRust code, to flag suspicious patterns — new jargon, broken existing patterns, and creation-order changes that reduce decoupling.
g_mention_in:
  - CLAUDE.md
---

# Suspicious Patterns

Flag a human if you see any of these patterns in a plan document, or in code under review.

## Don't introduce jargon

If we're introducing a new word to the codebase, and it's not a general programming term, and no `glossary.md` file contains it, flag it to the human.

Example: Macros produce AHT denizens which seed the initial global env. But if we call them "seeds", that's jargon. It's not in a glossary.md, so flag it to the human. Better: GeneratedAhtDenizen, because "AHT" and "denizen" are already in glossary.md.

## Existing patterns

If it seems like there's an existing pattern to some code, and we're about to break it, **flag it to the human.**

## Preserve Existing Decoupling / Independence

If we change the order in which two things are created, in a way that affects more than just this one function, **flag it to the human.**

Example: The GlobalEnv was created fully before the CompilerOutputs. If we're about to change that, flag the human.

## Required Reading

 * Luz/skills/approach-review.md - Also look for the things described in there.
