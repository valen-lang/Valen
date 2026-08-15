---
name: good-arcana
description: Write an arcana doc for a cross-cutting concern — generate a title and ID, draft and get approval, create the doc, find all code sites, and add @ID references.
g_read_when: Read when writing or adding an arcana doc for a cross-cutting concern.
g_mention_in:
  - CLAUDE.md
---

# Good Arcana

If any piece of information is an arcana (cross-cutting concern):

1. **Generate title and ID.** The title describes the concern plainly (does NOT contain the word "arcana"). The ID is an uppercase initialism of the title words with Z appended. Keep the acronym readable (4-10 letters before the Z). Present to user for approval.

2. **Draft the arcana text and get a second approval.** Once the user approves the title and ID, write the tentative arcana doc body inline in chat (not to disk yet) and ask the user to approve the text before it's written to a file. The user may want to tweak wording, add nuance, or cut fluff. Only after they approve the drafted text do you move on to step 3.

3. **Create the arcana doc** at `<feature>/docs/arcana/<HammerCaseTitle>-<ID>.md` in the `docs/` directory of the feature that *causes* the cross-cutting effect. HammerCase with the initialism at the end, like `PostParserSynthesizesParserASTNodes-PPSPASTNZ.md`. Include information such as: a brief description of the concept, at least one example concisely illustrating it, why the concept exists, and what its cross-cutting effect is. If there are other arcana that it affects or is affected by it, mention those as part of regular prose (not as an extra section). Notes:
   * It should be concise. Don't include fluff. Don't be redundant. Get to the point.
   * Instead of long paragraphs, feel free to break things up with newlines.
   * It should be one markdown section, it should not have subsections headers. If it must be long enough that subsections are needed, feel free to use bold lines like, `**Interactions with IDKWTHI:**`.
   * Don't add a section framed around the reader or the effect-as-a-topic — no `**Cross-cutting effect:**`, no `**How this affects call-sites/readers**`, no "why a reader trips on this," "this looks surprising but," etc. The whole arcana already *is* the cross-cutting effect, so state it as plain facts about the code (what holds at each affected site), woven into the prose. By the time someone reads the arcana their confusion is already resolved — they don't need to be told they were ever confused. (This is prose-reviewer's "state the rule, not the reader's reaction.")
   * **Focus on *why*, not *what*.** The arcana's job is to explain the strategic reason the code behaves this way — the design invariant, the trade-off, the concern that drives this behavior. It's fine to anchor the reader with a function or type name, but don't narrate tactical implementation: specific call chains, control-flow sequences, "which branch runs when," step-by-step mechanics. Readers come to the arcana for the *why*; they can read the code for the *what*. Tactical narration also dates fast — which is the stronger form of the no-line-numbers rule below.
   * Do NOT reference file/line numbers (e.g. `function_compiler.rs:194`). Code moves around constantly and line-anchored references go stale fast. Refer to code by concepts, function names, type names, or module/file names only — readers can find the current location by searching for those. The `@ID` markers added to code sites in step 5 are the reverse pointer; the arcana doc doesn't need to point back at specific lines.

4. **Find all relevant code sites.** Search the codebase for every place this arcana manifests: struct fields, code blocks, function signatures, comments. Use Grep, Glob, and Read. Be thorough — missing a site defeats the purpose.

5. **Add `@ID` references.** At each relevant site, add a comment referencing the arcana. The reference must always appear in a sentence:
   - `// Per @PPSPASTNZ, synthesize a constructor call as parser AST.`
   - `// Needed because postparser creates parser nodes (see @PPSPASTNZ)`

   Never write a bare `@ID` without a sentence. The sentence gives local context; the `@ID` tells readers where to find the full explanation. Add references in code as comments, and add references to other documentation and other arcana where relevant.

   **Keep code-comment references concise.** Preferably one sentence. Ideally one line. The arcana doc is the place for the full explanation — the comment just needs to tell the reader "this is an instance of `@ID`, go read it" plus whatever local context is genuinely needed to understand what *this* site is doing. If you find yourself writing a three-line comment explaining the arcana again, cut it — readers can follow the `@ID` to the doc.
