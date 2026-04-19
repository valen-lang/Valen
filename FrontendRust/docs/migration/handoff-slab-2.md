# Handoff: Typing Pass Slab 2 — Name Hierarchy

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slab 1 (leaf types: `OwnershipT`, `MutabilityT`, primitive `KindT` payloads) is done and committed. You're doing Slab 2 — the ~95 name types in `src/typing/names/names.rs`. It's the biggest slab in raw line count, but mechanically repetitive once you've got the first few right. Budget: plan for this to take a full workday; it's boring, not hard.

**Read these first in this order**, then come back:

1. `quest.md` — at least §§1.2, 1.4, 1.5, 6.2, 6.3, 12.1 Slab 2 paragraph. That is the design spec; it is the source of truth. (§12.0 refers to a `// mig:` audit trail that Step 0 below explicitly removes — ignore the `// mig:` parts and follow the `/* scala */` block rules.)
2. `.claude/rules/postparser/IDEPFL-postparser-interning.md` — the dual-enum "value enum for lookup / reference enum for storage" pattern. Every interned typing-pass type family follows this pattern, and Slab 2 creates four of them.
3. `FrontendRust/docs/migration/handoff-god-struct-progress.md` — background on the slice pipeline and the pre-commit hook that guards the `/* scala */` blocks. You won't *do* god-struct work, but you'll follow the same audit-trail convention (blocks, not markers).
4. This doc.

You shouldn't need to read the Scala source externally — every Scala case class and sealed trait is already embedded inline in the `/* ... */` blocks in `src/typing/names/names.rs`. Treat those as your spec. If you can't figure out what a Scala type does from its block, ask; don't guess.

## The big picture: why Slab 2 exists

Scala `IdT[+T <: INameT]` — a name path like `myapp::foo::bar` — is at the heart of how the typing pass identifies every function, type, local, and template. The postparser has its own `INameS` hierarchy; the typing pass translates those into `INameT` (the "T" suffix is just a Scala convention meaning "typing-pass version", no relation to "type"). Scala's `INameT` is a sealed trait; ~60 concrete `case class`es extend it, and ~14 sub-traits (`IFunctionNameT`, `IStructNameT`, `IVarNameT`, …) slice the hierarchy into groups.

In Rust, we're turning all of this into:

