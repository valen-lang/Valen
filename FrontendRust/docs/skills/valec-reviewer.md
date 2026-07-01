# Valec reviewer notes

Every addition to this doc should be 30 words of prose or less, plus a BEFORE example that is as concise as possible, and an AFTER example that is as concise as possible. Ask the user if we need more words to express something.

The following rules are phrased in typing-pass terms but apply to every pass — postparser, typing, instantiator, hammer, backend. Whenever code branches on one of these proxy signals, the fix is to route on the honest attribute or flag instead.

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
