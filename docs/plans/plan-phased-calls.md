# Phased Calls

Redesigning generics solving at a call site, from scratch.

## Strategic Directions (human-only)

A call site does these phases:

 * Phase 1: Candidate selection / filtering. Narrow down the exact *only* candidate to attempt the rest with.
 * Phase 2: Upcastability, for each argument try to solve the impl that casts it to the expected parameter template.
 * Phase 3: Seed initial knowns into conclusions map
 * Phase 4: Main solve, to detect problems and determine the function's runes mapping.
 * Phase 5: Resolve bounds and register instantiation bounds.
 * Phase 6: Substitute for final parameter type
 * Phase 7: Convert: insert upcast operations at the callsite.
 * Phase 8: Borrow check

Note that we're intentionally not doing a full function-wide bidirectional solve like rust.
This means that instead of writing:
```rs
let my_vec = Vec::new();
...
let my_opt = None();
```
we'll be writing:
```
v = Vec<int>();
x Opt<int> = None<int>();
```
This is an acceptable cost, to get the simpler 8-phase approach.


### Preparation §P

Before Phase 1 can work, we'll reorganize things:

 * Change the vale corpus:
    * Add files for the various types, so they have their own scopes for their functions.
       * Add int.vale. Register that it's `int`'s file.
       * Add bool.vale. Register that it's `bool`'s file.
       * Add float.vale. Register that it's `float`'s file.
       * Add i64.vale. Register that it's `i64`'s file.
       * Add borrow.vale. Register that it's borrow refs' file.
       * Add void.vale. Register that it's `void`'s file.
       * Register str.vale as `str`'s file (str.vale already exists).
       * Add none.vale for `struct None<T>` and its functions.
       * Add some.vale for `struct Some<T>` and its functions.
       * Add ok.vale for `struct Ok<T, E>` and its functions.
       * Add err.vale for `struct Err<T, E>` and its functions.
       * Add str_slice.vale for StrSlice to be defined in.
       * Add rsa.vale. Register that it's runtime-sized array's file.
       * Add ssa.vale. Register that it's static-sized array's file.
    * Move things to those files.
       * print(str) -> str.vale
       * drop.vale's things will go into the files for those types. Same with `func ==`.
       * str(int) -> int.vale
       * str(i64) -> i64.vale
       * str(float) -> float.vale
       * `struct None<T>`, `impl<T> Opt<T> for None<T>`, and None's functions -> none.vale
       * `struct Some<T>`, `impl<T> Opt<T> for Some<T>`, and Some's functions -> some.vale
       * `abstract func isEmpty<T>(virtual opt &Opt<T>) bool;` -> opt.vale
       * `func isEmpty<T>(opt &None<T>) bool { return true; }` -> none.vale
       * `func isEmpty<T>(opt &Some<T>) bool { return false; }` -> some.vale
       * Same pattern for result.vale: Ok/Err structs + impls + functions -> ok.vale/err.vale
       * `func contains(haystack str, needle str) bool` -> contains_str in str.vale
       * `func contains(haystack StrSlice, needle str) bool` -> contains_str in str_slice.vale
       * `func contains(haystack str, needle StrSlice) bool` -> contains_slice in str.vale
       * `func contains(haystack StrSlice, needle StrSlice) bool` -> contains_slice in str_slice.vale
       * `drop<T>(&T)` -> borrow.vale
       * arith.vale's functions -> distribute to int.vale, i64.vale, float.vale (all +, -, `*`, /, mod, <, >, <=, >=, negation, type conversions)
       * clone.vale's functions -> distribute to int.vale, bool.vale, float.vale, i64.vale, str.vale
       * logic.vale's functions -> bool.vale (not, ==)
    * Delete some things:
       * `func isEmpty<T>(opt Some<T>) bool`. Users will need to say `isEmpty(&my_opt)`. Might fix with auto-ref eventually.
       * `func +(s str, i int) str`
       * `func +(s str, b bool) str`
       * `func +(s str, f float) str`
       * Delete `+(int, str)`.
    * Add some things:
       * `str(bool)` in bool.vale.
       * `as` in as.vale, for (infallible) upcasts. Make `as` implicitly imported, like the other builtins.
 * Make it so every bound (ImplBoundS, ResolveSR) contains its rules and its own runes.
 * Make it so postparsing an impl puts not only rules in the ImplS, but also a `sub_citizen_type: ITypeST` and `super_interface_type: ITypeST`.
 * Make it so postparsing includes not only rules in the ParameterS, but also a `type: ITypeST`.
 * Make it so function bounds hold a `super_interface_type: ITypeST` too.
 * Make it so a generic param's hold its default value as an ITypeST too.
 * ITypeST is read-only, only the postparser constructs one. Substituting things into it produces a KindT, never a new ITypeST.
 * Enforce that an impl only lives in its struct's file or its interface's file.
 * When we declare a bound like `where exists drop(T)void`, put that PrototypeTemplata into the placeholder's outer env (this helps 5.8).

Things to do during or after the plan, depending on what failures they'd fix:

 * Enable disambiguation syntax, for example if we say `MyStruct.fromInts(4, 3)` then we'll only look inside both MyStruct's file and MyStruct's definition.
 * Long term (out of scope) we'll instead be able to say `import stdio` which will import all the print functions from a stdio.vale.
 * Add the ability to import specific functions into scope, stop ignoring ImportS.
 * Forbid the user from writing an `as` function.
 * Rename the current downcast `as` to `try_as`. `try_as` should be for just (fallible) downcasts. Make `try_as` defined as a builtin method for every interface.


### Phase 1B: Callsite explicit template args solve §1B

Do the explicit template args solve, to determine the actual templatas that the callsite wants to explicitly send in.
Note that this is a full solve, but using only things visible from the callsite, so we don't need to know anything about the callee yet.

### Phase 1F: Candidate selection / filtering. §1F

 1. Look at each callsite argument. For each:
    * Ignore any references on it (peel them away).
    * Look at its type's package env (package's top-level namespace).
    * Look at its type's outer env (the methods defined inside the struct/interface).
    * In both, find any method of the desired name, and the right arity. If you found zero, continue to next one. If one, stop here. If multiple, show an error, require them to disambiguate.
 2. (Later when we have default trait methods)
   For each impl in the argument type's file and any imported interfaces files, whose sub_citizen_type's template is the callsite argument's template:
    * Look in the super_interface_type's outer env.
    * Find any method of the desired name, and the right arity.
   If you found exactly one, stop here. If zero, continue. If multiple, show an error.
 3. If the function has the same name as a type that is visible to (imported by) the callsite:
    * Look at its type's package env (package's top-level namespace).
    * Look at its type's outer env (the methods defined inside the struct/interface).
    * In both, find any method with that name of the right arity. If you found zero, continue to step 4. If one, stop here. If multiple, show an error, require them to disambiguate.
 4. If the function was imported by the callsite:
    * Check if it has the right arity. If found zero, continue to step 5. If one, stop here. If multiple, show an error, require them to disambiguate.
 5. If zero were found, show an error.

We stop at the first one, to support this bunch of functions:
 1. str.vale: `func contains(haystack str, needle str) bool`
 2. str.vale: `func contains_slice(haystack str, needle StrSlice) bool`
 3. str_slice.vale: `func contains(haystack StrSlice, needle str) bool`
 4. str_slice.vale: `func contains_slice(haystack StrSlice, needle StrSlice) bool`
Otherwise, `contains("hi","hello")` has a conflict between #1 and #3.

### Phase 1G: Postparse the callee §1G

If the callee hasn't been postparsed, request a postparse now.
Main purpose is to generate the "steps" that we'll use later on in phase 4.
Basically, do a topo sort of the bounds (regardless of whether theyre specified in where clauses or in the generic parameters) and the function params (because e.g. `func tag<T: Named>(label T.Name, thing T)`), to produce "steps". Error on cycle (though, we could relax this, as explicitly specified args do make it callable).

This is also where we do the lookups. So instead of referring to things by their imprecise name, we do the actual resolve.
 * We note a parent_runes Set here, runes the callee doesn't declare (like a lambda's `__call` mentioning its parent's `T`). Phase 4 will use this.

