# Borrowing Design

## Design (human-only)

TODO to prepare:

 * (DONE) Rename LocationInFunctionEnvironment<'t> to Loc<'t>.
 * (DONE) Add Loc to `FunctionCallTE`, `WhileTE`, and `IfTE`.
 * (DONE) Make it so StructMemberT.name contains a MemberNameT, not an IVarNameT.
 * rename RegionS to GroupS
 * rename BorrowRefST.region to group
 * rename RegionGenericParameterType -> GroupGenericParameterType
 * rename RegionGenericParameterTypeS -> GroupGenericParameterTypeS
 * rename Consecutor to Sequence

Out of scope:

 * `rc` groups.
 * Return borrows without groups, like `func get(self &IndexMap<K, V>, key K) &V`. When the user writes that, we should give a compile error.
 * Fields that are borrow references, like `struct Moo<g'> { ship &Ship in g; }` or `struct Moo { ship &Ship; }`. When the user writes that, we should give a compile error.
 * Variables that shadow. If we detect this, panic. We don't do shadowing yet in Valen.

### Context

 * The borrow checker is in src/typing/borrow_checker and src/typing/test/borrow_checker.
 * `function_compiler_core.rs` after `coutputs.add_function` is the only place that can call into borrow_checker code, by calling `check_function`.
 * The only public method from the borrow_checker is `check_function`.

### Borrow Checking Happens After Typing (BCHATZ)

`BorrowRef` looks like this:
```
pub struct BorrowRefT<'s, 't> {
  pub inner: KindT<'s, 't>,
}
```
Note how it *doesn't* have a `group: GroupT`. That's because borrow checking is kept separate from type checking.

The borrow checker reads typing pass output, and consults the original postparsed AHT for any groups/annotations, such as `FunctionS`'s `effects` and `ParameterS`'s `tyype: ITypeST`.

`KindT` never contains anything about groups.

### check_function Has Two Phases

`check_function` has two phases.

 * It calls `groupify_function`, which makes an AST that has the "true types" of everything (types with groups).
 * It calls `check_usages`, which tracks what references are valid, and checks uses.

```rs
pub fn check_function<'s, 'ctx, 't, 'g>(
  &self, // Compiler
  coutputs: &CompilerOutputs<'s, 't>,
  function_s: &'s FunctionS<'s>,
  function_t: &'t FunctionDefinitionT<'s, 't>,
  check_arena: &'g Bump,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let body_g = self.groupify_function(coutputs, function_s, function_t, check_arena);
  self.check_usages(coutputs, &body_g)
}
```
**This function must stay pure (all immutable inputs, only error outputs).**

`check_function` and all the other things it calls will be methods on `Compiler`.

`check_arena` should be made by the caller in the core compiler. In future versions, we'll empty typing's temporary state arena and reuse it for this. Normal arena rules apply: no Vec in it, no Box in it, none of that, use TFITCX instead.

### groupify_function

`groupify_function` produces the new groupified body, an `IExpressionGE<'s, 't, 'g>`.

```rs
fn groupify_function<'s, 'ctx, 't, 'g>(
  &self,
  coutputs: &CompilerOutputs<'s, 't>,
  function_s: &'s FunctionS<'s>,
  function_t: &'t FunctionDefinitionT<'s, 't>,
  check_arena: &'g Bump,
) -> IExpressionGE<'s, 't, 'g>
```

Every borrow's group is derived during groupify from the expression that produces it. Where a group cannot be derived, the checker reports a compile error.

IExpressionGE is similar to IExpressionTE except it has its groups filled in, explained more below.

### G (Grouped) AST

There will be a G variant of most expressions and types.

There's no G variant of function definitions (FunctionT) and type definitions (StructDefinitionT etc.).

The borrow checker doesn't do any interning, it compares all things by equality deeply. This shouldn't be so bad because:

 * They'll usually bottom out in typing pass's outputs which are interned and those comparisons are cheap.
 * It should all be hot in cache, in the 'g arena.

#### IExpressionGE

IExpressionGE is similar to IExpressionTE except it has its groups filled in:

 * Every expressions' result() returns a `KindGT`.
 * Every BorrowRefGT contains a `group: GroupExprG` of where it's borrowing from.
 * Every FunctionCallGE has a `mut_effects: &'g [&'g MutEffectPath]` of what groups it's mutating.
 * Every WhileGE has a `mut_effects: &'g [&'g MutEffectPath]` of what groups were mutated inside it.
    * If a `WhileGE` contains another `WhileGE`, the outer one also contains all of the inner one's mut_effects.

