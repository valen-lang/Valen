# Lazy Rust Imports — working doc

Single source of truth for this thread. Importing a Rust type like `Vec` must synthesize only the 2–3
methods the program calls, not all ~100 (each `fn_sig` is an expensive rustc query). Slices 1, 2, and
2.5 are done and green; the forward work is Slices 3–4 (making real `Vec` usable).

## Roadmap

- **Slice 1 — id-only env entries + postparsed cache. LANDED** on `experimental` (`570dfb707`,
  `f2cef70f9`). Env entries and definition templatas hold template `IdT`s, backed by four
  `template_id_to_postparsed_*` tables on `CompilerOutputs`, seeded at index time in `Compiler::evaluate`.
- **Slice 2 — lazy Rust synthesis. DONE, green, uncommitted.** Importing a Rust free function or method
  registers an id-only entry with no `fn_sig`/synthesis/seed; `create_postparsed_function` in
  `rust_interop/importer.rs` synthesizes on first lookup (recovering the rustc item by re-resolving the
  canonical name its id carries). The compile loop skips the `rust` package. Proven by
  `lazy_synthesis_only_queries_called_functions`.
- **Slice 2.5 — source imports from real `import rust.crate.X.Y` statements. DONE, green, uncommitted.**
  The allowlist is gone: `Compiler::evaluate` loops `program.imports`, `oracle.resolve_import` maps each
  crate-qualified import to one `ResolvedName`, and `declare_rust_import` builds its entry. Every
  `// ZSPORK` is realized — none remain in the tree.
- **Slice 3 — Vec default type parameters.** Forward.
- **Slice 4 — Vec `.new()` and methods.** Forward.

## State (regenerate, don't trust stale)

- Both suites green (uncommitted, on `temp-lazy-imports`): default
  `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib` = `622/127/8`; interop
  `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib --features rust_interop` = `678/127/8`.
  The 127 are the known onion-era failures. Both build clean (8 pre-existing `unreachable` warnings).
  `grep -rn ZSPORK src/` and `grep -rn '1_000_000\|2_000_000' src/typing/rust_interop/` both return nothing.
- The offset-encoding trick is retired (Phase 1 of the full-zspork plan): a denizen's rustc item is
  recovered by re-resolving the canonical name its id carries, and the synthetic range is one shared
  constant `SYNTHESIZED_RANGE_OFFSET` in `declarations.rs`.
- Imports are **crate-qualified and singular**: `import rust.mycrate.Widget;` names the crate as the first
  segment; `resolve_crate_qualified_path` descends from *that* crate, so one path resolves to at most one
  item (no cross-crate ambiguity). An import that resolves to nothing (unknown crate, missing item, a
  module rather than a fn/struct, or the crate omitted) is a real compile error,
  `ICompileErrorT::UnresolvableRustImport` — not silently ignored. The test oracle is built from the
  program's parsed imports (harness runs `ScoutCompilation` first; there is no `Case.allowed`).

## What Slice 2 landed (fn = symbol, not line)

- **N1/C1 seam.** `get_or_create_postparsed_function` on `Compiler` (in `compiler.rs`) is the one seam
  every function read routes through: peek the sealed table, else the `#[cfg]` `create_postparsed_function`
  hook, else vfail. The four tables' fields are private and their peeks `pub(in crate::typing)`.
- **Deviation from the ZSPORK (works, flag if you dislike it):** only the *function* accessor is the
  `get_or_create` seam. Struct/interface/impl reads stay on the total `coutputs.get_postparsed_*`
  accessors — those kinds are always eager, and their only readers are a free function
  (`citizen_or_templata_rune_type_lookup`) and `.filter()` closures with no `&Compiler`. The total
  accessors return `&T` (vfail on miss), so they still never expose existence; the invariant holds.
