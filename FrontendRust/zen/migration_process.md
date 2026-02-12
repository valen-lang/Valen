
# P1: Do not use scripts

Please do not use scripts to update the code, prefer edit, search_replace, and your other own direct editing tools.


# P2: Provenance

I've left the old Scala code in as comments. For any new piece of Rust, try to put it as close as possible to the old Scala comment.

If there's no equivalent Scala code, please write a "// NOVEL CODE" comment and explain what the closest equivalent Scala code in the old compiler was. You can find the old compiler in /Frontend.

Ensure that all Rust code is either above its corresponding old Scala code, or preceded with a `// NOVEL CODE` comment.


# P3: All tests are extremely important and should pass

Dont assume that any tests are unimportant or unnecessary. They are all extremely important.

Ensure that all the Scala tests have corresponding Rust tests.


# P4: Don't make temporary programs

If trying to debug, please dont make new programs. just use the existing tests to see whatas happening, adding debug output to only the compiler itself if necessary.


# P5: If you notice inconsistencies, stop and ask

If you notice any inconsistencies between the rust and scala versions, stop and let me know.


# P6: There are no valid simplifications, no excuses

Don't assume something is a "valid simplification for migration purposes", and don't assume that we can make up something that's simpler because we're migrating piece by piece. Port the Scala code exactly. Don't take shortcuts like that.


# P7: New files should be inspired by ones in the original Scala

When you make new files, make sure that it's inspired by a corresponding file in the original Scala.


# P8: No expensive clones

Stop and ask the human when you're about to implement Clone for a potentially large data structure.


# P9: Scala sealed traits to Rust enums

Default to making Scala sealed traits into Rust enums, but it's also fine if you instead want to make them into Rust traits.

If you make them into Rust enums, and the enums have fields in them, adhere to ESCCD.


# P10: Port structure exactly

Port the structure exactly as it is in Scala, with panics for the parts that aren't implemented yet.

