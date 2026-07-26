# Rust interop — the pivot to synthesized declarations

Written 2026-07-25, at the point where the oracle-answers-per-call design was abandoned. Revised
the same day after two sibling-tree surveys, a `rustc_resolve` survey, and direct verification
against this tree, so none of it has to be rediscovered.

Companion to `rust-interop-frontend-plan.md` (now largely superseded — see §11) and
`rust-interop-callout-map.md` (its call-site inventory stands; its prescribed *seam* does not). The
architecture doc (`vale-rust-interop-architecture.md`) is
**authoritative** wherever this document and it disagree; several items below are corrections
bringing implementation notes back in line with it.

---

## 1. What was wrong with the shipped design

The seam built a finished `PrototypeT` at environment-build time, from
`oracle.fn_sig(item, &[], interner)` — note the **empty args**. That is structurally incapable of
representing a generic Rust function: `fn pick<A, B>(a: A, b: B) -> A` has no single signature,
only one per instantiation. It is also why `fn_sig`'s `args` parameter — the entire @EarlyBinder
discipline — had never once been passed a non-empty value.

Two arcana in this repo already forbid it:

- **@ECSIIOSZ** — every call site is lowered into its own self-contained vector of solver rules;
  the typing pass spins up a fresh solver per call site.
- **@BDPFWDZ** — each solve reaches into the calling env for what it needs *at solve time*, rather
  than depending on something pre-pushed into a shared store.

Pre-pushing a resolved prototype into a store is exactly what those forbid.

---

## 2. The two sibling implementations

Both were surveyed read-only. **They converge**, and neither invents new machinery.

| | `/Volumes/V/ValeRustInterop` | `/Volumes/V/RustInteropReiImpl` |
|---|---|---|
| language | Scala `Frontend/` (its Rust port is stale) | **both** a Scala `Frontend/` and a Rust `FrontendRust/` |
| relation | worktree whose parent repo is gone; no git history | **a branch of THIS repo**: `rust-interop-reimpl` |
| evidence | green tests for `extern struct`/`extern func` | 32 end-to-end `tests/rust-interop/ri_*.vale`, plus 7 green in-tree `FrontendRust` tests |

**The shared design:** `extern` is a **body kind**, not a denizen kind. A Rust item becomes an
ordinary Vale generic function template, and the concrete `PrototypeT` is minted **per instantiation**
inside `make_extern_function`, after the solver has resolved the type params. Generics need no
special path because nothing special happens until the ordinary machinery has run.

Working there, both own and inherited generics
(`RustInteropReiImpl`, `FrontendRust/src/integration_tests/tests/hammer_tests.rs:685`):

```vale
extern struct Foo<A> imm { extern func bar<C>(c C) int; }
exported func main() int { return Foo<int>.bar<str>("hello"); }
```

`ITemplataT::ExternFunction` **panics in both overload resolvers** — so a "new templata arm" is not
the answer either. In ValeRustInterop `ExternFunctionTemplataT` holds a finished `FunctionHeaderT`,
has zero producers, and its `tyype` is `vfail()`. **That is the shipped design, and it is the shape
that cannot work.** Do not port it; it is dead but *reachable*, so producing one anywhere silently
restores the eager-header behavior with no compile error. Harmonious's counsel on this hazard class:
delete it, keep the narrative in docs — *"'never ported' isn't a guard; it's a plan to remember."*

### 2.1 ⚠ Tree-mismatch warning for future surveys

A survey agent recommended synthesizing at a **`FunctionA`/`StructA`** level, citing an `Astrouts`
cache keyed on `range.begin` in `higher_typing_pass.rs`. **That recommendation does not apply to this
tree, and the line numbers it cited were RustInteropReiImpl's.** In Vale4:

- `lib.rs:26` — *"higher_typing was retired outright."* There is no `higher_typing/` directory, no
  `FunctionA`, no `StructA`, no `Astrouts`.
- `FunctionTemplataT.function` is `&'s FunctionS<'s>` (`templata.rs:158-160`), not `&'s FunctionA`.
- `IEnvEntryT::Function(&'s FunctionS<'s>)` (`i_env_entry.rs:15`) is what environments hold, and
  `entry_to_templata` (`environment.rs:408-413`) feeds it straight into `FunctionTemplataT`.

RustInteropReiImpl still has `higher_typing/`, which is why its `templata.rs` differs. **`FunctionS`
is the only level in Vale4.** Any future survey of that branch must state which tree each `file:line`
belongs to.

