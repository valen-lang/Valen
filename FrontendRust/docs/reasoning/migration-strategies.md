# Scala → Rust Migration Strategies

This document describes the general design thinking and strategies used when translating this compiler frontend from Scala to Rust. It captures the systematic rules that were applied across the entire codebase, the reasoning behind them, and concrete examples.

This is a reference for understanding *why* the Rust code looks the way it does and for guiding future migration work.

> **Partially stale — needs curation.** Part 2 (Memory Management) still describes the original **four-arena** model (`'a` Interner / `'p` / `'s` / `'ctx`) and references the deleted `early-lifetimes.mdc`. The current model is **per-pass arenas** (`'p` parser, `'s` scout; no long-lived `'a` Interner) — see `docs/background/arenas.md` for canonical lifetimes. Parts 5–8 and the appendix largely restate individual `Luz/shields/` rules; the durable, non-duplicated value here is the Part 1 (type-system) and Part 3 (code-organization) translation narrative.

---

## Part 1: Type System Translation

### 1.1 Sealed Trait Hierarchies → Enums (XSSTRE)

Every Scala `sealed trait` with `case class` / `case object` subtypes becomes a Rust `enum`. Virtual dispatch (abstract `def` on a trait) becomes `match`-based methods on the enum.

**Scala:**
```scala
sealed trait IExpressionSE {
  def range: RangeS
}
case class IfSE(range: RangeS, condition: IExpressionSE, ...) extends IExpressionSE
case class BlockSE(range: RangeS, locals: Vector[LocalS], ...) extends IExpressionSE
```

**Rust:**
```rust
pub enum IExpressionSE<'a, 's> {
  If(IfSE<'a, 's>),
  Block(BlockSE<'a, 's>),
  // ...
}

impl<'a, 's> IExpressionSE<'a, 's> {
  pub fn range(&self) -> RangeS<'a> {
    match self {
      IExpressionSE::If(x) => x.range,
      IExpressionSE::Block(x) => x.range,
      // ...
    }
  }
}
```

This applies universally: `IDenizenP`, `INameS`, `IRulexSR`, `ISolverOutcome`, `ICitizenAttributeS`, `IBodyS`, `IGenericParameterTypeS`, etc. — every sealed trait in the codebase.

### 1.2 Enums Hold Structs, Not Complex Inline Data (XESCCD)

Enum variants should contain structs with named fields, not inline complex data. This enables:
- Easier pattern matching in tests (can bind to the struct directly)
- `cast!`-style macros to extract inner types
- Interning the struct contents (enums themselves are never interned; only their contents are)

**Wrong:**
```rust
pub enum IRuneS<'a> {
  CodeRune(StrI<'a>, CodeLocationS<'a>),  // inline fields
}
```

**Right:**
```rust
pub enum IRuneS<'a> {
  CodeRune(&'a CodeRuneS<'a>),  // holds a struct
}
pub struct CodeRuneS<'a> {
  pub name: StrI<'a>,
  pub code_location: CodeLocationS<'a>,
}
```

### 1.3 Sub-Trait Hierarchies → Separate Enums With Conversions

When Scala has nested sealed traits (`sealed trait B extends A`), each level becomes its own Rust enum. The sub-enum's variants are a subset of the parent enum's variants. Conversion between levels uses `From` impls or explicit methods.

**Scala hierarchy:**
```scala
sealed trait INameS extends IInterning
  sealed trait IVarNameS extends INameS
    case class CodeVarNameS(...) extends IVarNameS
    case class ClosureParamNameS(...) extends IVarNameS
  sealed trait IFunctionDeclarationNameS extends INameS
    case class FunctionNameS(...) extends IFunctionDeclarationNameS
    case class LambdaDeclarationNameS(...) extends IFunctionDeclarationNameS
  case class LetNameS(...) extends INameS
```

