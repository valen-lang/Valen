# FFI Refs Move, Accessors Consume (FRMACZ)

Across the FFI, reference counts are never adjusted *at the boundary itself*. A share ref **moves** into C, the auto-generated accessors **consume** their arguments like any Vale function, and C adjusts the refcount **explicitly** with `_alias`/`_dealias` — where `_alias` returns the same handle so it composes inline.

C's rule is mechanical: **alias a handle each time you pass it to an accessor, and dealias each handle you still own once you're done** — except one you return, which moves out.

```c
int describe(vtest_Ship ship) {                          // ship moved in — C owns it
  int a = vtest_Ship_altitude(vtest_Ship_alias(ship));   // alias for the pass; the getter consumes it
  int f = vtest_Ship_fuel(vtest_Ship_alias(ship));       // alias for the pass
  vtest_Ship_dealias(ship);                              // done with our handle
  return a + f;
}
```

`vtest_Ship_altitude` **consumes** its argument — exactly what `func getAltitude(s Ship) int { s.altitude }` compiles to (alias the returned member, dealias the struct). Because C aliases at every pass, each accessor gets its own count and `ship` stays live no matter what order its reads evaluate in; the single trailing `_dealias` discharges the ownership that moved in. Following the rule mechanically means never reasoning about which use is the "last" one, or how a C compiler orders subexpressions.

**Why the boundary is silent.** An `extern func` call moves its argument into C the same way an ordinary call moves an argument to its callee. In:

```vale
extern func cStow(s Ship);   // C takes ownership of s
exported func main() {
  cStow(Ship(100));          // the fresh Ship's +1 moves straight into C
}
```

`Ship(100)` is created with one owned reference, and the call hands it to C with no alias — exactly as `someValeFunc(Ship(100))` would hand it to a Vale callee. (If `s` were reused after the call, normal codegen would alias it first, +1, again just like any call.) We keep reference counting out of the boundary because it's a **simpler mental model**: every +1/-1 is either an ordinary Vale operation inside a function body, or an explicit call in C — there's no third set of hidden rules in the boundary glue. It's a deliberate choice, not a permanent one; a more robust scheme (say, a boundary that tracks ownership itself) could replace it later.

The handles being passed around are themselves @HTSLVBDTCZ. RC balance is checked by building under `--census` (`VALE_TEST_CENSUS=1`), which counts live heap objects and asserts zero at program exit.
