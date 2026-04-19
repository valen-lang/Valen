# Handoff: Collapse Typing-Pass Sub-Compilers Into A Single God-Struct

**⚠️ HISTORICAL — Phase 2 complete.** This doc describes the god-struct refactor that shipped. One lifetime example inside (line ~219, `env: &'s IEnvironmentT<'s, 't>` on `IFunctionGenerator::generate`) used the then-current assumption that envs lived in `'s`. That was corrected in Slab 4 planning — envs are now `'t`-allocated; the example signature would use `&'t` today. See `TL-HANDOFF.md` overrides, `quest.md` §3.1, and `docs/reasoning/environments-per-denizen-long-term.md`.

## Who this is for

You're a junior engineer picking up a medium-scope structural refactor in a Scala→Rust compiler-migration project. This doc gives you the full picture: the goal, the current state, the target state, the theory behind the design, and the step-by-step approach. Read the whole thing before touching code, then keep it open while you work.

## 90-second project context

- **Sylvan** is the overall repo. `Frontend/` is a Scala compiler (the original, authoritative source). `FrontendRust/` is a Rust reimplementation being migrated piece-by-piece from Scala.
- The migration uses a **"slice pipeline"** that inserts `// mig:` marker comments above Scala definitions and generates Rust placeholder stubs. The Scala source stays in the file as a commented-out `/* ... */` block above each corresponding Rust stub, so a reviewer can always see the original. **Never touch the `/* ... */` Scala blocks or `// mig:` markers.** A pre-commit hook enforces this.
- We are focused on the **typing pass** (`FrontendRust/src/typing/`). The typing pass does type inference and type checking.
- There is a design doc at `/Volumes/V/Sylvan/quest.md` describing the three-arena lifetime model and god-struct architecture for this pass. **Read the "Status" section at the top, §1 (Arena and Lifetime Model), §2 (The God Struct), and §2.3 (Macros) before starting.** This doc assumes you've read those.
- Most Rust stub bodies are `panic!()` right now. The typing pass's real logic hasn't been ported yet. This refactor is **purely about restructuring the type/fn layout**, not about implementing any logic.

## Your task in one sentence

**Collapse the ~20 separate sub-compiler structs (`StructCompiler`, `ArrayCompiler`, `TemplataCompiler`, `FunctionCompiler`, etc., plus `NameTranslator` and helpers like `ConvertHelper`/`LocalHelper`) into a single `Compiler<'s, 'ctx, 't>` god struct, moving all their methods onto it and deleting the delegate traits and layer-split wrappers.**

## Why this is being done

Scala's typing pass is organized as a bunch of separate classes — `StructCompiler`, `FunctionCompiler`, `ArrayCompiler`, etc. — each holding references to the others as fields (`StructCompiler(opts, interner, keywords, templataCompiler, inferCompiler, ...)`). They wire up at runtime via mutable constructor closures. That pattern worked in Scala because of GC + flexible mutation, and because Scala's `class X(f: Foo => Bar)` lets you thread arbitrary closures.

In Rust that pattern is extremely painful:

- **Circular references** — `StructCompiler` holds a `TemplataCompiler`, `TemplataCompiler` holds a `StructCompiler`. Can't represent cleanly in Rust without `Rc<RefCell<...>>` or `Weak`, which we're avoiding per the arena-allocation design.
- **Mutation vs. shared borrow** — the slice-pipeline-generated field lists (`pub delegate: Box<dyn IStructCompilerDelegate<'s, 't>>`) are type-system overhead that never actually gets used in stub bodies.
- **Delegate-trait plumbing** — there are ~10 `IXxxCompilerDelegate` traits solely to let sub-compilers call into each other. They add nothing structural.
- **Layer splits are vestigial** — `FunctionCompilerMiddleLayer`, `FunctionCompilerSolvingLayer`, `FunctionCompilerClosureOrLightLayer`, `StructCompilerCore`, `StructCompilerGenericArgsLayer` exist because Scala's `FunctionCompiler.scala` was getting too big for one file. In Rust we can organize by `impl Compiler { ... }` blocks across multiple files without the struct split.

The target architecture, per `quest.md` §2.1–2.4:

```rust
pub struct Compiler<'s, 'ctx, 't> {
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub typing_interner: &'ctx TypingInterner<'t>,
    pub keywords: &'ctx Keywords<'s>,
    pub opts: &'ctx TypingPassOptions<'s>,
}
```

Four fields, all shared-borrow context. Deliberately **not** on the god struct:

- **`global_env`** — Scala builds `globalEnv` as a local inside `Compiler.compile()`, not as a constructor arg. Rust does the same: pass it through as a method parameter on `compile_program` and anything downstream that needs it.
- **`name_translator`** — Scala's `NameTranslator` is a stateless helper (its methods just translate postparser names through the interner). In Rust every `Compiler` method already has `self.scout_arena` and `self.typing_interner`, so the helper adds nothing. Its translate methods move directly onto `impl Compiler` and the struct is deleted.

Every typing-pass method is an `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { fn foo(&self, coutputs: &mut CompilerOutputs<'s, 't>, ...) -> ... }`. Mutual recursion works because `&self` is shared-borrow and `&mut coutputs` is re-borrowed at each call level (quest.md §2.2).

**Why this is worth doing now** — most method bodies are still `panic!()`, so the refactor is almost purely signature manipulation. If we wait until Slab 1+ fills in real logic, every method body ends up routing through the wrong struct and will need to be rewritten twice. Now is the cheapest time.

## Background: arenas and interning (essential reading)