**Rust translation:**
```rust
// Top-level enum wraps sub-enums as variants
pub enum INameS<'a> {
  VarName(&'a IVarNameS<'a>),
  FunctionDeclaration(&'a IFunctionDeclarationNameS<'a>),
  LetName(&'a LetNameS<'a>),
  // ...
}

// Sub-enum is its own independent enum
pub enum IVarNameS<'a> {
  CodeVarName(&'a CodeVarNameS<'a>),
  ClosureParamName(&'a ClosureParamNameS<'a>),
  // ...
}

// Sub-enum is its own independent enum
pub enum IFunctionDeclarationNameS<'a> {
  FunctionName(&'a FunctionNameS<'a>),
  LambdaDeclarationName(&'a LambdaDeclarationNameS<'a>),
  // ...
}

// Conversions between levels
impl<'a> From<&TopLevelStructDeclarationNameS<'a>> for TopLevelCitizenDeclarationNameS<'a> { ... }
impl<'a> From<&TopLevelInterfaceDeclarationNameS<'a>> for TopLevelCitizenDeclarationNameS<'a> { ... }
```

The parent enum wraps each sub-enum in one of its own variants (e.g. `INameS::VarName(&'a IVarNameS<'a>)`). Code that needs the broader type uses the parent enum; code that knows the specific sub-type works with the narrower enum directly. `From` impls let you go from narrow → wide.

### 1.4 Case Objects → Unit Structs

Scala `case object Foo extends Bar` becomes `pub struct Foo;` (a zero-sized type), wrapped in an enum variant where needed.

**Scala:**
```scala
case object PureS extends IFunctionAttributeS
case object SealedS extends ICitizenAttributeS
```

**Rust:**
```rust
pub struct PureS;
pub struct SealedS;

pub enum IFunctionAttributeS<'a> {
  Pure(PureS),
  // ...
}
```

---

## Part 2: Memory Management

### 2.1 GC References → Arena Allocation With Lifetime Parameters

Scala objects lived on the JVM's garbage-collected heap. In Rust, we use `bumpalo::Bump` arenas with explicit lifetime parameters. Every type that was a plain value in Scala gains lifetime parameters in Rust.

The codebase uses **four arena lifetimes** (documented in detail in `.claude/rules/postparser/early-lifetimes.mdc`):

- **`'a`** — Interner arena (longest-lived): all interned strings, names, types, coordinates
- **`'p`** — Parser AST arena: input nodes from the parser
- **`'s`** — Scout (postparser output) arena: transformed output nodes
- **`'ctx`** — Context/infrastructure borrows: `&'ctx Interner<'a>`, `&'ctx Keywords<'a>`

Functions that produce AST nodes call `arena.alloc(...)` and return `&'s T` instead of owned `T`. This means return types change from owned values in Scala to arena references in Rust:

**Scala:** `def translateStruct(...): StructA`
**Rust:** `fn translate_struct(...) -> &'s StructA<'a, 's>`

### 2.2 Arena-Allocated Structs Must Not Contain Malloc'd Collections (XAASSNCMC)

Structs stored via `bumpalo::Bump::alloc` (as `&'s T`) must use arena slices (`&'s [T]`) instead of `Vec<T>` for their collection fields.

**Why:** Bumpalo does not run destructors. A `Vec<T>` inside an arena-allocated struct will leak its heap allocation when the arena is dropped, because `Vec::drop` never runs.

**Scala:**
```scala
case class StructA(
  genericParameters: Vector[GenericParameterS],
  members: Vector[IStructMemberS])
```

**Rust:**
```rust
pub struct StructA<'a, 's> {
  pub generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
  pub members: &'s [IStructMemberS<'a>],
}
```

Use `alloc_slice_from_vec()` from `utils/arena_utils.rs` to convert a `Vec<T>` into `&'s [T]` before storing it in an arena-allocated struct.

### 2.3 Interning: `HashMap[T, T]` → Dual-Enum IDEPFL Pattern (XIID)

Scala's `Interner` used `HashMap[T, T]` with JVM reference equality (`eq`) to canonicalize case classes. Rust replaces this with arena-backed interning using two parallel enums per type hierarchy:

- A **reference enum** (canonical, holds `&'a` pointers) — used everywhere in the program
- A **value enum** (owned, used as HashMap lookup keys) — used only for interner lookups

The interning flow is:
1. Build an owned value (the Val enum variant) with all data inline
2. Look it up in `HashMap<ValEnum, RefEnum>` — if found, return the existing canonical ref
3. If new: allocate the payload into the `'a` arena, wrap in the ref enum variant, store the mapping