For example, if we have:
```
func foo<l'>(level &Level in l, tile in l.tiles) {
  while true { // Loc: 0,2,1
    print(tile.mana);
    level.tiles.clear(); // Loc: 0,2,1,2,2
  }
}
```

Then `groupify_function` should return an IExpressionGE that looks like IExpressionTE, except:

 * `level`'s type is BorrowRefGT{inner: StructGT(Level's IdT, []), group: GroupExprG::Rune(l)}
 * `tile`'s type is BorrowRefGT{inner: StructGT(Tile's IdT, []), group: Elements(Member(Rune(l), tiles))}
 * the `clear` call has a `mut_effects = [[Local("level"), Member("tiles")]]` ("`mut`ated level.tiles")
 * `while` has a `mut_effects: [MutEffectPath([0,2,1,2,2], [Local("level"), Member("tiles")])]` ("a call at 0,2,1,2,2 `mut`ated level.tiles")

All IExpressionGE variants hold expression structs, just like IExpressionTE. Each GE expression struct must have a field `result: KindGT`.

#### MutEffectPath

```rs
// A specific mutation to a specific group (as opposed to GroupExprG which an expression for expressing the group(s) a ref might point at).
struct MutEffectPath<'g> {
  effecting_node_loc: Loc, // Which expr had this mut effect (e.g. loc of `level.tiles.clear()`)
  steps: &'g [&'s GroupStep<'s>], // What group the effect mutated (e.g. ["level", "tiles"])
}
enum GroupStep<'s> {
  Rune(&'s IRuneS), // a group param, e.g. <g'>, resolved to its id
  ParamAnonymousGroup(&'s StrI<'s>), // A param's group if it doesn't come from a rune or another param. The string is the param name
  Local(&'s StrI), // A local's implicitly declared group.
  Member { member_name: &'s StrI<'s> }, // `x.items`
  Elements, // the `[]` part of `x.items[]`
  // No `Empty` variant, that just becomes not a MutEffectPath at all.
  // No `Union` variant, that just becomes multiple MutEffectPath.
}
```

Note: future version should explore making Local contain an actual IVarNameT, not sure why we dont.

#### KindGT and ITemplataG

The typing pass never sees groups. So in a way, the typing pass never sees a thing's _true_ type, because the groups have been erased from the typing pass. However, in the borrow checker, we truly do need to see the true type.

That "true type" is `KindGT` (and `KindTemplataG`).

`KindGT` is shaped exactly like `KindT`, except:
 * Its `BorrowRefGT` also contains a `GroupExprG`.
 * It can use T names (IVarNameT, etc.) and T-flavored IDs or expressions when it knows no groups will be in them.
 * Template args are not stored in IDs.

KindGT looks like this:
```rs
pub enum KindGT<'s, 't, 'g> {
  Struct(StructGT<'s, 't, 'g>),
  Interface(InterfaceGT<'s, 't, 'g>),
  StaticSizedArray(StaticSizedArrayGT<'s, 't, 'g>),
  RuntimeSizedArray(RuntimeSizedArrayGT<'s, 't, 'g>),
  BorrowRef(BorrowRefGT<'s, 't, 'g>),
  OwnRef(OwnRefGT<'s, 't, 'g>),
  ShareRef(ShareRefGT<'s, 't, 'g>),
  WeakRef(WeakRefGT<'s, 't, 'g>),
  // These contain nothing interesting to the borrow checker:
  Void(VoidT),
  Int(IntT),
  Bool(BoolT),
  Str(StrT),
  Float(FloatT),
  USize(USizeT),
  Never(NeverT),
  OverloadSet(&'t OverloadSetT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
}
```
BorrowRefGT is the interesting one, because it's the only place that has a `GroupExprG`:
```rs
pub struct BorrowRefGT<'s, 't, 'g> {
  pub group: GroupExprG<'s>,
  pub inner: &'g KindGT<'s, 't, 'g>,
}
```