Before you touch anything, understand the arena model. The current Rust codebase has a **placeholder `Interner<'s>` type** that is pure vestigial baggage — you will delete it at the end of this refactor (Step 8), once no sub-compiler still references it. Here's why it exists and what replaces it.

### Scala has `Interner`; Rust has three arenas

In Scala, every pass shared one `Interner` GC'd HashMap that dedup'd case-class instances. Typical constructor:

```scala
class FunctionCompiler(opts: TypingPassOptions, interner: Interner, keywords: Keywords, ...)
```

In Rust, the `Interner` GC pattern is replaced by **three scoped arenas** — see `quest.md` §1.1:

| Arena | Lifetime | What it holds |
|---|---|---|
| `ParseArena<'p>` | `'p` | Parser AST, `StrI<'p>`, parse-time coords |
| `ScoutArena<'s>` | `'s` | Post-parser / higher-typing output (`FunctionA<'s>`, `StructA<'s>`, interned names `IRuneS<'s>` / `INameS<'s>` / `IImpreciseNameS<'s>`), **and typing-pass environments** |
| `TypingInterner<'t>` | `'t` | Interned typing-pass types (`INameT`, `IdT`, `KindT`, `ITemplataT`, `PrototypeT`, `SignatureT`, `OverloadSetT`) and typing output AST (`FunctionDefinitionT`, expressions, `HinputsT`) |

Each arena has its own set of **intern methods** that accept transient "Val" keys, check for existence, and either return the existing canonical reference or promote the Val into arena storage (see `.claude/rules/postparser/IDEPFL-postparser-interning.md` for the full interning discipline).

Representative methods (may not all exist yet — some are future work):
- `ScoutArena::intern_str(&self, s: &str) -> StrI<'s>`
- `ScoutArena::intern_rune(&self, val: IRuneValS<'s, 'tmp>) -> IRuneS<'s>`
- `ScoutArena::intern_name(&self, val: INameValS<'s>) -> INameS<'s>`
- `ScoutArena::intern_imprecise_name(...)`
- `TypingInterner::intern_kind(&self, val: KindValT<'s, 't>) -> &'t KindT<'s, 't>`
- `TypingInterner::intern_name(&self, val: INameValT<'s, 't>) -> &'t INameT<'s, 't>`
- `TypingInterner::intern_templata(&self, val: ITemplataValT<'s, 't>) -> &'t ITemplataT<'s, 't>`

These mostly don't exist in the Rust code yet — they'll be built as part of Slab 0 per `quest.md` §12. For this refactor, you just need to know they're coming and plumb `&'ctx ScoutArena<'s>` + `&'ctx TypingInterner<'t>` into method signatures where they're needed.

### What the typing pass needs access to

- When a typing-pass method produces a scout-interned value (e.g. it builds a new `IFunctionDeclarationNameS<'s>` from a user-defined function name), it reaches into `self.scout_arena`.
- When it produces a typing-pass-interned value (a new `IdT<'s, 't>`, `KindT<'s, 't>`, `ITemplataT<'s, 't>`), it reaches into `self.typing_interner`.
- Most methods need both, which is why the god struct holds both as fields.

### Arena-parameter convention (critical invariant)

Per `quest.md` §1.2 invariant 5: **arena references use a short borrow lifetime (`'ctx`), not the arena's own lifetime**:

```rust
// CORRECT
pub struct Compiler<'s, 'ctx, 't> {
    pub scout_arena: &'ctx ScoutArena<'s>,        // short borrow, long-lived arena
    pub typing_interner: &'ctx TypingInterner<'t>, // same
    ...
}

// WRONG (don't do this)
pub scout_arena: &'s ScoutArena<'s>,
```

The decoupling works because `ScoutArena::alloc(&self, val: T) -> &'s mut T` returns `'s`-lifetimed data from a short `&self` borrow. Callers don't need to hold the arena for all of `'s` just to allocate into it. This same pattern is already followed in `ParseArena` and `ScoutArena` usage in the parser, postparser, and higher-typing passes — you don't need to invent it, just mirror it.

### Standalone functions

Not every function is a `Compiler` method. The god struct covers the main pipeline, but some code is genuinely stateless:

- Scala `object Foo { def bar(...) }` companions — translate to Rust `pub fn bar(...)` at module level.
- Visitor/collector helpers that walk AST and don't need state.
- Generic utilities like `get_compound_type_mutability`.

For those, **pass `&'ctx ScoutArena<'s>` and `&'ctx TypingInterner<'t>` in as parameters individually**:

```rust
pub fn translate_generic_function_name<'s, 'ctx, 't>(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'t>,
    function_name: IFunctionDeclarationNameS<'s>,
) -> &'t IFunctionTemplateNameT<'s, 't> { panic!() }
```

**Don't bundle them into a context struct.** We discussed this and decided to keep the two refs as individual params. It makes signatures longer but keeps the type of each param explicit and readable.

### The `Interner` placeholder

Right now, `FrontendRust/src/interner.rs` contains this one line:

```rust
pub struct Interner<'s>(pub std::marker::PhantomData<&'s ()>);
```

It was added to satisfy ~70 stale imports across typing-pass files. It has no methods, no fields, and no behavior. **You will delete it in the Step 8 cleanup commit, after every sub-compiler that references it is gone.** Don't delete it earlier — sub-compilers hold `pub interner: &'ctx Interner<'s>` fields, and removing the type before the sub-compilers would break the build for the entire duration of the refactor. When each sub-compiler is merged onto `Compiler`, its method bodies reach through `self.scout_arena` or `self.typing_interner` (both real fields on the god struct) instead of the `interner` field, which disappears with the sub-compiler struct.

### Keywords and TypingPassOptions

`Keywords<'s>` is a real type in `FrontendRust/src/keywords.rs` — a cache of pre-interned `StrI<'s>` values for keywords like `self`, `int`, `drop`, etc. Scout-lifetimed, because it's interned into the scout arena and survives through instantiation. It stays as a god-struct field per `quest.md` §9.

`TypingPassOptions<'s>` is configuration (global options, debug output callback, tree-shaking flag). Also a god-struct field.