---

## 3. Why they generate `.vale` text, and why we should not

Their stated reason is that files generated late would still need parsing — an argument about *text
generated late*, not about synthesizing IR, which is dismissed in six words.

That dismissal is **factually wrong about their own codebase**: the macro system bypasses the parser
for every struct in every Vale program.

The real constraint was architectural: **ValeRuster is a separate OS process**, a Rust binary reading
hundreds of MB of rustdoc JSON, invoked via `system()`. Text files were the IPC format. Our `TyCtxt`
oracle is in-process, so that constraint evaporates — along with its taxes (regex pre-scan of source
for `import rust\.`, no caching, a resolver-ordering arcana).

---

## 4. The approach: synthesize `FunctionS` / `StructS` from oracle data

### 4.1 `ExternBody` *is* the wrapper-with-externcall-node path

Worth stating plainly, because it reads like a fork and isn't. `IBodyS::ExternBody` dispatches to
`make_extern_function` (`function_compiler_core.rs:149-160`), and that function already builds the
whole wrapper:

- the Vale-facing `FunctionHeaderT`
- an `ExternFunctionNameT`-named `extern_prototype`
- **empty instantiation bounds, registered right there** (`:352-362`) — at instantiation time, which
  is the correct timing
- one `ArgLookupTE` per param, body = `Return(ExternFunctionCall(extern_prototype, arg_lookups))`
  (`:369-380`)
- `add_function` + `add_function_extern`

There is exactly **one** producer of each end: `ExternFunctionCallTE::new` at
`function_compiler_core.rs:376`, and `IBodyS::ExternBody` at `postparsing/function_scout.rs:657` (the
postparser, on a source-level `extern func`). No body macro emits an extern call. So "GeneratedBody
+ externcall node" is not a thing that exists here — choosing it would mean duplicating
`make_extern_function`.

`make_extern_function` reads nothing parser-specific: it works off the solved `env.id`
(`human_name`, `template_args`, `parameters`), `params2`, `ret_coord`, and
`full_env.function.{range, attributes}`. Nothing requires the declaration to have come from source
text.

**The one genuinely untested combination** is a *synthesized* `FunctionS` carrying `ExternBody`. All
four in-tree macros synthesize with `GeneratedBody`; every `ExternBody` today comes from parsed
source. Test that first, before building on it.

### 4.2 Precedent in this tree

`FunctionS::new` has **7 call sites, 6 of them in the typing pass** — declarations built
programmatically and injected as env entries:

- `typing/macros/citizen/struct_drop_macro.rs:89` and `:169` ← **closest model**
- `typing/macros/citizen/interface_drop_macro.rs:81`
- `typing/macros/struct_constructor_macro.rs:105`
- `typing/macros/anonymous_interface_macro.rs:891`
- `typing/expression/expression_compiler.rs:2091`

`StructS::new` has 2 call sites. Established practice here, not a novel move.

### 4.3 Machinery already in this tree (verified)

| what | where |
|---|---|
| `ExternBody(ExternBodyS)` body kind | `postparsing/ast.rs:433`, `:441` |
| dispatch on it | `typing/function/function_compiler_core.rs:149` |
| `make_extern_function` | `typing/function/function_compiler_core.rs:316` |
| `ExternFunctionCallTE` | `typing/ast/expressions.rs:854` |
| opaque struct lowering | `simplifying/struct_hammer.rs:163-190` → `OpaqueHT`; map at `simplifying/hamuts.rs:48` |
| the extern gate | `instantiating/instantiator.rs:1134-1135` → `kind_externs` |

`make_extern_function` reads `template_args` off `env.id` — i.e. **already solved and concrete** by
the time it runs. That is the whole reason generics fall out for free.

### 4.4 Our rules are simpler than the Scala tree's

`IRulexSR` variants here (`postparsing/rules/rules.rs:19`): `Equals, Literal, Lookup, Call,
RuneParentEnvLookup, KindList, CallSiteFunc, DefinitionFunc, Resolve, BorrowRef, WeakRef, OwnRef`.

**There is no `CoerceToCoordSR`** — commented out at `typing/infer/compiler_solver.rs:114-115`,
because the onion refactor dissolved `CoordT` into ref-wraps inside `KindT`. Consequences:

