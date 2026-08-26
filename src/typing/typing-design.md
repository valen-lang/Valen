# Typing Pass Design

## Design (human-only)

(very incomplete, working on it)

### LIFE/LID

We shouldn't use source location to uniquely identify something, because source location will change any time the user makes an edit, and we want things to be relatively stable (for LSP, incremental compilation, etc.).

Because of that, we made **LocationInDenizen** and **LocationInFunctionEnvironment** which are the same thing but in different passes.

**LocationInDenizen** ("LID") is created by postparser, and it's simply a list of numbers: 1.1 means "the first-thing's first-child". 1.3.2 means "first-thing's third-child's second-child". LIDs never contain 0.

**LocationInFunctionEnvironment** ("LIFE") is the same thing, but created in the typing pass (for example for typing-pass-made temporary variables). LIFEs start at 0. If there's a 0 in it, then you know it's a location conjured in the typing pass. All LIFEs should start from a LID. If typing pass wants three children for LID 1.3.2, then it should make LIFE 1.3.2.0. Then from there, one can make any number of LIFE children for it, like 1.3.2.0.0, 1.3.2.0.1, 1.3.2.0.2.

The typing pass should only ever construct one LIFE from any particular LID. If we do that, then all these are guaranteed collision-free.

Every declaration in a function (local variable declaration, closure declaration, etc.) and expression node (let, call, etc) should have a LID or LIFE. Every NodeEnvironmentT should have a unique LIFE.

Every top-level denizen gets an emtpy LID/LIFE to start with, but lambdas do not. They are part of their parent's LID/LIFE.

Nobody should ever make anything with an empty LID/LIFE (`[]`), except when seeding the original LID/LIFE for the entire top-level function/struct (NOT for struct members, function parameters, expressions, lambdas, etc).

### Names

There are two kinds of names throughout the compiler:

 * An **imprecise name**, which acts like someone's first name ("Mike"). There will be many Mikes.
 * A **declaration name**, which is absolute and unique. There's many Mikes, but only one "Mike whose SSN is 123-456-7890".

A declaration name is made of two things:
 * A location (LIFE/LID). This alone is enough to identify it.
 * An imprecise name. This is for convenience (makes it a little easier for typing pass to know what a declaration's imprecise name). The instantiator lowers this to a string so that the backend is decoupled from all the frontend's name variants.

An imprecise name:
 * Must be interned, always. Even the one in the declaration name.
 * Might contain a string, or might not. Typing pass should never look at or care about the raw string in an imprecise name, it should be looking things up by the whole imprecise name.

### Interners

If something is internable, there should be no way to make it other than via an interner.

## Design Proposals

<!-- Claude adds concise simple proposals here. The human ratifies by moving them up into the Design section above. -->

**P2 — The typing pass ignores regions and groups.**
Per the borrow-checker design, the typing pass does nothing with regions or groups. It ignores them
entirely rather than tracking or solving them: a rule's region (e.g. a `BorrowRef`'s region) is not a
rune the solver treats as something it must conclude.

**P1 — Give struct members a location too; keep `IVarNameT` unified.**
Every name = an imprecise name + a location (LID/LIFE). Let that include struct members: a member is
declared at a source point, so it can carry that point's LID, just like a local. A member having a
location does no harm, and it means *every* `IVarNameT` variant has one — so we don't split members
out of `IVarNameT`, and `IVarNameT::life()` becomes total.

Changes:
- LID and LIFE stay two distinct types (`LocationInDenizen` vs `LocationInFunctionEnvironmentT`). A
  postparse-declared name's `life` is its LID put into LIFE space through a single typed `Lid → Life`
  conversion — that one seam is what makes the reserve-`0` rules (LIDs never contain 0; a LIFE always
  starts from a LID) compiler-enforced instead of convention.
- `LocalNameT { name, lid }` → `{ imprecise_name: &CodeNameS, life }` — store the concrete
  `CodeNameS` (the spelling payload), not the full `IImpreciseNameS` enum, since a local/member
  is always a `CodeName`. `MemberNameT` takes the identical shape.
- Add a `lid` field to `NormalStructMemberS`, `ClosureParamNameS`, and `ConstructingMemberNameS`. The
  postparse `lidb` cursor is already in scope at each, so it's just adding the field. (Closure captures
  already carry the parent local's LID.)
- Give the remaining location-less variants a location: the range/`code_location` ones get a LID; the
  singletons (`Self_`, `TypingPassFunctionResultVar`) get the function-root life.
- Then metal lowers `IVarNameI` → `VarNameM { name, life }` uniformly.

This depends on one rule: **look members up by spelling, never by full name** (else two members
spelled the same get different LIDs and stop matching). One site breaks it today and must switch to a
spelling compare — `fn get_member_and_index` in `src/typing/ast/citizens.rs`, whose only caller is the
`.`-access in `src/typing/expression/expression_compiler.rs`.

## Details

