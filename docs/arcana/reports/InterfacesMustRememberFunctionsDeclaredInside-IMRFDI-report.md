# Accuracy report: Interfaces Must Remember Functions Declared Inside (IMRFDI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Environments.md:1 ("## Interfaces Must Remember Functions Declared Inside (IMRFDI)")

## Verdict
This section is a Scala-era design-options musing (Option A "concept" keyword vs Option B "namespace scoping" vs Option C "declaration-only contract, with sealed-only external abstract functions"), written in old Vale surface syntax (`abstract CitizenRef2`, `interface IRulexTR`, `ITemplataType`) that no longer resembles current Vale/Rust naming. It ends with two open "note from later" questions and is not itself a settled, checkable spec — it proposes, it doesn't assert a final invariant the code must uphold. The one concrete decision it states ("We're going with C" — sealed-only external abstract methods) does have a living counterpart in current code (`src/typing/edge_compiler.rs`, `src/typing/compiler_error_humanizer.rs:273`, test `report_when_abstract_method_defined_outside_open_interface` at `src/typing/test/compiler_tests.rs:5189`), but the section never names any current type/function, and its only "site" citation is a bare listing entry with zero surrounding context (`docs/architecture/instantiator-design.md:663`, part of a comma-separated list of ~20 unrelated IDs — "various spot citations"). No code cites IMRFDI directly (`grep -rn -w IMRFDI` across src/ and docs/ turns up only that one list entry). Given the syntax is obsolete and the section makes no verifiable claim actually tied by name to present code, I'm calling this obsolete rather than accurate/minor — a reader following this arcana today would learn nothing usable about the current interface/abstract-method mechanism.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | External abstract functions like `def lookup(citizenRef: abstract CitizenRef2)` can be declared outside an interface, and this kind of external abstract function can only be used for sealed interfaces. | DRIFTED | src/typing/compiler_error_humanizer.rs:273; src/typing/test/compiler_tests.rs:5189-5230 | The idea survives ("Open (non-sealed) interfaces can't have abstract methods defined outside the interface"), but the syntax (`abstract CitizenRef2`, free global `def`) and the vocabulary (Scala `CitizenRef2`) are gone; current Vale abstract methods are declared with `abstract` as a param qualifier resolved via `get_abstract_interface()` in edge_compiler.rs, not documented anywhere near this wording. |
| 2 | "We're going with C" — only functions inside an interface's own declaration count as part of its contract (internal methods). | UNVERIFIABLE / DRIFTED | src/typing/rust_interop/declarations.rs:616-853; src/typing/edge_compiler.rs:138-202 | Modern code does track a notion of "internal methods" of an interface (e.g. synthesize_abstract_interface_method, abstract_function_headers keyed by interface template), consistent with the spirit of Option C, but the section gives no current type name, so this can't be checked claim-by-claim against source. |
| 3 | Option A (`concept` keyword) and Option B (namespace-scoped contract) were considered and rejected. | TRUE (as historical record) | — | No `concept` keyword exists in the current grammar/parser (not found in src/); this is accurately described as a rejected option, i.e. the claim is simply that these were considered, which is unfalsifiable design history, not a code claim. |
| 4 | "There's no way to have an interface express `concept Printer { fn __call(x: #X) Str; }`" (generic/templated interface methods). | UNVERIFIABLE | — | This is a forward-looking open question about generic method support in interfaces, not a claim about existing code; not checkable. |
| 5 | Macros like InterfaceFreeMacro add interface methods found via FullName prefix matching, collected during "compileInterface". | FALSE/OBSOLETE (name) | grep for `InterfaceFreeMacro`, `compileInterface` in src/: no matches | No `InterfaceFreeMacro` or `compileInterface` exists in current Rust code. The closest current concept is unrelated (rust_interop importer treats trait methods structurally, src/typing/rust_interop/importer.rs:136), not a FullName-prefix macro mechanism. |

## Stale citation sites
docs/architecture/instantiator-design.md:663 — this is a bare list entry ("...DDSOT, NNSPAFOC, EHCFBD, IMRFDI, UINIT, PRIIROZ..." — "various spot citations. Most defined in docs/; see grep.") with no surrounding discussion of interfaces, abstract methods, or the like-rule at all. It doesn't corroborate or contradict the arcana's content — it's just a pointer into a glossary, so it isn't really a functional citation of the mechanism.

## Uncited sites that embody the arcana
- src/typing/edge_compiler.rs:138 — `function.header.get_abstract_interface()` resolves which interface an abstract/external function belongs to.
- src/typing/compiler_error_humanizer.rs:273 — the sealed-only-external-abstract-method error message, the one rule this arcana actually settled on.
- src/typing/test/compiler_tests.rs:5189 — `report_when_abstract_method_defined_outside_open_interface`, the regression test for that rule.
- src/typing/rust_interop/declarations.rs:653 — `synthesize_abstract_interface_method`, synthesizing an interface's internal abstract methods from a Rust trait.

## Suggested rewrite
The section is a preserved historical design debate (Scala era, pre-decision) rather than a description of current machinery, and should probably be re-scoped as "historical note" rather than an arcana claiming to describe live code. A minimal fix:

> ## Interfaces Must Remember Functions Declared Inside (IMRFDI)
>
> *(Historical design note, Scala era — surface syntax below no longer matches current Vale; kept for context.)*
>
> [...existing options A/B/C discussion, unchanged...]
>
> **Current status:** we went with Option C. In the present Rust compiler, an interface's own internal (abstract) methods are what its "contract" consists of; a function *outside* the interface's declaration may still act as an abstract override only when the interface is sealed (see `src/typing/compiler_error_humanizer.rs` and the `get_abstract_interface` resolution in `src/typing/edge_compiler.rs`); open interfaces reject externally-declared abstract methods. The `InterfaceFreeMacro`/`compileInterface`/FullName-prefix mechanism described below no longer exists — interface-method collection today goes through `edge_compiler.rs` and, for Rust-interop trait imports, `synthesize_abstract_interface_method` in `src/typing/rust_interop/declarations.rs`.
