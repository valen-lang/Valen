# Typing Pass

## The "Core"

Typing pass is part of the "core", and AI is not allowed to edit the core compiler.

Exceptions (AI is allowed to edit these parts of the typing pass):
* `rust_interop` directories.
* `borrow_checker` directories.
* `macros`

AI can edit these if the user explicitly authorizes specific edits by saying "fire core edits".

AI can add read-only print statements for debugging purposes only, if the user explicitly authorizes by saying "fire core prints".

## Compiler laws

**non-generic is the degenerate case of generic.** Never branch on "does this function/type have type parameters?" A non-generic item is simply one with zero type args — it goes through the same instantiation path as a generic one. Code that special-cases the non-generic path creates false distinctions and latent bugs when items gain type params or the code is reused in a more general context. Always write the general path; zero args is a valid input to it.

**`self` is just another parameter.** Avoid separate code paths for self vs non-self parameters. If syntax separates the receiver (e.g., dot notation), reassemble the full parameter list so downstream logic can handle everything uniformly. Semantically, there is no difference between `self` and non-`self` parameters, so treating them differently is almost always a bug.

**`drop` is just another function.** Avoid separate code paths for drop vs non-drop functions. Semantically, there is no difference between `drop` and non-`drop` parameters, so treating them differently is almost always a bug. The one exception is where the typing pass automatically inserts drop calls at the end of scopes.