- Scala needs *three* rules per generic citizen mention; **we need two** (`Lookup` + `Call`).
- Our field is `maybe_ret_kind_rune` (a *kind* rune), not `maybeRetCoordRune`.
- Scala's `FunctionA` constructor asserts every rule rune appears in `runeToType`. **We have no
  `runeToType`**, so that entire class of assertion does not apply.

Pattern, from `struct_drop_macro.rs:60-107`:
```
LookupSR(template_rune, imprecise_name_of_citizen)      // the template
CallSR(kind_rune, template_rune, [generic_param_runes]) // applied to args
```
and a method reuses the citizen's params directly:
`let function_generic_parameters = struct_a.generic_params;`

Open: whether `CallSR` is required at *zero* arity or `LookupSR` alone yields the kind for a
non-generic citizen. The drop macro always emits both, but its citizen is generic.

---

## 5. Naming and resolution

This section replaces the earlier assumption that a Rust type could simply be named. It is the
largest open design area and the one most changed by the `rustc_resolve` survey.

### 5.1 The problem

A `FunctionS` holds runes and rules, not types. The only rule that names a type is
`LookupSR { range, rune, name: IImpreciseNameS }` (`rules.rs:131-135`), which resolves **by
source-level (imprecise) name**. `LiteralSR` carries only int/string/bool (`:190-194`). **No rule
carries a pre-resolved templata.**

So although rustc hands us an exact item and we hold a precise, package-qualified identity for it,
writing the declaration forces us to downgrade that to the string `Counter` and look it up again.

And the imprecise lookup has **no tiebreak**:

- `PackageEnvironmentT::lookup_with_name_inner` takes `_get_only_nearest` and **ignores it**
  (`environment.rs:880`) — it walks builtins plus every global namespace and concatenates.
- `lookup_nearest_with_imprecise_name` then does `_ => panic!("Too many with name")`
  (`environment.rs:164`).

No shadowing, no precedence, no error — a compiler crash, with no way for a user to disambiguate.
Note the asymmetry: **functions are fine** (they go through `lookup_all_with_imprecise_name`, plural,
feeding overload resolution, which scores multiple candidates); **types are not**.

### 5.2 `RuneParentEnvLookupSR` is *not* an escape hatch

Investigated and ruled out, for three independent reasons:

- It is stripped on exactly three paths — `overload_resolver.rs:361` (call-site attempt),
  `array_compiler.rs` (×3), `pattern_compiler.rs:73`. The **function-definition** solve
  (`function_compiler_solving_layer.rs:710`, and `:556`) does not strip it, and the @SROACSD filters
  (`infer_compiler.rs:908-926`) do not remove it — they only drop `DefinitionFunc` from call-site
  solves and `CallSiteFunc`/`Resolve` from definition solves.
- Reaching the solver is a hard error:
  `panic!("vwat: RuneParentEnvLookupSR should have been MKRFA-preprocessed before reaching the solver")`
  (`compiler_solver.rs:1045-1049`). Good news incidentally — the queued fix from
  `mkrfa-protocol-leak.md` has landed, so the silent-couldn't-solve hazard is gone.
- Where it *is* stripped it resolves against `calling_env` **by rune name**. A Vale program calling
  `make_counter()` has no `Counter` rune bound.

Definition solves also pass `&[]` for `initial_knowns` unconditionally (`:711`), so there is no seam
there either.

### 5.3 What rustc actually does (surveyed at `~/rust`, 1.95.0)

Four findings that decide the design.

**Modules are not first-class values in rustc either.** There is no `Res::Module`; a module is
`Res::Def(DefKind::Mod, def_id)`, and `DefKind::Mod` is rejected by every arm of
`PathSource::is_expected` — it never reaches `Ty`. What rustc has is a **resolver result type
strictly larger than the typechecker's value universe** (modules legal as *intermediates*, illegal as
*finals*), plus a side graph of module nodes keyed by `DefId`.

> **Consequence for us:** we do **not** need an `ITemplataT::Module` variant (`ITemplataT` has 15
> variants, none a module — `templata.rs:67-83`). We need a resolver-result enum that is *not*
> `ITemplataT`. That is far cheaper than a new concept in the type system.

**A full-path key map cannot work for Rust.** `library/std/src/lib.rs:575` is
`pub use alloc_crate::vec;` and `Vec` is defined at `library/alloc/src/vec/mod.rs:440`. So the key
`["rust","std","vec","Vec"]` **names no definition** — its def path is `alloc::vec::Vec`. There is no
canonical path to key on: rustc runs an entire `visible_parent_map` query, BFS over the whole forest
of crates, purely to invert def-path → writable-path for diagnostics, and the result is explicitly
many-to-one and lossy. `std` is re-exports all the way down.