```rust
// Reference enum (canonical):
pub enum IRuneS<'a> {
  CodeRune(&'a CodeRuneS<'a>),     // holds &'a to arena
  ImplicitRune(&'a ImplicitRuneS),
}

// Value enum (for HashMap lookup):
pub enum IRuneValS<'a> {
  CodeRune(CodeRuneS<'a>),         // holds owned value
  ImplicitRune(ImplicitRuneS),
}
```

Identity comparison uses `ptr_eq()` / `canonical_ptr()` instead of JVM `eq`. Interned values must be interned immediately after creation — a bare value should only exist very temporarily before being handed to the `Interner`.

For nested interned types, a "shallow Val" struct exists where children are already-canonical references. Children must be interned first, then the parent Val is built with canonical child references. See the IDEPFL document in `.claude/rules/postparser/` for the full details.

### 2.4 Strings: `String` → `StrI<'a>` (More Aggressive Interning)

Scala: `case class StrI(str: String)` — GC-managed, interned via `HashMap`.
Rust: `pub struct StrI<'a>(pub &'a str)` — arena-backed interned string reference.

Rust interns strings more aggressively than Scala did. Many places where Scala held a plain `String` now hold `StrI<'a>`.

### 2.5 `Map[K, V]` → `HashMap<K, V>`

Scala's immutable `Map` becomes Rust's `HashMap`. Straightforward replacement.

---

## Part 3: Code Organization

### 3.1 Classes With Fields → Methods on a Shared Struct (or Free Functions With Explicit Params)

Scala had many small classes (`FunctionScout`, `ExpressionScout`, `PatternScout`, `TemplexScout`, `RuleScout`) each storing `interner`, `keywords`, etc. as constructor fields. In Rust, these were handled two ways:

**Strategy A — Collapse into one struct:** All the scout classes became methods on `PostParser`, which holds `interner`, `keywords`, and `arena` once. The delegate/callback patterns between them disappeared since they're all `self.method()` now.

**Scala (separate classes with delegate):**
```scala
class FunctionScout(postParser: PostParser, interner: Interner, ...) {
  val expressionScout = new ExpressionScout(
    new IExpressionScoutDelegate {
      override def scoutLambda(parentStackFrame, lambdaFunction0) = {
        FunctionScout.this.scoutLambda(parentStackFrame, lambdaFunction0)
      }
    },
    templexScout, ruleScout, patternScout, interner, keywords
  )
}
```

**Rust (methods on one struct, no delegate needed):**
```rust
impl<'a, 'p, 'ctx, 's> PostParser<'a, 'p, 'ctx, 's> {
  fn scout_function(&self, ...) { ... }
  fn scout_expression(&self, ...) {
    // Can call self.scout_lambda() directly — no delegate needed
    self.scout_lambda(stack_frame, &lambda.function)?;
  }
  fn scout_lambda(&self, ...) { ... }
  fn translate_pattern(&self, ...) { ... }
}
```

**Strategy B — Free functions with explicit params:** Where methods couldn't easily be on a struct (e.g. helper functions in `higher_typing_pass.rs`), fields like `interner` became explicit function parameters:

```rust
// Scala: method on a class that has `interner` as a field
// private def coerceKindLookupToCoord(runeAToType, ruleBuilder, range, resultRune, name) = ...

// Rust: free function, interner passed explicitly
fn coerce_kind_lookup_to_coord<'a>(
  interner: &Interner<'a>,  // was a class field in Scala
  rune_a_to_type: &mut HashMap<IRuneS<'a>, ITemplataType>,
  rule_builder: &mut Vec<IRulexSR<'a>>,
  range: RangeS<'a>,
  result_rune: RuneUsage<'a>,
  name: &IImpreciseNameS<'a>,
) { ... }
```

### 3.2 Anonymous Classes → Named Structs With Trait Impls

Scala's `new IFoo { override def bar(...) = ... }` anonymous class pattern can't exist in Rust. These become named structs with explicit trait implementations.

**Scala (anonymous class):**
```scala
val solveRule = new ISolveRule {
  override def complexSolve(...) = { ... }
}
```

**Rust (named struct + trait impl):**
```rust
pub struct RuneTypeSolverDelegate { pub predicting: bool }

impl SolverDelegate<...> for RuneTypeSolverDelegate {
  fn complex_solve(&self, ...) -> ... { ... }
}
```

