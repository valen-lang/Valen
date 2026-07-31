# Plan document

Source: `/Users/verdagon/.claude/plans/resilient-riding-volcano.md`
Session: 8b824c88-fe28-4a78-8b17-8f7d4cada5a8

---

# Let MBXX accept the heredoc form its own skill documents

## ⚠ This plan cannot be applied by an AI without your say-so

`NoModificationsToShieldFiles-NMSFX` forbids exactly this edit, and its DENY example is
literally a shield's `src/main.rs`:

> Shield files (`.md` files in any `shields/` directory) **and their companion programs**
> define the rules that govern AI behavior. AI agents must not modify them — only humans may
> edit shields. Any change to a file whose path contains `/shields/` is a violation.

Guardian is disabled right now, so the edit would go through unnoticed. That is a reason to be
more careful, not less. Either apply this yourself, or authorize the override explicitly.

## Context

`docs/skills/use-mailbox.md` documents this as *the* way to send a message:

```bash
cat <<'EOF' | cargo run --manifest-path Luz/mailbox/Cargo.toml --release -- send --as "$ME" --to <recipient>
<message body>
EOF
```

MBXX rejects it. I hit this sending the phoenix report to LangNotesValen and had to write the
body to a file and pipe it with `cat` instead. So a skill instructs one thing and a shield
refuses it — and the shield's own `description:` already claims to allow
`<echo|cat> … | mailbox send …`.

## Root cause — two independent barriers

Both in `Luz/shields/MailboxWorkflow-MBXX/src/main.rs`:

1. **`scan_for_forbidden_metachars` (line ~51)** rejects any unquoted newline:
   ```rust
   if b == b'\n' || b == b';' { return true; }
   ```
   A heredoc is inherently multi-line, so this returns `NotTriggered`, and then
   `command_mentions_mailbox_invocation` finds the buried invocation and hard-denies with
   `MAILBOX_NOT_STANDALONE_VIOLATION`. That is the message I got.

2. **`has_unquoted_redirect_or_background` (line ~109)** rejects any unquoted `<`, so
   `is_upstream_feeder_segment` would refuse `cat <<'EOF'` even on a single line.

## Design

Treat a heredoc body as **data, not command text** — because that is what it is. Split it off
before any existing check runs, and validate only the command line.

New helper, alongside the existing scanners:

```rust
/// Splits `cat <<'EOF' | mailbox send …\n<body>\nEOF` into the command line and the body.
/// `None` when there is no heredoc, when the delimiter is unquoted, or when the terminator
/// is missing.
fn split_quoted_heredoc(cmd: &str) -> Option<(&str, &str)>
```

Then:
- `parse_mailbox_invocation` and `command_mentions_mailbox_invocation` operate on the command
  line only, so the body never reaches the metachar scan.
- `is_upstream_feeder_segment` accepts `cat <<'DELIM'` as a feeder, checking the redirect rule
  against the segment with the heredoc operator removed (otherwise barrier 2 still fires).

### The security boundary: only *quoted* delimiters

This is the crux, and the reason the rule isn't simply "allow heredocs."

- `cat <<'EOF'` — quoted: the shell performs **no expansion**. The body is literal bytes.
- `cat <<EOF` — unquoted: the shell **expands** `$(…)` and backticks *inside the body*. A body
  containing `$(rm -rf /)` would execute.

So an unquoted delimiter must stay denied. That preserves exactly what
`deny_echo_with_dollar_paren_substitution_into_mailbox_send` already protects — it would
otherwise be trivially bypassable by moving the substitution into a heredoc body.

`<<-'EOF'` (tab-stripping) is out of scope and stays denied, documented with a test the way
`deny_cat_with_flag_upstream_into_mailbox_send` documents the `cat -n` boundary.

## RFIGA

Baseline first: MBXX green, per `tdd` §0.

The test surface is already right — `is_approved` / `is_denied` / `violations_of` in
`mod tests`, all driving the `run()` dark box. No new harness needed.

1. **The documented heredoc form is approved.**
   - R: `approve_quoted_heredoc_piped_into_mailbox_send`, using the exact shape from
     `use-mailbox.md`.
   - F: `cargo nextest run --manifest-path Luz/shields/MailboxWorkflow-MBXX/Cargo.toml`;
     expect failure. Report "Tests are correctly failing".
   - I: `split_quoted_heredoc` + wire it into `parse_mailbox_invocation`; accept
     `cat <<'DELIM'` in `is_upstream_feeder_segment`.
   - G: re-run; expect pass. **A:** full MBXX suite.

2. **The body is data, not command text.**
   - R: `approve_heredoc_body_containing_shell_metacharacters` — body holding `;`, `|`,
     `$(`, and backticks (the real report I sent contained `ps aux | grep …`); and
     `approve_heredoc_body_mentioning_mailbox`, which pins that the body cannot trip
     `command_mentions_mailbox_invocation`.
   - F / I / G / **A** as above.

3. **An unquoted delimiter stays denied.**
   - R: `deny_unquoted_heredoc_delimiter_into_mailbox_send`, plus
     `deny_unquoted_heredoc_body_with_substitution` — the bypass this rule exists to stop.
   - F: expect failure only if slice 1 was written too permissively; if these pass
     immediately, say so rather than counting them as verified.
   - I / G / **A** as above.

4. **Malformed and out-of-scope heredocs stay denied.**
   - R: `deny_heredoc_without_terminator`, `deny_tab_stripping_heredoc_is_out_of_scope`.
   - F / I / G / **A** as above.

5. **The deny message names the heredoc form.**
   - R: `the_not_standalone_message_names_the_heredoc_shape` — assert `violations_of` on a
     genuinely chained command mentions the accepted heredoc spelling, so a session that
     trips the shield learns the shape that works.
   - F / I / G / **A** as above.

Not RFIGA: update the shield's `description:` frontmatter to name the heredoc alongside
`<echo|cat> … | mailbox send …`. `use-mailbox.md` needs no change — it already documents the
form; the shield is what was wrong.

## Verification

```bash
cargo nextest run --manifest-path Luz/shields/MailboxWorkflow-MBXX/Cargo.toml
```

Then the whole tree, which must stay at its current count:

```bash
OPENROUTER_API_KEY=$(cat Guardian/api_key.txt) \
GUARDIAN_OPENCODE_ROOT=$(pwd)/Guardian/opencode \
GUARDIAN_BUN_PATH=/Users/verdagon/.bun/bin/bun \
  ./Guardian/check-tests.sh > ./tmp/mbxx.txt 2>&1
grep -E "^✗|FAILED:" ./tmp/mbxx.txt          # expect nothing
```

End-to-end, and only meaningful once Guardian is rebuilt and re-enabled (it rebuilds companion
binaries at hook time, so the new logic loads on the next command): send a real message with
the documented heredoc form and confirm it is auto-allowed rather than hard-denied.

## Still open elsewhere

- **Guardian is disabled and on a pre-change binary** — the convo-lineage work is not live, and
  this fix could not be verified end-to-end until that changes.
- **`docs/skills/scripting.md` vs BESWX** — the same class of contradiction: the doc promises
  read-only `python3 ./tmp/scripts/*.py` auto-allows, BESWX's own tests assert it does not.
- **Nothing is committed**, across six repos — including someone else's in-progress
  `session_id`/`transcript_path` feature that my ShieldFile/ContextifiedShield test repairs are
  now entangled with.