**The two real-world analogues of our oracle both walk segments.** clippy's `lookup_with_base`
(`clippy_utils/src/paths.rs:245`) sits outside the resolver against a finished `TyCtxt` — exactly our
position — and loops over `tcx.module_children`. rustdoc's `resolve_rustdoc_path`
(`rustc_resolve/src/lib.rs:2262`) splits a string on `::` and feeds the same loop. Two details worth
inheriting: clippy's header says the function is *expensive, use sparingly*, and it returns
`Vec<DefId>` rather than `Option`, because `memchr::memchr` can resolve to two major versions of the
crate at once.

**Precedence is a struct field, not a comparison.** `rustc_resolve/src/imports.rs:243-266`:

```rust
pub(crate) struct NameResolution<'ra> {
    pub single_imports: FxIndexSet<Import<'ra>>,
    pub non_glob_decl: Option<Decl<'ra>>,
    pub glob_decl: Option<Decl<'ra>>,
}
pub(crate) fn best_decl(&self) -> Option<Decl<'ra>> {
    self.non_glob_decl.or(self.glob_decl)
}
```

"Explicit `use` silently shadows a glob" is literally `non_glob_decl.or(glob_decl)` — no ambiguity to
detect, because the two live in different slots. E0252 is a collision *in the data structure*; E0659
is a loser stapled to the winner and reported lazily at the use site. That is the three-tier model we
want, for one struct with two `Option`s.

Also worth adopting on day one (`rustc_resolve/src/ident.rs:66`): user-defined names outrank
built-in/stdlib names, **so that adding to the stdlib is not a breaking change**.

### 5.4 Scale, and the glob decision

`rustc_resolve` is 26,099 lines, but the irreducible kernel — walk a path against a module tree
honoring imports and shadowing — is **~1,500–2,500 lines**. The rest is diagnostics (~40%), macros,
hygiene, rustdoc, lints, and Rust's own features. Editions cost ~120 lines.

**Globs are what force the fixed-point iteration.** Without globs, imports form a DAG and a
topological sort suffices. rustc's own fixed point still fails to converge in four open 2024 issues,
all the shape "explicit import shadows a glob whose own resolution depends on that glob." Strong
argument for Vale not having globs, or having them late.

And: rustc populates a foreign module's children **on first touch**, not up front
(`rustc_resolve/src/lib.rs:1967`). Directly applicable — never enumerate `std`.

### 5.5 The direction

- **Representation:** an interned **qualified** name — a vector of segments — as the source-level
  imprecise-name form. This fixes Vale's real defect (no way to disambiguate anything, ever), and it
  is right under either resolution strategy. Encode it as a **sibling variant** of
  `IImpreciseNameValS` (`postparsing/names.rs:223-238`, which already has ~14 variants, most of them
  names a user cannot type) rather than by widening `CodeNameS` — that struct has 102 references and
  is the representation of every source identifier in the language.
- **Resolution:** a **walk**, with a per-step primitive `children_of(item) -> [(name, ns, item)]`.
  Backed by our existing store for Vale modules, and by `tcx.module_children` for Rust, as clippy
  does. **Deferred** — not needed for a crate-root item.
- **Escape hatch worth knowing:** rustc's own answer for naming library items from compiler code is
  not paths at all — `#[rustc_diagnostic_item = "Vec"]`, 397 of them, a flat `Symbol → DefId` map
  declared at the *definition* site. If Vale's prelude only needs a handful of Rust items, that is a
  registry rather than a resolver.
- **Not blocking:** putting the segment vector on `LookupSR` instead of on the name does not help.
  `compiler_solver.rs:1028-1033` passes `r.name` straight into a lookup keyed on a single interned
  `IImpreciseNameS` (`environment.rs:519`), so a vec on the rule either needs the walker anyway or
  needs the name-level form regardless.

### 5.6 Vale's own name story (independent of interop)

Correction to an earlier claim: **`import` is closer to working than previously recorded.**
`ImportS { range, module_name, package_names, importee_name }` (`postparsing/ast.rs:341-347`) carries
the full path *and* the imported name intact into postparsing. The only consumer of
`program.imports` anywhere is a test traversal. The data is all there; nothing reads it.

Three pieces, in value order, neither of the first two needing a walker:

