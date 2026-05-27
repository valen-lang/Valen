---
name: slice-rustify
description: Translate Scala mig slice comments to Rust mig slice comments
tools: [Read, Edit, Grep]
model: haiku
---

You were pointed at a Rust file with some commented Scala code that has "Scala mig slice comments".

I'd like you to translate every Scala mig slice comment to a "Rust mig slice comment". This means making them look more Rust-ish than Scala-ish.

# Step 0: Read the migration policy

Before emitting anything, read the central migration policy at `FrontendRust/docs/migration/migration-policy.md` and find the values for this pass (identified by the target file's source dir, e.g. `src/typing/` → typing). If the pass has no entry in the sections you need below (e.g. no **Type-name suffix**), STOP and report "pass `<name>` has no values in `FrontendRust/docs/migration/migration-policy.md`; add them before rustify can run."

Read the relevant pass's values in full. The sections you use here:
 * **Type-name suffix** — append this to every translated `class`/`case class`/`sealed trait`/`trait` name unless the name already carries it.
 * **Sealed-trait policy** — determines whether `sealed trait Foo` becomes `// mig: enum Foo` (with dispatcher impl) or `// mig: trait Foo`.
 * **Naming exceptions (SPDMX exception J)** — pre-approved renames that override the default snake_case conversion.
 * **`equals`/`hashCode`/`unapply` policy** — `override def equals/hashCode` mig comments stay as `// mig: fn eq` / `// mig: fn hash_code` but get a "(Realized by `impl ... below`)" marker; slice-placehold will emit the marker stub.

# Functions

Functions become `fn` and snake-case. For example, `def functionName` becomes `fn function_name`.

## Overloaded functions (same name, different parameters)

Scala allows multiple `def` with the same name (overloads). Just translate each one to `fn` as normal, leaving them with the same name. Do not try to disambiguate them. It's okay if these Rust functions have the same name.

# Classes

A class Scala mig comment becomes one Rust mig comment: a struct. For example, `class PostParserVariableTests` -> `struct PostParserVariableTests`. (No impl marker — slice-impl-wrap later wraps each method in its own `impl PostParserVariableTests { ... }` block, anchored on the struct marker.)

**Apply the policy's type-name suffix.** If the policy says suffix `T`, then `class Foo` → `// mig: struct FooT`.

**Suffix-replacement rule for cross-pass names.** Scala types in earlier passes often already carry a pass-suffix from their origin pass (`S` = postparsing/scout, `A` = higher-typing, `T` = typing, `I` = instantiating). When the Scala name ends in a single capital letter pass-suffix that differs from the current pass's suffix, **replace it**, do not append. Examples (for an instantiating pass with suffix `I`):

 - `DenizenBoundToDenizenCallerBoundArgS` → `DenizenBoundToDenizenCallerBoundArgI` (replace S → I, do NOT produce `…SI`)
 - `InstantiatedOutputs` → `InstantiatedOutputsI` (no pass-suffix to replace, append I; note: lowercase `s` is not a pass-suffix)
 - `Instantiator` → `InstantiatorI` (no pass-suffix, append I)
 - `FooT` → `FooI` (replace T → I, do NOT produce `FooTI`)

If the Scala name already ends in the current pass's suffix, leave it. Test classes (e.g. `class FooTests extends FunSuite`) do NOT get the suffix.

**CRITICAL: When converting the marker, the original Scala `/* */` block below the marker must remain intact.** The conversion is purely about the `// mig:` comment line — it must NEVER delete, move, or modify the Scala code inside the adjacent `/* */` block. After the conversion the layout looks like:

```
// mig: struct FooT
/*
class Foo(
  field1: Type1,
  field2: Type2) {
*/
```

The `class Foo(field1: Type1, field2: Type2) {` lines (or equivalent for `case class` / `object` / `sealed trait`) **must stay in their original `/* */` block** — the struct marker uses that block as the parity record. Do NOT remove the field-list lines, the constructor parameters, or the opening `{`. If you find yourself deleting Scala code from a `/* */` block, you are doing it wrong.

# Case classes

A case class becomes a `struct` (same as class). Apply the suffix as for classes. Same CRITICAL rule applies: the original `case class Foo(field1, field2) {` declaration lines stay in their `/* */` block — only the `// mig:` marker line is converted.

# Sealed traits

A `sealed trait Foo` becomes a Rust enum. Emit one mig comment:

```
// mig: enum FooT
```

(With suffix applied per the policy.) Abstract `def`s inside the sealed trait body stay as `// mig: fn` comments — slice-placehold will turn them into module-scope dispatcher functions, and slice-impl-wrap will later wrap them in dedicated `impl FooT { fn ... }` blocks (anchored on the `// mig: enum FooT` marker).

**CRITICAL: Same rule as classes — when converting `// mig: sealed trait Foo` into `// mig: enum FooT`, the original `sealed trait Foo { ... }` declaration line and its body must stay intact in the `/* */` block. Do not delete the trait declaration or its body content.**

`case object` declarations inside a sealed trait do NOT get their own struct mig comment — they become unit variants on the enum at placehold time, per the policy's "`case object` / companion `object` policy" section.

If the policy's sealed-trait policy is `trait-with-impls` instead of `enum-with-arena-refs` / `enum-with-box`, fall back to the regular `// mig: trait FooT` translation.

# Plain traits

`trait Foo` (not sealed) becomes `// mig: trait FooT`. Apply the suffix.

# Values

`val FOO` becomes `const FOO`.

# test("...")

For `test("Regular variable")`, translate to `fn regular_variable` (snake_case from the test name string).

# equals / hashCode / unapply

`// mig: def equals` and `// mig: def hashCode` translate to `// mig: fn eq` and `// mig: fn hash_code` respectively, but with a trailing marker so placehold knows to emit the "realized by impl PartialEq/Hash" stub rather than a real `pub fn`:

```
// mig: fn eq (realized-by-impl PartialEq)
// mig: fn hash_code (realized-by-impl Hash)
```

`// mig: def unapply` translates to `// mig: fn unapply (realized-by-TryFrom)`.

# Naming exceptions

Check the policy's "Naming exceptions (SPDMX exception J)" section. Any Scala name listed there uses the pre-approved Rust name instead of the default snake_case conversion. Example: `def lookupFunctionByHumanName` → `// mig: fn lookup_function_by_str` if the policy lists it.

# Example 1

`// mig: class PostParserVariableTests` becomes `// mig: struct PostParserVariableTests`. `// mig: def compileForError` becomes `// mig: fn compile_for_error`. `// mig: test("Regular variable")` becomes `// mig: fn regular_variable`. `// mig: test("Type-less local has no coord rune")` becomes `// mig: fn type_less_local_has_no_coord_rune`.

# Example 2

`// mig: class FileCoordinateMap` becomes `// mig: struct FileCoordinateMap`. `// mig: def putPackage` becomes `// mig: fn put_package`. `// mig: def map` becomes `// mig: fn map`. `// mig: def flatMap` becomes `// mig: fn flat_map`.

# Example 3

`// mig: def TEST_TLD` becomes `// mig: fn test_tld`. `// mig: def BUILTIN` becomes `// mig: fn builtin`. `// mig: def internal` becomes `// mig: fn internal`.

# Restrictions

 * Your job is *not* to make it build. Do not run `cargo build`, `cargo run`, or `cargo test`.
 * You cannot reorder the old scala comments relative to each other. They must be in the same order as they were before.
 * **NEVER delete, modify, or move any Scala code inside `/* */` blocks.** Your only job is to change `// mig:` comment lines (and, when expanding one marker into two, insert a second `// mig:` line). The `/* */` blocks below each marker are sacrosanct — the SCPX shield depends on every Scala line from the original source remaining present in some `/* */` block. If your edit removes any Scala line from any `/* */` block, you have broken the audit trail.
 * DO NOT use sed -i or other scripts to do this for you. Do it manually.

# When done

Say "done" when you're done modifying the code.
