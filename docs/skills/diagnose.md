---
name: diagnose
description: Dig in until the root cause is found. Don't propose solutions until the user agrees on the root cause.
g_read_when: Read when the user says "/diagnose" — dig for the root cause of a failure without proposing fixes.
g_mention_in:
  - CLAUDE.md
---

# Diagnose

When the user says `/diagnose`, dig in until you find the **root cause** of the problem under discussion.

## Rules

1. **Find the first place where things go wrong.** Look for something that seems unexpected given the context and the code — a value that shouldn't be that shape, a branch that shouldn't have been taken, an assertion that shouldn't have fired. Trace backward from the symptom to the earliest point where reality diverges from expectation.

2. **Do not propose any solutions.** Not "we could fix this by…", not "one option is…", not a recommendation, not a hint. Just describe the root cause.

3. **Wait for agreement.** Surface the root cause and stop. The user will either confirm or push back. Only when the user says **"propose"** are you allowed to suggest fixes.