1. Make `import X.Y.Z` bind `Z` in the importing scope — registration-time mapping.
2. Turn `panic!("Too many with name")` into a real ambiguity error.
3. Qualified paths as an escape hatch.

---

## 6. Identity — the top implementation risk

**Four things key on a synthesized declaration's range/location**, and the first failure mode is
silent:

| consumer | where |
|---|---|
| `FunctionTemplataT` eq/hash on `(function.range, function.name)` — **ignores `outer_env`** | `templata.rs:162-176` |
| `StructDefinitionTemplataT` eq/hash on `(origin_struct.range, origin_struct.name)` | `templata.rs:202-214` |
| `FunctionNameS { name, code_location }` → `FunctionTemplateNameT { human_name, code_location }` | `postparsing/names.rs:497`, `typing/names/names.rs:1304` |
| `ExternTemplateNameT` — `code_loc` is its **only** field | `typing/names/names.rs:1236-1238`, built at `function_compiler_core.rs:387` |

Overload candidates are deduped through a `HashSet` — `undeduped_candidates.into_iter().filter(|c|
seen.insert(*c))` (`overload_resolver.rs:576`, also `:191`) — over
`ICalleeCandidate::Function(FunctionCalleeCandidate { ft: FunctionTemplataT })`
(`ast/ast.rs:194-204`, all `#[derive(Eq, Hash)]`).

**So two synthesized externs sharing a sentinel range collapse into one candidate, with no error.**
`Vec::new`, `String::new`, `Box::new` all have human name `new`. You get either "couldn't find
function" for a function that exists, or a silent call to the wrong one.

**Fix:** a distinct negative offset per synthesized denizen, derived deterministically from the
rustc `DefId` (not a counter — @IIIOZ determinism). One mechanism covers all four consumers.

`struct_drop_macro` is safe only by accident of construction: it uses the **real** `struct_a.range`
and `struct_a.range.begin` for the generated function's identity, and sentinels (`-64002`, `-1340`)
only on rules and params, which are never identity keys. We have no real range to borrow.

Note `StructTemplateNameT` carries **no** code_location (`names.rs:1408`) — the deliberate "structs
don't overload" asymmetry — so unique offsets disambiguate function *templatas*, but two same-named
Rust structs still collide at the name level. That is §5, not this section.

---

## 7. Foreign types get a real definition, with empty members

**Corrects Option A's premise.** The assumption was that a Rust type would be declared but never
defined, since we cannot see private fields. That is wrong. The working trees create a completely
ordinary definition with **zero members**, and carry opacity as an *attribute*. Verified three ways
in our own tree:

- `instantiator.rs:1133` inserts into `monouts.structs` **before** the `ExternI` check at `:1134`
- `struct_hammer.rs:177` — `translate_opaque_i` does `hinputs.lookup_struct(&struct_it.id)`, i.e.
  opaque translation *reads* the definition
- `struct_hammer.rs:178` — `assert!(struct_def_i.members.is_empty())`

So `lookup_citizen_by_template_name` → `lookup_struct_template` →
`.expect("Struct template not found")` (`compiler_outputs.rs:560-566`, reached from
`function_compiler_core.rs:405` for internal-method externs) is **an invariant to satisfy, not a
landmine to route around**. Synthesize the `StructS`, with `members: &[]`.

**The gate is the `extern` citizen attribute, not a package check.** It threads
`parser.rs:585` → `post_parser.rs:1215` → `struct_compiler_core.rs:183` → `instantiator.rs:1207`
unmodified, lands in `kind_externs`, and simplifying branches on membership in that map
(`struct_hammer.rs:113`, `type_hammer.rs:49`, `hammer.rs:323`). Our `is_rust_backed` package check is
not what the machinery uses.

**Opacity is H-level only.** All three trees have `OpaqueHT`; none has an `OpaqueTT` or any
typing-level opaque concept. The "give the opaque struct one synthetic blob member" alternative was
tried and abandoned — commented-out corpses across ValeRustInterop — and both successors now *assert*
emptiness. Do not revive it.

**Layout never enters their frontend**: a separate tool generates a throwaway Rust program printing
`size_of`/`align_of`, runs it, and the backend parses the output. Our in-process `TyCtxt` can supply
`layout_of` directly and collapse three steps and a process boundary. Not needed before codegen.

