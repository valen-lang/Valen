
## Allowed differences

No need to mention any of these.

### GC vs Arc

The Scala version used GC, because it's Scala. Rust doesn't have GC.

Because of this, anything that we need shared should have an Arc around it. This includes:

 * Interner, Keywords.
 * Interned values, like StrI, FileCoordinate, PackageCoordinate.

However, Arc<Mutex<T>> is a code smell. It's invalid, unless there is a comment above the T describing why it's okay to be Arc-Mutex'd. AI isn't allowed to write those, only users.

### Clone

Cloning is sometimes needed in Rust when it's not needed in Scala.

However, only do this on value types, nothing that might ever be mutated.

### Scala traits vs Rust enums

It's fine if Rust uses enums for Scala sealed traits.

Though, they should contain the same things.

### Box

Box is sometimes needed in Rust, that's fine.

### Minor allowed differences

These aren't worth mentioning:

 * Ignore Scala Profiler.frame() calls.
 * Scala StringBuilder is equivalent to Rust String::new()/push().
 * Scala vimpl() is equivalent to Rust panic!().
 * Scala vassert() is equivalent to Rust assert!()
 * Scala's iter.code() is equivalent to Rust iter.code.chars().nth()
 * Scala's iter.code.slice(begin, end) is equivalent to Rust's &iter.code[begin as usize..end as usize]
 * Scala's vassertSome is equivalent to Rust .expect()
 * If there's an unused Scala variable, and Rust also has it but underscored, that's fine.
 * Scala's Accumulator is equivalent to Rust Vec
 * Scala's Either vs equivalent custom Rust enums
 * Using multi-line strings vs single-line strings.
 * Using triple-quote strings instead of single-quote strings.
 * Checking via `match` statement vs `assert!(matches!(...` statement, as long as they're doing logically equivalent checks.
 * Method of asserting length, like asserting args.len() == 3 vs Scala pattern Vector(_, _, _).