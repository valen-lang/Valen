# Handoff: Continuing the God-Struct Refactor

## Who this is for

You're a junior engineer picking up an in-progress refactor partway through. A colleague started this and merged the first 11 sub-compilers onto `Compiler`. Your job is to keep going, one sub-compiler at a time, until the list is done. Then hand off to whoever does macros and the Step 8 cleanup.

**Read `handoff-god-struct-refactor.md` first.** That's the master plan. This doc is the "continuation" — status, patterns I converged on, gotchas I hit, and exactly what to do next. The master plan describes the "what" and "why"; this doc tells you what's *done* and the practical "how" you'll need to execute cleanly.

## 90-second catch-up

- **Project:** Rust port of a Scala compiler frontend. The typing pass (`FrontendRust/src/typing/`) is being collapsed from ~20 separate `FooCompiler` structs into one `Compiler<'s, 'ctx, 't>` god struct.
- **Method bodies are almost all `panic!()` stubs.** This refactor is structural: move method signatures from `impl FooCompiler` blocks into `impl Compiler` blocks. You're almost never reading real logic.
- **The audit trail matters.** Every Rust definition sits next to its `/* scala */` comment block. The pairing is enforced by a pre-commit hook. Don't break it.
- **The god struct has four fields**, nothing else:
  ```rust
  pub struct Compiler<'s, 'ctx, 't>
  where 's: 't,
  {
      pub scout_arena: &'ctx ScoutArena<'s>,
      pub typing_interner: &'ctx TypingInterner<'t>,
      pub keywords: &'ctx Keywords<'s>,
      pub opts: &'ctx TypingPassOptions<'s>,
  }
  ```
  `global_env` and `name_translator` are **not** on it — see the master handoff for why.

## Status as of last commit

On branch `rustmigrate-z`. `cargo check --lib` is **clean** (0 errors, 0 warnings — `src/lib.rs` sets `#![allow(unused_variables, unused_imports)]`). Phase 2 (god-struct refactor) is complete: all 8 upper-tier sub-compilers merged, all 15 macros merged, and Step 8 cleanup done (vestigial `ArrayCompiler`, `InferCompiler`, `TemplataCompiler`, `ImplCompiler`, `StructCompiler`, `DestructorCompiler`, `ExpressionCompiler`, `NameTranslator`, `ConvertHelper`, `OverloadResolver`, `CompilerSolver` PhantomData structs deleted). Only `Compiler` itself remains as a real type; the other `*Compiler` names are now just `// mig: struct` markers with Scala comments beneath. The `IInfererDelegate` trait is still kept vestigial because fn signatures in `compiler_solver.rs` reference it as `&dyn IInfererDelegate<'s, 't>`; that goes away when those fn signatures get rewritten in a later phase. Next phase is body migration — filling in the typing-pass skeleton per `quest.md` §12 (Slabs 1–6).

**Known cleanup debt:** the `// mig: struct` / `// mig: enum` / `// mig: fn` / `// mig: impl` / `// mig: trait` markers are slice-pipeline artifacts and can be deleted entirely. Slicing is done; the `/* scala */` blocks remain as the audit trail (and are what the pre-commit hook enforces) — the `// mig:` lines add nothing. The Slab 2 handoff (`handoff-slab-2.md`) schedules this cleanup as a Step 0 prereq: sweep `src/typing/` and delete every `// mig:` line. Expected scope: ~500+ sites across ~90 files.

### Done

