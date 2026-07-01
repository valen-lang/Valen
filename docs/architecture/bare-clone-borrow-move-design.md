# Bare-Use Borrow, Postfix `x^` Move (with target-side auto-coercions)

**Status:** design (partial implementation in progress). Captures the long-term semantic direction for local-variable / field use. Reflects the refined model landed after the CHECKPOINT-22 Mission redesign (see `vcoord-handoff.md` § "Overload resolution & dispatch model redesign"). The original framing was "bare-use → Clone"; that has been refined to "bare-use → Borrow, with target-side auto-coercions" — same outcomes for most cases, but stricter for Own non-primitive → Own (compile error rather than silent clone) and cleaner when the target only wants a borrow.

## The model

At source level, two syntactic forms govern how a value is referenced (plus `x&` as an optional explicit spelling for the bare-use borrow):

| Source | Meaning | Desugars to |
|---|---|---|
| `x` (bare) | **Borrow** — produce a `Borrow`-flavored coord | `LocalLoadTE(target=Borrow)` |
| `x&` | **Borrow** (explicit spelling; identical to bare) | `LocalLoadTE(target=Borrow)` |
| `x^` | **Move** — transfer ownership; `x` is dead afterward | `LocalLoadTE(target=Own)` (Unstackify) |

`&` and `^` are **postfix** — they sit after the expression they modify. This generalizes naturally to chained access: `x.field&` and `x.field^` borrow / move the field.

The same rule applies to field access:

| Source | Meaning |
|---|---|
| `s.x` | Borrow the field (Borrow-flavored coord over `s.x`) |
| `s.x&` | Borrow the field (explicit spelling; identical) |
| `s.x^` | Move the field out (partial move; `s` is in a destructured state) |

### Target-side auto-coercions

The bare-use borrow is materialized at the source; the target position decides whether a coercion fires. Two auto-coercions exist:

| Source shape | Target wants | Action |
|---|---|---|
| `Borrow + primitive` | `Own + primitive` | Auto `implicit_clone(&p)` → fresh Own primitive (scalar copy) |
| `Borrow + share-kind` | `Share T` | Auto-alias via `__rc_alias` (refcount bump) |
| `Borrow + primitive` | `Borrow + primitive` | pass-through |
| `Borrow + share-kind` | `Borrow + share-kind` | pass-through |
| `Borrow + kind` | `Borrow + kind` | pass-through |
| `Borrow + non-primitive kind` | `Own + non-primitive kind` | **error** — user must write `x^` (move) or `clone(&x)` (explicit) |
| `Borrow + *` | `Own + *` (other than primitive) | **error** |
| `Own + *` (from `x^`) | matching Own | pass-through |
| `Share + *` (already Share) | `Share + *` | pass-through |

Only the two auto-coercions listed on lines 1–2 fire silently. Everything else is either pass-through (same shape) or a compile error requiring explicit user action (`x^`, `clone(&x)`, etc.).

### Method-call receivers

**Dot is pure sugar over the free-function call form.** `x.foo()` and `foo(x)` use the identical mechanism — the receiver `x` bare-uses as a Borrow-flavored coord, and the target-side rules gate any coercion. There is no separate method-dispatch path.

| Source | Meaning |
|---|---|
| `x.foo()` | Bare-use x, dispatch to `foo`. Identical to `foo(x)`. |
| `x&.foo()` | Same (explicit spelling of bare-use as borrow). |
| `x^.foo()` | Move-call: `x` moves into `foo`. Identical to `foo(x^)`. |

The `x.foo()` and `foo(x)` equivalence generalizes to multi-arg dispatch: `foo(ship, rocket)` looks in both Ship's and Rocket's namespaces (see `vcoord-handoff.md` § Dispatch model). No parameter is "special."

### Cloneability

Two distinct notions of "clone" exist under the refined model, and it's important not to conflate them:

- **`implicit_clone(&T) T`** — the auto-firing intrinsic. Built-in only for **primitives** (Int/Bool/Float/Void/Never), backed by `__copy_prim`. Fires *only* at target-side coercion sites where source shape is `Borrow + primitive` and the target wants `Own + primitive`. Not user-overridable, not user-callable for non-primitives.
- **User-space `clone(&T) T`** — a regular function the user can define for any type and call directly (`clone(&x)`). Never auto-fires from bare-use. Ships built-in for class types (via `__rc_alias`), optionally hand-written for struct types.

