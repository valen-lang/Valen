
# Don't conveniently change requirements (DCCR)

If the implementation has a bug, or a test fails, do not change the requirements of the implementation or test.

For example, if a test fails, never make the test expect the current bad behavior (that defeats the entire purpose of tests). The Scala tests all passed. The Rust tests should pass, and they should expect the exact same behavior the Scala tests did.

For example, if the implementation isn't working right, don't change the code or comments to be okay with it. Do not take liberties with what should be ported over. If you think something isnt needed yet, then leave a panic!() (or assert) there.

Figure out where the Rust version's logic doesn't match the scala version's logic, and make it more consistent.

Ensure that all Rust code/test requirements exactly match the old Scala code/test requirements.

**Architect-level escape hatch.** When the Scala carries dead-weight machinery whose mutation/dispatch surface is unused on the call paths being ported (`FunctionEnvironmentBoxT`'s `setReturnType`/`addEntry` mutators on read-only paths is the canonical case), don't write the Rust without it and add a "diverges from Scala" note — those rot, and reviewers can't verify the divergence. Instead, edit the Scala source first to match what the Rust will become, update the Rust audit-trail `/* ... */` blocks to reflect the new Scala, then make the Rust change. Only valid when the wrapper is unused on the ported paths (verify with `grep`), the replacement is design-doc-blessed, and SCPX `--check-all` still passes after. **TL/architect-level only — juniors must escalate.**


# P2: Rust Code Should Be Above its Scala Code (RCSBASC)

I've left the old Scala code in as comments.

IMPORTANT: For every new Rust definition (function, type, etc.), put it directly above the old Scala definition comment. New Rust definitions should be interleaved with old Scala definition comments.

IMPORTANT: Do not change or remove any Scala comments. But feel free to split any comment into two comments so you can put rust code between them.

If there's no equivalent Scala code, please write a "// NOVEL CODE" comment and explain what the closest equivalent Scala code in the old compiler was. You can find the old compiler in /Frontend.

Ensure that each Rust definition is either above its corresponding old Scala definition comment, or preceded with a `// NOVEL CODE` comment.