⚠ Harmonious's warning, worth a grep before committing: *"empty" is safe until a consumer iterates
your members and cross-references them against something parallel.* Sky shipped zero layout fields on
a struct with ≥1 source field and eventually ICE'd rustc's debuginfo walker
(`build_struct_type_di_node` assumes `source_fields.len() == layout.fields.count()`), surfacing only
when the type appeared *inside a generic* because debuginfo recursion descends into type params.
**We already inherited the fix** — arch §10.1/§10.4.5 specify wrapper-as-field for exactly this
reason. Searching our tree, the closest analogue is five positional member lookups in `simplifying/`
(`mutate_hammer.rs:144/190`, `load_hammer.rs:173/227/352`), all name-keyed and reached only via field
access, which is impossible on an opaque type — so the parallel-walker shape does not appear to
exist here. Recorded as "looked, did not find," not as a clean bill.

---

## 8. Extern drop — the gate, and the answer we already have

**Every extern struct in this tree panics today**, two ways, both deliberate:

- `struct_compiler_core.rs:86-92` — extern + `share` → *"post-cut design forbids share-flavored
  extern structs"*
- `struct_drop_macro.rs:232-234` — `SharednessT::Single` + extern → *"auto-generated drop for extern
  struct is unsupported; supply an explicit extern func drop(...)"*

and `derive_struct_drop` is in the default macro set (`struct_compiler_core.rs:94-102`). So
synthesizing a `StructS` lands on `struct_drop_macro.rs:234` the first time a Rust type is
instantiated.

The sibling trees' equivalent tests are green only because they declare their extern structs `imm`,
which routes drop to a bare `DiscardTE` that never touches layout — and their design **leaks
knowingly**, since their generated `extern func drop` never actually runs. Plumbing model, not
correctness model. We banned that combination on purpose in the post-cut design.

### 8.1 `todo/opaque-extern-drop.md` conflicts with the architecture doc

That todo proposes per-monomorphization symbol naming plus a user-written
`#[no_mangle] extern "C" fn drop(x: X) { std::mem::drop(x) }` shim per instantiation, and defers two
questions "to the Rust-interop TL." **Both are already answered by arch §1.7**, which specifies the
`__vale_drop<T>(&local)` scope-end wrapper (Sky §F.22 pattern), and by Harmonious's account of the
shipping design:

```rust
#[inline(always)]
pub unsafe fn __vale_drop<T>(x: *mut T) { core::ptr::drop_in_place(x) }
```

One generic wrapper in the stub crate; the compiler appends an ordinary call node at scope end;
rustc resolves `DropGlue` while walking the wrapper's MIR, invisibly. Consequences:

- *"Do `Vec<i32>` and `Vec<str>` call the same symbol, or per-mono mangled?"* — **neither**. No
  mangling decision exists.
- *"Someone must supply a shim per monomorphization; this is the user's obligation."* — **the
  obligation disappears.** One wrapper covers every `T`, and we generate the stub crate.
- Types that aren't `needs_drop` cost nothing for free — `drop_in_place::<T>` is a no-op and
  `#[inline(always)]` erases the wrapper. Sky wrote a `needs_drop` predicate and **deleted** it; it
  could never answer correctly for a bare type parameter `let x: T`, where the answer depends on the
  substitution. Do not write it.
- **By-pointer, not by-value.** Sky first tried `pub fn drop<T>(_x: T) {}` and reverted the same day:
  by-value materializes a drop through a stack copy for every `let`, including moved-out ones, and
  `Vec<Vec<Widget>>` double-frees. The todo's proposed shim is exactly that by-value shape. (Vale
  does track moves statically — `unstackified_locals`, `function_environment_t.rs:201` — so that
  specific mechanism is less likely to bite us, but the pointer shape is correct regardless and is
  required anyway because opaque layout forbids `Destroy`.)

**Action:** `todo/opaque-extern-drop.md` should be corrected against arch §1.7 before anyone
implements from it.

### 8.2 Unwinding: settled

**Global `panic = "abort"`** — ratified 2026-07-25, and it was already arch §1.7 and §16: *"All
Vale-emitted code: `panic = "abort"`. No landing pads, no `catch_unwind`, no panic-as-cancellation."*
Cargo enforces panic-strategy consistency across the whole build graph, so a Rust panic cannot unwind
into Vale frames. This dissolves the `Void`/`Never` destructor-return constraint rather than
answering it. Known cost, accepted: `catch_unwind` does not work, including inside Rust libraries
that sandbox with it.

---

## 9. Remaining hard constraints

