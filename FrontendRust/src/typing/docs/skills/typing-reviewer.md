# Typing-pass reviewer notes

Every addition to this doc should be 30 words of prose or less, plus a BEFORE example that is as concise as possible, and an AFTER example that is as concise as possible. Ask the user if we need more words to express something.

## Inline into the match

Prefer destructuring the exact constants you care about inside the match pattern over destructuring a binding and asserting on it after.

BEFORE:
```rust
match err {
    Foo { name, .. } => assert!(name == "bar"),
    other => panic!("got {:?}", other),
}
```

AFTER:
```rust
match err {
    Foo { name: "bar", .. } => {}
    other => panic!("got {:?}", other),
}
```

## Inspect the AST, don't just compile

`expect_compiler_outputs` alone only proves the code compiled. Add a match or `collect_` that inspects the AST for the specific shape the test's spirit is about.

BEFORE:
```rust
compile.expect_compiler_outputs();
```

AFTER:
```rust
let coutputs = compile.expect_compiler_outputs();
let main = coutputs.lookup_function_by_str("main");
match main.header.return_type {
    CoordT { ownership: OwnershipT::Own, .. } => {}
    other => panic!("got {:?}", other),
}
```
