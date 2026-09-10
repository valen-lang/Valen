# Accuracy report: ITables Need to be 16-Byte Aligned (ITN16BA)

Audited against the working tree on 2026-09-06. Source: recovered from git history — git:a502ea0e:docs/SeparatedFFI.md (added), persisted through git:b9a4dcdf^:docs/SeparatedFFI.md before deletion.

## Verdict
The one live claim — that itables are forced to 16-byte alignment — is TRUE and still enforced by a single, exact citation in Backend/src/region/common/defaultlayout/structs.cpp:243. But the *reason* given in the recovered text ("Because of URSL") no longer holds: URSL (Universal Reference Struct Layout) and its sibling arcanum RMB16BA are both absent from the current tree (no hits anywhere in Backend/ or docs/), so nothing in the live codebase explains *why* itables still need this alignment — the code just asserts it with a comment pointing at this arcanum and no rationale of its own. Recommendation: migrate into docs/arcana as a proper Z-suffix doc, but rewrite the "why" — either recover the real current reason (if compression/tagging of itable pointers is still done somewhere, e.g. in the region-pointer packing code) or, if the alignment is now vestigial/defensive with no live consumer of the low bits, say that plainly instead of citing dead URSL machinery.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Because of URSL, itables need to be 16-byte aligned so they can be compressed." — mechanism claim: itables ARE forced to 16-byte alignment | TRUE | Backend/src/region/common/defaultlayout/structs.cpp:243 `LLVMSetAlignment(itablePtr, 16);`, with the citing comment at line 242 | none |
| 2 | The alignment exists "because of URSL" (Universal Reference Struct Layout) so itable pointers "can be compressed" (low 4 bits reused for tagging) | DRIFTED / stale rationale | No hits for "URSL" or "RMB16BA" anywhere in Backend/ or docs/; no pointer-tag-bit compression code found near structs.cpp:242-243 | The URSL scheme this rationale depends on has been removed from the tree along with its sibling arcanum RMB16BA. The alignment call is still present but its justification is not verifiable in current code — either a replacement rationale exists elsewhere in Backend/'s region-pointer packing code (not found in this pass) or the constraint is now vestigial. |

## Stale citation sites
Backend/src/region/common/defaultlayout/structs.cpp:242 — comment reads "ITables need to be 16-byte aligned, see ITN16BA." This still correctly documents the alignment call on the next line, but the arcanum it points to explains the constraint via URSL, a scheme no longer present in the codebase. The citation is not wrong about *what* happens, only silent about *why* it still needs to.

## Uncited sites that embody the arcana
None found — grep for other `LLVMSetAlignment(...,16)` calls in Backend/src returned only this one site.

## Suggested text
**ITables Need to be 16-Byte Aligned (ITN16BA):** Interface vtables (itables) are declared with forced 16-byte LLVM alignment (`LLVMSetAlignment(itablePtr, 16)` in Backend/src/region/common/defaultlayout/structs.cpp) rather than left to the platform default. Before writing new region/layout code that reads or stores itable pointers, don't assume natural alignment is enough — this call exists specifically to guarantee the stronger 16-byte bound. (Note for the doc author: the original rationale tied this to the now-removed URSL universal-reference-compression scheme; before publishing, confirm whether a current consumer still needs the low bits of an itable pointer free, or rewrite this as a legacy constraint kept for compatibility/ABI stability.)
