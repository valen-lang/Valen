---
name: slice-impl-wrap
description: "Wrap each module-scope fn in its own dedicated impl block, using `// mig: struct Foo` / `// mig: enum Foo` markers to determine the receiver type"
tools: [Read, Edit, Grep]
model: haiku
---

You were pointed at a Rust file that already went through slice-start → slice-rustify → slice-placehold. Every Scala definition has a `// mig:` marker above a placeholder stub. `fn` stubs are currently at module scope; your job is to wrap each one in its own dedicated `impl<…> Foo<…> { fn … }` block (one impl per method), matching the style used in `FrontendRust/src/typing/`.

This is the inverse of the "open impl, emit many methods inside, close impl" pattern — instead, we use **one impl block per method**. This gives each method an independently-editable wrapper and is easier for downstream agents to reason about.

# Step 0: Read the pass migration policy

Read the central migration policy at `FrontendRust/docs/migration/migration-policy.md` and find the values for this pass (identified by the target file's source dir, e.g. `src/typing/` → typing). If the pass has no values there, STOP and report "pass `<name>` has no values in `FrontendRust/docs/migration/migration-policy.md`." Otherwise, read its **lifetimes** and **where-clauses** to apply to each impl opener.

# Algorithm

Walk the `// mig:` markers top to bottom, maintaining a **current receiver type** — the name of the most recent `// mig: struct Foo` / `// mig: enum Foo` marker you've crossed. Initially the receiver is `None`.

For each `// mig:` marker:

1. **`// mig: struct Foo` / `// mig: enum Foo`** — update the current receiver to `Foo`. Leave the existing stub; emit nothing here.
2. **`// mig: trait Foo`** — do nothing, and do NOT set a receiver (a plain trait's fns are not impl-wrapped this way). Leave the existing stub.
3. **`// mig: fn foo`** — find the module-scope `pub fn foo(...)` stub directly below this marker. If the current receiver is `Some(Foo)`, wrap that stub in its own dedicated impl block:
   ```rust
   impl<'s, 't> Foo<'s, 't>
   where /* policy's where-clause */
   {
       pub fn foo(/* same params */) -> /* same return */ {
           /* same body */
       }
   }
   ```
   - Use the policy's lifetimes and where-clause on every impl opener.
   - The closing `}` of the impl goes immediately after the closing `}` of the wrapped fn.
   - If the fn took an explicit `self_: &Foo<…>` first parameter (added by slice-placehold), convert it to `&self` inside the impl.
   - If the current receiver is `None` (i.e. the fn is logically module-scope — e.g. a free function not inside any Scala `class`/`object`), leave the fn at module scope and emit nothing.
4. **Sealed-trait dispatchers** (the `/* Guardian: disable-all */ pub fn method(this: &Foo, ...)` stubs slice-placehold emits for a `// mig: enum Foo` from a sealed trait) — wrap each in its own `impl<...> Foo<...> { fn method(&self, ...) ... }` block too. Preserve the `/* Guardian: disable-all */` annotation immediately above the impl.

# Scope-end heuristic

The Scala class body has a closing `}` inside its `/* */` block. When you cross it, reset the current receiver to `None`. The next `// mig: fn` after that point is at module scope until another `// mig: struct Foo` / `// mig: enum Foo` re-opens a receiver.

# Guidelines

 * **This step rewrites existing module-scope `pub fn` stubs in place.** It must not delete any `// mig:` markers, Scala `/* */` blocks, or struct/enum/trait definitions.
 * Each impl block wraps exactly one method.
 * Preserve the body of each fn verbatim (usually just `panic!("Unimplemented: foo");`).
 * Apply the policy's lifetimes and where-clause to every impl opener.
 * **Drop per-fn `<'s, 't>` generics** when wrapping. If the original module-scope stub was `pub fn foo<'s, 't>(...)`, strip the `<'s, 't>` from the fn signature — the impl block's `impl<'s, 't> Foo<'s, 't>` already provides them, and duplicating them produces shadowing warnings (per TL.md §143). Keep any *additional* per-fn generics that don't appear on the impl (e.g. `<T: SomeTrait>`).
 * Preserve any doc comments / `/* Guardian: disable-all */` annotations attached to the fn — move them above the impl block (the impl block becomes the new outer scope).
 * Trait impls and `impl PartialEq for Foo { ... }` blocks emitted by slice-placehold for identity equality are left untouched.
 * Tests (`#[test] fn ...`) stay at module scope — never wrap a test in an impl.

# Restrictions

 * Do not build or test.
 * Do not reorder `// mig:` markers or Scala comments.
 * **NEVER delete, modify, or move any Scala code inside `/* */` blocks.** Your only job is to wrap existing module-scope Rust `pub fn` stubs in `impl<...> Foo<...> { ... }` blocks. The `/* */` blocks containing the original Scala code must remain exactly in place — same content, same order. If your edit removes or reorders any Scala line from any `/* */` block, you have broken the SCPX audit trail.
 * Do not use sed -i or scripts.

# When done

Say "done" and report the number of fns wrapped and the number left at module scope (with a brief reason for each module-scope fn).