**Names → `StrI` lowering** (from Design "Names"). Within typing, `LocalNameT`/`MemberNameT` hold
`imprecise_name: &CodeNameS`, never a bare `StrI`; lookups and compares use the whole imprecise name.
The error humanizer displays a name by handing its imprecise name to `humanize_imprecise_name`
(`src/postparsing/post_parser_error_humanizer.rs`). The instantiator's `translate_var_name`
(`src/instantiating/instantiator.rs`) is the lowering boundary: it humanizes each imprecise name to a
`StrI` (needs the codemap and the `'s` interner threaded in), so `LocalNameI`/`MemberNameI` carry a
lowered `StrI`.

## Discussed Examples and Test Cases

## Background

### Landed and tested (postparse — green standalone)

A declaration name (`IVarDeclarationNameS`) is identity-bearing and never interned (@WVSBIZ), and
**every variant now carries its `lid`** in a `*NameDeclarationS { <payload>, lid }` (e.g.
`CodeVarNameS { name, lid }`, `IterableNameDeclarationS { range, lid }`, `ClosureParamNameDeclarationS
{ code_location, lid }`), built directly. The Val form `IVarDeclarationNameValS`, `INameValS::VarName`,
and `alloc_var_name_canonical` are deleted — declaration names are no longer interned. Postparse mints
the lids at every construction site (pattern locals/params in `pattern_scout.rs`, loop vars, magic
params in `expression_scout.rs`, closure param + desugared param in `function_scout.rs`).

Use-sites carry imprecise names (`LocalLoad`/`Unlet`/`LocalMutate`/`GroupS::Local`), via `imprecise_name`
(a method on both `IVarDeclarationNameS` and `IVarNameT`). A `self.x` constructing-member **read**
resolves to the Let-pattern's declaration through `IImpreciseNameS::ConstructingMemberImpreciseName` (a
new variant), matched by spelling in `VariableDeclarations::find`; it never mints a second declaration
(the Let pattern is the sole declarer). Lesson: use-sites correlate to declarations by *imprecise name*,
never by full lid-bearing identity — a use that mints its own lid de-correlates and surfaces as a
spurious closured/unresolved name.

**Lambda LID nesting** (Design: "top-level denizens get an empty LID... lambdas do not"): `scout_function`
roots its `lidb` at a `denizen_root_path` argument — empty for a top-level function / interface method,
the lambda's own unique LID for a lambda. That LID is minted at the `IExpressionPE::Lambda` arm in
`expression_scout.rs` (`lidb.child()`, one per lambda whether or not it captures) and threaded through
`scout_lambda`. Guarded by `sibling_lambdas_get_distinct_lids` in `post_parser_tests.rs`. The full
postparse suite is green standalone (98 tests) and deterministic.

The former `CodeVarNameT` is now `MemberNameT` (struct members only; `IVarNameT::CodeVar` is `Member`).

### Typing re-link in progress — the crate does NOT compile

The typing pass (and instantiating/backend_ffi/pass_manager/testvm) is re-linked in `lib.rs`. Its
**name structs are reshaped, but the function logic is unfinished**, so the whole crate is currently
red. This is expected mid-work, not a regression to diagnose.

- Structs in place: `LocalNameT`/`MemberNameT`/`ConstructingMemberNameT` = `{ imprecise_name: &CodeNameS,
  life }`; `ClosureParamNameT` = `{ imprecise_name: &ClosureParamImpreciseNameS, life }`; the loop-var /
  `Self_` / function-result names are life-only; `MagicParamNameT` = `{ index, life }`.
  `LocationInFunctionEnvironmentT::from_lid` converts a declaration's lid to its life as a plain path
  copy (no `.0` — a declaration's life *is* its lid).
- Unfinished logic (do not treat these panics/errors as bugs): `translate_var_name_step`
  (`name_translator.rs`) must fill each variant's `life` from its source lid via `from_lid` and its
  `imprecise_name`; the magic-param `index` is computed in `assemble_function_params`
  (`function_compiler_middle_layer.rs`) from the function's ordered params (`FunctionS.params` carries the
  magic params in order at `function_scout.rs`), NOT stamped in postparse; `get_member_and_index`
  (`ast/citizens.rs`) must compare by spelling; plus the cross-file cascade from the name-shape changes,
  `SelfName`-now-a-tuple, and `ConstructingMemberName`-now-a-struct.
- `life` for typing-conjured vars (temporaries, block-results) is still minted fresh, not seeded from a
  lid via the `.0` rule — the collision-free continuation is unbuilt.

### Next session

Every `*NameDeclarationS` should **embed its corresponding `*ImpreciseNameS`** instead of the raw field —
the uniform shape `{ imprecise_name: &*ImpreciseNameS, lid }`. All 11 need it (`CodeVarNameS`,
`ConstructingMemberNameDeclarationS`, `ClosureParamNameDeclarationS`, `MagicParamNameDeclarationS`,
`Iterable`/`Iterator`/`IterationOption`/`WhileCondResultNameDeclarationS`, `SelfNameDeclarationS`,
`AnonymousSubstructMemberNameDeclarationS`, `DesugaredParamNameDeclarationS`); today each holds the bare
`name`/`range`/`code_location`/`index` (or nothing). `ClosureParamImpreciseNameS` and `SelfNameS` are
empty markers.

## Open Questions
