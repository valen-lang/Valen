# Accuracy report: Default Parameters Can Only Depend on Other Default Parameters (DPCODODP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1528 ("## Default Parameters Can Only Depend on Other Default Parameters (DPCODODP)")

## Verdict

Obsolete. The section is a first-person debugging note recording that `struct Functor1<F Prot = func(P1)R>` (with `P1`/`R` free in the default rule, not declared as their own generic params) used to fail to compile, and proposing the fix of hoisting `P1`/`R` into explicit generic parameters (`struct Functor1<P1 Ref, R Ref, F Prot = func(P1)R>`). Current code contradicts this: `src/typing/test/compiler_solver_tests.rs:327` defines and successfully compiles exactly the un-hoisted form the note says "had no idea what to do" with, via `test_single_parameter_function`, which passes (`cargo test ... test_single_parameter_function` → `ok`). The bug the note describes no longer exists (or was fixed by a different mechanism than the one proposed), so the section documents a historical, already-resolved problem and its (evidently unadopted, or superseded) proposed fix — not a current invariant of the code.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `struct Functor1<F Prot = func(P1)R> imm where P1 Ref, R Ref {}` fails to compile because the definition-site rule doesn't know what coords to use for `P1`/`R`. | FALSE (as a current fact) | src/typing/test/compiler_solver_tests.rs:327-334 | The equivalent form (`struct Functor1<F Prot = func(P1)R> share {}`, using `func __call<F Prot = func(P1)R>`) compiles and runs successfully today; `test_single_parameter_function` passes. |
| 2 | The fix is to hoist `P1`/`R` into their own generic params: `struct Functor1<P1 Ref, R Ref, F Prot = func(P1)R> imm {}`. | UNVERIFIABLE / not adopted as a general rule | src/typing/test/compiler_solver_tests.rs:327 | The live test uses the un-hoisted form, so this specific fix was not the one that ended up needed, or the underlying solver was later changed to handle the un-hoisted case directly (e.g. via placeholders auto-generated for `DefinitionFuncSR`/`CallSiteFuncSR`, see src/typing/infer_compiler.rs:883-914 and src/typing/rule_runes.rs:24-27). No code implements "default params can only depend on other default params" as a checked invariant. |
| 3 (title) | "Default Parameters Can Only Depend on Other Default Parameters" — implies a rule enforced somewhere in the compiler. | FALSE / not implemented | grep for `DPCODODP` across src/ and docs/ finds only the doc itself; grep for "default param" restriction logic found nothing enforcing this | No such restriction exists in current code; the passing test above uses a default parameter (`F`'s default `func(P1)R`) that depends on non-default, ordinary-looking free runes, not on "other default parameters". |

## Stale citation sites

None — `grep -rn -w "DPCODODP"` across src/, Backend/, docs/ (excluding target/, tmp/, guardian-logs/, Guardian/, Luz/, docs/convos/) finds no citations of this ID anywhere in code or other docs.

## Uncited sites that embody the arcana

None found — the mechanism this note worried about (placeholder generation for `DefinitionFuncSR`/`CallSiteFuncSR` rules referencing free runes in a default) is presumably handled inside src/typing/infer_compiler.rs and src/typing/infer/compiler_solver.rs, but there is no code that implements "default parameters can only depend on other default parameters" as a rule, so there's nothing to cite this ID from.

## Suggested rewrite

This is a stale personal debugging note whose premise (the un-hoisted `Functor1` form fails) is now false and whose proposed fix was not the one that stuck (the un-hoisted form works directly). Recommend deleting the section, or replacing it with a short historical note:

> ## Default Parameters Can Only Depend on Other Default Parameters (DPCODODP)
>
> Historical note: this used to be a real bug — `struct Functor1<F Prot = func(P1)R>` (free runes `P1`/`R` inside a default rule, not declared as their own generic params) failed to compile because the definition-site rule had no coords to use for them. The fix proposed here (hoisting `P1`/`R` into explicit generic params) was never the one adopted; the compiler now handles the un-hoisted form directly (see `test_single_parameter_function` in src/typing/test/compiler_solver_tests.rs), so this restriction no longer applies and there is no current invariant to depend on.
