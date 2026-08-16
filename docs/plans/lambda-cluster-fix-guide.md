# Lambda-cluster fix guide

Greening the ~17 lambda/closure failures by recovering the pre-onion working code and translating
it to the onion `KindT` model. Derived from git archaeology of the last green pre-onion commit.

The lambda failures span three interlocking stubs today:
- `rune_type_solver.rs:692` — "LookupSR pre-computation error path not yet implemented" (~10 tests).
- `expression_compiler.rs:284` — closure-capture construct `unimplemented!()` (~4 tests).
- `call_compiler.rs:222` / `overload_resolver.rs` — `CouldntFindFunctionToCallT`, curried/reused
  lambdas (~3 tests).

Most of this is *un-comment-and-translate* from a working predecessor; only the capture read/write is
a genuine (but templated) re-implementation. Current-tree `file:line`s drift; the pre-onion commit
hashes are stable.

## Recovery mechanics

- **Last green pre-onion commit: `f1dc30f48`** ("Phase 2 partial landing", suite 1111/0/95). Every
  function below exists there in working form. The pre-onion Rust is the best recovery source — it was
  a faithful, green port.
- Files moved during the refactor. Recover bodies with (note the `./` — the git root is one level
  above `src/`):
  - `git show f1dc30f48:./src/typing/expression/local_helper.rs`
  - `git show f1dc30f48:./src/typing/expression/expression_compiler.rs`
  - `git show f1dc30f48:./src/typing/overload_resolver.rs`
  - `git show f1dc30f48:./src/typing/templata_compiler.rs`
  - `git show f1dc30f48:./src/postparsing/rune_type_solver.rs` (pre-onion the typing-side solver lived
    in `postparsing`; today it is `src/typing/rune_typing/rune_type_solver.rs`)
- The onion cascade begins at `e82f77576` and runs through `5d2650ac9` (convert() rewritten off the
  ownership axis, `SoftLoadTE`/`AliasTE` retired), `07a792c9a` ("a mention already IS a borrow"),
  `af3a3c17a`, `6c859ed12` (sends reach the solve), `778feb7b3` (rune solver renamed
  `solve_rune_types`).
- The Scala tree is deleted; Scala bodies survive only as inlined `//` comments at `88e91c1ef`
  ("Archival Scala-body inlining sweep"). Each stub already carries its Scala one-liner in a trailing
  `// return Err(...)` comment. Prefer the pre-onion Rust at `f1dc30f48`.

## The model change every translation hinges on

| Pre-onion (two-axis) | Onion (one-axis) |
|---|---|
| `CoordT { ownership: OwnershipT, region, kind: KindT }` | ref folded into `KindT::{BorrowRef,OwnRef,ShareRef,WeakRef}(&Inner{inner,region})` (`types.rs:52-69`) |
| `IVariableT`: 4 variants (`Addressible/Reference × Local/Closure`) + `ILocalVariableT` | `IVariableT`: **2** variants `Local(LocalVariable{name, tyype:KindT})` / `Capture(CapturedVariableT{name, closured_vars_struct_type:&StructTT, kind:KindT})`. The Addressible/Reference split and `ILocalVariableT` are gone. |
| `StructMemberT` with `IMemberTypeT::Reference/Address` | flat `StructMemberT{name, tyype:KindT}` — `tyype` already carries its ref-ness |
| `SoftLoadTE`, `AliasTE` | **retired — no longer exist** |
| lookups bare; ownership decided by `soft_load` | `LocalLookupTE`/`*MemberLookupTE::new` **auto-wrap** `result = BorrowRef(inner)`; `DerefTE::new` peels one ref for the `&&→&` decay |
| helpers: `get_sharedness`, `substitute_for_coord`, `get_borrow_ownership`, `borrow_soft_load`, `soft_load`, `maybe_borrow_soft_load`, `determine_if_local_is_addressible`, `pointify_kind` | retired. New helpers: `peel_one_reference` (`templata_compiler.rs:75`), `peel_all_references` (`:87`), `replace_value_type_in_ref` (`:102`), `substitute_for_kind` (`:926`) |

Rule of thumb: **salvage the pre-onion logic, re-express the representation.** A captured own is now a
`BorrowRef`/`ShareRef`/`WeakRef` *inside* the kind; sharedness (Share vs Borrow) is folded away and
decided target-side in `convert()`.

