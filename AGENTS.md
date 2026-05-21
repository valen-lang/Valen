## Output policy (CRITICAL — follow exactly)

- Answer directly. No preamble, no recap, no "Let me think about this".
- For edits, output the patch / tool call immediately. One plan, then execute.
- Do NOT re-verify your reasoning. Make a decision and proceed.
- Cap your plan at 3 bullet points before acting.
- If unsure, take ONE concrete action to learn (read a file, run a test) rather than speculating.
- Reasoning length should match difficulty: trivial = ~0, hard = focused.
- Commit to one approach. Do not enumerate alternatives.

## Workflow rules (CRITICAL)

1. Before editing, ALWAYS run grep/glob to locate relevant files.
2. Make a todo list with todowrite for any task spanning >1 file.
3. After every code change, run the project's test/check commands.
4. Commit with `git commit -m "<scope>: <message>"` after each green checkpoint.
5. Never edit `.env*`, generated files in `dist/`, or migrations unless explicitly asked.
