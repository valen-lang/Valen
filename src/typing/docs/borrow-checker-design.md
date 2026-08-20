# Borrow checker guidelines

This serves as the high-level design doc for the borrow checker.
**Keep this doc up to date.** It is a living doc.
If anyone needs to change this doc, that must be the *first* step of a plan, and must be explicitly approved by the architect.

## Context

 * The borrow checker is in src/typing/borrow_checker and src/typing/test/borrow_checker. Anyone working on only the borrow checker should stay within these directories.
 * `function_compiler_core.rs` after `coutputs.add_function` is the only place that can call into borrow_checker code, by calling `check_function`.
 * The only public method from the borrow_checker is `check_function`:
   ```
   pub fn check_function<'s, 'ctx, 't>(
     function: &'t FunctionDefinitionT<'s, 't>,
     function_s: &'s FunctionS<'s>,
     coutputs: &CompilerOutputs<'s, 't>,
     compiler: &Compiler<'s, 'ctx, 't>,
   ) -> Result<(), ICompileErrorT<'s, 't>> {
   ```
   **This function must stay pure (all immutable inputs, only error outputs).**
   The checker runs at the tail of each *user-body* typecheck, in `function_compiler_core.rs` right
after `coutputs.add_function`.

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

## Layers

Each file owns one job. Keep the jobs apart, and a change stays in one file.

 * `borrow_check.rs` walks the finished body and runs a check at each node it cares about. It owns traversal. It does not decide violations, and it does not build errors.
 * `place_path.rs` says where an argument points, as a root plus segments (`x`, then `.a`, then `[]`), and whether two paths overlap. It is plain data. It never takes a `Compiler` or `CompilerOutputs`.
 * `call_check.rs` decides whether one call is a violation. It reads groups and effects off the callee's `FunctionS` and asks `place_path.rs` about overlap. It does not walk the body, and it does not render error text.
 * `borrow_error.rs` holds the error kinds and renders them. Nothing else renders errors.
 * `borrow_check_types.rs` holds the checker's own types (`GroupB`, `EffectB`). Nothing here touches a `KindT`.

When liveness tracking arrives (rung 2), it gets its own file, not a corner of `call_check.rs`.

A new concept gets a group name (`GroupB`, "reach", "child group"), never a lifetime name (`Loan`, `Origin`, `Region`). A `Loan` type would teach the next reader that exclusivity applies here. It does not.


## Similar to Polonius

We should reuse some of Polonius's design.

However, polonius spends most of its code on two jobs we don't have, so we skip both.

Rust infers each reference's region; we declare groups, so there is nothing to solve. Rust enforces aliasing-xor-mutable; we allow aliased mutation, so there is no exclusivity to prove.

**Don't copy:**

 * **Region inference.** Rust invents a lifetime variable per borrow and solves a constraint graph to find it. About a third of `rustc_borrowck`. We are handed the group.
 * **The aliasing-xor-mutable conflict engine** (`check_access_for_conflict`). It rejects a read beside a live mutable borrow, a second mutable borrow, a write beside any borrow, and so on. We reject one case only: using a reference after a destroy reaches it. Keep that case, drop the rest.
 * **Two-phase borrows.** They exist only to let `vec.push(vec.len())` pass aliasing-xor-mutable. We allow it by default.
 * **CFG scaffolding** (`FakeRead`, `FalseEdge`, `FalseUnwind`, match fake-borrows). They patch a control-flow graph so it stays conservative. We walk the typed tree, so there is no graph to patch.
 * **Dropck and `#[may_dangle]`.** Our `dangle`/`opaque` effect plus poisoning cover the same ground.
 * **Lifetime-naming diagnostics** (inventing `'1`, printing "this borrow lives here because..."). Our groups already have names.

What we keep is small: a flow-sensitive liveness pass, a place-overlap check reduced to "does a destroy reach a live reference," and the "used after destroyed" rule.

Two things would drag region inference back, so avoid them: a return group the compiler must infer (Rust's `impl Trait`), and one closure invoked at many different groups.


## Testing

We have two kinds of tests:

 * Success tests, that expect a certain program to compile successfully.
    * Uses `utils.rs`'s `assert_compiles_clean(program)`.
 * Error tests, that expect the compiler correctly reported a compile error, and rendered it correctly.
    * Every error test should compare its output to a golden output.
    * Uses `utils.rs`'s `assert_borrow_error_renders(program, expected)`.

Tests must adhere to test-guidelines.md.


## Required Reading

 * test-guidelines.md.