This was applied in multiple places: `IdentifiabilitySolverDelegate`, `RuneTypeSolverDelegate`, `HigherTypingRuneTypeSolverEnv` (which replaced six inline Scala anonymous classes with one named struct).

### 3.3 Constructor-Parameter Closures → Trait Methods on Delegate

When Scala passed lambda parameters to a class constructor, Rust moves those lambdas into trait methods on a delegate.

**Scala:**
```scala
class Solver(
  ruleToPuzzles: Rule => Vector[Vector[Rune]],
  ruleToRunes: Rule => Iterable[Rune],
  solveRule: ISolveRule)
```

**Rust:**
```rust
trait SolverDelegate<Rule, Rune, ...> {
  fn rule_to_puzzles(&self, rule: &Rule) -> Vec<Vec<Rune>>;
  fn rule_to_runes(&self, rule: &Rule) -> Vec<Rune>;
  fn complex_solve(&self, ...) -> ...;
}

struct Solver<D: SolverDelegate<...>> {
  delegate: D,
}
```

### 3.4 By-Name Parameters → `FnOnce` Generic Params

Scala's by-name parameters (lazy evaluation blocks passed to functions) became Rust generic `F: FnOnce(...)` parameters.

```rust
fn new_block<F>(
  &self,
  env: EnvironmentS<'a>,
  parent_stack_frame: Option<StackFrame<'a>>,
  lidb: &mut LocationInDenizenBuilder,
  range_s: RangeS<'a>,
  context_region: IRuneS<'a>,
  initial_locals: VariableDeclarations<'a>,
  scout_contents: F,  // was a by-name block in Scala
) -> Result<..., ICompileErrorS<'a>>
where
  F: FnOnce(StackFrame<'a>, &mut LocationInDenizenBuilder) -> Result<..., ICompileErrorS<'a>>,
```

### 3.5 Companion Objects → `impl` Blocks or Free Functions

Scala companion objects with factory methods, extractors (`unapply`), and utilities became either `impl` blocks with associated functions or standalone `pub(crate) fn` functions.

---

## Part 4: Error Handling

### 4.1 Exception Throwing → `Result<T, E>` With `?` (XRRIF)

Scala's `throw CompileErrorExceptionS(...)` becomes `return Err(...)` with `Result` return types and `?` propagation. Because of this, a vast number of functions in Rust have `Result` return types — this is expected and fine. Many functions that didn't return `Result` in Scala now do, because one of their indirect callees returns a `Result`.

### 4.2 Unimplemented Code Must `panic!()` (XTUCMP)

Every TODO or unimplemented branch gets a `panic!()` with a unique identifying message. Functions that are entirely `panic!()` stubs are fine during migration — they mark code that hasn't been translated yet and will fail loudly at runtime if accidentally reached.

```rust
pub fn scout_loop(&self, ...) -> ... {
  panic!("Unimplemented scout_loop");
}
```

### 4.3 Fail Fast, Never Recover (XFFFL, XNRAF)

Propagate errors immediately using `?`. Panic on unexpected conditions, protocol violations, and malformed input. Never silently fail, log-and-continue, use defaults, skip bad data, or gracefully degrade.

---

## Part 5: Fidelity Principles

These rules ensure the Rust code stays as close to Scala as possible, making it easier to verify correctness and complete the migration.

### 5.1 Mirror Scala As Close As Possible (XRSMSCP)

Keep Rust implementations mirroring Scala exactly: same functions, their positions relative to each other, their names, their logic, and where possible variable names too. `panic!()`s for unimplemented features are acceptable.

### 5.2 Port Structure Exactly (XPSE)

Port the Scala structure as-is, with panics for unimplemented parts. Don't restructure, simplify, or "improve" during migration.

### 5.3 No Novel Code During Migrations (XNNCDM, XNND)

All Rust code must correspond to Scala code. Novel implementations are forbidden. No new functions, structs, traits, enums, impl blocks, type aliases, or consts/statics that don't exist in the corresponding Scala. Use `panic!()` stubs instead of novel implementations.

The exceptions to this are infrastructure required by Rust's ownership model (arenas, lifetime parameters, the dual-enum interning system, `From` impls between enum levels, `canonical_ptr`/`ptr_eq` methods).