This is also where, for each parameter, we look up whether the type is a class or a struct (because we desugar bare mentions of classes like `x: MyClass` into something like `x: BorrowRef(ShareRef(MyClass))``), when we produce the ITypeST for the parameter.

Steps:
 * ArgumentStep
 * ImplBoundStep
 * FuncBoundStep
 * GenericDefaultStep
    * This one is skipped if the user actually specified it via an explicit template arg.
    * "Dead defaults" cause a compiler error. Example: `func append<T = int>(v Vec<T>, x T)`'s `T` is always determined by the arguments so will never happen.

The topo sort's order is determined by what a step's inputs and outputs are.

Generally, it works like this:
 * ArgumentStep's input is the argument type, and its outputs are all the runes mentioned in the parameter.
    * Example: `x: MyThing<T, Y>` input is the argument type, outputs are T and Y.
 * ImplBoundStep's input is the runes in the sub_citizen_type.
    * Example: `implements(T, Sporkle<Y, Z>)` input is T, outputs are Y and Z.
 * FuncBoundStep's input is the runes in the arguments, output is the runes in the returns.
    * Example: `where exists foo(T,Y)R` inputs are T and Y, output is R.
 * GenericDefaultStep's input is the runes in the default, output is the result.
    * Example: `H = DefaultHasher<K, V>` inputs are K and V, output is H.

However, associated projections (`T.Item`) can make it trickier. Generally they make `T` an input, unless `T` is already an output of the same step.
 * ArgumentStep examples:
    * `x: MyThing<T, Y>` input is the argument type, outputs are T and Y.
    * `x: MyThing<T.Spork, Y>` input is the argument type and T, output is Y.
    * `x: MyThing<T, T.Spork>` input is the argument type, output is T.
    * `x: MyThing<T.Spork, T>` input is the argument type, output is T. (Same as above, but note how the order doesn't matter)
 * ImplBoundStep examples:
    * `implements(T, Sporkle<Y.Bork, Z>)` inputs are T and Y, output is Z.
    * `implements(T, Sporkle<Y, Y.Bork>)` input is T, output is Y.
 * FuncBoundStep examples:
    * `where exists foo(T,Y)R` inputs are T and Y, output is R.
    * `where exists foo(T,Y)R.Thing` inputs are T, Y, and R. No outputs.
To make this work, determine for each non-projection rune-mention (`T`, not `T.Item`) whether it's an input or output, then look at the projections.

Example §1G1
 * callee: `func foo<T, Y>(x Opt<T>) where implements(T, MyInterface<Y>) { ... }`
 * impl: `impl MyStruct for MyInterface<bool>;`
 * callsite: `foo(Some(MyStruct()))`
In the topo-sort, the `implements(T, MyInterface<Y>)` only needs to happen once `T` is known.

More examples to work through: ????
 1. implements(T, MyInterface<Y>) — waits on T. The common case.
 2. implements(Pair<T, U>, IFoo<Y>) — waits on T and U; it needs the whole left type built.
 3. implements(MyStruct, ISerializable) — waits on nothing; it can even be checked once when the declaration compiles, since no call site changes it.
 4. implements(T.Item, IComparable<Y>) — waits on the computation of T.Item, not just on a rune.
 5. implements(T, MyInterface<U.Out>) — right-side plain runes like Y never cause waiting, but a right-side computed type like U.Out does, because computed types are never run backward
 6. `func add_into<T>(x T, vec Vec<T>)`. I think here we actually want to order the `vec` argument first, to get a clearer picture of what `T` is, because top-level `T`s like `x` go through the confusing phase 4 argument coercion. We should delay e.g. top-level `T` and `&T` until after the other params.

Note for future features: we're sorting once, at postparse time, and every call reuses the same order. This only works if one order fits all callers. This is true, but only because of these facts:
 1. No matter how much a caller explicitly specifies, it doesn't change the order.
 2. No matter what arguments a caller supplies, it doesn't change the order.
 3. Each step has inputs and outputs that are set in stone, we don't rearrange rules depending on what's available.
 4. Information doesn't need to flow backwards into the arguments (like lambdas can in Rust).

### Phase 1H: Check explicit template args types against callee §1H

Make sure that the explicit template args types' match the expected types (generic_params[i].tyype).

### Phase 2A: Dyn Upcastability. §2A

Look at each callsite argument ("uncoerced-argument"). For each:
 * Peel it ("peeled-uncoerced-argument").
 * Look at the parameter's type ITypeST.
 * Look past any outer refs (peel them).
 * If the remaining is a CallST, then continue. Otherwise, skip to the next argument.
 * Get the CallST's template, that's the "expected template".
 * If the argument template and expected template are the same, then skip this argument. Otherwise continue.
 * Ignore any references on both (peel them away).
 * Find all impls for the uncoerced argument type and the parameter type. In this example:
   ```
   fn callee<T>(x &Opt<T>) { ... }
   fn caller(x Some<int>) {
     callee(&x)
   }
   ```
   we're looking for the impl that turns `Some<int>` into an `Opt`:
   ```
   impl<T> Some<T> for Opt<T>
   ```
 * If there is no impl, then give a compiler error and stop the whole callsite.
 * Once we have that impl, match the peeled-uncoerced-argument (`Some<int>`) against the impl's sub_citizen_type (`Some<T>`) to get impl_rune_to_type map (T = int).
 * Substitute the impl_rune_to_type map into the impl's super_interface_type (`Opt<T>`) to get the result ("peeled-coerced-argument") (`Opt<int>`).
 * Enforce peeled-coerced-argument contains no impl runes, otherwise compile error.
 * Remember the peeled-coerced-argument (`Opt<int>`). It's handed into the ArgumentStep's input.
 * Note that none of this should be registering any instantiation bounds yet. We must remember to do that at the end when we're sure this call will work.
 * The upcast is one hop, not transitive.

Note that there might be multiple impls for a given struct to a given interface, like (§2.1):
```
impl IObserver<SignalA> for MyController;
impl IObserver<SignalB> for MyController;
func watch<T>(o &IObserver<T>) { }
watch(&MyController())
```
In that case, we expect the user to disambiguate with a cast:
```
watch(as<IObserver<SignalA>>(&MyController()))
```

Note that lambda arguments are seen as normal lambda capture structs. They aren't special. Their signature isn't dealt with until later when we consider the lambda struct's bounds.

### Phase 3: Initial Knowns §3

Look up the explicit template args, and the env-supplied values for the parent runes from §1B, and put them into a `conclusions: IndexMap<Rune, ITemplata>` map.

### Phase 4: Execute Steps §4

Iterate through the postparsed denizen's sorted steps.
 A. If it's an argument step, then do a recursive walk matching the incoming argument KindT and the function's param's ITypeST.
 B. If it's a function bound step, then do what step 5 was doing. It can teach.
    * We might naturally be calling into a lambda's call function here.
 C. if it's an impl bound step, then do what step 2B and 5 were doing. they can teach.
Each of these is explained in more detail below.

We shouldn't register any instantiation bounds yet. We must remember to do that at the end when we're sure this call will work.
 * One exception: the FuncBoundStep might be compiling a lambda. The lambda's body will be registering some instantiation bounds.

#### Phase 4's ArgumentStep §4A

The basic idea here is:
 * First do an "outer compare" which adds or peels outer references (like the first `&` in `&Vec<&Spork>`).
 * Once everyone agrees on outer references, do a stricter recursive compare of the "value type" (like the `Vec<&Spork>` in `&Vec<&Spork>`).

Actual steps:
 1. Do the "outer compare". Look at the incoming argument KindT and the function's param's ITypeST.
    * If the param's ITypeST is a rune:
       * Look first in the conclusions map, populated by previous steps and by phase 3 initial knowns.
       * If rune not known, fill it with the corresponding part of the incoming KindT. End here.
       * If rune is known, use that known value for the param and keep going.
    * If they both have a ref, strip it off both, repeat the "outer compare" step.
    * If the incoming argument KindT has a ref but the param's ITypeST doesn't, allow it if the type implements Copy or has a `__copy_prim`, which phase 7 will call. Strip off the incoming argument's ref, proceed to the "inner compare".
    * If the incoming argument KindT has no ref, but the param's ITypeST does. Either (depending on experimental flags):
       * Allow it, and phase 7 will automatically insert a temporary local variable and give a ref of that. Strip off the param's ITypeST's ref, proceed to the "inner compare".
       * Reject it, compile error.
    * If you come across anything else, proceed to the "inner compare".
       * Also, assert here that the arg template and ITypeST template match, because phase 2A should have upcast things until they match. **This step does not think about upcasting.**
 2. Do the "inner compare". Do a recursive walk matching the incoming argument KindT and the function's param's ITypeST.
    * When you come across a rune:
       * Look first in the conclusions map, populated by previous steps and by phase 3 initial knowns.
       * If rune not known, fill it with the corresponding part of the incoming KindT.
       * If rune is known, compare against that known value instead of the ITypeST.
    * When you come across anything else: keep recursing, comparing the two. Give an error if they don't match.
If reach the end with no conflicts, success.
(Note, when we substitute into a rune and then compare the substitution, that's comparing two KindTs, not comparing with ITypeST anymore. Different logic.)

Example §4A1UV:
```
func store<T>(x T) { ... }
store(make_ship_val()); // Hands in a `Ship`
```
Outer compare sees arg `Ship` and param's rune `T`, rune not known, so concludes T = `Ship`.

Example §4A1UR:
```
func store<T>(x T) { ... }
store(make_ship_ref()); // Hands in a `&Ship`
```
Outer compare sees arg `&Ship` and param's rune `T`, rune not known, so concludes T = `&Ship`.

Example §4A1RVD:
```
func store<T>(x T) { ... }
store<&Ship>(make_ship_val()); // Hands in a `Ship`
```
Assuming the experimental flag for auto-ref (making temporary locals) is disabled:
 * Phase 3 concludes T = `&Ship`
 * Outer compare sees arg `Ship` and param `&Ship`, stops with a compile error (which suggests adding an `&`).

Example §4A1RVE:
```
func store<T>(x T) { ... }
store<&Ship>(make_ship_val()); // Hands in a `Ship`
```
Assuming the experimental flag for auto-ref (auto making temporary locals) is enabled:
 * Phase 3 concludes T = `&Ship`
 * Outer compare sees arg `Ship` and param `&Ship`, strips off of param, proceeds to inner compare.
 * Inner compare sees Ship = Ship, success.
If the user didn't want to explicitly specify, they could have called it like `store(&make_ship_val())`.
Note this experimental flag is different than the "mention = ref" flag (where `foo(x)` lowers to `foo(&x)`).

Example §4A1VR:
```
func store<T>(x T) { ... }
store<Ship>(make_ship_ref()); // Hands in a `&Ship`
```
 * Phase 3 concludes T = `&Ship`
 * Outer compare sees arg `&Ship` and param `Ship`, enforces arg Copy/`__copy_prim`, proceeds to inner compare.
 * Inner compare sees Ship = Ship, success.


#### Phase 4's FuncBoundStep §4F

Something similar to 4I.
(fill this out a little more perhaps).

We'll be resolving the function's header, to see what its return type is, so it can inform the rest of the steps.
This usually just means resolving a function's header, but could also mean resolving an entire lambda's body.
 * Note that un-verified types might make it into a lambda's body. I think it's safe, but might result in weird error messages.



Check that that all of the callee's required bounds are met.

To do this, it should:
 1. Do all substitutions into the bound.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Note that we *only* look in the rune substitution value's environment, no other environments (*not* the caller's environment anymore).
       * This means that if a bound asks for a `foo(T)void`, and T = int, it can only be satisfied by functions in int.vale.
    * Note that we *aren't* peeling references away from the rune substitution value, before looking in that type's environment.
 3. Error if not exactly one was found.

value-drop example (§5.5):
```
-------- main.vale --------
func foo(x Some<Ship>) {
  drop(x^)
}
-------- some.vale --------
func drop<D>(opt Some<D>) where exists drop(D)void { ... }
-------- ship.vale --------
struct Ship { }
func drop(self Ship) { ... }
-------- borrow.vale --------
func drop<D>(r &D) { ... }   // <-- We don't want to call this
```
To properly compile `drop(x^)`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where exists drop(D)void` and D=Ship into `where exists drop(Ship)void`
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did D=Ship, the rune substitution value is `Ship`, so we look in `Ship`'s environment (ship.vale) for something named `drop`, and find `func drop(self Ship)`, stop.
Note that we *only* looked in `Ship`'s environment ship.vale, we didn't even *consider* the `func drop<D>(r &D) { ... }` in borrow.vale.

value-clone example (§5.6):
```
-------- main.vale --------
func bar(z Some<Ship>) {
  z2 = z.clone();
}
-------- some.vale --------
func clone<T>(opt Some<T>) where exists clone(&T)T { ... }
-------- ship.vale --------
struct Ship { }
func clone(self &Ship) Ship { ... }
-------- borrow.vale --------
func clone<T>(r &&T) &T { ... }   // <-- We don't want to call this.
```
To properly compile `z.clone()`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where exists clone(&T)T` and T=Ship into `where exists clone(&Ship)Ship`.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did T=Ship, the rune substitution value is `Ship`, so we look in `Ship`'s environment (ship.vale) for something named `clone`, and find `func clone(self &Ship)Ship`, stop.
Note that we *only* looked in `Ship`'s environment ship.vale, we didn't even *consider* the `func clone<T>(r &&T) &T` in borrow.vale.

ref-clone example (§5.7):
```
-------- main.vale --------
func bar(z Some<&Ship>) {
  c = z.clone();
}
-------- some.vale --------
func clone<T>(opt Some<T>) where exists clone(&T)T { ... }
-------- ship.vale --------
struct Ship { }
func clone(self &Ship) Ship { ... }   // <-- We don't want to call this.
-------- borrow.vale --------
func clone<T>(r &&T) &T { ... }
```
To properly compile `z.clone()`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where exists clone(&T)T` and T=&Ship into `where exists clone(&&Ship)&Ship`.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did T=&Ship, the rune substitution value is `&Ship`, so we look in the **borrow ref** environment (borrow.vale) for something named `clone`, and find `func clone<T>(r &&T) &T`, stop.
Note that we *aren't* peeling references away from the rune substitution value (&Ship). We take it straight, and look in that type's environment. That's why we looked in borrow.vale for a `clone`, instead of finding the `func clone(self &Ship) Ship` in ship.vale.

placeholder example (§5.8):
```
func bar<Y>(x Y) where exists drop(Y)void { ... }
func foo<T>(x T) where exists drop(T)void {
  bar(x)
}
```
To properly compile `bar(x)`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where exists drop(T)void` and T=`foo$T` into `where exists drop(foo$T)void`.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did T=`foo$T`, the rune substitution value is `foo$T`, so we look in `foo$T`'s environment for something named `drop`, and find `foo`'s `where exists drop(T)void`, stop.
Note that this requires that bounds, like `foo`'s `func drop(T)void` need to be declared inside the `foo$T` placeholder's environment.

#### Phase 4's ImplBoundStep §4I

We'll process an ImplBoundStep, which points at a bound like `implements(T, IObserver<X, U>)`.
The `T` is guaranteed already known by the time we get to this point.

 * That ImplBoundS's super_interface_type is the "expected template".
 * Find any impl in the argument type's file and any imported interfaces files, whose sub_citizen_type and super_interface_type match what we're looking for. In this example:
   ```
   fn callee<X, T, U>(x &T) where implements(T, IObserver<X, U>) { ... }
   fn caller<A>(x MyController<A>) {
     callee(&x)
   }
   ```
   we're looking for the impl that turns `MyController` into an `IObserver`:
   ```
   impl<Z> MyController<Z> for IObserver<Z, int>;
   ```
 * If there is no impl, then give a compiler error and stop the whole callsite.
 * For each one of those impls:
    1. Look up the postparsed impl's `sub_citizen_type` (`MyController<Z>`) ITypeST and `super_interface_type` (`IObserver<Z, int>`) ITypeST.
    2. Recursively compare the impl's sub_citizen_type (`MyController<Z>`) with the uncoerced argument type (`MyController<A>`). Note what runes in the former (`Z`) is matched with what in the latter (`A`). Build a map of impl_rune_to_argument_type (`Z` -> `A`).
    3. Using that map (`Z` -> `A`), substitute into the impl's super_interface_type (`IObserver<Z, int>`) and note the result (`IObserver<A, int>`) which is phrased in terms of the caller.
     * Assert that the result doesn't contain any impl runes (§2.5.1)
    4. Match callee bound interface (`IObserver<X, U>`) against step 3's result (`IObserver<A, int>`). Note what runes in the former are matched with what in the latter (`X` with `A`, `U` with `int`). Build a map of callee_rune_to_argument_type (`X` -> `A`, `U` -> `int`).
    5. Check for any conflicts with any explicit arguments. If any conflicts, dont issue a compile error, just reject this impl and hope that another one succeeds.
    6. Remember the resulting parent (`IObserver<A, int>`), this is how Phase 4 knows about the impl's trait's generic args.
 * Only one of those impls should have succeeded. Use its resulting parent.
 * Note that the solve should not be registering any instantiation bounds yet. We must remember to do that at the end when we're sure this call will work.
 * The upcast is one hop, not transitive.

double-IObserver example (§2.6):
This shows when we need explicit template args.
```vale
impl IObserver<SignalA> for MyController;
impl IObserver<SignalB> for MyController;   // <-- We don't want this one
func f<T, U>(x T) where implements(T, IObserver<U>) { }
f<U = SignalA>(MyController())   // <-- User must specify SignalA here
```
The user must specify that U = SignalA, for the compiler to know to choose the `impl IObserver<SignalA>`.
For impl 1 (`impl IObserver<SignalA> for MyController`):
  1. Look up impl's sub_citizen_type (`MyController`), super_interface_type (`IObserver<SignalA>`).
  2. Recursively compare `MyController` with `MyController`, resulting in empty map.
  3. Substitute nothing, get `IObserver<SignalA>`.
  4. Match callee bound interface (`IObserver<U>`) against (`IObserver<SignalA>`), get map `U` -> `SignalA`.
  5. Check that map (`U` -> `SignalA`) against explicit args (`U` -> `SignalA`). Agrees.
  6. Remember `IObserver<SignalA>`.
For impl 2 (`impl IObserver<SignalB> for MyController`):
  1. Look up impl's sub_citizen_type (`MyController`), super_interface_type (`IObserver<SignalB>`).
  2. Recursively compare `MyController` with `MyController`, resulting in empty map.
  3. Substitute nothing, get `IObserver<SignalB>`.
  4. Match callee bound interface (`IObserver<U>`) against (`IObserver<SignalB>`), get map `U` -> `SignalB`.
  5. Check that map (`U` -> `SignalB`) against explicit args (`U` -> `SignalA`). DOESN'T AGREE. Skip it.
  6. Remember nothing.
Only impl 1 succeeded, so proceed with it.
This is why we sometimes need the user to specify explicit args.

nesting-handles example (§2.7):
```vale
impl IHandler<IEvent<ClickEvent>> for Button;
impl IHandler<IEvent<HoverEvent>> for Button;
func handle<T, U>(x T) where implements(T, IHandler<IEvent<U>>) { ... }
handle<Button, ClickEvent>(Button())
```
This is why each bound (ImplBoundS, ResolveSR) carries the entire type (we used to have it reach into a central pool of rules, that was a mistake).
 * The first impl `impl IHandler<IEvent<ClickEvent>> for Button` contains its `IEvent<ClickEvent>` type.
 * The second impl `impl IHandler<IEvent<HoverEvent>> for Button` contains its `IEvent<HoverEvent>` type.
And we can pull those in for the argument impl matching without pulling in any rules, like the old approach.

impl-trait-generic-arg example (§2.8):
```
impl IObserver<SignalA> for MyController;
func f<T, U>(x T) where implements(T, IObserver<U>) { ... }
f(MyController())
```
In this example, the impl solve is the only way to determine what U should be. Here, we must remember it for Phase 4.

interface-independent-runes example (§2.9):
There could theoretically be some impls that have "interface independent runes":
 * `impl<T> MyOption<T> for MyNone` (theoretical, we could do this if we wanted `None` to not be `None<T>`)
    * Generalized: any sort of "empty variant" or sentinel value (`None`) of various kinds of generic interfaces (`Option<T>`).
 * `impl<T: ?Sized> RangeBounds<T> for RangeFull` (in Rust stdlib)
 * `unsafe impl<T> SliceIndex<[T]> for usize` (in Rust stdlib)
We don't support these. If we wanted to, then we'd need to check for any unresolved impl runes at §2.5.1 and let them get to the next step, which would map callee runes to those, resolved by explicit args (or some solving).


### Phase 5: Fine-Print Verifying/Resolving §5

During phase 4 and other phases, we conjured a lot of types.

For example, MyStruct's `T`s must implement `Copy`:
```
struct MyStruct<T> where implements(T, Copy) { val T; }
```
We have a feature where `wrap` doesn't need to declare those bounds:
```
func wrap<X>(x X) MyStruct<X> { ... }
```
We want `wrap(42)` to succeed.
We want `wrap(Ship())` to fail.

Pretend that we had some sort of feature that repeated arguments' bounds, transforming `wrap` into this:
```
func wrap<X>(x X) MyStruct<X> where implements(X, Copy) { ... }
```
Those would have been processed in a step in phase 4.
Alas, that didn't happen.
So, we make up for that now.

Basically, execute the ImplBoundStep/FuncBoundStep code here, for bounds pulled in via arguments.


### Phase 6: Substitute for final parameter type §6

 * Walk the ParameterS's type, using it and phase 4's conclusions, to construct the full final parameter type.

### Phase 7: Convert §7

 * Insert callsite instructions to upcast the argument to the expected parameter type.
 * If the language ever does auto-ref, that would happen here. For now, we'll require & at callsites.
 * If the user handed in a raw literal `5`, but the callee was able to determine the expected type, this is where we'd convert the 5 into that expected type. (If the callee couldn't figure it out, then it's an error)


### Phase 8: Borrow check §8

 * (Not designed yet)


### Post-cleanup

 * Make it so Phase 4 produces rules from the ITypeST, instead of reading the ones from the postparser.
 * Make it so the other phases use the ITypeST, not the rules from the postparser.
 * Make it so the postparser doesn't produce rules, it just produces the ITypeST.

### Future Notions

One day we could have a:
```
func unzip<T, A, B>(iter T) (Vec<A>, Vec<B>)
where implements(T, Iterator), Pair<A, B> = T.Item
```
Specifically the new `Pair<A, B> = T.Item` step, which takes a source (`T.Item`), and destructures it into the runes on the left (A and B).


## Strategic Directions Proposals

§29: **Uniform check-at-use: one `use()` operation; §5 becomes the registrar.**

 * **One model, no special cases.** Every denizen — the callee, a bound-found function, a selected
   impl, a constructed type — is the same shape: generic params, clauses, outputs. Each gets the
   same §1G prep (clauses and trees sorted into steps, cached once). There is one operation,
   `use(denizen, knowns)`: run its steps; they teach its runes and check its clauses as they run;
   out come its outputs plus collected obligations. The call-site pipeline is just
   `use(callee, args + explicits)`, and steps recurse: a FuncBoundStep is `use(found function)`,
   an ImplBoundStep is `use(impl)`, building `MyStruct<Ship>` is `use(MyStruct)` — whose `Copy`
   bound checks right there (the wrap example).
 * **Everything checks at its use.** No §4-checks-these / §5-checks-those split, and no
   productive-vs-restrictive clause classification to maintain (a drift hazard that every future
   teaching feature would have eroded anyway). Errors attach to the exact use that caused them.
   Which programs compile is unchanged: §1F is final, so a failure is terminal wherever it fires.
 * **§5 shrinks to the registrar.** Obligations collect during the run and register once at the
   end, keyed by the callee instantiation. A rejected candidate trial's collected obligations are
   dropped, so reject-leaves-no-trace holds by construction instead of by rule. (Compiling real
   declarations mid-trial is not trace — those are program facts, idempotent either way.)
 * **The cycle-breaking invariant (STCMBDP, miniaturized).** The old "check calls later" deferral
   broke the declare→check→resolve→declare cycle; its load-bearing core was never "batch checks
   into §5" but "expose outputs before discharging own clauses." `use()` keeps it internally:
   output steps run first, the under-construction prototype is memoized, then clause checks run.
   A re-entry (mutual bounds; a recursive type's drop needing itself — `List<T>`'s drop needs
   `drop(Opt<List<T>>)` needs itself) answers with the in-progress prototype; without that,
   recursive types can't compile. A re-entry not satisfied that way falls to a named depth/cycle
   error — today's recursion is unguarded (see Background), so this makes an existing hazard's
   guard explicit rather than adding a new cost.
 * **The lambda residue is a denizen property, not an architecture fork.** Most denizens' outputs
   are declared trees (substitute to produce); a lambda's `__call` output is computed (compile the
   body). Same rule — `use()` runs whatever output steps exist — different price. §4's
   lambdas-only mid-run registration note stands.
 * **On ratification this touches:** the §5 section rewrites as registrar-plus-invariant; the
   FuncBoundStep and ImplBoundStep texts become two instances of `use()` (a FuncBoundStep is the
   ImplBoundStep shape applied to a signature — match the parameter trees, substitute the declared
   return tree; works because named functions declare returns); §2.9's ban becomes a language
   choice rather than an architectural necessity (a productive impl clause would just be one of
   the impl's own steps; if ever supported, clause failure stays an error, never try-next-impl);
   and the Plan Details entries "No impl walk exists at match time" and "§5 collects…" get
   re-derived from this.

**CORRECTION — §29's "STCMBDP, miniaturized" misreads the original.** *(Added after an
archaeology pass over the 2022 primary sources. This supersedes the history in §29's
cycle-breaking bullet; the §29 mechanism may still be sound — this corrects only what it
attributes to STCMBDP.)*

§29 claims STCMBDP's *"load-bearing core was never 'batch checks into §5' but 'expose outputs
before discharging own clauses,'"* and motivates it with recursive types (`List<T>`'s drop
needing itself). The 2022 record does not support that reading:

 * **STCMBDP was about plain declaration-ordering within one denizen, not recursion.** Its
   motivating cycle (`docs/Generics.md`, the STCMBDP section — byte-for-byte unchanged since
   commit `f184f4d85`, 2022-09-28) is: to declare a denizen we need its param/return types; to
   know those we must check their `where` requirements; to check those the denizen's own bounds
   must already be declared. One solve, two steps each wanting to go first. No self-reference and
   no recursive type appears anywhere in it.
 * **Its load-bearing core WAS "check calls later" — a deferral, the very thing §29 denies.** The
   doc's conclusion is literally *"we do all the checking of calls later."* The 2022 mechanism is a
   deferred queue — `DeferredEvaluatingFunction` / `DeferredEvaluatingFunctionBody`
   (`CompilerOutputs.scala`), drained after declaration in `Compiler.scala`. The clearest
   contemporaneous statement is `ImplCompiler.scala`'s *"Don't verify conclusions… we can't pull
   in any declared function bounds that come from them. We'll check them later."*
 * **Recursive-type self-reference appears in no 2022 source** — not the doc, not the
   `// see STCMBDP` comments, not the tests. The motivating 2022 tests are plain
   declaration-ordering (`templatedoption.vale`'s `struct MySome<T> where func drop(T)void`; the
   `CompilerSolverTests` "concept function" cases). `List<T>`'s-drop-needs-itself is a later graft.

**What this means for the plan.** The original STCMBDP concern — header bound-vs-parameter ordering
— is genuinely dissolved by §1G's topo sort plus §P's bound-in-placeholder-env (`:89`, `:347`). The
predict/verify split that survives (§4 matches, §5 verifies) is justified by rejection-safety
(`:256`) and cycle-detection (`:136`), not by recursion. If §29's in-progress-prototype memoization
is worth keeping, keep it on its own merits as a recursion guard — but it is a **new** invariant,
not STCMBDP's "load-bearing core."

§30: **Alternative to §29 — recursion needs no mechanism; just resolve-compile normally.**
*(An alternative to §29, not an addition. §29 adds an in-progress-prototype memo to break a recursion
cycle; §30's point is that the cycle isn't real for recursive types, so neither the memo nor the
predict/verify split is needed. There is no "produce a handle vs. check its bounds" rule to maintain —
just resolve-compile.)*

Vocabulary (now in `docs/background/glossary.md`): a **define-compile** compiles a denizen's own
definition in its `foo$` placeholders; a **resolve-compile** is a call site inside a define-compile,
locally solving the *callee's* rules to get the callee's prototype.

 * **Resolve-compile is normal, and never looks at the callee's members.** For `baz` to drop a
   `List`, `baz` resolve-compiles `List`'s drop — its prototype plus its bounds — and stops. It never
   looks at `List`'s members, because the `Opt<List>` inside is `drop(List)`'s own *body* business,
   not `baz`'s. An explicit bound is the same normal process: define-compiling
   `qfoo(y Vec<MyStruct>)` calling `qcopy<T>(v Vec<&T>) where copy(&T)T` resolve-compiles
   `copy(&MyStruct)MyStruct` in full, exactly like any call.
 * **A drop bounds only its abstract members; concrete members are body work.** `drop<T>(List<T>)`'s
   only bound is `drop(T)void`, for the placeholder member `head T`. A concrete member like
   `tail Opt<List<T>>` is dropped in the *body* — a resolve-compile of `drop(Opt<List<T>>)` — not
   carried as a bound. So a recursive type produces no self-referential *bound*: recursion lives in
   concrete member *types* (bodies), not in abstract-member *bounds*, and resolve-time bounds bottom
   out at placeholder bounds.
 * **So there is no recursion cycle to break.** The `drop(List)` ↔ `drop(Opt<List>)` "cycle" only
   exists if `drop(List)` *requires* `drop(Opt<List>)` as a bound. It doesn't — `drop(List)`'s bounds
   bottom out immediately, and the `Opt<List>` drop is a body call, each body define-compiled once.
   Nothing to predict-then-verify, nothing to memo.
 * **Recursion rides on ordinary machinery only.** All declarations are up front in the top-level map
   (`compiler.rs:749`), so a recursive reference is a plain lookup. Each denizen is define-compiled
   once; bodies drain from a uniform worklist (`compiler.rs:1246`, enqueue
   `function_compiler_core.rs:119`) and a body resolve-compiles callees to prototypes, never into
   their bodies — so mutual recursion is free. The monomorphizer (`instantiating/instantiator.rs`, a
   separate pass) walks the concrete instantiation graph once with a visited-set, declaring an id
   before its members (`instantiator.rs:1115`). All of this is declare-before-define and
   do-each-once — what every program needs, recursive or not. None of it is recursion-specific.
 * **The only genuine cycle is hand-written circular requirements.** `func f<T>(x T) where g(T)void`
   beside `func g<T>(x T) where f(T)void` loops at resolve time. That is a *program error* — a named
   "circular requirement" / depth error — not something to satisfy silently, and the same bucket as
   polymorphic recursion (a type instantiating itself at ever-larger args). Neither is guarded today
   (`resolve_impl → is_parent → resolve_impl` is unguarded, `:1111`; no test exercises either). A
   termination error is worth adding for safety — but it guards against bad programs, it does not make
   ordinary recursion work. Ordinary recursion already works.
 * **What this means for §5.** The checking §5 does folds into the phase-4 steps: each step fully
   resolve-compiles the satisfiers of its bound, in place — the *normal* resolve-compile, which in
   turn checks the satisfier's own bounds, and which terminates because bounds bottom out (per the
   bullets above). Building `MyStruct<X>` checks its `Copy` bound right there — `wrap(42)` passes,
   `wrap(Ship())` fails at that step — and a `where copy(&T)T` bound fully resolve-compiles
   `copy(&MyStruct)MyStruct` like any call. Since nothing is postponed, §5's SFWPRL-discharge job
   disappears. What remains of §5 is the **registrar**: collect the satisfier prototypes found across
   the steps and write the instantiation-bounds record once, keyed by the callee instantiation —
   batched only because the write is one-shot (`add_instantiation_bounds` is write-once, MFBFDP-
   merged), not because anything is deferred. So §5 stops being a verify-then-resolve phase; it
   becomes a one-shot write of answers the steps already produced.
 * **Where the return type is built — filling §5's one gap.** The §5 bullet says `MyStruct<X>` "is
   built right there"; this names where. There is no separate "reachable bounds" mechanism — every
   type the callee's signature mentions, parameters *and* return, is resolve-compiled at the call
   site, and resolve-compiling a type checks that type's own declared bounds at the concluded runes.
   §6 already builds the final parameter types this way (walk the `ITypeST`, substitute §4's
   conclusions); the return type is built the same way, as part of producing the callee's prototype.
   So `wrap`'s return `MyStruct<X>` is built at the concluded `X` — `MyStruct<int>` checks
   `int: Copy` (passes), `MyStruct<Ship>` checks `Ship: Copy` (fails) — with no §5 reachable-bounds
   pass. It cannot happen at `wrap`'s own define-compile, where the return is `MyStruct<wrap$X>` and
   `wrap$X` carries no `Copy` bound; that is exactly why the check is per-call-site and why `wrap`
   need not declare the bound.
 * **Relation to §29 and the CORRECTION.** §29's memo, and the CORRECTION's tentative "keep it as a
   guard," are unnecessary: there is no ordinary-recursion cycle to guard. What §29 rightly wants —
   check-at-use, no §4/§5 split — stands on its own and doesn't need the memo to justify it.

