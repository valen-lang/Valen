# Mirroring Rust's borrowing into imported functions — plan

## Goal

When `rust_interop` imports a Rust function, faithfully translate Rust's borrow/lifetime/mutation
information from the rustc signature into the generated postparsed (`FunctionS`) function, so Vale's
group borrow checker enforces at Vale call sites the same aliasing/mutation rules Rust enforces.

Target examples:
- `fn foo(ship: &mut Ship)` → `func foo<s'>(ship &Ship in s) mut(s)`
- `fn get<T>(self: &Vec<T>, index: usize) -> &T` → `func get<T, v'>(self &Vec<T> in v, index usize) &T in v...`

## What Rust exposes vs. what Vale can express

Two independent facts decide each Rust borrow feature's phase: whether rust_interop can **extract** it
from rustc, and whether Vale's group model can **represent+check** it.

### Extraction (rustc oracle, `src/typing/rust_interop/tyctxt_oracle.rs`)

Every borrow fact is currently discarded. Both reference-lowering arms — `lower_sig_ty`:486 and
`lower_ty`:538 — match `TyKind::Ref(_, inner, _)`, dropping field 0 (`Region`) and field 2
(`Mutability`) and keeping only the referent. `ValeSigType::Borrow` (`oracle.rs`:75) has no mut/region
slot; `ValeSig` (`oracle.rs`:117) has no safety slot.

- **Reference mutability (`&T` vs `&mut T`)** — FREE: already-matched tuple field at `lower_sig_ty`:486,
  just underscored. Only blocked by the missing IR slot.
- **`unsafe fn` / safety** — FREE to read (`sig.safety`, an unread public field on the `FnSig` bound at
  `fn_sig`:635), but deliberately **not mirrored** (see the mapping): an imported Rust call already
  crosses an opted-into FFI boundary.
- **Which input region a returned `&T` is tied to** — present in the fn's binder but never decoded
  (`fn_sig` does `instantiate_identity().skip_binder()`, dropping late-bound lifetimes). For the common
  cases this is derivable from **Rust's elision rules** without decoding the binder: exactly-one-reference
  input, or a `&self`/`&mut self` method, ties the output reference to that input. Explicit/multiple
  named lifetimes need real binder decoding.
- **Outlives / where-clause region bounds (`'a: 'b`, `T: 'a`)** — `explicit_predicates_of` is one query
  away but nothing consumes it; deliberately unread today.
- **Interior mutability (`Cell`/`RefCell`)** — NOT a signature fact; rustc does not surface it through
  `fn_sig`. A `&self` method that mutates through interior mutability is indistinguishable from a pure
  `&self` method. Unextractable in principle.

### Representation + checking (Vale group borrow checker)

Groups never reach the typed AHT (`@BCHATZ`): `BorrowRefT` carries no group; the checker reconstructs
group info by reading the postparse `FunctionS`/`ParameterS` at its seam
(`function_compiler_core.rs`, after `add_function`). So "mirroring into the AHT" means **mirroring into
the generated `FunctionS`'s postparse annotations** — the AHT itself stays group-free by design.

The postparse slots all already exist at HEAD:
- `<g'>` region param → `RegionGenericParameterTypeS` (`postparsing/ast.rs`:507).
- `in g` on a borrow → `BorrowRefST.region: RegionS` (`postparsing/rules/types.rs`:178), variant
  `RegionS::Group(GroupS::Rune(...))`.
- `mut(g)` → `FunctionS.effects: &[EffectS]` (`postparsing/ast.rs`:565), `EffectS::Mut(GroupS)`.

**Vale has no outlives, subtyping, or variance, and never will** — a group is "an identity, not an
extent" (`path-to-borrowing.md`; exp-2 `borrowing-design.md` "Out of scope"). Rust's outlives lattice
(`'a: 'b`) is therefore **unmappable by design**, not merely unbuilt.

## The long-term mapping (the faithful target)

**Governing principle: a Rust lifetime lowers to a descendant (`...`) group in Vale.** A parameter that
*introduces* a lifetime declares a fresh rune `<a'>` and sits directly `in a`; every *reuse* of that
lifetime — on the return, or nested inside a type — lowers to the conservative descendant form `in a...`.
We never derive an element/member-precise group (`in a[]`, `in a.field`) from a lifetime, because Rust's
signature says the borrow comes *from* the region, not *what part* of it — `...` is sound in all cases
(any churn of `a` invalidates it) and is the permanent policy, not a stepping stone.

