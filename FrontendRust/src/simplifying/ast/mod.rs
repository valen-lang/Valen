// From Frontend/FinalAST/src/dev/vale/finalast/
//
// The H-suffix output AST for the simplifying (hammer) pass. Each module here
// mirrors one source file in Frontend/FinalAST/src/dev/vale/finalast/:
//  - `types`   ← types.scala
//
// More modules (ast, instructions) will be added as the simplifying-pass
// migration progresses. For now `types` is the minimum needed to unblock
// `src/simplifying/conversions.rs`. Per migration-policy.md, the simplifying
// pass's interner type and default map type are still TBD; the H-side types
// scaffolded here use `()`-shaped placeholder bodies that will be filled in
// when the architect signs off on those policy cells.

pub mod types;
pub mod ast;
pub mod hamuts;
pub use types::*;
pub use ast::*;
pub use hamuts::*;
