
Migration guidelines:

# 1. Rust should mirror Scala as close as possible

Keep making sure that everything in the rust version mirrors almost exactly whats in the scala version. down to the functions, their positions relative to each other, their names, their logic, and if possible variable names too.

# 2. Do not use scripts

Please do not use scripts to update the code, prefer edit, search_replace, and your other own direct editing tools.

# 3. You can temporarily leave TODOS + things unimplemented, if they panic nicely

If you must leave todos or unimplemented things, please make them panic with a unique message that will make it immediately clear when failures are from not-yet-brought-over code.

# 4. Provenance

I've left the old Scala code in as comments. For any new piece of Rust, try to put it as close as possible to the old Scala comment.

If there's no equivalent Scala code, please write a "// NOVEL CODE" comment and explain what the closest equivalent Scala code in the old compiler was. You can find the old compiler in /Frontend.

# 5. Don't conveniently change test expectations

If a test fails, never make the test expect the current behavior (that defeats the entire purpose of tests). instead, figure out where the rust version's logic doesn't match the scala version's logic, and make it more consistent. the scala tests pass. if a couple attempts don't make it work, stop and ask me for help.

# 6. All tests are extremely important and should pass

Dont assume that any tests are unimportant or unnecessary. They are all extremely important.

# 7. Don't make temporary programs

If trying to debug, please dont make new programs. just use the existing tests to see whatas happening, adding debug output to only the compiler itself if necessary.

8. Just matching behavior isn't enough

Don't just make the behavior match. do not take liberties with what should be ported over. if you think something isnt needed yet, then leave a panic!() there. keep translating things over 1:1, even if its a lot, until we have enough to accomplish our runs.

# 9. If you notice inconsistencies, stop and ask

If you notice any inconsistencies between the rust and scala versions, stop and let me know.

# 10. There are no valid simplifications, no excuses

Don't assume something is a "valid simplification for migration purposes", and don't assume that we can make up something that's simpler because we're migrating piece by piece. Port the Scala code exactly. Don't take shortcuts like that.

# 11. New files should be inspired by ones in the original Scala

When you make new files, make sure that it's inspired by a corresponding file in the original Scala.

# 12. No expensive clones

Stop and ask the human when you're about to implement Clone for a potentially large data structure.

# 13. Scala sealed traits to Rust enums

Default to making Scala sealed traits into Rust enums, but it's also fine if you instead want to make them into Rust traits.

If you make them into Rust enums, be sure to adhere to ESCCD.

14. Port structure exactly

Port the structure exactly as it is in Scala, with panics for the parts that aren't implemented yet.