| Tier | Sub-compiler | Pattern |
|---|---|---|
| Prep | `TypingInterner<'t>` | Created (`src/typing/typing_interner.rs`) |
| Prep | `Compiler<'s, 'ctx, 't>` | Filled in with four fields + `new` |
| Leaf | `VirtualCompiler` | Struct deleted (no methods to move — Scala methods were all commented out) |
| Leaf | `LocalHelper` | Struct deleted; 10 instance methods wrapped, 2 statics kept as free fns |
| Leaf | `NameTranslator` | **Struct kept vestigial** (9 sub-compilers hold `name_translator: NameTranslator<'s>` fields); 9 methods wrapped |
| Leaf | `ConvertHelper` | Struct → PhantomData vestigial; `IConvertHelperDelegate` trait deleted; 3 methods wrapped |
| Mid | `DestructorCompiler` | Struct kept vestigial; 2 stub methods wrapped |
| Mid | `SequenceCompiler` | Struct deleted; 3 methods wrapped |
| Mid | `OverloadResolver` | Struct kept vestigial; 11 stub methods wrapped |
| Mid | `InferCompiler` | Struct kept vestigial; `IInferCompilerDelegate` trait deleted; 15 methods wrapped + 2 statics |
| Mid | `InferCompiler` (companion `compiler_solver.rs`) | `CompilerSolver` struct kept vestigial; `IInfererDelegate` trait **kept vestigial** (fn params still reference it); 5 class methods wrapped + 9 statics |
| Upper | `PatternCompiler` | Struct deleted; 12 stub methods wrapped |
| Upper | `CallCompiler` | Struct deleted; 4 stub methods wrapped |
| Upper | `BlockCompiler` | Struct deleted; `IBlockCompilerDelegate` trait deleted; 2 stub methods wrapped |
| Upper | `ExpressionCompiler` | Struct kept vestigial (held by `as_subtype_macro`/`lock_weak_macro`); `IExpressionCompilerDelegate` trait deleted; 21 stub methods wrapped |
| Upper | `TemplataCompiler` | Struct kept vestigial (held by 8 sub-compilers); `ITemplataCompilerDelegate` + `IPlaceholderSubstituter` traits deleted along with 8 trait-abstract-method stubs; 14 instance methods wrapped; 35 `object TemplataCompiler` statics left as free fns |
| Upper | `EdgeCompiler` | Struct deleted entirely (no external holders); 4 instance methods had existing `impl EdgeCompiler` wrappers re-targeted to `impl Compiler` |
| Upper | `ImplCompiler` | Struct kept vestigial (held by `as_subtype_macro`); 9 stub methods wrapped |
| Upper | `StructCompiler` + `StructCompilerCore` + `StructCompilerGenericArgsLayer` | `StructCompiler` struct kept vestigial (held by 3 function-compiler layers); `StructCompilerCore` and `StructCompilerGenericArgsLayer` structs deleted (no external holders); `IStructCompilerDelegate` trait + 2 abstract method stubs deleted; 9+9+6=24 methods wrapped; collisions resolved by `_layer` (GenericArgsLayer) and `_core` (Core) suffixes |
| Upper | `ArrayCompiler` | Struct kept vestigial (held by `rsa_mutable_new_macro`/`rsa_drop_into_macro`); 13 instance methods wrapped (11 pub, 2 private helpers) |
| Upper | `BodyCompiler` | Struct deleted entirely (no external holders); `IBodyCompilerDelegate` trait deleted; 3 stub methods wrapped; had to move `ResultTypeMismatchError` data struct out of the impl block (Rust disallows nested structs in impls) |
| Upper | `FunctionCompiler` + `FunctionCompilerCore` + `FunctionCompilerMiddleLayer` + `FunctionCompilerSolvingLayer` + `FunctionCompilerClosureOrLightLayer` | Shipped across 5 separate commits rather than one (the bundle is 3200 lines). All 5 structs deleted; `IFunctionCompilerDelegate` trait deleted; 8+8+10+10+11=47 methods wrapped; collisions resolved with `_core` / `_solving` / `_closure_or_light` suffixes. Wrapping closure_or_light_layer's fns fixed the 14 baseline "self outside impl" errors — error count dropped 30→19 |

### Remaining (in order)

Upper-tier: **none, all done.**

Next phase: macros (~20 files in `src/typing/macros/**` and subdirs). Then Step 8 cleanup (see master handoff).

## The macros pattern — how to do one macro

Macros are different from sub-compilers in one way: they're dispatched dynamically (stored in the global env, looked up by generator id). But the shape is the same — all state has already been hoisted onto `Compiler`, so the macro struct has nothing real left in it. We collapse them onto `Compiler` the same way we did sub-compilers, plus wire a per-trait unit-variant enum as the dispatch tag.

