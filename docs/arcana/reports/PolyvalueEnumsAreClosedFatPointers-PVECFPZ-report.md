# Accuracy report: Polyvalue Enums Are Closed-Set Fat Pointers (PVECFPZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/PolyvalueEnumsAreClosedFatPointers-PVECFPZ.md

## Verdict
The core mechanism is intact and well-supported by code: `IEnvironmentT`, `IInDenizenEnvironmentT`, `IEnvEntryT`, `ITemplataT`, `KindT`, `INameT` are all still Copy enums carrying `#[derive(PartialEq, Eq, Hash)]` and citing `@PVECFPZ` next to `@TFITCX`, and the eq/hash-trap narrative matches the manual `self.id == other.id` impls found on the variant env structs. Two things are off: the doc's flagship "textbook example," `IEnvEntryT`, misdescribes its payload shape (its Function/Struct/Interface/Impl variants hold small wrapper structs containing a `&'t IdT` field, not bare `&'s` refs to the AST nodes themselves), and one of the listed citation sites (src/typing/types/types.rs:226) is inside a commented-out (dead) enum definition, not live code. Neither undermines the mental model, so this is minor-inaccuracies, not major.

## Claims
| # | Claim (quoted or closely paraphrased from the doc) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A polyvalue enum is a `Copy` wrapper enum, ~16 bytes, whose variants hold non-owning values; canonical examples `IEnvironmentT`, `IInDenizenEnvironmentT`, `IEnvEntryT`, `ITemplataT`, `KindT`, `INameT` | TRUE | src/typing/env/environment.rs:74-90, src/typing/env/i_env_entry.rs:40-50, src/typing/templata/templata.rs:68-88, src/typing/types/types.rs:52, src/typing/names/names.rs:149-151 | |
| 2 | `&dyn Trait` vs polyvalue: same 2-word layout, Copy-by-value, identity-based equality; dispatch differs (vtable vs match) | TRUE (conceptual, not independently checkable in code) | — | |
| 3 | Per-variant ref-or-value: identity-bearing payloads get non-owning refs/handles, no-identity payloads get inline values, e.g. `MutabilityTemplataT(MutabilityT)` | TRUE | src/typing/templata/templata.rs (grep confirms `MutabilityTemplataT` inline-value pattern exists among ITemplataT-family types) | |
| 4 | `IEnvEntryT` textbook example: "four variants hold `&'s` refs to identity-bearing AST (Function, Struct, Interface, Impl), one variant holds an `ITemplataT`" | DRIFTED | src/typing/env/i_env_entry.rs:40-49 (`Function(FunctionEnvEntry<'s,'t>)`, `Struct(StructEnvEntry<'s,'t>)`, `Interface(InterfaceEnvEntry<'s,'t>)`, `Impl(ImplEnvEntry<'s,'t>)`); src/typing/env/i_env_entry.rs:6-38 shows `FunctionEnvEntry`/`StructEnvEntry`/`InterfaceEnvEntry`/`ImplEnvEntry` are small structs holding `template_id: &'t IdT<'s,'t>` (plus a `tyype` field on two of them), not bare `&'s FunctionA`/`StructA`/`InterfaceA`/`ImplA` refs | The four variants hold small Copy wrapper *structs* (each carrying a `&'t IdT` handle, not a direct AST reference) — still non-owning/identity-based, but the doc's "hold `&'s` refs to identity-bearing AST" phrasing is no longer literally accurate. |
| 5 | Eq/hash trap: hand-rolled `ptr::eq(self, other)` on outer `&self` breaks under by-value use; polyvalues must `#[derive(PartialEq, Eq, Hash)]` | TRUE | All six canonical types carry `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` immediately after an `/// Polyvalue ... @PVECFPZ` comment, e.g. src/typing/env/environment.rs:74-75, src/typing/templata/templata.rs:68-69, src/typing/names/names.rs:149-150 | |
| 6 | Derived eq delegates into variant env's `self.id == other.id`, `IdT` is sealed/canonical so it reduces to identity eq | TRUE | src/typing/env/environment.rs:1307-1312 (`CitizenEnvironmentT` `PartialEq` impl uses `self.id == other.id`), same pattern at src/typing/env/function_environment_t.rs:60,154,847 and environment.rs:1423,1496 | |
| 7 | "When to use the Polyvalue category" criteria (enum, Copy, non-owning payloads, not canonical owner) | TRUE (consistent with @TFITCX shield doc wording) | docs/shields/TypesFitIntoTheseCategories-TFITCX.md:18 | |
| 8 | See-also links: @TFITCX, @IEOIBZ, @SICZ, @WVSBIZ all exist | TRUE | docs/shields/TypesFitIntoTheseCategories-TFITCX.md, docs/arcana/IdentityEqualityOnIdentityBearingTypes-IEOIBZ.md, docs/arcana/SealedInternedConstruction-SICZ.md, docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md | |

## Stale citation sites
- src/typing/types/types.rs:226 — the arcana citation here (`/// Polyvalue ... @PVECFPZ` on the line above `pub enum IRefKindTT`) sits inside a fully commented-out block (`// ... // pub enum IRefKindTT<'s, 't> { ... }`, types.rs:223-244). The enum and its citation are dead code, not a live polyvalue instance. Not misleading per se (a reader will see the `//` prefix), but it's no longer a real citation site and should probably be dropped or the dead code removed.

All other listed sites (docs/architecture/instantiator-design.md:652, simplifier-design.md:96, typing-pass-ai-guide.md:213, typing-pass-design-v3.md:76/167/220/489, TFITCX.md:84, environment.rs:74/245, names.rs:149, templata.rs:68, types.rs:22/29/35/41/50/246/288/324/436, typing-pass-todo.md:17) still sit directly above or describe a live `#[derive(Copy, Clone, PartialEq, Eq, Hash, ...)]` polyvalue enum or accurately restate the eq/hash-trap rule; no other drift found.

## Uncited sites that embody the arcana
None found beyond the already-cited canonical set — `src/instantiating/ast/names.rs:72` and `src/instantiating/ast/ast.rs:323` also carry the same `/// Polyvalue ... @PVECFPZ` comment (outside the given citation list but already correctly cited, not missing).

## Suggested rewrite
Not applicable — verdict is minor-inaccuracies. Suggested edit to §"Per-variant ref-or-value" paragraph in the doc: replace

> `IEnvEntryT` is the textbook example: four variants hold `&'s` refs to identity-bearing AST (Function, Struct, Interface, Impl), one variant holds an `ITemplataT` (itself a polyvalue).

with something like:

> `IEnvEntryT` is the textbook example: four variants (`Function`, `Struct`, `Interface`, `Impl`) hold small Copy wrapper structs (`FunctionEnvEntry`, `StructEnvEntry`, `InterfaceEnvEntry`, `ImplEnvEntry`) each carrying a `&'t IdT` handle back to the identity-bearing AST node, and one variant holds an `ITemplataT` (itself a polyvalue).
