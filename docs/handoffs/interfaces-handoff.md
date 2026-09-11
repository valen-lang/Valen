# Interfaces — master handoff

The **interface rework**: moving Vale interfaces from one representation (everything is a fat pointer)
to a **two-representation model** — a *closed* interface used inline will lower to a **tagged enum**
(thin pointer, tag-dispatch), while `dyn` erases it to a **fat pointer** (vtable) — then building
downcast/`try_as`, heap `Box`, and the rest of the open/closed-trait model on top. Read this whole doc
before touching interface code. The endeavor spans many sessions; everything here is the mut region
(`Backend/src/region/unsafe/`), orthogonal to `share`/RC.

## Where the migration stands — the three questions

**Is the frontend moved to the new syntax?** Yes, fully. All `.vale` fixtures, the builtins, and every
test — running *and* `#[ignore]`d — are on `dyn`/`Box<dyn>` syntax. `try_as`/`try_take_as` return
`Box<dyn ResultI<…>>`.

**No trace of old syntax?** Yes, everywhere — proven two ways. In running code the tripwire *proves* it
(every remaining suite failure is downstream of a deferred blocker, not an unmigrated user site). In the
ignored tests and comments — which the tripwire can't see (it only type-checks compiled code, and virtual
selves are exempt) — grep proves it: `grep -rn "sealed interface" src/ | grep -vE "rust_interop|docs/"`
and the Vale-source `\b(Opt|Result|Some|None|Ok|Err)<` sweep over `.vale` + `.rs` return only lowercase
module names and Rust's own `Result`. The only bare-interface spellings left are by design: the
`interface Foo` declaration and the `virtual self &Foo` receiver (the convergence-point ruling), never a
user value.

**No trace of coming enum syntax?** No enum syntax exists yet (enums are unbuilt), so nothing enum-shaped
to remove. The enum-reserved *names* (`Opt`/`Result`/`Some`/`None`/`Ok`/`Err`) are now freed **everywhere**
— renamed to `OptI`/`ResultI`/… (the `dyn` fat-path versions) across all Vale source, tests, and comments.
The bare `interface Foo` declaration and `&Foo` receiver *are* the future enum's eventual spellings, but
today exist only as the canonical virtual receiver, never as user enum values.

## The `dyn` migration rules (canonical reference)

| Old | New |
|---|---|
| `sealed interface Foo` | `interface Foo` (`sealed` is the default) |
| `interface Foo` external crates impl | `open interface Foo` |
| borrow `&Foo` | `&dyn Foo` |
| weak `&&Foo` / `weak Foo` | `&&dyn Foo` / `weak dyn Foo` |
| owned `Foo` (local/param/return/member/type-arg) | `Box<dyn Foo>` |
| construct owned interface value from concrete `C` | `Box<dyn Foo>(Box<C>(C(...)))` — **double-Box** |

**The one exception:** an abstract/virtual method's **self** stays bare — `virtual self &Foo` /
`virtual self Foo` / `virtual self weak Foo`. Never add `dyn` to a virtual self. A fixture that gains a
`Box` needs `import v.builtins.box.*;` and the loading Rust test needs
`Source::builtin_module(&parse_arena, &parser_keywords, "box")` in its `code_source`.

## The tripwire (enumeration + enforcement; TEMPORARY)

`ICompileErrorT::BareInterfaceUseInDynMigrationT` fires when a bare (non-`dyn`) interface appears where
old syntax lived. It is the driver and the guarantee that no old user syntax survives. It lives in four
files (all tagged `TEMPORARY TRIPWIRE (dyn migration)`): the variant + `range()` arm in
`compiler_error_reporter.rs`, the message in `compiler_error_humanizer.rs`, the **upcast** check at the
top of `convert_via_upcast` in `convert_helper.rs` (catches interface values created by coercion —
construction/return/arg/annotated-local), and the **param** check in `assemble_function_params`
(`function_compiler_middle_layer.rs`, after `evaluate_maybe_virtuality` — non-virtual param whose
`peel_all_references(coord)` is `KindT::Interface`).