### 5.4 No Moved Definitions (XNMD)

Definitions must stay in their original file locations relative to the Scala comments. Moving them breaks the 1:1 spatial mapping that makes migration verification possible.

### 5.5 Same Helper Calls (XSHCNE)

Rust must call the same helper functions as Scala. Don't inline helpers, extract new ones, or substitute different functions. If Scala calls `foo()` then calls `bar()`, Rust should call `foo()` then `bar()`.

### 5.6 Closer to Scala, Not Further (XCSTNF)

Every change must move the Rust code closer to Scala's structure, not further away. If you're tempted to restructure something, resist — match Scala first, refactor later (if at all).

### 5.7 No Valid Simplifications (XNVSE)

Don't assume something is a "valid simplification for migration purposes." Don't assume we can make up something simpler because we're migrating piece by piece. Port the Scala code exactly. Don't take shortcuts.

### 5.8 Don't Conveniently Change Requirements (XDCCR)

If a test fails, never make the test expect the current bad behavior. The Scala tests all passed. The Rust tests should pass with the exact same expectations. Fix the code to match Scala semantics, don't adjust expectations.

---

## Part 6: Naming and Style

### 6.1 `camelCase` → `snake_case` (XNRD)

Field names, method names, and local variables convert: `packageCoord` → `package_coord`, `scoutLambda` → `scout_lambda`, `maybeParent` → `maybe_parent`. Definition names must otherwise match Scala exactly (accounting for this case convention change).

### 6.2 Suffix Variables for Stages (XSWDWMS)

In functions handling multiple data stages (which is common — most functions transform data from one stage to the next), suffix local variables so it's clear which stage they reference:

```rust
let header_range_s = PostParser::eval_range(file, header_range);
let range_s = PostParser::eval_range(file, range);
let ret_range_s = PostParser::eval_range(file, ret_range);
```

### 6.3 Rust Code Goes Above Its Scala Comment (RCSBASC)

For every Rust definition, put it directly above the old Scala definition comment. New Rust definitions are interleaved with old Scala comments. Don't change or remove Scala comments, but you may split a comment into two so you can put Rust code between them.

### 6.4 Migrate All Comments (XMACT)

All Scala comments must be ported to Rust. Rust may have additional comments that Scala doesn't have, that's fine.

### 6.5 Use `use` Imports, Not `crate::` Paths in Bodies (XUUSNNCB)

Add `use` statements at the top for short names; avoid `crate::module::Type` inline in function bodies.

### 6.6 Eliminate All Warnings (XEAW)

Work is not done if there are still compiler warnings.

### 6.7 No Expensive Clones (XNEC)

Stop and ask the human before implementing `Clone` for a potentially large data structure. Cloning is sometimes needed in Rust when it's not in Scala, but only for value types — nothing that might ever be mutated.

### 6.8 Keep Inline Comparisons Inline (XKICI)

If Scala has an inline comparison/check inside a `match`/`case`, keep that check inline in the Rust pattern. Don't move it into a guard. Don't move it outside the match.

### 6.9 `Profiler.frame` Wrappers → Dropped

Scala wrapped hot paths in `Profiler.frame(() => { ... })`. Rust drops these entirely — there is no equivalent and they're not needed.

### 6.10 Explicit Arguments, No Defaults (XEANODV)

Every argument must be explicitly supplied. No optional or defaulted parameters.

### 6.11 Never Downcast Traits (XNEDC)

No `downcast_ref`, `downcast_mut`, or `Any` inspection. If you need a downcast, the trait is missing methods.

---

## Part 7: Testing

### 7.1 All Tests Must Pass (XATEISP)

Every Scala test needs a corresponding Rust test. Don't assume any tests are unimportant or unnecessary.

### 7.2 No Conditionals in Tests (XNHCIT, XNCTOBPAOP)

Tests should be rigid with hard expectations, no if-statements. If a match is needed, one branch proceeds, all others panic.

### 7.3 Use `expect_` Functions and `collect_` Macros (XUEFIAI, XUCMTRS)

Instead of asserting length then indexing, use `expect_1`, `expect_2`, etc. Use `collect_only!`, `collect_where!` for recursive pattern matching across ASTs.