1. **`maybe_ret_kind_rune` must be `Some`.** Externs cannot infer return types.
2. **Unique `CodeLocationS` per synthesized denizen** — §6. Highest risk; silent failure.
3. **Package coordinate must agree** across the range's file, the declaration name, and the owning
   store. ⚠ New tiebreak: `simplifying/hammer.rs:332-333` is
   `panic!("translate functionExterns: rust-package empty-name branch")` when
   `package_coord.module.0 == "rust"`. A real `rust.*` coordinate walks straight into it; an
   *internal* coordinate sidesteps it and gives graceful humanizer degradation for free
   (`CodeLocationS::internal`, `utils/range.rs:29-37`) — but that hardcodes package coord `""`,
   conflicting with the coordinate-agreement invariant (`templata.rs:472`, `:390`). Minting our own
   `FileCoordinate` with the real coordinate plus a synthetic filename and negative offsets is the
   third option.
4. **Inherited generic params go LAST.** ⚠ **Resolved** — the two surveys disagreed; four
   independent sources in RustInteropReiImpl plus our own comment at
   `function_compiler_core.rs:398-402` settle it: *"internal-method externs inherit the container's
   generic params at the end of their templateArgs."* The arcana's expansion ("Parent Runes Inherited
   In Reverse Order") means parent-**last**, not reversed-within. Better: the @SMLRZ re-split
   projector already exists — `GenericParametersInheritance { num_inherited_generic_parameters }` is
   computed from citizen arity, passed to `add_function_extern`, and used by Hammer to reshape the
   wire-format SimpleId so inherited args land on the citizen step.
5. ~~**Suppress the struct-constructor macro.**~~ **Struck** — non-issue. Our default macro set is
   only `derive_struct_drop` (`struct_compiler_core.rs:94-102`); `DeriveStructConstructor` is not a
   keyword in this tree. The corollary is worse news: `DeriveStructDrop` **is** in the default set,
   which is what walks into §8.
6. **Extern drop** — §8. A prerequisite, not an epilogue.

---

## 10. The @SMLRZ trap

ValeRustInterop once baked Rust's *name shape* into the typing pass. It broke three escalating ways
and took a full rollback. The architect's conclusion:

> Rust's `Vec<i32>::push` form has **no internal justification in Vale** — no specialization, no
> coherence, no type-first dispatch. It is purely a **foreign rendering concern that was wrongly
> baked into the typing pass.**

**We are at higher risk than they were.** They read Vale source text — already in Vale's shape — and
had to work to convert it to Rust's. We read `TyCtxt`, where the Rust shape is what we are handed
natively. *Preserving* it is our path of least resistance and it is the wrong one. They had to climb
toward the mistake; we would fall into it. It already happened once: the seam was minting
`rust.mycrate :: [Struct(Counter)] :: ExternFunction(get)`.

**Self-check for every synthesized declaration:** it should be structurally indistinguishable from
what the postparser produces for a hand-written `extern func get(self Counter) int` inside
`extern struct Counter`. If the oracle's knowledge of *which args came from the impl* is visible
anywhere in the `FunctionS`, @SMLRZ is being rebuilt.

**One deliberate, recorded exception:** a qualified/synthetic imprecise name (§5.5) is visibly
*not* what the postparser emits for a hand-written declaration. It is accepted as a **namespacing**
decision, not a Rust-shape leak — but it is a knowing deviation from this rule, not an oversight.

---

## 11. What this deletes from the current tree

- `rust_package_stores` and its hook in `compiler.rs`
- `push_rust_call_candidates` (already free-functions-only after the importer landed)
- `import_rust_types`' prototype minting and its `add_instantiation_bounds` calls
- the per-call `resolve_function` / `resolve_method` oracle queries

The oracle shrinks from a query service consulted at every call site to a **binding generator**
consulted once, whose job is to produce declarations. `fn_sig(item, args, ..)` survives and finally
receives real args — from `make_extern_function`, per instantiation.

Surviving unchanged: the `'tcx`-free `RustOracle` trait shape, `Oracles` on `Compiler`, the `rust`
reserved package coordinate, `is_rust_backed`, the driver host, the logging oracle, and the
cargo-feature/`build.rs` gating.

---

## 12. Near-term plan

**Build `add_two_numbers` first. Do not build a resolver yet.**

The walking-vs-key question only bites when a path has more than one segment or crosses a re-export.
`add_two_numbers` is one item at a crate root, so it proves the thing actually in doubt — that a
synthesized `FunctionS` with an `ExternBody` flows through the solver and `make_extern_function` —
without paying for name machinery we cannot yet validate. It also has no citizen init step, so it
avoids `lookup_struct_template` and the §8 drop gate entirely.

