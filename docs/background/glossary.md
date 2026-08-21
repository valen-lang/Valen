# Glossary

In this codebase, we don't like jargon.

We will only use terms known to general programmers, plus the terms listed below.

 * AHT: Abstract High-level Tree, it's the input to the typing pass. Sometimes also called by deprecated terms "postparsed" and "scoutput".
 * define-compile: Compiling a denizen's own definition. For example, define-compiling `foo<T>(x T)` makes a placeholder `foo$T` and compiles foo's body in terms of that placeholder. A define-compile never waits on another denizen's define-compile — it only resolve-compiles the denizens it mentions.
 * Denizen: A function, struct, interface, or impl.
 * resolve-compile: Within a denizen's define-compile, compiling a call site by locally solving the *callee's* rules in terms of the caller's placeholders. For example, inside `foo<T>(x T)`'s define-compile, the call `bar(x)` (to `func bar<B>(b B)`) is resolve-compiled: a local solve of bar's rules in terms of `foo$T`, yielding bar's prototype. It does not define-compile bar.
 * Bound: On a generic denizen, we can declare e.g. `where func drop(T)void` or `where implements(T, IMyInterface)`. These are bounds.
 * Satisfier: When a use-site is resolving a generic denizen with bounds, the use-site needs to find things (functions for `exists`, impls for `implements`) that satisfy those bounds. Those are satisfiers.

Explicitly don't use these words:

 * Dynamism

This file/process is new, so please eagerly flag anything that you're unsure of, so we can build up this file.

## Proposed Additions