- **N2 lazy registration.** `declare_rust_import` (in `importer.rs`, called per import from the
  `evaluate` loop) mints an id-only `FunctionEnvEntry` for a free function via
  `lazy_extern_function_local_name` — no `fn_sig` — and an eager opaque `StructS` + `StructEnvEntry` for a
  type (returned as a seed the loop inserts into the struct map). Free functions, methods, and drops are
  all lazy; only types seed a table.
- **N2b methods (and drop) in the type's outer env.** `rust_method_entries` (called from
  `precompile_struct`'s `all_outer_entries`) mints id-only lazy entries in each Rust type's outer
  `CitizenEnvironmentT`, keyed `struct_template_id.add_step(name)` — one per method **plus a `drop`**.
  `v.get()` resolves via the receiver's outer env; an associated function is called type-prefixed
  (`Counter.new()`) and resolves through the type's env; a scope-end `drop` resolves the same way. Drop
  is just a method with no rustc signature — no special store case.
- **N4 create hook.** `create_postparsed_function` recovers the rustc item by **re-resolving the id's
  canonical name** (`ResolvedName` + `oracle.resolve`), not by decoding a synthetic offset. A free
  function resolves its own id's name; a method resolves its owner then finds itself among
  `oracle.methods` by name; a `drop` (dispatched on the `keywords.drop` name) manufactures a
  `drop(self Owner<T…>) void` sig instead of calling `fn_sig`. Then `synthesize_extern_function` +
  `register_postparsed_function`. The offset-encoding trick is gone; the synthetic range is a shared
  constant (`SYNTHESIZED_RANGE_OFFSET` in `declarations.rs`) that carries no identity, because a
  denizen's identity is its template id (unique by `package_coord` + `init_steps` + `human_name`).
- **C2 + citizen-compile skip.** The function-compile phase skips `if is_rust_backed(package_id)`, and the
  citizen-compile loop in `struct_compiler_core.rs` (which force-compiles every function in a type's outer
  env) skips `if is_rust_backed(id)` — the analog that keeps outer-env Rust methods lazy.
- **N5 + decline migration.** `lazy_synthesis_only_queries_called_functions` proves `fn_sig` fires only
  for the called function. The five `declines_*` tests now assert an imported-but-uncalled unrepresentable
  function is **never queried** (flipped from "asked and declined").

## Governing invariant — whether a postparsed exists must be undetectable

No caller may ever ask "is this id in the postparsed tables yet?" Only two operations touch postparseds:
(1) ask an environment what denizens it holds, and (2) `get_or_create_postparsed_*` for a denizen id,
which **always returns**, building on a miss. With existence unobservable, a lookup that memoizes is
indistinguishable from a pure read — so the fact that a "read" during overload resolution now *mutates*
`CompilerOutputs` (registering the synthesized `FunctionS`, growing the once-write-only index cache
during the compile loop) is a non-issue: pure memoization behind a total function. This is what makes
the slice clean rather than leaky, and it is load-bearing for C1, C2, and overload resolution. Enforce
it structurally (see the `// VCOORD` on the tables): move the four tables and their peek behind a
private module exposing nothing but the total accessors; a Guardian shield guarding against new
existence queries may be worth adding on top.

## `ResolvedName` — core type

