# Real `Vec` interop — forward plan

Goal: typecheck real `Vec<int, Global>` against a live rustc — `Vec<int, Global>.new()`,
`v.push(42)`, `v.len()`, and a scope-end drop. Nothing runs yet; this is typing-pass only.

Companion to `investigations/lazy_imports_handoff.md` (the single source of truth for the lazy-imports
thread). This doc scopes the *forward* Slice 3/4 work and pulls together the design trail scattered
across convos 79, 84, 85, 88, 98, 99.

## Baselines and how to run

- Clean rebuild is mandatory after the folder move: `env!("CARGO_MANIFEST_DIR")` is baked at compile
  time, so a stale artifact points fixture loads at the pre-move project path and every disk-reading
  test fails. `cargo clean` first, then:
  - default `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib` = **690 / 0 / 69**
  - interop `cargo test --manifest-path ./FrontendRust/Cargo.toml --lib --features rust_interop` = **746 / 0 / 69**
- The interop suite is invisible to CI and the fire-commit gate; run it by hand. It needs
  `rustup component add rustc-dev --toolchain nightly-2025-12-09`.
- `FrontendRust/src/typing/` is human-edit-only (its `.claude/CLAUDE.md`). Every typing-pass change
  below needs an explicit "ok proceed"; the fixture/corpus files under `rust_interop/` count.

## Scoping — what actually blocks real `Vec`

Only some of the "known interop gaps" sit on the path to typechecking basic `Vec`. The onion arc that
has since landed cleared the surrounding foundations (generic substitution through the four reference
wraps, argument types reaching the call-site solve, `UpcastTE`, peeled-namespace method lookup) and
zeroed the old 127-failure baseline.

| Item | On the basic-`Vec` path? | Why |
| --- | --- | --- |
| `&self` / `&mut self` methods | **DONE** (slice 1) | `Vec.push`/`len` are borrow-receiver methods. The onion arc landed the core borrow *resolution* (`substitute_templatas_in_kind` ref-wrap arms, the `convert`/`is_type_convertible` borrow arms), but the interop *synthesizer* couldn't emit a borrow parameter at all — a `&self` lowered to a settled `Kind(BorrowRef(Struct))` that `vale_type_name` can't name. Fixed by a structural `ValeSigType::Borrow` variant + the `TyKind::Ref` lowering arm + emitting a `BorrowRefSR` in the parameter's @PFVSZ outer-ref bucket (arg binds to the value rune, borrow concludes the full rune). Proven by `calls_a_borrow_self_method_on_a_local`. |
| `Vec::new` associated-fn arity | **DONE** (slice 2), no core change | `new` lives in `impl<T> Vec<T, Global>` — it ranges over one generic (`T`); `Global` is concrete in its return `Vec<T, Global>`. The `overload_resolver.rs:255` `own_rune_count = identifying(1) − receiving(N)` underflow only fires when the call **over-specifies** the type application (`Vec<int, Global>.new()` — two args to a one-generic function). The intended call supplies only `T` — `Vec.new<int>()` **or** `Vec<int>.new()` — so `receiving`/explicit args = 1, no underflow. `Global` is written only when naming `Vec` **as a type** (no default-generic-param support), never at the constructor call. Both call spellings resolve today; no `overload_resolver.rs` edit needed. |
| `usize` for `len()` | **DONE** (slice 3), as a Vale primitive | `usize` is now a first-class Vale primitive `KindT::USize(USizeT)`, alongside `int`/`bool`/`float` — a distinct kind (never unified with `int`/`i64`), keyword `usize`, registered in the builtins store, `is_primitive` true so it needs no drop. Rust `usize` lowers to it; the other unsigned widths (`u8`..`u64`) still decline. |
| Borrow-read on locals (borrow → owned `self`) | **No** | Only bites *consuming* (by-value `self`) methods called on a local, where a local read is `BorrowRef` but the method wants owned. `Vec`'s core methods are borrow receivers, so they match a local directly. Vale intends `^v` for a genuine move. This looked central only because the fixtures used by-value `self` as the `&self` workaround — which the row above retires. |
| `is_primitive` rename / export-extern boundary | **No** | The Vale4 *export* front line, not on the path to typechecking a `Vec` method call. Tracked in the onion handoff. |

So the forward path is three slices: reprobe `&self` and retire the workaround, then `Vec::new` arity,
then `usize`. The last two are the genuine unbuilt mechanics; the first is likely already done.

## Design trail (where each decision was made)

- **`Vec::new` arity.** Design home is convo-88 (`Vec<int>.new()` dot syntax, `Vec<int, Global>`
  keeps its honest arity, outbound `GenericArgs` reconstruction is tier-2 codegen and out of scope for
  typing). convo-98/99 found the underflow empirically and named the sound fix: the extra container arg
  is not a function rune — **unify the container type `Vec<int, Global>` against `new`'s return type
  `Vec<T, Global>`**, binding `T = int` and checking `Global = Global`. The rejected hack is erw's
  `@ETASTZ`: truncate the extra arg and assume it matches (a flagged soundness hole; rejects nothing).
