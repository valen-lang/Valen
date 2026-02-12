## Warnings

Your work is not done if there are still warnings. Make sure there are no warnings before saying you're done. Warnings are very important to eliminate.

## Interning

There are various classes that are interned.
For example, StrI, PackageCoordinate, FileCoordinate, etc.
These can be identified by a comment above them saying that they're interned.
These should always immediately be interned, just after creation. A function should never store a StrI directly in a struct, and should never return a StrI. A bare StrI should only exist very temporarily and immediately be handed to the Interner.

Equality can be done directly on the Arc instances, like `arc_a == arc_b`.
