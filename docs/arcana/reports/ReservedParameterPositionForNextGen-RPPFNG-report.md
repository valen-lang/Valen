# Accuracy report: Reserved Parameter Position For Next-Gen (pointer) (RPPFNG)

Audited against the working tree on 2026-09-06. Source: reconstructed from citing code (Backend/src/function/expression.cpp:290, plus history)

## Verdict
The reconstruction correctly describes a real mechanism that once existed in the Midas/Backend C++ codegen — a reserved 0th LLVM parameter carrying a `noalias`-tagged "next generation number" pointer used by the generational-references (HGM) scheme, with `FunctionState::getParam` and an `argumentIndex + 1` load translating user argument indices past it. But that entire scheme was deleted in commit `ed6f54f8` ("Fold Backend into FrontendRust as an in-process C ABI; retire JSON pipeline, generations, and other subprocess-era plumbing"): `FunctionState::getParam` (Backend/src/function/function.cpp:244) is now a plain `LLVMGetParam(containingFuncL, userArgIndex.userArgIndex)` with no `+1` and no `nextGenPtrLE` branch, the `nextGenPtrLE` member is gone from function.h, and `definefunction.cpp`'s `addRawFunction` no longer exists in Backend/src (only stale .o build artifacts remain). The lone surviving citation, expression.cpp:290, is now a dangling/false comment — the line beside it does no `+1` arithmetic. Verdict: obsolete — migrate nothing; instead delete the stale comment at Backend/src/function/expression.cpp:290 (it misdescribes the current getParam call) or, if kept, fix it to note the scheme is retired.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Every generated LLVM function had its parameter list prefixed with a 0th "next generation number" pointer, `restrict`/`noalias` tagged | FALSE (present tense) / TRUE (historically) | Backend/src/function/function.cpp:244; no `nextGenPtrLE` anywhere in Backend/src | Scheme was removed by commit ed6f54f8; no such prefix parameter exists today |
| 2 | Real arguments are shifted by +1 in the underlying LLVM function; `FunctionState::getParam(UserArgIndex)` and the `argumentIndex+1` load exist to skip the reserved slot | FALSE | Backend/src/function/function.cpp:244 (`return LLVMGetParam(containingFuncL, userArgIndex.userArgIndex);` — no +1, no branch) | getParam is now a direct passthrough; no reserved slot exists |
| 3 | No standalone definition was ever written for RPPFNG; it was defined only implicitly via inline comments added when importing Backend from an experimental "regions" branch | TRUE | git log -S'RPPFNG': commit b93cf939 "Merging backend from experimental regions branch (#597)" introduced it | — |

## Stale citation sites
Backend/src/function/expression.cpp:290 — comment says "This +1 is because the 0th argument is always the next gen ptr, see RPPFNG", but the adjacent code (`functionState->getParam(UserArgIndex{argument->paramIndex})`) does no +1 and getParam itself does no +1 either. The comment describes a mechanism that no longer exists in this function or in getParam.

## Uncited sites that embody the arcana
None found — the mechanism it describes is gone. The codebase does have an unrelated, newer per-parameter `noalias` mechanism (borrow-checker-driven, tags real parameter indices directly, no reserved slot) at Backend/src/function/function.cpp:30-47, but it is a different concept and should not be tagged RPPFNG.

## Suggested text
Not applicable in the normal sense (kind F asks for suggested text, but the concern itself is dead code from a retired scheme). If the maintainer wants a historical note rather than deletion:

> RPPFNG (historical, retired in ed6f54f8): the pre-FFI Midas/Backend codegen reserved LLVM parameter 0 of every generated function for a `noalias` pointer to a "next generation number," part of the generational-references (HGM) memory-safety scheme; all real argument indices were offset by +1 through `FunctionState::getParam`. Removed when Backend was folded into FrontendRust as an in-process C ABI and the generations scheme was retired. No longer applicable — delete stray comments citing RPPFNG rather than migrating this into docs/arcana.

Pragmatic recommendation: delete the stale comment at Backend/src/function/expression.cpp:290; nothing to migrate.
