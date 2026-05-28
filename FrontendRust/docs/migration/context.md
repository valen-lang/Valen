---
g_read_when: "Read when working on the Scala-to-Rust migration."
g_auto_load_when_editing:
  - FrontendRust/src/**/*.rs
---

# FrontendRust Migration Context

- This part of the codebase is a gradual migration from older Scala Frontend/ codebase to Rust FrontendRust/.
- It's an exact 1:1 translation, down to the type names, function names, and argument names. The only differences have to do with borrow checking.
- Every bit of Rust code is directly above its Scala counterpart comment.
- Scala comments:
   - Keep legacy Scala comment blocks when they still document behavior that Rust code is actively migrating.
   - Do not remove Scala context comments during refactors unless the behavior is fully implemented and validated in Rust.
   - If Scala and Rust appear to disagree, Scala is correct. The old Scala codebase passed all its test and its logic was verified.