## Corrected premise: "SelfName" is a non-issue

An earlier analysis claimed the lambda blocker is resolving `SelfName → KindTemplataType` in
`TemplataCompilerRuneTypeSolverEnv::lookup`. It is not. A lambda's self param is a `LookupSR` on
`LambdaStructImpreciseName` (synthesized in `function_scout.rs::create_closure_param`), not `SelfName`,
and the lookup **already** has the arm returning `KindTemplataType` for it
(`templata_compiler.rs:1144-1152`) — identical to the pre-onion code. `SelfName` is produced only for
citizens, via a path lambdas never take. There is nothing to recover there. (There is a standing
`// VCOORD: remove this entire branch and see if it just works` at `templata_compiler.rs:1143` worth
trying once capture is filled, but it is not a blocker.) **The real lambda work is closure capture.**

## Fix items

### 1. `get_param_environments` — peel refs before matching (trivial)

`overload_resolver.rs` (~`:511`). Pre-onion (`f1dc30f48:overload_resolver.rs:518-532`) matched
`tyype.kind`, i.e. the ownership axis was already stripped. Today `param_filters: &[KindT]`, and a
borrowed closure arrives as `KindT::BorrowRef(&{inner:Struct})`, which matches nothing and falls to
`_ => Vec::new()`, so a borrowed closure never reaches its struct env and method dispatch fails.

```rust
param_filters.iter().flat_map(|tyype| {
    match peel_all_references(*tyype) {
        KindT::Struct(sr)         => vec![coutputs.get_outer_env_for_type(range, self.get_struct_template(sr.id))],
        KindT::Interface(ir)      => vec![coutputs.get_outer_env_for_type(range, self.get_interface_template(ir.id))],
        KindT::KindPlaceholder(kp)=> vec![coutputs.get_outer_env_for_type(range, self.get_placeholder_template(kp.id))],
        _ => Vec::new(),
    }
}).collect()
```

Peel with `peel_all_references` (not `peel_one`) so `&&T` still reaches the citizen. The sibling
`get_placeholder_extra_call_envs` (~`:532`) matches the same way and needs the same peel, or a
placeholder behind a borrow won't yield its impl'd-interface envs.

### 2. Rune-solver error paths — fill the stub (trivial un-comment)

`rune_type_solver.rs:692` was **always a stub** (identical panic at `f1dc30f48:postparsing/rune_type_solver.rs:550`);
the onion didn't break it, it was never filled. A live copy of the correct shape exists at
`rune_type_solver.rs:822-831` (and commented at `:728`). Mirror it:

```rust
Err(e) => {
    return Err(RuneTypeSolveError {
        range: vec![lookup.range.clone()],
        failed_solve: FailedSolve {
            steps: vec![], conclusions: HashMap::default(),
            unsolved_rules: rules_s.to_vec(), unsolved_runes: vec![],
            error: ISolverError::RuleError(RuleError { err: e.into(), _phantom: PhantomData }),
        },
    });
}
```

Same fix applies to the twin panics at `:446` (`LookupSR solve error path`) and `:478`
(`RuneParentEnvLookupSR solve error path`). Filling `:692` greens the pure-array test
`reports_when_ssa_from_values_has_unknown_element_type`; it does not unblock lambdas by itself.

### 3. Capture READ (genuine re-impl; do before 4)

The `Capture` arm of `evaluate_lookup_for_load` (`expression_compiler.rs:116-135`, `panic!` at `:125`).
Template: `f1dc30f48:expression_compiler.rs:279-306` (the `ReferenceClosure` arm of
`evaluate_addressible_lookup`).

```rust
Some(IVariableT::Capture(rcv)) => {
    let closured_vars_struct = *rcv.closured_vars_struct_type;
    let tmpl = self.get_struct_template(closured_vars_struct.id);
    let name = match tmpl.local_name { INameT::LambdaCitizenTemplate(n) => n, _ => panic!(/* ... */) };
    let closure_param = LocalVariable {
        name: IVarNameT::ClosureParam(self.typing_interner.intern_closure_param_name(
            ClosureParamNameT { code_location: name.code_location })),
        tyype: KindT::Struct(self.typing_interner.alloc(closured_vars_struct)), // bare struct kind
    };
    // LocalLookupTE::new auto-wraps result = BorrowRef(struct) — this IS the borrow.
    let struct_expr = ExpressionTE::LocalLookup(self.typing_interner.alloc(
        LocalLookupTE::new(self.typing_interner, ranges[0], closure_param)));
    Ok(Some(ExpressionTE::ReferenceMemberLookup(self.typing_interner.alloc(
        ReferenceMemberLookupTE::new(self.typing_interner, ranges[0], struct_expr, rcv.name, rcv.kind)))))
}
```

