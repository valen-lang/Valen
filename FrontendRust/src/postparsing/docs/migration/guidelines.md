---
g_read_when: "Read when migrating post-parser code from Scala."
g_auto_load_when_editing:
  - FrontendRust/src/postparsing/*.rs
---

# PostParser Migration Guidelines

Look at the below allowable differences.

# Rust vs Scala Allowable Differences

 * Rust functions can have these extra parameters:
    * `Interner`
    * `Keywords`
    * `PostParser`
 * It's okay if the Rust version doesn't have these parameters:
    * `ExpressionScout` (the function receives the full `PostParser` instead, which contains it)
    * `FunctionScout` (the function receives the full `PostParser` instead, which contains it)
 * Rust can intern things more than Scala if it decides to. I'm allowing Rust to intern more.
 * Scala `vimpl` is the same thing as a Rust `panic!`.
 * When Rust has a `panic!`, ignore any difference there, even if Scala has fully implemented code there. Since we're mid-migration, I allow them to put `panic!` placeholders in. DO NOT mention these differences.
 * Rust doesn't need to wrap its code in `Profiler.frame(() => { ... })` like Scala does.
 * Rust doesn't need to create `evalRange` closure functions like Scala does.
 * Rust can suffix its local variables (and arguments) with e.g. `_p` for parser, `_pe` for parser expression, `_pt` for parser templex, `_s` for postparser, `_se` for postparser expression, `_st` for postparser templex, and so on.
 * Rust variables/arguments/fields should use snake case (like `self_uses`) instead of Scala's camel case (like `selfUses`).
 * Scala uses `U` methods like `U.extract`, Rust doesn't need to use those, Rust can use stdlib equivalents.
