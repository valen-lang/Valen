# Typing Pass Rust Interop

This doc `typing-rust-interop-design.md` fills out the details of how rust interop works in the typing pass.

For things outside the typing pass, see `rust-interop-design.md`. If there's anything inconsistent or conflicting between these two docs, please **raise it to the architect**.

## Design (human-only)

By design, Rust interop is mostly abstracted away from the typing pass; the typing pass doesn't really think about Rust interop that much. 

The typing pass (and the rest of the compiler) don't _depend_ on anything in `rustc` to compile Valen code. This is also enforced by all the tests that have the `rust_interop` flag turned off. If Rust interop ever requires something to change in the core compiler, it's likely a sign that there is a corresponding bug that we can trigger with pure Valen.

One way we abstract it away is that the typing pass doesn't do typechecking against Rust items directly; the Rust interop code first **generates a corresponding postparsed AHT** for that rust, and then Valen type-checks against that. This is true of functions, structs, interfaces, everything.

## Design Proposals

S1. A Rust trait imports as a synthesized AHT-level `InterfaceS`, the same way a called Rust function imports as a synthesized `FunctionS`. The interface's abstract methods carry the trait's method signatures. There is no separate rust-trait concept — this synthesized interface is the only representation.

S2. Valen typechecks a struct's `impl` of an imported Rust trait, signature match included, with the existing interface/override machinery unchanged. `rust_interop` only synthesizes the `InterfaceS`; a mismatch is a Valen error, not a deferred rustc error on generated source.

S3. The inbound wrapper for a Rust→Valen callback is built during backend codegen, not by a separate pass. This is needed because Rust is going to try to call us with Rust ABI, and we might not be using that ABI. So we need a wrapper they can call that will do the right conversions.

## Details

## Test cases

## Background

### Self-evident from the code

### Documented

### Undocumented

## Open Questions

## Required Reading

 * design-assistant
