---
name: slice-placehold
description: Add Rust placeholder stubs below each mig comment, inferred from Scala
tools: [Read, Edit, Grep]
model: haiku
---

You were pointed at a Rust file that has `// mig:` comments (from the slice + slice-rustify steps). Each `// mig:` comment sits right above a commented-out Scala definition.

Your job: add a Rust placeholder stub right below each `// mig:` comment, keeping the `// mig:` comment in place. Infer the signature from the Scala code below. Guessing is fine; it does not need to build.

**IMPORTANT: Completely ignore any existing Rust definitions in the file.** Do not look at them, do not move them, do not delete them. Only look at `// mig:` comments and the Scala code below each one.

**IMPORTANT: Use the EXACT name from the `// mig:` comment.** Do NOT add suffixes like `Mig`, `Placeholder`, `New`, etc. Do NOT rename anything to avoid name collisions with existing code. Name collisions are expected and intentional -- the reconcile step will fix them.

# Step 0: Read the migration policy

Read `FrontendRust/docs/migration/migration-policy.md` (fixed path, single universal file across all passes). Find the row in its **Per-pass values** table whose **Path prefix** column matches the target file's path. If no row matches, STOP and report: "no migration-policy.md row for pass containing <file>; need an architect-approved row before placehold can run." Do not fall back to generic defaults — generic defaults produced the ~32 orphan-fn / wrong-lifetime / missing-interner cleanup burden documented in TL.md for the typing pass.

If a row's column you would need says `(TBD — defer to architect when first file gets migrated)`, STOP and escalate to the architect for that column's value before continuing.

The columns you use here:
 * **Lifetimes** + **Where clause** — applied to every `struct`/`enum`/`trait`/`impl` opening.
 * **Interner type** — added as the first parameter on any `fn` whose Scala body intern-allocates (look for `interner.intern(` / `arena.alloc(` / `new FooT(...)` patterns in the body).
 * **Default slice / map / string types** — for translating `Vector`/`List`/`Array`/`Map`/`Set`/`String` literals in stub signatures.
 * **Sealed-trait policy** — how to emit `// mig: enum FooX`.
 * **Identity equality** — what `derive`s / impl blocks to emit alongside the type.
 * **MustIntern seal** — whether to emit the seal field on interned structs.
 * **Suffix** — applied to every type name (already added at slice-rustify time, but check stubs use the right name).

