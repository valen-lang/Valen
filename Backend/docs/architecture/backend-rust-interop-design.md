# Backend Rust Interop

This doc `typing-rust-interop-design.md` fills out the details of how rust interop works in the typing pass.

For things outside the typing pass, see `rust-interop-design.md`. If there's anything inconsistent or conflicting between these two docs, please **raise it to the architect**.

## Design (human-only)

**Backend is mostly decoupled** from Rust-specific logic. Whereas the typing pass reads from rustc, and instantiator will work together with rustc to instantiate things, backend doesn't know anything about Rust and doesn't call into rustc at all.

Still, the backend does have some implicit concerns because of Rust, hence this doc.

### Rust Calls Valen with the Wrong ABI

Rustc sends Rust-ABI-flavored arguments to Valen functions, because rustc thinks they're normal Rust functions.

Because of that, we can't let Rust functions directly call Valen functions. Instead, they call a wrapper function.

The instantiator notes down which wrapper functions are needed, and then the backend generates them.

Possible todo: make Valen conform to Rust ABI?

## Design Proposals

S1. The inbound wrapper for a Rust→Valen callback is built during backend codegen, not by a separate pass. This is needed because Rust is going to try to call us with Rust ABI, and we might not be using that ABI. So we need a wrapper they can call that will do the right conversions.

## Details

## Test cases

## Background

### Self-evident from the code

### Documented

### Undocumented

## Open Questions

## Required Reading

 * design-assistant
