---
name: slice-rustify
model: composer-1.5
description: Translate Scala mig slice comments to Rust mig slice comments
---

You were pointed at a Rust file with some commented Scala code that has "Scala mig slice comments".

I'd like you to translate every Scala mig slice comment to a "Rust mig slice comment". This means making them look more Rust-ish than Scala-ish.

# Functions

Functions become `fn` and snake-case. For example, `def functionName` becomes `fn function_name`.

## Overloaded functions (same name, different parameters)

Scala allows multiple `def` with the same name (overloads). Just translate each one to `fn` as normal, leaving them with the same name. Do not try to disambiguate them. It's okay if these Rust functions have the same name.

# Classes

A class Scala mig comment becomes two Rust mig comments: one struct and one impl. For example, `class PostParserVariableTests` -> `struct PostParserVariableTests` and `impl PostParserVariableTests`.

# Case classes

A case class becomes `struct` and `impl` (same as class).

# Values

`val FOO` becomes `const FOO`.

# test("...")

For `test("Regular variable")`, translate to `fn regular_variable` (snake_case from the test name string).

# Example 1

`// mig: class PostParserVariableTests` becomes `// mig: struct PostParserVariableTests` and `// mig: impl PostParserVariableTests`. `// mig: def compileForError` becomes `// mig: fn compile_for_error`. `// mig: test("Regular variable")` becomes `// mig: fn regular_variable`. `// mig: test("Type-less local has no coord rune")` becomes `// mig: fn type_less_local_has_no_coord_rune`.

# Example 2

`// mig: class FileCoordinateMap` becomes `// mig: struct FileCoordinateMap` and `// mig: impl FileCoordinateMap`. `// mig: def putPackage` becomes `// mig: fn put_package`. `// mig: def map` becomes `// mig: fn map`. `// mig: def flatMap` becomes `// mig: fn flat_map`.

# Example 3

`// mig: def TEST_TLD` becomes `// mig: fn test_tld`. `// mig: def BUILTIN` becomes `// mig: fn builtin`. `// mig: def internal` becomes `// mig: fn internal`.

# Restrictions

 * Your job is *not* to make it build. Do not run `cargo build`, `cargo run`, or `cargo test`.
 * You cannot reorder the old scala comments relative to each other. They must be in the same order as they were before.

# When done

Say "done" when you're done modifying the code.
