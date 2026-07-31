---
description: The compact shorthand for typed-AST ids used in Generics.md, investigation notes, and code comments.
g_read_when: "Read when decoding the compact id shorthand used in docs/Generics.md, investigation notes, or comments — forms like `dis$0`, `Foo<^int>`, `MySome.bound:drop:66`."
g_mention_in:
  - CLAUDE.md
---

# ID Shorthand Notation (ISNZ)

Throughout `docs/Generics.md`, investigation notes, and code comments, typed-AST ids are written in a compact shorthand instead of the verbose `IdT(packageCoord, Vector(...), ...)` dump. This doc is the reference for that notation.

## Tokens

| Token | Meaning |
|---|---|
| `<X>` | generic args (in a use-site) or generic params (in a declaration). `Foo<int>`, `func bar<T>`. |
| `{X}` | "generic template args" — a per-call-site tag on a lambda template that disambiguates separate compilations of the same lambda body. Per @LAGTNGZ. *Not* generic args. `__call{bool}`, `__call{int}`. |
| `$T` | placeholder named after generic parameter `T`. `Foo$T`, `dis$X`. |
| `$0` | placeholder for the 0-th positional generic parameter. `ri$0`, `case$3`. |
| `:loc` | file-location disambiguator. `drop:66` (offset), `lam:2:6` (line:col), `impl:98` (offset). |
| `.` | path separator between nested denizens. `MySome.bound:drop:66`, `genFunc<int>.lam:2:6`. |
| `bound:name` | a bound declared on a denizen. `BorkForwarder.bound:__call`, `MySome.bound:drop:66`. |
| `^X` | owned coord. `^impl:98$0`. |
| `&X` | borrowed coord. `&BorkForwarder<T>`. |
| `*X` | shared/immutable coord. `*int`. |

## Bare name vs `<>` — template vs instantiated form

A name written without any `<...>` is always a **template** id (`ITemplateNameT`-leaved). A name with `<...>` — including empty `<>` — is always an **instantiated** id (`IInstantiationNameT`-leaved). The `<>` is required even when there are no generic args, because it's the visible signal that distinguishes the two forms.

| Shorthand | Form | Type at the leaf |
|---|---|---|
| `Foo` | template | `StructTemplateNameT` (or other `ITemplateNameT`) |
| `Foo<>` | instantiated, no args | `StructNameT(template, [])` (or other `IInstantiationNameT`) |
| `Foo<^int>` | instantiated, with one coord arg | `StructNameT(template, [own-int-coord])` |
| `bar` | function template | `FunctionTemplateNameT` |
| `bar<>()` | function instantiated, no template args, no params | `FunctionNameT(template, [], [])` |
| `bar<^int>(^Foo)` | function instantiated with one template arg, taking one param | `FunctionNameT(template, [own-int-coord], [own-Foo-coord])` |

This rule applies uniformly to citizens (struct/interface/anonymous-substruct) and functions (regular, function-bound, anonymous-substruct-constructor, etc). When a name appears in a path's `initSteps` it can be either form depending on what the underlying typed-AST holds — the shorthand simply reflects whichever form is there.

## Coords vs kinds — `^`/`&`/`*` are load-bearing *(Scala-era vocabulary; see the onion mapping below)*

Every coord-typed value (parameters, return types, generic args of coord type) **must** carry exactly one ownership prefix. Bare `X` (no prefix) is a *kind* or a *placeholder of kind type*, not a coord. The ownership prefix is the visible signal that something is a coord; eliding it changes the meaning.