### The four dispatch enums

Scala has four macro traits. Each becomes a unit-variant enum in Rust (`src/typing/compiler.rs` or a new `src/typing/macros/macros.rs` — same place the current `IFunctionGenerator` stub lives):

```rust
pub enum FunctionBodyMacro {
    LockWeak,
    AsSubtype,
    StructDrop,
    InterfaceDrop,
    StructConstructor,
    AbstractBody,
    SameInstance,
    RsaLen, RsaMutableNew, RsaImmutableNew, RsaDropInto,
    RsaMutableCapacity, RsaMutablePop, RsaMutablePush,
    SsaLen, SsaDropInto,
    // (only macros that extend IFunctionBodyMacro in Scala — roughly the list above)
}
pub enum OnStructDefinedMacro { StructConstructor, StructDrop, SameInstance, /* ... */ }
pub enum OnInterfaceDefinedMacro { /* ... */ }
pub enum OnImplDefinedMacro { /* ... */ }
```

A given macro (e.g. `StructDropMacro`) appears in multiple enums iff Scala had it extending multiple traits. These are Copy unit enums — cheap, no lifetimes.

### Per-macro per-method transformation

For each macro file `src/typing/macros/.../foo_macro.rs`:

**Before (the slice-pipeline output we inherited):**

```rust
// mig: struct FooMacro
pub struct FooMacro<'s, 'ctx, 't> {
    pub keywords: Keywords<'s>,
    pub expression_compiler: ExpressionCompiler<'s, 'ctx, 't>,    // or other sub-compilers
    pub destructor_compiler: DestructorCompiler<'s, 'ctx, 't>,
}
// mig: impl FooMacro
impl<'s, 'ctx, 't> FooMacro<'s, 'ctx, 't> {}
/*
class FooMacro(keywords: Keywords, expressionCompiler: ExpressionCompiler, ...) extends IFunctionBodyMacro {
*/
// mig: fn generate_function_body
fn generate_function_body<'s, 't>(
    env: &FunctionEnvironmentT<'s, 't>,
    coutputs: &CompilerOutputs<'s, 't>,
    /* ... */
) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
    panic!("Unimplemented: generate_function_body");
}
/* scala body */
```

**After:**

```rust
// mig: struct FooMacro
// (Scala class fields absorbed by Compiler; all logic lives on impl Compiler.)
/*
class FooMacro(keywords: Keywords, expressionCompiler: ExpressionCompiler, ...) extends IFunctionBodyMacro {
*/
// mig: fn generate_function_body
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_foo(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        /* ... same params as before, minus the sub-compiler fields ... */
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        panic!("Unimplemented: generate_function_body_foo");
    }
/* scala body */
}
```

**What changed:**

1. **Struct deleted entirely.** The `pub struct FooMacro<'s, 'ctx, 't> { ... }` goes away — nothing Rust-level holds it (the sub-compiler fields it held are all already merged). The `// mig: struct FooMacro` marker stays so the audit trail is preserved; add a `//` note explaining the fields absorbed into `Compiler`.
2. **Empty `impl FooMacro<'s, 'ctx, 't> {}` deleted.**
3. **Every Scala instance method (`generateFunctionBody`, `onStructDefined`, etc.) wraps into its own per-fn `impl Compiler` block**, exactly like sub-compiler methods. The method name gets a suffix derived from the macro name to avoid collisions (every `FunctionBodyMacro` has a `generateFunctionBody` in Scala).
4. **Name suffix convention:** strip the `Macro` suffix from the macro name and lower-snake. `LockWeakMacro.generateFunctionBody` → `generate_function_body_lock_weak`. `RSADropIntoMacro.generateFunctionBody` → `generate_function_body_rsa_drop_into`. `StructConstructorMacro.onStructDefined` → `on_struct_defined_struct_constructor`. Keep it verbose — readability beats brevity for audit-trail grep.
5. **Companion object helpers stay as free fns** (same rule as sub-compilers — see Gotcha 4).

### Dispatcher methods on `Compiler`

