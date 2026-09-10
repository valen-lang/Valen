# Accuracy report: Compile Dispatcher Function Given Interface (CDFGI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1057 ("## Step 2: Compile Dispatcher Function Given Interface (CDFGI)")

## Verdict
The core mechanism the section describes still matches the code: `edge_compiler.rs` substitutes the dispatcher-case placeholders into the abstract function's parameter type and calls `evaluate_generic_virtual_dispatcher_function_for_prototype` to resolve the overload, exactly as narrated. However, the doc itself carries unresolved author TODO notes (`ZHERE:` lines immediately above the CDFGI header) that flag a genuine, still-present self-contradiction: the section heading says "Function" while the body text says "We're conceptually compiling a match's **case**", and the note explicitly instructs renaming the heading to "Compile Dispatcher Case Function Given Interface" while keeping the ID. This is a real, acknowledged-but-unfixed inaccuracy in the section's own wording, not merely a nit — it directly affects whether a reader understands what's being compiled (a case function vs. "the" dispatcher function). Rated minor-inaccuracies.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Given original abstract func `launch<X,Y,Z>(...)`, we "try to compile it given the dispatcher interface... as the first parameter" | TRUE | src/typing/edge_compiler.rs:383-390 builds `dispatcher_placeholdered_abstract_param_type` via `replace_value_type_in_ref` and calls it "Step 2: Compile Dispatcher Function Given Interface, see CDFGI" | — |
| 2 | Result: `abstract func launch(self ISpaceship<int, dis$0, dis$1>, bork int) where exists drop(dis$0)void;` | TRUE (as illustrative example) | consistent with substitution logic at edge_compiler.rs:361-390 | — |
| 3 | "We're conceptually compiling a match's case, from which we'll resolve a function" | DRIFTED / self-contradicts heading | docs/arcana/Generics.md:1050-1054 (ZHERE notes) explicitly say this line is "the one place this section gets it right — it just contradicts the heading" | Rename heading to "Compile Dispatcher Case Function Given Interface" per the doc's own pending TODO, or reword the body to match "Function" framing — the doc has not yet done either. |
| 4 | We did this because `bork X` becomes `bork int`, and bounds `drop(T)void` translate to `drop(dis$0)void` | TRUE | matches the substitution mechanism and `IBoundArgumentsSource` translation used at edge_compiler.rs:361-379 | — |
| 5 | "Now we have our inner environment from which we can resolve some overloads" | TRUE | edge_compiler.rs:394-401 calls `evaluate_generic_virtual_dispatcher_function_for_prototype` with `dispatcher_outer_env`, returning `dispatching_func_prototype` / `dispatcher_inner_inferences` | — |

## Stale citation sites
None found among cited sites — src/typing/edge_compiler.rs:390 and docs/architecture/instantiator-design.md:663 both correctly point to this section and the code there matches.

## Uncited sites that embody the arcana
None found beyond the existing citations.

## Suggested rewrite
Not required — no FALSE claim or changed mechanism. The correction needed is the one the doc's own ZHERE notes already specify: rename the heading to "Compile Dispatcher Case Function Given Interface" (keeping the CDFGI id), and adjust "try to compile it given the dispatcher interface" to "dispatcher case interface" for consistency with the body's correct "match's case" framing.
