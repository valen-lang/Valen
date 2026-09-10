# Accuracy report: Templatas Must Remember Environment (TMRE)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md:1-24

## Verdict
The core mechanism described — that a templata pairs a raw function/struct/interface with the environment it was pulled from, so callers can later ask that environment for things (e.g. method lookup by scanning argument environments, closures storing `__call` in their own environment) — is still true of the Rust typing pass: `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, and `ImplDefinitionTemplataT` in src/typing/templata/templata.rs all carry an environment field (`outer_env`, `declaring_env`, `declaring_env`, `env` respectively). No code currently cites TMRE (it has zero live sites — confirmed D3 kind), so nothing is stale, but the idea itself is accurate and still load-bearing. Recommendation: migrate into docs/arcana as a proper Z-suffix doc (e.g. rename to reflect current terminology — "templata" survived the port unchanged) — the environment-pairing rationale isn't otherwise documented in the current codebase.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "An environment will contain raw functions and structs and interfaces." | TRUE | src/typing/env/environment.rs (IEnvironmentT holds entries incl. function/struct/interface templatas) | — |
| 2 | "When we pull things out of those environments, we need to remember the environment it came from... We pair those two things together... into a templata." | TRUE | src/typing/templata/templata.rs:182-186 (`FunctionTemplataT.outer_env`), :208-212 (`StructDefinitionTemplataT.declaring_env`), :330-334 (`InterfaceDefinitionTemplataT.declaring_env`), :341-345 (`ImplDefinitionTemplataT.env`) | Not all templata variants carry an env — `KindTemplataT`, `PlaceholderTemplataT`, `PrototypeTemplataT`, `BooleanTemplataT`, `IntegerTemplataT`, `GroupTemplataT` do not (templata.rs:154-390). The code itself flags this as unresolved: a comment at templata.rs:203-205 reads "AFTERM: figure out why some templatas compare environment and some don't — equality includes `declaring_env`." So the general claim holds for the templata kinds that need it (function/struct/interface/impl definitions), but is not universal across the `ITemplataT` enum. |
| 3 | Method lookup example: "we need to know where to look for fly; we look in the environments of every argument" | UNVERIFIABLE (plausible, not directly traced) | — | This is a design rationale about overload resolution strategy; src/typing/overload_resolver.rs exists and does argument-driven lookup, but tracing the exact "look in each argument's environment" mechanism was not done in this pass — treat as still-plausible but unconfirmed in detail. |
| 4 | Closures: "when we create a closure struct, its `__call` has to be somewhere, we put it in its environment." | UNVERIFIABLE (renamed/restructured, not traced in detail) | src/typing/function/function_compiler_closure_or_light_layer.rs (closure-related compiler exists) | Naming (`__call`) not confirmed to survive verbatim in Rust; not load-bearing to the arcana's core claim, so left unverified rather than chased further. |

## Stale citation sites
None — TMRE has zero code citations (confirmed via grep -rn -w "TMRE" across src/, Backend/, docs/, excluding target/tmp/guardian-logs/Guardian/Luz/docs/convos): only self-references inside the same old doc file (docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md:169, :501).

## Uncited sites that embody the arcana
- src/typing/templata/templata.rs:182-186 — `FunctionTemplataT` pairs a function with `outer_env`.
- src/typing/templata/templata.rs:208-212 — `StructDefinitionTemplataT` pairs a struct definition with `declaring_env`.
- src/typing/templata/templata.rs:330-334 — `InterfaceDefinitionTemplataT` pairs an interface definition with `declaring_env`.
- src/typing/templata/templata.rs:341-345 — `ImplDefinitionTemplataT` pairs an impl with `env`.
- src/typing/templata/templata.rs:203-205 — comment marking the very tension TMRE's claim glosses over (inconsistent env-carrying across templata kinds), a good anchor for a migrated doc.

## Suggested text
Not required for D3 kind unless major-inaccuracies or obsolete — this is minor-inaccuracies at most (claim 2's universality caveat), so omitted. If migrated, the doc should be updated to note explicitly that only definition-kind templatas (function/struct/interface/impl) carry an environment, not every `ITemplataT` variant, and should link the AFTERM comment at src/typing/templata/templata.rs:203 as the live open question the original rationale left unresolved.
