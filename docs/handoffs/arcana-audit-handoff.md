# Arcana audit — handoff

Every arcana in the Vale tree (docs/, src/, Backend/; not Guardian/, Luz/, or sibling repos) has been
found, categorized, and audited claim-by-claim against the current code. This handoff says what the
audit produced, where it lives, and what to do with it. The category tables are
`docs/arcana/reports/00-census.md`.

## What an arcana is, in this tree

`docs/meta.md` and the `good-arcana` skill define the modern form: one file per concern under an
`arcana/` dir, named `<HammerCaseTitle>-<ID>Z.md`, cited from code as `@IDZ`. The tree also carries
three older forms, all counted by the audit:

- `# Title (ID)` sections inside multi-topic files (`docs/arcana/Generics.md` alone holds ~60), no Z
  suffix, cited as `see ID` or `(ID)`.
- The same shape under `docs/old/`, some still cited from live code (WTHPFE, BEAFB, SAFHE, LNASC and
  others; see the census D1 table).
- IDs defined inline in architecture docs, mostly the `§26` block of
  `docs/architecture/vale-rust-interop-architecture.md` and `docs/HigherTypingPass.md`.

Four IDs are cited from code and defined nowhere (GROUPS, MCFBRBF, RPPFNG, SITTX). Ten had their
definition deleted from the tree and were recovered from git history; each of those fourteen has a
report in `docs/arcana/reports/` ending in a "Suggested text" paragraph.

## The audit artifacts

Each audited arcana that is not fully accurate has a report, and its defining section carries a
pointer line directly under its header: `(This arcana has inaccuracies; see <path>-report.md for
corrections.)`. Accurate arcana have neither. Reports do not modify the arcana; they list each claim
as TRUE / DRIFTED / FALSE / UNVERIFIABLE with file evidence, stale citation sites, uncited sites that
embody the concern, and a suggested rewrite where the mechanism changed.

Reports live in a `reports/` dir beside the defining file: `docs/arcana/reports/` (also holds the
never-defined and git-recovered ones), `docs/old/reports/`, `docs/architecture/reports/`,
`docs/reports/` (for HigherTypingPass.md), `docs/todo/reports/`, `src/postparsing/docs/arcana/reports/`,
`src/postparsing/docs/architecture/reports/`, `src/solver/docs/arcana/reports/`.

Counts rot; regenerate them:

```bash
find docs src Backend -path '*/reports/*-report.md' | wc -l        # reports
grep -rl 'arcana .*has inaccuracies' docs src Backend --include='*.md' | grep -v convos | wc -l   # files carrying pointers
grep -rho 'see [^ ]*-report.md' docs src Backend --include='*.md' | sed 's/see //' | sort -u | while read p; do [ -f "$p" ] || echo "MISSING: $p"; done
```

Verdict tally across the 225 audited (measured at handoff time): 70 accurate, 49 minor, 11 major,
42 obsolete, 53 not-an-arcana (design musings, open questions, or unbuilt plans with no checkable
claim about code).

Reports are fix recipes, which `docs/meta.md` says do not get permanent documents. Delete a report and
its pointer line as soon as its arcana is rewritten or deleted; they must not become a second source
of truth.

## The 11 major-inaccuracy arcana

