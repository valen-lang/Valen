# Bare-Use Clone, Postfix `x&` Borrow, Postfix `x^` Move

**Status:** design (not yet implemented). Captures the long-term semantic direction for
local-variable / field use. Authored after the kind-mutability cut as the principled
destination that resolves the cut's remaining band-aids (Q1/Q5/Q6/Q7 in the cut review).

## The model

At source level, three syntactic forms govern how a value is referenced:

| Source | Meaning | Desugars to |
|---|---|---|
| `x` (bare) | **Clone** the value | `clone(&x)` |
| `x&` | **Borrow** — produce a `&T` reference | `LocalLoadTE(target=Borrow)` |
| `x^` | **Move** — transfer ownership; `x` is dead afterward | `LocalLoadTE(target=Own)` (Unstackify) |

Both `&` and `^` are **postfix** — they sit after the expression they modify. This generalizes
naturally to chained access: `x.field&` borrows the field; `x.field^` moves it out.

The same rule applies to field access:

| Source | Meaning |
|---|---|
| `s.x` | Clone the field via `&s.x` |
| `s.x&` | Borrow the field |
| `s.x^` | Move the field out (partial move; `s` is in a destructured state) |

### Method-call receivers

Until overloading is removed and proper auto-borrow lands, method receivers use **postfix `&`
in dot-call position**:

| Source | Meaning |
|---|---|
| `x&.foo()` | Borrow-call: receiver is `&T`. Equivalent to `foo(x&)`. |
| `x^.foo()` | Move-call: receiver is `T` (owning). Equivalent to `foo(x^)`. |
| `x.foo()` | Clone-call: receiver is `T` (cloned). Equivalent to `foo(clone(&x))`. |

Once overloading is removed, the `x.foo()` shorthand will auto-borrow at the receiver — that's
the **one** auto-coerce the language permits in this model. Until then, callers write `x&.foo()`
explicitly.

### Cloneability

A type `T` is cloneable iff there is a `func clone(&T) T` in scope at the use site. There are
three kind-classes, each with a different runtime `clone` implementation:

| Kind | `clone(&T) T` source | Runtime cost |
|---|---|---|
| Primitive (`int`, `bool`, `float`, `void`, `never`) | Compiler-provided builtin via `__copy_prim` | Scalar copy (free) |
| `class` type (formerly `share`) | **Compiler-auto-derived** via `__rc_alias` | Refcount bump (cheap) |
| `struct` type (owned/unique) | User-provided | Whatever the author wrote |

```vale
// Primitive built-ins (in builtins/clone.vale):
func clone(x &int) int { return __copy_prim(x); }
func clone(x &bool) bool { return __copy_prim(x); }
// etc.

// Class auto-derived (synthesized at typing-pass time per `class` decl):
class Ship { fuel int; }
// Compiler synthesizes:
//   func clone(x &Ship) Ship { return __rc_alias(x); }
// User cannot override this — it's load-bearing for the kind-class invariant.

// Struct user-provided:
struct Vec3 { x int; y int; z int; }
func clone(v &Vec3) Vec3 { return Vec3(v.x, v.y, v.z); }
```

For generic functions that need to bare-use a `T`, a `where func clone(&T) T` bound is required.
**Class instantiations satisfy this bound for free** (auto-derived `clone` is always in scope);
struct instantiations require the struct to have a user-provided clone.

If bare-use of `x: T` is attempted and `clone(&T) T` is not in scope, the typing pass produces
a typed compile error:

> Cannot use bare reference to `x` of type `T` because `T` is not cloneable here.
> Try `x^` to move, `x&` to borrow, or add `where func clone(&T) T` to the surrounding function.

### Class types: RC-aliasing bare-use

For `class Ship { fuel int; }`:

```vale
let f = Ship(42);   // creates Ship, refcount = 1
let g = f;          // bare-use → clone → __rc_alias → refcount = 2; f and g point at same Ship
let h = f^;         // move; f dead; refcount unchanged (h owns f's slot)
let i = f&;         // borrow; refcount unchanged
```

