# Parameter Full-Type / Value-Type Split (PFVSZ)

A function parameter's type is stored on `ParameterS` in two halves: the outer reference wraps, and the value those wraps enclose.

Take `func foo(x &Ship)`. The **value type** is `Ship`, the citizen being referred to. The **full type** is `&Ship`, that citizen inside one borrow wrap. `&&Ship` has the same value type `Ship` and a full type of two wraps. A bare `func foo(x Ship)` has no wraps at all, so its full type and value type are the same.

The `ParameterS` holds:
- `full_type_rune` holds the rune for the full type.
- `value_type_rune` holds the rune for the value type.
- `value_type_rules` holds the `Lookup` / `Call` / etc. that build `value_type_rune`.
- `type_outer_ref_rules` holds the chain of `BorrowRef` / `WeakRef` / `OwnRef` wraps whose outermost result is `full_type_rune`.

`translate_signature_templex` produces the split. It peels the outermost run of wraps into `type_outer_ref_rules`, and puts the value type, plus anything nested inside it (including wraps buried in template args), into `value_type_rules`.

**Why store two halves.** The typing pass ignores the outermost references when looking for functions to call. For example, if `my_ship_ref &Ship` and we call `my_ship_ref.launch()`, the typing pass ignores the `&` and looks in the namespace of `Ship` for the `launch` method. Having a separate `value_type_rules` (without the outer references) makes it easier for the typing pass to do this.

**Invariants.** `type_outer_ref_rules` may hold only the four wrap rules. When it is empty (no wraps), `full_type_rune` and `value_type_rune` are the same rune. `ParameterS::new` checks both with `debug_assert!` rather than making them unrepresentable, so an illegal bucket or a mismatched pair fails loudly at construction.
