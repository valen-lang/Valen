# Accuracy report: IRegion Interface in Backend (IRIIB)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/IRegion.md:1 ("# IRegion Interface in Backend (IRIIB)")

## Verdict
The core idea (IRegion is the abstract interface the backend compiles expressions against, with concrete subclasses per memory-safety strategy) is still true, and the doc has already been partly updated (it correctly marks LinearRegion RETIRED). But two of the still-live claims are now wrong: `receiveUnencryptedAlienReference` no longer exists on `IRegion` (removed along with the linearized-FFI scheme the doc itself says was retired), and the `--region-override=unsafe-fast` flag it cites for selecting UnsafeRegion no longer exists — Backend/src/vale.cpp now hard-codes Unsafe as the sole mutable region unconditionally.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | IRegion is the main interface for managing/accessing memory in the backend | TRUE | Backend/src/region/iregion.h:14 | — |
| 2 | `allocate`, `constructStaticSizedArray`, `loadMember`, `getRuntimeSizedArrayLength` are methods on IRegion | TRUE | Backend/src/region/iregion.h:18,70,122,166 | — |
| 3 | `receiveUnencryptedAlienReference` is a method on IRegion that copies an object from another region | FALSE | grep found zero occurrences in Backend/src; only appears in docs/todo/*.md as a retired name (docs/todo/remove-record-replay-on-experimental.md:47) | Method was removed with the opaque-handle FFI rework; drop this bullet or replace with the current opaque-handle mechanism |
| 4 | Subclasses include UnsafeRegion, RCImmRegion, LinearRegion (retired) | DRIFTED | Backend/src/region/unsafe/unsafe.h:13 (`class Unsafe : public IRegion`), Backend/src/region/rcimm/rcimm.h:14 (`class RCImm : public IRegion`) | Only two live subclasses, named `Unsafe` and `RCImm` (not `UnsafeRegion`/`RCImmRegion`) |
| 5 | UnsafeRegion used "for Unsafe blocks" and "the main mutable region, if --region-override=unsafe-fast" | FALSE | Backend/src/vale.cpp:695-697 unconditionally constructs `Unsafe` as `globalState->mutRegion`; the only `regionOverride`/ASSIST reference left is commented out at Backend/src/vale.cpp:612 | Unsafe is now always the mutable region — there is no `--region-override` flag selecting it |
| 6 | RCImmRegion is the region for immutable objects, and is the region behind the opaque-handle FFI | TRUE | Backend/src/region/rcimm/rcimm.h:14; Backend/src/vale.cpp:691 (`globalState->rcImmOwned = std::make_unique<RCImm>(...)`) | — |
| 7 | With IRegion, the backend stage compiles expressions against a common lower interface | TRUE | Backend/src/vale.cpp:706,724,733,742 etc. dispatch via `globalState->getRegion(...)->...` uniformly | — |

## Stale citation sites
None — no other file in src/, Backend/, or docs/ cites IRIIB (grep -rln -w "IRIIB" found only docs/arcana/IRegion.md itself).

## Uncited sites that embody the arcana
- Backend/src/region/iregion.h:14 — the IRegion interface definition itself.
- Backend/src/region/unsafe/unsafe.h:13 — Unsafe subclass.
- Backend/src/region/rcimm/rcimm.h:14 — RCImm subclass.
- Backend/src/vale.cpp:691-697 — region construction/wiring in main compile driver.

## Suggested rewrite
Replace the "subclasses" bullet list and drop the stale FFI-copy method:

```
 * allocate: allocates a struct and populates its members.
 * constructStaticSizedArray: allocates a static sized array.
 * loadMember: loads a member from a struct.
 * getRuntimeSizedArrayLength: gets an array's length.

There are two live subclasses today:

 * Unsafe: A region using no memory safety. It is currently the only mutable
   region — always constructed as globalState->mutRegion (Backend/src/vale.cpp),
   used for both unsafe blocks and ordinary mutable code. The old
   `--region-override` flag that used to select between region strategies is
   gone (commented out in Backend/src/vale.cpp).
 * RCImm: A region for immutable objects, and the region behind the
   opaque-handle FFI: imm values cross to/from C as handles, not linearized
   buffers.
 * LinearRegion: RETIRED (2026-07). Was a bump allocator that linearized imm
   values into buffers for C and for record/replay files. FFI moved to opaque
   handles and record/replay was removed; see the PSBCBO/PRCBO appendix in
   `todo/metaprogrammed-record-replay.md` for the retired scheme and its
   successor design.
```
