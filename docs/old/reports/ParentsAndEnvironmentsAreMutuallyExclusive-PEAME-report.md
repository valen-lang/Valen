# Accuracy report: Parents and Environments Are Mutually Exclusive (PEAME)

Audited against the working tree on 2026-09-06. Source: `docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md:165-188`

## Verdict
This is a Scala-Templar-era design musing about a data shape — `FunctionTemplata(env, function: FunctionEnvEntry)` where `FunctionEnvEntry` itself carried a `parent: Option[IEnvEntry]` — and a forward-looking TODO ("When we support nested structs better, we'll need assertions and tests that make sure... the parent environments and parents dont overlap"). It makes no assertion about present-day behavior; it flags a risk to guard against later. That data shape no longer exists: in the current Rust code, `FunctionEnvEntry` (src/typing/env/i_env_entry.rs:5-11) is just `{ template_id }` with no `parent` field at all, and the "parent environment" lives instead on `FunctionTemplataT { outer_env, function_template_id }` (src/typing/templata/templata.rs:185-188) — one field, not two things that could overlap. The redundancy the note worried about was designed away rather than fixed with the assertions it proposed; nothing in the current code embodies or needs this concern. It has zero citations, and no plausible current site to migrate it to. Recommendation: leave it in docs/old/ (or delete) — it does not describe current code and does not warrant a docs/arcana/ entry.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `FunctionTemplata(env: IEnvironment, function: FunctionEnvEntry)` | OBSOLETE/FALSE for current code | src/typing/templata/templata.rs:185-188 | Current `FunctionTemplataT` is `{ outer_env, function_template_id }` — no `FunctionEnvEntry` field; the env and the "function" are collapsed into an env + id, not env + entry-with-its-own-parent. |
| 2 | `FunctionEnvEntry(parent: Option[IEnvEntry], function: FunctionA)` | FALSE for current code | src/typing/env/i_env_entry.rs:5-11 | Current `FunctionEnvEntry<'s,'t>` is `{ template_id: &'t IdT<'s,'t> }` only — no `parent` field, no `function: FunctionA` field. |
| 3 | "(same with structs and interfaces)" | FALSE for current code | src/typing/env/i_env_entry.rs:13-29 | `StructEnvEntry`/`InterfaceEnvEntry` are `{ template_id, tyype }` — no `parent` field either. |
| 4 | "We only need to know the parent's rules if we don't already have all of the runes that were produced by those rules." | UNVERIFIABLE / not a checkable code claim | — | This is design reasoning about an inference optimization, not a claim tied to a specific current mechanism; no corresponding rune/parent-rule-skipping logic was found to check it against. |
| 5 | "When we support nested structs better, we'll need assertions and tests that make sure... the parent environments and parents dont overlap." | Not-a-claim (a TODO/plan) | — | This is an open action item, not a statement about current code. It was never acted on (no such assertion exists), and is now moot because the two-parents data shape it warns about (env AND entry-parent) doesn't exist in the current design — see claims 1-3. |

## Stale citation sites
None — `sites: []` in the record, and `grep -rn -w "PEAME"` across src/, Backend/, docs/ returns no hits.

## Uncited sites that embody the arcana
None found. The concern (avoid double-tracking of parent lineage through both an env chain and a per-entry parent pointer) isn't present in current code because there is only one parent-carrying field (`outer_env` on `FunctionTemplataT`) — there's nothing left to keep mutually exclusive.

## Suggested text
Not applicable (D3 kind; only required for major-inaccuracies/obsolete, and here there's no viable modern equivalent to state — the note describes a superseded internal data shape with no current analog worth re-deriving).
