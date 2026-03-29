---
name: document
description: Document information by splitting it into the correct categories (background, usage, arcana, shields, architecture, reasoning, skills) and writing it to the appropriate docs/ directories.
---

# Document

The user wants to document something. Your job is to categorize the information and write it to the correct locations per the documentation strategy in `docs/meta.md`.

## Step 1: Understand what's being documented

Ask the user (or infer from context) what they want to document. Gather the full picture before writing anything.

## Step 2: Read the documentation strategy

Read `docs/meta.md` to refresh on the category definitions and conventions.

## Step 3: Categorize

Split the information into the categories it belongs to. A single piece of knowledge often spans multiple categories. Present the split to the user for approval before writing.

The categories are:

1. **Background** — General knowledge needed to read code in this area.
2. **Usage** — How to interact with this feature correctly when writing code.
3. **Arcana** — Cross-cutting concerns with non-obvious effects elsewhere. Has a unique ID (initialism + Z suffix) and `@ID` references at affected code sites.
4. **Shields** — Enforceable rules/constraints. Has a unique ID (initialism + X suffix).
5. **Migration** — Ephemeral migration status, known Scala/Rust differences, workarounds.
6. **Architecture** — Internal design, data flow, invariants for modifying the feature itself.
7. **Reasoning** — Why the current approach was chosen over alternatives. Sub-category of architecture.
8. **Skills** — Step-by-step AI workflow methodology.
9. **Bugs** — Known bugs go as `#[ignore]`'d tests, not documents.
10. **Requirements** — Tests are requirements, not documents.

For each piece of information, identify:
- Which category it belongs to
- Which feature/directory it's closest to (determines which `docs/` directory it goes in)
- Whether it extends an existing doc or needs a new one

## Step 4: Check for existing docs

Before creating new files, check whether relevant docs already exist in the target `docs/` directories. Prefer extending existing docs over creating new ones.

## Step 5: Write the documents

For each category, write to the appropriate location:

- Single file: `docs/<category>.md`
- Multiple files: `docs/<category>/<topic>.md`

Follow the naming conventions from `docs/meta.md`.

### Arcana-specific steps

If any piece of information is an arcana (cross-cutting concern):

1. **Generate title and ID.** The title describes the concern plainly (does NOT contain the word "arcana"). The ID is an uppercase initialism of the title words with Z appended. Keep the acronym readable (4-10 letters before the Z). Present to user for approval.

2. **Create the arcana doc** at `<feature>/docs/arcana/<HammerCaseTitle>-<ID>.md` in the `docs/` directory of the feature that *causes* the cross-cutting effect:

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

3. **Find all relevant code sites.** Search the codebase for every place this arcana manifests: struct fields, code blocks, function signatures, comments. Use Grep, Glob, and Read. Be thorough — missing a site defeats the purpose.

4. **Add `@ID` references.** At each relevant site, add a comment referencing the arcana. The reference must always appear in a sentence:
   - `// Per @PPSPASTNZ, synthesize a constructor call as parser AST.`
   - `// Needed because postparser creates parser nodes (see @PPSPASTNZ)`

   Never write a bare `@ID` without a sentence. The sentence gives local context; the `@ID` tells readers where to find the full explanation.

### Shield-specific steps

If any piece of information is a shield (enforceable rule):

1. **Generate title and ID.** The ID is an uppercase initialism of the title words with X appended. Present to user for approval.

2. **Create the shield doc** at `<feature>/docs/shields/<HammerCaseTitle>-<ID>.md`.

## Step 6: Report

Tell the user:
- What categories the information was split into
- What files were created or updated, and where
- For arcana: how many code sites were annotated
- For shields: the ID created