**Do NOT remove it yet.** The remaining ~214 hits are all real deferred-blocker fallout, proven by
investigation: 100% of param hits are the generic `drop<T>(x &T)` (drop.vale) monomorphized with
`T`=interface (blocker [2]); the single upcast hit is bound dispatch (blocker [3]); there are **zero**
`struct→interface` upcasts left. "Zero hits → remove tripwire" (Step C) is only reachable once blockers
[1] and [2] land. The return/member/local declaration checks were deliberately **not** added (invasive
for a rare pass-through catch); the ignored-test/comment stragglers they'd have caught were instead
cleared by grep, and that mop-up is complete.

## Current tree state (verify with `git log`/`git status`; shifts under concurrent sessions)

Several `TEMP CHECKPOINT:` commits on `exp-3-wipbx` (pushed; `main` not advanced) carry the migration —
list them with `git log --oneline | grep 'TEMP CHECKPOINT'`; the latest moves the last ignored-test
stragglers off old syntax and frees the reserved names everywhere. Suite is **intentionally red** — run
`cargo nextest run --manifest-path Cargo.toml --no-fail-fast`; the reds are all deferred blockers, and
match the pre-migration baseline count (no net regression). **Borrow-checking is globally disabled** —
three `// DO NOT SUBMIT` markers (`src/pass_manager/full_compilation.rs`, two in
`src/typing/test/compiler_test_compilation.rs`); must be re-enabled before a real (non-temp) commit.

## Plans from here

The frontend `dyn` syntax migration (the sweep) is **done** — every use-site is on `&dyn`/`Box<dyn>`, and
the tripwire proves it. Two threads remain: **A** finishes `dyn`'s deferred blockers (which clears the
intentionally-red suite); **B** is the **paused enum thread**, the natural next pickup now that the sweep
that displaced it is complete. They're independent — A can be finished first, or B resumed directly.

### A. Finish `dyn` (interface support) — unblocks the red suite
1. **Owned `Box<dyn X>` construction** — `Box<dyn X>(Box<C>(C()))` fails "Couldn't find function
   `Box(Box<C>)`". Teach the arg-upcast pass to read explicit template-arg bindings —
   `compute_upcast_coerced_arg` in `src/typing/type_st_match.rs` deliberately ignores them. Canary: the
   `#[ignore]`d probe `dyn_construct_upcast` in `compiler_virtual_tests.rs` (un-ignore when it passes).
2. **Owned-`dyn` drop** — `drop(Box<dyn X>)` isn't found; the owned-drop self-kind rune conflicts. This
   is the vtable consuming-drop (`func drop(self: Box<Self>)` per the design). Clears 100% of the param
   tripwire hits.
3. **Bound dispatch — `BoundCallTE`** (a distinct, principled endeavor). `impl_rule` and
   `method_call_on_generic_data` (`after_regions_tests.rs`): `implements(T, IShip)` + `x.getFuel()`
   internally upcasts `&T → &IShip` (bare). The honest fix is a new `BoundCallTE` node **plus** making
   bounds supply a per-`T` specialized prototype (`&T`-self) instead of the abstract `&IShip` dispatcher
   — so bound dispatch is static/devirtualized, no interface upcast. Load-bearing change spans
   `struct_compiler.rs`/`templata_compiler.rs`/`infer_compiler.rs`/`instantiator.rs`.
4. **Re-enable borrow-checking** — remove the three `// DO NOT SUBMIT` markers. Also needs the
   `group_anon` "borrow with no group and no parameter context" fix
   (`docs/plans/group-generic-closures-plan.md`) — borrow/`dyn` `try_as` trips it; that's *why* the
   checker was globally switched off. Re-enabling also fixes
   `noalias::sole_borrow_param_gets_noalias_same_group_does_not`.
