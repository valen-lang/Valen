# Accuracy report: Same Interface Tags Twice (SITTX)

Audited against the working tree on 2026-09-06. Source: reconstructed from citing code — sole comment at Backend/src/region/rcimm/rcimm.cpp:962.

## Verdict
The reconstructed concern is exactly right: `RCImm::generateInterfaceDefsC` (Backend/src/region/rcimm/rcimm.cpp:960-967) and `RCImm::defineConcreteTypeTagFunction` (Backend/src/region/rcimm/rcimm.cpp:1301-1330) both iterate the same `edgesByInterface[interfaceName]` vector returned by `getEdgesForInterface`, and both use the vector index as the tag value — the `#define ..._TAG_<struct> <i>` in one and the `LLVMConstInt(int32LT, (uint64_t)i, ...)` select-chain result in the other. That vector is populated once, in declaration order, by a single `push_back` in `RCImm::declareEdge` (rcimm.cpp:313-315), so the two sites are structurally guaranteed to agree — there's no separate ordering to drift. All claims are TRUE. This is a real, still-live invariant with exactly one comment and no arcana backing it. Recommendation: migrate into docs/arcana as a proper Z-suffix doc (short one, e.g. "SameInterfaceTagsTwice-Z"), cited from both rcimm.cpp:962-ish and rcimm.cpp:1301, since the invariant spans two functions and is easy to break if either site's iteration order or source vector ever diverges.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `generateInterfaceDefsC` emits `#define <name>_TAG_<struct> <i>` per edge | TRUE | Backend/src/region/rcimm/rcimm.cpp:966-967 | — |
| 2 | The runtime typeTag body (`defineConcreteTypeTagFunction`) also assigns tag `i` per edge, in the same order | TRUE | Backend/src/region/rcimm/rcimm.cpp:1301-1325 | — |
| 3 | Both iterate the same `edgesByInterface[interfaceName]` vector via `getEdgesForInterface` | TRUE | Backend/src/region/rcimm/rcimm.cpp:963, 1303, 333-336 | — |
| 4 | That vector is built by `push_back` in `RCImm::declareEdge`, in declaration order, with a comment naming this exact invariant | TRUE | Backend/src/region/rcimm/rcimm.cpp:304-317 | comment at line 313-314 reads "Track edge order so the typeTag body and generateInterfaceDefsC agree on the tag value assigned to each substruct" — matches reconstruction verbatim |
| 5 | SITTX was coined/cited only in this one comment, no arcana doc, no other commit references it | TRUE | grep across src/, Backend/, docs/ (excluding convos) returns only rcimm.cpp:962 | — |

## Stale citation sites
None — the single site (rcimm.cpp:962) accurately describes the current code.

## Uncited sites that embody the arcana
- Backend/src/region/rcimm/rcimm.cpp:304-317 (`RCImm::declareEdge`) — already has its own explanatory comment but doesn't cite SITTX by name; natural second citation site.
- Backend/src/region/rcimm/rcimm.cpp:1301-1330 (`RCImm::defineConcreteTypeTagFunction`) — the runtime half of the invariant; currently uncited.
- Backend/src/region/rcimm/rcimm.cpp:333-336 (`RCImm::getEdgesForInterface`) — the shared accessor both sites rely on; worth a one-line pointer to the arcana.

## Suggested text
**Same Interface Tags Twice**: An interface's runtime type tag (used by `typeTag()` in generated C and by the exported `TAG_*` `#define`s) is computed independently at two codegen sites — the C header generator and the runtime `typeTag` function body — rather than being looked up from one shared table. Both sites derive the tag purely from the position of an edge (interface→struct implementation) in `RCImm`'s `edgesByInterface` vector, which is populated once in declaration order. Because both consumers read the same backing vector by index, the two independently-generated tag numberings are guaranteed to agree without any explicit cross-check; this note exists so that anyone changing how edges are declared, filtered, or ordered — including introducing a second declaration path or a sort — verifies both codegen sites are still driven from the same ordered source, since nothing else enforces that they match.
