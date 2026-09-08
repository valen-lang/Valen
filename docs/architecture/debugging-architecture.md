# Debugger Architecture

Source-level lldb debugging for Vale programs (file:line backtraces, per-statement
stepping, local/field inspection) via DWARF, targeting `-O0`.

## Status (read first)

This is being brought in **in steps**. What exists on this branch today:

- **Source-location plumbing — built.** Every typed and instantiated expression carries a
  source `RangeS`, the lowerer resolves each to a `SourceLocation`, and that crosses the FFI
  into the backend on every instruction and function (stored on the C++ `Expression`/`Function`).
- **M1 (function DWARF) + M2 (stepping) + M3 (locals) + Layer-1 (struct) + pointer-to-struct +
  Layer-1.5 (SSA arrays) — built and validated.** `--debug` builds a `DIBuilder` + module flags, emits
  a `DISubprogram` per function, a `DILocation` per expression, a `DILocalVariable` + `llvm.dbg.declare`
  per local, a flattened `DICompositeType` for inline structs, a `DW_TAG_pointer_type` to that composite
  for a borrow-ref struct local, and a `DW_TAG_array_type` for a static-sized-array local;
  `LLVMDIBuilderFinalize` runs before codegen. Backtraces resolve `file.vale:N`, `step` advances by
  source line, `frame variable x` shows a primitive's value, and `frame variable -P 1` walks the fields
  of an inline struct or a borrow-ref struct local and the elements of a static-sized-array local.
  Proven live by 25 lldb/dwarfdump gates in `src/end_to_end_tests/tests/debugger.rs` (Native only; they
  skip under wasi).
- **Remaining composite Layers — NOT built.** Strings, interfaces, and runtime-sized arrays (RSA,
  deferred by standing order) still show as an opaque pointer rather than walking their fields. See
  [Deferred](#deferred-composite-type-layers).

## Scope and goals (the eventual capability)

A `lldb`-driven source-level experience for programs compiled with `--debug` at `-O0`:

- File:line backtraces (`bt` shows `:main at file.vale:N`).
- Per-statement stepping (step advances by source line, not by instruction).
- Local variable inspection (`frame variable x`), with per-type rendering for primitives,
  structs, arrays, strings, and interface fat pointers.
- Field access via `frame variable -P N`.

Out of scope: optimized-build debugging (`-O0` only), a DAP shim for IDEs, lldb pretty-printers
that hide artificial runtime fields, and source lines on type definitions.

## Pipeline and source-location flow

This repo's pipeline has **no `final_ast`/`ProgramH` and no hammer/`simplifying` stage** — those
were removed in the instantiating bring-up. `HinputsI` (the instantiated IR) is the sole backend
contract, and a thin FFI lowerer translates it straight into the backend's metal IR:

```
RangeL (lexer byte offsets)
  → RangeS<'s>         typing pass — file pointer + begin/end offsets
    → ExpressionTE     every typed-expression variant has `pub range: RangeS<'s>`
      → ExpressionIE   instantiator copies TE.range into every IE node's `range`
        → metal_lowerer.rs   resolves RangeS via code_map → (file, line, col),
                             interns a SourceLocation, passes it to each builder
          → metal_cache_ffi   SourceLocation crosses the C ABI on every metal_expr_* + function
            → C++ backend     stores nothing yet — the handle is accepted and discarded
```

### RangeS → (line, col)

`src/utils/source_code_utils.rs` owns one line/column primitive, `line_col_in(source, pos)`
(1-based; clamps past-the-end). Both `humanize_pos_path` (error humanization) and
`resolve_line_col` (code_map lookup, used by the lowerer) route through it — there is no second
line/column computation in the tree.

### Frontend: range on every node

- **Typing** (`src/typing/ast/expressions.rs`): every `ExpressionTE` variant carries
  `pub range: RangeS<'s>` with a `range()` accessor. (`FunctionCallTE.range` is a `&[RangeS]`
  slice, an intentional exception.)