| Kind | `implicit_clone(&T) T` (auto) | User-callable `clone(&T) T` | Notes |
|---|---|---|---|
| Primitive | Compiler-provided via `__copy_prim` (scalar copy) | Same function; delegates to `__copy_prim` | Auto-fires at Own-primitive target. |
| `class` type (formerly `share`) | None | Compiler-auto-derived via `__rc_alias` (refcount bump) | Bare-use of class-type at Share-typed target auto-aliases via the target-side rule; direct `clone(&x)` also works. |
| `struct` type (owned/unique) | None | User-provided if the author wants a clone semantic | No auto-firing. Bare-use of struct at Own-non-primitive target = compile error. |

```vale
// Primitive built-ins (in builtins/implicit_clone.vale):
func implicit_clone(x &int) int { return __copy_prim(x); }
func implicit_clone(x &bool) bool { return __copy_prim(x); }
// etc. — also exposed as `clone(&int) int` for direct user calls.

// Class auto-derived (synthesized at typing-pass time per `class` decl):
class Ship { fuel int; }
// Compiler synthesizes:
//   func clone(x &Ship) Ship { return __rc_alias(x); }
// User cannot override this — it's load-bearing for the kind-class invariant.

// Struct user-provided (optional):
struct Vec3 { x int; y int; z int; }
func clone(v &Vec3) Vec3 { return Vec3(v.x, v.y, v.z); }
```

**Generic functions do not need a `where func clone(&T) T` bound to bare-use `T`.** Bare-use produces Borrow regardless of `T`. A clone bound is only required when the function body wants to *own* a fresh `T` (e.g., store it in a returned struct) and the target-side auto-coercions don't cover the case.

If a bare-use ends up at an `Own + non-primitive` target — a struct/interface/array target — the typing pass produces a typed compile error:

> `MustExplicitlyMove`: bare-use of `x` produces a borrow, but the target expects owning `T`. Write `x^` to move, or call `clone(&x)` explicitly.

### Class types: target-side RC-aliasing

For `class Ship { fuel int; }`:

```vale
let f = Ship(42);   // creates Ship, refcount = 1 (f is Share Ship)
let g = f;          // bare-use produces Borrow+share-kind; target `g` (Share Ship) auto-aliases via __rc_alias → refcount = 2
let h = f^;         // move; f dead; refcount unchanged (h owns f's slot)
let i = f&;         // bare-use spelled explicitly; i is Borrow+share-kind
```

Bare-use of a class-typed local is always cheap when landing at a Share target: refcount bump, no body copy. The RC-alias fires at the *target* boundary via the target-side coercion, not at the source. Runtime semantics are equivalent to the old "share" types; the desugar path is cleaner.

### Struct types: bare-use borrows; owning requires `^` or explicit `clone`

For `struct Vec3 { x int; y int; z int; }`:

```vale
let a = Vec3(1, 2, 3);      // a is Own Vec3
let b = a;                  // bare-use produces Borrow Vec3; target `b` is Own Vec3 → MustExplicitlyMove ERROR
let b = a^;                 // move; a dead; b owns
let b = clone(&a);          // explicit clone (requires user-provided clone(&Vec3) Vec3)
let d = a&;                 // bare-use as borrow (identical to bare `a` — d is Borrow Vec3)
```

Bare-use of a struct at an Own-non-primitive target never silently clones — the user must write `^` (move) or an explicit `clone(&a)` call. This is a stricter semantic than the earlier "bare = clone" framing: it forces move-vs-clone intent to be visible at every non-primitive owning binding.

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

New: bare-use produces a Borrow (never consumes). Only `x^` consumes. Whether the borrow's ultimate materialization involves a runtime clone, an RC-alias, or nothing at all depends on the target-side coercion table above.

```vale
// Today:
let y = x;       // moves x; x is dead.
foo(x);          // moves x into foo; x is dead.
return x;        // moves x out as return value.

// New (T = primitive):
let y = x;       // bare-use produces Borrow; target `y` is Own primitive → implicit_clone; x still valid.
let y = x^;      // moves x; x dead, y owns.
foo(x);          // bare-use produces Borrow; foo's param decides (auto-clone if Own+primitive, alias if Share, pass-through if Borrow).
return x;        // bare-use produces Borrow; return-type-side coercion (or ERROR if Own non-primitive).
return x^;       // moves x out; x dead.

// New (T = class):
let y = x;       // bare-use → Borrow+share-kind; target `y` (Share T) → auto-alias via __rc_alias; refcount bump.
let y = x^;      // moves x; x dead, y owns the share ref.

// New (T = struct, non-primitive owned):
let y = x;       // ERROR: MustExplicitlyMove — bare-use produces Borrow, but target `y` is Own struct.
let y = x^;      // moves x; x dead, y owns.
let y = clone(&x); // explicit clone (requires user-provided clone).
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

### Generic function bounds don't proliferate for bare-use

Because bare-use produces a Borrow uniformly (regardless of `T`'s kind class), generic code that only *uses* `T` — reading fields, passing to another function that takes `&T`, storing in a Borrow-typed slot — does **not** need a `where func clone(&T) T` bound.

```vale
// Reads a field on a borrow → no clone bound needed:
func print_fuel<T>(s &T) where func fuel_of(&T) int {
  print(fuel_of(s));   // bare-use of s produces Borrow (identity); fuel_of takes &T; pass-through.
}
```

A `where func clone(&T) T` bound *is* needed only when the body wants to *own* a fresh `T` and the source can't provide one via `^`:

```vale
// Needs to return a fresh owning T → user must supply clone (or callers must pass owning):
func first_two<T>(a &T, b &T) (T, T) where func clone(&T) T {
  return (clone(a), clone(b));   // explicit clone calls
}
```

Class instantiations satisfy `where func clone(&T) T` for free (auto-derived clone). Struct instantiations require the struct to have a user-provided clone. But the vast majority of generic code — collections that only borrow their elements, callbacks, dispatch — is bound-free under the refined model.

### TSUGAR markers split into two classes

The ~310 `// TSUGAR:` markers in the current tree fall into:

