---
name: valec-guidelines
description: "Valec reviewer notes: cross-pass compiler style rules (never discard Err payloads, route on honest attributes)."
g_read_when: Read when reviewing or writing FrontendRust compiler code in any pass.
g_mention_in:
  - CLAUDE.md
---

# Valec reviewer notes

Every addition to this doc should be 30 words of prose or less, plus a BEFORE example that is as concise as possible, and an AFTER example that is as concise as possible. Ask the user if we need more words to express something.

The following rules are phrased in typing-pass terms but apply to every pass — postparser, typing, instantiator, hammer, backend. Whenever code branches on one of these proxy signals, the fix is to route on the honest attribute or flag instead.

## Never discard an Err payload

An error variant carries diagnostic detail (which candidates were tried, why each was rejected). Silently dropping it produces a bare error the user can't act on. Preserve it in the emitted error. Watch out for `Err(_)` and `Err(_something)`.

BEFORE:
```rust
Err(_fff) => {
    return Err(ICompileErrorT::MustExplicitlyMoveT { range, source_type, target_type });
}
```

AFTER:
```rust
Err(fff) => {
    return Err(ICompileErrorT::MustExplicitlyMoveT { range, source_type, target_type, underlying: fff });
}
```

## No jargon soup in comments

Write like Feynman: simple words in clear sentences, even for complex ideas. Comments strung from insider terms only help a reader who's already inside; the point of a comment is to help everyone else.

BEFORE:
```rust
// Bare-use of an Own struct local at a Borrow target resolves without an
// explicit `&` — bare-use produces a Borrow-flavored coord and the call
// resolves against `func bork(&Struct)` directly.
```

AFTER:
```rust
// If you have `x: Ship` and call `bork(x)` where `bork` takes `&Ship`,
// you don't need to write `bork(&x)` — the compiler treats `x` as a
// borrow automatically.
```

## Define coined terms or drop them

A coined term like "silent boundary", used as if the reader already knows it, is noise. Say it plainly, or define it in a doc and link there.

BEFORE:
```cpp
// Silent boundary: the handle is a packed pointer, unpack it without
// touching the RC.
```

AFTER:
```cpp
// The handle is a packed pointer. Unpack it without changing the
// refcount — C-side alias/dealias is explicit via the auto-gen'd helpers.
```

## No "tombstones" comments; no historical "used to be" context in comments

Don't preserve the pre-refactor shape or explain what a change simplified from. Nobody working with the system today needs to mentally filter that out. Describe only the current invariant.

BEFORE:
```rust
// Simplified after Vale1's `ec53b65e7` retired ImmutableShare/ImmutableBorrow:
// Share target → MutableShare; Borrow target → MutableBorrow; Own/Weak
// pass-through. Immutable-region conditional flavoring was a dead branch.
```

AFTER:
```rust
// Share → MutableShare; Borrow → MutableBorrow; Own/Weak pass-through.
```

## No transient timeline references in comments

Don't anchor comments to phase/slice/arc/era/plan/slab/project labels ("Phase 2 slice 4+6", "sub-arc a"). The label is meaningless a session later and actively confusing once the next timeline starts. Explain the invariant the code enforces, not when it was added.

BEFORE:
```rust
/// Phase 2 slice 4+6: I-IR mirror of typing pass's AliasTE. Reflavors a
/// reference expression's ownership.
pub struct AliasIE<'s, 'i> { ... }
```

AFTER:
```rust
/// I-IR mirror of typing pass's AliasTE. Reflavors a reference expression's
/// ownership without changing its underlying value.
pub struct AliasIE<'s, 'i> { ... }
```

## Don't gate on struct member count

An empty Vale struct and an extern struct both have zero members but need opposite drop treatment. Gate on the honest attribute (extern, opaque), not the count.

BEFORE:
```rust
if struct_def.members.is_empty() {
    ExpressionT::Discard(source)
}
```

AFTER:
```rust
if struct_def.attributes.iter().any(|a| matches!(a, ICitizenAttributeT::Extern(_))) {
    ExpressionT::Discard(source)
}
```

## Don't gate on function parameter count

A nullary Vale function and a zero-arg extern shim share `params.len() == 0` but need different codegen. Match on the intent-carrying attribute, not the arity.

BEFORE:
```rust
if header.params.is_empty() {
    emit_factory_call(header)
}
```

AFTER:
```rust
if is_factory(&header.attributes) {
    emit_factory_call(header)
}
```

## Don't gate on function generic parameter count

`header.template_args.len() == 0` collapses "never generic" and "fully-monomorphized" into one condition, but they need different handling downstream. Check the concrete property instead.

BEFORE:
```rust
if header.template_args.is_empty() {
    skip_bound_resolution(header)
}
```

AFTER:
```rust
if header.bounds.is_empty() {
    skip_bound_resolution(header)
}
```

## Don't gate on struct generic parameter count

