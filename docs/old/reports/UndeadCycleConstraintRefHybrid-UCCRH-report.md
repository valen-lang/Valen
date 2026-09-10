# Accuracy report: Undead Cycle / Constraint Ref Hybrid (UCCRH)

Audited against the working tree on 2026-09-06. Source: docs/old/HGM V16, V17, V18.md:557-566

## Verdict
This is a two-sentence speculative aside in the HGM (generational references memory-model) notes, not a definition of any implemented mechanism. It says: "Keeping something alive is still actually a valid strategy sometimes. We can't do it for arrays, or inline objects, but we can still do it for heap objects. We could have a release mode that's a little more resilient for this case." There is no name given to a concrete design (no algorithm, no data structure, no API), just a musing that a "release mode" combining undead/keep-alive semantics with constraint refs might be worth building for heap objects. It has zero citations anywhere in src/, Backend/, or docs/ (confirmed by `grep -rn -w "UCCRH"`), so there's nothing live to check it against, and no claims about current code exist to verify as TRUE/FALSE. Recommendation: delete — it's an unrealized brainstorm fragment with no citing code and no actionable claim, not something worth migrating into docs/arcana.

## Claims
No checkable claims about code as it exists today — this is an open design musing/idea fragment, not a definition of an implemented or specified mechanism. (Per instructions, stating this in lieu of a claims table.)

## Stale citation sites
None — zero citations found (`grep -rn -w "UCCRH"` across src/, Backend/, docs/ returns nothing outside docs/old/).

## Uncited sites that embody the arcana
None found. The "release mode" / undead-keep-alive idea for heap objects was never implemented under this name; no code in the borrow_checker or elsewhere corresponds to this specific hybrid proposal.

## Suggested text
Not applicable (D3 kind, verdict is not major-inaccuracies/obsolete but not-an-arcana — no suggested text needed).
