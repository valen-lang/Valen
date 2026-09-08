# Interfaces — master handoff

This is the top-level handoff for the **interface rework**: moving Vale interfaces from one representation
(everything is a fat pointer) to a **two-representation model** — a *closed* interface used inline lowers to a
**tagged enum** (thin pointer, tag-dispatch), while `dyn` erases it to a **fat pointer** (vtable) — and then
building downcast/`try_as`, heap `Box`, and the rest of the Valen open/closed-trait model on top. A next
session with none of this context should read this whole doc first, then start on the **cupcake plan**
(linked below). Written 2026-09-08; the endeavor spans many sessions.

## The immediate plan to start on

**`~/.claude/plans/principled-but-please-plan-atomic-cupcake.md`** — the current, authoritative implementation
plan for finishing the `dyn` conversion's dispatch. Start there. (An earlier, higher-level frontend-conversion
plan, `~/.claude/plans/please-plan-out-slice-clever-hinton.md`, is largely **superseded** by cupcake but has
extra context on the Result/Opt rename and the full test-conversion catalog if useful.)

## Phase ordering (the whole endeavor)

1. **`dyn` conversion — IN PROGRESS.** Make the fat-pointer representation explicit via `dyn`/`open`/`Box`,
   and move every interface test onto the `dyn` spelling, so the *bare* spellings (`X`, `&X`, default
   `interface`) are freed for enums. Slices: (1) `dyn` keyword + `DynInterface` kind; (2) `Box<T>` dumb
   builtin; (3) `open` attribute + flip default to sealed; (4) honest `dyn` dispatch + convert the tests;
   (5) rename `Result`/`Opt` → `ResultI`/`OptI`. Slices 1–3 and slice-4 **dispatch** (cupcake Part 1) are
   built; what remains is slice-4b (migrate every owned-interface fixture to `Box<dyn X>`, blocked on owned
   `Box<dyn X>` construction) and slice 5.
2. **Tagged enums.** Build the enum representation for bare (closed) interfaces: `{tag, max-payload}` layout,
   construction (branch the struct→interface upcast), `&Foo` thin-ptr, tag-dispatch (`LLVMBuildSwitch`), enum
   drop (switch on tag → drop live variant), and the `isa`/`downcast` metal primitives with the `try_as`
   rewrite. Then **flip bare `Interface` from fat to the enum path** (it's a thin pointer, `EnumInterface` is
   a new kind). Create enum copies of the interface *behavior* tests (borrow/dispatch/downcast/generics/
   linked-lists/drop) using bare-sealed interfaces + `&Foo`/value spellings.
3. **Downcast / `try_as` → `Result`.** `try_as` returns `Result<&Sub,&Super>` **by value** — an owned
   interface value — and `Result` is a *closed* sum type, so it must be a **tagged enum**. This is why enums
   come before real downcast. Decide what canonical `Result`/`Opt` become (the enum sum types, now that
   `ResultI`/`OptI` hold the dyn versions).
