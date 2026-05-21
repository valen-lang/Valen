# Bug: `data_substituted.txt` artifact still contains `{{shield_rule}}` placeholder

## Summary

The artifact saved as `data_substituted.txt` in Guardian log directories has a misleading name. It claims to be the "substituted" prompt, but the `{{shield_rule}}` placeholder is never filled in — it is left as the literal string `{{shield_rule}}` in the saved file.

## Location

`Guardian/ShieldFile/src/lib.rs`

- `substitute_check_template` (line ~1171) fills in `{{file_path}}`, `{{file_content}}`, `{{referenced_defs}}` but explicitly leaves `{{shield_rule}}` unfilled.
- The result is saved as `data_substituted.txt` (lines ~1235–1236, ~1327–1328).
- `run_shield_file` later fills in `{{shield_rule}}` at line ~940 when building the actual prompt sent to the LLM.

The artifact is saved *between* these two substitution steps, so it represents a partially-substituted intermediate — not the final prompt the LLM sees.

## Impact

1. **Debugging confusion.** When investigating why a shield fired or failed, `data_substituted.txt` is the natural first artifact to read. But it doesn't show the shield rule, so you can't see the full prompt the LLM received. You have to mentally merge the file with the shield `.md` to reconstruct what was actually sent.

2. **Manual replay is broken.** If you copy `data_substituted.txt` and send it verbatim to the LLM (e.g. via `curl` or `check-direct`), the model receives `{{shield_rule}}` as a literal string. In testing this, kimi-k2.6 responded: *"the Rule section is literally `{{shield_rule}}` — I cannot determine what specific rule to enforce"* and returned empty observations, silently appearing to pass.

## Repro

```
cat FrontendRust/guardian-logs/request-001-1778726528945/hook-001/evaluate_lookup_for_load--201.0.ScalaParityDuringMigration-SPDMX.data_substituted.txt | grep shield_rule
# Output: {{shield_rule}}
```

## Fix

Either:

**A. Save the fully-substituted prompt.** After `run_shield_file` fills in `{{shield_rule}}`, save the final `prompt` string as `data_substituted.txt` instead of saving the intermediate. This makes the artifact actually match its name and makes manual replay work correctly.

**B. Rename the artifact.** Keep saving at the current point but rename to something like `data_pre_shield.txt` or `prompt_template.txt` so it's clear that `{{shield_rule}}` is still a placeholder. Also save the final prompt (after shield substitution) as `data_substituted.txt`.

Option A is simpler and makes the artifact more useful for debugging.
