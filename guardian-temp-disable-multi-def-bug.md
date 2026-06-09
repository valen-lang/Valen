# Bug report: temp-disable placement edge cases

**Reported:** 2026-05-29 — multi-def part **RESOLVED 2026-06-09**
**Repo:** Vale, mid-migration session
**Endpoint:** Guardian `POST /temp-disable` (HTTP)

## Multi-def disambiguation — RESOLVED

Originally JR couldn't disambiguate when a definition name resolved to multiple defs (struct + impls), because the MCP tool surface didn't expose `target_line`. **Resolved 2026-06-09**: the MCP tool path was retired in favor of `curl`-POSTing directly to Guardian's `/temp-disable` HTTP endpoint, which natively accepts `target_line` (see `Guardian/src/serve/temp_disable.rs`). Callers (TL or JR) include `"target_line": <N>` in the JSON body when a name has multiple defs. VRBX auto-allows the curl pattern so it goes through silently.

## Workaround used in-session (now obsolete)

TL (ordained) ended up fixing the underlying slicing instead, which sidestepped the temp-disable. With the curl path, the disambiguation works directly — no need to restructure the def or bypass with ordination.

## Related failure mode: wrong-adjacent-function placement — STILL OPEN

Observed 2026-05-30 in `FrontendRust/src/simplifying/struct_hammer.rs`: a temp-disable on `translate_member` (line 382) landed at lines 442-443 — inside `make_box`'s `/* */` audit block (which starts at line 441) instead of inside `translate_member`'s block (lines 394-423). As a result, the disable doesn't take effect on `translate_member`; its next edit attempt fires the same SPDMX verdict.

This is a server-side placement bug: the endpoint can locate the function by name but mis-targets where to insert the comment when two functions and their audit blocks are nearby. Now that callers can pass `target_line`, the server could use it to choose the *correct* audit block when multiple candidates are adjacent — the line resolves both "which def" AND "which trailing /* */".

TL workaround used: manually relocate the inserted comment from the wrong audit block to the right one. Mechanical text move; should be unnecessary if the server picks placement reliably from `target_line`.

## Related failure mode: no-trailing-/* */-block — STILL OPEN

Observed 2026-06-09 in `FrontendRust/src/instantiating/ast/names.rs`: temp-disables targeting `IFunctionNameI::template_args` (line 552) and `IInstantiationNameI::template_args` (line 437) failed with `{"success":false,"error":"Definition 'template_args' has no trailing /* ... */ post-comment. Cannot insert temp-disable directive."}`. Both dispatcher impls had their parent-trait Scala in a leading `/* */` block above the impl, but no trailing block to inject the directive into.

TL workaround: hand-insert empty `/* */` blocks after each impl. Mechanical scaffolding. The endpoint could synthesize a fresh trailing block (`/* */`) as part of the directive landing when none exists, removing the need for the human to scaffold first.