**Carry now** (cheap, expensive to retrofit):

1. A qualified interned name in `LookupSR`, not a bare `CodeName` (§5.5).
2. Key on `DefId`, not strings, in the oracle (§13).
3. A unique `DefId`-derived `CodeLocationS` per synthesized item (§6).

**Defer deliberately:** the path walker and `children_of` primitive, lazy module population, the
`non_glob_decl`/`glob_decl` precedence struct, and the `Vec<DefId>`-shaped oracle return. All
correct; none needed for one crate-root function; all better designed once real signatures have
flowed through.

**Then:** the `StructS` path, which requires §8 resolved first.

---

## 13. Known defect: the oracle matches by name

`TyCtxtOracle` violates arch §26's **@ATAFLBZ** invariant, which already states the rule:
*"Walks of `tcx.all_impls(...)` filter by `is_from_vale_stubs(self_type_did)`… Self-type-name check
is ambiguous because std and Vale could both define a type named (e.g.) `Box`. Under single-symbol
architecture, wrong DefId produces wrong rustc-mangled name when Vale's bitcode emits a body."*

Three sites:

- `tyctxt_oracle.rs:73-76` — walks `tcx.crates(())`, i.e. **every loaded crate**, matching
  `child.ident.to_string()` against a bare-name allowlist.
- `tyctxt_oracle.rs:240-256` — `resolve_method` matches the method's **owner type by human-name
  string**, in a linear scan.
- `tyctxt_oracle.rs:259-264` — `resolve_function` is `position(...)` by name, first-wins.

Also **insufficient, not merely slow**: `module_children` on a crate root yields only that root's
*direct* children, so `std::vec::Vec` (a child of `std::vec`) would never be found. Today's fixture
works solely because its items sit at the crate root. A `VCOORD` note recording all of this sits on
`TyCtxtOracle::new`.

The identification mechanism is already designed: arch §6.3's `__VALE_STUBS_MARKER` plus a
**DefId-parentage check** (Sky's empirical correction — glob re-exports can otherwise re-export the
marker into a downstream crate and falsely flag it). Provenance must be an in-band property of the
artifact, never an env var or wrapper flag — that was Sky's `CARGO_PRIMARY_PACKAGE` bug.

---

## 14. Tree state

- Committed: `699241ffb` on `experimental-4`, ratcheted to local `experimental`. 41 files.
- Uncommitted after it: `rust_package_stores`, `importable_functions` across four oracle impls, the
  `CompilerOutputs::new()` move + namespace hook in `compiler.rs`, the `push_rust_call_candidates`
  deletion from `overload_resolver.rs`, the `Counter`/`make_counter` fixture additions, the §1.7
  arch-doc bullet, the `TyCtxtOracle::new` VCOORD note, and this document.
- Suites: **573/175/8** default, **574/175/8** with `--features rust_interop`.
- `cargo check --lib --features rust_interop` clean, 7 pre-existing warnings.
- The driver typechecks `exported func main() int { return (make_counter()).get(); }` against a real
  `TyCtxt`.
- `rustc-dev` is installed locally but **deliberately not pinned** in `rust-toolchain.toml` —
  pinning would force a several-hundred-MB fetch on everyone, on every branch, in every CI job.

### 14.1 Blocked elsewhere

- `substitute_templatas_in_kind` — all four reference-wrap arms are `unimplemented!()`
  (`typing/templata_compiler.rs:522-525`). A `&self` receiver lowers to `BorrowRef` and hits it.
- `is_type_convertible` — "unhandled borrow read-out `BorrowRef(T) -> T`" (`:1209`).

Vale2 confirmed 2026-07-25 that this cluster belongs to their "overload resolution & dispatch model
redesign" mission — **owned, not started, no date**. So the fixture's by-value `self` plus
parenthesized method syntax (which moves rather than borrows) is a long-lived workaround. They also
report that `.` performs a **receiver adjustment** (autoref/deref) and is *not* pure sugar for
`foo(x)` — relevant if any part of the method story assumed otherwise.

- The 7 extern-struct tests are `#[ignore]`d **and** `integration_tests` is commented out of
  `lib.rs:37` entirely, so they do not compile. Un-ignoring them is not a shortcut; relinking that
  module is onion-arc work.
