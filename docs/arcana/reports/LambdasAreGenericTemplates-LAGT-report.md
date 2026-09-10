# Accuracy report: Lambdas Are Generic Templates (LAGT)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:703 ("# Lambdas Are Generic Templates (LAGT)")

## Verdict

The section's core mechanism is superseded and contradicted by both the current code and the codebase's own newer arcana, `docs/arcana/LambdasAreGenericTemplatesNotGenerics-LAGTNGZ.md` (LAGTNGZ). LAGT describes lambdas as producing ordinary **generic functions** (`func moo<T>`-style) that the typing pass emits with placeholder-ish "generic template args," which the Instantiator then instantiates a second time into concrete `<int>`/`<bool>` monomorphizations, incidentally deduplicating repeats. Current code does the opposite: the typing-pass name builder for generics explicitly panics on lambdas (`panic!("Lambdas are generic templates, not generics")`, src/typing/names/name_translator.rs:43), and lambda specialization happens once, per call site, directly in the typing pass by baking concrete arg types into `LambdaCallFunctionTemplateNameT` — there is no second Instantiator-driven specialization step for lambda calls (src/instantiating/instantiator.rs:2364-2368 just carries the already-concrete name through unchanged). LAGTNGZ documents this precisely and even records that a prior session's belief matching LAGT's model was wrong (see docs/convos/convo-73-*.md:3143 "I was wrong about lambdas. LAGTNGZ explains why. Correcting the doc."). That correction produced a new doc instead of fixing LAGT itself, leaving LAGT in docs/arcana/Generics.md actively describing the disproven model. Treat as major-inaccuracies (core mechanism wrong), bordering on obsolete.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Lambdas are instantiated every time they're called." | DRIFTED | src/typing/names/name_translator.rs:22-29 | They are *specialized in the typing pass* per call site, not "instantiated" in the Instantiator sense used everywhere else in this doc — the word conflates two distinct mechanisms LAGTNGZ deliberately separates. |
| 2 | The example's three lambda expansions ("`__call{bool}`", "`__call{genFunc$0}`", "`__call{int}`") "are each generic functions, just like a normal `func moo<T>(a T) {...}`/`moo<moo$0>` generic function. ... these are **not** instantiations." | FALSE | src/typing/names/name_translator.rs:40-45 (`IFunctionDeclarationNameS::LambdaDeclarationName(_) => panic!("Lambdas are generic templates, not generics")`); LAGTNGZ.md:1-3 | Lambdas are explicitly *not* generic functions — the generic-name builder refuses them outright. They are templates (`LambdaCallFunctionTemplateNameT`), a distinct kind from `FunctionTemplateNameT` used for real generics. |
| 3 | "after the instantiator pass these would be the three final instantiations" — i.e. the Instantiator takes the typing pass's generic lambda functions and instantiates them again into `<bool>`/`<int>`-suffixed names, with duplicates collapsing ("Serendipitously, this approach will result in the same ending instantiation name ... so we don't have to instantiate that lambda an extra time needlessly"). | FALSE | src/instantiating/instantiator.rs:2364-2368; src/instantiating/ast/names.rs:1533 ("Per @LAGTNGZ, paramTypes stays baked in (specialization happened earlier)") | There is no second lambda-specialization stage in the Instantiator. `LambdaCallFunctionNameI` is produced by copying the already-concrete typing-pass name (code_location + param_types) through; nothing is re-instantiated or deduplicated at this stage the way generic functions are. |
| 4 | "So in a way, a generic is a template that makes a generic function." | FALSE | Same as #2-3; LAGTNGZ.md:3 ("The lambda-vs-top-level choice is made at overload resolution ... inside the templated path ...") | Per LAGTNGZ, lambdas and top-level functions use two *disjoint* mechanisms (template vs. generic), not a template-that-makes-a-generic composition. |
| 5 | (Implicit, via `See also`) the cross-reference to LAGTNGZ for "how this design interacts with ... `function.isLight()`" | DRIFTED | src/typing/function/function_compiler.rs:118,150 (`function.is_light()`); src/postparsing/ast.rs:636 (`pub fn is_light`) | Correct in substance but uses the Scala-era camelCase name `isLight()`; current Rust code calls it `is_light()`. Minor, but worth flagging since the surrounding section is otherwise wrong. |

## Stale citation sites

None — no code cites `LAGT` (grep -rn -w "LAGT" across src/, Backend/, docs/ excluding target/tmp/guardian-logs/Guardian/Luz/convos finds only self-references inside docs/arcana/Generics.md itself, at lines 6, 703, 758).

## Uncited sites that embody the arcana

None found as *embodiments of LAGT's described mechanism* — the actual current mechanism (per-call-site typing-pass specialization, no second Instantiator stage) is instead documented and cited under LAGTNGZ, e.g.:
- src/instantiating/ast/names.rs:1533, 1615 (`// Per @LAGTNGZ, ...`)
- src/typing/names/name_translator.rs:22-45 (uncited, but this is the real code LAGTNGZ describes)

## Suggested rewrite

The `# Lambdas Are Generic Templates (LAGT)` section in docs/arcana/Generics.md should be replaced with a short pointer to LAGTNGZ rather than left as an independent (and wrong) description, e.g.:

> # Lambdas Are Generic Templates, Not Generics (LAGT)
>
> Lambdas are **not** generic functions. Unlike a normal `func moo<T>(a T) {...}`, which is compiled once against abstract placeholders and later monomorphized by the Instantiator, a lambda is specialized fresh at each call site directly in the typing pass: the concrete argument types are baked into the resulting `LambdaCallFunctionTemplateNameT` as it's produced, and there is no later Instantiator-driven specialization step for it. See `docs/arcana/LambdasAreGenericTemplatesNotGenerics-LAGTNGZ.md` (LAGTNGZ) for the full mechanism, the `FunctionA.isLambda()` / `isLight()` dispatch, and worked examples of one lambda body producing multiple independently-named `__call` entries.

The `LHPCTLD` subsection that follows LAGT in the doc should be re-checked separately (out of scope here), since it opens with "Look at LAGT's example" and currently depends on the now-incorrect example.