One dispatcher per Scala trait, wired in `src/typing/compiler.rs` or alongside the enums. These exist solely to turn a `FunctionBodyMacro` tag into the correct `generate_function_body_*` call:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't {
    pub fn dispatch_function_body_macro(
        &self,
        which: FunctionBodyMacro,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        /* shared params */
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        match which {
            FunctionBodyMacro::LockWeak => self.generate_function_body_lock_weak(coutputs, env, /* ... */),
            FunctionBodyMacro::AsSubtype => self.generate_function_body_as_subtype(coutputs, env, /* ... */),
            // ...
        }
    }
}
```

Since no one calls the dispatcher yet (env lookup is still panic-stubbed), you can leave the match with `todo!()` arms for macros not yet migrated. Fill in arms as you merge each macro.

### Order of macro merges

Start with the simplest (unit-struct / one-method) macros first:

1. `lock_weak_macro.rs` — two fields, one method. Good first example.
2. `as_subtype_macro.rs` — four fields, one method.
3. `same_instance_macro.rs`, `functor_helper.rs` — small helpers.
4. RSA/SSA macros (10 files) — mostly copy-paste of the same shape.
5. `struct_drop_macro.rs`, `interface_drop_macro.rs` — extends both `IFunctionBodyMacro` and `IOnStructDefinedMacro` / `IOnInterfaceDefinedMacro`; two methods to merge.
6. `struct_constructor_macro.rs` — multiple methods + helper fns.
7. `abstract_body_macro.rs`, `anonymous_interface_macro.rs` — the complex ones, leave for last.

Commit one macro per commit. Error count should stay at 0 throughout (we're clean right now).

### Unblocking Step 8

Two macros currently hold `ExpressionCompiler` as a field — `as_subtype_macro.rs` and `lock_weak_macro.rs`. Once both are merged (struct deleted → field gone), `ExpressionCompiler` is no longer held anywhere and can be deleted in Step 8. Same chain for other vestigial `*Compiler` structs (ArrayCompiler held by RSA macros, ImplCompiler held by AsSubtypeMacro, etc.).

## The pattern — how to do one sub-compiler

For each file `src/typing/.../foo_compiler.rs`:

### Step A: Understand the shape

1. Read the file.
2. Identify:
   - The main `pub struct FooCompiler<'s, 'ctx, 't>` (usually PhantomData or a few fields).
   - The empty `impl FooCompiler<'s, 'ctx, 't> {}` right below it.
   - Any `pub trait IFooCompilerDelegate { ... }` — a delegate trait.
   - Methods in the file. They come in two shapes:
     - **Instance methods**: fns inside a `class FooCompiler { ... }` Scala block. In Rust they may already be wrapped in per-method `impl FooCompiler<'s, 'ctx, 't> { fn xxx(...) {} }` blocks, OR they may be free fns at module level (the slice pipeline is inconsistent).
     - **Companion statics**: fns inside an `object FooCompiler { ... }` or `object FooCompilerHelper { ... }` Scala block. In Rust these are always free fns and **stay as free fns** — don't wrap them on `Compiler`.
   - Any supporting types (`case class Foo`, `sealed trait Bar`, enums). **Leave these alone.** They're not the compiler struct; they're associated data.

### Step B: Check call sites

Before editing, grep for the sub-compiler's name across the whole codebase:

```
grep -rn "FooCompiler\|foo_compiler" FrontendRust/src/
```

What you want to know:
- **Rust-level field references** like `pub foo_compiler: FooCompiler<'s, 'ctx, 't>` inside another sub-compiler's struct definition. Those are real Rust and mean the type can't be deleted yet.
- **Rust call sites** like `self.foo_compiler.do_thing(...)`. These are rare at this stage because bodies are panic — but if you find any, you'll need to rewrite them as `self.do_thing(...)` after the merge.
- **Scala-comment references** like `val fooCompiler = new FooCompiler(...)` inside a `/* */` block. These are not Rust and you ignore them.

### Step C: Pick deletion strategy

Based on what you find:

- **If no other Rust code references `FooCompiler<'s, 'ctx, 't>` as a type** (the common case for pure leaf types): **delete the struct entirely.** The markers and Scala block stay.
- **If other sub-compilers still hold `pub foo_compiler: FooCompiler<'s, 'ctx, 't>` fields**: **keep the struct as a `PhantomData` placeholder with a `// vestigial` comment.** It gets deleted in Step 8 cleanup once all holders are gone.
- **If a delegate trait `IFooCompilerDelegate` exists and no Rust code references it outside the file**: delete it, leaving the marker and Scala block.
- **If the delegate trait is referenced by fn parameter types in the file** (like `IInfererDelegate` in `compiler_solver.rs`): **keep it vestigial** — don't delete the trait body until the fn params referencing it are rewritten (which is separate work, not part of this refactor).

### Step D: Make the edits

A typical sub-compiler merge touches three regions of the file:

**1. Add the import.** Near the existing `use` statements add:

```rust
use crate::typing::compiler::Compiler;
```

**2. Reshape the struct/impl region.** Before:

```rust
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: struct FooCompiler
pub struct FooCompiler<'s, 'ctx, 't>(pub std::marker::PhantomData<(&'s (), &'ctx (), &'t ())>);
// mig: impl FooCompiler
impl<'s, 'ctx, 't> FooCompiler<'s, 'ctx, 't> {}
/*
class FooCompiler(...) {
*/
```

After, if deleting entirely:

```rust
// mig: struct FooCompiler
// mig: impl FooCompiler
/*
class FooCompiler(...) {
*/
```

After, if keeping vestigial:

```rust
// mig: struct FooCompiler
// vestigial: kept until Step 8 cleanup because sub-compilers still hold `foo_compiler: FooCompiler<'s, 'ctx, 't>` fields
pub struct FooCompiler<'s, 'ctx, 't>(pub std::marker::PhantomData<(&'s (), &'ctx (), &'t ())>);
// mig: impl FooCompiler
/*
class FooCompiler(...) {
*/
```

Note we always delete the empty `impl FooCompiler {}` line — it has nothing in it.

**3. Wrap each instance method.** This is the bulk of the work.

Before:

```rust
// mig: fn do_thing
fn do_thing<'s, 't>(&self, x: Foo<'s, 't>) -> Bar<'s, 't> {
    panic!("Unimplemented: do_thing");
}
/*
  def doThing(x: Foo): Bar = {
    ... scala body ...
  }

*/
```

After:

```rust
// mig: fn do_thing
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn do_thing(&self, x: Foo<'s, 't>) -> Bar<'s, 't> {
        panic!("Unimplemented: do_thing");
    }
/*
  def doThing(x: Foo): Bar = {
    ... scala body ...
  }

*/
}
```

The signature changes:
- Remove fn-level `<'s, 't>` generics (they come from the impl block now).
- Add `pub` (Scala defaults to public, and callers across files will need access).
- Ensure `&self` is there as the first parameter. If the original was a free fn without `&self`, add it.

The structural changes:
- Wrap in `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't, { ... }`.
- Indent the fn by 4 spaces (inside the impl).
- **The Scala `/* ... */` block stays *inside* the impl braces, directly after the fn's closing `}`**, with nothing between them (no closing impl brace).
- The outer `}` that closes the impl comes *after* the Scala block.

**4. Close the last fn's impl.** After the last method's Scala block, add a final closing `}`:

```rust
  }
}
*/
}           <-- this is new
```

The triple-closing-braces region always looks weird. The sequence is: `}` (Scala class close) + `*/` (Scala comment close) + `}` (your impl close).

### Step E: Fix impl-boundary gaps (you will miss these)

After wrapping each method, the file looks like this:

```rust
// mig: fn method_a
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn method_a(&self, ...) { panic!(...); }
/*
  scala for method_a
*/

// mig: fn method_b       <-- PROBLEM: method_a's impl is still open!
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn method_b(&self, ...) { panic!(...); }
...
```

You need a `}` between the `*/` of method_a and the `// mig: fn method_b`. I got bitten by this a lot. The fix:

```rust
/*
  scala for method_a
*/
}                            <-- add this

// mig: fn method_b
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    ...
```