5. **Step C — remove the tripwire** once hits are zero (after 1+2): delete the variant + its two
   accessor/humanizer arms + the two check sites (search `TEMPORARY TRIPWIRE (dyn migration)`).
6. **Heap `Box`** — `Box<T>` is currently a dumb inline builtin (`struct Box<T>{inner T;}`,
   `src/builtins/resources/box.vale`). Make it actually heap-allocate; wire `Box<Concrete> → Box<dyn X>`
   unsizing and the vtable-driven free.

### B. Enums (the future representation) — the paused thread, the next pickup
This is the work that was paused to run the `dyn` migration first. **Resume from** the enum plan
`~/.claude/plans/please-plan-out-slice-clever-hinton.md` (tagged-enum design + slice breakdown) plus the
`// ILOOK`/`ILOOK DUP` inventory in stash `f42e9067` (B.6). The design is already settled — two-representation
model, `isa`/`downcast` primitives, the `@CVOZ` crate-boundary ruling, sealed-by-default; the steps below
are what building it entails.
1. **Tagged-enum representation** for a bare/closed interface: `{tag, max-payload}` layout, construction
   (branch the struct→interface upcast into a tag+payload write), `&Foo` = thin pointer, tag-dispatch via
   `LLVMBuildSwitch`, enum drop (switch on tag → drop the live variant).
2. **`isa` / `downcast` metal primitives** (compiler-internal); rewrite `try_as` as
   `isa ? OkI(downcast) : ErrI`.
3. **Flip bare `Interface` from fat to the enum path** — introduce an `EnumInterface` kind; a
   non-virtual `&MyInterface` goes **thin**: a pointer to the whole enum `{tag, payload}`, dispatched by
   loading the tag and switching. A `virtual self &MyInterface` is different — it receives a **plain
   pointer to the concrete variant struct** that lived inside the enum (an ordinary `&Concrete`), not the
   whole enum and not the tag. That's why the bare `&Foo` receiver is the convergence point: the enum
   caller switches on the tag then passes a pointer to the inner struct, and the `&dyn` caller passes its
   `contents_ptr` — both hand the callee the same plain pointer-to-concrete. This is where
   `NarrowInterfaceTE` stops being a fat→fat pass-through and becomes a real narrow, and where the
   enum-source form is added.
4. **Cupcake Part 2 (deferred)** — add `vtable_source_kind: KindT` to `InterfaceFunctionCallTE`/`IE` and
   hoist dispatch to the call site (needed once `Interface` goes thin / `EnumInterface` arrives, because
   the shared abstract-method trampoline can't carry the per-call-site source). Plan:
   `~/.claude/plans/principled-but-please-plan-atomic-cupcake.md`.
5. **Decide canonical `Result`/`Opt`** — they're now free names; make them the enum sum types (the
   `OptI`/`ResultI` `dyn` versions stay for the fat path). Repoint builtins/tests as chosen.
6. **Enum behavior tests** — copies of the interface behavior tests using bare-sealed interfaces. The
   `// ILOOK`/`ILOOK DUP` markers (147 plain / 40 DUP) catalog which tests get enum twins; they live in
   `git stash` `f42e906751b0d8fd88c04dc7e3a3a0bfd2b04788` (tag `wip-ilook-slice1-clever-hinton`). **Do NOT
   `git stash apply` it to resume** — the `dyn`/slice-1 work it also carries was redone independently and is
   already committed, so applying it conflicts. Mine it read-only for the ILOOK inventory
   (`git stash show -p f42e9067…`).

### C. Deferred buckets (broader, standing)
RSA (runtime-sized arrays) as a thin/unsafe owning pointer; `share`/RC-imm interfaces; imm-interface
override dispatch; the other `deferred:` e2e buckets (`grep -rn 'deferred:' src/end_to_end_tests/`).

