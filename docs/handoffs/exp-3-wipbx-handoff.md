# exp-3-wipbx — handoff

This branch pushes the onion type model all the way to the backend: rewrite the instantiator to emit
an **onion-typed `HinputsI`**, delete the simplifying (hammer) pass and `ProgramH`/`final_ast`, and
rewire the C++ backend + TestVM to consume `HinputsI` directly. The full four-step plan is
`docs/plans/complete-backend-plan.md` — read its "Background" before touching code.

## State (regenerate, don't trust stale)

- Branch tip: `git log --oneline -1`; uncommitted work: `git status --short`.
- Only the **typing** suite links today — `backend_ffi`, `final_ast`, `instantiating`, `simplifying`,
  `testvm` are commented out in `src/lib.rs` (grep `pub mod` there for the live set). The downstream
  passes are stale, not merely gated: the instantiator matches typing enum variants that no longer
  exist and imports deleted typing types.
- Build/test: `cargo test --manifest-path ./Cargo.toml --lib` (typing suite). Read counts from
  `grep "test result"`.

## Where the plan stands

No step started. Step 1 (typing handles Defer, delete `DeferTE`) is the entry point and needs architect
approval first — `src/typing/` is human-edited. Steps 1–3 land as `TEMP CHECKPOINT` commits with the
green-at-commit invariant suspended; only step 4 (backend + TestVM) restores a full-suite gate.

## The load-bearing constraint

Blast `Coord`/`Ownership`/`Location` away wherever you touch it — never make new code interoperate with
`CoordI`/`CoordH`/`ProgramH`. Placement (Inline/Yonder) is **derived from the onion shape at codegen**,
not a carried field; that derivation (step 4) is the one genuinely new semantic in the whole plan.

## Lessons Learned

- The typing pass is human-authored (`src/typing/.claude/CLAUDE.md`): get explicit architect approval
  before editing `src/typing/`.
- Do not reason from the downstream passes as if they were the target — the tree is mid-migration, and
  the instantiator/hammer/final_ast are frozen at a pre-onion checkpoint that the plan deletes, not revives.