| ID | Where | Action |
|---|---|---|
| DPSFDOZ | vale-rust-interop-architecture.md §26.3 | Replace the sentence naming `is_from_vale_stubs`; the real function is `is_vale_codegen_target` in `src/instantiating/rust_interop/mod.rs`, gated on `#[vale::emit_consumer_body]`. |
| CMWAR | same doc §26.17 | Mark as unbuilt design intent; no `after_rust_analysis` callback, cache, or token type exists. |
| IRIIB | docs/arcana/IRegion.md | Drop the `receiveUnencryptedAlienReference` bullet; subclasses are `Unsafe` and `RCImm`; `Unsafe` is unconditionally the mutable region (no `--region-override`). |
| IEOIBZ | docs/arcana/IdentityEqualityOnIdentityBearingTypes-IEOIBZ.md | Replace the "Where" bullet: FunctionA/StructA/InterfaceA/ImplA do not exist; IEnvironmentT derives rather than hand-implementing ptr-eq. |
| MCFBRBF | comment in `structs_can_resolve_other_structs_instantiation_bound_arguments`, compiler_tests.rs | Replace the comment: bounds resolve against substituted templata in `substitute_citizen_own_bounds`, no deferral; point at MFBFDP. No arcana. |
| DRSINI | docs/arcana/DefaultRulesShouldBeIncrementalNotInitial-DRSINI.md | Core rule true. Rewrite "Why" and "Where the rules live": the postparser keeps `EqualsSR` bundled inside `GenericParameterDefaultS.rules`, it does not hoist it. Rename to DRSINIZ. |
| ATAFLBZ | vale-rust-interop-architecture.md §26.13.5 and the §26 summary row | The prescribed `is_from_vale_stubs` impl-walk filter was never built; the real fix (§26b.6) deleted the name-matching resolvers, derives identity from `def_path` in `package_coord_for`, and backstops with the `no_rust_item_identity_comes_from_a_human_name` lint. |
| RMLRMO | docs/old/Compiler/Templar/Addresses.md; cited twice in `src/typing/ast/expressions.rs` | Conclusion reversed: `MemberLookupTE::new` always yields a `BorrowRefT`, `MutateTE::new` unwraps it, SoftLoad is dead code. Rewrite per the report, migrate to a Z-file in docs/arcana/, re-point both comments. |
| LAGT | docs/arcana/Generics.md | Contradicted by LAGTNGZ; the generic-name builder panics on lambdas. Delete the body, leave a pointer to LAGTNGZ, re-parent its subsections LHPCTLD, GLIOGN, DMPOGN. |
| CCAL | docs/arcana/Generics.md | Built on `MaybeCoercingLookup`/`MaybeCoercingCall` rule kinds that no longer exist. Delete; rune typing settles the kind-vs-coord question. |
| IGBMN | docs/arcana/Impls.md | Three-name impl scheme is gone; one `ImplTemplateNameT`, both-direction lookup via HinputsT indices. Delete, or shrink to the surviving `AnonymousSubstructImplTemplateNameT` naming fact. |

## Migration candidates

Accurate, cited from live code, defined only under `docs/old/`: WTHPFE, BEAFB, SAFHE, LNASC, MDRTCUT,
CSFMSEO, OFCBT, MSFDRF, LDNEIR, SRCAO, ULTMCIE. Each should become a Z-file in `docs/arcana/` (or
`Backend/docs/arcana/` for ULTMCIE and SRCAO) and its `see ID` comments become `@IDZ`.

Accurate and uncited, in `docs/old/`: DBTSAE, DEBDA, DIPRA, DORPAR, EAET, VDND, VWKOA. Their reports
name the code that would cite them.

Never defined but the reconstruction checks out: SITTX (interface TAG_* order must agree between
`generateInterfaceDefsC` and the typeTag body in rcimm.cpp). Its report has the text to use.

## Deletion candidates

Obsolete (describes removed machinery) and not-an-arcana (musings) together cover 95 sections, most of
them under `docs/old/` and the HGM memory-design notes. The census tables list them by verdict. Three
obsolete ones are still cited from code and the citing code is itself commented-out dead code:
SAIRFU/IRFU and SRCAMP (`compiler_solver.rs`, the CoordSend rule mechanism never ported), and RPPFNG
(`Backend/src/function/expression.cpp`, the reserved next-gen parameter removed with the Backend
fold-in).

The rust-interop `§26` block mixes true invariants (RTMEIZ, NNGZ, ELASZ) with unbuilt plans written in
the same present tense (CIDD, DRAFD, HBAB, MAMFC, MIGPROP, CMWAR). Readers cannot tell which is
which; an explicit unbuilt marker on the plans is the fix.

## Out of scope

Arcana under `Guardian/`, `Luz/`, and the Sky prototype (`/Volumes/V/Harmonious/`) were located but
not audited. UTAIRZ is a Sky arcana cited from the Vale doc by design. VCOORD is the V-marker
convention, not an arcana; RTMHTPS, INSHN, NBIFPR, AFCTD, OMCNAGP are aliases of audited IDs.

## Lessons learned

- Old-style definitions often put the `(ID)` alone on the line below the header, or in prose as
  "Previously: ... (ID)"; a header-only regex misses a third of them.
- Spawned agents may only write outside the repo; have them write reports to the scratchpad and copy
  the files in. One agent in ~200 reports a file it never wrote, so check existence before copying.
- A sonnet low-effort agent auditing one section is reliable for FALSE claims with evidence and
  unreliable at the accurate/minor boundary; two runs on the same section can disagree there.
- The census's `@ID` grep drowns in `@TFITCX` (hundreds of hits) and V-markers; exclude them up front.
- Deleting a docs directory in a reorganization commit (the FrontendRust flatten removed
  `docs/regions/Regions.md` and `docs/InstantiatorRegions.md`) silently orphans every `see ID` comment
  that pointed into it; grep for cited IDs before deleting a doc.