| Rust | Generated Vale | Notes |
|---|---|---|
| `&T` param | `&T in g` (fresh rune per param) | disjoint by default = Rust elision default |
| `&mut T` param | `&T in g` + `mut(g)` | |
| `&self` / `&mut self` | same; `self` is param 0 | |
| return `&'a T` tied to one param P | `&T in <P's rune>...` | `...` so any churn of P invalidates it |
| return `&'a T` tied to several params | `&T in (p1|p2)...` (union) | needs union groups |
| nested `&'b` inside a param type, reused on return | `&T in <b>...` | needs deep binder decoding |
| explicit multiple named lifetimes | resolve which input(s) the return ties to | needs binder decoding |
| `unsafe fn` | — | **dropped**: an imported Rust call already crosses an opted-into FFI boundary |
| `'a: 'b` outlives, `T: 'a` | — | **unmappable**; Vale has no outlives |
| interior mutability | — | **invisible** to the signature |

Soundness note on returns: the descendant form is load-bearing. Vale's churn rule invalidates a group's
*descendants* but leaves references to the group *itself* alive (exp-2 `borrowing-design.md`); a returned
Rust borrow is invalidated by any `&mut` reborrow of its source, so only `in P...` matches Rust —
whole-group `in P` would be unsound.

Note: shared input lifetimes do **not** imply a shared Vale group. `fn foo<'a>(x: &'a mut A, y: &'a mut B)`
is two distinct objects that merely both outlive `'a`; per-parameter distinct groups stay faithful even
under shared lifetimes. Binder decoding buys us only *return-tie resolution*, never parameter aliasing.

**Until that decoding exists, a lifetime shared across two or more parameters is a compile error, not a
guess.** We refuse to import such a Rust function rather than assume distinct groups. It is a legitimate
signature we will support later; for now the honest move is to reject it loudly. This holds from Phase 1
on — Phase 1 detects the shared-across-parameters shape and declines; Phase 3's binder decoding is what
lifts the restriction.

## Phase 1 — doable today, with the incomplete checker (exp-4 HEAD)

**Scope: parameter mutability + per-parameter disjoint groups.** Delivers the `fn foo(ship: &mut Ship)`
→ `func foo<s'>(ship &Ship in s) mut(s)` example, fully checked by HEAD's borrow checker.

HEAD already supports exactly this shape: `<g'>` region params (ceremonial `ITemplataT::Group`,
inert in typing), `in g` with single-name Rune groups, and `mut(g)`. The checker enforces rung-1
joint-argument disjointness (aliasing into distinct mutated groups; borrow-into-moved-argument) and
rung-2 use-after-churn on runtime-sized-array elements — so these annotations are genuinely checked at
Vale call sites, not merely recorded.

The mapping:
- Each reference parameter (including `&self`) gets its **own** fresh region rune `<g'>` and `in g`.
  Own-rune-per-param is the faithful mirror of Rust's elision default (each param an independent
  lifetime) and is conservative: a call passing the same object into a `&mut` param and another param
  is rejected, exactly as Rust rejects it.
- A `&mut T` param additionally contributes `mut(g)` to the function's effects.

Work items (all in `src/typing/rust_interop/`, AI-editable):
1. Grow `ValeSigType::Borrow` (`oracle.rs`:75) to carry a `mut` flag; stop discarding `Mutability` at
   `tyctxt_oracle.rs`:486 (and `lower_ty`:538).
2. In `synthesize_extern_function` (`declarations.rs`): mint one region generic param per reference
   parameter (extend the loop at :90–102), emit the param `tyype` as
   `ITypeST::BorrowRef(BorrowRefST { inner, region: Group(Rune(g)) })` instead of the bare
   `ITypeST::Rune` at :191, and populate `effects` at :237 with `EffectS::Mut(Rune(g))` for each
   `&mut` param. Mirror into the abstract-method twin (`synthesize_abstract_interface_method`,
   :601–663/692).

Out of Phase 1 (not expressible/checkable at HEAD): any returned borrow group (HEAD does not do
return-group flow), and path groups `in g.items` / `in g[]` / `in g...` (the scout panics
`POSTPARSER_GROUP_MEMBER_ELEMENT_UNION_NOT_YET_IMPLEMENTED`, `templex_scout.rs`:838).

Risk to manage: this turns on borrow checking of Vale programs that call imported Rust functions for
the first time. Validate against the interop corpus for false positives from checker incompleteness
before committing.

## Phase 2 — doable once exp-2-wipbx lands its borrow-checker rewrite

