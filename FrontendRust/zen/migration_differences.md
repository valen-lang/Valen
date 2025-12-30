## GC vs Arc

The Scala version used GC, because it's Scala. Rust doesn't have GC.

Because of this, anything that we need shared should have an Arc around it. This includes:

 * Interner, Keywords.
 * Interned values, like StrI, FileCoordinate, PackageCoordinate.

However, Arc<Mutex<T>> is a code smell. It's invalid, unless there is a comment above the T describing why it's okay to be Arc-Mutex'd. AI isn't allowed to write those, only users.

## Clone

Cloning is sometimes needed in Rust when it's not needed in Scala.

However, only do this on value types, nothing that might be mutated.