When doing edits via the Edit tool, I found the easiest approach is: include the boundary in the `old_string` of the *next* method's edit, like this:

```
// For method_b, use this as old_string:
scala end of method_a
*/
// mig: fn method_b
fn method_b(...) ...

// And new_string:
scala end of method_a
*/
}

// mig: fn method_b
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn method_b(...) ...
```

So each method's edit closes the *previous* impl and opens the current one. Only the very last method needs a trailing `}` added separately.

### Step F: Verify

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml 2>&1 | tee /tmp/god-struct-refactor.txt
grep -c "^error" /tmp/god-struct-refactor.txt
```

Compare against the previous commit's count. **It should be non-increasing.** If it went up:
- Check for missing `}` at impl boundaries (Step E).
- Check for unused-import warnings in the file you edited — those aren't errors, ignore them.
- Check if you accidentally broke someone else's file (very unusual — the refactor is file-local).

Spot check that call sites are gone:

```bash
grep -rn "self\.foo_compiler\." FrontendRust/src/typing/
```

Should be **zero matches** after the merge (because no one was calling `self.foo_compiler.xxx` in Rust anyway — those calls were all in Scala comments).

### Step G: Commit

One commit per sub-compiler, with an exception for `StructCompiler` and `FunctionCompiler` — merge those with their layer files in one commit each. Sample commit message:

```
God-struct refactor: merge ExpressionCompiler onto Compiler.

Wrap N methods from src/typing/expression/expression_compiler.rs in
per-fn impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> blocks. The struct
<was deleted / is kept vestigial because ...>. <Any trait deletions.>

Error count: X -> Y.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
```

## Gotchas

### 1. The pre-commit hook blocks `/* */` edits

There's a hook at `.claude/hooks/check-scala-comments` that compares every `/* */` block in Rust files against the original Scala source. It blocks the commit if they drift. This means:

- **Don't put English prose in a `/* */` block.** If you want a "no scala counterpart — this is Rust scaffolding" note, use a `//` line comment above the impl instead. The master handoff has an example.
- **Don't reformat Scala content.** If a `/* */` has weird indentation or trailing whitespace, leave it. The hook does exact-match comparison (with a few filtering exceptions, see `docs/usage/check-scala-comments-hook.md`).
- **If you split a `/* */` block in two**, both halves must still parse as the original content concatenated. In practice, don't split them.

The hook fires on Edit/Write before the change lands. If you get a rejection, read the diff in the error message and fix it.

### 2. "self outside impl" baseline errors are OK

`cargo check` has ~30 pre-existing errors of the form:

```
error: `self` parameter is only allowed in associated functions
   --> src/typing/.../some_file.rs:LINE:5
    |
LINE |     &self,
```

These are the slice pipeline leaving `&self` on free fns that weren't inside any `impl` block. Each sub-compiler merge that includes one of these fns will *fix* one such error (by wrapping the fn in `impl Compiler`). So your error count may *drop* by 1–5 per merge, not just hold steady.

Do not worry about these errors as "caused by your change" — they existed before.

### 3. Indent mismatch doesn't matter

The pipeline-generated fns are at 2-space or 0-space indent. Wrapping them in `impl Compiler { ... }` means they should technically be at 4-space indent (inside the impl). I kept 4-space indent for the fn signature and 8-space for body when rewrapping. Rust doesn't care about whitespace; don't stress about getting every fn perfectly indented. Keep the convention consistent *within a single merge* if possible, but don't redo previous merges to match.

### 4. Companion object methods stay as free fns

Scala distinguishes:

```scala
class FooCompiler(...) {
  def instanceMethod(...) = { ... }       // becomes a Compiler method
}

object FooCompiler {
  def staticMethod(...) = { ... }         // stays a free fn
}
```

Both translate to module-level free fns in the slice-pipeline output, but they have different semantic destinations. To tell them apart, look at the Scala comment — everything between `class FooCompiler(...) {` and the `}` that closes it is an instance method; everything between `object FooCompiler {` and its closing `}` is a static.