- **Instantiating** (`src/instantiating/ast/expressions.rs`): every `ExpressionIE` variant
  carries `pub range: RangeS<'s>` with a `range()` accessor; the unit/no-lifetime variants
  (`BreakIE`, `VoidLiteralIE`, `ConstantIntIE`, `ConstantBoolIE`, `ConstantFloatIE`) carry an
  `<'s>` lifetime for it. `src/instantiating/instantiator.rs` populates each from the source TE
  node (`FunctionCall` takes the first of its range slice; a synthetic fallback covers an empty
  slice).

### Lowerer: RangeS → SourceLocation

`src/backend_ffi/metal_lowerer.rs` holds a `code_map: &FileCoordinateMap` and resolves one loc
per expression via `loc_of(&expr.range())`:

- `offset >= 0` → `cache.get_source_location(file_path, line, col)`.
- synthetic (`offset < 0`) or a function with no range → the empty `("", 0, 0)` "no source info"
  location.

`populate_metal_cache` takes the `code_map` (from `get_code_map()`); the standalone path
(`pass_manager.rs`) passes the real `vale_code_map`, the rust-interop path passes an empty map
(interop debugging is out of scope).

### FFI contract

- `SourceLocation` (`Backend/src/metal/instructions.h`): `{ std::string filePath; int32_t line;
  int32_t col; }`. Interned by `MetalCache::getSourceLocation(file, line, col)` keyed on
  `file:line:col` (`Backend/src/metal/metalcache.h`).
- `metal_cache_get_source_location(...)` interns and returns a handle;
  **every** `metal_expr_*` (46 of them) and `metal_function_new` take a trailing
  `SourceLocationHandle*` (`Backend/src/metal/metal_cache_ffi.{h,cpp}`).
- Rust mirror (`src/backend_ffi/metal_cache.rs`): a `SourceLocation<'cache>` newtype,
  `get_source_location`, and a non-optional `loc: SourceLocation` on every builder + `new_function`.
  An empty file path is the "no source info" marker (not a null handle) — the interner always
  returns a valid handle.
- `--debug` flows as `debug: bool` through `BackendCompileOptions` /
  `BackendCompileOptionsFFI` (Rust `src/backend_ffi/mod.rs` mirror + C++
  `Backend/src/backend_options_ffi.h`), wired from valec's existing `-g` flag
  (`src/bin/valec/{build,midas}.rs`).

## Backend no-op ABI layer

The C++ backend is "core" (see `Backend/.claude/CLAUDE.md`). This step added only the minimal
ABI so the Rust side links, all of it accept-and-ignore:

- `SourceLocation` class + `getSourceLocation` interner (so the FFI has a type to return).
- `SourceLocationHandle*` on every `metal_expr_*` and `metal_function_new` — the `.cpp` bodies
  take it as an **unnamed** parameter and never pass it to the instruction constructor. No
  `Expression` subclass stores it, so there is no base-ctor churn across the ~54 subclasses.
- `debug` field on `BackendCompileOptionsFFI`; `loadFromFfi` never reads it.
- The three dormant DI fields in `globalstate.h` and the two dead `<llvm-c/DebugInfo.h>`
  includes are left as-is.

## Built: M1 + M2 + M3 + Layer-1 emission

  Every DWARF-emission function lives in `Backend/src/function/debugging.{h,cpp}`; the rest of the
  backend just calls into it.

- **M1 — function-level DWARF (built).** `initDebugInfo` (called from `createModule` in vale.cpp)
  builds a `DIBuilder` on `GlobalState` + module flags (Dwarf 4 / Debug Info 3); `attachDISubprogram`
  emits one `DISubprogram` per user function with a lazy `DICompileUnit` and a per-source-path
  `DIFile` cache; `finalizeDebugInfo` (called from `finalizeCompile`) calls `LLVMDIBuilderFinalize`
  before codegen.
