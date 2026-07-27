# Type Names Lower To Zero-Arg Calls (TNLTZACZ)

A type name in a templex never lowers to a bare `Lookup`. `translate_templex` emits two rules — a `Lookup` naming the type, and a `Call` applying it to no arguments — and returns the **Call's** result rune.

```
x int        →  Lookup { rune: L, name: "int" }
                Call   { result: R, template: L, args: [] }      // returns R

x Opt<int>   →  Lookup { rune: L, name: "Opt" }
                Call   { result: R, template: L, args: [int] }   // returns R
```

**Why.** It front-loads whether a name is a template or a finished kind. A name is always looked up and always applied, so nothing downstream branches on whether the user wrote a generic. The rune-type solver used to settle that itself, coercing Template to Kind behind a two-pass prepass; making the application explicit is what let that machinery go.

The `Lookup`'s rune names the *template*, and is not the type — the type is the `Call`'s result. Code reading a rule list for "the rune of this templex" wants the Call; a rune concluded by a `Lookup` is one hop short. The `Lookup`/`Call` pairs sitting in a parameter's `value_type_rules`, per PFVSZ, are these.

**Applying a template to no arguments collapses it to its return type.** For a bare name that is the whole point — `Opt` on its own is not a usable type. For a name that is about to receive arguments it is destructive, so a templex's template position takes the `Lookup` alone, via `translate_template_position_templex`. Route it through the bare-name lowering instead and the outer application is handed a finished kind where its template belongs, with its arguments left nothing to apply to.

The empty tuple `()` lowers the same way.

## See also

- [Parameter Full-Type / Value-Type Split (PFVSZ)](ParameterFullTypeValueTypeSplit-PFVSZ.md)
