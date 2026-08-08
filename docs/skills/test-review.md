---
name: test-review
description: Style rules for reviewing tests in FrontendRust — raw strings for source literals, pattern-specific collect_ macros over manual filtering.
g_read_when: Read when reviewing or writing typing-pass tests (or other tests that walk the AST with collect_ macros).
g_mention_in:
  - CLAUDE.md
---

# Test Review

Style rules for typing-pass and similar AST-walking tests.

## 1. Understand the test's spirit before reviewing its assertions

The first thing to do when reviewing or extending a test is figure out **what unique value it provides relative to neighboring tests in the file**. Every assertion the test makes should serve that value. Every assertion *not* relevant to that value is a future false-positive waiting to fire on an unrelated change.

Read the surrounding tests, the test's name, and the program fixture together. Ask:
- What does *this* test cover that no sibling test covers?
- What kind of regression is this test designed to catch?
- What is the test deliberately *not* about?

Then assert (or `collect_*!`) only for the things on the "what it covers" list. Leave everything else unasserted — including counts, ordering, and surrounding context — unless it's specifically the spirit of this test.

Example anti-pattern: a test whose spirit is "verify a closure produces both its `__call` and its drop" should assert each call's pattern individually. It should NOT also assert "and there are *exactly* two FunctionCalls in main." That extra check is orthogonal to the spirit — if some future arc adds a hidden `argc`/`argv` grabber call into main, the test should keep passing. Pinning the total breaks for the wrong reason and signals incorrect cause-of-failure when it fires.

Same logic in the other direction: an assertion that's already implied by the lookup that produced the value is redundant (e.g., `name_is_lambda_in(lookup_lambda_in("main").id, "main")` — the lookup already filtered to lambdas in main). Cut it.

## 2. Raw strings for embedded source code

Embedded Vale (or any source-language) literal goes in a raw string, never an escape-sequence string. `\n` inside a regular string is a footgun — invisible whitespace, awkward to edit, hard to read.

Bad:
```rust
let code = "\nexported func main() int { return { 7 }(); }\n";
```

Good:
```rust
let code = r#"
exported func main() int { return { 7 }(); }
"#;
```

Applies to any non-trivial string fixture, not just Vale source.

## 3. One `collect_` per specific pattern — no post-hoc filtering

Every assertion should be a `collect_only_tnode!` / `collect_where_tnode!` invocation whose **pattern** matches the exact shape you're looking for. Let the pattern do the specificity. Do not collect a broad set and then filter, count, or destructure in Rust.

Anti-patterns to flag:
- `.any(|x| ...)` over a collected `Vec`
- `let last_two = &steps[steps.len().saturating_sub(2)..];` or similar `id.steps()` slicing
- `matches!` inside a closure that walks collected results
- `assert_eq!(matches.len(), N)` followed by per-element checks — split into N specific `collect_only_tnode!` calls instead
- Reaching for `name_is_lambda_in`-style helpers when a destructure pattern would express the same shape directly

Good shape:
```rust
collect_only_tnode!(
    NodeRefT::FunctionDefinition(main),
    NodeRefT::FunctionCall(FunctionCallTE {
        callable: PrototypeT { id: IdT {
            local_name: INameT::LambdaCallFunction(_), ..
        }, .. },
        ..
    }) => Some(())
);
```

The pattern asserts the exact AST shape; the macro asserts exactly-one-match. No `Vec` post-processing.

If you need to assert two distinct shapes are both present, write **two** `collect_only_tnode!` invocations with different patterns. The macro counts each one independently, and each pattern stays narrow and readable.

## 4. Tests should fail at the shape, not at a helper

If a test fails because a string-matching helper (`name_is_lambda_in`, `human_name == "drop"`) returns false, the failure message gives almost no signal. If a test fails because a destructure pattern didn't match, the panic includes the actual AST node debug-printed at the failure site — which is exactly what you need to diagnose the regression.

Prefer the destructure pattern. Reserve helpers for cross-cutting concerns the pattern syntax can't express.

## 5. No `|` in a test's match pattern

A test pins one exact shape. An or-pattern accepts several, so a wrong-but-listed variant passes silently. Match the single shape the code actually produces.

BEFORE:
```rust
match slot {
    None | Some(DestinationLocalP { decl: INameDeclarationP::IgnoredLocalNameDeclaration(_), .. }) => {}
    other => panic!("got {:?}", other),
}
```

AFTER:
```rust
match slot {
    Some(DestinationLocalP { decl: INameDeclarationP::IgnoredLocalNameDeclaration(_), .. }) => {}
    other => panic!("got {:?}", other),
}
```

## 6. Never an `if` guard in a test's match

Pin a literal in the pattern, don't test it in a guard. A guard hides the check outside the pattern.

BEFORE:
```rust
match name {
    IVarNameS::CodeVarName(s) if s.as_str() == "a" => {}
    other => panic!("got {:?}", other),
}
```

AFTER:
```rust
match name {
    IVarNameS::CodeVarName(StrI("a")) => {}
    other => panic!("got {:?}", other),
}
```

## 7. No `cast!` in a test; match the variant

Don't `cast!` into a variant, then work on the result. Match the variant in one flat pattern, so a wrong variant fails through `other => panic!` with the actual value.

BEFORE:
```rust
let let_se = cast!(&expr, IExpressionSE::Let);
match &let_se.pattern.destructure {
    Some(_) => {}
    other => panic!("got {:?}", other),
}
```

AFTER:
```rust
match &expr {
    IExpressionSE::Let(LetSE { pattern: AtomSP { destructure: Some(_), .. }, .. }) => {}
    other => panic!("got {:?}", other),
}
```

## 8. Pin the exact result; don't `if let` a shape check

A test asserts the one correct shape. An `if let ... panic!` only fires on a specific wrong shape and lets every other invalid result pass silently. Match the expected shape with `other => panic!`, so anything unexpected fails.

BEFORE:
```rust
// only catches one bad shape; a totally different (also-wrong) body head passes
if let IExpressionSE::Consecutor(cons) = &block.expr {
    if let Some(IExpressionSE::Let(_)) = cons.exprs.first() {
        panic!("did not expect a param let here");
    }
}
```

AFTER:
```rust
match &block.expr {
    IExpressionSE::Void(_) => {}
    other => panic!("expected an untouched Void body head, got {:?}", other),
}
```
