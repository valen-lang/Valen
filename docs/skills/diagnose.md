---
name: diagnose
description: Dig in until the root cause is found, then report it — lead-in, symptom, cause, and no proposed fix.
g_read_when: Read when reporting a failure to the human, or when the user says "/diagnose" — dig for the root cause of a failure without proposing fixes.
g_mention_in:
  - CLAUDE.md
---

# Diagnose

When diagnosing, find the **root cause** of the problem under discussion, report it, and stop short of the fix.

## Rules

1. **Find the first place where things go wrong.** Look for something unexpected given the context and the code — a value that shouldn't be that shape, a branch that shouldn't have been taken, an assertion that shouldn't have fired. Trace backward from the symptom to the earliest point where reality diverges from expectation.

2. **Do not propose any solutions.** Not "we could fix this by…", not "one option is…", not a recommendation, not a hint, not even an obvious one-liner. A fix offered early anchors the reader to your framing before they have formed their own.

3. **Wait for agreement.** Surface the root cause and stop. The user will either confirm or push back. Only when the user says **"ok propose"** are you allowed to suggest fixes.

## Report it in three parts, about one failure

Head them in the reader's terms, not the report's — **What the test wants**, **Why it fails**, **The cause** — adapting the first to whatever the entry point is.

1. **What the test wants.** Name it by path and line, show its source, say in one sentence what it asks for. The reader has not been staring at this code.

2. **Why it fails.** Open by saying where the code was and what it was doing — *"while compiling `main`'s body, on the `ship IShip = Raza(42)` line"* — then walk to the failure. **When it's a crash the collapsed call tree goes here**, not in an appendix (`collapsed-call-tree`). **Say what succeeded**: *"it finds the impl successfully"* tells the reader which subsystems to stop suspecting, and a backtrace never says it.

3. **The cause.** **Open with what is wrong, in one sentence, before any code** — an accurate account
   of what the code does is not yet a cause, and only becomes one when it names the expectation being
   violated. Then quote the code with its path and line range. **No origin stories** — how the code
   came to be this way is a guess, and it anchors the reader exactly as a premature fix does. Say
   what the code does; where reading couldn't settle something, say that.

## Report rules

- **One failure.** No census, no other tests dying at the same site, no grouping by first blocker.
- **Every snippet carries its path and line, relative to the git root**, so it is ctrl-clickable.
- **Plain terminal text** — no HTML entities.
- **Terse.** Cut the adjacent finding, the taxonomy, and the second example.
- **Plain, which is not the same as terse** — see `prose-reviewer`'s *"minimize reader effort, not word count"*, and `ewhy` when a rewrite still doesn't land.
- **Read, don't guess.** Every claim about what the code did comes from that code.

## Required reading

 * collapsed-call-tree