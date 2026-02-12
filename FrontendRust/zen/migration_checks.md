
# M1: Rust should mirror Scala as close as possible

Keep making sure that everything in the rust version mirrors almost exactly whats in the scala version. Down to the functions, their positions relative to each other, their names, their logic, and if possible variable names too.


# M2: TODOS + unimplemented code MUST panic

If you must leave todos or unimplemented things, ensure they panic with a unique message that will make it immediately clear when failures are from not-yet-brought-over code.


# M3: Don't conveniently change requirements

If the implementation has a bug, or a test fails, do not change the requirements of the implementation or test.

For example, if a test fails, never make the test expect the current bad behavior (that defeats the entire purpose of tests). The Scala tests all passed. The Rust tests should pass, and they should expect the exact same behavior the Scala tests did.

For example, if the implementation isn't working right, don't change the code or comments to be okay with it. Do not take liberties with what should be ported over. If you think something isnt needed yet, then leave a panic!() there.

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
