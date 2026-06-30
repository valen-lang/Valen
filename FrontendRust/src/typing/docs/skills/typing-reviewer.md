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