Gotchas: use `rcv.kind` (the current field), not the stale `rcv.coord` in the commented code; drop
`get_sharedness`/`borrow_soft_load` entirely (the mention already is the borrow; sharedness is decided
target-side in `convert()`); the pre-onion mutable-capture `AddressibleClosure` variant has no onion
counterpart — implement only the `ReferenceMemberLookup` path here (see item 6).

### 4. Capture WRITE (genuine re-impl; depends on 3)

The member loop of `make_closure_struct_construct_expression` (`expression_compiler.rs:269-285`,
`panic!` at `:284`). Template: `f1dc30f48:expression_compiler.rs:331-360`.

```rust
closure_struct_def.members.iter().map(|member| {
    let StructMemberT { name: member_name, tyype } = member;
    let expr = self.evaluate_lookup_for_load(
            coutputs, nenv, range, /* call_location */, region, *member_name)
        .unwrap_or_else(|_| panic!("evaluate_lookup_for_load error"))
        .unwrap_or_else(|| panic!("Couldn't find {:?}", member_name));
    let substituted = substituter.substitute_for_kind(coutputs, *tyype);
    assert_eq!(peel_all_references(substituted), peel_all_references(expr.result()));
    // Closures never contain owned objects: a captured own is a Borrow/Share/WeakRef inside.
    assert!(peel_one_reference(&substituted).is_some());
    expr
}).collect()
```

Gotchas: the old `assert!(coord.ownership != OwnershipT::Own)` becomes "member is a reference kind" via
`peel_one_reference(...).is_some()` (and it must not be `OwnRef`); keep the `Variadic`→panic guard;
`substitute_for_coord` → `substitute_for_kind`; this depends on item 3 because it calls
`evaluate_lookup_for_load`.

### 5. Delete the retired soft-load machinery (do not translate)

`local_helper.rs:89-218` (`maybe_borrow_soft_load`, `soft_load`, `borrow_soft_load`,
`get_borrow_ownership`) and the commented soft-load tail at `expression_compiler.rs:158-183` all build
`SoftLoadTE`, which no longer exists. Their behavior was reabsorbed:
- "a mention is a borrow" → `LocalLookupTE`/`*MemberLookupTE` auto-`BorrowRef` result;
- coercion decisions (clone/alias/must-move) → target-side `convert()` and `is_type_convertible`'s
  borrow arms (`templata_compiler.rs:1184-1203`);
- `Move`/`Unlet`+`mark_unstackified` → the `^x`-scouts-to-`Unlet` construct landed in `07a792c9a`.

`get_borrow_ownership`/`get_sharedness`/`determine_if_local_is_addressible` have no onion callers.
Deleting them is correct; translating them would reintroduce the retired axis.

### 6. Mutable capture for `mutate` — architect decision required

The `Capture` arm of `evaluate_addressible_lookup_for_mutate` (`expression_compiler.rs:200-217`,
`panic!`). This is the addressible/mutable-capture path (old `AddressibleClosure` →
`AddressMemberLookup`). The onion dropped the `Addressible` variable variant, so there is no faithful
mechanical port — how a mutable capture is modeled now is an open design question. Leave the panic and
surface to the architect.

## Recommended sequence

1. Item 1 (`get_param_environments` + `get_placeholder_extra_call_envs` peel) — independent, trivial,
   unblocks borrowed-closure method dispatch. Do first.
2. Item 2 (rune-solver error paths `:692`, `:446`, `:478`) — independent, trivial un-comment.
3. Item 3 (capture read) — genuine re-impl, depends only on the AST.
4. Item 4 (capture write) — genuine re-impl, depends on item 3.
5. Item 5 (delete soft-load machinery) — cleanup, dead code.
6. Item 6 (mutable capture) — architect decision; leave panic until ruled.

Items 1–2 are un-comment-and-translate quick wins; 3–4 are the genuine re-implementations with a
working pre-onion template at `f1dc30f48`; item 6 is the one architectural question.
