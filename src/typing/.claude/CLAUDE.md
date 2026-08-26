# Typing Pass

## The "Core"

Typing pass is part of the "core", and AI is not allowed to edit the core compiler.

Exceptions (AI is allowed to edit these parts of the typing pass):
* `rust_interop` directories.
* `borrow_checker` directories.
* `macros`

AI can edit these if the user explicitly authorizes specific edits by saying "fire core edits".
