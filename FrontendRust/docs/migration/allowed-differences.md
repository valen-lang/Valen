# Allowed Scala→Rust Differences

Differences that are expected during migration and should **not** be flagged as divergences.

## Memory / ownership

- **GC → Arc**: shared things (interner, keywords, interned values like `StrI`/`FileCoordinate`/`PackageCoordinate`) get an `Arc`. `Arc<Mutex<T>>` is a code smell — invalid unless a human-written comment above `T` justifies it. AI may not write those.
- **Clone**: sometimes needed where Scala didn't need it, but only on value types — never anything that might be mutated.
- **Box**: fine where Rust needs it (e.g. recursive types).
- Lifetime / borrowing / ownership changes and value↔reference changes are always allowed (Rust's memory model, not a logic change).

## Sealed traits

Rust may use an `enum` for a Scala `sealed trait` — but it must contain the same variants.

## Minor (not worth mentioning)

- `Profiler.frame()` calls dropped.
- Scala `StringBuilder` ≈ Rust `String::new()` / `push`.
- `vimpl()` ≈ `panic!()`; `vassert()` ≈ `assert!()`; `vassertSome` ≈ `.expect()`.
- `iter.code()` ≈ `iter.code.chars().nth()`; `iter.code.slice(b, e)` ≈ `&iter.code[b as usize..e as usize]`.
- Scala `Accumulator` ≈ Rust `Vec`; Scala `Either` ≈ custom Rust enums.
- Unused Scala variable that's underscored in Rust.
- Multi-line vs single-line / triple-quote vs single-quote strings.
- `match` vs `assert!(matches!(...))` when logically equivalent.
- Asserting length (`args.len() == 3`) vs Scala pattern `Vector(_, _, _)`.