Same as functions: `StructTT.template_args.len()` conflates "never parametric" with "fully-substituted instance." Check for unresolved placeholders in the args, not the length.

BEFORE:
```rust
if struct_tt.template_args.is_empty() {
    treat_as_concrete(struct_tt)
}
```

AFTER:
```rust
if !struct_tt.template_args.iter().any(is_placeholder) {
    treat_as_concrete(struct_tt)
}
```

## Don't gate on whether a function is generic

"Is this function generic?" is a proxy for "needs monomorphization" or "has unresolved bounds." Name the concrete predicate — a fully-substituted generic behaves like a concrete function.

BEFORE:
```rust
if !header.template_args.is_empty() {
    register_for_monomorphization(header)
}
```

AFTER:
```rust
if requires_monomorphization(&header) {
    register_for_monomorphization(header)
}
```

## Don't gate on whether a struct is generic

"Is this struct generic?" is a proxy for "needs an instantiation table" or "can't be exported by value." Name the property; a fully-substituted generic struct behaves like a concrete one.

BEFORE:
```rust
if !struct_def.template_args.is_empty() {
    needs_instantiation_table(struct_def)
}
```

AFTER:
```rust
if has_unresolved_placeholders(&struct_def) {
    needs_instantiation_table(struct_def)
}
```

## Every test opens with a comment saying what it protects

A test carries a comment stating the invariant it protects, placed as the first line inside the test body, not above `#[test]`.

BEFORE:
```rust
// A bare param keeps its real name.
#[test]
fn bare_param_keeps_name() {
    let program = compile(..);
}
```

AFTER:
```rust
#[test]
fn bare_param_keeps_name() {
    // A bare param keeps its real name; no synthetic DesugaredParamName.
    let program = compile(..);
}
```

## No silent catch-all `else`

A bare `else` silently swallows the next case added to the ladder. Give every real case its own condition and let the `else` assert unreachable, so additions fail loud.

BEFORE:
```cpp
} else if (name == "__vbi_strcmp") {
  emitStrcmp();
} else {                      // silently the __vbi_strindexof case
  emitStrindexof();
}
```

AFTER:
```cpp
} else if (name == "__vbi_strcmp") {
  emitStrcmp();
} else if (name == "__vbi_strindexof") {
  emitStrindexof();
} else {
  assert(false);              // unreachable
}
```

## Avoid early-returns; keep equivalent branches together

Sibling match arms read as analogous operations. Hoisting some into early-returns above the match signals they're different, losing that. Keep equivalent branches at the same indentation, not pulled out.

BEFORE:
```rust
// ref layers hoisted above the match, reading as "special"
if let Some(r) = as_ref_layer(t) {
    return emit_ref_layer(r);
}
match t {
    Call(c) => emit_call(c),
    Array(a) => emit_array(a),
}
```

AFTER:
```rust
match t {
    Borrow(r) => emit_ref_layer(Borrow, r),
    HeapOwn(r) => emit_ref_layer(HeapOwn, r),
    Call(c) => emit_call(c),
    Array(a) => emit_array(a),
}
```

## Arrange the diff to be easy to review

When you rewrite something, put the replacement where the original lived. A reviewer diffs old-against-new in place; code that moves across the file reads as an unrelated deletion plus addition.

BEFORE:
```diff
@@ line 40 @@
-fn translate(t) { /* old body */ }
@@ line 380 @@
+fn translate(t) { /* new body */ }
```

AFTER:
```diff
@@ line 40 @@
-fn translate(t) { /* old body */ }
+fn translate(t) { /* new body */ }
```

## Pointer-keyed maps need a deterministic hasher

Pointer/address keys hash by their address, which varies run to run, so iteration order leaks nondeterminism. Give any pointer-keyed unordered_map/set an AddressHasher; value-keyed maps are already deterministic.

BEFORE:
```cpp
std::unordered_map<InterfaceKind*, std::vector<Edge*>> edgesByInterface;
```

AFTER:
```cpp
std::unordered_map<InterfaceKind*, std::vector<Edge*>,
    AddressHasher<InterfaceKind*>> edgesByInterface;
```

## Outlaw an impossible state, or assert it

If a combination of data should never occur, make it unrepresentable via the type system (ask before changing a type), else guard it with a debug_assert stating the invariant.

BEFORE:
```rust
// Typed to hold any rule, so a Lookup can silently sneak in.
pub outer_shape_rules: &'s [IRulexSR<'s>],
```

AFTER:
```rust
debug_assert!(
  outer_shape_rules.iter().all(|r| matches!(r,
    IRulexSR::BorrowRef(_) | IRulexSR::HeapOwnRef(_) | IRulexSR::ShareRef(_) | IRulexSR::WeakRef(_))),
  "outer_shape_rules may only hold onion ref wraps");
```

## Pin a shape with one full match, not asserts

Pin a shape with one `match { expected => {}, other => panic! }` over the whole value. Fold `expect_1`/`.unwrap()` into the pattern. Never `.any` or `assert!(matches!)`; asserts are only for numbers and equality.

