# Bug: `check-direct` should not require `--mode`

## Summary

`guardian check-direct` requires a `--mode` flag when `--config` is provided, but `check-direct` already accepts the shield to run via `--check` or `--check-filter`. The mode is only needed to look up which shields to run — a concern that doesn't apply when the shield is supplied directly on the command line.

## Problem

Running `check-direct` with `--config` but without `--mode` errors out:

```
--mode is required when using --config (e.g. --mode guard_mode)
```

But the caller already specified which shield to run:

```
guardian check-direct \
  --config FrontendRust/guardian.toml \
  --check-filter SPDMX \
  ...
```

The mode (`guard_mode`, `migrate_mode`, etc.) controls which shields are included in a normal hook run. `check-direct` bypasses that entirely — you hand it a contextified diff and a shield, and it runs just that shield. The mode selection is irrelevant.

## Impact

- Forces callers to know and supply a mode name even when they don't care about the mode's shield list.
- The mode's shield list is ignored anyway (overridden by `--check`/`--check-filter`), so requiring it is pure friction.
- Makes the `check-direct` command harder to use for quick manual replays of a single shield.

## Fix

When `--check` or `--check-filter` is provided alongside `--config`, `--mode` should be optional. The config is still needed for backend/model settings (API keys, tier configs, etc.) — just not for shield selection. If `--mode` is absent and no `--check`/`--check-filter` is given, then error as before.