## Key design rulings (do not re-litigate)
- **A bare interface can't be an owned user value.** The user holds `Box<dyn X>` or `&dyn X`. Bare
  `Interface` exists only as the canonical virtual receiver.
- **The bare `&Foo` receiver is the CONVERGENCE POINT** both a future enum caller and a `&dyn` caller
  narrow into. A `virtual self &Foo` receives a **plain pointer to the concrete variant struct** (an
  ordinary `&Concrete`) — not the whole enum, not the tag. The enum caller switches on the tag then
  passes a pointer to the inner struct; the `&dyn` caller passes its `contents_ptr`; both hand over the
  same plain pointer-to-concrete. So the override-dispatcher self and `drop<T=interface>` must **stay
  bare** — do not exempt them or make them `&dyn`; that would sever the enum path. The tripwire firing on
  them is correct (they're blocked on [1]/[2], not old syntax).
- **`Interface`, `DynInterface`, `EnumInterface` are distinct kinds** that share a fat representation
  today and will diverge. The relationship is a directional conversion (`NarrowInterfaceTE`), never `==`.
- **`dyn X` = `BorrowRef(DynInterface(X))` / `Box<Dyn(X)>`**, wrapping a distinct interned `DynInterfaceTT`
  (holds an `InterfaceTT`), not a bare `InterfaceTT`.
- **Design-doc fix owed:** `valen-design-1.md` `@CVOZ` says a closed trait's variants are "declared
  inside; no external types can implement" — the rule is "declared inside **the crate**; no external
  **crates** can implement" (same-crate impls are the variants), and `dyn` is usable on a sealed
  interface too.

## File map (cite symbols; verify lines)
- Tripwire: `convert_helper.rs` (`convert_via_upcast`), `function_compiler_middle_layer.rs`
  (`assemble_function_params`), `compiler_error_reporter.rs`, `compiler_error_humanizer.rs`.
- `dyn` kind: `KindT` (`src/typing/types/types.rs`), `KindIT` (`src/instantiating/ast/types.rs`), metal
  `Kind` (`Backend/src/metal/types.h`); `DynInterfaceTT`, `NarrowInterfaceTE`, `UpcastInterfaceTE`.
- Rename coupling: six `intern_str` literals ×2 arenas in `src/keywords.rs`; `get_result`/`get_option`
  in `src/typing/expression/expression_compiler.rs` resolve via keyword fields. Builtins:
  `src/builtins/resources/{opt,result,as,weak,box}.vale`.
- Blocker [1]: `compute_upcast_coerced_arg` in `src/typing/type_st_match.rs`.

## Lessons learned
- **Owned-interface construction is double-Box:** `Box<dyn Foo>(Box<Concrete>(Concrete()))`, not
  `Box<dyn Foo>(Concrete())`. This is the form blocker [1] is written to accept.
- **The tripwire enumerates by kind, not name — trust it, not grep.** The same name is a struct in one
  test and an interface in another (`Bork`), so text-sweeping across files corrupts the struct sites.
- **The tripwire only sees compiled code.** Ignored tests and virtual-self-only declarations slip it;
  a closing grep is required for true "no trace" — the enum phase's new tests will slip it the same way.
- **Never exempt the dispatcher-self / `drop<T=interface>` to silence the tripwire** — they are the
  enum/dyn convergence point and must stay bare. Silencing them severs the future enum path.
- **`sealed interface `→`interface` must keep the space** — a no-trailing-space sweep produced glued
  `interfaceOpt`; corrective `\binterface([A-Z])` → `interface \1` (leaves `interface_def`/`interfaces`).
- **`rust_interop`/`docs` `sealed interface` mentions are prose** describing a Rust concept — leave them.
- **Print-and-continue defeats first-error masking** when auditing which tripwire hits are genuine vs
  deferred-blocker fallout (used to prove zero genuine sites remain).
