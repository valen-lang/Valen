
possibly now:
- rename everything to Ty instead of Kind


exp-2: Add group borrowing (IN PROGRESS: rung 0 + rung 1 joint-argument check landed)
Me, to support:
 * DeferTE

exp-3: Resurrect the backend.
Me, to support:
 * DeferTE

exp-4: Rust interop


Nobilia: make stuff awesome


eventually:
- Simplify tests, they each make arenas, and they have weird string literals
- LLDB support
- more apps:
  - subterfuge clone
  - tactics game
  - veraph editor



smaller TODO:

  get rid of all concat! in tests

  UpgradeWeakTE actually returns an option<borrow ref>. if the user wants to go from share ref to weak ref, we'll chain an UpgradeWeakTE with a BorrowToShareTE.

  AliasTE should go away
  BorrowToShareTE should be a thing, turns a borrow ref into a share ref
  ShareToBorrowTE should be a thing
  BorrowToWeakTE should exist
  UpgradeWeakTE should exist
  ShareToWeakTE should exist

  we'll have a DerefTE which dereferences a double borrow ref.

  have HeapOwnToBorrowTE

  no BorrowToOwnTE.
  we'll still have CopyPrimTE for primitives.
