# Vale4 coordination — handoff

Coordination with the Vale4 interop work: the standing answers already given to them, the branch
relationship, the live interop cases, and the commit-shape gotchas. Vale4 owns the `rust_interop`
synthesis; this handoff records where their work and ours meet.

## Branch relationship

`exp-2-wipbx` (ours) **runs behind the Vale4 interop work** whenever it lands and ratchets. **Their
commits do reach our files** — a lookup-path change touched `compiler_solver.rs` (`typing/infer/`) — so
read the incoming diff rather than assuming a clean rebase. The branches are `exp-1-wipbx` …
`exp-4-wipbx` plus `main`/`stable`; `exp-4-wipbx` is the Vale4 worktree. Regenerate the actual
relationship rather than trusting a written one:

```
git log --oneline -1 exp-1-wipbx exp-2-wipbx exp-3-wipbx exp-4-wipbx
git log --oneline exp-2-wipbx..exp-4-wipbx     # Vale4 commits we have not absorbed
```

## Standing answers already given to Vale4

In case they come back on any: bounds are near-term, so sequence behind them and prepare against roughly
the shape the codebase implies, since what remains is patching and hole-filling. A design pass before the
work lands was **declined**, under a general *make-it-work-then-adapt* policy that applies equally to
`GenericParameterDefaultS` — which is **in scope for rework, not settled**. `reachability.rs` is
**optional for us**; they can have whatever shape they want or write it themselves. `instantiating/` is
roughly three weeks out. Regions are independently roadmapped with the architecture settled, `convert()`
belongs to the not-yet-started dispatch mission, and their `is_primitive` finding is right but **the fix
is renaming, not moving the `Str` row**.

## Live interop cases

**Vale4's `opt_with_undroppable_contents` passes.** Its sibling `opt_with_undroppable_mutable_ref_contents`
is `#[ignore]`d pending borrowing (a `re enable w borrowing` VCOORD) — a `Some<&Spaceship>` argument at an
`Opt<&Spaceship>` parameter, which the phased-calls plan's §2 upcastability step resolves
(`docs/plans/plan-phased-calls.md`).

## Commit-shape gotcha

When the Vale4 worktree (`experimental-4`) is checked out on our tip, land corrections as a **follow-up
commit rather than an amend** — amending rewrites the commit under them. This has mattered twice.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **The interop lane is gated on `rust_interop/**` changes**, so a commit that doesn't touch that path
  never runs it — even when it breaks it. A change that alters what flows *through* the pass (group params,
  new templata shapes) can break the instantiator/interop without touching `rust_interop/**`; run the
  interop lane by hand for such changes.
- **Check master's copy before declaring machinery never-ported.** A deliberate mid-migration removal looks
  identical to an unfinished port in the working tree, and the removed version documents what the
  replacement must cover; `git show master:<file>` settles it in one command.
- **`8d40eff9d` and `699241ffb` reshaped the interop seam from the Vale4 side** — read them before
  planning interop work (archaeology; do not resurrect the pre-reshape seam).
- **When our tip carries the Vale4 worktree, correct with a follow-up commit, never an amend** — an amend
  rewrites the commit the other worktree is standing on.
