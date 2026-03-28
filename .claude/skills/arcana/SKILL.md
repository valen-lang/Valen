---
name: arcana
description: Document a cross-cutting concern ("arcana") — create a zen/ doc, generate an ID, and add @ID references at all relevant code sites.
---

# Arcana Documenter

The user has identified a cross-cutting concern — something local that affects the codebase in non-obvious ways. Your job is to document it and mark every relevant code site.

## Step 1: Understand the arcana

Ask the user (or infer from context) what the cross-cutting concern is:
- What is the local thing?
- What does it affect across the codebase?
- Why does it exist?

## Step 2: Generate the title and ID

The title describes the concern plainly. The ID is an acronym of the title words, plus a Z suffix.

Example: "PostParser Synthesizes Parser AST Nodes" → `PPSPASTNZ`

Rules:
- The title does NOT contain the word "arcana"
- The ID is uppercase, formed from initial letters of title words, with Z appended
- Keep the acronym readable (4–10 letters before the Z)

Present the title and ID to the user for approval before proceeding.

## Step 3: Create the zen/ document

Write `FrontendRust/zen/<HammerCaseTitle>-<ID>.md`.

HammerCase means each word is capitalized and concatenated with no separators, e.g., `PostParserSynthesizesParserASTNodes-PPSPASTNZ.md`.

The document structure:

```markdown
# <Title> (<ID>)

<One-paragraph description of what this arcana is.>

## Where

<Which files/areas of the codebase are involved.>

## Cross-cutting effect

<What the non-obvious impact is and why it matters.>

## Why it exists

<Motivation — why this pattern was chosen over alternatives.>
```

Add additional sections if needed for the specific arcana.

## Step 4: Find all relevant code sites

Search the codebase for every place this arcana manifests. This includes:
- Struct fields that exist because of it
- Code blocks that implement the pattern
- Function signatures affected by it
- Comments that would be confusing without context

Use Grep, Glob, and Read to find these sites. Be thorough — missing a site defeats the purpose.

## Step 5: Add @ID references

At each relevant site, add a comment referencing the arcana. The reference must always appear in a sentence:

- `// Per @PPSPASTNZ, synthesize a constructor call as parser AST.`
- `// Needed because postparser creates parser nodes (see @PPSPASTNZ)`

Never write a bare `@ID` without a sentence. The comment should make sense to someone who hasn't read the arcana doc — the sentence gives local context, and the `@ID` tells them where to find the full explanation.

## Step 6: Report

Tell the user:
- The ID and filename created
- How many code sites were annotated and where
