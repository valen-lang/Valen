# Accuracy report: Inline Struct Generationed FFOP (ISGFFOP)

Audited against the working tree on 2026-09-06. Source: docs/old/HGM V20.md:276-291

## Verdict
This is a self-contained design musing exploring a "very weird" variant memory-model trick (XOR-ing final owning pointers with a container's generation to derive an inner object's generation, reusing the low bit as a scope tether) for the never-shipped HGM generational-references design. It makes no claims about the current codebase — it's speculative, and the text itself rejects the idea in its last line ("Probably best not do this part."). There are zero citations of ISGFFOP anywhere in src/, Backend/, or docs/ (only the docs/old source file itself and this session's convo log mention the ID), and no code in the current Rust tree implements or references struct/array owning-pointer generation-XOR tethering. Verdict: not-an-arcana — recommend leave in docs/old/ (or delete) since the design was explicitly abandoned and never had any downstream citations to clean up.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "We can xor the final owning pointers with their container's generation, and have that serve as the inner object's generation" — a proposed mechanism | N/A (design musing, not a claim about current code) | — | Not implemented anywhere in src/ or Backend/; not a factual claim to verify. |
| 2 | The idea is impractical ("defeat the optimizer a lot") and rejected ("Probably best not do this part") | TRUE (self-consistent) | docs/old/HGM V20.md:289-291 | The document itself confirms it was never adopted. |

No claims describe present-day compiler behavior; nothing to mark FALSE/DRIFTED.

## Stale citation sites
None — zero code or doc citations exist for ISGFFOP outside docs/old/HGM V20.md and the session convo log.

## Uncited sites that embody the arcana
None found. No generation-XOR / tether mechanism exists in src/typing or Backend for struct/array owning pointers; the shipped borrow-checker design (aliasing_info.rs, groupify.rs) uses group-based noalias analysis, an entirely different mechanism.

## Suggested text
Not applicable (D3 kind, verdict is not-an-arcana, not major-inaccuracies/obsolete).