## Current state — inventory

### Sub-compiler structs to merge (~20)

These all currently exist as `pub struct FooCompiler<'s, 'ctx, 't> { ... }` (some as PhantomData stubs) with fields like `opts, interner, keywords, *_compiler`. All their methods get moved onto `Compiler<'s, 'ctx, 't>`. The structs themselves get deleted.

**Leaves (no references to other compilers as fields — easiest first):**
- `VirtualCompiler` — `src/typing/function/virtual_compiler.rs`. Small.
- `LocalHelper` — `src/typing/expression/local_helper.rs`. Small.
- `NameTranslator` — `src/typing/names/name_translator.rs`. Currently `pub struct NameTranslator<'s>(PhantomData)`. ~6 translate methods; delete the struct entirely, move methods onto `impl Compiler`.
- `ConvertHelper` — `src/typing/convert_helper.rs`. Small. Exercises delegate-trait deletion (`IConvertHelperDelegate`).

**Middle tier (reference only leaf compilers):**
- `DestructorCompiler` — `src/typing/function/destructor_compiler.rs`.
- `SequenceCompiler` — `src/typing/sequence_compiler.rs`.
- `OverloadResolver` — `src/typing/overload_resolver.rs`.
- `InferCompiler` — `src/typing/infer_compiler.rs` (plus the `compiler_solver.rs` module inside it which contains the `advance_infer`/`continue`/`solve`/`sanity_check_conclusion` free fns).

**Upper tier (reference multiple mid-tier compilers):**
- `PatternCompiler` — `src/typing/expression/pattern_compiler.rs`.
- `CallCompiler` — `src/typing/expression/call_compiler.rs`.
- `BlockCompiler` — `src/typing/expression/block_compiler.rs`.
- `ExpressionCompiler` — `src/typing/expression/expression_compiler.rs`.
- `TemplataCompiler` — `src/typing/templata_compiler.rs`.
- `EdgeCompiler` — `src/typing/edge_compiler.rs`.
- `ImplCompiler` — `src/typing/citizen/impl_compiler.rs`.
- `StructCompiler` — `src/typing/citizen/struct_compiler.rs`. Big.
- `ArrayCompiler` — `src/typing/array_compiler.rs`. Big.
- `BodyCompiler` — `src/typing/function/function_body_compiler.rs`.
- `FunctionCompiler` — `src/typing/function/function_compiler.rs`. Biggest.

**Vestigial "layer" splits to eliminate entirely:**
- `FunctionCompilerMiddleLayer` — `src/typing/function/function_compiler_middle_layer.rs`
- `FunctionCompilerSolvingLayer` — `src/typing/function/function_compiler_solving_layer.rs`
- `FunctionCompilerClosureOrLightLayer` — `src/typing/function/function_compiler_closure_or_light_layer.rs`
- `FunctionCompilerCore` — `src/typing/function/function_compiler_core.rs`
- `StructCompilerCore` — `src/typing/citizen/struct_compiler_core.rs`
- `StructCompilerGenericArgsLayer` — `src/typing/citizen/struct_compiler_generic_args_layer.rs`

These are artifacts of Scala's splitting one big file into multiple "layer" files. In Rust they offer nothing — merge all their methods into `impl Compiler` too. The files stay (for the migration audit trail), but the struct definitions are deleted and their methods get the `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` wrapper.

### Delegate traits to delete (~10)

These all exist solely to let sub-compilers call into each other without holding direct references. Per `quest.md` §2.4 they go away entirely:

- `IExpressionCompilerDelegate` — `src/typing/expression/expression_compiler.rs:69`
- `IFunctionCompilerDelegate` — `src/typing/function/function_compiler.rs:55`
- `IInfererDelegate` — `src/typing/infer/compiler_solver.rs:102`
- `IStructCompilerDelegate` — `src/typing/citizen/struct_compiler.rs:107`
- `IConvertHelperDelegate` — `src/typing/convert_helper.rs:48`
- `IBlockCompilerDelegate` — `src/typing/expression/block_compiler.rs:44`
- `IBodyCompilerDelegate` — `src/typing/function/function_body_compiler.rs:46`
- `IFunctionGenerator` (trait, not to be confused with the `enum IFunctionGenerator` in quest.md §2.3) — `src/typing/compiler.rs:67`
- Anywhere else you find `pub trait I*Delegate<'s, 't> {}` or `pub trait I*Generator<'s, 't> {}` that looks like a dispatch placeholder.

