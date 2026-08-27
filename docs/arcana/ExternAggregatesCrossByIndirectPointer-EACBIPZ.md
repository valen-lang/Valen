---
description: How a large struct crosses the interop extern boundary — as an indirect pointer to a caller-owned copy, not LLVM byval.
g_read_when: "Read when adding or changing how a large struct (aggregate) crosses the interop extern boundary as an argument or return, or when tempted to mark such an argument LLVM `byval`."
---

# Extern Aggregates Cross By Indirect Pointer (EACBIPZ)

**Principle:** A large aggregate crosses the interop extern boundary as an indirect pointer to a caller-owned copy, never as an LLVM `byval` value.

rustc classifies these as `PassMode::Indirect { on_stack: false }`, a pointer passed in a register; `byval` is the on-stack convention, so using it mismatches the ABI and corrupts the call.

For `add_and_return(d: Domino, loc: i32) -> Domino`, where `%Domino = type { [6 x i64] }` (48 bytes), the extern is declared and called like this:

```
declare void @add_and_return(ptr sret(%Domino), ptr, i32)

%dslot = alloca %Domino
store %Domino %d, ptr %dslot
call void @add_and_return(ptr sret(%Domino) %retslot, ptr %dslot, i32 7)
```

The return is an out-pointer (`sret`); the `Domino` argument is a plain `ptr` to a spilled copy, with no `byval`. The classification comes from `fn_abi_of_instance`. The backend holds no `tcx`, so `compute_extern_abi` records it and the consumers (`buildBoundarySignature`, `buildCallOrSideCall`, `declareExternFunction`) act on that descriptor without deriving any ABI of their own.

The `byval` form declares the argument `ptr byval(%Domino) align 8`. LLVM lowers a lone `byval` argument to a register pointer, so a one-argument extern still passes. A second argument after it makes the on-stack semantics corrupt the call and crash at runtime. A lone-argument test therefore cannot prove the argument ABI: exercising it needs a second argument, ideally with an sret return.

The return out-pointer also carries an `sret` attribute, which routes it to the platform's hidden result register (x8 on aarch64). Without it the pointer lands in the first ordinary argument register, and the callee reads a garbage sret address.

`compute_extern_abi` handles only `on_stack: false`. The on-stack byval case (`on_stack: true`) panics, alongside the register-pair `PassMode::Pair`/`Cast` cases, until a target needs them.