§31: **Handoff sync note (ratified here, stale there).** The handoff's "a generic argument that
  needs an upcast must be written explicitly" ruling and its "no impl walking anywhere in phases
  0–2" consequence are superseded by §2A/§4I: explicit args (or a §2.1 cast) are needed only when
  several impls match. Narrow both passages at the next handoff sync, then delete this note.


## Plan Details

**§6 is a walk; completeness and conflicts are step-side checks.** *(derived from §4 and §6)*

 * §6 never touches a solver. It walks the ParameterS's `type: ITypeST`, replaces each rune with
   §4's conclusion, and builds the final parameter `KindT` outside-in. No
   `commit_step`/`incrementally_solve` resumption exists anywhere in the pipeline.
 * Conflicts are inline: every rune is write-once, and a later writer must agree or it is an error
   naming the rune and both values.
 * Completeness runs after the steps: every rune must be valued, and the error names the unvalued
   runes and says to write them (§1G's caller-must-supply classification gives the list up front).
 * `full_type_rune` never receives a conclusion. Anything downstream that wants the parameter's
   full type takes §6's walk result.

**ArgumentStep semantics.** *(derived from §4)*

 * Match the coerced argument's `KindT` against the parameter's `ITypeST`: holes bind, known runes
   check, disagreement is a conflict error.
 * Within a step, plain-rune positions bind before projection positions evaluate. That intra-step
   rule is what makes `x MyThing<T, T.Spork>` self-contained in §1G's table.
 * Projection positions only check; they never run backward (non-injective).
 * An untyped-literal argument has no productive step; §7 coerces it against the concluded
   parameter type, per §7's literal bullet.

**FuncBoundStep semantics.** *(derived from §4)*

 * Keys: every rune in the bound's parameter positions, plus any projections anywhere in it.
   Teaches: return-position holes only, from the resolved function's return. For a closure that
   means expanding the templated `__call`, which compiles the lambda body here (the
   "Registration during §4" entry below covers why that's safe).
 * A rune reachable only through a param-position hole is caller-must-supply; there is nothing to
   search by.
 * Name-uniqueness per env keeps the candidate set near one. Zero found: error naming the bound
   and the substituted signature. Several: ambiguity error.

**What §1G computes and caches, per declaration.** *(derived from §1G)*

 * The steps and their topo order. The sort is purely structural (sub/super, param/return, and
   projection-subject positions determine inputs and outputs), so it needs no name resolution.
 * Name resolution of the declaration's trees, in the declaration's own env.
 * Rune-sort well-formedness: each rune's declared sort (bare `<T>` is Kind, `<N Int>` Integer)
   checked against its positions' demands, e.g. `N` against StaticArray's first parameter. This
   replaces rune-typing; the rune-type solver retires with no successor.
 * Classifications: env-supplied runes (§1G's parent_runes set), caller-must-supply runes (produced by no step), and
   zero-input concrete bounds like `implements(MyStruct, ISerializable)`, which can be checked here
   once rather than per call site.
 * Cycle detection, per §1G's error-on-cycle rule.

**The two-file impl search.** *(derived from §P's orphan rule)*

 * Every impl lookup scans exactly two flat stores: the sub's file and the interface's file. Both
   are nameable at the point of use (the argument supplies the sub; the bound or parameter tree
   names the interface). The orphan rule is what makes the multiplicity check complete without any
   global index.
 * Coherence (decision 16's duplicate-impl detection) becomes a two-file scan at impl-compile time.
 * §1F's default-method reach (step 2) walks only impls written in the type's own file,
   transitively through files so reached. Impls living in the interface's file participate when
   the interface is in view (named in the signature, or imported).

**Primitive and BorrowRef env registration: use synthetic template IDs.** *(derived from Preparation)*

 * `declare_type_outer_env` requires a template `IdT`. Primitives (int, bool, float, i64, str, void,
   __never) have none — they're registered as `INameT::Primitive` with no `IdT`. BorrowRef is
   parameterized (`BorrowRefT` wraps an inner kind) so there's no single template.
 * Fix: create synthetic template IDs for each primitive kind and for BorrowRef-the-concept, call
   `declare_type_outer_env` for each, and add arms to `get_param_environments`
   (`overload_resolver.rs`).

**Constructor macro must populate `value_type_rules` like the postparser does.** *(derived from §2)*

 * Today `struct_constructor_macro.rs:88-96` sets `value_type_rules = []` and puts the type-building
   rules in `header_rules`. §2 searches `value_type_rules` for the CallSR and finds nothing. The fix
   is in the macro: it should populate `value_type_rules` and `type_outer_ref_rules` the same way
   `translate_signature_templex` does for user-written functions.
 * The macro must also construct each parameter's `type: ITypeST`, now that ParameterS carries one
   (§P). Every ParameterS producer owes that field, macros included. (The rules half is
   transitional and retires with Post-cleanup; the `ITypeST` is the load-bearing part.)

**Enforce ITypeST's read-only rule with a construction seal.** *(derived from §P)*

 * Private constructors plus a `_sealed` field, the interner's existing `@SICZ` shape: only the
   postparsing module can build one, so "substitution produces `KindT`, never a new tree" holds by
   compile error rather than by review. A `KindT` can't hold unresolved runes, so half-substituted
   types are impossible by construction; code that wants one is doing something the design forbids.

**Engine disciplines.** *(standing build rules; each receipt is a documented 2021-era failure from
the git-history dig)*

 * **One shared match/substitute library.** Two functions — `match(KindT, ITypeST) → rune map` and
   `substitute(ITypeST, rune map) → KindT` — in one module, called by §2A, §4's steps, §5's
   re-checks, and §6's walk. Any new match-shaped loop over an `ITypeST` elsewhere is a defect.
   Receipt: three near-identical Evaluator/Matcher pairs (~6,000 lines, identical copy-pasted
   headers) drifted for years before `034bec27f` deleted them all.
 * **Keep `ITypeST` minimal; a new node kind is a design decision.** Each kind costs a match arm, a
   substitute arm, and a §1G input/output row. Receipt: the old tree rules died partly of
   vocabulary bloat, forcing mirror-image thousand-line traversals (the 2021 flattening commits).
 * **Every failure names the rune and the fix.** Completeness: the unvalued runes plus "write the
   type argument." Conflict: the rune, both values, both sources. Zero impls: the sub, the super,
   the two files searched. Receipt: the old matcher's flagship error was the string
   "Not deeply satisfied!" with no rune named.

**Template extraction reads the ITypeST, not a rule chain.** *(derived from §2A, §4I, and §P)*

 * For a parameter like `x Opt<T>`, the template name sits directly on `ParameterS.type`
   (`CallST`'s template name). No traversal.
 * For an ImplBoundS, it sits on the bound's `super_interface_type` the same way. §P gives both,
   so no rule-chain traversal exists anywhere in §2A or §4I.

**One type per file is load-bearing for synthesized functions.** *(derived from the `as`/`try_as`/`drop` migration)*

 * Two citizens in one file give two synthesized `as` (or `drop`) of the same arity in one env,
   which §1F cannot separate. `result.vale`'s `Ok` and `Err` want splitting for the same reason
   `opt.vale`'s `Some` and `None` do.

**§1F needs a flat-only env lookup.** *(derived from §1F)*

 * Today's `lookup_all_with_imprecise_name` walks `parent_env` up to the `PackageEnvironmentT`,
   which unions all global namespaces. So searching Ship's env finds every function in every package.
   §1F needs a lookup that reads only the citizen's own `TemplatasStore` without walking parents.

**The sends machinery retires.** *(derived from §4)*

 * ArgumentStep matching subsumes `assemble_initial_sends_from_args` and the
   InitialKnown + Equals + rune_to_type triple. Do not extend them; they become archaeology when
   §4 lands. Rejection arms of the send era (`KindIsNotBorrowRef` and kin) become ordinary match
   failures.

**No impl walk exists at match time; the one real resolve is §5's.** *(derived from §2A, §4I, §5)*

 * §2A and §4I match stored `ITypeST`s: no `partial_resolve_impl`, no environments, no
   `CompilerOutputs`. `is_parent` / `get_impl_parent_given_sub_citizen` are not called for
   matching.
 * The winning impl is *resolved* exactly once, in §5: its own where-clauses discharged (§29) and
   its instantiation bounds registered. Rejected candidates from a step's loop are never resolved
   at all (§28).

**FuncBoundStep prerequisite: reference wrappers need env registration.** *(derived from §4 and §5)*

 * `BorrowRef` has no template ID and no `declare_type_outer_env` call. `get_outer_env_for_type`
   panics on it. The non-peeling bound search (§4's FuncBoundStep and §5's re-checks) needs to
   look in `borrow.vale`'s env for `drop<T>(&T)`, which requires hardcoding BorrowRef's env the
   same way primitives and arrays need theirs. The "Move `drop<T>(&T)` to `borrow.vale`" migration
   item is a prerequisite, not aspirational.

**Registration during §4: lambdas only, and why it's safe.** *(derived from §4's lambda exception)*

 * Ordinary found functions register nothing at step time: the FuncBoundStep matches the found
   function's parameter trees and substitutes into its declared return tree — pure tree work, the
   ImplBoundStep shape applied to a signature. Their real resolution (own where-clauses, own
   registration) happens in §5, uniformly with impls and built types. (Today's machinery resolves
   eagerly — `attempt_candidate_banner` registers on every success path — so this is a behavior
   change to build, not preserve.)
 * The lambda is the one true exception: no declared return tree, so its return exists only by
   compiling its body at the step — the only eager instantiation and the only mid-§4 registration.
 * Why that's safe: §1F is final, so a later §5 failure is terminal rather than a backtrack;
   nothing registered mid-§4 can appear in a successful build. Worst case is error ordering.
 * The absolute part: within a step's candidate loop, a rejected trial must leave no trace.
   Trials are pure matches, so this costs nothing.

**§5 collects through its checks and registers once at the end.** *(derived from §5; obligation sets per §29)*

 * The registration cannot be incremental, so every answer has to be in hand before the first write.
 * `check_resolving_conclusions_and_resolve` already has this shape and can be followed rather than
   redesigned.

**What retires.** *(derived from §4, §6, and §1G)*

 * The sends machinery (see its entry above). The rune-type solver (§1G's declared-sorts check
   replaces it; no successor). `commit_step`/`incrementally_solve` as pipeline machinery; @DRSINI
   defaults move to §1G's GenericDefaultStep. `complex_solve` stays dead. Do not resurrect any of these
   from the convo record; each has a named replacement here.

**§7's upcast-through-wrap is already handled by `replace_value_type_in_ref`.** *(derived from §7)*

 * §2 computes upcasts on peeled value types (Some→Opt). §6 adds the reference wrap (&Opt). §7
   emits the instruction via `UpcastTE::new`, which calls `replace_value_type_in_ref` to walk through
   the wraps and swap the innermost citizen. `&Some<int>` → `&Opt<int>` in one call. No additional
   composition logic needed.

**How associated types work without solving.** *(derived from §4I, §26, §27)*

Associated types are coming soon, and the architecture accounts for them without a full solve: a
projection is one more step input/output kind in §1G's table, and the read is one more move in §4I.

**What an associated type is.** A trait declares a type member, and each impl provides a concrete
value:

```vale
interface Iterator {
    comptime Item: type;
    abstract func next(virtual self &Iterator) Opt<Self.Item>;
}

struct Counter { }
impl Iterator for Counter {
    comptime Item = int;
    func next(self &Counter) Opt<int> { ... }
}
```

A bound can constrain the associated type:

```vale
func sum<T, I>(iter T) int where implements(T, Iterator), T.Item == I { ... }
sum(Counter())
```

**How it's stored.** Each impl carries its associated type values as `ITypeST`s alongside its sub and
super types. These are read-only and available from postparse time:

```
impl Iterator for Counter:
    sub:   CallST("Counter", [])
    super: CallST("Iterator", [])
    associated_types: { "Item" → CallST("int", []) }
```

When the associated type is generic — `impl<T> Iterator for Wrapper<T> { comptime Item = Pair<T>; }`
— the value is an `ITypeST` containing the impl's own runes:

```
impl<T> Iterator for Wrapper<T>:
    sub:   CallST("Wrapper", [ITypeST::Rune(T)])
    super: CallST("Iterator", [])
    associated_types: { "Item" → CallST("Pair", [ITypeST::Rune(T)]) }
```

**How §4I handles it.** The existing structural matching steps stay the same. One additional step
reads the associated type after the impl is selected:

 1. Walk impl's sub `ITypeST` against argument `KindT` → build impl rune map (e.g. `T → int`).
 2. Substitute into impl's super `ITypeST` → build super `KindT`.
 3. Assert no impl runes remain (§2.5.1).
 4. Match super `KindT` against bound's super `ITypeST` → build callee rune map.
 5. Check for conflicts with explicit args.
 6. **New step: for each associated type constraint (e.g. `T.Item == I`), read the value from the
    selected impl's `associated_types` map. It's an `ITypeST`. Walk it, substituting impl runes
    from step 1's map, to build a `KindT`. Conclude the callee rune (e.g. `I = int`).**

Step 6 is "lookup-read-substitute":

 * **Lookup**: the impl was already selected in steps 1-5. No searching.
 * **Read**: read `"Item"` from the impl's `associated_types` map. Get an `ITypeST`.
 * **Substitute**: walk the `ITypeST` using step 1's impl rune map to produce a `KindT`.

No solving. No rules. No CompilerOutputs mutation. The associated type value is a static `ITypeST`
on the impl, read and resolved through the same structural operations as everything else in §4I.

**Example with a generic associated type.** `sum(Wrapper<int>())`:

 1. Walk `Wrapper<int>` against impl's sub `Wrapper<T>` → map = `{ T → int }`.
 2. Substitute into super → `Iterator` (no impl runes in super).
 3. No unresolved runes. ✓
 4. Match `Iterator` against bound's `Iterator` → match.
 5. No conflicts.
 6. Read `Item` from impl: `CallST("Pair", [ITypeST::Rune(T)])`. Substitute `T → int` →
    build `KindT = Pair<int>`. Conclude `I = Pair<int>`.

§4 then has `I = Pair<int>` as an `InitialKnown`. If the function's body uses `I`, it's concrete.

**Why this doesn't need a solve.** The associated type value is declared on the impl — it is data,
not a derivation. Rust confirms this architecture: `TraitRef.args` never contains associated types,
and rustc processes associated type constraints through a separate pipeline
(`ProjectionPredicate` → `project_and_unify_term`) that finds the impl, reads the value, and checks
equality. Rustc's projection pipeline does call back into trait selection to find the impl, but §4I
has already found it, so the read is a field access.

**Cross-bound chains are the sort's job, not a solve's.** If an associated type's value references
another bound's output (`comptime Item = Other.Output`), that is a dependency edge between steps,
which §1G's topo sort orders like any other; a true cycle errors per §1G's rule. Two impls whose
`comptime` values reference each other (`Item = B.Item`, `Item = A.Item`) are a different beast: a
value cycle at the definitions, detected exactly during normalization (we evaluate a finite impl
set, so a revisit is a real diagnosis, unlike rustc's E0275 fuel gauge) and reported at the impls.

**What this rests on.** §4I's structural matching handles impl selection; the read-only `ITypeST`
carries the impl's sub/super types and its associated type values. Associated types cost one more
field on the impl and one more move in §4I. The optional destructuring-binding extension is parked
under Future Notions (`Pair<A, B> = T.Item`); when it activates, note the edges: a template
mismatch errors by name, a rune repeated in the pattern binds once and checks its second
occurrence, and the right side must be fully computable (its runes are inputs).

## Discussed examples and test cases

Cases we walked through, with what §1F does to each. All corpus quotes were read from the files.

### The four `contains`, after the rename (§1F)

```
1. str.vale:       func contains(haystack str, needle str) bool
2. str.vale:       func contains_slice(haystack str, needle StrSlice) bool
3. str_slice.vale: func contains(haystack StrSlice, needle str) bool
4. str_slice.vale: func contains_slice(haystack StrSlice, needle StrSlice) bool
```

| call | argument 0 | searches | finds |
|---|---|---|---|
| `contains("hi","hello")` | `str` | `str.vale` | #1, stops |
| `contains(mySlice,"hello")` | `StrSlice` | `str_slice.vale` | #3, stops |
| `contains_slice("hi",mySlice)` | `str` | `str.vale` | #2, stops |
| `contains_slice(mySlice,mySlice)` | `StrSlice` | `str_slice.vale` | #4, stops |

Stop-at-first is what makes this work. A collecting union would give #1 and #3 together for the first
row.

### The `+` family, and why stop-at-first needed the deletions (§1F)

`str.vale` declares `func +(a &str, b &str) str`. `tests/castutils/castutils.vale` declares six more,
all arity 2, three of them taking a `str` first. Any of those three collides with `str.vale`'s `+`
whichever parameter files it: by the second parameter, `"hi" + x` finds `+(&str, &str)` first and
stops, so the right function is never reached; by the first, they all land in `str.vale` together.
Renaming is unavailable because `+` is minted by the parser from a token.

After deleting all three, the survivors sit in four distinct files and no call reaches two:
`+(i int, s str)`, `+(b bool, s str)`, `+(f float, s str)`, `+(&str, &str)`.

**The general shape, for the next cross-product:** stop-at-first is sound only where a name appearing
in an earlier argument's file guarantees that file holds the right overload.

### `isEmpty` and virtual dispatch (§1F)

`opt.vale` declares six `isEmpty` and six `get`, every one arity 1, because `Opt`, `Some` and `None`
share a file. Splitting into `opt.vale` / `some.vale` / `none.vale` separates the abstract from its
overrides, and dispatch stays correct: `isEmpty(&mySome)` searches `Some`'s env and finds the
override, `isEmpty(&myOpt)` searches `Opt`'s env and finds the abstract. Static dispatch where the
concrete type is known, virtual where it is not.

Override lookup survives the split. `look_for_override` (`typing/edge_compiler.rs`) already passes
`extra_envs` holding both `get_outer_env_for_type(interface_template_id)` and
`get_outer_env_for_type(sub_citizen_template_id)`.

### The conversion functions (§1F)

`str(5)` matches the visible type `str`, so the first branch searches `str`'s files and finds
nothing; the second branch searches `int`'s files and finds `func str(x int) str`. This only works
because the two searches are additive. Finding one function through both branches would be harmless
anyway, since `get_candidate_banners_inner` and `find_potential_function` each dedup.

### `has.vale` — resolved by rename + file split

```vale
// rsa.vale:
func has_where<E, F>(arr &[]E, elem &E, equator &F) bool
func has<E>(arr &[]E, elem &E) bool
// ssa.vale:
func has_where<E, F, N Int>(seq &StaticArray<N, E>, elem &E, equator &F) bool
func has<E, N Int>(seq &StaticArray<N, E>, elem &E) bool
```

The equator-taking versions become `has_where`, the `==`-defaulting ones stay `has`. Split across
`rsa.vale` and `ssa.vale` per the Preparation section. Arity separates the two functions within each
file.

### Return position is not an inference source (§4.1)

`v = Vec<int>()` and `x Opt<int> = None<int>()` are the accepted spellings. A declared local type
never flows into the call on its right, so a rune nothing pins is an error rather than something the
expectation fills in. This is the one thing rustc's bidirectional inference buys that this design
gives up, and it is deliberate.

### §2 with a CallSR parameter — upcast changes the argument type (§2)

```vale
func callee<T>(x &Opt<T>) { ... }
callee(&Some<int>(5))
```

§2 reads the parameter's type tree → template is `Opt`. Argument template is `Some`. Different.
Match `Some<int>` against the impl's sub_citizen_type `Some<T>` → T = int; substitute into its
super_interface_type `Opt<T>` → `Opt<int>`. Hand `Opt<int>` to §4.
§7 emits an upcast `&Some<int>` → `&Opt<int>`.

### §2 with an ImplBoundS parameter — walk extracts conclusions, no upcast (§2)

```vale
impl IObserver<SignalA> for MyController;
func f<T, U>(x T) where implements(T, IObserver<U>) { ... }
f(MyController())
```

§2 reads the bound's super_interface_type → template is `IObserver`. Argument template is
`MyController`. Different. Match against the impl's trees, get `IObserver<SignalA>`. Hand that to
§4 as an initial known for the bound's super rune. Do NOT change the argument type — T stays
MyController. §7 emits no upcast.

§4's private rules then include `Call(IObserver, [U]) → super_rune`. It already knows super_rune =
`IObserver<SignalA>`. Runs the rule in reverse: U = SignalA. So §2 doesn't extract U itself — the
solver does it naturally.

Without this walk, §4 would have T = MyController but U unsolved.

### §2 ImplBoundS with nested generics and disambiguation (§2)

```vale
interface IHandler<T> {}
interface IEvent {}
struct ClickEvent {} impl IEvent for ClickEvent;
struct HoverEvent {} impl IEvent for HoverEvent;
struct Button {}
impl IHandler<IEvent<ClickEvent>> for Button;
impl IHandler<IEvent<HoverEvent>> for Button;

func handle<T, U>(x T) where implements(T, IHandler<IEvent<U>>) { ... }
handle<Button, ClickEvent>(Button())
```

§4I reads the bound's `super_interface_type` → template is `IHandler`. Two impls from Button to
IHandler. Ambiguous.

But the user wrote explicit template args: U = ClickEvent. The bound's super_interface_type is
`IHandler<IEvent<U>>`. Substitute U = ClickEvent into it: `IHandler<IEvent<ClickEvent>>`. No
solver; the nesting lives in the tree and substitution recurses through it.

§2 searches for `impl IHandler<IEvent<ClickEvent>> for Button`. One impl. Done.

### §2 zero impls — early error (§2)

```vale
func f<T>(x Opt<T>) { ... }
f(Dog())
```

§2 reads the parameter's type tree → template is `Opt`. Argument template is `Dog`. Different.
Search for impls from Dog to Opt. Zero found. Compiler error, stop.

### Common ancestor is not inferred (§4)

```vale
interface IShip {}
struct Firefly {} impl IShip for Firefly;
struct Serenity {} impl IShip for Serenity;
func moo<T>(a T, b T) { }
moo(Firefly(), Serenity())
```

§2 skips both (bare rune). §4 gets T = Firefly from argument 0, T = Serenity from argument 1.
Conflict. Error. The user writes `moo<IShip>(Firefly(), Serenity())`.

The existing test `assume_most_specific_common_ancestor` asserts the old common-ancestor behavior and
contradicts this design.

### §5 bound resolution searches rune values' envs (§5)

Three examples are in the Strategic Directions under §5 (§5.5, §5.6, §5.7). The key principle:
§5 substitutes rune values into the bound, then searches each **rune value's** env — not the
substituted parameter type's env. This is why `where exists clone(&T)T` at T=Ship searches Ship's env
(not &Ship's env), finding `clone(&Ship) Ship` in ship.vale.

### §5 with `==` bound — referent's env has the function (§5)

```vale
func has<E>(arr &[]E, elem &E) bool where exists ==(&E, &E)bool { ... }
has(myIntArray, 5)
```

§5 resolves `==(&int, &int)bool`. The rune is E = int. Search int's env (int.vale). Find
`==(&int, &int) bool`. Done. borrow.vale is never searched.

### Acceptance tests

- **`opt_with_undroppable_mutable_ref_contents`** is the right one to measure the phases by. Its
  mismatch is `Some<&Spaceship>` against `Opt<&Spaceship>`, which §2 resolves.
- **`downcast_with_as` is not.** It fails on a `str` local mentioning as `&str` against a bare
  parameter, which is the mention model rather than the call site.

## Background and Current State

What the compiler does today, so the phases above can be planned against it. Every claim carries a
reference a sub-agent can check without asking: a code file plus a symbol name, or a markdown path
plus the date that passage was last updated.

Everything here describes the **code**, not the design. Where a ruling and the tree disagree, this
section reports the tree.

### What rustc does here, and which parts transfer

Read from `~/rust` this session rather than from notes.

**rustc checks arguments in two passes, and says why in place.**
`check_argument_types` (`compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs`, around line 429):

```rust
// Check the arguments.
// We do this in a pretty awful way: first we type-check any arguments
// that are not closures, then we type-check the closures. This is so
// that we have more information about the types of arguments when we
// type-check the functions. This isn't really the right way to do this.
for check_closures in [false, true] {
    // More awful hacks: before we check argument types, try to do
    // an "opportunistic" trait resolution of any trait bounds on
    // the call. This helps coercions.
    if check_closures {
        self.select_obligations_where_possible(|_| {})
    }
```

The reason is not lifetimes, coherence, or Rust's lack of overloading. It is that a Rust closure
literal has no type of its own: its signature is inferred from the expectation, which the other
arguments pin down.

**That reason does not transfer, because Vale's lambdas are templates rather than generics.** Per
@LAGTNGZ (`docs/arcana/LambdasAreGenericTemplatesNotGenerics-LAGTNGZ.md`), a lambda expression
produces a closure struct whose name takes no template arguments at all —
`LambdaCitizenTemplateNameT.make_struct_name` asserts the args are empty — so the argument
`&{ _ == _ }` has a fully concrete type the moment it is written. Its *parameter* types are not
inferred here and are not needed here. They are fixed later, when the callee's
`where func(&F, &E, &E)bool` bound is discharged in §5, which expands the `__call` template at
concrete arg types and bakes them into `LambdaCallFunctionTemplateNameT`.

So the ordering holds without a second pass: §4 concludes `E` from `arr` and `F` from the closure
struct, and §5's bound resolution is where the lambda body gets typed. Every lambda-argument site in
the corpus is this shape — `has.vale`, `arrays.vale`, `migrate.vale`, `hashmap.vale`, and the
array-from-callable tests.

**Rust has no overload resolution for free functions at all.** A path names exactly one function, so
`foo(x)` needs no candidate set. All of Rust's resolution machinery is in method calls, where
`method::probe` runs the solver per candidate and eliminates on failed obligations. §1F is doing
something closer to probing than to path resolution, with less information than probe has.

**What rustc lacks that this plan has.** `coerce_unsized` drives `SelectionContext::select` from
inside coercion, because a coercion must decide whether to write an adjustment and there is nowhere
to record *maybe*. Its escape is a whole-body writeback pass that rewrites adjustments afterward.
The §2-decides / §7-emits split needs no writeback, so this plan avoids the problem rather than
inheriting it.

**Where rustc's speed actually comes from.** Trait selection is heavily cached, and the obligation
pool is per typeck-root rather than per call site. This plan scopes per call site, which is more
local, but §5 resolves each bound by a full recursive call resolution with no memo, and
`resolve_impl` → `check_resolving_conclusions_and_resolve` → `is_parent` → `resolve_impl` has no
depth guard. Dropping bidirectionality does not by itself buy compile time.

**What does not transfer.**

- **Integer literal inference.** Rust's literals are inference variables defaulting to `i32`. Vale
  converts explicitly (`func i64(x &int) i64`), so one classic source of bidirectional pressure is
  simply absent.
- **Region inference.** rustc erases regions at writeback and re-typechecks the whole MIR body to
  regenerate constraints. Vale keeps the region on the type.
- **LUB.** `try_find_coercion_lub` is reachable from match arms, if/else, loop/break, array literals
  and the return coercion, never from call arguments. No-most-specific-common-ancestor costs nothing.

### Candidate lookup, as it runs today

`get_candidate_banners` (`typing/overload_resolver.rs`) collects from four sources and unions the
results, applying no ordering afterward:

- the calling environment
- `get_param_environments`, one environment per argument type
- `get_placeholder_extra_call_envs`, the interface envs that a placeholder argument's bounds name
- `extra_envs_to_look_in`, which its own comment records as empirically dead on the corpus

**`get_param_environments` reads the outer env only, for three kinds only.** `KindT::Struct`,
`KindT::Interface` and `KindT::KindPlaceholder` each yield `get_outer_env_for_type`. Every other kind
falls through to `_ => Vec::new()`.

**A borrow argument therefore contributes no namespace.** Under the onion, a `&Ship` argument is a
`KindT::BorrowRef`, which matches none of those three arms. Nothing peels the wraps before the match.

**A primitive argument contributes no namespace either, and cannot.** `int`, `i64`, `bool`, `float`,
`str`, `void` and `__never` are registered in `Compiler::compile` as `INameT::Primitive` entries in
the builtin package's top-level store. They are kinds, not templates, so they have no defining file
and no `type_name_to_outer_env` entry to fetch. There is no `int.vale`; `builtins/resources/` holds
23 files and the arithmetic lives in `arith.vale`, the drops in `drop.vale`, `print(s &str)` in
`print.vale`.

**An array argument contributes no namespace, and its environment is empty.**
`KindT::RuntimeSizedArray` and `KindT::StaticSizedArray` are not among `get_param_environments`'
three arms. `array_compiler.rs` does declare an outer and an inner env per array template, but the
outer one is a synthesized `CitizenEnvironmentT` built from
`TemplatasStoreBuilder::new(template_id).build_in(...)` with nothing added, so it holds no functions.
`arrays.vale`'s contents are not in it. Arrays therefore need the same registration work as
primitives, not merely two new match arms.

### What a bound is, and what registering one means

**A bound is a lookup, never a predicate.** The whole bound vocabulary is `PrototypeT` and `IdT`
(`InstantiationBoundArgumentsT`, `typing/hinputs_t.rs`), and discharging one means calling the
overload resolver and seeing whether it finds anything. There is no assertion rule in `IRulexSR`.

Three kinds exist:

- **Function bounds**, written `where exists drop(T)void`. `opt.vale` declares
  `func drop<T>(opt Some<T>) where exists drop(T)void`. Postparse lowers it to an `IRulexSR::Resolve`,
  and `resolve_function_call_conclusion` (`typing/infer_compiler.rs`) discharges it: it reads the
  concluded parameter and return types out of the conclusions, calls `resolve_function` with
  `exact = true`, and then checks the found prototype's return type matches. Its two failures are
  `CouldntFindFunctionForConclusionResolve` and `ReturnTypeConflictInConclusionResolve`.
- **Impl bounds**, written `where implements(T, IShip)`. Carried as `ImplBoundS` rather than as a
  rule, and discharged by `is_parent` (`typing/citizen/impl_compiler.rs`). Its failure is `IsaFailed`.
- **Reachable bounds**, which nobody writes. If a callee's parameter mentions `Some<T>`, and `Some`
  itself declares bounds on `T`, those must hold at this call site too. `get_reachable_bounds`
  (`typing/templata_compiler.rs`) fetches the citizen's bound prototypes, and each is resolved by the
  same `resolve_function` path.

**Registering means leaving the answers behind for the instantiator.** The instantiator stamps out a
copy of the callee specialized to these type arguments, and inside that copy every bound call must
point at a concrete prototype. It does **zero** verification of its own, so it can only substitute
what the typing pass recorded. That record is `InstantiationBoundArgumentsT`, three maps:

- `rune_to_bound_prototype` for function bounds
- `rune_to_citizen_rune_to_reachable_prototype` for reachable bounds
- `rune_to_bound_impl` for impl bounds

`add_instantiation_bounds` (`typing/compiler_outputs.rs`) writes it into
`coutputs.instantiation_name_to_bounds`, keyed by the callee instantiation's `IdT`.

So calling `drop(mySomeOfShip)` checks that `drop(Ship)void` exists, and registers "for
`drop<Ship>(Some<Ship>)`, the `drop(T)void` bound is satisfied by the prototype `drop(Ship)void`".

### How the bound checks are ordered, and why the write is one shot

`check_resolving_conclusions_and_resolve` (`typing/infer_compiler.rs`) checks in four stages,
collecting into `reachable_bounds`, `runes_and_prototypes` and `runes_and_impls`, then builds one
`InstantiationBoundArgumentsT` at the end and **returns** it for its caller to register. The order is
load-bearing:

- **Reachable bounds first**, because `import_reachable_bounds` builds the environment every later
  stage resolves in.
- **Template calls**, discharging the struct and interface resolutions the value solve postponed per
  SFWPRL.
- **Function bounds**, the `Resolve` rules.
- **Impl bounds** last, because they write their results back into the conclusions map.

`add_instantiation_bounds` (`typing/compiler_outputs.rs`) takes a whole `InstantiationBoundArgumentsT`
and is write-once: a second write whose contents differ trips an equality assert, under a comment
saying *"sometimes when we evaluate the same thing twice we get different results."*

### Resuming a finished solve

`commit_step` (`solver/simple_solver_state.rs`) takes `new_rules` and `new_runes`, appends the rules,
and registers their puzzles into `open_rule_to_puzzle_to_runes`. It asserts
`self.all_runes.contains(rune)` for every rune in a new rule's puzzle, so a rule's runes must arrive
in the same call as the rule.

`incrementally_solve` (`typing/infer_compiler.rs`) loops `r#continue` around a callback that commits
more work. That is how @DRSINI injects generic parameter defaults partway through a solve.

`derive_rune_to_type` (`typing/rune_typing/derive.rs`) is on-demand and cached nowhere, so deriving
twice costs nothing beyond the second derivation.

### Checking a bound is resolving it

There is no lighter "does it exist" path in the tree. `resolve_function_call_conclusion` calls
`resolve_function`, which calls `find_function`, which calls `find_potential_function`, which runs
`get_candidate_banners` and then `attempt_candidate_banner` on every candidate it found.

For a `Function` candidate, `attempt_candidate_banner`
(`typing/overload_resolver.rs`) rune-types the explicit template args, runs `solve_for_resolving`,
and then calls `evaluate_generic_light_function_from_call_for_prototype`, or
`evaluate_templated_function_from_call_for_prototype` when the callee is a lambda. Those are the
function compiler. They produce the callee's header and prototype.

Immediately after, on every success path, it asserts:

```rust
assert!(coutputs.get_instantiation_bounds(
    self.typing_interner, resolve_success.prototype.prototype.id).is_some());
```

So by the time a candidate is accepted, **the callee's own instantiation bounds are already
registered**.

The reason there is no lighter path is structural rather than incidental: for a generic callee like
`func drop<T>(x &T)`, you cannot know its signature at `T = Ship` without running its solve, and
running its solve is what produces the prototype.

**Bodies are not compiled here.** The `..._for_prototype` names are accurate; they stop at the
header. `finish_function_maybe_deferred` (`typing/function/function_compiler_core.rs`) drains bodies
later, off the deferred queue that `Compiler::evaluate` walks.

### The calling env's package level is a union over every namespace

Verified this session. `PackageEnvironmentT::lookup_with_imprecise_name_inner`
(`typing/env/environment.rs`) extends its result from the builtins store, then loops
`for global_namespace in self.global_namespaces` and extends from each. It takes a
`get_only_nearest` parameter and never reads it, so there is no nearest-wins shadowing.

And `global_namespaces` is *every* top-level environment:

```rust
let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
    global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
```

`Compiler::compile` pushes the Rust package stores into that same list
(`for (package_id, store) in rust_package_stores(self)`), guarded only by a panic if a Vale package
claims the reserved `rust` module. So every function in every package, Vale or Rust, is reachable by
name from every call site's package env.

### A citizen's inner env holds runes, not methods

`compile_struct_layer` (`typing/citizen/struct_compiler_generic_args_layer.rs`) builds the inner env
from exactly one thing: the generic-parameter bindings, `inferences.iter().map(|(rune, templata)| (INameT::Rune(...), IEnvEntryT::Templata(...)))`.
Nothing else is added. So §1F's *"the methods defined inside the struct/interface"* names something the
tree does not have yet; the inner env is a rune-binding scope.

Per-citizen functions are synthesized as **siblings** instead. `get_struct_sibling_entries` dispatches
to `get_struct_sibling_entries_struct_constructor` and `get_struct_sibling_entries_struct_drop`
(`typing/macros/macros.rs`), and each returns an entry keyed under the citizen's *containing* env, so
the results land in that citizen's outer env.

### Which names have an environment

`declare_type_outer_env` and `declare_type_inner_env` (`typing/compiler_outputs.rs`) are called for
structs and interfaces (`typing/citizen/struct_compiler.rs`,
`typing/citizen/struct_compiler_generic_args_layer.rs`), understructs
(`typing/citizen/struct_compiler_core.rs`), impls (`typing/citizen/impl_compiler.rs`), arrays
(`typing/array_compiler.rs`), and kind placeholders (`typing/templata_compiler.rs`).

**Primitives are on neither list.** `int`, `i64`, `bool`, `float`, `str`, `void` and `__never` never
reach `declare_type`, so `get_outer_env_for_type` on one of them takes its
`None => panic!("No outer env for type: {:?}", name)` arm.

### Functions named after a primitive type

Six exist in the builtins, and a call to any of them names a visible type:

- `func float(x &int) float`, `func int(x &float) int`, `func i64(x &int) i64` in `arith.vale`
- `func str(x int) str`, `func str(x i64) str`, `func str(x float) str` in `str.vale`

### What `as` is today

`builtins/resources/as.vale` declares two overloads, and both are **downcasts**:

```vale
extern("vale_as_subtype")
func as<SubType, SuperType>(left &SuperType) Result<&SubType, &SuperType>
where implements(SubType, SuperType);

extern("vale_as_subtype")
func as<SubType, SuperType>(left SuperType) Result<SubType, SuperType>
where implements(SubType, SuperType);
```

The first explicit template argument is the **sub** type, the parameter is the **super** type, and
the result is a `Result`, not a plain reference. `as.vale` declares no type, and both parameters are
bare runes.

There is no upcast counterpart anywhere in the builtins.

**`as` does not work today.** Measured with `cargo test --lib downcast`: 1 passed, 3 failed. §2.1's
disambiguating cast therefore rests on machinery that has never run end to end.

`downcast_with_as` fails for a reason unrelated to casting: a `str` local mentions as `&str` and
reaches a parameter declared bare. That is the mention model rather than the call site, so the
phases neither cause nor cure it, and this test is not an acceptance test for them.

### Where a synthesized constructor lives

`get_struct_sibling_entries_struct_constructor` (`typing/macros/struct_constructor_macro.rs`) returns
its function under an id built from `struct_name.package_coord` and `struct_name.init_steps`, so the
constructor is a **sibling** of the struct, in the struct's containing environment. It is not in the
struct's inner env.

Its parameters are the struct's members. For `struct Some<T> { value T; }` that is `Some(value T)`,
whose one parameter is the bare rune `T`. So the constructor mentions no concrete type in any
parameter, and the type it constructs appears only in its return.

**No call-site path reads an inner env.** `get_inner_env_for_type` (`typing/compiler_outputs.rs`)
exists, takes `&self`, and reads `type_name_to_inner_env`, but nothing in lookup calls it.
`get_inner_env_for_function` in the same file is `panic!("Unimplemented: Slab 10")`.

**Four templata kinds panic inside the candidate loop.** `get_candidate_banners_inner` handles
`ITemplataT::Prototype` and `ITemplataT::Function`. `OverloadSet`, `Struct`, `Interface` and
`ExternFunction` each reach a `panic!` carrying the Scala they were ported from.

### Arity, and where parameters are compared

`params_match` (`typing/overload_resolver.rs`) checks arity first, then compares each parameter pair,
either by equality or through `is_type_convertible`, chosen by its `exact` flag. All three of its
callers sit inside `attempt_candidate_banner`, **after that candidate has been solved**. So arity
rejects a candidate late rather than during lookup.

### A parameter's two rule buckets

`ParameterS` (`postparsing/ast.rs`) stores:

- `full_type_rune`: the rune for the outer wraps plus the value type they enclose. Its doc comment
  states it equals `value_type_rune` when `type_outer_ref_rules` is empty.
- `value_type_rune`: the rune for the named-type root, past the outer wraps.
- `type_outer_ref_rules`: the wraps that build the full type. Its doc comment admits `BorrowRefSR`
  and `WeakRefSR` only, though `OwnRefSR` is a live variant elsewhere.
- `value_type_rules`: the `Lookup`/`Call` that build the value type.

**Empty `value_type_rules` means the value type is a bare rune.** `translate_signature_templex`
(`postparsing/rules/templex_scout.rs`) peels the wrap layers into `type_outer_ref_builder` and hands
the non-ref root to `translate_type_into_rune`. A written type name emits `Lookup` plus a zero-arg
`Call` per @TNLTZACZ; a rune emits nothing and returns the rune itself. So the discriminator is
statically readable: an outermost `Call` gives a template name, and its absence means the parameter
accepts anything.

**One solve consumes both buckets today.** Three sites in
`typing/function/function_compiler_solving_layer.rs` build a single `all_rules` from
`function.header_rules ++ params.flat_map(value_type_rules ++ type_outer_ref_rules)`, and hand that
same list to the solve and to `derive_rune_to_type`. `include_rule_in_call_site_solve`
(`typing/infer_compiler.rs`) excludes only `DefinitionFunc`, so both buckets reach the call-site
solve.

### Sends

`assemble_initial_sends_from_args` (`typing/function/function_compiler_solving_layer.rs`) has four
callers, all in that file:

- **Two consume their sends.** Each pushes an `InitialKnown` for the sender rune and an `Equals`
  tying sender to receiver.
- **Two never run.** The two `evaluate_templated_*` sites open with
  `unimplemented!("header_rules alone: fold in the per-param type-binding rules, see @PFVSZ")`, which
  precedes their `assemble_call_site_rules(function.header_rules)` call.

### Impl walking

- `get_impl_parent_given_sub_citizen` (`typing/citizen/impl_compiler.rs`) seeds the impl's
  `struct_kind_rune` with the child kind, calls `resolve_impl`, and reads `interface_kind_rune` back
  out. It panics four ways if the conclusion is missing or is not an interface.
- `is_parent`'s fast path (same file) finds an already-compiled `IsaTemplataT` matching **both** sides
  concretely, then calls `coutputs.add_instantiation_bounds` with three empty vectors. So the fast
  path writes.
- `is_parent` accepts `ITemplataT::ImplDefinition` and `ITemplataT::Isa` from one lookup, so a
  declared bound and a real impl edge arrive by the same route with nothing distinguishing them.
- `ImplBoundTemplate` panics in four places: `resolve_impl`, `partial_resolve_impl`, `compile_impl`,
  and `is_parent`. That is the shape a `where` clause produces inside a generic body.

### The read-only solve already exists, and has a name

- `partial_solve` (`typing/infer_compiler.rs`) is `make_solver_state`, then `continue`, then
  `userify_conclusions`. It runs no resolve step.
- `predict_struct_layer` (`typing/citizen/struct_compiler_generic_args_layer.rs`) builds a `StructTT`
  through `partial_solve` and deliberately registers nothing, saying so in place: *"Usually when we
  make a StructTT we put the instantiation bounds into the coutputs, but we unfortunately can't here
  because we're just predicting a struct; we'll try to resolve it later and then put the bounds in.
  Hopefully this StructTT doesn't escape into the wild."*
- `partial_resolve_impl` (`typing/citizen/impl_compiler.rs`) is the impl-shaped sibling. It returns
  conclusions and stops before `check_resolving_conclusions_and_resolve`. It still takes
  `&mut CompilerOutputs`.

## Open Questions

### Need a ruling from the architect

- **What is the implicit set, exactly?** Searching it *before* the arguments forces it to be a fixed
  reserved list rather than the calling env, since the calling env's package level unions every
  namespace (see Background) and would find all four `contains` before `str.vale` was ever consulted.
  And the list can only hold names a user cannot declare, or the same masking hits user code.
- **How does a user disambiguate when §1F finds several, before `MyStruct.fromInts(...)` exists?** No
  call-site syntax exists yet. An explicit import is file-scoped, so it would disambiguate every call
  in the file rather than the one that is ambiguous.
- **What does a wrong argument type report?** Whichever phase rejects it, the natural message is a
  rule-level conflict naming a rune rather than "argument 0 is a `MyStruct`, expected an `Opt<T>`".
  That is the shape `opt_with_undroppable_mutable_ref_contents` produces today. Every candidate phase
  knows which argument each value came from, so any of them could say the better thing.
- **Numbering.** The preamble lists eight phases; phase 1 is split into §1B, §1F, §1G, §1H. The
  S1/S2 paragraph scheme from design-assistant is not yet in use, so Plan Details cites phase tags.
- **Steps ordered only by their inputs leave one corner underdetermined.** A bound's super-side
  rune that a *different bound* produces (e.g. `implements(T, IObserver<U>)` beside
  `where exists moo(W)U`): input-only edges leave the two steps unordered, and whether the walk
  filters by U or ambiguity-errors depends on the tiebreak. Written-position tiebreak is
  deterministic and declaration-static; the alternative is also ordering a step after the producers
  of runes it merely mentions, when acyclic. Rare; wants a ruling eventually.
- **`assume_most_specific_common_ancestor` contradicts the design.** The test
  (`compiler_solver_tests.rs:706`) asserts that `moo(Firefly(), Serenity())` upcasts both to IShip.
  The design says no common ancestor — the user writes `moo<IShip>(...)`. Test needs updating to
  expect an error, or rewriting with the explicit type argument.

### Out of scope but worth knowing

- **Let-binding upcasts are not covered by the phases.** `ship IShip = Raza(42)` goes through
  `infer_and_translate_pattern` → `convert()` → `convert_via_upcast`, not through §1B–§8.
  Already working since the `UpcastTE::new` fill. No action needed.

### Answerable from the code, unmeasured

- **How many call sites in the suite have a parameter with an unsolved rune and an argument that
  needs an upcast to fill it?** That is the population §2A/§4I's teaching exists for, and it has
  never been counted.
- **How many corpus bounds introduce runes their parameters don't mention** (the `U` in
  `implements(T, IObserver<U>)`)? Zero would mean the teaching steps are pure future-proofing for
  the current corpus.
- **How many `Prot`-typed generic params survive in the corpus** (the HashMap-era `H`/`E`
  prototype params)? They are the one existing feature that leans on func-bound teaching
  (`CallSiteFuncSR` unpacking) today.
