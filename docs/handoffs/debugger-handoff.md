# Debugger arc — handoff

**M1 + M2 + M3 + Layer-1 (struct) + pointer-to-struct + Layer-1.5 (SSA arrays) landed and validated.**
Function-level DWARF (backtraces resolve `file.vale:N`), per-statement stepping (`step` lands on
distinct lines), local-variable inspection (`frame variable x` for primitives), inline-struct field
walking (`frame variable -P 1 s` shows `s`'s fields), borrow-ref struct locals (`ref = &s`) walking
their pointee's fields, and static-sized-array locals (`a = [#](…)`) walking their elements
(`frame variable -P 1 a`) all emit real DWARF and pass live lldb gates. Remaining composite Layers —
strings, interfaces, and runtime-sized arrays (RSA, deferred by standing order) — are not started. The
debugger models the user-visible value and **ignores control blocks**, so weak/boxed refs (the cases
that box a control block between the pointer and the data) are out of scope until they're un-deferred.
Full design:
`docs/architecture/debugging-architecture.md`. This arc is a hand-port of an older `debugging`
branch from a pre-move Vale checkout (no shared git history; ported by content); the per-commit
patches are in `tmp/debugging-patches/` and the squash in `tmp/debugging-arc-squashed.diff` —
reference material, not applied as-is (our metal instruction set differs from the arc's).

## What landed (code)

- **Ranges on every node.** Every `ExpressionTE`/`ExpressionIE` variant carries `pub range:
  RangeS<'s>` + a `range()` accessor; the instantiator copies TE→IE range at every construction site.
- **Metal carries the source location, required at construction.** The C++ `Expression` base,
  `Function`, and `Local` all take a `SourceLocation*` as a **ctor parameter** — no default, no
  setter. So a metal node cannot be built without deciding its location (null = explicit "synthetic",
  a real handle otherwise); forgetting is a compile error. Every `Expression` subclass ctor threads it
  to the base; the FFI builders in `metal_cache_ffi.cpp` pass it as the first ctor arg
  (`new X(srcloc(source_loc), …)`, the same idiom the file uses for `Function`/`Local`). This replaced
  an earlier default-null field + post-construction setter, which let a new builder silently skip it.
- **Lowerer** resolves a `SourceLocation` per node (`metal_lowerer.rs::loc_of`) and passes it to every
  `metal_expr_*`. `lower_function` anchors the function's location to its body's range
  (`self.loc_of(&f.body.range())`). The body IE's range starts at the function *declaration*, so the
  DISubprogram's `DW_AT_decl_line` resolves to the `func`-keyword line even when the `{` is on a later
  line — validated by a multi-line-signature gate. (`FunctionHeaderI` still has no range of its own.)
- **Backend DWARF emission** (core, all in `Backend/src/function/debugging.{h,cpp}` — the one home for
  every DWARF function): `initDebugInfo` (called from `createModule` in vale.cpp) builds a DIBuilder +
  `Dwarf Version`/`Debug Info Version` module flags; `finalizeDebugInfo` (called from `finalizeCompile`)
  calls `LLVMDIBuilderFinalize`. `attachDISubprogram` emits a per-function `DISubprogram` + lazy
  `DICompileUnit`/`DIFile` (M1); `translateExpressionInner` (expression.cpp) sets a per-expression
  `DILocation` guarded by `ScopedDebugLoc` (M2); **`makeHammerLocal`** (shared.cpp) — the single point
  where every local's alloca is created — calls `emitLocalVariableDebugInfo` to emit a
  `DILocalVariable` + `llvm.dbg.declare` for that local, so no local can slip through undeclared (a
  per-call-site emission did, and missed the destructure paths).
  It reads the location off the metal **`Local`** itself (`Local::sourceLocation`, set by
  `metal_cache_get_local` and threaded from the lowerer's `lower_local`/`lower_locals`), and picks its
  DIType by the local's storage `translateType`: an inline value uses `getOrCreateDIType` (Int/Bool/Float
  precise; StructKind → a flattened inner-struct `DICompositeType`, Layer 1; StaticSizedArrayT → a
  `DW_TAG_array_type` over the element DIType + a subrange of count N, Layer 1.5; else opaque "ref"); a
  pointer local uses `getOrCreateDIPointerType`, which returns a `DW_TAG_pointer_type` to the
  inner-struct composite for a borrow ref to a struct (so `frame variable -P 1` walks the fields) and
  the opaque "ref" otherwise. A borrow lowers to a pointer straight to the inner struct in this
  region — no control block in between, so no wrapper flattening (M3 + Layer-1 + pointer-to-struct).
  `--debug` flows valec `-g` → `opt->debug`.
- **Real DWARF source paths (`DW_AT_comp_dir`).** The frontend conveys a `basename → absolute path`
  map to the backend so DWARF records where source actually lives (was `comp_dir "."` + basename,
  which meant lldb couldn't `list`/show source, and source-pattern breakpoints `br s -p` failed). The
  path is captured verbatim at `build_inputs_code_map` (`pass_manager.rs`), rides `BackendInputs.source_paths`
  through the FFI, is stored on the metal **`Program`** (`Program::sourcePaths`, populated by
  `loadSourcePaths` in vale.cpp), and `getOrCreateDIFile` (debugging.cpp) resolves the incoming
  basename against `globalState->program->sourcePaths`. `getOrCreateCompileUnit` anchors the CU to the
  lexically-first **user** file (in the map), not whatever function compiled first — a builtin
  (`arith.vale`) anchor left `comp_dir "."` and varied with compile order.
  (The paths are conveyed via the `BackendInputs` compile-input FFI today; they arguably belong on
  the program-builder FFI since they're program metadata — an open cleanup.) The frontend `SourceLocation.filePath` stays the
  basename (identity/`is_test()`/diagnostics unchanged; the absolute path lands only in DWARF, so
  temp-dir paths never reach anything a test asserts).
- **Harness + gates.** In `src/end_to_end_tests/mod.rs`: `compile_inline_debug` + dsymutil,
  `lldb_check` (unordered substrings), `lldb_check_ordered` (asserts the *sequence* of stops),
  `dwarfdump_capture` (raw DIE assertions, drift-resistant). 26 gates in
  `src/end_to_end_tests/tests/debugger.rs`: 4 M1 backtrace/line, 6 M2 stepping (ordered), 1 structural
  DWARF (CU + producer + `:main` subprogram), 1 decl-line boundary (multi-line signature → line 1),
  1 backtrace-depth (caller+callee frames), 1 array line-stepping, 3 M3 (`frame variable` for an int
  local, a function arg, and bool+float), 3 Layer-1 struct (single field, mixed multifield, and a
  `dwarfdump` DIE-shape gate), 1 destructure (`[x, y] = ...` locals are inspectable), 2
  pointer-to-struct (a borrow-ref struct local walks its fields via `-P 1`; a `dwarfdump`
  pointer-to-structure DIE gate), 2 Layer-1.5 SSA array (a static-sized-array local walks its
  elements via `-P 1`; a `dwarfdump` array-type/subrange DIE gate), and 1 sentinel-breakpoint probe
  (a `0; // lldb breakpoint: <label>` line binds via `br s -p`, stopping at the sentinel with prior
  state visible) — 26 total.
  Native-only: they skip loudly under wasi via `skip_non_native()` (an eprintln, since libtest has no
  runtime skip status).
- **e2e behavior tests double as real debug gates.** `assert_compile_and_run_dbg` /
  `assert_inline_compile_and_run_dbg` (mod.rs) compile+run exactly as the plain helpers, and on
  Native additionally drive a scoped lldb session: a `&[Step]` script where each `cmd`/`expect`/
  `reject` step asserts against *that command's own output* (the combined `lldb -b` capture is split
  on the `(lldb) <cmd>` echoes — see `split_lldb_session`), so a value can't match a breakpoint echo
  or another step's text. 25 supported live tests (`misc.rs` 11, `structs.rs` 5, `ifelse.rs` 3,
  `arrays.rs` 2 SSA, `inline.rs` 3, `while_loop.rs` 1) assert **per-program behavior** — a mutated
  local's before/after value, struct-field and array-element walks, a loop counter incrementing
  across `continue`s, the taken branch line, a two-frame (helper←main) backtrace. Separate from the
  25 dedicated gates in `debugger.rs`. Five are shape-limited to a stepping/backtrace gate because
  their programs have no inspectable state (`structmutfield`/`bigstructmutfield`/`structmut` build
  the struct as an unnamed inline temporary; the trivial inline `return 3` / `Some<int>(3)`) — the
  generated-function work below would let the struct ones inspect their constructor args.
  Every interior breakpoint anchors on a labeled `0; // lldb breakpoint: <label>` sentinel line in
  the fixture via `br s -p '<label>' -f <basename>` (line-number-robust; the discarded `0;` is a
  no-op); the only non-sentinel breakpoints are function-entry ones (`b :main` / `b helper`). No gate
  uses `b file:line`. Branch and reachability gates (`ifelse`, `panic`/`panicnot`/`unreachablemoot`)
  put a sentinel in the branch that must NOT run and assert its breakpoint never fires. Fixtures are
  copied into a temp dir and compiled from there (test-review #9), which is also what gives DWARF a
  real `DW_AT_comp_dir`. The debug half is Native-gated inside the helper, so under
  wasi these run exit-code-only (loud skip). Every e2e test the debugger can't yet cover carries a
  `// VDBG:` marker (see coverage gaps).
- `line_col_in` is the single line/column primitive in `source_code_utils.rs`.

## What's next

All core (`Backend/`, needs `fire core edits`) unless noted:

1. **Remaining composite Layers.** Inline structs walk (`getOrCreateDIStructType`), borrow-ref struct
   locals walk via a pointer-to-composite (`getOrCreateDIPointerType`), and static-sized arrays walk
   via a `DW_TAG_array_type` (`getOrCreateDIArrayType`); still to do:
   - **Strings (1.7), interfaces (1.6).** Per-kind DIType builders off the element/inner layout
     (`__Str` inner / `getInterfaceRefStruct`), each behind a `frame variable -P N` or `dwarfdump`
     gate, pointing at the user data past any control block. Recursive structs need the RAUW
     forward-decl pattern (`getOrCreateDIStructType` already does this).
   - **Runtime-sized arrays (RSA) — deferred.** RSA is deferred by standing order on this branch
     (every RSA e2e test in `src/end_to_end_tests/tests/arrays.rs` is `#[ignore]`d, one fixture
     missing), so it's not reachable or testable here. Its local is a pointer to a
     `{ controlBlock, i32 size, [i32 capacity]?, [0 x elem] }` wrapper with a runtime count (in the
     `size` member, not DWARF) — revisit only when RSA is un-deferred.
2. **Un-VDBG as kinds land (non-core).** The reachable form of item 8 breadth is done: the 25
   supported live tests carry per-program behavior debug gates and every unsupported
   program-behavior test is
   `// VDBG:`-marked (coverage gaps below). When a deferred kind lands (strings/share, interfaces,
   RSA) or the FFI/closure debug paths are proven, drop that test's `// VDBG:` and give it a real
   gate. The original "required lldb param on every `assert_*`" form was dropped deliberately — it
   would force unvalidatable red gates onto deferred kinds.
3. **Interface debug coverage (non-core, blocked).** An interface upcast / virtual-dispatch stepping
   gate is wanted (the loc plumbing on those builders is untested), but interface compilation is
   deferred on this branch — the `virtuals` e2e tests are `#[ignore]`d and the instantiated humanizer
   panics on interface programs. Add the gate when interface compilation is un-deferred.
4. **Debugging generated functions** (design below). The next milestone — makes constructors/drops
   debuggable, and is what lets `structmutfield`/`bigstructmutfield`/`structmut` inspect ctor args.

## Future: debugging generated functions (constructors, drops, thunks)

Generated functions (a struct's constructor — the symbol is `_Vec3i(i32,i32,i32)` — its `.drop`,
interface thunks, lambda `__call`, etc.) are **real functions in the binary but have no DWARF**: the
frontend gives them a null `sourceLocation`, so `attachDISubprogram` bails. You *can* already break
on one by its mangled symbol (`br set -n`), but there's no source-level break, no file:line
backtrace, and no named args (`frame variable` in the ctor frame shows nothing). Making them
debuggable is the natural next capability, and it's what "break on the constructor and print its
args (`x=4, y=5, z=6`)" needs.

Key design decision: **a generated function should inherit the source location of the definition it
was generated for** — a `Vec3i` constructor/drop points at `struct Vec3i { … }`. (Rejected: line 0 /
synthetic — ugly backtraces, no source; the call site — a generated fn has many call sites, no single
home.) Cleanest architecture: give metal **type definitions** (`StructDefinition`/`InterfaceDefinition`)
a source location (the `struct`/`interface` keyword range), and have generated functions inherit
their origin definition's location — set at synthesis in the frontend (where the origin is in hand),
not derived in the backend.

Tiers (pick how far):
- **Tier 1 — break + backtrace:** generated fn gets its origin's decl line → `bt` resolves,
  break-by-source works. Small once definitions carry locations.
- **Tier 2 — + named args** (the "print the args" goal): the ctor's params get `DILocalVariable`s.
  Likely *nearly free* after Tier 1 **if** generated-function params flow through `makeHammerLocal`
  like user args do (the M3 "frame variable for a function arg" gate proves args work once a function
  has a `DISubprogram`) — verify this.
- **Tier 3 — step through the generated body:** synthesized statements have no real lines; skip
  initially (low value).

Open questions to resolve before committing:
- **Which generated kinds** — start with constructors + drops, or all (interface thunks, lambda
  `__call`, generic instantiations)? Each has an origin; the set is broader than it first looks.
- **No-origin generated functions** (`__vale_main`, deep builtins) — fall back to null (undebugged)
  or a builtin location.
- **Param locations** — point every param at the struct-def line, or at each field's own line
  (`x int;`)? Field lines are nicer but more threading.
- **Audit what else keys on `sourceLocation == null` meaning "synthetic"** — giving generated
  functions real locations must not change unrelated behavior.

Lean: Tier 2, constructors + drops first, via *definitions carry a location + generated functions
inherit it*. Write it up as a short design doc and verify the two load-bearing unknowns (does the
arg-DWARF path fire for generated-fn params; what else reads null-location) first.

## Debugger coverage gaps (`// VDBG:`)

The debugger gates only kinds it provably walks today. Every other program-behavior e2e test is
marked `// VDBG: no debugger gate yet — …`; regenerate the live count with
`grep -rc '// VDBG:' src/end_to_end_tests/tests/`. To close a gap: land the kind (or prove the
path), then replace the marked test's plain `assert_*` call with the `_dbg` variant and drop the
marker. Not yet supported, by kind:

- **strings / share** — all of `strings.rs`, the share-deferred cases in `externs.rs`, `structs.rs`
  `memberrefcount`, `inline.rs` `string_len`. Blocked on the share region.
- **interfaces / upcast / downcast / virtuals** — all of `downcast.rs`, `virtuals.rs`, the interface
  cases in `externs.rs`, `ifelse.rs` `upcastif`. Blocked on interface compilation (deferred).
- **runtime-sized arrays (RSA)** — the RSA cases in `arrays.rs` and `externs.rs`. Deferred by
  standing order.
- **SSA across the C extern boundary by value** — `externs.rs` `ssamutreturnexport` and
  `feature_arr_read_ssa`.
- **FFI / native-C debug path** — the live `externs.rs` roundtrips (`simpleexternreturn`,
  `simpleexternparam`, `structmutparamexport`, `ssamutparamexport`) and both `native_walker.rs`
  tests. Not deferred: the debugger *supports* their kinds, but a DWARF/lldb gate through native-C
  linking + dSYM is unproven. First candidate to un-VDBG.
- **closures** — `lambdas.rs`, `arrays.rs` `ssamutdestroyintocallable`. No closure DIType yet.

Not candidates (no gate, no marker): `noalias.rs` (LLVM-IR golden), `extern_header_goldens.rs`
(header golden), `mod.rs` (harness self-tests) — none are exit-code program tests.

## State / gotchas

- **Suite is green with the gates in it.** Regenerate: `cargo nextest run --manifest-path Cargo.toml`
  (917 pass) and its `VALE_TEST_BACKEND=wasi` variant (gates skip). Debugger gates need macOS lldb +
  dsymutil + `llvm-dwarfdump` (PATH, else `/opt/homebrew/opt/llvm/bin/`); they run only on Native.
- **Intermittent "leaky" warnings** on lldb-driven tests (0/1/2 across runs, always still passing):
  `lldb_capture` leaves the inferior stopped at a breakpoint when the batch session exits, and
  nextest's leak detector catches the lingering process on some timings. Harmless but nondeterministic;
  the fix (not yet applied) is to append `kill` as the last command in `lldb_capture` so every session
  terminates its inferior.
- **Backend is core.** `Backend/` edits need the architect's literal `fire core edits`.
- **A function needs a non-empty source location or the module gets NO DWARF.** An empty file path
  makes `metal_cache_get_source_location` return null → metal `Function.sourceLocation` null →
  `attachDISubprogram` bails → no `DISubprogram`/`DICompileUnit` at all → empty dSYM. Per-expression
  DILocations also need the subprogram (they read `LLVMGetSubprogram(containingFuncL)`).
- **The arc never calls `LLVMDIBuilderFinalize`** — we add it in `finalizeCompile`. Without it DWARF
  is malformed/empty.
- Touch `build.rs` after any backend edit to force the C++ recompile (cmake-rs output isn't on stdout).
- The rust-interop lowerer path passes an **empty** code_map (interop debugging out of scope) → its
  locs are null; DWARF simply isn't emitted there, which is fine.

## Lessons Learned

- A bulk `replace_all` that rewrites a call-expression prefix will also rewrite that same expression
  **inside a helper you just defined with it** — here it turned `withLoc`'s own body into an infinite
  self-call. Define such helpers *after* the sweep, or exclude their bodies; a build passes but any
  runtime call stack-overflows.
- Do not attribute a stack overflow to "the migration deferred this" without a backtrace. The
  compile+run pipeline works for trivial programs today; the overflow that looked pre-existing was the
  `withLoc` self-recursion. `lldb -b -o run -o "bt N"` on the nextest binary gives the real frame.
- Measure a suspected approximation before documenting it as wrong: the function-loc "anchors to the
  body, not the decl line" concern was false — the body IE's range starts at the declaration, so
  `DW_AT_decl_line` is the `func` line. A `dwarfdump` gate on a multi-line signature settled it.
- Assert stepping as an **ordered** sequence (`lldb_check_ordered`), not a set of substrings: an
  unordered `contains` set passes even if stops arrive out of order or a line appears incidentally.
- Emit a local's DWARF at the chokepoint that creates it (`makeHammerLocal`), not at each caller. The
  first cut emitted it in the `Stackify`/`LetAndLend` arms and silently missed the two destructure
  call sites — `[x, y] = …` locals were invisible in lldb. If N call sites must each remember to do X,
  one eventually won't; put X where the N converge.
- A source location is intrinsic to a node, so make it a **constructor requirement**, never a
  default-null field set afterward. The original `Expression::sourceLocation = nullptr` + post-`new`
  `withLoc` setter let a new builder silently omit it. Threading it through all 47 subclass ctors (the
  atomic seal) turns "forgot the location" into a compile error — the compiler is the safety net for
  the cascade itself (a subclass that doesn't call the base ctor won't build).
- "IE carries range" was true for only 5 of ~45 variants when first surveyed; verify a field's presence
  variant-by-variant, not from one grep hit.
- `Backend/` is core and needs `fire core edits`; this surfaces only when you first read a `Backend/`
  file (its `.claude/CLAUDE.md`), so check before planning C++ work, not mid-edit.
- Verify a region's `translateType` before assuming a pointer local boxes a control block. On this
  branch a borrow ref lowers to a pointer *straight to the inner struct* (no wrapper in between), so
  the arc's "heap struct needs the wrapper composite + flattened control block" plan did not apply —
  a plain `DW_TAG_pointer_type` to the inner composite is all it takes.
- Before planning a debug gate for a kind, confirm a **non-ignored** e2e test compiles+runs that kind
  on this branch. Most runtime-sized-array, interface, and weak-ref fixtures are `#[ignore]`d
  (onion/borrow migration) and a gate against them can't be validated (some panic at compile); only
  static-sized arrays, primitives, structs, and borrow refs are live today.
- e2e debug breadth covers only kinds the debugger provably walks; a test for a deferred or unproven
  kind gets a `// VDBG:` marker (a visible IOU), never a real gate — so the gap shows in-code and is
  enumerated in the coverage-gaps section. This is why item 8's "required lldb param on every
  `assert_*`" form was dropped: a mandatory param would force red gates onto strings/interfaces/RSA
  that can't be validated on this branch.
- A debug gate must assert the program's *actual behavior*, not just that DWARF exists. The first cut
  of the breadth gates was a uniform `b :main; run; bt` smoke check — it passed on every program
  while testing nothing (a function-call program never checked the call, a struct program never
  walked a field). Each gate now drives a program-specific session. Scope expectations to the
  producing command (the `Step`/`split_lldb_session` split) so a value can't match a breakpoint echo.
- Calibrate lldb sessions by running, not by reading source. A breakpoint stops *before* its line
  executes, so place a value sentinel *after* the mutation you want to observe (and a separate
  "before" sentinel to read the prior state). Non-exported functions get a bare DWARF name (`helper`,
  not `:helper` — only the exported entry is `:main`). Both surfaced as real gate failures on the
  first run.
- A generated function (constructor, drop, thunk) is a real function but a debug **black hole** — it
  has a symbol but no DWARF (null `sourceLocation` → `attachDISubprogram` bails), so you can't
  break-by-source or inspect its args. Do not assume a struct-construction program is debuggable via
  its constructor; it isn't yet (see the generated-functions design section).
- `DW_AT_comp_dir` must be a real absolute directory, or lldb can't read source: it can't `list`,
  and `br s -p` (source-pattern breakpoints) fail with "no locations" even when the line table is
  correct. Two layers bit us: the frontend dropped the directory (basename only → `comp_dir "."`),
  and the CU anchored to whichever function compiled first — a builtin (`arith.vale`) for some
  programs, so `comp_dir` was both wrong and compile-order-dependent (a latent nondeterminism).
  Fix: convey a `basename → abspath` map to the backend and anchor the CU to a user file. A bare
  `0;` statement compiles (the discarded int auto-drops) and gets its own line-table row, making it a
  usable breakpoint sentinel.
- Architect preferences (generalized): the debugger models the user-visible value and **ignores
  control blocks** — point at the inner-struct composite / the array elements, never describe the
  control-block header; represent "absent" with an existing marker (empty file path) rather than
  wrapping in `Option`; dedupe onto one primitive instead of adding a parallel helper; prefer coverage
  via existing integration tests over dedicated `#[cfg(test)]` units.
