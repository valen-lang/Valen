# Opaque extern-struct drop — target design

## Context

Extern structs (declared via `extern struct X { ... }`) have opaque layout from Vale's PoV. The compiler can't `Destroy` them — it doesn't know the members — and the simplifier's `translate_destroy` panics at `expect_struct_access: not a struct` if we try. Historically the `struct_drop_macro` special-cased `SharednessT::Single if struct_def.members.is_empty()` (a rough proxy for "extern") and emitted a `Discard` — which silently leaks any C-side resources the extern struct owns. Item #17 in the 26-item review flagged this as a wrong-shaped special case.

**Interim state (this doc's premise):** the extern branch now panics loudly (`auto-generated drop for extern struct is unsupported; supply an explicit extern func drop(...)`), and the 7 tests that were exercising the leaky path are `#[ignore]`d with rationale pointing here. That surfaces the gap; the design below is the way out.

## The target model

Auto-generate the extern drop function as a Vale-declared extern that the foreign language provides an implementation for. Vale calls it at drop time; the foreign language destructs.

The default derivation depends on the FFI target:

| Setup | Auto-derive drop? | Default extern symbol name |
|---|---|---|
| `extern(c) struct X { ... }` — no annotation | **No.** Compile error if the struct is instantiated and no drop resolves. | n/a |
| `extern(c) struct X { ... }` + `#!DeriveExternStructDrop(foo_drop)` | Yes. | `foo_drop` |
| `extern(rust) struct X { ... }` — no annotation | **Yes.** | `drop` |
| `extern(rust) struct X { ... }` + `#!DeriveExternStructDrop(bar)` | Yes. | `bar` |
| Any of the above + explicit `func drop(x X) { ... }` | Suppresses the macro (existing `#!DeriveStructDrop` semantic). | n/a |

Both attributes compose: `#!DeriveStructDrop` is the escape hatch for "I'm writing my own body"; `#!DeriveExternStructDrop(name)` is the knob for "the auto-generated body should call this extern symbol."

## Why the C vs Rust split

- **C users expect nothing implicit.** No calling convention around drop exists in C. If the Vale user forgets to name a symbol, they should hear about it at compile time — not link, ship, and leak. `extern(c)` is strict.
- **Rust has a familiar `drop` convention.** Defaulting the symbol name to `drop` (unmangled) matches Rust idiom. Users get sane defaults without ceremony. `extern(rust)` is relaxed.
- **`share` does not apply.** There's no such thing as `extern(rust) share` (or `extern(c) share`). Shared refcount semantics are Vale's own; extern languages don't participate. The `Shared` branch in `struct_drop_macro` keeps its current `Discard` behavior (releases the refcount, no extern call).

## Design questions to resolve at implementation time (defer to the Rust-interop TL)

1. **Generics + naming.** `extern(rust) struct Vec<T>` — do `Vec<i32>` and `Vec<str>` call the same `drop` symbol (which would need to be generic somehow), or does each monomorphization get a distinct mangled symbol? Rust's `Drop::drop` is per-monomorphization but not directly extern-callable — the user has to write a `#[no_mangle] extern "C" fn <name>(v: Vec<i32>) { std::mem::drop(v) }` shim per monomorphization. Whether Vale mangles the auto-generated symbol name, or requires `#!DeriveExternStructDrop(<name>)` on generic externs, is a Rust-interop-TL call.

2. **Rust-side shim requirement.** For `extern(rust) struct X` with auto-derived `drop`, someone must supply a `#[no_mangle] extern "C" fn drop(x: X) { std::mem::drop(x) }` (or the mangled equivalent) on the Rust side. Vale does NOT generate Rust code. This is the user's obligation and should be documented in the extern-Rust user guide.

3. **Migration of existing `extern struct` (no language annotation).** Current Vale syntax has plain `extern struct` with no C-vs-Rust distinction — `ICitizenAttributeT::Extern(ExternT { package_coord })` carries no language marker. Introducing the split is a language surface change. Migration options:
   - **Default old-syntax to `extern(c)`** — strict; some existing tests must add either `extern(rust)` or `#!DeriveExternStructDrop(name)`. Loud, correct semantic; requires touching test fixtures.
   - **Default old-syntax to `extern(rust)`** — relaxed; existing tests keep passing as-is, but the plain `extern struct` semantic silently gains a "calls `drop`" contract. Foot-gun for C users.
   - Recommendation TBD by the interop TL.

## Current gap and retirement plan

**Where the gap sits in code:**
- `FrontendRust/src/typing/macros/citizen/struct_drop_macro.rs` — the `SharednessT::Single if is_extern` branch. Currently panics with `auto-generated drop for extern struct is unsupported; supply an explicit extern func drop(...)`. When this design lands, this branch is replaced by extern-function-header synthesis (see below).

**What retiring the gap looks like:**
1. Extend `ICitizenAttributeT::Extern` (or introduce a sibling) to carry the target language (`C` vs `Rust`).
2. Introduce `#!DeriveExternStructDrop(symbol_name)` at the parser layer; wire through postparser + typing into a new `ICitizenAttributeT::DeriveExternStructDrop(StrI)` (or similar).
3. In `struct_drop_macro`, replace the panic with:
   - Look up the effective drop symbol per the table above.
   - Emit a `FunctionHeaderT` with an extern-function attribute + no body (or a body that's a call to `panic!()` since the extern is invoked directly, never through this generated function).
   - Register the synthesized extern with `coutputs.extern_functions` so the Backend picks it up and emits an extern declaration for it in the C header.
4. At drop-emission sites (Unlet etc.), the compiler resolves `drop(v)` as a call to the extern; simplifier lowers it as a plain FunctionCall. No `Destroy` on the extern struct — sidesteps the opaque-layout problem entirely.
5. Update Backend's extern-function handling if any changes needed (probably minimal — it's just another extern func).
6. Un-`#[ignore]` the 7 tests listed below, adjust their Vale source to declare the required attributes (or accept the extern(rust) default), and add tests for `#!DeriveExternStructDrop(name)` + the C-side strictness error.

## Ignored tests waiting on this design

All 7 fail today because the compiler panics at `struct_drop_macro`'s extern branch. They exercise different corners of extern struct behavior; keep the coverage — just gate on this design landing.

**Compile-time / Hammer-IR wire-format tests** (`FrontendRust/src/integration_tests/tests/hammer_tests.rs`):
- `extern_method_in_generic_extern_struct_puts_container_args_on_citizen_step_in_wire_format_simple_id`
- `mixed_own_inherited_template_args_split_correctly_in_wire_format_simple_id`
- `top_level_extern_functions_wire_format_simple_id_has_flat_shape`

**Runtime end-to-end tests** (`FrontendRust/src/integration_tests/tests/integration_tests_a.rs`):
- `extern_function_returning_extern_struct`
- `extern_method_on_generic_extern_struct_returns_expected_value`
- `extern_rust_vec`
- `extern_rust_vec_capacity`

Each test's `#[ignore]` rationale points here.

## Related

- Item #17 (this arc) — `struct_drop_macro` extern gating cleanup. Landed the `is_extern`-honest gate on top of the panic.
- Item #19 (pending) — retire `ITemplataT::Mutability` + `SharednessTemplataT` + `SharednessTemplataType`. Adjacent-but-independent cleanup; can proceed without this design landing.
- `vcoord-handoff.md` §"Frontend cleanups triggered by the OwnInline landing" — this design is closer to the FFI arc than to OwnInline; separate concern.
- `vcoord-handoff.md` §"Mission — Replay / FFI design for the own-based world" — this design intersects the FFI mission's language-level invariant ("C can't modify Vale data through pointers"). Drop is a legitimate cross-boundary operation; the invariant is that C never *dereferences* Vale pointers to inspect/mutate the payload. Calling a Vale-supplied drop function on an extern struct is FFI in the normal direction (Vale calls out), so it's compatible with the invariant.
