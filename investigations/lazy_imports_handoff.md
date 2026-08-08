# Lazy-imports handoff — id-only env entries + postparsed cache

**Goal of the whole project:** make Rust imports lazy so importing `Vec` builds only the 2–3 methods you call, not all ~100. Full plan (Slices 1–4) is `~/.claude/plans/cheeky-chasing-elephant.md`; Slice 1 is the eager-refactor foundation and is what's done. Slices 2–4 (lazy Rust synthesis, `Vec` default type params, `Vec<int>.new()`) are not started.

## State

Slice 1 is implemented and green, uncommitted on branch `temp-lazy-imports` (based on `experimental`). The work sits on top of `16e66cd55`; see it with `git diff 16e66cd55 -- FrontendRust/src/typing/`.

Suite is at the branch baseline — regenerate with `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib` and confirm the split is unchanged (the failing count is the known onion-era set, not regressions; any deviation from baseline is a real regression). `cargo build --lib` (not the default target — binaries fail on unrelated backend_ffi/pass_manager) must be warning-clean beyond the pre-existing `unreachable` set.

## What Slice 1 changed (code)

- `IEnvEntryT` variants (i_env_entry.rs) hold `template_id: &'t IdT` instead of the postparsed declaration; Struct/Interface entries also carry `tyype`. Impl entries are id-only — the sub/super imprecise names live on `ImplTemplateNameT`/`AnonymousSubstructImplTemplateNameT` (names.rs), read back via `IImplTemplateNameT::imprecise_names()`.
- Definition templatas (templata.rs) hold ids + `tyype` instead of `&StructS` etc. `ITemplataT::tyype()` returns the stored `tyype` for Struct/Interface, so it needs no cache.
- Postparsed cache: four `template_id_to_postparsed_*` maps on `CompilerOutputs` (compiler_outputs.rs), seeded at index time in `fn evaluate` (compiler.rs). `coutputs` is created after `global_env`, taking the maps by value.
- Sibling-entry macros return `Vec<GeneratedAhtDenizen>` (macros.rs); `fn evaluate` derives both the env entry (`GeneratedAhtDenizen::env_entry`) and the cache seed from each. Lambdas are the one exception — seeded at closure-eval time via `register_postparsed_function` in `fn make_closure_understruct_core` (struct_compiler_core.rs).

All cache keys are **template** ids. `fn templata` on `FunctionEnvironmentT` uses `template_id`, not the instantiated `id` — a prior bug used the instantiated id and broke virtual dispatch.

## Open cleanups (from review, not yet applied)

- Remove the dead `use crate::typing::env::i_env_entry::*;` glob in the four converted macros (struct_constructor_macro.rs, anonymous_interface_macro.rs, citizen/interface_drop_macro.rs, citizen/struct_drop_macro.rs) — they no longer reference any entry symbol; the glob hides that from rustc.
- Rename `_interface_name_t` → `interface_name_t` in `fn preprocess_interface` (compiler.rs) — it's used, so the underscore misleads.
- Drop the `// We can add tyype here if convenient` note on `FunctionEnvEntry` (i_env_entry.rs).
- Naming nits to decide: `template_id_to_postparsed_*` vs the neighbor convention `*_name_to_*`; `register_postparsed_function` vs the `add_*` family; the `get_postparsed_*` `.unwrap()`s lack a `vfail:`-style message.

## Open risks (design decisions, not bugs)

- Definition-templata equality got stricter: the old hand-rolled `PartialEq`/`Hash` for `FunctionTemplataT` deliberately ignored `outer_env`; the derive now includes it. Harmless today (env is functionally determined by the template id) but reverses a documented invariant — matters only if these are ever deduped across differing envs.
- `fn look_for_override`'s range lookup (edge_compiler.rs) tolerates a cache miss for builtin-macro functions (rsa/ssa-drop, `lock_weak`, `same_instance`, `as_subtype`) that aren't seeded, but nearby sites `.unwrap()` the same accessor. Safe today because only abstract interface methods reach the unwrapping sites; confirm before adding an abstract-dispatch path that could carry a builtin-macro origin.

## Lessons learned

- Variables/fields holding a template id are named `template_id`, never bare `id` — architect convention.
- The postparsed cache is keyed by template ids; every seed and lookup must use the template-level id. Deriving both the env entry and the cache seed from one `GeneratedAhtDenizen` is what keeps seed-key and entry-id from drifting — keep that single-source shape.
- All index-time cache seeding lives in `fn evaluate`; lambdas are the sole use-time exception because their ids depend on the enclosing instantiation. Don't scatter seeding back into the macros or precompile.
- The Guardian edit hook times out on `anonymous_interface_macro.rs` specifically (~13.8s server-side validate vs. a shorter client cutoff) — a file-specific slow shield companion, not a Guardian outage; other files edit fast. Retry the edit or raise the hook client timeout.