BEFORE:
```rust
let param = expect_1(&func.header.params.as_ref().unwrap().params);
assert!(matches!(param.pattern.as_ref().unwrap().destructure, Some(_)));
```

AFTER:
```rust
match &func.header.params {
    Some(ParamsP { params: [ParameterP {
        pattern: Some(PatternPP { destructure: Some(_), .. }), .. }], .. }) => {}
    other => panic!("expected one destructuring param, got {:?}", other),
}
```

## Don't talk about specific callers

A function's comments shouldn't talk about its callers, it should talk in general about what it's useful for.

BEFORE:
```
/// The (local name, template id) for one of a citizen's internal methods, derived from the
/// citizen's own template id. This is the single source of truth for that id: the index loop in
/// Compiler::evaluate seeds the postparsed cache under it, and precompile_struct/precompile_interface
/// build the outer-env entry from it — the two must agree, so both go through here.
pub fn internal_method_template_id(&self, parent_template_id: &'t IdT<'s, 't>, internal_method: &'s FunctionS<'s>, )
```

AFTER:
```
/// The (local name, template id) for one of a citizen's internal methods, derived from the
/// citizen's own template id. This is the only place that should calculate this.
pub fn internal_method_template_id(&self, parent_template_id: &'t IdT<'s, 't>, internal_method: &'s FunctionS<'s>, )
```

## Don't preserve backwards compatibility

We're pre-alpha. Code as if we don't have any users. It's important to not maintain code that we don't use.

BEFORE:
```rust
// Superseded by translate_signature_type_st (the ITypeST twin) and now caller-free except its own
// recursion. Kept during the migration.
pub fn translate_signature_templex<'s, 'p>(...) -> (RuneUsage<'s>, RuneUsage<'s>) {
    // ~100 lines, called only by translate_signature_templex itself
}
```

AFTER: (deleted)


## When adding a helper method

How many callers does it have? If just one, lean towards inlining it.

If it only has a few callers, please think through: what would it look like if it were inlined? Would any of the inlined sites end up having simplifications?

Sometimes a helper method is actually more complex than just inlining at every callsite.


## When changing a helper method

Does the helper method make sense anymore?

What was the mindset when it was built? Is that mindset valid anymore? If not, consider removing the helper or inlining it or changing it.


## Style guide

 * Don't reorder existing params, but when adding new ones, add them in roughly lifetime order.
    * Example: if `Game` contains `Level` contains `Tile`, a signature would be `fn tile_collect_mana(game: &Game, level: &Level, tile: &Tile)`.
    * Example: if `CompilerOutputs` outlives environments, a signature would be `fn something(coutputs: &CompilerOutputs, env: &IEnvironment, key: String)`.
 * If you want to pipe some data to where it isn't already, prefer:
    * As a function parameter.
    * If it's immutable (calculated once at construction, never changed), you can include it in whatever struct seems best.
    * Do not add it to a convenient mutable struct (like World or CompilerOutputs) without human approval.


## Need Human Approval For Optional Returns and Fields and Empty Variants

Get human approval for any function that returns any Option, and any struct field that contains an Option, or any enum that contains a "None"/"Empty"/"Null"/"Unspecified"/etc variant.

If you think you need an Option in a return type, you are probably wrong.

 * If you think "I need this Option because the data might not exist", you're wrong, the data should have existed, and you need to ask the human to make it exist.

If you think you need an Option in a struct field, you're probably wrong.

If you think an enum needs a "None"/"Empty"/"Null"/"Unspecified"/etc variant, you're probably wrong.

Changing the Option to a two-state enum is **NOT** appropriate, that is just a workaround that violates the spirit of this rule. Ask a human for the proper resolution.

If you see an Option in a return or field or an Empty-ish variant that doesn't have `// VOPT:` above it, tell the human, so the human can look into it and see if we need to rip it out.

Only the human may explicitly specifically allow one of these by adding a `// VOPT:` comment above it, or by describing it in a `## Design (human-only)` section in a `*-design.md` doc. If you see one not explicitly authorized by `// VOPT:` or a `*-design.md`'s `## Design (human-only)` section, let the human know.

## Need Human Approval For Fallbacks

Get human approval for any code that does a "fallback", or a "default" or a "graceful degradation" or "graceful callback". Fallbacks are always, without fail, an error in this compiler.

Do not try to justify adding a gracefull fallback. Catch yourself if you try to justify it. That's a signal that you need to escalate it.

Only the human may explicitly specifically allow a fallback by adding a `// VFALLBACK:` comment above it, or by describing it in a `## Design (human-only)` section in a `*-design.md` doc. If you see one not explicitly authorized by `// VFALLBACK:` or a `*-design.md`'s `## Design (human-only)` section, let the human know.

For example, flag any `unwrap_or`.

## Required Reading

 * prose-reviewer
