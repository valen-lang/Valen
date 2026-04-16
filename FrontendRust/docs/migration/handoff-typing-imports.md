# Handoff: Add `use` Statements To `src/typing/**/*.rs` Files

## Who this is for

You're a junior engineer picking up a well-scoped mechanical task in a Scala→Rust compiler-migration project. This doc gives you everything you need. Read the whole thing before touching code.

## 90-second project context

- **Sylvan** is the overall repo. `Frontend/` is a Scala compiler (the original, authoritative source). `FrontendRust/` is a Rust reimplementation being migrated piece-by-piece from Scala.
- The migration uses a **"slice pipeline"** that inserts `// mig:` marker comments above Scala definitions and generates Rust placeholder stubs. The Scala source stays in the file as a commented-out `/* ... */` block above each corresponding Rust stub, so a reviewer can always see the original.
- We are focused on **the typing pass** (`FrontendRust/src/typing/`). The typing pass does type inference and type checking.
- There is a design doc at `/Volumes/V/Sylvan/quest.md` describing the three-arena lifetime model for this pass. **Read the "Status" section at the top.** Every typing type now carries lifetime parameters like `<'s, 't>` or `<'s, 'ctx, 't>`. (You don't need to understand the lifetime model in depth to do this task — but do skim §1.1–1.5 for vocabulary.)

## Your task in one sentence

**Add `use` statements to the top of each `.rs` file under `src/typing/` so that its references to other typing types actually resolve.**

## Why this is needed

The slice pipeline copied the Scala `import` lines into the `/* ... */` block at the top of each file (they live inside a Scala comment, not as Rust code). Rust doesn't see those — it needs real `use` statements. So right now, files reference types like `IdT`, `KindT`, `CoordT`, `RangeS` without having imported them, and the compiler yells:

```
error[E0425]: cannot find type `IdT` in this scope
error[E0425]: cannot find type `KindT` in this scope
error[E0425]: cannot find type `CoordT` in this scope
```

Before the handoff, the codebase had ~540 compile errors. Of those, roughly 350 are "cannot find type X in this scope" — all fixable by adding `use` lines. That's your job.

## Current state (what's already done — don't redo)

1. **Lifetime parameters** on every `pub struct`/`enum`/`trait` in `src/typing/**` have already been corrected (most take `<'s, 't>`; compilers take `<'s, 'ctx, 't>`). Don't touch type signatures.
2. **Module declarations** in `src/typing/mod.rs` and the sub-`mod.rs` files are complete. You don't need to add `pub mod foo;` anywhere.
3. `src/typing/types/types.rs` and `src/typing/names/names.rs` were done manually and already have most of what they need — they're the "leaves" of the dependency graph. **Skip them** in your first pass.
4. `hinputs_t.rs` is partially stubbed and uses fully-qualified paths (`crate::typing::names::names::IdT`) rather than `use` statements — leave it alone.

## What "done" looks like

After your work:

1. Running `cargo check --lib` from `FrontendRust/` produces **zero** `E0425 "cannot find type"` errors for standard typing types (IdT, KindT, CoordT, INameT, ITemplataT, etc.).
2. Other error classes (duplicate `fn equals`/`hash_code`, mismatched types, missing trait impls) are **unchanged** — you haven't fixed those and you haven't made them worse.
3. No new types were introduced, no existing types renamed, no `// mig:` comments altered, no `/* ... */` Scala blocks altered.

## The pattern — worked example

### Example file: `src/typing/ast/ast.rs`

Open the file. The top looks like this:

```rust
/*
package dev.vale.typing.ast

import dev.vale.highertyping.FunctionA
import dev.vale.typing.names._
import dev.vale.typing.templata.FunctionTemplataT
import dev.vale.{PackageCoordinate, RangeS, vassert, vcurious, vfail}
import dev.vale.typing.types._
...
*/
// mig: struct ImplT
pub struct ImplT<'s, 't> {
    pub templata: ImplDefinitionTemplataT<'s, 't>,
    pub instantiated_id: IdT<'s, 't>,
    ...
}
```

The `/* ... */` block at the top is the **Scala source comment** — it's inert, don't touch it. But it's a useful reference: it tells you what the file USES. Translate each Scala import into a Rust `use`:

- `import dev.vale.highertyping.FunctionA` → `use crate::higher_typing::ast::FunctionA;`
- `import dev.vale.typing.names._` → `use crate::typing::names::names::*;`
- `import dev.vale.typing.templata.FunctionTemplataT` → `use crate::typing::templata::templata::FunctionTemplataT;`
- `import dev.vale.typing.types._` → `use crate::typing::types::types::*;`
- `import dev.vale.{RangeS, ...}` → `use crate::utils::range::RangeS;`

Add those as a block **right after the closing `*/`** of the Scala header comment and **before the first `// mig:`**:

```rust
/*
... scala imports ...
*/
use crate::higher_typing::ast::FunctionA;
use crate::interner::StrI;
use crate::parser::PackageCoordinate;  // or wherever PackageCoordinate actually lives — verify!
use crate::postparsing::names::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::utils::range::RangeS;
// mig: struct ImplT
pub struct ImplT<'s, 't> {
    ...
```

That's it. Save, move to the next file.

## Rough mapping: Scala package → Rust module path

Use these as starting points; verify each with `Grep` before committing.

| Scala import | Rust `use` | Notes |
|---|---|---|
| `dev.vale.typing.types._` | `use crate::typing::types::types::*;` | KindT, CoordT, OwnershipT, etc. |
| `dev.vale.typing.names._` | `use crate::typing::names::names::*;` | IdT, INameT, all the `*NameT` structs |
| `dev.vale.typing.ast._` | `use crate::typing::ast::ast::*;` AND `use crate::typing::ast::citizens::*;` AND `use crate::typing::ast::expressions::*;` | Split into three in Rust |
| `dev.vale.typing.templata._` | `use crate::typing::templata::templata::*;` | ITemplataT, heavy templatas |
| `dev.vale.typing.env._` | `use crate::typing::env::environment::*;` AND `use crate::typing::env::function_environment_t::*;` | |
| `dev.vale.typing._` | The typing-pass level — `CompilerOutputs`, `HinputsT` etc. `use crate::typing::compiler_outputs::CompilerOutputs;` `use crate::typing::hinputs_t::*;` | |
| `dev.vale.highertyping._` | `use crate::higher_typing::ast::*;` | `FunctionA`, `StructA`, `InterfaceA`, `ImplA`, `CitizenA` |
| `dev.vale.postparsing._` | `use crate::postparsing::names::*;` | `IRuneS`, `INameS`, `IImpreciseNameS`, `IVarNameS`, `IFunctionDeclarationNameS`, etc. |
| `dev.vale.postparsing.IRuneS` | `use crate::postparsing::names::IRuneS;` | Or glob-import the module |
| `dev.vale.parsing._` | `use crate::parsing::*;` | `MutabilityP`, `OwnershipP`, `VariabilityP`, `LocationP` |
| `dev.vale.{RangeS, ...}` | `use crate::utils::range::RangeS;` `use crate::utils::range::CodeLocationS;` | `RangeS` and `CodeLocationS` live in utils |
| `dev.vale.StrI` | `use crate::interner::StrI;` | |
| `dev.vale.PackageCoordinate` | Grep for it — I'm not sure off the top of my head | |
| `dev.vale.Keywords` | Grep for it | |
| `dev.vale.Interner` | Grep for it | |
| `scala.collection.immutable.HashMap` etc. | `use std::collections::HashMap;` / `use std::collections::HashSet;` | |

**When uncertain where a type lives, use the Grep tool with `pattern: "^pub (struct|enum|trait) YourType"` across the whole `src/` tree. The file path tells you the module path.**

## Step-by-step workflow

**Build-output convention (important).** Always pipe `cargo check`/`build`/`run`/`test` into a **fixed file** in `/tmp` (e.g. `/tmp/typing-imports.txt`) and reuse that same file for the whole session. **Do NOT chain `| tail`, `| head`, `| grep`, `| wc`, etc. onto the same command.** `tee` must be the last stage — no `> file` redirection either. Run the build as one command (teeing fully into the file), then inspect the file with a **separate follow-up command**. Chaining defeats the purpose — you lose the ability to re-analyze a different slice of the output without re-running the expensive build. This rule comes from the project-global `CLAUDE.md` and overrides any shorter patterns you may have seen.

1. **Set up a scratch file.** `echo "" > /tmp/typing-imports-progress.txt`. You'll track progress here.

2. **Get a baseline error count.** From `FrontendRust/` run these as **two separate commands**:
   ```
   cargo check --lib 2>&1 | tee /tmp/typing-imports.txt
   ```
   Then:
   ```
   grep -c "^error" /tmp/typing-imports.txt
   grep -c "cannot find type" /tmp/typing-imports.txt
   ```
   Write the numbers down. You want them to decrease after each file.

3. **List the files to edit.** Enumerate:
   ```
   find src/typing -name "*.rs" -not -path "*/tests/*" | sort
   ```

4. **For each file, in order:**
   a. Read the top 50 lines to see the Scala `/* ... import ... */` block.
   b. Read the rest of the file to see what types are actually referenced.
   c. Pick the minimal set of `use` statements that covers those references.
   d. Insert them between the closing `*/` of the header Scala comment and the first `// mig:` marker.
   e. Run the build into the fixed file:
      ```
      cargo check --lib 2>&1 | tee /tmp/typing-imports.txt
      ```
      Then, as a **separate follow-up command**, inspect it:
      ```
      grep -c "^error" /tmp/typing-imports.txt
      ```
      Confirm the count went down (or stayed same — some files reference things not yet defined). **Never chain the `grep` onto the `cargo` line.**
   f. Record in `/tmp/typing-imports-progress.txt` the filename and delta.

5. **Repeat until all files are done.**

## Recommended order (dependencies flow downward)

Do the leaves first so downstream files can import them cleanly.

1. `src/typing/types/types.rs` *(already done, skip)*
2. `src/typing/names/names.rs` *(already done, skip)*
3. `src/typing/ast/ast.rs`
4. `src/typing/ast/citizens.rs`
5. `src/typing/ast/expressions.rs`
6. `src/typing/templata/templata.rs`
7. `src/typing/env/environment.rs`
8. `src/typing/env/function_environment_t.rs`
9. `src/typing/compiler_outputs.rs`
10. `src/typing/compiler.rs`
11. `src/typing/compilation.rs`
12. Any remaining top-level `src/typing/*.rs`
13. `src/typing/citizen/*.rs`
14. `src/typing/expression/*.rs`
15. `src/typing/function/*.rs`
16. `src/typing/infer/*.rs`
17. `src/typing/macros/**/*.rs`

## Gotchas & rules

### MUST NOT

- **Don't touch anything inside `/* ... */` blocks.** A pre-commit hook (`check-scala-comments`) verifies the Scala comments match the original Scala source exactly. If you accidentally change one, the hook blocks the edit.
- **Don't add inline `/* ... */` comments in Rust code** (use `// line comments` instead). Same hook, same reason — the hook thinks your inline block comment is Scala source.
- **Don't touch `// mig:` marker comments.** They pair Rust stubs to Scala defs.
- **Don't remove or rename any existing Rust type declaration, struct field, or function signature.** You're adding imports only.
- **Don't change lifetime parameters.** Already done.
- **Don't run `sed -i` or other in-place bulk edit tools.** Use the `Edit` tool one file at a time.
- **Don't use agents/subagents to make the edits.** Project rule: only the main conversation does modifications.

### WATCH OUT FOR

- **Ambiguous globs.** `use crate::typing::names::names::*;` brings in ~100 names. If two modules both export something called `CoordT` (unlikely, but check), glob-importing both causes a "conflicting import" error. If that happens, convert one glob to an explicit list.
- **Tests directory.** `src/typing/tests/*.rs` uses different conventions — it already has `use` statements and test-specific helpers. **Skip the tests directory** in this pass.
- **`compilation.rs` and `compiler.rs` use `'p` (parser lifetime)** at the boundary. Don't rewrite those to `'s` — they're legitimate.
- **The crate root lifetime names.** `'s` = scout arena, `'t` = typing arena, `'ctx` = short borrow, `'p` = parser arena. If you see one you don't recognize, ask before changing.
- **Fully-qualified paths already in use.** Some files (`hinputs_t.rs`) deliberately use `crate::typing::names::names::IdT` instead of a `use`. Leave those alone — they're correct, just verbose.

### IF SOMETHING BREAKS

- **Error count went up, not down.** You probably imported something that conflicts with an existing definition. `git diff <file>`, revert, try with a narrower `use` (named import instead of glob).
- **"unresolved import"** on your `use` line. The path is wrong. Grep for the type to find its real location.
- **New "ambiguous name"** errors appear. Two modules export the same name; you glob-imported both. Pick one, qualify the other.

## How to verify you're done

Run from `FrontendRust/` — **two separate commands** (see the build-output convention above; never chain the `grep` onto the `cargo` line):

```
cargo check --lib 2>&1 | tee /tmp/typing-imports.txt
```

Then, as separate follow-up commands:

```
grep -c "cannot find type" /tmp/typing-imports.txt
grep -c "^error" /tmp/typing-imports.txt
```

Target for the first: **0** (or a small handful of remnant cases you couldn't resolve — flag each in your handoff).

Should be much lower than the baseline. Expected remaining error classes:
- "the name X is defined multiple times" (duplicate free-function stubs — out of scope for you)
- "trait bound not satisfied" (missing trait impls — out of scope)
- "mismatched types" (deeper type work — out of scope)

## Handing off when done

Write a final summary at `/tmp/typing-imports-progress.txt` with:
- Starting error count
- Ending error count
- List of files touched
- Any files you skipped and why
- Any types you couldn't locate and needed help with

Then commit with a message like:
```
Add use statements to src/typing/**/*.rs files

Resolves E0425 "cannot find type" diagnostics across the typing pass
by adding use preludes for IdT, KindT, CoordT, INameT, ITemplataT, and
friends. No logic or type-signature changes.

Error count: 540 → <N>.
```

## Questions to ask the senior before starting

1. If you're uncertain about a type's location and grep doesn't find it, ask.
2. If you see a file that looks genuinely broken (syntax errors, malformed braces) — ask, don't try to fix.
3. If the error count goes UP after your edit and you can't figure out why — revert and ask.

## Files & references

- `/Volumes/V/Sylvan/quest.md` — design doc for the typing pass; read the "Status" section.
- `/Volumes/V/Sylvan/FrontendRust/src/typing/` — your working directory.
- `/Volumes/V/Sylvan/Frontend/TypingPass/src/dev/vale/typing/` — the Scala source, for reference.
- `/Volumes/V/Sylvan/.claude/rules/postparser/` — style rules; most don't apply here but skim them.
- `/Volumes/V/Sylvan/FrontendRust/docs/usage/check-scala-comments-hook.md` — explains the Scala-comment hook that may block edits.

Good luck. This is a well-scoped, mechanical task with a clear verification loop. Take your time, go file by file, and don't skip the `cargo check` between edits — the feedback loop is your safety net.
