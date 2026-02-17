
# M1: Rust should mirror Scala as close as possible (RSMSCP)

Keep making sure that everything in the rust version mirrors almost exactly whats in the scala version. Down to the functions, their positions relative to each other, their names, their logic, and if possible variable names too.

Note that it's fine to leave panic!s/assert!s for anything unimplemented. Those differences are okay.


# M2: TODOS + unimplemented code MUST panic

If you must leave todos or unimplemented things, ensure they panic (or assert) with a unique message that will make it immediately clear when failures are from not-yet-brought-over code.


# M3: Don't conveniently change requirements

If the implementation has a bug, or a test fails, do not change the requirements of the implementation or test.

For example, if a test fails, never make the test expect the current bad behavior (that defeats the entire purpose of tests). The Scala tests all passed. The Rust tests should pass, and they should expect the exact same behavior the Scala tests did.

For example, if the implementation isn't working right, don't change the code or comments to be okay with it. Do not take liberties with what should be ported over. If you think something isnt needed yet, then leave a panic!() (or assert) there.

Figure out where the Rust version's logic doesn't match the scala version's logic, and make it more consistent.

Ensure that all Rust code/test requirements exactly match the old Scala code/test requirements.


# M4: No expensive clones

Ensure that there are no .clone()s for large data structures.


# M5: Migrate comments too

Ensure that all comments in the Scala version are also in the Rust version.

(You can ignore MIGALLOW comments though)


# M6. Enums Shouldn't Contain Complex Data (ESCCD)

We generally don't like enums that contain complex data as direct fields. We prefer the enum variant to contain a struct with the fields. This is so that data can be in a NodeRefP entry, so it's easier for tests to look directly for them. It also makes it so we can more easily make a cast! macro to "cast" an enum to its inner type.
Also, enums themselves should never be interned; only their contents should be interned.


# M7: Avoid `if matches!(...` in tests if possible (AIMITIP)

Here we have an unnecessary `if matches!(`:

```
let mutability_literal_rule = crate::collect_only_sstruct!(
  imoo,
  NodeRefS::LiteralRule(literal_rule)
    if matches!(
      &literal_rule.literal,
      ILiteralSL::MutabilityLiteral(mutability_literal)
        if mutability_literal.mutability == crate::parsing::ast::MutabilityP::Mutable
    ) => Some(literal_rule)
);
assert_eq!(mutability_literal_rule.rune, imoo.mutability_rune);
```

When possible, combine these into the original pattern like this:

```
crate::collect_only_sstruct!(
  imoo,
  NodeRefS::LiteralRule(
    literal_rule @ LiteralSR {
      literal: ILiteralSL::MutabilityLiteral(mutability_literal),
      ..
    }
  ) if mutability_literal.mutability == crate::parsing::ast::MutabilityP::Mutable
    && literal_rule.rune == imoo.mutability_rune => Some(())
);
```

This rule only really matters for tests. Implementation can do whatever it wants.


# M8: Suffix When Dealing With Multiple Stages (SWDWMS)

In functions that handle two different stages of data (which is common, most functions transform data from the last stage to the next stage), suffix your local variables so it's clear whether it's pointing to the old data or the new data.

For example, the old Scala did this well:

```scala
  def scoutFunction(
    file: FileCoordinate,
    functionP: FunctionP,
    maybeParent: IFunctionParent):
  (FunctionS, VariableUses) = {
    val FunctionP(range, headerP, maybeBody0) = functionP;
    val FunctionHeaderP(headerRange, maybeName, attrsP, maybeGenericParametersP, templateRulesP, maybeParamsP, returnP) = headerP
    val FunctionReturnP(retRange, maybeRetType) = returnP

    val headerRangeS = PostParser.evalRange(file, headerRange)
    val rangeS = PostParser.evalRange(file, range)
    val codeLocation = rangeS.begin
    val retRangeS = PostParser.evalRange(file, retRange)
```


# M9: Keep inline comparisons inline (KICI)

This is not a style preference. It is a Scala-parity requirement.
If Scala has an inline comparison/check inside a `match`/`case`, keep that check inline in the Rust pattern itself.
Do NOT move it into a guard.
Do NOT move it outside the match.

Look for these and change them to match the Scala shape:

Scala source shape:
```scala
node match {
  case NameS(StrI("x")) =>
}
```

Wrong (moved into guard):
```rust
match node {
  Node::Name(name) if name.as_str() == "x" => {}
  _ => panic!("expected x"),
}
```

Right (inline in pattern):
```rust
match node {
  Node::Name(StrI("x")) => {}
  _ => panic!("expected x"),
}
```

Scala source shape:
```scala
node match {
  case NameS(StrI("x")) =>
}
```

Wrong (moved outside match):
```rust
let name = match node {
  Node::Name(name) => name,
  _ => panic!("expected name"),
};
assert_eq!(name.as_str(), "x");
```

Right (inline in pattern):
```rust
match node {
  Node::Name(StrI("x")) => {}
  _ => panic!("expected x"),
}
```

Scala source shape:
```scala
Collector.only(program, {
  case LocalLoadSE(_, _, UseP) =>
})
```

Wrong (property checked later):
```rust
let load = collect_only!(program, Node::LocalLoad(load) => Some(load));
assert_eq!(load.target_ownership, LoadAsP::Move);
```

Right (property checked inline):
```rust
collect_only!(
  program,
  Node::LocalLoad(LocalLoadSE {
    target_ownership: LoadAsP::Move,
    ..
  }) => Some(())
);
```