`ITemplataG` mirrors `ITemplataT` but with groups and group-annotated types. It looks like this:
```rs
pub enum ITemplataG<'s, 't> {
  Kind(KindTemplataG<'s, 't>), // Contains a KindGT
  Group(GroupExprG<'s>),
  // The below ones don't have anything interesting for the borrow checker.
  Integer(i64),
  Boolean(bool),
  String(StrI<'s>),
  Prototype(&'t PrototypeTemplataT<'s, 't>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT),
  Function(&'t FunctionTemplataT<'s, 't>),
  StructDefinition(&'t StructDefinitionTemplataT<'s, 't>),
  InterfaceDefinition(&'t InterfaceDefinitionTemplataT<'s, 't>),
  ImplDefinition(&'t ImplDefinitionTemplataT<'s, 't>),
  ExternFunction(&'t ExternFunctionTemplataT<'s, 't>),
  // These two are only ever created by the typing solver, which doesn't handle group information
  Isa(&'t IsaTemplataT<'s, 't>),
  CoordList(&'t KindListTemplataT<'s, 't>),
  // It's weird that this one doesnt have anything interesting to the borrow checker
  Placeholder(&'t PlaceholderTemplataT<'s, 't>),
}
```

As you can see, it's really only types that contain groups. And types that contain types, that contain groups.

The things from typing pass that never even carry a GroupTemplataT don't even need corresponding KindGT/ITemplataG things.

The G AST doesn't store template args in the name, it stores them next to the old IdT:
```rs
pub struct StructGT<'s, 't, 'g> {
  pub id: IdT<'s, 't>,
  pub template_args: &'g [&'g ITemplataG<'s, 't>],
}
```


#### `make_kind_g` / `make_templata_g`

groupify_function calls these two functions to make the above grouped AST.

We can make a `KindGT`/`ITemplataG` via `make_kind_g`/`make_templata_g`. `make_kind_g` takes in the typing-pass type, and the original postparsed `ITypeST` (because it still has group annotations like the `in g` in `&Ship in g`), and mashes those together (with knowledge of the groups in the local scope) to make the true type `KindGT`. Same with `make_templata_g`.

`make_kind_g` looks like:

```rs
pub fn make_kind_g(
  &self,
  kind: KindT<'s, 't>,
  tyype: &'s ITypeST<'s>,
  param_name: Option<StrI<'s>>,
) -> KindGT<'s, 't, 'g> { ... }
```

`param_name` is the surrounding parameter, if we're in one. Useful for interpreting `ship: &Ship` as `ship: &Ship in anonymous_ship_group`.

`make_templata_g` looks like:

```rs
fn make_templata_g(
  &self,
  templata: ITemplataT<'s, 't>,
  written: Option<&'s ITypeST<'s>>,
  param_name: Option<StrI<'s>>,
) -> ITemplataG<'s, 't> {
```

#### GroupExprG

As it recurses through the function, it tracks what the "actual types" are. Here, they're `KindGT` instead `KindT`. `KindGT` is generally shaped like `KindT` except its BorrowRefT also contains a `GroupExprG`.

`GroupExprG` looks like this:
```rs
// An expression for expressing the group(s) a function might mutate or a ref might point at (as opposed to GroupStep which is a specific mutation to a specific group).
enum GroupExprG<'s, 't, 'g> {
  Rune(&'s IRuneS), // a group param, e.g. <g'>, resolved to its id
  ParamAnonymousGroup(&'t IVarNameT<'s, 't>), // A param's group if it doesn't come from a rune or another param. The StrI is the parameter's name
  Local(&'t IVarNameT<'s, 't>), // A local's implicitly declared group.
  Member { base: &'g GroupExprG<'s, 't, 'g>, member_name: StrI<'s> }, // `x.items`
  Elements { base: &'g GroupExprG<'s, 't, 'g> }, // the `[]` part of `x.items[]`
  Union { members: &'g [&'g GroupExprG<'s, 't, 'g>] }, // This ref points at multiple groups, or this function mutates multiple groups
  Ellipsis { base: &'g GroupExprG<'s, 't, 'g> }, // the `...` part of `x...`
}
```

