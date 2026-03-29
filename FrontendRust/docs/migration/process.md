
# P1: Do not use scripts (DNUS)

Please do not use scripts to update the code, prefer edit, search_replace, and your other own direct editing tools.


# P2: Rust Code Should Be Above its Scala Code (RCSBASC)

I've left the old Scala code in as comments.

IMPORTANT: For every new Rust definition (function, type, etc.), put it directly above the old Scala definition comment. New Rust definitions should be interleaved with old Scala definition comments.

IMPORTANT: Do not change or remove any Scala comments. But feel free to split any comment into two comments so you can put rust code between them.

If there's no equivalent Scala code, please write a "// NOVEL CODE" comment and explain what the closest equivalent Scala code in the old compiler was. You can find the old compiler in /Frontend.

Ensure that each Rust definition is either above its corresponding old Scala definition comment, or preceded with a `// NOVEL CODE` comment.


# P3: All tests are extremely important and should pass (ATEISP)

Dont assume that any tests are unimportant or unnecessary. They are all extremely important.

Ensure that all the Scala tests have corresponding Rust tests.


# P4: Don't make temporary programs (DMTP)

If trying to debug, please dont make new programs. just use the existing tests to see whatas happening, adding debug output to only the compiler itself if necessary.


# P5: If you notice inconsistencies, stop and ask (INISA)

If you notice any inconsistencies between the rust and scala versions, stop and let me know.


# P6: There are no valid simplifications, no excuses (NVSE)

Don't assume something is a "valid simplification for migration purposes", and don't assume that we can make up something that's simpler because we're migrating piece by piece. Port the Scala code exactly. Don't take shortcuts like that.


# P7: New files should be inspired by ones in the original Scala (NFIOS)

When you make new files, make sure that it's inspired by a corresponding file in the original Scala.


# P8: No expensive clones (NEC)

Stop and ask the human when you're about to implement Clone for a potentially large data structure.


# P9: Scala sealed traits to Rust enums (SSTRE)

Default to making Scala sealed traits into Rust enums, but it's also fine if you instead want to make them into Rust traits.

If you make them into Rust enums, and the enums have fields in them, adhere to ESCCD. Enums themselves should never be interned; only their contents should be interned.



# P10: Port structure exactly (PSE)

Port the structure exactly as it is in Scala, with panics for the parts that aren't implemented yet.


# P11: Returning Result is Fine (RRIF)

Scala threw exceptions whenever it encountered an error. Rust should instead return Result. Because of this, a vast number of functions in Rust will have Result, because one of their indirect callees is returning a Result. This is fine.