exp-2's uncommitted rewrite replaces the checker with a two-phase `groupify_function` +
`check_usages`, and adds: path groups `in g.items` / `in g[]` / `in g...` (parse+scout+check, param
and return position), **returning a borrow tied to an input region** (groupify's `call_result_group` +
`substitute_groups` map the callee return group onto the caller's argument groups), use-after-churn
through returned references and member/element paths, and groups read at every type depth.

**Scope: returned references tied to an input region, for the elision-derivable cases.** Delivers the
`fn get(self: &Vec<T>, i) -> &T` → `func get<v'>(self &Vec<T> in v, i usize) &T in v...` example.

The mapping:
- When Rust's elision resolves the output reference's region to a single input P — i.e. there is exactly
  one reference input, or the method is `&self`/`&mut self` — emit the return type as
  `&Ret in <P's rune>...` (descendant). exp-2's return-group substitution + use-after-churn then invalidate
  the returned reference whenever P is churned, mirroring Rust.
- This needs no binder decoding: it reads off Rust's own elision rules, which are a cheap structural
  property of the signature.

Work items:
1. Add elision resolution to the oracle: for a signature with a reference return, determine the single
   input it ties to (one-reference-input, or `&self`). Extend `ValeSig`/`ValeSigType::Borrow` (or a
   return-side field) to carry "return borrows from input i."
2. In `synthesize_extern_function`, emit the return `tyype` as a `BorrowRefST` with
   `region: Group(<descendant of P's rune>)` when the elision fact is present.

Out of Phase 2: returns tied to several inputs (union), explicit/multiple named lifetimes (binder
decoding), and everything exp-2 itself leaves out (effect *checking*, `not(mut)`, `Box`/`Variant`
child groups, generic `Vec<T>` element groups, borrow-typed fields, shadowing).

## Phase 3 — full lifetime decoding

**Scope: decode Rust's fn binder to resolve every return-tie elision can't, including multi-source
(union) and nested lifetimes.** This is the general form of Phase 2's return-tying: instead of reading
off elision, walk the signature's late-bound region vars and correlate the return region(s) to the
parameter(s) that introduce them.

Three included capabilities:
- **Explicit / multiple named lifetimes** — `fn pick<'a, 'b>(x: &'a Ship, y: &'b Ship) -> &'a Ship`
  → `func pick<a', b'>(x &Ship in a, y &Ship in b) &Ship in a...`. Decode which input the return region
  equals when elision doesn't apply. New oracle plumbing over the binder's region vars.
- **Multi-source (union) returns** — `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
  → `func longest<a', b'>(x &str in a, y &str in b) &str in (a|b)...`. Blocked until exp-2's union
  groups are parser-produced (the `Union` variant exists but is unbuilt).
- **Nested-reference lifetimes** — `fn inner<'a, 'b>(x: &'a Opt<&'b Ship>) -> &'b Ship`
  → `func inner<a', b'>(x &Opt<&Ship in b> in a) &Ship in b...`. exp-2's `make_kind_g` already carries
  groups at every type depth; the work is decoding nested `TyKind::Ref` regions in the binder and tying
  the return to the inner one.

Explicitly **not** pursued (decided):
- **`unsafe fn`** — dropped. An imported Rust call already crosses an FFI boundary the programmer opted
  into; `unsafe` gets no Vale representation.
- **Element/member-precise return groups** (`in P[]`, `in P.field`) — not pursued. Every Rust lifetime
  lowers to the conservative descendant `in P...` per the governing principle; the precise forms aren't
  derivable from a lifetime anyway.

Fundamental limits (documented so nobody re-derives them as blockers):
- **Outlives / subtyping / variance** (`'a: 'b`, `T: 'a`) — Vale has no such concept and never will.
- **Interior mutability** — not a signature fact; unextractable from `fn_sig`.

## References

- Extraction: `src/typing/rust_interop/{tyctxt_oracle,oracle,importer,declarations}.rs`.
- Representation: `src/postparsing/{ast.rs, rules/types.rs, rules/templex_scout.rs}`,
  `src/parsing/ast/templex.rs`.
- Checker + design: `src/typing/borrow_checker/`, seam at
  `src/typing/function/function_compiler_core.rs`; design authority
  `src/typing/docs/architecture/borrowing-design.md`; roadmap `docs/plans/path-to-borrowing.md`.
- exp-2 rewrite (pending): the same paths in `/Volumes/V/Valen/exp-2-wipbx`.