- **`__copy_prim(...)` source wraps** — ~150 markers, **removable**. The typing pass auto-inserts
  CopyPrim for bare-use of primitives; the source wrap becomes redundant.
- **`&` / `__copy_prim` on function-arg/member-access** (e.g. `m.has(&N)`, `&true bork &false`,
  `__copy_prim(m.hp)`) — ~160 markers, **stay** (they're explicit borrows / member-clones).

Roughly half the markers go away; the other half become natural Vale syntax.

## What this resolves from the cut's review

The kind-mutability cut left several band-aids tracked as Q1–Q11 in the post-cut review. This model resolves these as follows (many have since landed):

| Question | Resolution under this model | Status |
|---|---|---|
| Q1: LocalLoadH absorbs borrow→value | Inverted. LocalLoadH(target=Borrow) on primitive produces `MutableBorrowH+InlineH+prim`. CopyPrimH does the borrow→value conversion. | **LANDED** (Q1 borrow-shape arc, see vcoord-handoff.md § "Mission — Q1 LocalLoadH / borrow-shape honest-mode arc" — DONE) |
| Q5: 11 `inline_primitive_uniform_coord_h` call sites | Density collapses to ~1–2 sites (canonical primitive-literal shape rule). Load sites use `target_ownership` directly. | LANDED alongside Q1. |
| Q6/Q7: "OwnH-only-for-primitive" testvm asserts | Disappear. LocalLoadH/MemberLoadH never produce OwnH for primitives; CopyPrim does. | LANDED alongside Q1. |
| Q9: `__copy_prim` panic for non-primitive | Retitled `implicit_clone` internally. Trigger sites move from user source to typing-pass auto-insertion at `convert_helper.rs` / target-side coercion table. | Partially landed via `implicit_clone(&T) T` intrinsic wiring in CHECKPOINT 19. |
| Q2/Q3/Q10/Q11 | Orthogonal pre-cut leftovers (Share+primitive producers, dead consumer arms, missing constructor assert). | Mostly LANDED via CHECKPOINT 19 sharedness cleanup + CoordT::new / CoordI::new / CoordH::new invariants. |
| Q4 | `share` keyword retires in favor of `class`. Parser disambiguation hack removable. | DEFERRED — see Phase H. |

## Out of scope (deliberately)

- **Auto-firing `clone` for non-primitives at call sites.** The refined model does NOT auto-insert `clone(&x)` when a function param is `Own T` and the caller passed bare `x` for a non-primitive `T`. That's a `MustExplicitlyMove` compile error; the user writes `x^` or `clone(&x)`. The two exceptions — `Borrow + primitive` → Own via `implicit_clone`, `Borrow + share-kind` → Share via `__rc_alias` — are the ONLY auto-fired coercions. Everything else requires explicit user action.
- **`Copy` marker trait (Rust-style).** This model treats primitives specially via `implicit_clone` and class types specially via `__rc_alias`; struct types have no auto-clone. There is no distinct "trivially copyable" trait, and no way for a user-defined struct to opt into auto-firing.
- **`Clone` trait shorthand** (e.g. `<T: Clone>` desugaring to `where func clone(&T) T`). Not in this arc; bare `where func clone(&T) T` is fine.
- **Drop semantics.** Unchanged. `clone(&T) T` does not interact with `drop(T) void`. A type may be cloneable but not droppable, or vice versa.
- **Last-use auto-move (NLL-style).** Bare-use always borrows. The author writes `^` to move. Future arc could promote a bare-use that would otherwise land at an Own non-primitive target and IS the syntactically last use into an implicit move, eliminating the `MustExplicitlyMove` error for the common "return x" case. Not in this design.

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

For *this* design we don't depend on `*ref = value` for primitives — bare-use borrows (with
`implicit_clone` firing at Own-primitive targets), `&x` is the explicit spelling of the same
borrow, `^x` moves. Mutating a remote primitive through a borrow isn't part of the surface
semantics described above. If we later add `*x = ...` as a language feature for
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

