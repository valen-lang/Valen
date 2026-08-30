# Borrowing Design

## Design (human-only)

TODO to prepare:
 * (DONE) Rename LocationInFunctionEnvironment<'t> to Loc<'t>.
 * (DONE) Add Loc to `FunctionCallTE`, `WhileTE`, and `IfTE`.
 * (DONE) Make it so StructMemberT.name contains a MemberNameT, not an IVarNameT.

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

 * It calls `loop_scout_phase`, which notes what effects happen to loops.
 * It calls `check_refs`, which tracks what references are valid, and checks uses.

The loop-scout phase produces a `loop_lif_to_mut_effects: HashMap<Loc, Vector<MutEffectPath>>`.

```rs
pub fn check_function<'s, 'ctx, 't>(
  &self,
  coutputs: &CompilerOutputs<'s, 't>,
  function: &'t FunctionDefinitionT<'s, 't>,
  function_s: &'s FunctionS<'s>,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let loop_lif_to_mut_effects = loop_scout_phase(function, coutputs, compiler);
  check_refs(function, function_s, &loop_lif_to_mut_effects, coutputs, compiler)
}
```
**This function must stay pure (all immutable inputs, only error outputs).**

`check_functions` and all the other things it calls will be methods on `Compiler`.

### loop_scout_phase

```rs
fn loop_scout_phase<'s, 'ctx, 't>(
  &self,
  coutputs: &CompilerOutputs<'s, 't>,
  function: &'t FunctionDefinitionT<'s, 't>,
) -> HashMap<Loc<'t>, Vec<MutEffectPath<'s, 't>>>
```

Produces `loop_lif_to_mut_effects: HashMap<Loc, Vector<MutEffectPath>>`.

```rs
struct MutEffectPath {
  effecting_node_loc: Loc, // Which expr had this mut effect (e.g. loc of `level.tiles.clear()`)
  steps: Vec<GroupNameStep>, // What group the effect mutated (e.g. ["level", "tiles"])
}
enum GroupNameStep {
  Elements,
  Local(name: IVarNameT),
  Member(name: MemberNameT),
}
```

For example, if we have:
```
func foo<l'>(level &Level in l, tile in l.tiles) {
  while true { // Loc: 0,2,1
    print(tile.mana);
    level.tiles.clear(); // Loc: 0,2,1,2,2
  }
}
```
Then `loop_scout_phase` should return:
 - [0,2,1] -> [MutEffectPath([0,2,1,2,2], [GroupNameStep::Local("level"), GroupNameStep::Member("tiles")])]

### check_refs

```rs
fn check_refs<'s, 'ctx, 't>(
  &self,
  coutputs: &CompilerOutputs<'s, 't>,
  function: &'t FunctionDefinitionT<'s, 't>,
  function_s: &'s FunctionS<'s>,
  loop_lif_to_mut_effects: &HashMap<Loc<'t>, Vec<MutEffectPath<'s, 't>>>,
) -> Result<(), ICompileErrorT<'s, 't>>
```

The checking phase uses that, and tracks what variables are live with these structs:

```rs
struct LocalEntry {
  local: IVarNameT;
  invalidated_by: Option<Loc>;
}
struct LiveGroup {
  locals: HashSet<LocalEntry>;
  name_to_child: HashMap<GroupNameStep, LiveGroup>;
}
```


## Design Proposals

S1. The borrow checker's only data structures are `LiveGroup`, `LocalEntry`, and
`loop_lif_to_mut_effects`. It is a full replacement of the old checker, which is deleted rather than
adapted.

## Details

### Phase entry points (from Two Phases)

`check_function` runs the two phases in order, threading phase 1's output into phase 2. All inputs
stay immutable and the only output is an error, so the entry point stays pure.

Phase 1 walks the body and buckets every churn under its enclosing loop, so phase 2 needs no loop
fixpoint.

Phase 2 walks the body once, tracks live references in the `LiveGroup` tree, and rejects a use of a
reference a churn invalidated.

## Test cases

### Array-element churn (rung 2)

```
let arr = [...];     // arr is its own group
let elem = &arr[i];  // elem: a reference into arr's child group (the elements)
churn(arr);          // churn declares mut(g) on its parameter's group
```

`loop_scout_phase` records `MutEffectPath { effecting_node_loc: <the churn call>, steps:
[Local("arr")] }` — the leading `Local("arr")` names the root. `check_refs` walks to the `LiveGroup`
at `arr`, invalidates every `LocalEntry` under its child groups (`elem`) with `invalidated_by =
effecting_node_loc`, and rejects a later use of `elem` as use-after-churn. A reference to `arr` itself,
or to an inline member, is in `arr`'s own `locals` and survives.

## Background

### Self-evident from the code

 * A local's identity is the interned `IVarNameT` (`names.rs`) that the checker keys `LocalEntry` on; its per-function uniqueness comes from the embedded `LocalNameT.life` (a unique `path: &[i32]` per declaration), so it is safe under shadowing.
 * `FunctionCallTE`, `WhileTE`, and `IfTE` (`expressions.rs`) carry no location field and no `RangeS` today; the Design TODO adds `Loc` to them so the walk can read one off the node.
 * `IVarNameT` (`names.rs`) is interned and `Copy`/`Eq`/`Hash`, so it is a hashable local key needing no pointer.
 * `LocationInFunctionEnvironmentT { path: &[i32] }` (`ast.rs`) is the `Loc` (rename pending per the Design TODO); the same type keys `loop_lif_to_mut_effects` and `LocalEntry.invalidated_by`.
 * `LocalVariable` (`function_environment_t.rs`) uses arena-pointer identity (@IEOIBZ), which the current `liveness.rs` keys on and the new checker drops in favor of `IVarNameT`.
 * The pipeline already keys locals by name, not pointer: testvm's `VariableAddressV { call_id, name: IVarNameI }` (`values.rs`) does, and its comment records that the typing pass makes the name unique per function (@VCOORD) while the per-mention-reallocated struct pointer is not a stable key.
 * Member names are `IVarNameT` today on both the definition (`StructMemberT.name`, `citizens.rs`) and the body node the walk reads (`MemberLookupTE.member_name`, `expressions.rs`), each wrapping a `MemberNameT` (`names.rs`, `imprecise_name` + `life`) as `IVarNameT::Member`. The two fields are populated independently (e.g. a closure capture at `expression_compiler.rs`), so the Design TODO's `StructMemberT.name` change does not by itself reach `MemberLookupTE.member_name`.

### Documented

### Undocumented

## Open Questions

## Required Reading

 * design-assistant