Wherever these traits are declared, check for:
1. `Box<dyn IFooDelegate<'s, 't>>` or `&'ctx dyn IFooDelegate<'s, 't>` fields on sub-compiler structs — delete them (the sub-compiler struct is going away anyway, but if you're migrating incrementally, these fields need to go in the first step of merging that sub-compiler).
2. Call sites like `self.delegate.do_thing(...)` — these become `self.do_thing(...)` after the method is merged onto `Compiler`.

### Macros (~20) — keep the structs, shed the fields

Per `quest.md` §2.3, macros are a different case:

```rust
pub enum IFunctionGenerator<'s, 't> {
    AsSubtype(AsSubtypeMacro<'s, 't>),
    LockWeak(LockWeakMacro<'s, 't>),
    StructDrop(StructDropMacro<'s, 't>),
    // ... ~20 variants
}

impl<'s, 't> IFunctionGenerator<'s, 't> {
    pub fn generate(
        &self,
        compiler: &Compiler<'s, '_, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'s IEnvironmentT<'s, 't>,
        // ...
    ) -> &'t FunctionHeaderT<'s, 't> { ... }
}
```

So the macro **structs** (`AsSubtypeMacro`, `LockWeakMacro`, `StructDropMacro`, `StructConstructorMacro`, `AbstractBodyMacro`, `AnonymousInterfaceMacro`, `SameInstanceMacro`, `FunctorHelper`, the `RSA*`/`SSA*` macros, `InterfaceDropMacro`) **stay** — they're stored in the global env as variants of the `IFunctionGenerator` enum. But they **shed their fields**: the current `opts, interner, keywords, destructor_compiler, expression_compiler, array_compiler, ...` fields all disappear. `generate()` takes `&Compiler<'s, '_, 't>` + `&mut CompilerOutputs` as parameters and reaches through the god struct for everything it used to access via its own fields.

**Expected end state of a macro struct:**

```rust
pub struct StructDropMacro<'s, 't> {
    pub _phantom: PhantomData<(&'s (), &'t ())>,  // or no fields at all if nothing is needed
}
```

Macros are ~20 files in `src/typing/macros/` and its subdirs (`citizen/`, `rsa/`, `ssa/`). Each needs the same treatment: strip fields, update `generate()` signature.

## What "done" looks like

After this refactor:

1. `FrontendRust/src/interner.rs` no longer has `pub struct Interner<'s>(...)`. (The `StrI<'s>` type stays — that's legit, a real arena-backed string wrapper.)
2. No file in `src/typing/` defines a `pub struct FooCompiler<...>` with compiler fields on it. Exception: the god struct `Compiler<'s, 'ctx, 't>` exists in one place (probably `src/typing/compiler.rs`).
3. Every method that was on a sub-compiler is now in an `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { fn ... }` block, preserved directly above its original `// mig:` marker and `/* ... */` Scala comment. **No code moves relative to its Scala comment.** The `impl Compiler` wrapper replaces the former `impl FooCompiler` wrapper in place.
4. No `IXxxCompilerDelegate` trait exists in `src/typing/`.
5. Macro structs (`AsSubtypeMacro`, etc.) have no fields (or just `_phantom: PhantomData`). Their `generate()` takes `&Compiler<'s, '_, 't>` + `&mut CompilerOutputs<'s, 't>`.
6. `cargo check --lib --manifest-path FrontendRust/Cargo.toml` produces **no E0425 "cannot find type `FooCompiler`"** errors (those types are gone — callers now reach through `self` on `Compiler`).
7. Total error count doesn't go up (much). Error delta should be dominated by:
   - **Cascading new-types-missing errors** if you rename a sub-compiler without updating all call sites. These should go to 0 if you're thorough.
   - **`ScoutArena`/`TypingInterner` "not yet built" errors** if you try to use intern methods that don't exist yet. Stub those with `panic!()`-bodied placeholder methods on the arena types if needed.

## Key design rules (must-preserve invariants)

These come from `quest.md` §11 and the project's `CLAUDE.md`. Violating them will block the refactor or break other work.

1. **`'s` outlives `'t`.** Every type that transitively holds `&'s` data needs `where 's: 't`. Already declared on existing types; keep it when you redeclare.
2. **No `&mut self` on `Compiler` or any of its methods.** Mutable state is `CompilerOutputs`. If you find a method that needs `&mut self`, something's wrong — figure out what state is supposed to be on `CompilerOutputs` instead.
3. **Arena parameters use short borrow lifetime (`'ctx`), not the arena's own.** `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`.
4. **Every Rust definition stays directly above its `/* scala */` comment.** No reordering. No movement. The `// mig:` markers pair Rust stubs to Scala definitions; re-arranging them breaks the audit trail.
5. **No `Vec`/`HashMap`/`String`/`Box`/`Rc` inside arena-allocated types** (AASSNCMCX). Not directly relevant to this refactor — you're not adding new arena-allocated types — but don't accidentally violate it when writing placeholders.
6. **Inherent `impl` blocks contain exactly one fn, and the `/* scala */` block lives inside the impl braces, immediately after the fn's closing brace.** This is our established convention — the Scala comment must directly follow its corresponding Rust fn with nothing between them, not even a closing impl brace:
   ```rust
   // mig: fn resolve_struct
   impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
       pub fn resolve_struct(&self, coutputs: &mut CompilerOutputs<'s, 't>, ...) { panic!() }
   /*
     def resolveStruct(...) = { ... }
   */
   }
   ```
   Don't lump multiple methods into one big `impl Compiler { ... }`, and don't move the Scala comment outside the impl braces.
7. **Never edit `/* ... */` Scala comment blocks.** Pre-commit hook will block you.
8. **No inline `/* ... */` block comments in Rust.** Use `//` line comments only. Same hook.
9. **Always pipe `cargo check`/`build`/`test` output into a fixed file in `/tmp`** and never chain `| grep`/`| tail` onto the command. See CLAUDE.md. Example:
   ```
   cargo check --lib --manifest-path FrontendRust/Cargo.toml 2>&1 | tee /tmp/god-struct-refactor.txt
   ```
   Then separately:
   ```
   grep -c "^error" /tmp/god-struct-refactor.txt
   ```

## Approach — leaf-first, one sub-compiler at a time

**Do not do this all at once.** You will deadlock on cross-file type errors. Follow the order below. After each sub-compiler, commit, verify the build state, continue.

### Step 0: Preparation (one-time, single prep commit)

The goal of this step is to get `Compiler` and `TypingInterner` into a shape where every sub-compiler merge can be done without breaking the build. **Leave `Interner<'s>` in place for now** — it's the vestigial placeholder currently held as `pub interner: &'ctx Interner<'s>` on every sub-compiler. If you delete it on day 1, every sub-compiler stops compiling until its merge commit lands, which can be weeks. Delete it in the final cleanup commit instead, once all sub-compilers that referenced it are gone.

1. **Create `TypingInterner<'t>`** (e.g. in `src/typing/typing_interner.rs`). Real struct, not just PhantomData. Stub the six intern methods with `panic!()` bodies so Compiler methods can reference them during merges:
   ```rust
   pub struct TypingInterner<'t> { /* real fields as needed, or PhantomData */ }

   impl<'t> TypingInterner<'t> {
       pub fn intern_name<'s>(&self, val: INameValT<'s, 't>) -> &'t INameT<'s, 't> { panic!("intern_name not yet implemented") }
       pub fn intern_kind<'s>(&self, val: KindValT<'s, 't>) -> &'t KindT<'s, 't> { panic!("intern_kind not yet implemented") }
       pub fn intern_id<'s, T: Copy>(&self, val: IdValT<'s, 't, T>) -> &'t IdT<'s, 't, T> { panic!("intern_id not yet implemented") }
       pub fn intern_templata<'s>(&self, val: ITemplataValT<'s, 't>) -> &'t ITemplataT<'s, 't> { panic!("intern_templata not yet implemented") }
       pub fn intern_prototype<'s>(&self, val: PrototypeValT<'s, 't>) -> &'t PrototypeT<'s, 't> { panic!("intern_prototype not yet implemented") }
       pub fn intern_signature<'s>(&self, val: SignatureValT<'s, 't>) -> &'t SignatureT<'s, 't> { panic!("intern_signature not yet implemented") }
   }
   ```
   The `*ValT` enums don't exist yet (they're Slab 2–3 work). If that makes the stubs unbuildable, either define the `*ValT` enums as empty placeholder enums (`pub enum INameValT<'s, 't> {}`) or weaken the stubs to take `()` — either is fine.

