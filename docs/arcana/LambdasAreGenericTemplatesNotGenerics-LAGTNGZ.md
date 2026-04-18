# Lambdas Are Generic Templates Not Generics (LAGTNGZ)

Top-level functions and lambdas are specialized by different mechanisms. Top-level functions are **generic**: compile the body once against abstract placeholders, have the Instantiator stamp concrete monomorphizations later from the call graph. Lambdas are **templates**: the typing pass solves each call site fresh with concrete arg types, and bakes those arg types into the resulting function's template name — so each distinct arg-tuple produces a separately-named `LambdaCallFunctionNameT` entry in `CompilerOutputs.functions`.

`FunctionCompiler` dispatches on `FunctionA.isLight()`: light functions (no captured variables) take the generic light-function path; non-light functions (real closures with captures) take the templated-closure path. `isLight` is defined on `FunctionA` in the higher-typing AST. The two name builders in `NameTranslator` split the same way: `translateGenericTemplateFunctionName` takes the call-site `params: Vector[CoordT]` and produces `LambdaCallFunctionTemplateNameT(codeLocation, paramTypes)` where the arg coords are part of the template name; `translateGenericFunctionName` explicitly refuses lambdas with `vfail() // Lambdas are generic templates, not generics`.

**Why this split exists.** Lambdas capture variables from enclosing scope. For a pure-generic model to handle them, the captured-variable types would need to be published on the enclosing function's interface as bounds, so the body could type-check against abstract placeholders for the captures. The template model sidesteps this: wait until a call site supplies concrete arg types (and by then the closure's captured-variable types are concrete too), then expand per distinct arg tuple.

**Example: one call site.**

```vale
func takeInt<F>(f F) int where func __call(&F, int)int { f(5) }
exported func main() int {
  takeInt((x) => { x })
}
```

`(x) => { x }` is scouted with one generic parameter for `x`'s type. When `takeInt` is typed, its `__call` bound resolves to the lambda with `params = [int]`. The typing pass produces one `LambdaCallFunctionNameT(template = LambdaCallFunctionTemplateNameT(loc, [int]), parameters = [int])` in `CompilerOutputs.functions`.

**Example: one lambda, two call sites with different types.**

```vale
exported func main() {
  lam = (x) => { x };
  (lam)(5);
  (lam)("hi");
}
```

`lam` is one closure struct — exactly one `LambdaCitizenNameT` is produced (closure structs aren't parameterized; `LambdaCitizenTemplateNameT.makeStructName` asserts `templateArgs.isEmpty`). But `CompilerOutputs.functions` contains **two** `__call` entries:

```
LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(loc, [int]), ..., [int])
LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(loc, [str]), ..., [str])
```

One closure struct, two instantiations of its call function — separately type-checked, separately named, both pointing at the same closure. This is the direct consequence of arg types being encoded in the template name: different tuple, different name, different entry.

**Interaction with ECSIIOSZ.** Per-call-site solves naturally mesh with per-call-site template expansion: each solve independently reaches the lambda dispatcher with its own `argTypes`, so independent solves produce independent `LambdaCallFunctionTemplateNameT`s without sharing state.

## See also

- `docs/Generics.md` — the generics system that top-level functions use.
- `docs/arcana/EachCallSiteIsItsOwnSolve-ECSIIOSZ.md` — the per-call-site solve model.