- **Real source paths / `DW_AT_comp_dir` (built).** The frontend conveys a `basename → absolute path`
  map (`BackendInputs.source_paths` → FFI → `GlobalState::sourcePaths`); `getOrCreateDIFile` resolves
  the basename on each `SourceLocation` against it, so each `DIFile` gets its real absolute directory
  (was `comp_dir "."` + basename — lldb couldn't find source, breaking `list` and `br s -p`).
  `getOrCreateCompileUnit` anchors the CU to the lexically-first **user** source file (not the
  first-compiled function, which could be a builtin — that left `comp_dir "."` and varied with
  compile order). The frontend keeps `SourceLocation.filePath` a basename; only DWARF carries the
  absolute path. The function's location comes from its **body's range**
  (`lower_function`), since `FunctionHeaderI` has no range yet. Backtraces resolve `file.vale:N`.
  **Requires `dsymutil <exe>` post-link on macOS** — DWARF lives in `.o` files and needs a sibling
  `.dSYM`; the test harness runs it.
- **M2 — per-statement stepping (built).** `translateExpressionInner` sets a `DILocation` per
  expression from its `SourceLocation` (guarded on non-null). Keystone: `translateExpression`
  wraps the recursion in a `ScopedDebugLoc` that **saves/restores** the current debug location, or
  parent instructions inherit the last child's line and stepping collapses to one line per subtree.
- **M3 — local variable inspection (built).** `makeHammerLocal` (shared.cpp) — the single point where
  every local's alloca is created — calls `emitLocalVariableDebugInfo` to emit a `DILocalVariable` +
  `llvm.dbg.declare` for that local, reading the declaration location off the metal `Local` itself (`Local::sourceLocation`, set by
  `metal_cache_get_local`, threaded from the lowerer's `lower_local`/`lower_locals`). Emitting at the
  chokepoint (not per call site) covers *every* local — including the two destructure paths a
  per-arm emission missed. Typed by `getOrCreateDIType` — Int/Bool/Float precise, else opaque `"ref"`.
  Function parameters come free (Vale lowers `func foo(a int)` to a `Stackify` into a named local).
  `frame variable x` shows a primitive's value; `[x, y] = …` destructure locals too.
- **Layer 1 — struct composite (built).** `getOrCreateDIStructType` builds a flattened
  `DICompositeType` from a struct's **inner** LLVM layout (`%<name>`, no control block — that's what
  an inline struct local's alloca holds), user members at their `LLVMOffsetOfElement` offsets, with a
  `LLVMDIBuilderCreateReplaceableCompositeType` forward-decl + RAUW for self-reference.
  `makeHammerLocal` picks the DIType by the local's storage `translateType`: an **inline** value uses
  this composite directly, and a **pointer** value uses `getOrCreateDIPointerType`.
- **Pointer-to-struct — borrow refs (built).** `getOrCreateDIPointerType` returns a
  `DW_TAG_pointer_type` to the inner-struct composite when the local is a `BorrowRef` to a
  `StructKind` (in this region a borrow lowers to a pointer *straight to the inner struct* — no
  control block in between), else the opaque `"ref"`. So `frame variable -P 1 ref` walks a borrow-ref
  struct local's fields. The debugger models the user-visible value and **ignores control blocks**, so
  no wrapper flattening is involved.
- **Layer 1.5 — static-sized arrays (built).** `getOrCreateDIArrayType` builds a `DW_TAG_array_type`
  from an SSA's inner LLVM type (`[N x elem]`, a bare array — what an SSA local's alloca holds): the
  element DIType via `getOrCreateDIType(def->elementType)`, the count N (`def->size`) on a
  `DW_TAG_subrange_type`, total size in bits from the `[N x elem]` type. So `frame variable -P 1 a`
  walks an SSA array local's elements. No control block, no wrapper — an SSA local is a bare array.
  Runtime-sized arrays are deferred (see below).

## Deferred: composite-type Layers

The remaining emission. All core (`Backend/`, needs `fire core edits`). The debugger **ignores
control blocks** — each Layer describes the user-visible value (the elements, the inner struct),
never the control-block header — so no wrapper-flattening or `controlblock.h`/`IRegion` plumbing is
planned.

- **Layers 1.6 / 1.7 — interfaces / strings.** Per-kind DIType helpers off the element/inner layout,
  so `frame variable -P N` walks their fields, following the array/borrow-ref pattern
  (`getOrCreateDIArrayType` / `getOrCreateDIPointerType`): point at the user data, past any control
  block. The RAUW forward-decl pattern is already in `getOrCreateDIStructType`.
- **Runtime-sized arrays (RSA) — deferred by standing order.** Every RSA e2e test is `#[ignore]`d and
  one fixture is missing, so RSA isn't reachable or testable here. Its local is a pointer to a
  `{ controlBlock, i32 size, [i32 capacity]?, [0 x elem] }` wrapper; the count is the runtime `size`
  member (not DWARF), which the C-API subrange (constant count only) can't express. Revisit when RSA
  is un-deferred.
- **Not planned: control-block cases.** A weak ref (`&&`) or a boxed owned ref does interpose a
  control block between the pointer and the data; these currently render as the opaque `"ref"` and
  stay that way. Weak refs are `#[ignore]`d on this branch anyway; revisit only if the debugger ever
  needs to surface control-block internals (it deliberately doesn't today).

Gotchas to preserve when building emission (from the original arc):

- `ptrSizeBits` is in **bits** (`LLVMPointerSize << 3`), not bytes — doubling it makes lldb
  silently drop composite members.
- A **bool in a struct must report 8-bit** DI storage, not 1-bit, or lldb drops the sub-byte
  member. Matches Rust/Swift.
- `b :main`, not `b main` — Vale prefixes `:` to avoid the C-runtime `main`.
- Touch `build.rs` after a backend rebuild or the static archive won't relink into the test
  binary.

## File map (current state)

Frontend / FFI (Rust):
- `src/utils/source_code_utils.rs` — `line_col_in`, `resolve_line_col`.
- `src/typing/ast/expressions.rs` — TE `range` + `range()`.
- `src/instantiating/ast/expressions.rs` — IE `range` + `range()`.
- `src/instantiating/instantiator.rs` — TE→IE range population.
- `src/backend_ffi/metal_lowerer.rs` — `loc_of`, per-node loc resolution; `lower_local`/`lower_locals`
  thread the declaring statement's loc onto each metal `Local`.
- `src/backend_ffi/metal_cache.rs` — `SourceLocation`, `get_source_location`, `loc` on builders and on
  `get_local`.
- `src/backend_ffi/mod.rs`, `src/bin/valec/{build,midas}.rs` — `--debug` option.
- `src/pass_manager/pass_manager.rs` — passes `vale_code_map` to the lowerer.

- `src/end_to_end_tests/mod.rs` — `compile_inline_debug`, `lldb_check`, `lldb_check_ordered`,
  `lldb_capture`, `dwarfdump_capture`, dsymutil; plus `assert_compile_and_run_dbg` /
  `assert_inline_compile_and_run_dbg`, which drive a Native-only scoped lldb session (a `&[Step]`
  script of `cmd`/`expect`/`reject`, each checked against its own command's output via
  `split_lldb_session`) on top of a normal behavior test's exit-code check.
- `src/end_to_end_tests/tests/debugger.rs` — 26 gates (Native only): M1 backtrace/line, M2 stepping
  (ordered), a structural DWARF gate, a decl-line boundary gate, a backtrace-depth gate, an array
  line-stepping gate, 3 M3 `frame variable` gates, 3 Layer-1 struct gates, a destructure gate, 2
  pointer-to-struct gates (a borrow-ref struct local walks its fields; a `dwarfdump`
  pointer-to-structure DIE gate), 2 Layer-1.5 SSA-array gates (a static-sized-array local walks
  its elements; a `dwarfdump` array-type/subrange DIE gate), and a sentinel-breakpoint probe.
- Breadth: 25 supported live e2e behavior tests (`misc`/`structs`/`ifelse`/`arrays`/`inline`/
  `while_loop`) call the `_dbg` helpers, each asserting that program's own debugger behavior — a
  mutated local's before/after value, struct-field/array-element walks, a loop counter across
  iterations, the taken branch, a two-frame backtrace (5 are stepping/backtrace-only for lack of
  inspectable state). Breakpoints anchor on labeled `0; // lldb breakpoint: <label>` sentinel lines
  via `br s -p` (line-robust), fixtures compiled from a temp dir (test-review #9).
  Every e2e test whose kind the debugger can't yet walk carries a `// VDBG:` marker; the full
  gap list lives in `docs/handoffs/debugger-handoff.md`.

Backend (C++):
- `Backend/src/metal/instructions.h` — `SourceLocation` class + a required `SourceLocation*` ctor
  param on the `Expression` base, threaded through every subclass ctor (no default, no setter).
- `Backend/src/metal/ast.h` — `SourceLocation*` ctor param on `Function` and `Local`.
- `Backend/src/metal/metalcache.h` — `getSourceLocation` interner.
- `Backend/src/metal/metal_cache_ffi.{h,cpp}` — the FFI builders pass the loc as the first ctor arg
  (`new X(srcloc(source_loc), …)`); `metal_function_new`/`metal_cache_get_local` do the same;
  interner FFI.
- `Backend/src/globalstate.h` — `diFileCache`, `diTypeCache`; `dibuilder`/`compileUnit`.
- **`Backend/src/function/debugging.{h,cpp}` — the single home for all DWARF emission:**
  `initDebugInfo`/`finalizeDebugInfo` (DIBuilder lifecycle + module flags), `attachDISubprogram`
  (+ `getOrCreateDIFile`/`CompileUnit`), `getOrCreateDIType` / `getOrCreateDIStructType` /
  `getOrCreateDIArrayType` / `getOrCreateDIPointerType` / `makeOpaqueRefDIType`,
  `emitLocalVariableDebugInfo`, and the `ScopedDebugLoc` RAII (in the header).
- `Backend/src/vale.cpp` — calls `initDebugInfo` (createModule) and `finalizeDebugInfo`
  (finalizeCompile).
- `Backend/src/function/function.cpp` — `declareFunction` calls `attachDISubprogram`.
- `Backend/src/function/expression.cpp` — the per-expression `DILocation` block, guarded by
  `ScopedDebugLoc` (M2).
- `Backend/src/function/expressions/shared/shared.cpp` — `makeHammerLocal` calls
  `emitLocalVariableDebugInfo` for every local (M3 + Layer-1 inline + pointer-to-struct + SSA array).
- `Backend/src/valeopts.{h,cpp}`, `Backend/src/backend_options_ffi.h` — `debug` option.

## Where to start when building the remaining Layers

Inline structs walk (`getOrCreateDIStructType`), borrow-ref struct locals walk via a
pointer-to-composite (`getOrCreateDIPointerType`), and static-sized arrays walk via a
`DW_TAG_array_type` (`getOrCreateDIArrayType`). Next, each behind its own `debugger.rs` gate
(`frame variable -P N`, or a `dwarfdump_capture` DIE-shape assertion — the helper is in place):

1. **Interfaces / strings** — per-kind DIType builders off the element/inner layout, pointing at the
   user data past any control block (the debugger ignores control blocks). Follow the array /
   borrow-ref pattern.
2. **Runtime-sized arrays** — deferred by standing order (all RSA e2e tests `#[ignore]`d); revisit
   when un-deferred.