4. **Heap `Box` + owned interfaces.** Make `Box<T>` actually heap-allocate (today it's a dumb inline wrapper).
   Then `Box<dyn X>` owns a heap object; `Box<Concrete>` → `Box<dyn Trait>` cast; the vtable carries a
   consuming `drop`(`Box<Self>`) so a `Box<dyn>` self-destructs without static size. Owned-interface tests go
   green here.
5. **RSA + the deferred buckets.** Runtime-sized arrays as a thin/unsafe owning pointer (drop length); then
   the standing deferred buckets (`share`/RC-imm interfaces, imm-interface override dispatch, etc.).

Everything here is the **mut region** (`Backend/src/region/unsafe/`), single-owner, non-refcounted —
orthogonal to `share`/RC, which is a separate deferred axis.

## The representation model (architect rulings this session)

- **Declaration default flips to sealed.** `interface Foo` is **sealed/closed by default** — its variants are
  the same-crate types that `impl` it (no *external crate* can). `open interface Foo` opts into non-sealed
  (external crates can impl → unbounded). Today's default is the opposite (unsealed = open); the flip is a
  one-line change at `struct_compiler.rs` (see file map). The `sealed` keyword is then redundant and dropped.
- **Use site picks representation:**
  - `Foo` / `&Foo` → **tagged enum** `{tag, max-payload}`, thin ptr, tag-dispatch. Legal only for a sealed
    (bounded) interface. (Built in Phase 2.)
  - `dyn Foo` / `&dyn Foo` / `Box<dyn Foo>` → **fat pointer** `{contents, itable}`, vtable-dispatch. Legal on
    **any** interface (sealed or open). `open` just *forbids* the enum form.
- **Three kinds, not two.** `Interface(X)` is the canonical receiver of an abstract/virtual method (fat today,
  **thin later**). `DynInterface(X)` = fat pointer. `EnumInterface(X)` (future) = tagged enum. `Interface`,
  `DynInterface`, `EnumInterface` are **distinct kinds that only happen to share a fat representation today**
  and will diverge — never treat them as equal (see Lessons).
- **Dispatch is by narrowing to `Interface`.** A `DynInterface`/`EnumInterface` value is **narrowed to
  `Interface`** — a *real conversion* (`NarrowInterfaceTE`), not an equality. Today, since both are the same
  fat repr, that narrow is a fat→fat pass-through; when `Interface` becomes thin it becomes an obj-ptr
  extract, and `EnumInterface → Interface` is added as a second source form.
- **`dyn X` is a whole type, usable bare.** `&dyn X` = `BorrowRef(DynInterface(X))`; `Box<dyn X>` = the `Box`
  struct wrapping a bare `DynInterface(X)`. So `dyn X` must parse as a plain type (a `Box` type-arg), not only
  behind `&`.
- **`Box<T>` is a dumb builtin for now** (`struct Box<T> { inner T; }`, like `opt.vale`) — it makes the
  frontend straight (`Box<dyn X>` type-checks) and **decouples the frontend from the heap**. It becomes a real
  heap allocation in Phase 4, with no frontend change. Owned interface positions convert to `Box<dyn X>` now
  but stay `#[ignore]`d at runtime until Box heaps.
- **`isa`/`downcast` are compiler-internal** (not user-facing); `try_as` composes them
  (`isa ? Ok(downcast) : Err`). Each dispatches on representation: enum → tag-compare / payload-read; dyn →
  itable-compare / pointer-bitcast.

## Current tree state (verify — it shifts under concurrent sessions)

Run `git status` and read the cupcake plan for the live state; don't trust a file list here. The dyn slices
1–3 + slice-4 dispatch (Part 1) are **uncommitted** in this worktree — the `dyn` keyword, the `DynInterface`
kind, `open`/sealed, the `Box` builtin, and the honest-dispatch machinery (`NarrowInterfaceTE`,
`DynInterfaceTT`, `UpcastInterfaceTE`). **A concurrent session may be working here** — coordinate before large
edits.

The suite is **intentionally red** — run `cargo nextest run --manifest-path Cargo.toml --no-fail-fast`; the
failures are all the deferred owned-`dyn`/box-construction bucket (slice-4b fixtures migrated to `Box<dyn X>`
plus other owned-interface tests). **Borrow checking is globally disabled** (a `// DO NOT SUBMIT` marker in
`test_typing_pass_options` in `compiler_test_compilation.rs` and in `full_compilation.rs`) — it must be
re-enabled before a real commit; the borrow checker's own tests re-enable it via
`compiler_test_compilation_with_borrow_check`.

**Set-aside exploration (NOT the current implementation):** `git stash` entry
`f42e906751b0d8fd88c04dc7e3a3a0bfd2b04788` (tag `wip-ilook-slice1-clever-hinton`) holds an *earlier* attempt
from a parallel planning line — `// ILOOK`/`// ILOOK DUP` markers on every interface test (147 plain / 40
DUP, meant to tag which tests get enum copies in **Phase 2**), an early fat-pointer backend slice, and
`dyn-handoff.md`/`dyn-findings.md` notes. It **predates and conflicts** with the current dyn implementation on
the shared test files — treat it as reference/salvage (esp. the ILOOK enum-duplication catalog for Phase 2),
not something to `apply` blindly. Recover with `git stash apply f42e906751b0d8fd88c04dc7e3a3a0bfd2b04788`
(never `pop` — the stack has other worktrees' entries).

## The cupcake plan (slice 4) — dispatch landed; what's left

Full detail in the plan. Status:
- **Phase 0 (done):** the `same_interface` equality no-op is gone; `Interface(X)` and `DynInterface(X)` are
  never treated as equal.
- **Part 1 (done):** honest `dyn X` dispatch for the current fat-`Interface` phase. `NarrowInterfaceTE`
  (`DynInterface → Interface`, its own instruction, fat→fat pass-through today) is threaded through
  typing/instantiating/groupify/collector/metal/testvm/backend. Resolution recognizes the directional
  `dyn X → X` conversion in `convert` (`convert_helper.rs`), `is_type_convertible`, `compute_upcast_coerced_arg`,
  and the `implements()` isa rule (`infer_compiler.rs`). Probes `dyn_borrow_upcast`, `dyn_dispatch`,
  `dyn_downcast` in `compiler_virtual_tests.rs` are green.
- **Part 2 (deferred):** when `Interface` goes thin / `EnumInterface` arrives, dispatch can't ride the self
  value alone — add `vtable_source_kind: KindT` to `InterfaceFunctionCallTE`/`IE` and **hoist dispatch to the
  call site** (per-call-site source: dyn word #1 vs enum tag). The abstract/super prototype's self stays
  `Interface`.
- **Slice 4b (blocked):** every owned-interface position must become `Box<dyn X>`. Blocked on owned
  `Box<dyn X>` construction — `Box<dyn IShip>(Raza())` fails (`Box`'s constructor param is a bare generic `T`
  and the arg-upcast pass ignores explicit template args, a pre-existing `Box<IShip>(Raza())` limitation), and
  `drop(Box<dyn X>)` isn't found. Migrated fixtures fail here (accepted). `convert` **panics**
  (`update the fixture please`) on a bare `dyn X → X` narrow, so any un-migrated owned-interface fixture is
  caught rather than silently narrowed.

## File map (baseline anchors — cite symbols, verify line numbers)

- **`dyn` parsing (non-core):** `parse_ref_prefix` in `src/parsing/templex_parser.rs` (add a `dyn` word-prefix
  arm beside `own`/`weak`); `ITemplexPT::DynInterface` in `src/parsing/ast/templex.rs`; a `dyn` keyword in
  `src/keywords.rs` (both `new_for_parse` and `new_for_scout` arenas).
- **`DynInterface` kind (core):** three mirrored enums — `KindT` (`src/typing/types/types.rs`), `KindIT`
  (`src/instantiating/ast/types.rs`), metal `Kind` (`Backend/src/metal/types.h`). Ownership is which wrap
  (`OwnRef`/`BorrowRef`) surrounds a bare kind — no separate field — so `&dyn X` and `Box<dyn X>` fall out of
  the existing wraps.
- **`open`/sealed (core for the typing half):** `sealed` plumbs `lexer.rs` (`IAttributeL::SealedAttribute`) →
  `parser.rs` (`IAttributeP::SealedAttribute`) → `post_parser.rs` (`ICitizenAttributeS::Sealed`) →
  `struct_compiler.rs` (computes the bool) → `interface_name_to_sealed` in `compiler_outputs.rs`. `open` is
  the inverse arm at each stage; the default flips at `struct_compiler.rs`. The one policy consumer is
  `compiler_error_humanizer.rs` ("Open (non-sealed) interfaces can't have abstract methods defined outside
  the interface").
- **Dispatch trampoline (verified):** the vtable is fetched only from the self arg's fat-ptr word #1
  (`Backend/src/region/common/common.cpp`, `interfacecall.cpp`); `InterfaceFunctionCallTE`
  (`src/typing/ast/expressions.rs`) is generated inside the abstract method's shared trampoline body
  (`abstract_body_macro.rs`); the caller emits a plain `FunctionCall` to the abstract prototype
  (`call_compiler.rs`). `get_abstract_interface` and the `instantiator.rs` dispatch sites read the
  abstract/super prototype's self and **stay `Interface`-only** (an abstract method is declared on
  `Interface`, never `DynInterface`).
- **`Box` builtin:** `src/builtins/resources/box.vale` (`struct Box<T> { inner T; }`) + registration in
  `src/builtins/builtins.rs`. NB the test-local `struct Box<T>{val T;}` at `compiler_drop_tests.rs` collides
  and must be renamed.
- **Result/Opt rename (slice 5):** definitions in `result.vale`/`opt.vale`; the only compiler string coupling
  is six `intern_str` literals ×2 arenas in `src/keywords.rs` (`opt/some/none/result/ok/err`) —
  `get_result`/`get_option` (`expression_compiler.rs`) resolve via those keyword *fields*, so renaming the
  literals suffices there. Also rename in `.vale` fixtures (~11 under `src/tests/`), embedded-Vale Rust tests,
  and hard-coded `StrI("Result")`/`StrI("Opt")` / `lookup_interface_by_human_name("Opt")` assertions. Suffix
  the variants too (`OkI`/`ErrI`/`SomeI`/`NoneI`) to fully free the enum namespace (architect-approved).

## Design-doc reconciliation (do when convenient)

`valen-design-1.md:1139` (`@CVOZ`) currently says a closed trait's variants are "declared inside; no external
types can implement." The architect's actual rule is "declared inside **the crate**; no external **crates**
can implement" — same-crate `impl`s are the variants — and `dyn` is usable on a sealed interface too. Update
`@CVOZ` and any paraphrase to match.

## Lessons learned

- **Never treat `Interface(X)` and `DynInterface(X)` (or `EnumInterface(X)`) as equal**, even while they share
  a fat representation. They are distinct kinds that will diverge (Interface → thin, one variant → enum). The
  relationship is a **directional conversion** (`dyn X` narrows to `X`), expressed as its own node
  (`NarrowInterfaceTE`), never an `==`/`same_interface` shortcut. An equality no-op passes today and becomes a
  silent miscompile the moment the reps diverge.
- **A dumb builtin `Box<T>` decouples the frontend from the heap.** Introducing `Box` as `struct Box<T>{inner
  T;}` lets `Box<dyn X>` type-check and all frontend `dyn` work land now; the heap allocation is a later,
  frontend-invisible representation change. General pattern: when a hard runtime feature blocks frontend
  progress, a transparent placeholder type unblocks the frontend and defers the runtime.
- **Typing/instantiating are representation-agnostic; only the LLVM backend cares** about enum-vs-fat. So a
  frontend `dyn` conversion can land and keep the suite green without the backend dispatch — the converted
  `&dyn` e2e tests type-check but stay `#[ignore]`d until the backend fat-ptr work lands.
- **`try_as`'s `Result` return is an owned interface value**, which is why downcast depends on enums (a closed
  sum type used inline is a tagged enum). Don't plan downcast before enums.
- **The abstract-method trampoline dispatches on its `Interface` self param**, and the source form
  (dyn/enum) is **per-call-site** — the shared trampoline can't carry it, so once `Interface` goes thin the
  dispatch node must move to the call site with a `vtable_source_kind`.
- **This worktree runs concurrent sessions** and the shared git stash stack holds other worktrees' entries —
  never bare `git stash`/`pop`; use `git stash push -u -m <tag>` + `apply <sha>`, and coordinate before large
  edits. (Stash `f42e9067` is set-aside exploration; see Current tree state.)
- **Interface tests span every tier** (e2e / typing / integration / parse / postparse / instantiating /
  solver). The enum phase only needs *behavior*-test copies (e2e + integration scenarios); parse/postparse/
  typing tests are representation-agnostic and get *modified in place*, not duplicated. The stashed ILOOK/DUP
  markers (stash `f42e9067`) encode which is which, if wanted for Phase 2.
- **An owned interface is always `Box<dyn X>`, never a bare owned interface** — the bare owned/value form is
  reserved for the future enum. `UpcastInterfaceTE` always produces the `dyn` form (its result is derived from
  the target interface — there is no `result_value_kind`), so every owned bare-interface position is a
  migration target. `convert` **panics** on a bare `dyn X → X` narrow so un-migrated fixtures surface loudly.
- **No speculative fallbacks or catch-all arms — panic on the case that shouldn't occur.** Several `_ =>`/`||`
  gates this endeavor were guesses for cases that can't happen yet (a placeholder super in `make_kind_g`, an
  ad-hoc `|| interface_tt().is_some()` in `is_type_convertible`); each should be a `panic!` naming the
  unimplemented case, not a silent guess, so it fails loudly if reached.
- **`DynInterfaceTT` is a distinct interned wrapper around `InterfaceTT`**, not a bare `InterfaceTT`, so the
  `dyn` form can carry more than the interface later. `UpcastTE` is split into `UpcastInterfaceTE` (reference →
  `dyn` fat pointer) and a commented-out future `UpcastEnumTE` (owned concrete → tagged enum).
- **In tests, look a denizen up with `lookup_*` methods** (`lookup_function_by_str`, `lookup_interface_by_human_name`,
  `lookup_impl`), not `coutputs.functions.iter().filter(...)` (architect preference).
