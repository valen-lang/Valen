// From Frontend/InstantiatingPass/src/dev/vale/instantiating/ast/
//
// Only `types` is registered here for now — the other files in this directory
// (ast.rs, citizens.rs, expressions.rs, hinputs.rs, names.rs, templata.rs,
// templata_utils.rs) are mid-migration and not yet build-ready. The simplifying
// pass needs the I-suffix output enums (MutabilityI, VariabilityI, OwnershipI,
// LocationI) from types.rs as inputs; everything else stays excluded until
// the instantiating migration finishes its current sweep.

pub mod types;
pub mod hinputs;
pub mod names;