- **`&self` borrow receivers.** The onion-arc convos 79/84/85 track the borrow read-out and
  `is_type_convertible` borrow arms; `convert_helper.rs:86/107/115` now carries them, and
  `substitute_templatas_in_kind` handles the ref wraps. The fixture comment predates that landing.
- **`usize` / opaque foreign scalars.** The handoff's "Future — opaque foreign scalars" section:
  represent `u64`/`f64`/`usize` as opaque `rust`-namespaced nominal types (generalizing the
  imported-struct path) so distinct names give distinct identity.

## RFIGA slices

Interop cases are integration-style black boxes: a case is Vale source plus assertions on what
typechecks and which `fn_sig`/resolve queries the oracle saw. That is the public interface throughout.

1. **Borrow-receiver (`&self`/`&mut self`) methods. DONE — green.**
   - Turned out to be real synthesis work, not a verification: the interop declaration layer had no
     way to emit a borrow parameter. Landed a structural `ValeSigType::Borrow(&ValeSigType)`
     (`oracle.rs`), a `TyKind::Ref` arm in `lower_sig_ty` (`tyctxt_oracle.rs`), the @PFVSZ param split
     that puts a `BorrowRefSR` in the parameter's outer-ref bucket with the argument binding to the
     *value* rune (`declarations.rs`), and the `SigPosition::Borrow` shape (`logging_oracle.rs`). All in
     `rust_interop`; no core typing-pass edits. Proven by `calls_a_borrow_self_method_on_a_local`
     (`c = make_counter(); c.peek()` on a local). Suites: default 690/0/69, interop 747/0/69.
   - Follow-up (optional, not done): the existing by-value fixture methods (`get`/`doubled`/…) were left
     as consuming methods; switching any to `&self` is a per-case semantics choice, not required.
2. **`Vec::new` associated-fn arity with a fixed impl param. DONE — green, no core change.**
   - Added a miniature two-param fixture whose impl fixes the second param (`impl<T> Boxed<T, Fixed>`
     with a no-arg `fn new() -> Boxed<T, Fixed>`, plus `Fixed` and a `boxed_ignore` consumer so no
     scope-end drop on a generic type is needed — that gap is separate and unfixed), and **two** cases,
     one per call spelling: `Boxed.new<int>()` and `Boxed<int>.new()`. Both resolve. The earlier belief
     that this needed a core `overload_resolver.rs` reconciliation was wrong — the underflow was purely
     the old test over-specifying `Vec<int, Global>.new()`. All in `rust_interop`. Suites: default
     690/0/69, interop 749/0/69 (+2).
   - Note: this also incidentally exercises **two-parameter generic-type import** (`Boxed<T, A>`), which
     the handoff had flagged as untested; it works.
3. **`usize` for `len()`. DONE — green.** Added as a **Vale primitive** (architect's call), not an opaque
   `rust` nominal type: a new `KindT::USize(USizeT)` variant in `types.rs`, a `usize` keyword, a builtins
   registration in `compiler.rs`, arms at the 8 exhaustive `KindT` match sites (mirroring `float`:
   `get_placeholders_in_kind`, `is_descendant_kind`, the export check, both `is_primitive`s, the
   humanizer, `substitute_templatas_in_kind`, the destructor's discard arm, and `test/traverse.rs`), and
   the `rust_interop` lowering (`lower_ty` maps `TyKind::Uint(Usize)` → `USize`; `vale_type_name` names
   it). Proven by `imports_usize_as_a_primitive` (`some_size() -> usize`, `consume_usize(usize) -> i32`).
   Suites: default 690/0/69, interop 750/0/69 (+1). This was a **core typing-pass** change (authorized).
   - Deviations from `int` (see report): no `usize` literal syntax and no arithmetic/comparison operators
     — `usize` is produce-and-pass-only from Rust. Other unsigned widths still decline.
4. **Real `std::vec::Vec` end to end. DONE — green, no new code needed.** Three cases import the real
   `Vec` and `Global` from the actual `alloc` crate (`import rust.alloc.vec.Vec;`,
   `import rust.alloc.alloc.Global;`) and exercise, against live rustc:
   `real-vec-new` (`v = Vec.new<int>();` bound to a local with a scope-end drop), `real-vec-push`
   (`v.push(42)` — a `&mut self` method), and `real-vec-len` (`v.len()` — a `&self` method returning the
   `usize` primitive, passed to `consume_usize`). All passed **on the first run** — slices 1–3 plus the
   already-working generic scope-end drop compose with no further changes. Only the called methods
   (`new`/`push`/`len`) synthesize; the rest of `Vec`'s ~150 methods stay id-only (the laziness payoff).
   Suites: default 690/0/69, interop 754/0/69 (+3), deterministic. Import path note: the crate-qualified
   path names the item's **canonical** crate (`alloc`), not the `std` re-export.

## Explicitly out of scope (not `Vec` blockers)

- Borrow → owned `self` for consuming methods on a local (use `^v`; intended Vale semantics).
- The `is_primitive` rename / export-extern boundary (Vale4 export front line; onion handoff owns it).

Pull either in only if a concrete `Vec` case in slice 4 actually needs it.
