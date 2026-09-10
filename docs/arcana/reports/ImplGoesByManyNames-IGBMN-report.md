# Accuracy report: Impl Goes By Many Names (IGBMN)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Impls.md:194 ("# Impl Goes By Many Names (IGBMN)")

## Verdict

The core mechanism described (a Scala-era `IImplTemplateNameT` triple of `ImplTemplateSuperNameT` / `ImplTemplateSubNameT` / `ImplTemplateDeclareNameT`, used to look impls up by either interface or by struct) no longer exists in the Rust port. There is now a single `ImplTemplateNameT` (src/typing/names/names.rs:975), keyed by `code_location` plus both the sub-citizen and super-interface imprecise names, not three separate lookup names. Lookup by interface vs. by struct is done differently now (via `HinputsT`/`EdgeT` indices in src/typing/hinputs_t.rs, not via alternate name variants). The anonymous-substruct naming claim (interface name + `AnonymousSubstructName()`) is the one part that still roughly holds, via `AnonymousSubstructImplTemplateNameT` which is keyed by `interface`. No code anywhere cites IGBMN, so there are no stale citation sites to flag, but the doc body itself is stale/false about the core "three names" mechanism.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Impls have three names: `ImplTemplateSuperNameT`, `ImplTemplateSubNameT`, `ImplTemplateDeclareNameT` | FALSE | src/typing/names/names.rs:975-978 defines a single `ImplTemplateNameT{code_location, sub_citizen_imprecise_name, super_interface_imprecise_name}`; no `ImplTemplateSuperNameT`/`ImplTemplateSubNameT` types exist anywhere (`grep -rn "ImplTemplateSuperName\|ImplTemplateSubName"` returns nothing) | Only one impl-template name type exists now, carrying both imprecise names plus the code location together, rather than three interchangeable lookup keys |
| 2 | This lets you look up all impls for a given interface, or for a given citizen | DRIFTED | src/typing/hinputs_t.rs:110 `lookup_impl_by_template`, :274 `lookup_impl` — lookup by interface/citizen is done through `HinputsT`/`EdgeT` indices, not through alternate impl-name variants | The two-way lookup capability still exists, but is implemented via separate index structures in `HinputsT`, not via the impl having multiple names |
| 3 | This means you can't write `impl ICollection<T> for Y where Y < MyThing<T>;` | UNVERIFIABLE | No current code models this trade-off explicitly; syntax and rationale not re-derivable from the Rust code | Likely still true in spirit but unverifiable against current code |
| 4 | Anonymous substructs are named as the interface name plus `AnonymousSubstructName()` | TRUE (drifted naming) | src/typing/names/names.rs:1509 `AnonymousSubstructImplTemplateNameT{interface: IInterfaceTemplateNameT, ...}`; src/typing/names/names.rs:216 `AnonymousSubstructNameT` variant | Naming convention holds, though the concrete Rust type is `AnonymousSubstructImplTemplateNameT`/`AnonymousSubstructNameT`, not the Scala-era name given |
| 5 | (Scala code block) `case class ImplTemplateSubNameT(subCitizenTemplateName...) extends IImplTemplateNameT`, `ImplTemplateSuperNameT(...)` | FALSE / obsolete | Same as claim 1 — these types don't exist in src/typing | This block is leftover Scala source that no longer matches any Rust type; should be removed or replaced with the current `ImplTemplateNameT`/`AnonymousSubstructImplTemplateNameT` definitions |

## Stale citation sites

None — `grep -rn -w "IGBMN"` across src/, Backend/, docs/ (excluding docs/convos) finds no citations of IGBMN anywhere outside docs/arcana/Impls.md itself.

## Uncited sites that embody the arcana

- src/typing/names/names.rs:975 — `ImplTemplateNameT`, the current single impl-template name (replaces the old three-name scheme)
- src/typing/names/names.rs:1509 — `AnonymousSubstructImplTemplateNameT`, keyed by interface (matches the anonymous-substruct naming claim)
- src/typing/hinputs_t.rs:110 — `lookup_impl_by_template`, the current interface/struct impl lookup mechanism
- src/typing/hinputs_t.rs:274 — `lookup_impl`

## Suggested rewrite

Impls have a single name, `ImplTemplateNameT`, keyed by the impl's code location plus both the sub-citizen's and super-interface's imprecise names:

```
pub struct ImplTemplateNameT<'s> {
  pub code_location: CodeLocationS<'s>,
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
}
```

Looking up all impls for a given interface, or all impls for a given citizen, is done through separate indices in `HinputsT`/`EdgeT` (src/typing/hinputs_t.rs), not by the impl carrying multiple alternate names as it did in the Scala compiler.

Anonymous substructs still follow the interface-first naming convention: their impl-template name (`AnonymousSubstructImplTemplateNameT`) is keyed by the interface they implement, which is the opposite of how a normal impl's name centers on the sub-citizen — this remains a little awkward but is fine.
