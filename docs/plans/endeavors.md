
**Project A:**

On Vale master:

1. pull in Frontend/ changes, with any necessary docs
2. pull in FrontendRust/, with any necessary docs
3. pull in CoordinatorRust/ and TesterRust/
4. Make a new branch called rust-migration as a snapshot of this state in time
5. Still for master, strip out all scala block comments in FrontendRust/
6. Delete Frontend/ and Coordinator/ and Tester/

Master is now fully rust migrated.


**INPROGRESS Project C2:** Cut away mutability
After B.
Shared types should no longer be *deeply* immutable.
This might have some interactions with our externs, which tend to only pass immutable things. lets look into that.
Most of this work will probably be in the frontend.

**DONE Project C1:** Take fearless FFI out of the backend.
After B. Destined for mirai-main.


C1/2/3/4 will be removing things, which can be done in parallel with each other and with project D and E.
B + C1/2/3/4 should land in a new branch called mirai-base


**Project D:** Add inline data.
After B. Destined for mirai-main.

right now, everything in vale is on the heap, we dont have inline data yet. lets fix that. i also want to move away from a java-ish Coord-based world, and move words a rust-like onion typing world.

1. make Type enum that first has only one variant Coord, then has all Kind entries. raw kinds will become inline things
  - make Coord into a PtrType, and remove hammer
  - rejected alternative: remove coord in favor of a struct like Pointer. lets not do that, lets keep refs first class.
2. simplify/remove highertyping solving since there's just Kind now
3. simplify the solver, hopefully we can get rid of a lot of complexsolve stuff


**Project E:** Add group borrowing
After B. Destined for mirai-main.

1. Region is still part of Coord (later PtrType), which is fine. but it should contain a region path, according to the group borrowing article and the google docs.


**Project F:** Basic new Rust interop
After B. Destined for mirai-main.
We'll add the new rust interop, with the per_instance_mir.


**Project F:** Complex new Rust interop
After D and F. Destined for mirai-main.
Once we have inline data (from D) we'll be able to do some really good rust interop stuff.

1. make sure it works well, fill in any missing gaps.
2. make a Subterfuge clone
3. make native version of domino
   - Then make a tactics game
   - Then make a roguelike game, with an epic iceball effect to domino
   - Then make a veraph editor
   - Then port geomancer


**Project G:** Debugger support
After A.
Just adding lldb support (dwarf stuff?).

1. Figure out a testing strategy that lets our tests launch lldb.
2. Basic breakpoints and printing values
3. Everything else


**Project H:** Fix broken tests
After A, for master.
The rust migration left some tests as broken, marked ~ in migrate-tl.md, we should fix those.


**Project I:** Improve FrontendRust/ tests
After A, for master.
Simplify FrontendRust/ tests, they each make arenas, and they have weird string literals

