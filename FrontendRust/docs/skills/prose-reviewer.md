# Prose reviewer notes

Every addition to this doc should be 30 words of prose or less, plus a BEFORE example that is as concise as possible, and an AFTER example that is as concise as possible. Ask the user if we need more words to express something.

These rules apply to any prose in the codebase — test-header comments, doc comments, error messages, ignore rationales, checked-in design notes. Each rule builds on the previous.

## State the invariant being enforced, not just observed

A test-header comment should say what the test *protects* — the rule that breaks if the test regresses. A bare statement of behavior reads as trivia; framing it as an enforcement makes the WHY explicit.

BEFORE:
```rust
// A user-defined `func implicit_clone(&Ship) Ship` is callable by name on an Own Ship local.
```

AFTER:
```rust
// Ensures that a user-defined `func implicit_clone(&Ship) Ship` is callable by name on an Own Ship local.
```

## Active voice describing an action, not passive properties

Readers model behavior as things that happen when code runs, not as static properties. Replace "X is callable" with "someone can call X." Drop insider adverbs ("by name") that only clarify against unstated alternatives.

BEFORE:
```rust
// Ensures that a user-defined `func implicit_clone(&Ship) Ship` is callable by name on an Own Ship local.
```

AFTER:
```rust
// Ensures that we can call a user-defined `func implicit_clone(&Ship) Ship` with an argument that is an Own Ship local.
```

## Front-load the load-bearing information

Put the interesting input or the surprising claim at the front of the sentence; demote the setup detail to the tail. The reader shouldn't hold context while waiting to learn what matters.

BEFORE:
```rust
// Ensures that we can call a user-defined `func implicit_clone(&Ship) Ship` with an argument that is an Own Ship local.
```

AFTER:
```rust
// Ensures that a callsite can give an Own Ship local as an argument to call a user-defined `func implicit_clone(&Ship) Ship`.
```

## Describe the general invariant; demote the specific example

State the language rule the test protects; use the specific function as illustration ("e.g. …"), not as the subject. The comment should stand alone as a claim about the system.

BEFORE:
```rust
// Ensures that a callsite can give an Own Ship local as an argument to call a user-defined `func implicit_clone(&Ship) Ship`.
```

AFTER:
```rust
// Ensures that a callsite can give an Own Ship local as an argument to a parameter that expects a borrow reference, e.g. `func implicit_clone(&Ship) Ship`.
```

## Lead with the takeaway, then an example, then the details

Structure an explanation as **TLDR → example → details**, in that order — including this rule.

BEFORE — details-first, dense, and describing the code instead of showing it:
```
// An opaque FFI handle's LLVM type is chosen by ref-layer, not kind: every
// concrete kind shares one {i64}, every interface one {i64,i64}, and
// getExternalType returns them per kind while per-kind distinctness lives only
// in the emitted C typedefs.
```

AFTER — takeaway, then a real example, then the details, in plain sentences:
```
// FFI handles share their LLVM type across classes, but each class gets its
// own C typedef.
//
// For each exported class we emit, e.g.:
//   typedef struct vtest_Ship { uint64_t _reserved; } vtest_Ship;
//   void vtest_Ship_fly(vtest_Ship ship);
// Ship and Boat are both {i64} inside the backend, but C sees vtest_Ship and
// vtest_Boat as distinct, incompatible types.
//
// getExternalType returns that shared type for every class. Each class's own
// typedef (vtest_Ship, vtest_Boat) is written only into the C header.
```

The takeaway gives the reader a frame; the example makes it real before any abstraction; the details refine it once the reader can absorb them. Lead with details instead and the reader holds uninterpreted facts, waiting to learn why they matter.

One companion habit at the example step — **show, don't describe**: paste the real code or values, don't paraphrase what they'd look like. (For effort at the sentence level, see the next rule.)

## Minimize reader effort, not word count

Don't compress ideas into dense clauses to save words. A longer plain sentence — one idea at a time — costs the reader less than a short crammed one.

BEFORE — one sentence, four facts crammed in:
```
// An opaque FFI handle is a ref-layer-chosen LLVM type shared as {i64} by every
// concrete class, with per-class distinctness living only in the C typedefs.
```

AFTER — same facts, one plain idea per sentence:
```
// Every concrete class's FFI handle is the same LLVM type: a struct holding one
// address. Each class still gets its own name in the generated C typedefs.
```
