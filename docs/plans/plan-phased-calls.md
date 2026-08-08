# Phased Calls

Redesigning generics solving at a call site, from scratch.

## Strategic Directions (human-only)

A call site does these phases:

 * Phase 1: Candidate selection / filtering. Narrow down the exact *only* candidate to attempt the rest with.
 * Phase 2: Upcastability, for each argument try to solve the impl that casts it to the expected parameter template.
 * Phase 3: Rune-typing, to figure out the types of all the callee's runes.
 * Phase 4: Main solve, to detect problems and determine the function's runes mapping.
 * Phase 5: Resolve bounds and register instantiation bounds.
 * Phase 6: Reference-solve: extend the main solve with type_outer_ref_rules to calculate each parameter's full_type_rune.
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
x Opt<int> = None();
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
 * Make it so postparsing doesnt produce rules, instead it produces ITypeST. (Phase 4 will privately make temporarily transient rules for its own purposes, otherwise, nobody will ever see rules)

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
    * Ignore any references on it (peel them away). Save these peeled args, phase 2B will use them.
    * Look at its type's package env (package's top-level namespace).
    * Look at its type's outer env (the methods defined inside the struct/interface).
    * In both, find any method of the desired name, and the right arity. If you found zero, continue to next one. If one, stop here. If multiple, show an error, require them to disambiguate.
 2. If the function has the same name as a type that is visible to (imported by) the callsite:
    * Look at its type's package env (package's top-level namespace).
    * Look at its type's outer env (the methods defined inside the struct/interface).
    * In both, find any method with that name of the right arity. If you found zero, continue to step 3. If one, stop here. If multiple, show an error, require them to disambiguate.
 3. If the function was imported by the callsite:
    * Check if it has the right arity. If found zero, continue to step 4. If one, stop here. If multiple, show an error, require them to disambiguate.
 4. If zero were found, show an error.

We stop at the first one, to support this bunch of functions:
 1. str.vale: `func contains(haystack str, needle str) bool`
 2. str.vale: `func contains_slice(haystack str, needle StrSlice) bool`
 3. str_slice.vale: `func contains(haystack StrSlice, needle str) bool`
 4. str_slice.vale: `func contains_slice(haystack StrSlice, needle StrSlice) bool`
Otherwise, `contains("hi","hello")` has a conflict between #1 and #3.

### Phase 1H: Check explicit template args types against callee §1H

Make sure that the explicit template args types' match the expected types (generic_params[i].tyype).

### Phase 1L: Lookups and Literals §1L

Extract and process the callee's LiteralSR and LookupSR and RuneEnvParentLookupSR rules.

These will be `InitialKnown`s for all solves in the rest of the phases.

### Phase 2A: Dyn Upcastability. §2

Look at each callsite argument (the "uncoerced argument"). For each:
 * Get the corresponding callee parameter value_type_rune.
 * If there's a CallSR rule whose result rune is the parameter's value_type_rune, then continue. Otherwise, skip to the next argument.
 * Get the CallSR's template, that's the "expected template".
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
 * Once we have that impl, do a solve on it to turn the `Some<int>` into its parent `Opt`. In other words, take `Some<int>`'s `<int>` and plug that into the `impl`'s solve and get the resulting trait, which is `Opt<int>`.
 * Remember the `Opt<int>`. We'll be handing that in as the argument in later phases, instead of the original callsite argument.
 * Note that the solve should not be registering any instantiation bounds yet. We must remember to do that at the end when we're sure this call will work.
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

### Phase 2B: Trait Upcastability. §2.5

