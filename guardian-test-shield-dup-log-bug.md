# Bug report: `test-shield` panics with "Log file already exists … case-1.log"

**For:** the Guardian team (you have access to this machine — paths below are live).
**Filed by:** the Vale TL session, while running `/guardian-diagnose` to add an SPDMX exception.
**Severity:** blocks `test-shield` entirely for an affected shield — i.e. blocks the normal shield-curation/validation workflow.

## Symptom

`guardian test-shield` aborts (exit 101) with:

```
thread 'main' panicked at Rabble/src/steppy_logger.rs:220:9:
Log file already exists: FrontendRust/guardian-logs/request-<TS>/log.test-shield.case-1.log
```

The panic is the assert in `SteppyLogger::create_file`:

```rust
// Guardian/Rabble/src/steppy_logger.rs:214-224
fn create_file(&self, print_log_path: bool) {
    ...
    let path = self.file_path();          // "{dir}/log.{local_name}.log"
    assert!(
        !Path::new(&path).exists(),
        "Log file already exists: {}",
        path
    );
    ...
}
```

## Root cause

`test-shield` assigns **two different cases the same logger `local_name`** (`case-1`), so the second case's `create_file` finds `log.test-shield.case-1.log` already present and the assert fires. The per-case log name is **not namespaced by the case's source queue**, so cases that share an index across different queue dirs collide.

For the SPDMX shield (`Luz/shields/ScalaParityDuringMigration-SPDMX/`), case files with index `001` exist in **multiple** queue dirs simultaneously:
- `tests/cases/001.*`
- `cases/need-shield-amendment/001.*`  ← created by `guardian expect-allow` in this session
- `cases/need-doublecheck-override/001.*`

`test-shield` appears to enumerate cases from more than one of these and names each `log.test-shield.case-N.log` by a per-queue index, so two `001`s both become `case-1` → collision.

## Reproduction

```bash
cd /Volumes/V/Vale
OPENROUTER_API_KEY=$(cat Guardian/api_key.txt) Guardian/target/debug/guardian test-shield \
  --shield Luz/shields/ScalaParityDuringMigration-SPDMX.md \
  --config FrontendRust/guardian.toml \
  --cache-dir /tmp/guardian-cache --log-level overview
```

**Deterministic.** Reproduced with three different fresh `--cache-dir` values (`/tmp/guardian-cache`, `/tmp/guardian-cache-spdmx-x`, `/tmp/guardian-cache-spdmx-x2`) — so it is **not** stale-cache state. Each run creates a fresh timestamped `FrontendRust/guardian-logs/request-<TS>/` dir, so it is **not** a cross-run leftover either — the duplicate `case-1.log` is created **within a single run**.

Live evidence dirs (each contains the panic in `log.test-shield.log`):
- `FrontendRust/guardian-logs/request-1779996231841/`
- `FrontendRust/guardian-logs/request-1779996551327/`
- `FrontendRust/guardian-logs/request-1779996958496/`

## Secondary finding: `expect-allow`/`expect-deny` create cruft at malformed nested paths

While investigating, I found two **untracked** case-tree copies at clearly-wrong nested paths under the shield dir:
- `Luz/shields/ScalaParityDuringMigration-SPDMX/SPDMX/cases/need-doublecheck-override/` (18 files)
- `Luz/shields/ScalaParityDuringMigration-SPDMX/ScalaParityDuringMigration-SPDMX/cases/need-doublecheck-override/` (21 files)

These look produced by a path-resolution bug in `expect-allow`/`expect-deny`: the `expect-allow` I ran printed its target as `FrontendRust/../Luz/shields/ScalaParityDuringMigration-SPDMX/cases/need-shield-amendment` (note the `FrontendRust/..` prefix), suggesting the shield root is being re-joined with a doubled `<shield-name>/` segment under some invocations, dumping case files into `…/<ShieldName>/<ShieldName>/cases/` and `…/<ShieldName>/cases/`. They are **not** clean duplicates of the real `cases/need-doublecheck-override/` (different file counts: 90 vs 18 vs 21), so they're stray snapshots, not copies. Moving them aside did **not** fix the panic (the `tests/cases` ↔ `cases/need-shield-amendment` index collision is sufficient on its own), but they compound the duplicate-case-discovery problem and should be cleaned up + the path bug fixed. They are still present (left untouched for you to inspect).

## Impact

This fires in the **normal** `/guardian-diagnose` workflow: `expect-allow`/`expect-deny` adds a `need-*/001` case while `tests/cases/001` already exists, and the very next `test-shield` (the skill's validation step) panics. So shield curation is blocked right at the validate step for any shield that already has a `001` in `tests/cases`.

## Suggested fixes (pick whichever fits the design)

1. **Namespace the per-case log name by source queue** — e.g. `log.test-shield.<queue>.case-N.log` or use a globally-unique case id (queue + index) instead of a bare per-queue `case-N`. This is the cleanest — removes the collision regardless of how many queues are enumerated.
2. **Make case discovery dedupe / use a single flat index** across all enumerated queue dirs so no two cases share `case-N`.
3. **Don't recurse into non-canonical subdirs** during case discovery (would skip the cruft dirs), and/or **fix `expect-allow`/`expect-deny` path resolution** so they stop creating `…/<ShieldName>/<ShieldName>/cases/` cruft.
4. **Soften `steppy_logger::create_file`** so a name collision is a clear error naming *which two cases* collided, rather than a bare assert — at minimum better diagnostics.

(1) + (3) together would fix both the immediate crash and the cruft source.

## Workaround for curators in the meantime

Promote the new `expect-allow`/`expect-deny` case from `cases/need-*/NNN` into `tests/cases/` with a **unique** (next-free) index before running `test-shield`, and ensure no two enumerated queue dirs share an index. (Not verified to fully avoid the crash if `need-doublecheck-override` also has the colliding index.)