### 7.4 Tests Prefer `unwrap` Over `expect` (XTPUTEFC)

Use `.unwrap()` instead of `.expect()` in tests for brevity.

### 7.5 Never Repeat Implementation Code in Tests (XNRICIT)

Tests should call public APIs and assert on behavior, not duplicate implementation logic.

### 7.6 Prefer Single Match Over Nested Matches (XPSMONM)

Use one match with nested patterns instead of multiple nested matches.

---

## Part 8: Allowed Differences

These differences between Scala and Rust are expected and should not be flagged:

- **Arc wrapping:** Shared values may need `Arc` (but `Arc<Mutex<T>>` is a code smell requiring human-written justification)
- **Clone:** Sometimes needed in Rust when not in Scala, but only on value types
- **Box:** Sometimes needed in Rust for recursive types
- **`Profiler.frame()`** calls are dropped
- **`StringBuilder`** → `String::new()` / `push_str()`
- **`vimpl()`** → `panic!()`
- **`vassert()`** → `assert!()`
- **`vassertSome`** → `.expect()`
- **`Accumulator`** → `Vec`
- **`Either`** → custom Rust enums
- **Unused Scala variables** → underscored in Rust
- **Multi-line vs single-line strings** in tests
- **`match` vs `assert!(matches!(...))}`** — logically equivalent checks are fine
- **Asserting length** — `assert_eq!(args.len(), 3)` vs Scala pattern `Vector(_, _, _)` are both fine

---

## Appendix: Luz Principle Cross-Reference

The strategies above incorporate the following principles from `/Volumes/V/Luz/`:

| Code | Principle | Sections |
|------|-----------|----------|
| XSSTRE | Scala Sealed Traits to Rust Enums | 1.1 |
| XESCCD | Enums Shouldn't Contain Complex Data | 1.2 |
| XAASSNCMC | Arena Structs No Malloc'd Collections | 2.2 |
| XIID | Immediate Interning Discipline | 2.3, 2.4 |
| XRSMSCP | Rust Should Mirror Scala | 5.1 |
| XPSE | Port Structure Exactly | 5.2 |
| XNNCDM | No Novel Code During Migrations | 5.3 |
| XNND | No New Definitions | 5.3 |
| XNRD | No Renamed Definitions | 6.1 |
| XNMD | No Moved Definitions | 5.4 |
| XSHCNE | Same Helper Calls No Exceptions | 5.5 |
| XCSTNF | Closer to Scala Not Further | 5.6 |
| XNVSE | No Valid Simplifications | 5.7 |
| XDCCR | Don't Conveniently Change Requirements | 5.8 |
| XNCWSR | No Changes Without Scala Reference | 5.3 |
| XNASC | No Adding Scala Comments | 6.3 |
| XRRIF | Returning Result Is Fine | 4.1 |
| XTUCMP | TODOs Must Panic | 4.2 |
| XFFFL | Fail Fast Fail Loud | 4.3 |
| XNRAF | Never Recover Always Fail | 4.3 |
| XMACT | Migrate All Comments Too | 6.4 |
| XSWDWMS | Suffix When Dealing With Multiple Stages | 6.2 |
| XNEC | No Expensive Clones | 6.7 |
| XKICI | Keep Inline Comparisons Inline | 6.8 |
| XEANODV | Explicit Arguments No Defaults | 6.10 |
| XNEDC | Never Downcast Traits | 6.11 |
| XUUSNNCB | Use `use` for Short Names | 6.5 |
| XEAW | Eliminate All Warnings | 6.6 |
| XATEISP | All Tests Must Pass | 7.1 |
| XNHCIT | No Conditionals in Tests | 7.2 |
| XNCTOBPAOP | One Branch Proceeds Others Panic | 7.2 |
| XUEFIAI | Use expect_ Functions | 7.3 |
| XUCMTRS | Use collect_ Macros | 7.3 |
| XTPUTEFC | Tests Prefer unwrap | 7.4 |
| XNRICIT | Never Repeat Implementation in Tests | 7.5 |
| XPSMONM | Prefer Single Match Over Nested | 7.6 |
| XAIMITIP | Avoid if matches! in Tests | 7.2 |
| XPROPRC | Prefer Result Over Panic for Recoverable | 4.1 |