The resolved (post-re-export) canonical name an import maps to, the currency between the import loop and
`rust_interop`. Lives in `typing/env/environment.rs`:
`{ module_name: StrI, package_names: &[StrI], importee_name: StrI, kind: ImportedItemKind }`, with
`ImportedItemKind { Type, Function }`. Backend-agnostic (interned strings, no `DefId`), so it doubles as
the future representation for Vale's own package imports. `oracle.resolve_import(&ImportS) ->
Option<ResolvedName>` maps a written import to it (crate-qualified, singular); `oracle.resolve(&ResolvedName)
-> Option<RustItemId>` maps it to the rustc item (a coordinate-qualified selection in the oracle's `items`
table). `LoggingOracle` forwards both. `resolved_name_of(package_coord, local_name)` in `importer.rs`
rebuilds it from a top-level id's own name (for lazy re-resolution).

## Slice 3 — generic Rust types (partly landed)

Decided: **the allocator is named explicitly** (`Vec<int, Global>`, not `Vec<int>`) — no defaulting hack.
This is already our natural model: `own_generic_param_names` keeps every param, so `Vec` is a genuine
2-param type, and `Global` imports as an ordinary opaque type (nothing to build). So "Slice 3" is *not*
about defaults. What it actually needs:
- **Parent-inclusive method generics** (`parent_inclusive_generic_param_names`, `tyctxt_oracle.rs`). A
  method's signature names the impl's inherited params (`Vec::push`'s `value: T`), which
  `generics_of(method)` reports under `.parent`, leaving the method's own params empty — so lowering used
  to decline `InheritedParameter`. Fixed by walking `.parent` (the impl at low indices, then the method's
  own — exactly `GenericArgs::for_item`'s order). Verified by `calls_a_method_naming_the_types_generic`
  (`Holder<T>.into_value() -> T`). The drop already did this by hand via `type_generic_params`; this
  generalizes it. Per-impl matters: `Vec::new` is in `impl<T> Vec<T>` (params `[T]`, `Global` fixed),
  `Vec::push` in `impl<T, A> Vec<T, A>` (`[T, A]`).

## Slice 4 — `Vec::new` associated-function arity, and borrow semantics (forward)

Real `Vec<int, Global>` now imports (2-param), `Global` imports, `new`/`push` synthesize. Two gaps remain,
both Vale-side, neither about the allocator:
- **`Vec::new` arity.** `new` lives in `impl<T> Vec<T>` — its parent-inclusive generics are `[T]` (A fixed
  to `Global`) — but `Vec<int, Global>.new()` supplies 2 container args `[int, Global]`. `overload_resolver.rs`
  (`attempt_candidate_banner`, ~`:287`) computes `function_runes − container_runes` = `1 − 2` and
  **underflows** (usize panic). Two levels of fix: (1) make it graceful (`saturating_sub`/guard), (2) the
  real one — resolve an associated function whose impl **fixes** some of the type's params: bind `T=int`
  and reconcile the extra `Global` against the fixed slot (unify `new`'s return `Vec<T, Global>` with the
  expected `Vec<int, Global>`, or erw's `@ETASTZ` truncate-the-extra). A receiver method like `push`
  (impl params match the type's) does not hit this.
- **Borrow-read on locals.** `h.method()` where `h` is a local reads as `BorrowRef(Holder<int>)`, but a
  by-value `self` method wants owned → `CouldntFindFunctionToCall`. Known Vale-side onion-arc gap (Vale2).
  Calling on an rvalue (`(make_holder()).into_value()`) dodges it.
Cases still wanted once these land: `v.push(42)`, `v.len()` (note `usize` return declines), scope-end drop
of `Vec<int, Global>`.

## Open decisions & risks

- **Associated function calls are type-prefixed (`Counter.new()`), decided.** A no-receiver associated
  function has no argument to route it to the type's outer env, so it is called `Type.new()`, resolving
  through the type's env. The two corpus cases that used bare `new()` were updated. (Slice 4's
  `Vec<int>.new()` is the same shape.)
- **Strict accessor uniformity (open).** The invariant is satisfied, but only the *function* accessor is
  the `get_or_create` seam (see "Deviation" above). If you want all four kinds identical, it needs
  threading `&Compiler` + `&mut coutputs` into `citizen_or_templata_rune_type_lookup` and the impl
  `.filter()` closures. Deferred as not worth it unless a kind other than function goes lazy.
- **Called decliner panics** (out of scope; Vale2's callsite/overload rework owns graceful errors). Also:
  a called name whose candidate set includes an unrepresentable overload panics while forcing it, even if
  another candidate would match. Deferred.
- **Undroppable Rust types (deferred).** Every imported type currently gets a drop; whether some should be
  undroppable (Vale allows it) changes only *which* types get a drop entry, not how drops are built.
- **`vale-rust-interop-architecture.md`.** The interop code cites an architecture doc with numbered
  sections (`§10`, `§20.3`, `§26b`), but no such file is in the tree. Locate or (re)create it before
  relying on those citations.

## Future — opaque foreign scalars (separate slice, after Vec basics)

`u64`/`f64`/`usize` decline today because forcing them into `IntT`/`FloatT` is lossy. Represent each as an
opaque `rust`-namespaced nominal type (generalizing the imported-struct path) — distinct names give
distinct identity, retiring most of the decline set that blocks real `std`. Costs a rustc-ABI link on the
synthesized type at tier-2 codegen. Doesn't cover `str`/`[T]`/`dyn` (unsized) or associated-type projections.

## Lessons learned

- Whether a denizen's postparsed exists must be undetectable to callers — only "what's in this env" and
  "get-or-create by id" are legal. This is the invariant the whole lazy design rests on; guard it.
- Variables/fields holding a template id are named `template_id`, never bare `id` — architect convention.
- The postparsed cache is keyed by **template** ids; every seed and lookup uses the template-level id.
  `FunctionTemplataT.function_id` must be the *template* id, not an instantiated id — they have different
  `local_name` shapes and the instantiated one misses the cache.
- A Rust type's methods live in its **outer `CitizenEnvironmentT`** (Vale's home for methods), lazy. The
  trap: the citizen-compile loop in `struct_compiler_core.rs` force-compiles *every* function in a type's
  outer env (correct for Vale internal methods), which would `fn_sig` all of an imported type's methods.
  It stays lazy only because that loop skips `is_rust_backed(id)` entries — the citizen-compile analog of
  C2's top-level `rust`-package skip. Do not remove either skip. Free functions stay top-level; a type's
  methods and its `drop` are type-nested (next entry).
- Drop is a normal function named `drop`, lazy and type-nested in the owner's outer env exactly like a
  method. It needs no `fn_sig` (its `drop(self Owner<T…>) void` receiver sig is manufactured on force),
  and `create_postparsed_function` recovers the owner (hence its generics) by re-resolving the id's owner
  name. There is no eager top-level drop and no special store case.
- A synthesized denizen's identity is its **template id** (unique by `package_coord` + `init_steps` +
  `human_name`), not its code-location range. So the synthetic range is one shared constant; do not
  reintroduce a per-item offset "for uniqueness" (the `FunctionTemplataT` eq/hash that once forced it is
  now derived over `{ outer_env, function_template_id }`).
- Adding a method to the `RustOracle` trait means forwarding it in `LoggingOracle` too — a decorator that
  inherits a default silently returns it (a `None` `resolve` fails every lazy synthesis with no error).
- The Rust oracle is **per-compilation, not per-feature**: `oracles.rust` is `None` for any program that
  imports nothing from Rust, even in the `--features rust_interop` build (the ~89 general typing tests run
  that way). Require the oracle only *inside* the rust-import branch, never around the whole loop, or those
  tests panic. An `import rust.…` with no oracle is a driver misconfiguration and panics.
- Imports are crate-qualified: the first path segment names the crate (`import rust.mycrate.Widget;`), and
  `resolve_crate_qualified_path` descends from that crate — a bare `import rust.Widget` is unresolvable. A
  method is never imported by name; you import its type and the method arrives via the type's outer env.
- The `@ATAFLBZ` lint (`no_rust_item_identity_comes_from_a_human_name`) flags any line with `human_name`/
  `.ident` and `==`/`!=`; a coordinate-qualified *selection* is legitimate but the `// ataflbz-allow`
  comment must sit on the comparison line itself, not the line above.
- Prefer a stable id property (`is_rust_backed`) over a mutating-set predicate for the compile-loop skip.
- The `get_postparsed_*` accessors return `&'s T` (arena-borrowed, not `coutputs`-borrowed), so one
  binding survives intervening `&mut coutputs` calls — repeated fetches are redundant, not borrow-forced.
- The Guardian edit hook occasionally times out client-side (server validates ~13.8s); retry the edit
  rather than assume failure.
