# Accuracy report: When Values Should Be Interned (WVSBIZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md

## Verdict
The seven arena-vs-inline principles and the "Interned Set" list are still substantially accurate — the sealed 5-kind-payload + `IdT` + 15-name interning machinery in `src/typing/typing_interner.rs` matches the doc's "Interned Set" section exactly, and is corroborated by `docs/arcana/SealedInternedConstruction-SICZ.md:11` and `docs/architecture/typing-pass-design-v3.md:386`. Two things have drifted, though: (1) the doc's example for the Recursion principle (`IEnvironmentT` holding `parent: &'t IEnvironmentT`) no longer matches the code — `IEnvironmentT` is now a Copy tagged-pointer (polyvalue) enum, and concrete environments hold `parent_env: IEnvironmentT` by value, not `&'t IEnvironmentT`; and (2) the claim that `SignatureT`/`PrototypeT` "stay as ... arena-allocated-non-interned" is overstated — `TypingInterner::intern_signature`/`intern_prototype` do perform real dedup-and-cache interning (a `val_to_ref` HashMap keyed on structural equality, returning a canonical `&'t` allocation), even though the resulting type is not sealed with `MustIntern` and is consumed as a Copy value at call sites. This is minor-inaccuracies: the core mechanism (arena-vs-inline, when-to-intern, the sealed set) is intact, but two of the doc's illustrative examples no longer describe the current shape of the code.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Arena vs inline decided by 7 principles (size, dynamic length, interned, identity, sharing, recursion, back-pointers) | TRUE | src/typing/types/types.rs:49 (primitives inline, compounds `&'t`) | |
| 2 | `IRuneS<'s>` is a Copy tagged pointer whose `&'s CodeRuneS<'s>` payload lives in the arena | TRUE | src/postparsing/names.rs:899-906, :1324 | |
| 3 | Recursion example: `IEnvironmentT` holds `parent: &'t IEnvironmentT` | DRIFTED | src/typing/env/environment.rs:76-89, :1206-1215 | `IEnvironmentT` is now a Copy polyvalue enum of `&'t` refs to concrete env structs (`Package`, `Citizen`, `Function`, …); the actual recursive field is `CitizenEnvironmentT::parent_env: IEnvironmentT<'s,'t>` (by value, no `&'t` on the field itself) — indirection lives inside the enum's variant pointers, not on the field. |
| 4 | Definitional components (`ParameterT`, `NormalStructMemberT`) are arena-allocated with their parent | TRUE (spot-checked types exist) | src/typing/types/types.rs (struct defs) | |
| 5 | "When Arena-Allocated Becomes Interned": subcollections / size / enum-budget triggers interning via `intern_*` | TRUE | src/typing/typing_interner.rs:170-650 (many `intern_*` wrappers) | |
| 6 | Interned Set = `INameT`/`ICitizenTT` variants, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `OverloadSetT`, `IdT` | TRUE | src/typing/typing_interner.rs:553-612 (`intern_kind_payload` match arms); docs/arcana/SealedInternedConstruction-SICZ.md:11 (same 5+IdT+15-names sealed list) | |
| 7 | "Other types (Templata variants, `SignatureT`, `PrototypeT`, `KindPlaceholderT`) stay as Value-types or arena-allocated-non-interned per @TFITCX even when they superficially fit one of the heuristics above" | DRIFTED | src/typing/typing_interner.rs:503-544 (`intern_prototype`/`intern_signature` do a `val_to_ref` HashMap lookup-or-insert exactly like the sealed interned types); src/typing/ast/ast.rs:231-241, :423-429 (`SignatureT`/`PrototypeT` tagged "Value-type (see @TFITCX)") | `SignatureT`/`PrototypeT` are not *sealed* (no `MustIntern` field, so a bare struct literal still compiles) and are consumed as Copy values at call sites, but `TypingInterner` does deduplicate them through a genuine intern-and-cache path. The doc should say they are **unsealed but still deduped by convention through `intern_signature`/`intern_prototype`**, not flatly "non-interned." `KindPlaceholderT` itself is accurate as stated — it goes through `intern_kind_payload`'s dispatch but is excluded from the SICZ sealed list and keeps structural `PartialEq`/`Hash` (src/typing/types/types.rs:401-404), matching "Value-type" semantics. |
| 8 | Once Interned, construction must go through the interner (seal enforcement), see @SICZ | TRUE | docs/arcana/SealedInternedConstruction-SICZ.md:11 | |

## Stale citation sites
- `docs/architecture/typing-pass-design-v3.md:386` — quotes "@WVSBIZ ('Scala Parity Overrides The Heuristics')" as if that were a section heading inside the WVSBIZ doc. No such heading exists in the current `docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md` (the doc's headings are "Arena vs Inline: Seven Principles", "When Arena-Allocated Becomes Interned", "The Interned Set"). Minor drift, doesn't affect correctness but the quoted title is now fictitious.

## Uncited sites that embody the arcana
- `src/typing/env/environment.rs:1206-1215` — `CitizenEnvironmentT::parent_env: IEnvironmentT<'s,'t>`, the actual current instance of the Recursion principle's canonical example; not tagged `@WVSBIZ`.
- `src/typing/ast/ast.rs:231-241` — `SignatureT` struct, tagged only `@TFITCX`, not `@WVSBIZ`, despite being exactly the case the arcana's "Interned Set" section calls out by name.
- `src/typing/ast/ast.rs:423-429` — `PrototypeT` struct, same as above.
- `src/typing/typing_interner.rs:503-527` — `intern_prototype`/`intern_signature`, the actual dedup-via-hashmap interning logic for those two types; not tagged `@WVSBIZ` even though it's a load-bearing example of the "Interned" mechanism definition ("deduplicate via `intern_*` so structurally equal values share a pointer").

## Suggested rewrite
Not included — verdict is minor-inaccuracies, core mechanism intact; a full rewrite is not warranted. The two corrections above (item 3's `IEnvironmentT` example, item 7's `SignatureT`/`PrototypeT` characterization) are the only prose that needs touching.
