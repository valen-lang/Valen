# Rust interop — docs and conversation logs

Everything about Vale's Rust interop lives in this directory. That containment is deliberate: the
interop work is separable from the main compiler, so the main compiler's docs stay about the main
compiler, and an agent can be pointed at this directory alone.

## The design docs

| doc | what it is |
|---|---|
| `vale-rust-interop-architecture.md` | The master design — 30 chapters, ~3,540 lines. Locked decisions with rejected alternatives recorded. §8.10 holds the ratified Option A representation; §3–§5 the two-binary split, the fork, and codegen; §19–§20 the rustc-side pipeline. |
| `synthesized-declarations-plan.md` | **Start here.** State, plan, and handoff for the Rust-interop arc — the design in one page, what's in the tree, decisions locked, verified facts worth not rediscovering, the testing plan, known defects, and what's blocked on whom. |
| `rust-interop-frontend-plan.md` | ⚠ Largely superseded by the above — it describes the abandoned per-call-site oracle seam. Its name-property refinement (§0) and build-config decisions still hold. |
| `rust-interop-callout-map.md` | ⚠ Partially superseded. Every place the existing compiler asks a question about a type that a Rust-backed type can't answer — ~30 live sites plus ~20 in the currently-unlinked passes. The **inventory stands**; the per-site *fixes* assume the abandoned seam. |

**If you are looking for "what do we still need to change in the core compiler", it is the callout
map, and specifically its §9 site index.** For a period those sites carried `// ZRI:` comments in the
source; they were removed (2026-07-25) to keep interop annotations out of the main compiler, and
every marker's content was folded into the map first. §9 is the table `grep ZRI` used to answer.

## The conversation logs

Verbatim transcripts, oldest first. `convo-0` through `convo-3` are the architecture design arc;
`convo-4` onward is implementation.

| # | file | subject |
|---|---|---|
| 0 | `convo-0-architecture.md` | initial architecture design |
| 1 | `convo-1-architecture.md` | architecture doc drafted (skeleton + four-pass fill) |
| 2 | `convo-2-architecture.md` | architecture review and revision |
| 3 | `convo-3-architecture.md` | architecture review and revision |
| 4 | `convo-4-doc-migration-and-tyctx-oracle.md` | canonical-syntax migration; the `TyCtxt` oracle shape |
| 5 | `convo-5-primitive-interop.md` | primitive interop (a parallel thread, from `experimental`) |
| 6 | `convo-6-option-a-frontend-plan.md` | Option A ratified; §8.10 written; frontend plan authored |
| 7 | `convo-7-callout-map-and-seam.md` | the twelve surveys, the callout map, the oracle seam landing |
| 8 | `convo-8.md` | Milestone 2 against a real `TyCtxt`; the pivot away from the per-call-site oracle |
| 9 | `convo-9-generics-seam-collapse-and-test-tiers.md` | generics via structural signature reading; methods/drop/free functions collapsed to one declaration path; a real `StructDefinitionT`; the two test tiers and the in-process-rustc experiment |

## A caveat on paths inside the transcripts

These files are **verbatim logs**, so the paths they mention are the paths that existed when the
words were written. They were not rewritten during this reorganization, because several are the
human's own typed instructions and editing them would put words in someone's mouth. Translate with
this table:

| as written in a transcript | now |
|---|---|
| `docs/architecture/vale-rust-interop-architecture.md` | `docs/convos/rust_interop/vale-rust-interop-architecture.md` |
| `docs/architecture/rust-interop-frontend-plan.md` | `docs/convos/rust_interop/rust-interop-frontend-plan.md` |
| `docs/architecture/rust-interop-callout-map.md` | `docs/convos/rust_interop/rust-interop-callout-map.md` |
| `docs/historical/vale-rust-interop-architecture-convo-0…3.md` | `convo-0-architecture.md` … `convo-3-architecture.md` |
| `docs/convos/convo-4-interop-doc-migration-and-tyctx-oracle.md` | `convo-4-doc-migration-and-tyctx-oracle.md` |
| `docs/convos/convo-4-primitive-interop.md` | `convo-5-primitive-interop.md` |
| `docs/convos/convo-5-rust-interop-option-a-frontend-plan.md` | `convo-6-option-a-frontend-plan.md` |
| `docs/convos/convo-6-rust-interop-callout-map-and-seam.md` | `convo-7-callout-map-and-seam.md` |

Note the renumbering: there used to be two `convo-4-*` files, since primitive-interop arrived from
another branch and collided. Numbers 5–7 shifted by one to resolve it, so a transcript that refers to
"convo-5" or "convo-6" by number means what is now 6 or 7.

## Related, outside this directory

- `todo/opaque-extern-drop.md` and `todo/ffi-drop-followups.md` — the auto-drop-for-extern-structs
  design. The first explicitly defers the generic-monomorphization naming question "to the
  Rust-interop TL."
- `/Volumes/V/LangNotesValen/Valen/valen-design-1.md` and `-2.md` — the Valen language reference the
  architecture doc reconciles against.
- `/Volumes/V/Harmonious/rust-interop-architecture.md` — the toylang/Sky prototype's architecture
  doc, ~7,700 lines. This design inherits from it by name throughout; it is the authority on what
  has actually been made to work.
