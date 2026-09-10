# Accuracy report: Only Work with Placeholders From the Root Denizen (OWPFRD)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:62 ("# Only Work with Placeholders From the Root Denizen (OWPFRD)")

## Verdict
The core idea (a placeholder like `SomeStruct$0` must be substituted away before it reaches a caller like `myFunc`, and placeholders are named/prefixed by their owning container so they don't collide) is still true of the current code — placeholder names in `src/typing/templata_compiler.rs` and `src/typing/names/names.rs` are interned with a `template` field identifying the owning denizen, matching the "prefix with container name" claim. However, the section's closing claim — "We also have a sanity check in the solver … search for OWPFRD" — does not hold: there is no code anywhere in the repo (src/, Backend/, docs/) that cites OWPFRD, and no solver check discoverable specifically by that string. The literal instruction to the reader ("search for OWPFRD") is now a dead end.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `SomeStruct.x` in the template is typed `SomeStruct$0`, substituted to `int` for `thing.x` | TRUE (design-level) | src/typing/templata_compiler.rs:1903-1963 (placeholder creation/substitution machinery) | — |
| 2 | Placeholders are prefixed by their container's name "both in FullNameT and here in the docs" | DRIFTED | src/typing/names/names.rs, src/typing/templata_compiler.rs:1920-1922 (`KindPlaceholderNameT { template: template_name }`, `INameT::KindPlaceholder`) | The Scala-era `FullNameT` type doesn't exist under that name in Rust; the equivalent naming (placeholder name carrying a `template` field identifying its owner) does exist under `INameT`/`KindPlaceholderNameT`. |
| 3 | "We also have a sanity check in the solver to make sure that we're never dealing with foreign placeholders, search for OWPFRD" | FALSE / stale | grep for `OWPFRD` across src/, Backend/, docs/ returns only the two lines inside Generics.md itself | No code cites OWPFRD. The closest candidate is an assertion in src/typing/edge_compiler.rs:366 (`assert!(impl_placeholder_id.init_id(...) == impl_t.template_id)`), but it isn't in "the solver" (it's in edge_compiler.rs, dispatcher-placeholder construction) and isn't tagged OWPFRD. The promised search-string leads nowhere. |

## Stale citation sites
None — there are no citation sites at all (see claim 3); the section promises one and it doesn't exist.

## Uncited sites that embody the arcana
- src/typing/edge_compiler.rs:366 — assert that an impl placeholder's owning template equals the impl's template, i.e. a "don't use a foreign placeholder" check, uncited.
- src/typing/templata_compiler.rs:1920-1922 — placeholder name creation stamping the owning template into the name, the mechanism the doc describes as "prefix with container name," uncited.

## Suggested rewrite
Only the last paragraph needs correction; the rest of the section stands:

> We also have sanity checks that placeholders stay scoped to their owning denizen — for example `src/typing/edge_compiler.rs` asserts that a dispatcher's impl placeholder shares its `template_id` with the impl it came from. There is currently no code literally tagged with the OWPFRD citation; if you add one, cite it here.