### 5. Delegate trait with param-type references = vestigial

When I deleted `IInfererDelegate` in `compiler_solver.rs`, the build broke because ~10 fn signatures took `delegate: IInfererDelegate<'s, 't>` as a parameter. I reverted the deletion and marked it vestigial. Pattern:

- **If the delegate trait is only referenced inside the file** (typically just in a struct field that's going away): delete it.
- **If fn parameter types reference the trait**: mark it vestigial, delete in Step 8 cleanup.

### 6. Method name collisions (watch for later)

Several sub-compilers have methods named `compile`, `resolve`, etc. When merging multiple sub-compilers, two of them might have `fn compile(...)` with the same signature — that's a compile error.

**You haven't hit this yet** in the early sub-compilers because stubs have different arg lists (mostly `()`). When you get to the big ones (StructCompiler, FunctionCompiler), rename for clarity:

- `StructCompiler.compile` → `compile_struct`
- `FunctionCompiler.compile` → `compile_function`
- `StructCompiler.resolve` → `resolve_struct`
- etc.

Match the convention in the master handoff (Step 7).

### 7. "One fn per impl block" convention

Do not batch multiple fns into a single `impl Compiler { ... }` block. Every single fn gets its own impl wrapper so the Scala comment can sit adjacent to it per the audit-trail convention. This makes files look redundant (10 copies of `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't, { ... }`), and that's intentional.

### 8. Don't touch other files

A sub-compiler merge should only touch the sub-compiler's file (and maybe the handoff doc). If a merge needs changes in 3+ files, stop and ask — you're probably doing something beyond the refactor scope.

## Useful commands

```bash
# Baseline error count
cargo check --lib --manifest-path FrontendRust/Cargo.toml 2>&1 | tee /tmp/god-struct-refactor.txt
grep -c "^error" /tmp/god-struct-refactor.txt

# Find errors in a specific file
grep "path/to/your_file.rs" /tmp/god-struct-refactor.txt | grep -v warning

# Check there are no new Compiler method call sites (should be zero)
grep -rn "self\.foo_compiler\." FrontendRust/src/

# Find external holders of FooCompiler<'s, 'ctx, 't> (tells you if struct can be deleted)
grep -rn "pub foo_compiler:\|FooCompiler<'s" FrontendRust/src/

# Find all sub-compiler structs (catalog of what's left)
grep -rn "^pub struct \w*Compiler\b" FrontendRust/src/typing/
```

## Where to file questions

- **Design** (is this the right approach?): Ask the senior. Don't invent. The master handoff is pretty detailed; most questions are answered there.
- **Scala semantics** (what does this Scala method actually do?): Check `Frontend/TypingPass/src/dev/vale/typing/FooCompiler.scala`. You usually don't need to understand it; bodies are panic!().
- **Hook rejections** (what's blocking my commit?): Read the error. `docs/usage/check-scala-comments-hook.md` has background.
- **Lifetime errors** when adding `&self`: If `where 's: 't,` on the impl isn't enough, check the master handoff's "If you get stuck" section.

## Done criterion for each merge

- `cargo check` error count non-increasing (with the understanding that stubs fixing pre-existing `self outside impl` errors will *decrease* count).
- No `cannot find type FooCompiler` errors (because the type is either kept-vestigial or all call sites are gone).
- No `self.foo_compiler.` call sites in Rust code.
- Struct deleted (or kept vestigial with `// vestigial` comment).
- Delegate trait deleted (or kept vestigial, documented).
- Every method is in its own one-fn `impl Compiler` block with Scala comment inside braces.
- Commit pushed with a message describing what struct/trait treatment you applied.

## Final advice

This refactor is boring and mechanical. That's a feature — it lets you do it quickly and correctly. The hardest part is the impl-boundary gaps (Gotcha 1) and not getting confused by the slice-pipeline's inconsistent style (some methods already in `impl FooCompiler` blocks, some free, some 2-indent, some 0-indent). Take one sub-compiler at a time, commit after each, and the whole thing moves forward at a predictable pace.

Good luck. Ping the senior if anything in this doc is unclear or missing.