Notes:

 * In the future, we'll have a GroupExprG::Empty, but we don't have one yet. We'll add it much later, when we want to support empty groups in generic arguments and associated types. We shouldn't add it until then because AI keeps using it as a hack to get around requirements.
    * There is no such thing as a groupless borrow. After groupify_function, **every single borrow ref should have a group**. No empty groups.
    * ParamAnonymousGroup is **only** to be used for the surface-most borrow in a function signature. Okay: `x: &Ship` -> `x: &Ship in ParamAnonymousGroup(x)`. Bad: `y: &Opt<&Ship> in a` -> `y: &Opt<&Ship in ParamAnonymousGroup(x)> in a`.
 * A `GroupExprG`'s runes are always in the current function's (the caller's) namespace.
 * `map`'s GroupSubtree is different than `map.size`'s GroupSubtree. However, in the code that detects a mutation to `map`, we'll make sure it doesn't invalidate references to `map.size`, because `map.size` isn't destructible independently from `map`.

Future version should explore making Local contain an actual IVarNameT, not sure why we dont.

#### groupify_function

Putting it all together, groupify_function walks the typed body once and produces the mirrored IExpressionGE, filling in group information:

 * Every expression's result KindGT,
 * Every borrow's GroupExprG,
 * Every FunctionCallGE's mut_effects
 * The mut_effects aggregated onto each WhileGE.

For every expression, it figures out the result type of it. Examples:
 * It figures out the result of a `items[0]` indexing expression, by getting the element type of the `items` array type.
 * It figures out the return value of a FunctionCallGE node, by looking at the callee and doing the substitutions.

### check_usages

```rs
fn check_usages<'s, 'ctx, 't, 'g>(
  &self,
  coutputs: &CompilerOutputs<'s, 't>,
  function_g_body: &'g IExpressionGE<'s, 't, 'g>,
) -> Result<(), ICompileErrorT<'s, 't>>
```

The checking phase uses that, and tracks what variables are live with these structs:

```rs
struct LocalEntry {
  invalidated_by: Option<Loc>;
}

// A subtree for a group as the containing function knows it. This grows over time as the function learns about new groups.
struct GroupSubtree {
  // enum RefKey { Named(IVarNameT<'s, 't>), Held(u32), }
  locals: IndexMap<RefKey, LocalEntry>;

  // The locals pointing at an ellipsis inside a certain group.
  // For example, in this function:
  //     func foo(vec &Vec<Ship>) {
  //       first_ref &Ship in vec... = vec[0];
  //       vec.append(Ship(42));
  //       print(first_ref.hp);
  //     }
  // At the start we'll just have GroupSubtree{[{vec,None}],[],[]}.
  // After `first_ref =` we'll have GroupSubtree{[{vec,None}],[{first_ref,None}],[]}
  // After `vec.append` we'll have GroupSubtree{[{vec,None}],[{first_ref,Some(...)}],[]}
  //
  // There's no such thing as a child of an ellipsis; doing `&x.hp` on a `&Ship in g...` produces a `&i32 in g...`.
  locals_in_ellipsis: IndexMap<RefKey, LocalEntry>;

  name_to_child: IndexMap<GroupStep, GroupSubtree>;
}
```

`check_usages` does two things:

 * Builds out the GroupSubtree tree as it discovers more locals and held registers.
 * Invalidates entries in the tree as it discovers mut effects.

#### Local/Register Discovery

As it encounters a local or a "held register" that contains a reference, we'll register it into the `GroupSubtree` tree as still live. A held register is one that's waiting to be passed to a function or another expression, for example if we say `foo(bar(x), baz(y))`, `bar(x)` will be in a register while `baz(y)` is evaluating. Treat it like a temporary unnamed local. Then, when the call finally happens, we'll do a final usage check on those references.

As it encounters a usage of a reference, whose `BorrowRefGT` will have a `GroupExprG` which we'll use to query the `GroupSubtree` tree to see if anywhere it's pointing at has been invalidated.

#### Invalidating (Churning)

(To "churn" a thing means to invalidate all references into that thing's descendant groups.)

As `check_usages` encounters `MutateGE`s and `FunctionCallGE`s which have `mut_effects: &'g [&'g MutEffectPath]`, it will flatten each `MutEffectPath` into lookups into that `GroupSubtree`.

If it was a `mut(g)` effect (no ellipsis), then:

 * For every `Elements` descendant of that path, and all descendants of those `Elements` descendants, fill the `invalidated_by`.
 * For each entry in this group's `locals_in_ellipsis`, fill the `invalidated_by`.
 * For each descendant group, for each of their `locals_in_ellipsis`, fill the `invalidated_by`.
 * For each ancestor group, for each of their `locals_in_ellipsis`, fill the `invalidated_by`.

Either way, any reference to the churned group itself survives. Example: `inv(arr)` moves the buffer, so `&arr[0]` dies but `&arr` is fine.

An example:
```
struct Inventory { items Vec<Item>; }
func first<g'>(inv &Inventory in g) &Item in g.items... { ... }
func restock<g'>(inv &Inventory in g) mut(g) { ... }
func main() {
  inv Inventory = ...;
  it &Item in inv.items... = first(&inv);
  restock(&inv); // Churns `inv`, _doesn't_ invalidate refs to inv, _does_ invalidate refs to `inv...`, `inv.items[]...`, etc.
  print(it.name);
}
```

If they mutate a group `g...`, that's the same as mutating group `g`.

Notes:

 * A churn of a group invalidates any references into that group's independently-destructible descendant groups. And the only independently-destructible child group in Valen is the Elements (`[]`) child group. Therefore, a churn of a group invalidates any references into that group's descendant Elements groups, and all of those groups descendants.
    * When unions arrive, their variants will also be in an Elements child group. When raw pointers arrive (like underlying `Box` and `Vec`), those too will be Elements child groups.

#### Check Argument Overlaps

`check_usages` also ensures that when we're calling a function, we don't borrow something and move it at the same time.

For example, it should reject this:

```
func main() {
  a Vec<Ship> = ...;
  do_something(^a, &a);
}
```

#### Check Group Aliasing

`check_usages` also ensures that the callsite doesn't supply arguments that make two callee groups alias when the callee doesn't expect it.

For example, it should reject this:

```
// grow treats r and s as disjoint: it mutates r while b holds a borrow into s.
func grow(a &Vec<int>, b &Vec<int>) mut(a) { ... }
// Desugared for clarity: func grow<r', s'>(a &Vec<int> in r, b &Vec<int> in s) mut(r) { ... }

func main() {
  v Vec<int> = ...;
  grow(&v, &v); // Reject because &v sends into both r and s, which violates grow's assumption that "r and s are disjoint"
}
```

But it should allow this, because the callee explicitly lets both arguments point into a single group `g'`:

```
struct Entity { hp int; }
func heal<g'>(a &Entity in g, d &Entity in g) mut(g) { }
func main() {
  e = Entity(5);
  heal(&e, &e);
}
```

## Design Proposals

**Group-generic closures.** A closure that captures a reference is generic over the groups its captures
need: for each capture, the closure struct gains a group parameter per free group in that capture's type
(found by walking the type, not its definition) plus a fresh outer group for a by-reference capture,
each bound at `&{...}` construction to the enclosing group. The closure body reads them off `self`'s
type, so a captured reference's use is checked like any group-generic call — no cross-function body
peek. Detailed plan: `docs/plans/group-generic-closures-plan.md`.

## Details

### Phase entry points (from Two Phases)

`check_function` runs the two phases in order, threading phase 1's output into phase 2. All inputs
stay immutable and the only output is an error, so the entry point stays pure.

Phase 1 (`groupify_function`) builds the grouped AST: it fills each borrow's group and attaches each
call's `mut_effects`, aggregating them onto the enclosing `while` node, so phase 2 needs no loop
fixpoint.

Phase 2 (`check_usages`) walks the grouped AST once, threads the `GroupSubtree` tree, and rejects a use
of a reference a churn invalidated.

### The grouped AST meets the GroupSubtree state (from check_usages)

The grouped AST and the invalidation state touch at three points and nowhere else:

 * Bind: register a reference as live under each group its `GroupExprG` names — a `let`-bound local,
   or a held register (a temporary holding a reference mid-expression, like `bar(x)` in
   `foo(bar(x), baz(y))`, treated as an unnamed local).
 * Churn: walk to the churned group's node, cross its child-group edges, and stamp `invalidated_by =
   <the call's Loc>` on every registered reference beneath.
 * Use: query the tree at the reference's group(s) — for a local at its use, for a held register when
   the call consumes it; if any group it points into is invalidated, it is a use-after-churn.

A churn reaches group→its registered references; a use reaches a reference→every group it points
into, so a `Union` reference is checked against all of them.

### Diagnostics (from check_usages)

A use-after-churn of a *named* reference renders as `BorrowErrorKind::UseAfterChurn`, pointing at the
use site. A use-after-churn of a *held register* — an unnamed mid-expression temporary — renders as a
distinct `BorrowErrorKind::UseAfterChurnTemporary`, pointing at the argument that holds the stale
reference; the per-argument source range comes from `FunctionCallTE.range`.

## Test cases

### Array-element churn (rung 2)

```
let arr = [...];     // arr is its own group
let elem = &arr[i];  // elem: a reference into arr's child group (the elements)
churn(arr);          // churn declares mut(g) on its parameter's group
```

`groupify_function` gives the `churn` call's `mut_effects` a `MutEffectPath { effecting_node_loc:
<the churn call>, steps: [Local("arr")] }` — the leading `Local("arr")` names the root. `check_usages`
walks to the `GroupSubtree` at `arr`, invalidates every `LocalEntry` under its child groups (`elem`) with
`invalidated_by = effecting_node_loc`, and rejects a later use of `elem` as use-after-churn. A reference to `arr` itself,
or to an inline member, is in `arr`'s own `locals` and survives.

### Use-after-churn through a returned reference (rung 3)

```
let v = map.get(k);   // get returns a reference into an element of self's group
map.remove(k);        // remove churns map (mut on self's group)
print(v);             // stale — use-after-churn
```

`groupify_function` reads `get`'s declared return group (a reference into `self`'s group's elements),
substitutes `self`'s group rune with the `map` argument, and gives `v`'s `BorrowRefGT` the group
`Elements(Local("map"))`. `check_usages` registers `v` under `map`'s elements at the binding; the
`remove` call churns `map`, invalidating the references under its child groups (`v`); and `print(v)`
is rejected as use-after-churn.

### Held register: use-after-churn through an unnamed call result

```
arr Vec<int> = ...;
use2(get(&arr), churn(&arr));   // get(&arr) returns a reference into arr, held in a register;
                                // churn(&arr) then churns arr; use2 consumes the stale reference
```

Unlike the returned-reference case above, `get(&arr)`'s result is never bound to a named local — it
lives in a register while the sibling argument `churn(&arr)` evaluates. `groupify_function` registers
that held register as an unnamed local pointing into `arr`'s elements; `check_usages` invalidates it
when `churn(&arr)` churns `arr`; and the final `use2` call that consumes the register is rejected as
use-after-churn. A test using a *named* local would not exercise this — the register must be an
unnamed temporary.

## Background

### Self-evident from the code

 * A local's identity is the interned `IVarNameT` (`names.rs`), which the checker uses as the `RefKey::Named` key for a live reference in the `GroupSubtree`; its per-function uniqueness comes from the embedded `LocalNameT.life` (a unique `path: &[i32]` per declaration), so it is safe under shadowing.
 * `FunctionCallTE`, `WhileTE`, and `IfTE` (`expressions.rs`) each carry a `loct: LocT<'t>` field, so the walk reads a `Loc` off the node.
 * `IVarNameT` (`names.rs`) is interned and `Copy`/`Eq`/`Hash`, so it is a hashable local key needing no pointer.
 * `LocT<'t> { path: &[i32] }` (`ast.rs`) is the `Loc`; the same type fills `MutEffectPath.effecting_node_loc` and `LocalEntry.invalidated_by`.
 * `LocalVariable` (`function_environment_t.rs`) uses arena-pointer identity (@IEOIBZ), which the current `liveness.rs` keys on and the new checker drops in favor of `IVarNameT`.
 * The pipeline already keys locals by name, not pointer: testvm's `VariableAddressV { call_id, name: IVarNameI }` (`values.rs`) does, and its comment records that the typing pass makes the name unique per function (@VCOORD) while the per-mention-reallocated struct pointer is not a stable key.
 * `StructMemberT.name` (`citizens.rs`) is now a `&'t MemberNameT` (`names.rs`, carrying `imprecise_name` + `life`), while the body node the walk actually reads, `MemberLookupTE.member_name` (`expressions.rs`), is still an `IVarNameT` (a `MemberNameT` wrapped as `IVarNameT::Member`). The two are populated independently (e.g. a closure capture at `expression_compiler.rs`), so a member step read off the body node must project the `MemberNameT` out.

### Documented

 * Groups never live on the value type, because a `KindT`'s structural `Eq`/`Hash` is monomorphization
   identity, so a group on `BorrowRefT` would split `Vec<int> in a` from `Vec<int> in b` into two
   monomorphizations — `docs/plans/path-to-borrowing.md`, §"The group representation". `KindGT` sidesteps
   this by being borrow-checker-only.

### Undocumented

## Open Questions

## Required Reading

 * design-assistant