And these cross-pass conventions from the policy:
 * **Abstract-def dispatcher** — what to emit for enum-impl methods.
 * **`equals` / `hashCode` / `unapply`** — emit marker stubs, not real `fn`s.
 * **`case object` / companion `object`** — how to integrate unit variants into a parent enum.
 * **Arena-classification doc-comment** — `/// Arena-allocated` / `/// Temporary state` / `/// Polyvalue` above every struct/enum.
 * **Default fn skeleton** — `whole-panic` body unless the architect whitelisted the fn for iteration-skeleton (never the agent's decision).

# Walking the file

Walk the `// mig:` comments top to bottom. Each marker is independent — there is no stack-tracking or nesting. Every `// mig: fn` emits a free-floating, module-scope stub. `// mig: impl Foo` markers emit **nothing** (they are left in place as markers only); methods are not nested inside impl blocks at this stage.

**ABSOLUTE RULE: do not emit any `impl` blocks under any circumstances.** Not empty ones (`impl<…> Foo<…> {}`), not multi-method ones, not wrappers around dispatchers. The string `impl<` must not appear in your output. Every method-like Rust definition must be at module scope. The Scala `case class Foo { def bar }` pattern translates to a flat struct + module-scope `pub fn bar(self_: &Foo, …)` — NEVER a wrapped impl. The only exception is `impl PartialEq for Foo`, `impl Hash for Foo`, and `impl TryFrom for Foo` blocks emitted under the equals/hashCode/unapply realization policy (these are trait impls, not inherent impls).

A **separate later pass** (see `.claude/agents/slice-impl-wrap.md`) is responsible for wrapping each module-scope `fn` in its own dedicated `impl<…> Foo<…> { fn … }` block (one impl per method, matching the style in `FrontendRust/src/typing/`). This placehold step must not do that wrapping itself — keep stubs flat.

# What to generate for each mig type

## `// mig: struct Foo`

Generate a `pub struct` with members guessed from the Scala `case class` / `class` parameters below. Apply:
 * The policy's **arena-classification doc-comment** above the struct (`/// Arena-allocated`, `/// Temporary state`, or `/// Polyvalue`).
 * The policy's **lifetimes** on the struct (e.g. `pub struct Foo<'s, 't>`).
 * The policy's **MustIntern seal** field if this struct is classified as interned (`pub _must_intern: MustIntern,`).
 * The policy's **identity-equality directive** as a `#[derive(...)]` line above the struct (or an `impl PartialEq` block below, if identity is `ptr::eq`-based). For non-interned types, derive `PartialEq, Eq, Hash`; for interned, emit the `impl PartialEq for Foo` via `ptr::eq` below the struct.
 * Field types translated via the policy's **default collection types** + **string type** sections.

### Bare-placeholder mode (when scaffolding a not-yet-migrated cross-pass type)

When TL (not this agent) is scaffolding a type whose upstream/downstream module isn't migrated yet (e.g. an H-side `IdH` or an exposed-but-not-yet-fleshed-out I-side `IdI`), the expected shape is a **bare placeholder with PhantomData absorbing the type/lifetime params**:

```rust
// mig: case class IdI[+R <: IRegionsModeI, +T <: INameI[R]]
pub struct IdI<'s, 't, R, T>(std::marker::PhantomData<&'s &'t (R, T)>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
```

Key shape rules for bare placeholders:
 * Use the **canonical Scala name** (no `_full`/`_placeholder`/`_stub` suffix — those would trip NRDX). The placeholder IS the in-progress migration shape; the full shape lives in the adjacent `/* … */` Scala block per SCPX.
 * **Type/lifetime params preserved** from the Scala signature; absorb them all in a single `PhantomData` tuple field. SPDMX exception D (extended) covers `_marker: PhantomData<…>` fields on otherwise-empty placeholders — they don't count as novel data per SPDMX S-3.
 * **No arena-classification doc-comment.** Bare placeholders aren't yet classified; the `/// Polyvalue` / `/// Temporary state` / `/// Arena-allocated` line gets added only when the type gains real fields. TFITCX permits unclassified bare placeholders.
 * **TODO comment** pointing at the upstream module that will populate the fields — gives reviewers and JR a clear breadcrumb for when to flesh it out.
 * **No `#[derive]`** until the type has real fields; derives are added with the field populace.

The bare-placeholder mode is TL/architect-only — the agent emits the full-shape struct (with fields guessed from Scala) by default. TL switches to bare-placeholder mode by hand-editing the emitted stub.

## `// mig: enum Foo`

Generate a `pub enum Foo<'s, 't> { /* variants */ }` per the policy's **sealed-trait policy**. For `enum-with-arena-refs`, each variant looks like `Variant1(&'t Variant1Payload<'s, 't>)`. For `case object Bar` inside the trait, emit `Bar(())` per the policy's **case object policy** (unit variant, no separate struct).

Above the enum: emit the policy's arena-classification doc-comment + the policy's derive directive (Polyvalue enums get `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`).

## `// mig: impl Foo`

Emit **nothing for the impl wrapper itself** — leave the `// mig: impl Foo` marker as a comment-only marker; do not generate any `impl` block (no opener, no closer, no empty `{}`).

However, if `Foo` corresponds to a previously-emitted `// mig: enum Foo` whose Scala `sealed trait` body has abstract `def`s, emit a **module-scope dispatcher fn** for each one per the policy's "Abstract-def dispatcher policy":

```rust
pub fn method(this: &Foo<'s, 't>, /* args from policy */) -> /* return from policy */ {
    match this {
        _ => panic!("Unimplemented: Foo::method dispatch"),
    }
}
```

The dispatcher is module-scope (the `impl`-wrapping pass will later wrap it in `impl<...> Foo<...> { ... }`).