Bare-use of a class type is always cheap (refcount increment, no copy of the body). This is the
runtime semantics of the old "share" types, surfaced cleanly via the bare-use rule.

### Struct types: explicit-clone bare-use

For `struct Vec3 { x int; y int; z int; }` with user-defined `clone`:

```vale
let a = Vec3(1, 2, 3);
let b = a;          // bare-use → clone → calls user's clone fn → b is independent Vec3
let c = a^;         // move; a dead; c owns
let d = a&;         // borrow
```

If `Vec3` has no `clone` defined, bare-use is a compile error. Author must add `clone`, or use
explicit `^` / `&`.

### Lambda captures (C++-style explicit list)

Closures specify their capture list explicitly:

| Capture | Meaning | Closure body sees `x` as |
|---|---|---|
| `[x]` | **Clone-capture** — clone outer `x` into closure state | `T` (owned by closure) |
| `[x&]` | **Borrow-capture** — closure holds `&T` borrow | `&T` |
| `[x^]` | **Move-capture** — outer `x` moved into closure; outer dead | `T` (owned) |

Inside the closure body, the same bare/`&`/`^` rule applies to the captured variable
recursively.

**For class types, `[x]` is the natural cheap default** — it RC-aliases the captured value. For
struct types, `[x]` might be expensive (full deep clone via user's fn); authors should consider
`[x&]` or `[x^]` instead. The kind-class makes the perf model visible at the capture site.

**Subtlety:** `clone(&T) T` must be in scope at the **closure definition site** for `[x]`
clone-capture (the closure's captured-state-init runs there).

## What this changes

### `__copy_prim` becomes purely internal

The user-facing source-level intrinsic `__copy_prim(x)` disappears. The intrinsic survives as
the implementation of `func clone(&int) int` and similar in builtins. The postparsing
`CopyPrimSE` variant, the scout handler, and the typing-pass `IExpressionSE::CopyPrim` branch
are all removable once typing-pass auto-insertion replaces source-level syntax.

The downstream IR nodes (`CopyPrimTE` / `CopyPrimIE` / `CopyPrimH` / Backend `CopyPrim`) stay —
they're the post-elaboration *operation*, produced by the typing pass at every bare-use site
for primitives.

### `__rc_alias` joins as a sibling intrinsic

Parallel to `__copy_prim`:

- New typing-level intrinsic `__rc_alias(x)` — produces a fresh RC-aliased reference to a class
  value
- Auto-synthesized into the body of every class's auto-derived `clone`
- Lowers to existing Backend RC-alias codegen (already present for current "share" alias paths)

### Move-tracker behavior flips

Today: bare-use of `x` consumes `x` (move semantics). Subsequent uses error.

New: bare-use clones (requires clone bound for struct types; free for class/primitive). Only
`x^` consumes.

```vale
// Today:
let y = x;       // moves x; x is dead.
foo(x);          // moves x into foo; x is dead.
return x;        // moves x out as return value.

// New (T = class or struct-with-clone):
let y = x;       // clones x; x and y both valid.
let y = x^;      // moves x; x dead, y owns.
foo(x);          // clones x to pass; x still valid.
foo(x^);         // moves x; x dead afterward.
return x;        // clones x; x lives until scope end (then dropped).
return x^;       // moves x out; x dead.
```

### `share` keyword becomes `class`

Today's `struct Ship share { fuel int; }` becomes `struct Ship class { fuel int; }` as an
intermediate step, then eventually `class Ship { fuel int; }` as the canonical syntax. Same
runtime semantics (RC'd, shared, refcount-aliased on bare-use).

The parser's current `share`-keyword overload disambiguation post-process (Q4 in the cut review)
is replaced by direct lexical recognition of `class` in struct-header position.

### `@T` syntax retires

Pre-cut `@T` meant Share-augmented T. With the cut + this model, kind-class is declared at the
type's definition site (`class Foo` vs `struct Foo`) and reference ownership is declared at the
use site via bare/`&`/`^`. No remaining need for `@T` syntax.

### Generic function bounds proliferate honestly — but less than feared

Generic code that uses `T` bare-style declares the requirement:

```vale
// Today (relies on Share=auto-copy, which the cut removed):
func swap<T>(a &T, b &T) (T, T) { return (a, b); }  // errors today

// New:
func swap<T>(a &T, b &T) (T, T) where func clone(&T) T {
  return (clone(a), clone(b));  // or just `(a, b)` if auto-insert handles the bare-use rule
}
```

But because class instantiations satisfy `where func clone(&T) T` for free (auto-derived clone),
**generic code over T parameters that are usually class types** (containers, callbacks holding
state) won't see bound-proliferation pain at use sites. Only generic code over struct types
forces the user-provided clone in scope.

### TSUGAR markers split into two classes

The ~310 `// TSUGAR:` markers in the current tree fall into:

- **`__copy_prim(...)` source wraps** — ~150 markers, **removable**. The typing pass auto-inserts
  CopyPrim for bare-use of primitives; the source wrap becomes redundant.
- **`&` / `__copy_prim` on function-arg/member-access** (e.g. `m.has(&N)`, `&true bork &false`,
  `__copy_prim(m.hp)`) — ~160 markers, **stay** (they're explicit borrows / member-clones).

Roughly half the markers go away; the other half become natural Vale syntax.

## What this resolves from the cut's review

The kind-mutability cut left several band-aids tracked as Q1–Q11 in the post-cut review. This
model resolves these as follows:

| Question | Resolution under this model |
|---|---|
| Q1: LocalLoadH absorbs borrow→value | Required to invert. LocalLoadH(target=Borrow) on primitive produces `MutableBorrowH+InlineH+prim`. CopyPrimH does the borrow→value conversion. |
| Q5: 11 `inline_primitive_uniform_coord_h` call sites | Density collapses to ~1–2 sites (canonical primitive-literal shape rule). Load sites use `target_ownership` directly. |
| Q6/Q7: "OwnH-only-for-primitive" testvm asserts | Disappear. LocalLoadH/MemberLoadH never produce OwnH for primitives; CopyPrim does. |
| Q9: `__copy_prim` panic for non-primitive | Stays meaningful but trigger sites move from user source to typing-pass auto-insertion at `convert_helper.rs`. Replace panic with typed `ICompileErrorT::CopyPrimNonPrimitive`. |
| Q2/Q3/Q10/Q11 | Orthogonal — these are pre-cut leftovers (Share+primitive producers, dead consumer arms, missing constructor assert). Fix surgically in **Phase 0** below; not dependent on this model. |
| Q4 | Folded in: `share` keyword retires in favor of `class`. Parser disambiguation hack removable. |

## Out of scope (deliberately)

- **Auto-borrow at call sites (general).** This model does NOT auto-insert `&` when a function
  param is `&T` and the caller passed bare `T`. Bare-use clones to `T`; if the param is `&T`,
  that's a type error. The single exception is method-receiver auto-borrow, deferred until
  post-overloading-removal.
- **`Copy` marker trait (Rust-style).** This model treats *every* type as cloneable iff
  `clone(&T) T` is in scope. There is no distinct "trivially copyable" trait. Primitives just
  happen to have a free `clone` impl via `__copy_prim`; classes via `__rc_alias`. If we want a
  perf distinction later, it lives as a separate concern, not a type-system feature.
- **`Clone` trait shorthand** (e.g. `<T: Clone>` desugaring to `where func clone(&T) T`). Not
  in this arc; bare `where func clone(&T) T` is fine.
- **Drop semantics.** Unchanged. `clone(&T) T` does not interact with `drop(T) void`. A type
  may be cloneable but not droppable, or vice versa.
- **Last-use auto-move (NLL-style).** Bare-use always clones. The author writes `^` to move.
  Future arc could promote last-use bare to move, but not in this design.

### vivem caveat: mutating-through-borrow on primitives

The testvm (vivem) treats every value as a separately-allocated heap object with a unique
`AllocationIdV`, where references are entries in a per-allocation `referrers` map. The same
machinery handles primitives and structs uniformly. Under that model, a borrow Ref to a
primitive (`MutableBorrow + Yonder + IntHT`) is just another referrer pointing at the same
`AllocationV` as the owning local — same shape as a borrow of a struct.

That's enough for *reading* through a primitive borrow (`__copy_prim(&my_int)` materializes a
fresh value). But a future `*my_int_ref = 42` (mutating the int's value through a borrow) is
genuinely hard to express in the vivem's allocation-with-identity model: a primitive
`AllocationV`'s `KindV::Int(IntV { value, .. })` is the storage, but mutating it through a
borrow would require either (a) interior-mutability on the IntV cell, breaking the vivem's
"plain HashMap + &mut self" discipline, or (b) re-allocating the IntV in place with the new
value, which would change observable behavior under cycle-of-references inspection.

For *this* design we don't depend on `*ref = value` for primitives — bare-use clones, `&x`
borrows for reading, `^x` moves. Mutating a remote primitive through a borrow isn't part of
the surface semantics described above. If we later add `*x = ...` as a language feature for
primitives, the vivem will need a non-trivial rework (probably adopting `Cell<KindV>` for
primitive allocations, or a different storage model for primitives specifically).

**Interim heap-assertion relaxation:** until that work lands, the vivem will accept
`Borrow + Yonder + primitive` references as runtime-equivalent to `Own + Inline + primitive`
references — both are entries pointing at the same primitive `AllocationV`, and the vivem's
load/discard paths treat them identically. The `heap.rs::get_reference_from_local` and
`check_reference` asserts that today require exact `(ownership, location, kind)` equality get
relaxed for primitive-kind allocations: kind must match; ownership/location flavors may
differ. This is honest because in the new model, a borrow of a primitive really is "just
another referrer to the same int allocation" — only reading. The day someone wants to write
through that borrow, the asserts come back (and the allocation model needs the rework).

## Implementation arc

Phased so the suite stays green throughout. Each phase verifiable.

### Phase 0 (lands first, independent of the rest)

Orthogonal surgical fixes from the cut's Q2/Q3/Q4/Q9/Q10/Q11 review. These remove the
Share+primitive band-aids that would otherwise interact weirdly with the bare-use desugar:

| Step | What | Files |
|---|---|---|
| 0.1 | Fix 4 typing-pass producers of `Share+primitive`: `DestroyMutRuntimeSizedArrayTE`, `RuntimeSizedArrayCapacityTE`, `PushRuntimeSizedArrayTE`, `DestroyTE` — emit `Own+Void` / `Own+Int`. (Q11) | `typing/ast/expressions.rs` |
| 0.2 | Delete dead `(Share, Bool)` consumer arm. (Q11) | `typing/expression/expression_compiler.rs:977` |
| 0.3 | Delete `SoftLoad (Share, Own) → Own` band-aid. (Q2) | `instantiating/instantiator.rs` |
| 0.4 | Delete `compose_ownerships` primitive short-circuit (regular `(Borrow, Own) → MutableBorrow` arm becomes correct). (Q3) | `instantiating/instantiator.rs:2081` |
| 0.5 | Add `CoordT::new()` / `CoordI::new()` / `CoordH::new()` ctor asserts: kind is primitive ⇒ ownership ∈ {Own, Borrow, Weak}. Locks Share+primitive out forever. (Q10) | `typing/types/types.rs`, `instantiating/ast/types.rs`, `final_ast/types.rs` |
| 0.6 | Replace `__copy_prim` panic at expression_compiler.rs:768 with typed `ICompileErrorT::CopyPrimNonPrimitive`. (Q9) | `typing/expression/expression_compiler.rs`, `typing/compiler_error_humanizer.rs` |
| 0.7 | Disambiguate `share` keyword in templex parser by context (struct/interface header position → Sharedness; otherwise → Ownership). Removes the post-parse alias-rewrite. (Q4) | `parsing/templex_parser.rs`, `parsing/parser.rs` |
| 0.8 | Decide tup0.vale Share-vs-Own question (likely flip to Own); rename `Sharedness → Sharedness` to match Backend's terminology (cosmetic; defer if not now). | `builtins/resources/tup0.vale`, `final_ast/ast.rs:208`, various |

After Phase 0 the suite holds at green and the 7 DO NOT SUBMIT markers in the cut's diff shrink
to ≤2 (the `tup0.vale` design note and the optional `Sharedness → Sharedness` rename).

### Phases A–I (the actual semantic shift)

| Phase | What | Key files |
|---|---|---|
| A | `convert_helper.rs`: when coercing `&T → T`, look up `clone(&T) T` in scope. For primitive: emit CopyPrim. For class (auto-derived clone): emit `__rc_alias`. For struct: emit user's `clone()` call. Error if no clone available. | `typing/convert_helper.rs` |
| A.5 | Auto-derive `clone(&T) T` for every `class` declaration; lower body to `__rc_alias` intrinsic. Mirrors `drop` auto-derivation. User-defined `clone` for a class is a compile error. | `typing/macros/citizen/`, `typing/macros/class_clone_macro.rs` (new) |
| B | Typing-pass bare-use desugar: `IExpressionSE::LocalLookup` / `MemberLookup` without postfix `&`/`^` wraps source in implicit `clone(...)` resolution. | `typing/expression/expression_compiler.rs` |
| C | Move-tracker change: bare-use no longer counts as move; only `x^` does. Update `determine_if_local_is_addressible` / move-tracking analysis. | `typing/expression/local_helper.rs`, `typing/expression/expression_compiler.rs` |
| D | Postfix `^` / `&` syntax: parser changes. Add `x^` and `x&` as expression suffixes; preserve prefix-`&` short-term during transition if needed, then retire. | `parsing/expression_parser.rs`, `postparsing/expression_scout.rs` |
| D.5 | `x&.foo()` / `x^.foo()` method-call receiver syntax. Lex/parse `&.` and `^.` as combined receiver-borrow / receiver-move tokens. | `parsing/expression_parser.rs` |
| E | Q1 H-IR alternative: `LocalLoadH(target=Borrow)` on primitive produces `MutableBorrowH+InlineH+prim`. CopyPrimH does real conversion. Delete primitive carve-out in `LocalLoadH::result_type()`. | `final_ast/instructions.rs`, `simplifying/load_hammer.rs`, `testvm/expression_vivem.rs` |
| F | Backend: widen `primitives.h` asserts to accept primitive borrows; **construct primitive borrow Refs ad-hoc at `metal_lowerer`** (no new singletons in `metalcache.h`). | `Backend/src/region/common/primitives.h`, `FrontendRust/src/backend_ffi/metal_lowerer.rs` |
| G | Lambda capture list parser + typing-pass capture-mode handling for `[x]` / `[x&]` / `[x^]`. | `parsing/expression_parser.rs`, `postparsing/expression_scout.rs`, `typing/expression/expression_compiler.rs` (closure path) |
| H | `share` → `class` keyword rename: lexer/parser. Intermediate accepts both `struct Foo share` and `struct Foo class`; eventually only the standalone `class Foo { ... }` form. Test-program sweep. | `parsing/parser.rs`, all `*.vale` files using `share` |
| I | `@T` syntax retires. Remove from parser; sweep test programs that still use it. | `parsing/templex_parser.rs`, test programs |
| J | TSUGAR sweep: remove `__copy_prim(...)` wraps in builtin Vale + test programs. Verify by running suite (bare-use auto-insertion replaces them). Also delete source-level `__copy_prim` syntax (CopyPrimSE, IRulexSR::CopyPrim, scout handler, typing-pass syntax branch). Keep CopyPrimTE/IE/H/Backend::CopyPrim for the operation. | `FrontendRust/src/builtins/`, `FrontendRust/src/tests/`, `FrontendRust/src/integration_tests/`, postparsing/ |

**Estimated total scope:** ~3–5 days of focused work, plus the migration sweep (~250–350 site
touches across builtins + test programs, net line-count reduction of ~100 lines). Each phase has
natural checkpoints.

## Migration scope investigation (informational)

Counted across 212 `.vale` files in scope (builtins + stdlib + test programs):

| Pattern | Count | Effect under new model |
|---|---|---|
| Bare-return `return identifier;` | 61 (51 test programs, 10 stdlib) | ~70% primitive (wins, no edit). ~30% non-primitive → need `x^` postfix where author intends move. |
| Bare assignment `var = identifier;` | 4 | Same split. |
| Single-arg function calls `foo(x)` | ~330 | Same split. Primitives become wins; non-primitive moves need `^`. |
| Generic functions in core stdlib | 68 (hashmap/list/arrays/result/opt) | ~30–40 likely need `where func clone(&T) T` bound. **Class instantiations satisfy for free**, so most call sites unchanged. |
| Existing `// TSUGAR:` markers | 310 | ~150 `__copy_prim(x)` wraps DELETED. ~160 `&`/member-access markers STAY. |
| `share` keyword in struct/interface decls | ~15 sites | Renamed to `class`. |
| `@T` source-level usage | small handful | Removed. |
| Method calls needing `x&.foo()` rewrite | ~20–50 estimated | Migration edit (until post-overloading auto-borrow). |
| Closure captures needing `[]` list | ~30–50 closures | `[x]` (clone-capture) is cheap for class types; struct closures should consider `[x&]`/`[x^]`. |

**Net edit volume:** ~250–350 site touches; ~100-line net deletion. Comparable to the
kind-mutability cut's scope. No programs break semantically — every change is an opt-in
annotation of move-vs-clone intent.

## Verification at completion

1. `cargo nextest run --manifest-path FrontendRust/Cargo.toml --lib --no-fail-fast` — must hold
   1166 / 0 / N (N ≤ pre-arc skipped count).
2. `git grep '__copy_prim'` in `FrontendRust/src/tests/` and `FrontendRust/src/integration_tests/`
   returns zero source-position hits (only in builtin definitions and as the intrinsic name).
3. `git grep 'TSUGAR' FrontendRust/src/` — count roughly halved (only `&` / member-access markers
   remain).
4. The 7 DO NOT SUBMIT markers in the cut's diff reduce to ≤1 (the `tup0.vale` design
   question if not resolved).
5. `CoordT` / `CoordI` / `CoordH` constructors assert: kind is primitive ⇒ ownership ∈ {Own,
   Borrow, Weak}. Share+primitive becomes statically impossible to construct.
6. `git grep '\bshare\b'` in test programs returns zero hits (all migrated to `class`).
7. `git grep '@\w' FrontendRust/src/tests/programs/*.vale` returns zero `@T`-style usages.

## Open questions

1. **Closure capture default: empty `[]` required, or implicit "auto-capture by reference"?**
   C++ requires explicit `[]`. Vale today implicitly captures by reference. Recommend: require
   `[]` (empty list) for "no captures"; require explicit capture for any used outer variable.
   Migration: add `[var1, var2&]` to every existing closure in test programs.
2. **Move-out of borrow.** `let y = ((&x).field)^;` — error (can't move through a borrow), or
   does it auto-clone? Recommend matching Rust: error.
3. **Last-use auto-promote-to-move.** Out of scope for this arc per the design rule, but worth
   noting: a future arc could promote bare-use that's the syntactically last use of a binding
   to move (NLL-style), eliding the clone where wasteful. Until then, every `return x;` clones
   even when the binding goes out of scope immediately afterward — a perf footgun for
   large structs (mitigated for class types by the cheap RC-bump clone).
4. **Method receiver auto-borrow timing.** Deferred to post-overloading-removal. Until then,
   every method call writes `x&.foo()` explicitly. Worth a tracking issue so the auto-borrow
   work is paired with overloading removal.
5. **`class Ship { ... }` standalone form vs `struct Ship class { ... }` intermediate.**
   Intermediate-step compatibility means both work for a transition period. Decide the
   end-state form (probably standalone `class Ship`) and the deprecation timeline for the
   `struct Ship class` intermediate.

These should be resolved before Phase B lands (the bare-use desugar phase).