2. **Fill in the god struct** in `src/typing/compiler.rs`. Replace the current PhantomData stub with four fields per quest.md §2.1:
   ```rust
   pub struct Compiler<'s, 'ctx, 't> {
       pub scout_arena: &'ctx ScoutArena<'s>,
       pub typing_interner: &'ctx TypingInterner<'t>,
       pub keywords: &'ctx Keywords<'s>,
       pub opts: &'ctx TypingPassOptions<'s>,
   }
   ```
   `ScoutArena<'s>` already exists at `src/scout_arena.rs`. `Keywords<'s>` at `src/keywords.rs`. `TypingPassOptions<'s>` at `src/typing/compilation.rs`. All four are real — no placeholders needed.

3. **Do NOT delete `Interner<'s>` yet.** Sub-compilers still reference it. Deletion happens in Step 8 cleanup (alongside `NameTranslator<'s>`, which is kept for the same reason during Step 3).

4. Get a baseline:
   ```
   cargo check --lib --manifest-path FrontendRust/Cargo.toml 2>&1 | tee /tmp/god-struct-refactor.txt
   grep -c "^error" /tmp/god-struct-refactor.txt
   ```
   Write the number down. Build should be **green or close to green** after this commit (we're only adding things). Each subsequent sub-compiler merge commit should keep the error count non-increasing.

### Step 1: Merge leaf sub-compiler `VirtualCompiler`

Pick `VirtualCompiler` first. Why: it's small, a pure leaf (holds no references to other compilers), and has no delegate trait. It's the cleanest demonstration of the pattern — the struct disappears entirely and its methods move onto `impl Compiler`. Use it as your template for the other leaves.

Workflow:

**a. Read the file.** `src/typing/function/virtual_compiler.rs`. Note: the struct definition, each method, and its `// mig:`/`/* scala */` pairs.

**b. Delete the struct.** Remove the `pub struct VirtualCompiler<'s, 'ctx, 't> { ... }` block. Preserve the `// mig: struct VirtualCompiler` marker and the `/* ... */` Scala block above it.

**c. Rewrap each method.** For each `fn xxx(...)` in the file:
   - Take it out of the `impl VirtualCompiler<'s, 'ctx, 't>` block and put it into a per-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { ... }` block in place.
   - The first parameter was already `&self` — semantics change (now a method on `Compiler`, not `VirtualCompiler`), but syntax is identical.
   - Body stays `panic!()` — we're not implementing logic here.
   - Keep the `// mig:` marker directly above the `impl Compiler { ... }` line.
   - **Keep the `/* scala */` block immediately below the fn's closing `}`, inside the impl braces.** Nothing between the fn and the Scala comment, not even a closing impl brace. The impl's own closing `}` goes after the Scala comment. See the example in Key Design Rules §6.

**d. Delete the `impl VirtualCompiler` wrapper.** Once all methods are rewrapped, the outer `impl VirtualCompiler<'s, 'ctx, 't> { ... }` block is empty — remove it.

**e. Update call sites.** Find every call:
   ```
   grep -rn "virtual_compiler\." FrontendRust/src/typing/
   grep -rn "VirtualCompiler\." FrontendRust/src/typing/
   ```
   Change `self.virtual_compiler.foo(...)` to `self.foo(...)` everywhere. Same for any `self.delegate` field on callers that used to hold a `VirtualCompiler` reference (drop the field, the call becomes direct).

**f. Verify.** Run cargo check. Error count should go down. `cannot find type VirtualCompiler` errors should be 0.

**g. Commit.** Commit message template:
   ```
   Merge VirtualCompiler methods onto Compiler god struct.

   Move <list methods> from impl VirtualCompiler onto
   impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>. Each method keeps its // mig:
   marker and /* scala */ block intact; wrapper impl block is per-fn per
   project convention. Delete the struct and the surrounding impl block.

   Update ~N call sites that used self.virtual_compiler.foo() to self.foo().

   Error count: <before> -> <after>.
   ```

### Step 2: Merge leaf `LocalHelper`

Same workflow, `src/typing/expression/local_helper.rs`. Small. Deletes the struct, moves methods onto `Compiler`, updates call sites.

### Step 3: Merge leaf `NameTranslator`

Same workflow, `src/typing/names/name_translator.rs`. ~9 `translate_*` methods. The methods move onto `impl Compiler`; the struct adds nothing in Rust once its methods are on `Compiler`.

**But keep `pub struct NameTranslator<'s>(PhantomData<&'s ()>)` in place for now** — same reasoning as `Interner<'s>` in Step 0. ~9 sub-compilers and macros hold `pub name_translator: NameTranslator<'s>` fields; deleting the type here would break their build until each is merged. The struct is a vestigial placeholder until Step 8 cleanup, which deletes both `Interner<'s>` and `NameTranslator<'s>` once every sub-compiler that referenced them is gone.

Update call sites: `self.name_translator.translate_struct_name(foo)` → `self.translate_struct_name(foo)`. (Anywhere that actually calls a translate method in Rust today — most call sites are inside Scala `/* */` blocks and don't need touching.)

### Step 4: Merge leaf `ConvertHelper`

Same workflow. Note: `ConvertHelper` has a `delegate: Box<dyn IConvertHelperDelegate<'s, 't>>` field currently. Drop the field, delete the `IConvertHelperDelegate` trait, and rewrite any `self.delegate.foo(...)` call sites as direct `self.foo(...)`.

### Step 5: Work upward through the tiers

Order (mid-tier):
- `DestructorCompiler`
- `SequenceCompiler`
- `OverloadResolver`
- `InferCompiler` (and its `compiler_solver.rs` free fns `advance_infer`, `continue`, `solve`, `sanity_check_conclusion`, `complex_solve`, `solve_receives`, `narrow`, `solve_rule`, `solve_call_rule`, `literal_to_templata`)

Order (upper-tier):
- `PatternCompiler`
- `CallCompiler`
- `BlockCompiler`
- `ExpressionCompiler`
- `TemplataCompiler`
- `EdgeCompiler`
- `ImplCompiler`
- `StructCompiler`
- `ArrayCompiler`
- `BodyCompiler`
- `FunctionCompiler` (do this last — it's the largest, and other sub-compilers reference it)

Eliminate the `*Layer` splits as you go, **in one big commit per owning sub-compiler**:
- When you reach `FunctionCompiler`, all of `FunctionCompilerMiddleLayer`, `FunctionCompilerSolvingLayer`, `FunctionCompilerClosureOrLightLayer`, `FunctionCompilerCore` get merged in the **same** commit. Their methods all go onto `impl Compiler`.
- When you reach `StructCompiler`, merge `StructCompilerCore` and `StructCompilerGenericArgsLayer` in one commit with `StructCompiler`.

Rationale for one-big-move on layer splits: the layers aren't independently meaningful in Rust; splitting them across commits just creates intermediate states where half the methods of a conceptual unit live on `Compiler` and half don't. Merge the whole unit at once.

### Step 6: Handle macros

Per `quest.md` §2.3. For each macro struct in `src/typing/macros/**`:

1. Strip its fields (`opts`, `interner`, `keywords`, and any `*_compiler: FooCompiler` refs). Replace with `_phantom: PhantomData<(&'s (), &'t ())>` or empty struct.
2. Rewrite `generate_function_body` (and any other methods) to take `&Compiler<'s, '_, 't>` as a param, and access things through it (`compiler.scout_arena`, `compiler.typing_interner`, `compiler.opts`, `compiler.keywords`, etc.).
3. Update the `IFunctionGenerator` enum (if it exists yet — probably not, may need to define it per quest.md §2.3) in `src/typing/compiler.rs` to have a variant per macro.

You can do this after the sub-compiler merge is complete, or interleaved — whichever is less confusing. I'd lean "after", because during the sub-compiler merge you probably don't have stable arena methods yet.

### Step 7: Method name collisions

While merging, you'll hit collisions. Scala had methods on separate classes with the same name. Rules:

- Prefix with the original sub-compiler name, snake_cased: `StructCompiler.compile` → `compile_struct`, `InterfaceCompiler.compile` → `compile_interface`, `StructCompiler.resolve` → `resolve_struct`.
- When the original method name is already unambiguous (e.g. `evaluate_templated_function_from_call_for_prototype`), leave it alone.
- When two sub-compilers both have a same-named method and the meaning is "compile the thing this sub-compiler is about", prefix with what the thing is: `compile_struct`, `compile_function`, `compile_interface`, `compile_impl`, `compile_static_sized_array`, etc.

Keep the `// mig:` marker matching whatever name you picked.

### Step 8: Final cleanup

After all sub-compilers and macros are merged:

1. **Delete `pub struct Interner<'s>(...)` from `src/interner.rs`.** By this point, every sub-compiler that referenced it is gone, so there should be no more `&'ctx Interner<'s>` fields anywhere. Leave `pub struct StrI<'s>` — that's real. Leave `InternedSlice<'a, T>` — also real.
2. **Delete `pub struct NameTranslator<'s>(...)` from `src/typing/names/name_translator.rs`.** Same reasoning as `Interner<'s>` — the struct was kept as a vestigial placeholder during Step 3 to avoid breaking sub-compilers that held `pub name_translator: NameTranslator<'s>` fields. Once every sub-compiler is merged, no such fields remain, and the struct can be removed. Leave the `// mig: struct NameTranslator` marker and the Scala `class NameTranslator(...)` block.
3. Remove the dead `use crate::interner::Interner;` imports across the typing pass.
4. Remove the dead `use crate::typing::*compiler::*` imports that referenced the deleted sub-compilers.
5. Audit the `src/typing/mod.rs` — are there `pub mod foo_compiler;` entries for files that now only exist as `impl Compiler` wrappers? Those `mod` entries still need to exist (the files are still there), but may no longer be needed as `pub mod` if they don't export anything externally.
6. Delete the delegate traits (`IExpressionCompilerDelegate`, etc.) — by this point they should have zero uses.
7. Search for any `Box<dyn I*Delegate>` or `&dyn I*Delegate` that are leftover and fix.
8. One final `cargo check`. Note the error count. Commit as a cleanup pass.

## Gotchas & watch-outs

### Slice-pipeline quirks you'll hit

- **Same-named `pub mod` and `pub struct`.** In this session we found `pub mod StructCompiler` (a translation of Scala's `object StructCompiler` companion) colliding with `pub struct StructCompiler`. Renamed to `pub mod struct_compiler_module`. Look for this pattern on other sub-compilers — Scala's `object Foo` companions may have been transliterated as `pub mod Foo` where `pub struct Foo` also exists. Rename the module, don't rename the struct.
- **Free fns with `&self` but not inside any impl block.** The slice pipeline sometimes writes `fn foo(&self, ...) { panic!() }` at module level. Invalid Rust. Wrap in an `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { ... }` block when you migrate — that's what this refactor fixes structurally anyway.
- **`T: KindT` / `T: IFunctionNameT` trait bounds on enums.** You'll find generic params like `<T: KindT<'s, 't>>` where `KindT` is an enum, not a trait. Drop the bound (change to `<T>`). This was cleared out in a prior session, but new ones may appear during the merge if you touch generic method signatures.
- **Bare `RegionT<'s, 't>` vs `RegionT`.** `RegionT` is currently a zero-field struct with no lifetime params, even though it's Scala-parity would suggest otherwise. Don't add `<'s, 't>` to it unless you also make it generic. If a stub has `RegionT<'s, 't>` in a signature, that's wrong — change to bare `RegionT`.

### Movement invariant

**No Rust code moves relative to its `/* scala */` comment.** Ever. Even when merging compilers, each method stays in the file it was in, directly above its Scala comment. You're only:
- Rewriting the surrounding `impl FooCompiler { ... }` block into `impl Compiler { ... }`.
- Renaming the method (if needed).
- Changing the first parameter from `&self` (on FooCompiler) to `&self` (on Compiler) — same syntax, different semantics.

If you find yourself moving a method from file A to file B, stop. That's not part of this refactor. Rust allows arbitrarily many `impl Compiler { ... }` blocks in arbitrarily many files. Leave things where they are.

### Interner reference cleanup

When you strip `pub interner: &'ctx Interner<'s>` from a sub-compiler, decide based on what the methods do:

- Methods that intern scout-lifetime values → replace with `scout_arena: &'ctx ScoutArena<'s>` (via `self.scout_arena` once on the god struct).
- Methods that intern typing-pass values → replace with `typing_interner: &'ctx TypingInterner<'t>`.
- Methods that don't intern anything → drop the field entirely.

**If you're not sure**, look at the Scala comment below the method. If it calls `interner.intern(SomeTypingName(...))`, it's a typing-pass interner. If it calls `interner.intern(SomeScoutName(...))` or builds a name from `StrI`/`RuneS`/etc., it's scout. If it just passes `interner` through to a helper, trace the helper and see what the helper interns.

When in doubt, include both on the god struct (which is the plan anyway) and route through `self.scout_arena` / `self.typing_interner` in the method body.

### Placeholder arena methods

You may need to stub arena methods that don't exist yet. Example: if a method body panics but its signature references `self.scout_arena.intern_name(...)`, rustc will complain the method doesn't exist. Fix: add a stub on `ScoutArena`:

```rust
impl<'s> ScoutArena<'s> {
    pub fn intern_name<'tmp>(&self, val: INameValS<'s, 'tmp>) -> INameS<'s> {
        panic!("intern_name not yet implemented")
    }
}
```

This is fine — it's consistent with how the rest of the typing pass is stubbed. Slab 0 will replace the panic with real logic.

### Don't run tasks in the background

Per project CLAUDE.md: "Please don't run tasks in the background." Run `cargo check` in the foreground. Don't use `run_in_background: true`. If you have a long-running build, just wait for it.

### Don't use sed

Per project CLAUDE.md "Bulk Sed Safety Protocol." Manual edits via the Edit tool only. Don't batch-rewrite with sed.

## How to verify at each step

After each sub-compiler merge:

1. `cargo check --lib --manifest-path FrontendRust/Cargo.toml 2>&1 | tee /tmp/god-struct-refactor.txt`
2. `grep -c "^error" /tmp/god-struct-refactor.txt` — should be non-increasing vs. previous commit (+/- small fluctuations as hidden errors get revealed).
3. `grep -c "cannot find type \`FooCompiler\`" /tmp/god-struct-refactor.txt` — should be 0 for the sub-compiler you just merged.
4. `grep "self.name_translator\." FrontendRust/src/typing/` — should be 0 hits after merging NameTranslator. Same pattern for each sub-compiler.
5. `git diff --stat` — should touch a bounded set of files per merge. If one merge touches > 20 files you're probably doing too much at once.

## If you get stuck

- **Rustc is angry about lifetime elision.** Usually means you added `&self` but the method body reaches into `self.scout_arena.something(...)` and rustc can't figure out the returned reference's lifetime. Try explicitly annotating: `fn foo(&self) -> &'t SomeT<'s, 't>` or `fn foo<'a>(&'a self) -> SomeT<'s, 't>` depending on what needs to outlive what.
- **"Method X is defined multiple times on Compiler."** Collision between two sub-compilers that had same-named methods. Rename one with a sub-compiler prefix (see Step 6 above).
- **"Cannot borrow `self` as mutable because it's also borrowed as immutable."** You're probably inside a method that takes `&self` on `Compiler` and trying to mutate something through `self`. Don't. Route the mutation through `&mut coutputs` or rethink the method — quest.md §2.2 relies on Compiler being immutable.
- **Some method has logic that actually calls into a sub-compiler's `&mut self` method.** Probably shouldn't happen (stubs are all `panic!()`), but if it does, look at the Scala. Scala's sub-compilers didn't mutate themselves either — they all mutated `coutputs` (the Scala equivalent of `CompilerOutputs`). Your method should also be `&self` + `&mut coutputs`.
- **You find a case the design doesn't cover.** Write down the case, skip it, and raise it as a question. Don't invent a solution — ask the senior first.

## Files & references

- `/Volumes/V/Sylvan/quest.md` — design doc. §1 arenas, §2 god struct, §3 envs, §11 invariants, §12 slab plan.
- `/Volumes/V/Sylvan/FrontendRust/docs/migration/handoff-typing-imports.md` — earlier handoff covering `use` statements; same style, same project conventions.
- `/Volumes/V/Sylvan/FrontendRust/src/typing/` — your working directory.
- `/Volumes/V/Sylvan/Frontend/TypingPass/src/dev/vale/typing/` — the Scala source, for reference. Useful to check "what did `interner.intern(...)` really do here?"
- `/Volumes/V/Sylvan/.claude/rules/postparser/IDEPFL-postparser-interning.md` — interning discipline, for understanding the ScoutArena pattern.
- `/Volumes/V/Sylvan/.claude/rules/postparser/postparser-migration.md` — general migration conventions (not typing-specific but informs style).
- `/Volumes/V/Sylvan/FrontendRust/docs/shields/ArenaTypesDontClone-ATDCX.md` — arena allocation rules (AASSNCMCX).
- `/Volumes/V/Sylvan/FrontendRust/docs/usage/check-scala-comments-hook.md` — describes the Scala-comment hook.
- Project `CLAUDE.md` files (global and project-level) — build-output convention, sed safety, background-task policy.

## Decisions already made (don't re-litigate)

1. **God struct has four fields.** `scout_arena`, `typing_interner`, `keywords`, `opts`. No `global_env`, no `name_translator` — see intro.
2. **`TypingInterner<'t>` is created in the Step 0 prep commit** as a real struct with six panic-bodied intern methods (`intern_name`, `intern_kind`, `intern_id`, `intern_templata`, `intern_prototype`, `intern_signature`). Not PhantomData — we want signatures that reference it to have something real to call.
3. **`Interner<'s>` stays in place until Step 8** so sub-compilers remain buildable during merges.
4. **Merge order:** leaves first (`VirtualCompiler` → `LocalHelper` → `NameTranslator` → `ConvertHelper`), then mid-tier, then upper-tier. `FunctionCompiler`+layers and `StructCompiler`+layers are one commit each.

## Questions for the senior before starting

1. **The macros' `IFunctionGenerator` enum** — does it exist yet? If not, should you define it during this refactor or defer to Slab 5?
2. **Method naming collisions** — when in doubt, prefix. But which prefix style is preferred? I suggested `compile_struct`, `resolve_struct`, but some codebases use `struct_compile`, `struct_resolve`. Mirror whatever convention you find elsewhere in `typing/`.

## Final advice

This is a medium refactor. Pacing:
- **Each sub-compiler takes ~1–3 hours** depending on size (leaves like `VirtualCompiler`/`LocalHelper`/`NameTranslator` fast; `FunctionCompiler` slow).
- **~20 sub-compilers + ~20 macros = ~4–6 focused sessions** to complete.
- **Commit per sub-compiler**, except for `FunctionCompiler`+layers and `StructCompiler`+layers which are one commit each. Don't accumulate unfinished merges across sessions.
- **Build after every commit.** Keep the error count visible and trending down.
- **Don't improvise.** The design in quest.md is detailed; follow it literally. If something is unclear, ask.

Good luck. This will make the codebase substantially easier to work with once it's done.