**Do NOT emit any `/* Guardian: disable-all */` annotation or other Guardian directives on the dispatcher.** Earlier versions of this spec required one because dispatchers have no 1:1 Scala counterpart (Scala's virtual call is the counterpart, realized differently in Rust). But emitting `Guardian:` text from the agent conflicts with NAGDX (No Adding Guardian Directives). When NNDX fires on the new dispatcher fn, JR escalates to TL via `for-tl.md`; TL evaluates and (if approved) adds the directive manually — TL/architect are exempt from NAGDX.

## `// mig: trait Foo`

Generate an empty `pub trait Foo {}` or trait with method stubs. Apply the policy's lifetimes. Only used when the policy's sealed-trait policy is `trait-with-impls`.

## `// mig: fn foo`

Always emit a module-scope stub: `pub fn foo(…) -> … { panic!("Unimplemented: foo"); }`. Do NOT nest inside any impl block — methods originally inside a Scala `class` body become free-floating module-scope `pub fn`s here. If the Scala def takes `self`/`this` implicitly (i.e. it was an instance method on a class), translate that to an explicit first parameter named `self_` (or appropriately typed) — do NOT use `&self`, since there is no surrounding impl.

Signature inference:
 * Translate Scala types via the policy's collection/string rules.
 * **Declare any policy-lifetime params the signature references.** Walk the emitted parameter and return types; for each policy-lifetime (e.g. `'s`, `'t`, `'h`, `'i`) that appears anywhere in the signature — most commonly via the policy's **String type** (e.g. `StrI<'h>`), **Default slice** (e.g. `&'t [X]`), or **Interner type** — add it to the fn's generic parameter list: `pub fn foo<'h>(…)`. Without this, the fn body fails to compile with `E0261: use of undeclared lifetime name`. Order: same as the policy's **Lifetimes** column (e.g. `'s, 't` not `'t, 's`). Skip lifetimes the policy declares but the signature doesn't actually mention.
 * If the Scala body intern-allocates (look for `interner.intern(` / `arena.alloc(` / `new FooT(...)` patterns in the body), thread the policy's **Interner type** as the first parameter, named `interner`. Above the fn emit:
   ```rust
   // Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass
   // arena-allocates where Scala used GC.
   ```
 * If the Scala code is `test("…")`, emit `#[test]` and `panic!("Unmigrated test: foo");` at module scope (tests never go inside an impl).
 * **Scala method overloads.** When two `// mig: fn` markers translate to the same snake_case name (Scala overloads — same name, different parameter lists), emit a stub for **each** marker. Do NOT skip the second marker just because a stub with that name was already emitted for an earlier overload — Rust lacks overloading, so the name collision is real and *expected* (the reconcile step / TL disambiguates with a `_<suffix>` per SPDMX-S). Skipping the second marker leaves a markerless Scala `/* */` block that compiles fine while gated but silently lacks a stub, which surfaces as a missing-method NNDX escalation during body migration (e.g. the 5-arg `translateName` in `instantiator.rs`). Every `// mig: fn` marker gets its own stub, full stop.

If the `// mig: fn` is suffixed `(realized-by-impl PartialEq)`, `(realized-by-impl Hash)`, or `(realized-by-TryFrom)`, do NOT emit a `pub fn`. Instead emit a marker stub per the policy's equals/hashCode/unapply policy:

```rust
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FooT` below.)
```

(Leave the Scala `/* */` block in place after the marker, as usual.)

## `// mig: const FOO`

Generate a `const` with a placeholder value.

# Guidelines

 * **This step should ONLY add lines. It must NEVER delete or replace any existing lines.** All comments (both `/* */` blocks and `// mig:` comments) must remain exactly as they are. The Scala code inside comments must not be touched, moved, or removed. If you find yourself deleting a line, you are doing it wrong.
 * **Ignore all existing Rust code in the file.** Only operate on `// mig:` comments.
 * **Keep every `// mig:` comment in place.** Add the Rust stub right below it; do not remove or replace the comment.
 * Infer signatures from the Scala code below each mig comment. Guessing is fine.
 * Convert Scala names to snake_case for function names.
 * Convert Scala types to Rust equivalents **via the policy's collection/string sections** (NOT via generic `String → &str` / `Vector → Vec` defaults, which are wrong for arena passes). `Option[T]` → `Option<T>` and `Unit` → `()` are always fine.
 * For `trait`, generate an empty `pub trait Foo {}`. For `impl`, emit nothing (leave the marker as a comment).

# Restrictions

 * Do not build or test. Do not run `cargo build`, `cargo run`, or `cargo test`.
 * Do not reorder Scala comments. They must stay in their original order.
 * DO NOT use sed -i or other scripts to do this for you. Do it manually.

# When done

Say "done" when you're done modifying the code.