Look at each callsite argument (the "uncoerced argument"). For each:
 * Get the corresponding callee parameter value_type_rune.
 * For each ImplBoundS whose sub_rune is the parameter's `value_type_rune`:
    * That ImplBoundS's super rune is the "expected template".
    * If the argument template and expected template are the same, then skip this argument. If the same, continue.
    * Assert no outer references on both (argument outer refs should have been peeled by phase 2B and value_type_rune doesn't contain outer refs)
    * Find all impls for the uncoerced argument type and the parameter type. In this example:
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
       1. Look up the impl's placeholdered struct (`MyController<Z>`) and placeholdered interface (`IObserver<Z, int>`), which were produced back when compiling the impl's definition.
       2. Recursively compare the impl's placeholdered struct (`MyController<Z>`) with the uncoerced argument type (`MyController<A>`). Note what placeholders in the former (`Z`) is matched with what in the latter (`A`). Build a map of impl_placeholder_to_argument_type (`Z` -> `A`).
       3. Using that map (`Z` -> `A`), substitute into the impl's placeholdered interface (`IObserver<Z, int>`) and note the result (`IObserver<A, int>`) which is phrased in terms of the caller.
        * Assert that the result doesn't contain any impl placeholders (§2.5.1)
       4. Match callee bound interface (`IObserver<X, U>`) against step 3's result (`IObserver<A, int>`). Note what placeholders in the former are matched with what in the latter (`X` with `A`, `U` with `int`). Build a map of callee_placeholder_to_argument_type (`X` -> `A`, `U` -> `int`).
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
  1. Look up impl's placeholdered struct (`MyController`), placeholdered interface (`IObserver<SignalA>`).
  2. Recursively compare `MyController` with `MyController`, resulting in empty map.
  3. Substitute nothing, get `IObserver<SignalA>`.
  4. Match callee bound interface (`IObserver<U>`) against (`IObserver<SignalA>`), get map `U` -> `SignalA`.
  5. Check that map (`U` -> `SignalA`) against explicit args (`U` -> `SignalA`). Agrees.
  6. Remember `IObserver<SignalA>`.
For impl 2 (`impl IObserver<SignalB> for MyController`):
  1. Look up impl's placeholdered struct (`MyController`), placeholdered interface (`IObserver<SignalB>`).
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
This is why each bound (ImplBoundS, ResolveSR) carries its own rules list.
 * The first impl `impl IHandler<IEvent<ClickEvent>> for Button` contains its `IEvent<ClickEvent>` etc rules.
 * The second impl `impl IHandler<IEvent<HoverEvent>> for Button` contains its `IEvent<HoverEvent>` etc rules.
And we can pull those in for the argument impl solve without pulling in everyone else's rules.

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
We don't support these. If we wanted to, then we'd need to check for any unresolved impl placeholders at §2.5.1 and let them get to the next step, which would map callee runes to those, resolved by explicit args (or some solving).

### Phase 3: Rune-typing. §3

 * We use the rune-typing to determine the type of each rune.

### Phase 4: Value-solve §4

 * This solve uses ParameterS's value_type_rules and value_type_rune, NOT full_type_rune and type_outer_ref_rules. This solve doesn't know about full_type_rune or type_outer_ref_rules at all.
 * We run the solver, feeding in phase 2's coerced arguments as initial knowns for the value_type_rune runes.
 * We shouldn't register any instantiation bounds yet. We must remember to do that at the end when we're sure this call will work.

Note that this phase's main goal is to work out the callee's runes and detect errors, like (§4.1):
 * Calling `f<T>(a T, b T)` like `f(5, true)` will error because `T` cannot be both int and bool.
 * Calling `func f<T>(x T) { }` like `f<int>(true)` will error because `T` cannot be both int and bool`
 * Calling `func zero<T>() T { ... }` like `zero()` will error because `T` wasn't specified.

This phase only ever sees these rules:
 * Equals
 * Call
 * KindList
 * BorrowRef/WeakRef/OwnRef (the ones in value_type_rules, not the ones in type_outer_ref_rules)

Note I'm not including **DefinitionFunc** in that list, because this doc is about the callsite, not about the definition site. We'll want a separate doc to talk about splitting apart the definition-side solve into phases, that could remove the DefinitionFunc from that solve.

### Phase 5: Resolve §5

Check that that all of the callee's required bounds are met.

To do this, it should:
 1. Do all substitutions into the bound.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Note that we *only* look in the rune substitution value's environment, no other environments (*not* the caller's environment anymore).
    * Note that we *aren't* peeling references away from the rune substitution value, before looking in that type's environment.
 3. Error if not exactly one was found.

value-drop example (§5.5):
```
-------- main.vale --------
func foo(x Some<Ship>) {
  drop(x^)
}
-------- some.vale --------
func drop<D>(opt Some<D>) where func drop(D)void { ... }
-------- ship.vale --------
struct Ship { }
func drop(self Ship) { ... }
-------- borrow.vale --------
func drop<D>(r &D) { ... }   // <-- We don't want to call this
```
To properly compile `drop(x^)`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where func drop(D)void` and D=Ship into `where func drop(Ship)void`
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
func clone<T>(opt Some<T>) where func clone(&T)T { ... }
-------- ship.vale --------
struct Ship { }
func clone(self &Ship) Ship { ... }
-------- borrow.vale --------
func clone<T>(r &&T) &T { ... }   // <-- We don't want to call this.
```
To properly compile `z.clone()`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where func clone(&T)T` and T=Ship into `where func clone(&Ship)Ship`.
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
func clone<T>(opt Some<T>) where func clone(&T)T { ... }
-------- ship.vale --------
struct Ship { }
func clone(self &Ship) Ship { ... }   // <-- We don't want to call this.
-------- borrow.vale --------
func clone<T>(r &&T) &T { ... }
```
To properly compile `z.clone()`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where func clone(&T)T` and T=&Ship into `where func clone(&&Ship)&Ship`.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did T=&Ship, the rune substitution value is `&Ship`, so we look in the **borrow ref** environment (borrow.vale) for something named `clone`, and find `func clone<T>(r &&T) &T`, stop.
Note that we *aren't* peeling references away from the rune substitution value (&Ship). We take it straight, and look in that type's environment. That's why we looked in borrow.vale for a `clone`, instead of finding the `func clone(self &Ship) Ship` in ship.vale.

placeholder example (§5.8):
```
func bar<Y>(x Y) where func drop(Y)void { ... }
func foo<T>(x T) where func drop(T)void {
  bar(x)
}
```
To properly compile `bar(x)`, it should:
 1. Do all substitutions into the bound.
    * Here, we turn `where func drop(T)void` and T=`foo$T` into `where func drop(foo$T)void`.
 2. Determine the envs to look in. For each rune substitution value, look for a method in that value's type's environment.
    * Here, since we did T=`foo$T`, the rune substitution value is `foo$T`, so we look in `foo$T`'s environment for something named `drop`, and find `foo`'s `where func drop(T)void`, stop.
Note that this requires that bounds, like `foo`'s `func drop(T)void` need to be declared inside the `foo$T` placeholder's environment.


### Phase 6: Reference-solve §6

 * Add the type_outer_ref_rules to the (completed) phase 4 solve.
 * Solve.
 * For each parameter, remember the type in its solved full_type_rune.

### Phase 7: Convert §7

 * Insert callsite instructions to upcast the argument to the expected full_type_rune type.
 * If the language ever does auto-ref, that would happen here. For now, we'll require & at callsites.

### Phase 8: Borrow check §8

 * (Not designed yet)


## Strategic Directions Proposals

§22: A bound declares a function in the placeholder's env.

 * User-defined functions satisfying bounds on primitives must live in that primitive's file.
   `func moo(x int)` goes in int.vale. If it's not there, §5 can't find it.
 * No fallback to the full calling env.

§23: **CallSR vs ImplBoundS difference:** When the parameter template came from a CallSR (e.g.
  `x Opt<T>`), §2 changes the argument type (Some→Opt) and §7 emits an upcast. When it came from
  an ImplBoundS (e.g. `x T where implements(T, IObserver<U>)`), §2 walks the impl to extract
  conclusions for other runes (like U = SignalA) but does NOT change the argument type — T stays
  the concrete type (MyController), and §7 emits no upcast. The walk is for discovering rune
  values, not for converting the argument.

§25: §2B uses structural matching against impls, not per-impl solving.

 * §2B already knows the concrete target super type (from explicit args + bound's rules) before it
   searches for impls. So the question is "does this impl match this specific type?" not "what does
   this impl produce?" Matching is sufficient; solving is for discovery.
 * For a concrete impl (`impl IObserver<SignalA> for MyController`): equality check.
 * For a generic impl (`impl<T> IObserver<T> for MyController`): compare the impl's super
   (`IObserver<T>`) against the known target (`IObserver<SignalA>`) by recursing through template
   args. T = SignalA falls out structurally.
 * Nested generics (`impl<T> MyTrait<BTrait<T>> for MyStruct<T>`) work the same way — match sub to
   get T, substitute into super, compare against target. Just deeper recursion.
 * No environments, no CompilerOutputs, no side effects. Pure structural comparison.
 * When associated types land, this extends with a lookup-read-substitute step (find the impl, read
   the associated type value, substitute). Still not a full solve.

§26: The postparser produces structured type trees (`ITypeST`), not solver rules, for types in
  parameter positions, impl sub/super types, and bound operands.

 * Today the postparser lowers `IObserver<U>` to rules: `Lookup("IObserver") → r1, Call(r1, [U]) →
   super_rune`. Getting the tree shape back requires traversing the rule chain or running the solver.
 * Instead, the postparser stores the tree directly: `ITypeST::Apply { template_name: "IObserver",
   args: [ITypeST::Rune(U)] }`. The tree is available from the moment the bound is postparsed, with
   no dependency on any denizen's definition-solve.
 * `ITypeST` is **read-only and never synthesized at runtime**. The typing pass reads it but never
   creates new `ITypeST` values. It can build `KindT` values by walking an `ITypeST` and resolving
   each rune — that's "read the pattern, produce a value," not mutating the pattern.
 * §4's solver privately builds rules from `ITypeST` when it needs them. Rules become an internal
   implementation detail of the solver, not the shared representation between passes.
 * This is rustc's shape: `EarlyBinder<TraitRef { def_id, args }>` is their equivalent — a
   structured type tree available from the collect phase, never modified, and the solver works with
   it directly rather than evaluating rule chains.
 * **Why this is needed:** without it, §2B's structural matching has a chicken-and-egg: the bound's
   super type is only available as a `KindT` after the callee's definition-solve runs (which creates
   placeholders and evaluates the rules). But a call site can reach §2B before the callee's
   definition-solve has run (the system is demand-driven and interleaves headers and bodies). Forcing
   the definition-solve first leads to deadlocks. `ITypeST` breaks the dependency: the tree shape is
   available from postparse time, no definition-solve needed.
 * Similarly, impl sub/super types should be stored as `ITypeST` rather than read from the impl's
   definition-solve conclusions. This avoids impl placeholders leaking into the calling function's
   context — placeholders are denizen-scoped (like rustc's Param indices), so the calling function
   should never see them.

## Plan Details

**§3 and §6 split the rune set, not just the rules.** *(derived from §4 and §6)*

 * §4 does not know about `full_type_rune` or `type_outer_ref_rules`, so §3 must not put the wrap
   runes into the map §4 solves against. Otherwise §5's completeness check rejects the call over a
   rune §6 was going to conclude.
 * §6 runs its own `derive_rune_to_type` over the wrap rules (seeded with §3's rune-type results,
   since `value_type_rune`'s type is already known), and hands `full_type_rune` to `commit_step` in
   the same `new_runes` set as those rules.
 * Because §3 excluded the wrap runes, the solver is not "complete" from their perspective — adding
   `full_type_rune` via `commit_step` gives the solver new work and `incrementally_solve` proceeds
   normally. This is the same pattern as @DRSINI (adding work to a settled solve).

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

**§2's template extraction requires a rule-chain traversal (temporary).** *(derived from §2)*

 * ImplBoundS.super_rune is a kind rune (e.g. IObserver<U>), not a template (IObserver). Getting the
   template requires: super_rune → find CallSR whose result_rune matches → read template_rune → find
   LookupSR whose rune matches → read the name. Same indirection for the CallSR path.
 * This goes away when lowering-resolves-names lands (deferred, convo-38). At that point the template
   identity is available directly with no rule traversal.

**One type per file is load-bearing for synthesized functions.** *(derived from the `as`/`try_as`/`drop` migration)*

 * Two citizens in one file give two synthesized `as` (or `drop`) of the same arity in one env,
   which §1F cannot separate. `result.vale`'s `Ok` and `Err` want splitting for the same reason
   `opt.vale`'s `Some` and `None` do.

**§1F needs a flat-only env lookup.** *(derived from §1F)*

 * Today's `lookup_all_with_imprecise_name` walks `parent_env` up to the `PackageEnvironmentT`,
   which unions all global namespaces. So searching Ship's env finds every function in every package.
   §1F needs a lookup that reads only the citizen's own `TemplatasStore` without walking parents.

**Sends must target `value_type_rune`, not `full_type_rune`.** *(derived from §4)*

 * Today `assemble_initial_sends_from_args` sends to `param.full_type_rune`. §4 does not know about
   `full_type_rune`, so the send must target `value_type_rune` instead, and the sent value must be the
   peeled argument type (no `&`).

**§2 needs a read-only impl walk.** *(derived from §2)*

 * `is_parent` and `get_impl_parent_given_sub_citizen` both write to `CompilerOutputs`.
   `partial_resolve_impl` is the near-miss — same solve, no `check_resolving_conclusions_and_resolve`.
   It still takes `&mut` and could write if the impl carries Resolve rules (`where func` bounds).
   For the current corpus (Some/None/Ok/Err impls have no bounds) this is safe. For the general case,
   either assert no Resolve rules or build a genuinely read-only solve variant.

**§5 prerequisite: reference wrappers need env registration.** *(derived from §5)*

 * `BorrowRef` has no template ID and no `declare_type_outer_env` call. `get_outer_env_for_type`
   panics on it. §5's non-peeling search needs to look in `borrow.vale`'s env for `drop<T>(&T)`,
   which requires hardcoding BorrowRef's env the same way primitives and arrays need theirs.
   The "Move `drop<T>(&T)` to `borrow.vale`" migration item is a prerequisite for §5, not
   aspirational.

**§5 collects through its checks and registers once at the end.** *(derived from §5)*

 * The registration cannot be incremental, so every answer has to be in hand before the first write.
 * `check_resolving_conclusions_and_resolve` already has this shape and can be followed rather than
   redesigned.

**§7's upcast-through-wrap is already handled by `replace_value_type_in_ref`.** *(derived from §7)*

 * §2 computes upcasts on peeled value types (Some→Opt). §6 adds the reference wrap (&Opt). §7
   emits the instruction via `UpcastTE::new`, which calls `replace_value_type_in_ref` to walk through
   the wraps and swap the innermost citizen. `&Some<int>` → `&Opt<int>` in one call. No additional
   composition logic needed.

**How associated types would work without solving.** *(derived from §25, §26)*

Associated types are not in scope yet (they come with the trait system), but the architecture is
designed to handle them without a full solve. This section records the design so it isn't lost.

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
    sub:   ITypeST::Apply("Counter", [])
    super: ITypeST::Apply("Iterator", [])
    associated_types: { "Item" → ITypeST::Apply("int", []) }
```

When the associated type is generic — `impl<T> Iterator for Wrapper<T> { comptime Item = Pair<T>; }`
— the value is an `ITypeST` containing the impl's own runes:

```
impl<T> Iterator for Wrapper<T>:
    sub:   ITypeST::Apply("Wrapper", [ITypeST::Rune(T)])
    super: ITypeST::Apply("Iterator", [])
    associated_types: { "Item" → ITypeST::Apply("Pair", [ITypeST::Rune(T)]) }
```

**How §2B handles it.** The existing structural matching steps stay the same. One additional step
reads the associated type after the impl is selected:

 1. Walk impl's sub `ITypeST` against argument `KindT` → build impl rune map (e.g. `T → int`).
 2. Substitute into impl's super `ITypeST` → build super `KindT`.
 3. Assert no impl placeholders remain (§2.5.1).
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
on the impl, read and resolved through the same structural operations as everything else in §2B.

**Example with a generic associated type.** `sum(Wrapper<int>())`:

 1. Walk `Wrapper<int>` against impl's sub `Wrapper<T>` → map = `{ T → int }`.
 2. Substitute into super → `Iterator` (no impl runes in super).
 3. No unresolved placeholders. ✓
 4. Match `Iterator` against bound's `Iterator` → match.
 5. No conflicts.
 6. Read `Item` from impl: `ITypeST::Apply("Pair", [ITypeST::Rune(T)])`. Substitute `T → int` →
    build `KindT = Pair<int>`. Conclude `I = Pair<int>`.

§4 then has `I = Pair<int>` as an `InitialKnown`. If the function's body uses `I`, it's concrete.

**Why this doesn't need a solve.** The associated type value is declared on the impl — it is data,
not a derivation. Rust confirms this architecture: `TraitRef.args` never contains associated types,
and rustc processes associated type constraints through a separate pipeline
(`ProjectionPredicate` → `project_and_unify_term`) that finds the impl, reads the value, and checks
equality. Rustc's projection pipeline does call back into trait selection to find the impl, but §2B
has already found it, so the read is a field access.

**What WOULD need a solve.** If an associated type's value depended on another associated type from a
different trait (e.g. `comptime Item = Other.Output` where `Other` is itself a bound), resolving it
would require first resolving the other bound. That's a dependency chain between bounds, which could
require iterating. Vale's `comptime` design may or may not allow this — if it does, the resolution
order in §2B would need to handle dependencies between associated type reads across bounds. That is
genuinely harder than a single read, but it's still lookup-read-substitute applied iteratively, not
constraint solving.

**What this means for §25 and §26.** §25's structural matching handles impl selection. §26's
`ITypeST` provides the read-only structured form for both the impl's sub/super types and its
associated type values. Together they cover associated types with no new machinery — just one more
field on the impl's stored `ITypeST` data, and one more step in §2B's flow.

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

§2 finds CallSR on `value_type_rune` → template is `Opt`. Argument template is `Some`. Different.
Walk impl `impl<T> Some<T> for Opt<T>` from `Some<int>`, get `Opt<int>`. Hand `Opt<int>` to §4.
§7 emits an upcast `&Some<int>` → `&Opt<int>`.

### §2 with an ImplBoundS parameter — walk extracts conclusions, no upcast (§2)

```vale
impl IObserver<SignalA> for MyController;
func f<T, U>(x T) where implements(T, IObserver<U>) { ... }
f(MyController())
```

§2 finds ImplBoundS on `value_type_rune` → template is `IObserver`. Argument template is
`MyController`. Different. Walk impl, get `IObserver<SignalA>`. Hand that to §4 as an initial known
for the ImplBoundS's super_rune. Do NOT change the argument type — T stays MyController. §7 emits
no upcast.

§4 then has a CallSR rule: `Call(IObserver, [U]) → super_rune`. It already knows super_rune =
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

§2 finds ImplBoundS on `value_type_rune` → template is `IHandler`. Two impls from Button to
IHandler. Ambiguous.

But the user wrote explicit template args: U = ClickEvent. Each ImplBoundS carries its own rules:
`[Lookup("IEvent") → r46, Call(r46, [U]) → r45, Lookup("IHandler") → r44, Call(r44, [r45]) → super_rune]`.
§2 runs these seeded with U = ClickEvent. The solver propagates:
r45 = IEvent\<ClickEvent\>, super_rune = IHandler\<IEvent\<ClickEvent\>\>.

§2 searches for `impl IHandler<IEvent<ClickEvent>> for Button`. One impl. Done.

### §2 zero impls — early error (§2)

```vale
func f<T>(x Opt<T>) { ... }
f(Dog())
```

§2 finds CallSR → template is `Opt`. Argument template is `Dog`. Different. Search for impls from
Dog to Opt. Zero found. Compiler error, stop.

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
substituted parameter type's env. This is why `where func clone(&T)T` at T=Ship searches Ship's env
(not &Ship's env), finding `clone(&Ship) Ship` in ship.vale.

### §5 with `==` bound — referent's env has the function (§5)

```vale
func has<E>(arr &[]E, elem &E) bool where func ==(&E, &E)bool { ... }
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

- **Function bounds**, written `where func drop(T)void`. `opt.vale` declares
  `func drop<T>(opt Some<T>) where func drop(T)void`. Postparse lowers it to an `IRulexSR::Resolve`,
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
- **Numbering.** The preamble lists eight phases. Phase 1 is now split into sub-phases (§1B, §1F,
  §1H, §1L). The S1/S2 paragraph scheme from design-assistant is not yet in use, so Plan Details
  cites phase tags instead.
- **§2 ImplBoundS disambiguation with multiple impls.** When a struct implements one interface
  template twice and the parameter is a bare rune with a bound, the user must write an explicit
  template arg. §P's Preparation requires each bound to carry its own rules and runes, so §2 can
  run them seeded with explicit-arg conclusions from §1B.
- **`assume_most_specific_common_ancestor` contradicts the design.** The test
  (`compiler_solver_tests.rs:706`) asserts that `moo(Firefly(), Serenity())` upcasts both to IShip.
  The design says no common ancestor — the user writes `moo<IShip>(...)`. Test needs updating to
  expect an error, or rewriting with the explicit type argument.

### Out of scope but worth knowing

- **Let-binding upcasts are not covered by the phases.** `ship IShip = Raza(42)` goes through
  `infer_and_translate_pattern` → `convert()` → `convert_via_upcast`, not through §1B–§8.
  Already working since the `UpcastTE::new` fill. No action needed.

### Answerable from the code, unmeasured

- **Is `partial_resolve_impl` genuinely read-only?** It still takes `&mut CompilerOutputs`, and
  nothing has checked whether a rule reachable from its solve writes.
- **Does §6's rune-typing pass need `value_type_rune`'s already-derived type seeded?** A `BorrowRef`
  rule names it as its inner, so the second `derive_rune_to_type` may or may not reach it alone.
- **How many call sites in the suite have a parameter with an unsolved rune and an argument that
  needs an upcast to fill it?** That is the population §2's walk exists for, and it has never been
  counted.