Examples:
- `Foo<^Bar>` — `Foo` instantiated with the coord `^Bar` (owned `Bar`). Coord generic arg.
- `Foo<Bar>` — `Foo` instantiated with the kind `Bar`. Kind generic arg. Different from above.
- `func push(^Vec<*int>, *int)` — function taking an owned `Vec<*int>` (where `*int` is a shared int coord, the Vec's type-arg) and a shared int.

The distinction reflects the Scala-era templata shapes: `CoordTemplataT(CoordT(ownership, region, kind))` is a coord (always carries an ownership prefix in shorthand); `KindTemplataT(kind)` is a kind (never carries one). The same applies to placeholders: `^anon:I$functor:moo` is the *coord* placeholder for `anon:I`'s functor:moo rune; `anon:I$functor:moo` without the prefix is the *kind* placeholder.

## Reading the ownership prefixes under the onion

The coord section above describes the model the shorthand was minted against — `CoordT(ownership, region, kind)` as a flat tag beside the kind. The onion dissolved `CoordT`: ownership is structural in `KindT`, and there is no coord templata. Old shorthand maps onto today's types as:

| Shorthand | Scala-era meaning | Onion equivalent |
|---|---|---|
| `&X` | borrowed coord | `KindT::BorrowRef(X)`, region on the wrap |
| `^X` | owned coord | bare `X` — an owned value is a kind with zero wraps |
| `*X` | shared coord | structural: a share citizen appears `ShareRef`-wrapped; a primitive is bare |
| bare `X` | a kind, never a coord | also bare `X` — the coord-vs-kind axis has no onion counterpart |

So when an old note writes `^impl:98$0`, the `^` tells you the position was coord-typed in the old model; in onion terms it is just the placeholder kind. The id half of the notation — `$`, `.`, `:loc`, `<>`, `{}`, the prefix table below — is unchanged and current.

## Precedence (tightest → loosest)

1. `<...>` / `{...}` / `$N` / `:loc` — these all bind directly to a name.
2. `.` — separates path steps.
3. `^` / `&` / `*` — ownership prefix, outermost on the whole expression.

So `^len.odis{impl:98}$0` parses as `^( len . odis{impl:98}$0 )` — an owned coord whose kind is the placeholder `$0` of the lambda-template-tagged `odis{impl:98}` reached via the path `len.…`.

## Common denizen-prefix abbreviations

| Prefix | Denizen kind |
|---|---|
| `ri` | receiving impl |
| `dis` | dispatcher |
| `case` | dispatcher case |
| `abst` | abstract function |
| `over` | override |
| `odis` | override dispatcher |
| `lam` | lambda |
| `anon:I` | the anonymous substruct synthesized for non-sealed interface `I` (auto-generated by `AnonymousInterfaceMacro`). One name token, not a path step — `I` here is a payload-style qualifier on a single name. Per the `:` qualifier convention. |

These are conventional names for placeholder origins discussed in `docs/Generics.md`'s override-dispatcher and NBIFP sections.

## Common rune-name conventions

| Rune name | Meaning |
|---|---|
| `functor:M` | An anonymous substruct's member-rune for interface method `M` (one per interface method, conceptually the callable that backs that method). E.g., for `interface I { func moo(...) int; }`, the substruct's single member-rune is humanized as `$I.anon.moo.functor` and written `$functor:moo` in shorthand. The `:moo` is the method-name qualifier on the conceptual rune kind `functor`. |

## Examples decoded

- `HashMap$T` — placeholder for HashMap's generic parameter `T`.
- `dis$0` — placeholder for the dispatcher's 0-th generic.
- `impl<ri$0, ri$1> ISpaceship<int, ri$0, ri$1> for Raza<ri$0, ri$1>` — an impl with two receiving-impl placeholders.
- `MySome.bound:drop:66<>(^impl:98$0)` — the bound `drop` (declared at file:66) on `MySome`, with empty generic args, taking an owned `impl:98`'s 0-th placeholder.
- `mvtest/genFunc<int>.lam:2:6.__call{bool}<bool>` — package `mvtest`, function `genFunc` instantiated with `int`, the lambda at line 2 col 6 inside it, the `__call` lambda compiled with template-arg-tag `{bool}` and called with generic arg `<bool>`.
- `anon:I` — the anonymous substruct of interface `I` (auto-generated by `AnonymousInterfaceMacro`).
- `anon:I$functor:moo` — kind placeholder parented by `anon:I` for the moo-functor member rune (the bare placeholder, no ownership — i.e. a `KindPlaceholderT`).
- `^anon:I$functor:moo` — the same placeholder wrapped in an owned coord (a `CoordTemplataT` of own-coord-of-kind-placeholder).
- `anon:I<^anon:I$functor:moo>` — the substruct, instantiated with its own `functor:moo` coord as its only generic arg (the typing-pass shape of the substruct seen during its own template-context compilation; the substruct's only generic param is coord-typed, hence the `^`).
- `anon:I.bound:__call<>(&anon:I$functor:moo)` — the `__call` bound declared on `anon:I`, taking a borrowed reference to the substruct's member-rune placeholder.
- `anon:I<^anon:I$functor:moo>.drop:0` — the synthesized drop function (file offset 0, since macro-generated functions get FileCoordinate offset 0) parented by the instantiated substruct.
