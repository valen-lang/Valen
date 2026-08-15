---
name: good-doc
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

1. **Background** — General knowledge needed to read code in this area. Background docs must **as concise as possible** and should reference other docs for details rather than repeating information inline, because background docs are included in every prompt to every LLM, and they should keep noise to a minimum.
2. **Usage** — How to interact with this feature correctly when writing code.
3. **Arcana** — Cross-cutting concerns with non-obvious effects elsewhere. Has a unique ID (initialism + Z suffix) and `@ID` references at affected code sites.
4. **Shields** — Enforceable rules/constraints. Has a unique ID (initialism + X suffix).
5. **Architecture** — Internal design, data flow, invariants for modifying the feature itself. Architecture docs should also surface *where the feature is heading* — architecture is about evolution, not just the current snapshot. If there's a planned refactor or a target design the code is converging toward, mention it here with a link to the Reasoning doc that holds the details.
6. **Reasoning** — Why the current approach was chosen over alternatives, **and future plans** the code is not yet implementing. If a design has a known target shape that's deferred (pending benchmarking, pending a decision), it belongs here. Sub-category of architecture. Always cross-referenced from the relevant Architecture doc so readers discover the future plan while reading about the current design.
7. **Skills** — Step-by-step AI workflow methodology.
8. **Bugs** — Known bugs go as `#[ignore]`'d tests, not documents.
9. **Requirements** — Tests are requirements, not documents.

For each piece of information, identify:
- Which category it belongs to
- Which feature/directory it's closest to (determines which `docs/` directory it goes in)
- Whether it extends an existing doc or needs a new one

## Step 3b: Extract enforceable rules

After categorizing, actively ask: **"Is any part of this wisdom concrete and enforceable?"** Shields are the most durable form of documentation — they can't drift because Guardian checks them. Any time you learn something that could be a rule, propose it as a candidate shield.

Present to the user:
- What the candidate shield would enforce (one sentence)
- A proposed title and ID
- Whether it's checkable by Guardian (pattern in code reviews) or only by the compiler/tests

The user decides which candidates are worth making into shields. Don't silently categorize something as "just background" when it could also be an enforceable rule.

Examples of wisdom → shield extraction:
- "We learned that stringly-typed errors are hard to test" → Shield: `NoStringlyTypedData-NSTDX` — error types must use structured data, not string messages
- "Arena types shouldn't clone" → Shield: `ArenaTypesDontClone-ATDCX` — already exists
- "The compiler now requires explicit drop bounds" → Not a shield (enforced by compiler itself), just background/usage docs

## Step 4: Check for existing docs

Before creating new files, check whether relevant docs already exist in the target `docs/` directories. Prefer extending existing docs over creating new ones.

## Step 5: Write the documents

For each category, write to the appropriate location:

- Single file: `docs/<category>.md`
- Multiple files: `docs/<category>/<topic>.md`

Follow the naming conventions from `docs/meta.md`.

If it's arcana, follow the arcana-specific steps in the good-arcana skill.

### Shield-specific steps

If any piece of information is a shield (enforceable rule):

1. **Generate title and ID.** The ID is an uppercase initialism of the title words with X appended. Present to user for approval.

2. **Create the shield doc** at `<feature>/docs/shields/<HammerCaseTitle>-<ID>.md`.

## Step 6: Cross-references

After writing docs, add a `## See also` section with relative markdown links following the cross-reference chain defined in `docs/meta.md`:

- **Background** docs → link to relevant **Usage** docs
- **Usage** docs → link to relevant **Arcana** and **Shield** docs
- **Architecture** docs → link to relevant **Reasoning** and **Skill** docs

Only add links where related docs actually exist. Don't create empty See also sections.

## Step 7: Report

Tell the user:
- What categories the information was split into
- What files were created or updated, and where
- For arcana: how many code sites were annotated
- For shields: the ID created


## Required Reading
 * good-arcana