- **60 concrete name structs** (`FunctionNameT<'s, 't>`, `StructNameT<'s, 't>`, etc.) — each with real Scala-parity fields. These are the leaves.
- **A top-level `INameT<'s, 't>` enum** — one variant per concrete name struct. Each variant holds `&'t SomeConcreteNameT<'s, 't>` (an arena reference).
- **14 sub-enums** (`IFunctionNameT<'s, 't>`, `IStructNameT<'s, 't>`, etc.) — same pattern, but only the concrete names that belong to that group. A given concrete name can appear in multiple sub-enums when Scala had it extending multiple sub-traits (see §6.2 "DAG rule"). This is the most error-prone part of the slab.
- **One generic `IdT<'s, 't, T: Copy>` struct** — `{ package_coord, init_steps, local_name: T }`, with conversion impls (`widen`, `try_narrow`) that move between `IdT<'s, 't, &'t IFunctionNameT<'s, 't>>` and `IdT<'s, 't, &'t INameT<'s, 't>>`.
- **Parallel `Val` enums/structs** (`INameValT`, `IFunctionNameValT`, etc.) that serve as HashMap lookup keys for IDEPFL interning. See that rules file for the rationale; the tl;dr is: reference enums hold `&'s` / `&'t` pointers (can't be constructed without allocating first), so we need a transient lookup key that holds payload by value. The Val is what goes into the `TypingInterner`'s `HashMap<NameValT, &'t NameT>` so you can ask "have I interned this before?" without allocating.

By the end of Slab 2, `names.rs` compiles cleanly (0 errors), nothing past names.rs has been touched, and `cargo check --lib` is still green. Body migration of name types only — no methods, no solver logic, no compiler logic. You're stamping out data-class bodies, adding one enum variant per leaf struct in each sub-enum it belongs to, and wiring up the conversions.

## What's already in place (don't duplicate; don't delete)

Open `src/typing/names/names.rs`. You'll see ~95 entries, each in one of two shapes:

**Shape A — concrete name struct (stub), pre-Step-0:**
```rust
pub struct FunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class FunctionNameT(
  template: FunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT { ... }
*/
```

Your job for each concrete struct is: read the Scala `case class` in the `/* */`, translate the fields into Rust per the arena/lifetime rules, replace the `PhantomData` tuple struct with a named-field struct, delete the `// TODO: placeholder PhantomData — replace with real fields during body migration` line if it exists above the struct. Keep the Scala `/* */` untouched — the pre-commit hook checks it verbatim.

**Shape B — sub-enum (stub), pre-Step-0:**
```rust
pub enum IFunctionNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IFunctionNameT extends INameT with IInstantiationNameT {
  def template: IFunctionTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
  ...
}
*/
```

For each sub-enum, read the Scala sealed trait to figure out membership (every concrete `case class` extending this trait is a variant). Replace `_Phantom(...)` with one variant per extending concrete class: `Function(&'t FunctionNameT<'s, 't>)`, `ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>)`, etc. The variant name is the concrete struct's name minus the trailing `T` (so `FunctionNameT` → variant `Function`). The payload is always `&'t <ConcreteStructName><'s, 't>` — an arena reference, never `Box`, never owned.

**Don't touch anything outside names.rs** unless this guide explicitly tells you to. If a name struct needs a field of some type that doesn't exist yet (like `templateArgs: Vector[ITemplataT[ITemplataType]]`), use the existing stub. `ITemplataT<'s, 't>` is already defined as a `_Phantom` enum; you can reference it by name. Same with `CoordT<'s, 't>` (already real from Slab 1).

## Rules for each field translation

For every Scala case class field, map per these rules:

| Scala type | Rust type |
|---|---|
| `StrI` | `StrI<'s>` — scout-lifetime-interned string |
| `PackageCoordinate` | `&'s PackageCoordinate<'s>` |
| `IRuneS` (any variant) | `IRuneS<'s>` — already-interned postparser rune |
| `IImpreciseNameS` | `&'s IImpreciseNameS<'s>` |
| `RangeS` / `CodeLocationS` | `RangeS<'s>` / `CodeLocationS<'s>` (both Copy) |
| `CoordT` | `CoordT<'s, 't>` (Copy, inline per §6.4) |
| `ITemplataT[...]` | `ITemplataT<'s, 't>` (small enum, passed by value; `_Phantom` for now — it'll grow in Slab 3) |
| `INameT` sub-trait like `IFunctionNameT` | `&'t IFunctionNameT<'s, 't>` |
| Concrete `FunctionNameT` / `StructTemplateNameT` / … | `&'t FunctionNameT<'s, 't>` (through the arena) |
| `Vector[T]` of interned/arena-stored items | `&'t [T]` (arena slice) — e.g. `&'t [ITemplataT<'s, 't>]`, `&'t [CoordT<'s, 't>]`, `&'t [&'t INameT<'s, 't>]` |
| `Int` | `i32` |
| `Long` | `i64` |
| `Boolean` | `bool` |
| `IdT[SomeNameT]` | `IdT<'s, 't, &'t SomeNameT<'s, 't>>` (the generic parameter is the *leaf* type) |

Every concrete name struct gets `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` — they're small (pointers and Copy primitives only), Copy by design per ATDCX. The structs are output data; no `Vec`, `HashMap`, or `String` fields ever (AASSNCMCX). Every collection field is an arena slice `&'t [...]` or `&'s [...]`.

Every sub-enum `INameT`, `IFunctionNameT`, etc. gets `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` too — they're tagged pointers.

Add `where 's: 't` as an impl-block-level bound on anything that transitively holds `&'s` data (§1.2). The struct/enum definitions themselves don't need the bound — the bound comes in when you write `impl<'s, 't> FunctionNameT<'s, 't> where 's: 't { ... }`.

## The DAG rule (§6.2) — the trap

Some Scala concrete names extend **multiple** sub-traits. Example from `names.rs`:

```scala
case class ExternFunctionNameT(...) extends IFunctionNameT with IFunctionTemplateNameT { ... }
```

`ExternFunctionNameT` is *both* an `IFunctionNameT` and an `IFunctionTemplateNameT`. In Rust, this means `ExternFunctionNameT` appears as a variant **in both** sub-enums:

```rust
pub enum IFunctionNameT<'s, 't> {
    Function(&'t FunctionNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),  // shared with IFunctionTemplateNameT
    Forwarder(&'t ForwarderFunctionNameT<'s, 't>),
    // ...
}

pub enum IFunctionTemplateNameT<'s, 't> {
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),  // shared with IFunctionNameT
    // ...
}
```

Both variants point at the *same* `&'t ExternFunctionNameT` pointer — the scout arena allocates one `ExternFunctionNameT`, and two enum tags (one in `IFunctionNameT`, one in `IFunctionTemplateNameT`) can wrap it. The enums are just differently-tagged views over the same arena payload.

**How to find the DAG memberships.** For each concrete struct, look at its Scala `case class ... extends A with B with C` line and split on `with`. Those are the traits it extends. Each of those (if it maps to one of our 14 Rust sub-enums) gets a variant.

Map from Scala sub-trait name to Rust sub-enum name:

| Scala sub-trait | Rust sub-enum |
|---|---|
| `INameT` | `INameT` (top-level) |
| `ITemplateNameT` | `ITemplateNameT` |
| `IFunctionTemplateNameT` | `IFunctionTemplateNameT` |
| `IFunctionNameT` | `IFunctionNameT` |
| `IInstantiationNameT` | `IInstantiationNameT` |
| `ISubKindTemplateNameT` | `ISubKindTemplateNameT` |
| `ISuperKindTemplateNameT` | `ISuperKindTemplateNameT` |
| `ISubKindNameT` | `ISubKindNameT` |
| `ISuperKindNameT` | `ISuperKindNameT` |
| `ICitizenTemplateNameT` | `ICitizenTemplateNameT` |
| `ICitizenNameT` | `ICitizenNameT` |
| `IStructTemplateNameT` | `IStructTemplateNameT` |
| `IStructNameT` | `IStructNameT` (inside `CitizenNameT` grouping — confirm against existing Rust `pub enum CitizenNameT` shape) |
| `IInterfaceTemplateNameT` | `IInterfaceTemplateNameT` |
| `IInterfaceNameT` | `IInterfaceNameT` |
| `IImplTemplateNameT` | `IImplTemplateNameT` |
| `IImplNameT` | `IImplNameT` |
| `IRegionNameT` | `IRegionNameT` |
| `IVarNameT` | `IVarNameT` |
| `IPlaceholderNameT` | `IPlaceholderNameT` |

If Scala uses a sub-trait that you can't find in the list above (happens rarely — e.g. `IInterning`, `Equatable`), it's a Scala marker trait with no Rust counterpart. Ignore it; it does not map to a Rust enum variant.

**If a concrete struct extends a sub-trait transitively** (`X extends IFunctionNameT` where `IFunctionNameT extends INameT`): you still put `X` as a variant in `IFunctionNameT` **and** in `INameT`. Both. The top-level `INameT` enum is the "everything" union; every concrete struct is a variant there regardless of which sub-traits it extends.

## The `IdT<'s, 't, T: Copy>` generic

Read `quest.md` §6.3 carefully before you start on `IdT`. The salient points:

```rust
pub struct IdT<'s, 't, T: Copy>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'t [&'t INameT<'s, 't>],
    pub local_name: T,
}
```

- `T: Copy` is the only bound on the struct itself. Keep it minimal.
- `init_steps` is an arena slice of `&'t INameT<'s, 't>` pointers (so every step is an already-widened `&'t INameT` — the widest form).
- `local_name: T` holds the leaf-kind-specific type (e.g. `&'t IFunctionNameT<'s, 't>` or `&'t StructTemplateNameT<'s, 't>`).
- Conversion methods live in separate `impl` blocks with conversion-specific bounds:
  - `widen` — `T: Into<&'t INameT<'s, 't>>`, narrow → wide
  - `widen_to<U>` — generic upcast, `T: Into<U>`
  - `try_narrow<U>` — `&'t INameT<'s, 't>: TryInto<U>`, wide → narrow (fallible)
- `Copy + Clone` derives on `IdT<'s, 't, T>` itself (it's a pointer triple + a small `T`).

You also need `From`/`TryFrom` impls between sub-enums (wide→narrow and narrow→wide). For each concrete name `XxxNameT` that appears in sub-enum `IYyyNameT`, generate:

```rust
impl<'s, 't> From<&'t XxxNameT<'s, 't>> for IYyyNameT<'s, 't> {
    fn from(x: &'t XxxNameT<'s, 't>) -> Self { IYyyNameT::Xxx(x) }
}
```

For narrowing, use `TryFrom` returning `Option<...>` (or `Result<_, ()>`). You don't strictly need `TryFrom` for every narrowing — only the ones that `IdT::try_narrow` needs. Start with the obvious ones (`INameT` → each sub-enum) and add more as needed.

## IDEPFL — the dual-enum Val pattern

Read `.claude/rules/postparser/IDEPFL-postparser-interning.md` before writing any `Val`. The pattern is used in the postparser and we're replicating it in the typing pass for these four families:

1. `INameT<'s, 't>` ↔ `INameValT<'s, 't>`
2. `IdT<'s, 't, T>` ↔ `IdValT<'s, 't, T>`
3. Each of the 14 sub-enums gets a `*ValT` companion (e.g. `IFunctionNameT` ↔ `IFunctionNameValT`).
4. Each concrete name struct either needs its own `XxxNameValT` struct or can reuse itself as its own Val (the "Simple" case from IDEPFL — when the struct contains only Copy fields and no `&'t` children).

**How to decide if a concrete name needs a separate Val struct:**

- **Simple** (no separate Val): The struct's fields are all Copy primitives or scout-lifetimed refs (`StrI<'s>`, `IRuneS<'s>`, `RangeS<'s>`, `&'s PackageCoordinate<'s>`). There are no `&'t` fields. Example: `CodeVarNameT<'s, 't> { name: StrI<'s> }` — the Val is just the same struct by value. The sub-enum's Val variant looks like `CodeVarName(CodeVarNameT<'s, 't>)`.
- **Shallow** (separate Val with same shape): The struct holds `&'t` refs to already-interned types. The Val struct holds the same fields but by value (still `&'t` refs, since those are owned pointer-sized tags — canonical). Example: `FunctionNameT<'s, 't> { template: &'t FunctionTemplateNameT<'s, 't>, ... }` — Val: `FunctionNameValT<'s, 't> { template: &'t FunctionTemplateNameT<'s, 't>, ... }`. Same fields.
- **Transient with `'tmp`** (separate Val with borrowed slices): The struct holds `&'t [...]` slices. The Val holds `&'tmp [...]` borrowed slices (slices allocated lazily on a miss inside the intern method). This follows the `@DSAUIMZ` / `ImmediateInterningDiscipline-IIDX` shield — keep the slice on the stack until we confirm a miss, then promote. Example: `IdT<'s, 't, T>` has `init_steps: &'t [&'t INameT<'s, 't>]` — Val: `IdValT<'s, 't, 'tmp, T>` with `init_steps: &'tmp [&'t INameT<'s, 't>]`.

The postparser names file (`FrontendRust/src/postparsing/names.rs`) has a complete worked example of all three kinds. You will consult that file constantly during Slab 2. That's the reference.

**Do not implement the `TypingInterner` intern methods themselves in Slab 2.** The intern methods live in `src/typing/typing_interner.rs` and are currently `panic!()` stubs. Leave them alone — body-filling those is part of Slab 3 or its own mini-slab. All you're doing in Slab 2 is *defining* the Val types so the interner signatures type-check. Construction of the actual `HashMap<NameValT, &'t NameT>` storage inside `TypingInterner` is a separate workstream.

## Step-by-step plan

Work bottom-up — concrete structs first, then sub-enums once their variants exist, then the top-level `INameT`, then `IdT` and conversions.

### Step 0: Strip all `// mig:` markers (prerequisite cleanup)

The `// mig: struct ...` / `// mig: enum ...` / `// mig: fn ...` / `// mig: impl ...` / `// mig: trait ...` markers are an artifact of the slice pipeline — they existed so slicing scripts could match each Rust definition to its Scala counterpart. Slicing is done, the pipeline has been fully consumed across `src/typing/`, and the markers no longer serve a purpose. The `/* scala */` blocks remain as the audit trail; those are what the pre-commit hook enforces, not the `// mig:` lines.

Do a pre-pass before starting Slab 2 work: sweep across all of `src/typing/` (including subdirs like `macros/`, `citizen/`, `function/`, `expression/`, `infer/`, `names/`, `templata/`, `ast/`, `env/`, `types/`) and delete every `// mig:` comment line.

Examples of lines to delete (there are ~500+ of them):

```
// mig: struct FunctionNameT
// mig: enum IFunctionNameT
// mig: fn generate_function_body_lock_weak
// mig: impl Compiler
// mig: trait IFunctionGenerator
```

**Rules for the cleanup:**

- **Delete only lines whose content starts with `// mig:`** (optionally preceded by whitespace). Don't delete anything else — in particular, don't touch `// TODO: placeholder PhantomData` lines (those flag stubs that later slabs will resolve) or `// vestigial:` notes (those flag intentional workarounds) or `// (no scala counterpart ...)` notes.
- **Preserve surrounding whitespace.** When deleting a `// mig:` line, remove the entire line including its newline — don't leave a blank line where it was unless the file had a blank line there before the marker was inserted.
- **Don't change the `/* scala */` blocks.** The pre-commit hook is strict about them; see Gotcha 1 below.
- **Do NOT run this as a bulk `sed`** against the whole tree in one go. The project's `CLAUDE.md` has a whole "Bulk Sed Safety Protocol" section; follow it. Safer approach:
  - Run per-file (or per-subdir) with the `Edit` tool.
  - After each batch of ~5 files, run `cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-2.txt 2>&1` and confirm `grep -c "^error" /tmp/sylvan-slab-2.txt` is still 0. If it jumped, you accidentally deleted something that wasn't a `// mig:` line (or deleted one that actually was a load-bearing annotation — shouldn't happen, but verify).
  - If you *do* want to use `sed` on a single file as a time saver, dry-run it first: `sed "/^\s*\/\/ mig:/d" FILE | diff FILE -`, eyeball the diff, then apply with `sed -i ''` (macOS) only if it looks right.
- **Expected scope:** ~500+ markers across ~90 files. Ballpark a few hours of mechanical work. Commit the whole sweep as one commit ("Strip `// mig:` markers from src/typing/ — slice pipeline artifacts, no longer needed"). `cargo check --lib` must stay at 0 errors throughout (it should — deleting line comments can't break code).

After this cleanup is committed, move to Step 1. From here on out — including in Slab 2 itself — do not add `// mig:` markers to any new code.

### Step 1: Confirm your starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-2.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-2.txt
```

That should say `0`. If it doesn't, stop and tell the senior.

Note: the project sets `#![allow(unused_variables, unused_imports)]` in `src/lib.rs`, so warnings are suppressed. Don't rely on the warning count — it'll always be 0. Rely on error count.

### Step 2: Concrete name structs (do ~60 of them, one at a time)

Order doesn't matter much — you can do alphabetical, or bottom-up from leafless ones. I recommend: start with a very simple one like `CodeVarNameT` to feel out the process, then bang through the rest.

For each concrete struct:

1. Read its Scala `case class` in the `/* */` block below it.
2. Figure out each field's Rust type per the translation table above.
3. Replace the `pub struct XxxNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);` line with a real `pub struct XxxNameT<'s, 't> { pub field1: T1, pub field2: T2, ... }`.
4. Delete any `// TODO: placeholder PhantomData — replace with real fields during body migration` line immediately above the struct (that comment is only for Phase-1-era stubs).
5. Add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` above the struct.
6. Leave the `/* scala */` block untouched. (The `// mig:` markers will have been deleted in Step 0.)

Do **not** write `impl XxxNameT { ... }` blocks. The Scala class bodies have lots of derived methods (`def packageId`, `def steps`, etc.) — those are Slab 8/9+ work. Just the struct bodies.

**Every few structs, run `cargo check --lib` to catch mistakes early.** Early errors are cheap to fix; late errors require scrolling through a dozen failing call sites.

### Step 3: Sub-enums (14 of them)

For each `pub enum IXxxNameT<'s, 't> { _Phantom(...) }`:

1. Scan `/* */` text above and below for `extends IXxxNameT` occurrences to find the concrete names that belong.
2. Replace `_Phantom(...)` with one variant per belonging concrete name: `Xxx(&'t XxxNameT<'s, 't>)`.
3. Add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`.
4. Keep a scratch list as you go of which concrete names you've placed into which sub-enums. Cross-reference the DAG rule. A concrete name that extends `A with B with C` should appear in `A`, `B`, and `C`.

The top-level `INameT<'s, 't>` is a special case: **every** concrete name is a variant of `INameT`. That's the union-of-everything enum.

### Step 4: `IdT<'s, 't, T: Copy>`

1. Replace `pub struct IdT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);` with the three-field struct per §6.3.
2. Add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`.
3. Add `where 's: 't,` clause.
4. Add `impl<'s, 't, T: Copy + ...> IdT<'s, 't, T> { widen / widen_to / try_narrow }` in separate impl blocks per their conversion bounds.

### Step 5: `From` / `TryFrom` bridges

For each concrete name `XxxNameT` belonging to sub-enum `IYyyNameT`:

```rust
impl<'s, 't> From<&'t XxxNameT<'s, 't>> for IYyyNameT<'s, 't> {
    fn from(x: &'t XxxNameT<'s, 't>) -> Self { IYyyNameT::Xxx(x) }
}
```

For each sub-enum `IYyyNameT` narrowing from `INameT`:

```rust
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IYyyNameT<'s, 't> {
    type Error = ();
    fn try_from(n: &'t INameT<'s, 't>) -> Result<Self, ()> { ... }
}
```

(You might need a `match` inside `try_from` that pattern-matches on `INameT`'s variants and returns `Ok(...)` for the subset. This can get tedious; only write the bridges that `IdT::try_narrow` or higher-layer code actually needs. Start minimal; add as downstream build errors surface.)

### Step 6: `*ValT` companions

Per IDEPFL. Goes adjacent to each name type:

- For each simple-kind concrete `XxxNameT`: no separate Val needed, the sub-enum's Val variant holds `XxxNameT<'s, 't>` by value.
- For each shallow-kind concrete `XxxNameT`: add `pub struct XxxNameValT<'s, 't> { ...same fields... }`. The sub-enum's Val variant holds `XxxNameValT<'s, 't>`.
- For `IdT`, add `pub struct IdValT<'s, 't, 'tmp, T: Copy> { package_coord: &'s PackageCoordinate<'s>, init_steps: &'tmp [&'t INameT<'s, 't>], local_name: T }`.
- Top-level `INameValT<'s, 't>` with one variant per concrete name, payload per the rules above.

Mark these as "(no scala counterpart — Rust-only interning scaffolding)" with a `//` line comment above each Val type. Do **not** put them inside `/* */` blocks — the pre-commit hook will yell.

### Step 7: Verify and commit

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-2.txt 2>&1
tail -30 /tmp/sylvan-slab-2.txt
grep -c "^error" /tmp/sylvan-slab-2.txt
```

Must be 0 errors. If not, triage one at a time from the top of the file.

Commit when clean:

```bash
git add FrontendRust/src/typing/names/names.rs
git commit -m "$(cat <<'EOF'
Typing Slab 2: flesh out name hierarchy — IdT generic + ~60 concrete
names + 14 sub-enums + IDEPFL Val companions.

<paragraph summarizing what you did>

Error count stays at 0.

Co-Authored-By: Claude <junior> <you@anthropic.com>
EOF
)"
```

## Gotchas (the senior hit these on Slab 1)

1. **Pre-commit hook on `/* */` blocks.** `.claude/hooks/check-scala-comments` runs before every commit and does exact-match comparison between every `/* */` block and the Scala source. If you accidentally put English prose inside a `/* */` or reformat Scala, the commit is rejected. Corollary: when you add the separate `Val` struct, use `//` line comments to explain it — never `/* */`.

2. **`IdT` generic is contagious.** As soon as `IdT` has a real body, everything that holds `IdT<'s, 't>` without specifying `T` breaks. Check usages before Step 4: `grep -rn "IdT<'s, 't>" FrontendRust/src/typing/`. You'll find callers in `types/types.rs`, `names/names.rs`, `ast/citizens.rs`, and expression types. Most of them want `IdT<'s, 't, &'t IXxxNameT<'s, 't>>` for the specific leaf kind Scala was using. Before writing the `IdT` body, expand the existing `IdT<'s, 't>` callers into `IdT<'s, 't, &'t <specific-leaf>NameT<'s, 't>>`. Do this as small, separate edits — it makes blame archaeology easier.

3. **`_Phantom` stays on `KindT`.** Slab 1 added six primitive variants to `KindT` but kept `_Phantom` so the `<'s, 't>` lifetimes are anchored until Slab 3 adds non-primitive variants. Don't touch `KindT`. It's not your slab.

4. **Arena parameters on fn signatures.** If you write any helper fn that takes an arena (you probably don't need to in Slab 2), use `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>` — never `&'s ScoutArena<'s>`. See §1.2 invariant 5.

5. **Don't derive `Clone` without `Copy` on arena-allocated types.** ATDCX shield. Since every concrete name is `Copy` by design (all fields Copy), derive both.

6. **DAG membership is asymmetric.** `ExternFunctionNameT extends IFunctionNameT with IFunctionTemplateNameT` means both sub-enums get a variant, but the *structs* themselves don't know about each other. Each sub-enum independently lists the concrete names that belong.

7. **Existing Rust cross-references.** Some types outside `names.rs` currently reference specific name types (e.g. `StructTT<'s, 't> { name: IdT<'s, 't> }` in `types/types.rs`). When `IdT` becomes generic in Step 4, these references turn into compile errors. Update each caller to the specific leaf-type version — `StructTT::name` should become `IdT<'s, 't, &'t IStructNameT<'s, 't>>` per Scala (Scala has `IdT[IStructNameT]`).

8. **Commit cadence.** One commit per sub-slab:
   - Concrete structs (maybe split into 2 commits if it's many)
   - Sub-enums
   - IdT + conversions
   - Val companions + bridges
   
   It's fine to collapse some steps if you're confident and the diff stays readable. If you end up with one giant commit, that's OK too — but make sure the build is clean at commit time.

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- Every concrete name struct has real fields and Copy derives.
- Every sub-enum has real variants (no more `_Phantom`) and Copy derives.
- Every concrete name appears as a variant in every sub-enum it transitively extends in Scala (the DAG rule).
- `IdT<'s, 't, T: Copy>` is a real struct with the three fields from §6.3, plus `widen`/`widen_to`/`try_narrow` impl blocks.
- `INameValT` + 14 sub-enum Vals + per-concrete Val structs (where the IDEPFL Shallow/Transient patterns apply) exist and compile.
- `From`/`TryFrom` bridges between sub-enums exist where needed.
- The `TypingInterner` intern methods remain `panic!()` stubs — not your problem.
- `src/typing/typing_interner.rs` unchanged (unless a signature needs a tiny adjustment to reference the now-real types; check with the senior first if so).
- No files outside `names.rs` substantively changed, except:
  - Small call-site updates in `types/types.rs`, `ast/citizens.rs`, `ast/expressions.rs`, etc. to specialize `IdT<'s, 't>` with explicit `T` arguments. Expect ~20 one-line edits.

## When you're stuck

- **Lifetime errors like `'t does not outlive 's`**: you forgot the `where 's: 't` bound on an impl block. Add it.
- **`cannot derive Copy because field X is not Copy`**: the field's type isn't Copy. Check whether the type is supposed to be (`StrI<'s>`, `&'t X<'s, 't>`, `CoordT<'s, 't>` yes; some stub enum/struct maybe no). If a dependency type isn't Copy yet, add Copy derives to *that* type (if it's a name type you own) or wrap the field in `&'t` (if it's a larger type not yours).
- **A Scala field has a type you don't recognize**: search its `pub struct` or `pub enum` in `src/typing/` to see what Rust name maps to it.
- **The DAG membership for a concrete name is ambiguous**: grep the Scala block for `extends X with Y with Z`, split, look each up. If Scala uses a sub-trait not in our 14 Rust sub-enums (rare), ignore it.
- **You're not sure whether a field should be `&'t` or inline-owned**: default to `&'t` for anything that's a name type or a templata. Default to inline for Copy primitives and scout-lifetime things (`StrI<'s>`, `RangeS<'s>`).
- **The senior was wrong about something**: flag it, don't silently "fix" it. Scala parity is the absolute rule per RSMSCPX and NCWSRX shields — if you think the Rust needs to diverge from Scala, ask.

## What you're NOT doing in Slab 2

- Filling `TypingInterner` intern method bodies (separate mini-slab).
- Writing any `impl NameT { fn foo ... }` method bodies (Slab 8+).
- Touching the `CompilerOutputs` struct (Slab 6).
- Touching expressions, envs, templatas (Slabs 3/4/5).
- Changing `KindT`, `CoordT`, or leaf enums/primitives (Slab 1 territory, done).
- Implementing anything in the `Compiler` god struct.
- Deleting `_Phantom` from `KindT` (Slab 3).

Stay in your lane. If you find yourself editing >3 files outside `names.rs`, stop and ask.

## Where to file questions

- **Design** ("is this the right shape for X?"): ask the senior. `quest.md` and the IDEPFL doc are the spec; if they disagree or aren't clear, don't guess.
- **Scala semantics** ("what does this case class do?"): the `/* */` block *is* the Scala source. If that's not enough, the senior has access to the external Scala repo.
- **Hook rejections** ("why is my commit blocked?"): the hook prints a diff. Read it. Almost always it's because you accidentally edited a Scala block or inserted English prose inside `/* */`. Revert that specific edit.
- **Lifetime spaghetti** that `where 's: 't` doesn't fix: ask before hacking around it with `unsafe` or `'static`.

## Final advice

This slab is boring, which is a feature — you can do it quickly and correctly by being patient. The hardest part is the DAG rule (§6.2 / Step 3 above). Draw yourself a scratch table of "concrete name → list of sub-enums it lives in" and keep it next to you as you fill in sub-enums. Build early and often; every new `cargo check` catches mistakes cheaply.

Good luck.
