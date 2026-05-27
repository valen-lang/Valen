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

**Scala method overloads — emit a stub per marker even when names collide.** When two `// mig: fn` markers translate to the same snake_case name (Scala overloads: same name, different parameter lists), emit a stub for **each** marker. Do NOT skip the second marker just because a stub with that name was already emitted — Rust lacks overloading, so the collision is real and *expected* (reconcile/TL disambiguates with a `_<suffix>` per SPDMX-S). Skipping it leaves a markerless Scala `/* */` block that compiles fine while gated but silently lacks a stub, surfacing later as a missing-method NNDX escalation during body migration (e.g. the 5-arg `translateName` in `instantiator.rs`). Every `// mig: fn` marker gets its own stub, full stop.

# Step 0: Read the migration policy

Before emitting anything, read the central migration policy at `FrontendRust/docs/migration/migration-policy.md` and find the values for this pass (identified by the target file's source dir, e.g. `src/typing/` → typing). If the pass has no values in the sections you need, STOP and report "pass `<name>` has no values in `FrontendRust/docs/migration/migration-policy.md`; add them before placehold can run." Do not fall back to generic defaults — generic defaults produced the ~32 orphan-fn / wrong-lifetime / missing-interner cleanup burden documented in TL.md for the typing pass.

Read the relevant pass's values in full. The sections you use here:
 * **Lifetimes** + default `where` clause — applied to every `struct`/`enum`/`trait`/`impl` opening.
 * **Interner type** — added as the first parameter on any `fn` whose Scala body intern-allocates (look for `interner.intern(` / `arena.alloc(` patterns in the Scala body).
 * **Default collection types** — for translating `Vector`/`Map`/`Set` literals in stub signatures.
 * **String type** — for translating `String` in stub signatures.
 * **Sealed-trait policy** — how to emit `// mig: enum Foo`.
 * **Abstract-def dispatcher policy** — what to emit for enum-impl methods.
 * **`equals`/`hashCode`/`unapply` policy** — emit marker stubs, not real `fn`s.
 * **Identity equality policy** — what `derive`s / impl blocks to emit alongside the type.
 * **`case object` policy** — how to integrate unit variants into a parent enum.
 * **`MustIntern` seal policy** — whether to emit the seal field on structs.
 * **Arena-classification doc-comment** — emit above every struct/enum.
 * **Default fn skeleton** — `whole-panic` body unless the policy whitelists the fn for iteration-skeleton.

# Walking the file

Walk the `// mig:` comments top to bottom. Each marker is independent — there is no stack-tracking or nesting. Every `// mig: fn` emits a free-floating, module-scope stub; methods are not nested inside impl blocks at this stage.

A **separate later pass** (see `.claude/agents/slice-impl-wrap.md`) is responsible for wrapping each module-scope `fn` in its own dedicated `impl<…> Foo<…> { fn … }` block (one impl per method, matching the style in `FrontendRust/src/typing/`). This placehold step must not do that wrapping itself — keep stubs flat.

# What to generate for each mig type

## `// mig: struct Foo`

Generate a `pub struct` with members guessed from the Scala `case class` / `class` parameters below. Apply:
 * The policy's **arena-classification doc-comment** above the struct (`/// Arena-allocated`, `/// Temporary state`, or `/// Polyvalue`).
 * The policy's **lifetimes** on the struct (e.g. `pub struct Foo<'s, 't>`).
 * The policy's **MustIntern seal** field if this struct is classified as interned (`pub _must_intern: MustIntern,`).
 * The policy's **identity-equality directive** as a `#[derive(...)]` line above the struct (or an `impl PartialEq` block below, if identity is `ptr::eq`-based). For non-interned types, derive `PartialEq, Eq, Hash`; for interned, emit the `impl PartialEq for Foo` via `ptr::eq` below the struct.
 * Field types translated via the policy's **default collection types** + **string type** sections.

## `// mig: enum Foo`

Generate a `pub enum Foo<'s, 't> { /* variants */ }` per the policy's **sealed-trait policy**. For `enum-with-arena-refs`, each variant looks like `Variant1(&'t Variant1Payload<'s, 't>)`. For `case object Bar` inside the trait, emit `Bar(())` per the policy's **case object policy** (unit variant, no separate struct).

Above the enum: emit the policy's arena-classification doc-comment + the policy's derive directive (Polyvalue enums get `#[derive(PartialEq, Eq, Hash, Clone, Copy)]`).

## Sealed-trait dispatcher fns

For a `// mig: enum Foo` that came from a Scala `sealed trait` with abstract `def`s, emit a **module-scope dispatcher fn** for each abstract def per the policy's "Abstract-def dispatcher policy":

```rust
/* Guardian: disable-all */
pub fn method(this: &Foo<'s, 't>, /* args from policy */) -> /* return from policy */ {
    match this {
        _ => panic!("Unimplemented: Foo::method dispatch"),
    }
}
```

The dispatcher is module-scope (the `impl`-wrapping pass will later wrap it in `impl<...> Foo<...> { ... }`, anchored on the `// mig: enum Foo` marker). The `/* Guardian: disable-all */` annotation is mandatory — these dispatchers have no 1:1 Scala counterpart.

## `// mig: trait Foo`

Generate an empty `pub trait Foo {}` or trait with method stubs. Apply the policy's lifetimes. Only used when the policy's sealed-trait policy is `trait-with-impls`.

## `// mig: fn foo`

Always emit a module-scope stub: `pub fn foo(…) -> … { panic!("Unimplemented: foo"); }`. Do NOT nest inside any impl block — methods originally inside a Scala `class` body become free-floating module-scope `pub fn`s here. If the Scala def takes `self`/`this` implicitly (i.e. it was an instance method on a class), translate that to an explicit first parameter named `self_` (or appropriately typed) — do NOT use `&self`, since there is no surrounding impl.

Signature inference:
 * Translate Scala types via the policy's collection/string rules.
 * If the Scala body intern-allocates (look for `interner.intern(` / `arena.alloc(` / `new FooT(...)` patterns in the body), thread the policy's **Interner type** as the first parameter, named `interner`. Above the fn emit:
   ```rust
   // Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass
   // arena-allocates where Scala used GC.
   ```
 * If the Scala code is `test("…")`, emit `#[test]` and `panic!("Unmigrated test: foo");` at module scope (tests never go inside an impl).

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