### Phase 0 (LANDED)

Orthogonal surgical fixes from the cut's Q2/Q3/Q4/Q9/Q10/Q11 review. All landed via the CHECKPOINT 16 → CHECKPOINT 22 arc plus incremental follow-ups. The `CoordT::new()`/`CoordI::new()`/`CoordH::new()` primitive-invariant asserts are in place, the four Share+primitive producer sites have been fixed, and the `Sharedness` naming is unified. `share`-keyword disambiguation (Q4) landed as part of the sharedness cleanup — the templex parser reads `share` in struct/interface header position directly as `SharednessP`. Only Q4's canonical `class`-keyword rename remains (Phase H).

### The current mission's Phase 2 supersedes Phases A/B/C below

The vcoord-handoff.md active Mission (§ "Overload resolution & dispatch model redesign") absorbs Phases A, B, and C of this doc into a single combined arc: the uniform bare-use → Borrow materialization, the target-side auto-coercion table, and the `Borrow + share-kind` vs `Share T` type-system distinction all land together. See vcoord-handoff.md § "Practical scope of work" for the current landing plan. Phases D onward (postfix syntax parser, method-receiver `&./^.`, lambda capture lists, `class`-keyword rename, `@T` retirement, TSUGAR sweep) are still ahead and independent of Phase 2.

### Phases A / B / C (SUPERSEDED — covered by vcoord-handoff.md Phase 2)

The original Phases A (target-side coercion in `convert_helper.rs`), A.5 (class-clone auto-derivation), B (bare-use desugar), and C (move-tracker change) are absorbed into the active Mission's Phase 2 arc — landing together with the type-system `Borrow + share-kind` vs `Share T` distinction, uniform bare-use materialization, and target-side `convert()` rewrites. See vcoord-handoff.md § "Practical scope of work" for the concrete landing plan.

### Phases D–J (still ahead, mostly independent of Phase 2)

| Phase | What | Key files |
|---|---|---|
| D | Postfix `^` / `&` syntax: parser changes. Add `x^` and `x&` as expression suffixes; preserve prefix-`&` short-term during transition, then retire. | `parsing/expression_parser.rs`, `postparsing/expression_scout.rs` |
| D.5 | `x&.foo()` / `x^.foo()` method-call receiver syntax. Lex/parse `&.` and `^.` as combined receiver-borrow / receiver-move tokens. | `parsing/expression_parser.rs` |
| E | (LANDED) Q1 H-IR arc — `LocalLoadH(target=Borrow)` on primitive honestly produces `MutableBorrowH+YonderH+prim`, CopyPrimH does the borrow→value conversion. | — |
| F | Backend: widen `primitives.h` asserts to accept primitive borrows; construct primitive borrow Refs ad-hoc at `metal_lowerer`. | `Backend/src/region/common/primitives.h`, `FrontendRust/src/backend_ffi/metal_lowerer.rs` |
| G | Lambda capture list parser + typing-pass capture-mode handling for `[x]` / `[x&]` / `[x^]`. Also depends on the share-lambda / own-lambda syntactic split (see vcoord-handoff.md § "Future direction: split lambdas"). | `parsing/expression_parser.rs`, `postparsing/expression_scout.rs`, `typing/expression/expression_compiler.rs` (closure path) |
| H | `share` → `class` keyword rename: lexer/parser. Intermediate accepts both `struct Foo share` and `struct Foo class`; eventually only the standalone `class Foo { ... }` form. Test-program sweep. | `parsing/parser.rs`, all `*.vale` files using `share` |
| I | `@T` syntax retires. Remove from parser; sweep test programs that still use it. | `parsing/templex_parser.rs`, test programs |
| J | TSUGAR sweep: remove `__copy_prim(...)` wraps in builtin Vale + test programs. Verify by running suite. Also delete source-level `__copy_prim` syntax (CopyPrimSE, IRulexSR::CopyPrim, scout handler, typing-pass syntax branch). Keep CopyPrimTE/IE/H/Backend::CopyPrim for the operation. | `FrontendRust/src/builtins/`, `FrontendRust/src/tests/`, `FrontendRust/src/integration_tests/`, postparsing/ |

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

These should be resolved before the active Mission's Phase 2 lands (the uniform bare-use materialization + target-side coercion arc that supersedes the original Phase A/B/C).
