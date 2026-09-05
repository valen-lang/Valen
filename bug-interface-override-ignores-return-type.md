# Pre-existing Vale bug: interface override typechecking IGNORES the override's RETURN type

Received via mailbox from **Valen-exp-3-wipbx-ivory** (to: anyone), 2026-08-28T21:10:11.153Z.
A fork of the exp-3 session building the Rust-trait callback feature. Found while implementing
"a Vale struct implements a Rust trait"; it's a pre-existing, native-wide Vale bug you can pick
up independently of the interop work. Root-caused but NOT fixed. Repro reverted from the tree —
re-add from below.

## THE BUG

Vale's interface-override typechecking checks an override's NAME + RECEIVER + PARAMETER TYPES, but
NOT its RETURN TYPE. A struct can "implement" an interface method with the wrong return type and it
compiles.

## MINIMAL NATIVE REPRO (no Rust interop)

```vale
sealed interface Callback { func on_call(virtual self &Callback) int; }
struct MyCb { }
impl Callback for MyCb;
func on_call(self &MyCb) bool { return true; }   // returns bool, not int
exported func main() int { return 0; }
```

This SHOULD fail to compile but currently typechecks.

## ROOT CAUSE (confirmed with debug prints)

1. `SignatureT` — the identity used for override matching — is JUST the function's `IdT` (name +
   param types; params ride the name). It has NO return type.
   - `src/typing/ast/ast.rs:219-221`  (struct `SignatureT { id }`)
   - `src/typing/ast/ast.rs:431-433`  (`PrototypeT::to_signature()` = `SignatureT { id }` — drops return_type)
2. `edge_compiler::look_for_override` resolves the concrete override via
   `self.find_function(override_imprecise_name, override_function_param_types)` — the overload
   resolver, keyed on name + param types, NO expected return type. The resolved `override_prototype`
   (`found_function.prototype`, its `OverrideT.override_prototype`) carries whatever return type the
   override has, and nothing ever compares it to the abstract method's return type.
   - Confirmed print at the resolve site: "override on_call(&MyCb) resolved with ret Bool, but
     abstract method expects ret Int — NOT checked".
   - The later completeness check (`edge_compiler make_interface_edge_blueprints`,
     `internal_methods_set.difference(&functions_set)`) is ALSO return-blind — it compares
     `SignatureT` — but its dispatchers carry the abstract's return on both sides, so the override's
     real return only surfaces at the `find_function` site above.

## REPRO TEST (re-add; ivory reverted it)

Add to `src/typing/test/compiler_virtual_tests.rs` (mirror an existing test's boilerplate for
arenas/`compiler_test_compilation`). It must assert compile FAILS:

```rust
#[test]
fn native_interface_rejects_a_wrong_return_override() {
  // ... standard parse/scout/typing bump + arena setup ...
  let code = r"
sealed interface Callback { func on_call(virtual self &Callback) int; }
struct MyCb { }
impl Callback for MyCb;
func on_call(self &MyCb) bool { return true; }
exported func main() int { return 0; }
";
  // compile via compiler_test_compilation(...)
  assert!(compile.get_compiler_outputs().is_err(),
    "Vale accepted a wrong-return override (bool where the interface method returns int)");
}
```

Currently FAILS (compiles). That is the bug.

## WHY THE NAIVE FIX IS WRONG (ivory tried it — breaks 12 tests)

Adding `if found_function.prototype.return_type != abstract_function_prototype.return_type { err }`
right after `find_function` breaks ~12 existing tests (`arrays::ssamutfromcallable`,
`externs::simpleexternreturn`/`simpleexternparam`/`ssamutparamexport`/`structmutparamexport`,
`ifelse::ifnevers`, ...). At that site the abstract method's return type is in the INTERFACE'S
PLACEHOLDER space (e.g. `IFunction<P1,R>`'s `R`) while the override's is concrete — they legitimately
differ. Do NOT special-case generic vs non-generic (that principle is explicit: never treat
non-generics as a special case).

A correct fix must compare the override's return against the abstract's return UNDER THE IMPL'S
SUBSTITUTION (non-generic = the identity/degenerate case). The needed substitution is exactly the one
`find_function` computes when it unifies the override's params against the dispatcher-space param
types — BUT `find_function` discards it:
- `AttemptedCandidate` (`src/typing/overload_resolver.rs:255-257`) carries only `{ prototype }`.
- `look_for_override` sets `inferences: IndexMap::default()` (drops the conclusions).

So a correct fix likely needs ONE of:
1. Thread `find_function`'s inference conclusions out (extend `AttemptedCandidate` / the attempt
   path), then substitute `abstract_function_prototype.return_type` with them and compare to
   `found_function.prototype.return_type`.
2. Constrain the override lookup by the expected return type so `find_function` won't match a
   wrong-return override — then turn the currently `panic!("Unimplemented: CouldntFindOverrideT
   error")` path in `look_for_override` into a real error.
3. An existing "express the abstract signature in the case/override space" helper (ivory didn't
   find one).

This is the open design fork — pick a mechanism.

## SCAFFOLDING (ivory added then reverted; re-add as needed)

- `ICompileErrorT::OverrideReturnTypeMismatch { range, expected_return_type: KindT,
  actual_return_type: KindT }` in `compiler_error_reporter.rs` (+ its `range()` arm), modeled on
  `CantImplNonInterface`.
- A humanizer arm in `compiler_error_humanizer.rs` (model on `ConditionIsntBoolean`, which renders a
  `KindT`).

## SEPARATE NOTE (interop only; probably not your concern)

For a rust-backed SYNTHESIZED interface, this check wouldn't fire anyway: the synthesized (Extern)
interface's abstract method isn't force-compiled, so it never enters `get_all_functions()` as an
abstract interface method, and its edge blueprint has `headers=0` — `look_for_override` never runs
for it. So the interop callback's return-type check can't ride the edge machinery; rustc-on-the-stub
is the backstop there. That's a distinct issue from the native bug above.

-- Valen-exp-3-wipbx-ivory
