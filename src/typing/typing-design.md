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

### Names

There are two kinds of names throughout the compiler:

 * An **imprecise name**, which acts like someone's first name ("Mike"). There will be many Mikes.
 * A **declaration name**, which is absolute and unique. There's many Mikes, but only one "Mike whose SSN is 123-456-7890".

A declaration name is made of two things: an imprecise name plus a location (LIFE/LID).

## Design Proposals

## Details

## Discussed Examples and Test Cases

## Background

### Landed in the code

A declaration name (`IVarDeclarationNameS`) is identity-bearing and never interned (@WVSBIZ): a user
local's `CodeVarName` carries a `lid: LocationInDenizen`, and `LocalNameT { name, lid }` keys on that
lid rather than a typing-pass `life`, so `make_user_local_variable` translates uniformly with no
`CodeVarName` special-case. `imprecise_name` is a method on `IVarDeclarationNameS` (postparse) and
`IVarNameT` (typing); use-sites (`LocalLoad`/`Unlet`/`LocalMutate`/`GroupS::Local`) carry imprecise
names. The former `CodeVarNameT` is now `MemberNameT` (struct members only; `IVarNameT::CodeVar` is
`Member`).

### Design not yet in the code

- `life` is still minted fresh in the typing pass (the header setup in `function_compiler_core.rs`,
  extended by `LocationInFunctionEnvironmentT::add`), NOT seeded from a declaration's lid via the
  `.0` rule — the collision-free LID→LIFE continuation is unbuilt.
- The range/location declaration variants (`IterableName`/`IteratorName`/`ClosureParamName`/
  `MagicParamName`/...) still carry a `RangeS`/`CodeLocationS`, not a lid.
- `ClosureParamName`/`MagicParamName` are the only declaration names still interned (via
  `INameValS::VarName` in `function_scout.rs`); de-interning them and deleting
  `IVarDeclarationNameValS` is gated on confirming nothing compares `INameS::VarName` by `ptr_eq`.

## Open Questions
