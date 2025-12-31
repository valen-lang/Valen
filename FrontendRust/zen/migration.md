
and also remember:
1. keep making sure that everything in the rust version mirrors almost exactly whats in the scala version. down to the functions, their positions relative to each other, their names, their logic, and if possible variable names too.
2. please do not use scripts to update the code, prefer edit, search_replace, and your other own direct editing tools.
3. if you must leave todos or unimplemented things, please make them panic with a unique message that will make it immediately clear when failures are from not-yet-brought-over code.
4. as you bring code over, please leave comments saying where in the scala code they come from.
5. if a test fails, never make the test expect the current behavior (that defeats the entire purpose of tests). instead, figure out where the rust version's logic doesn't match the scala version's logic, and make it more consistent. the scala tests pass. if a couple attempts don't make it work, stop and ask me for help.
6. dont assume that any tests are unimportant or unnecessary. they are all extremely important.
7. if trying to debug, please dont make new programs. just use the existing tests to see whatas happening, adding debug output to only the compiler itself if necessary.
8. dont just make the behavior match. do not take liberties with what should be ported over. if you think something isnt needed yet, then leave a panic!() there. keep translating things over 1:1, even if its a lot, until we have enough to accomplish our runs.
9. if you notice any inconsistencies between the rust and scala versions, stop and let me know.
10. don't assume something is a "valid simplification for migration purposes", and don't assume that we can make up something that's simpler because we're migrating piece by piece. Port the Scala code exactly. Don't take shortcuts like that.
11. When you make new files, make sure that it's inspired by a corresponding file in the original Scala.
12. Stop and ask the human when you're about to implement Clone for a potentially large data structure.
13. Default to making Scala sealed traits into Rust enums, but it's also fine if you instead want to make them into Rust traits.
14. Port the structure exactly as it is in Scala, with panics for the parts that aren't implemented yet.
