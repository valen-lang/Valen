// From Frontend/FinalAST/src/dev/vale/finalast/ast.scala
//
// H-side output AST types for the simplifying (hammer) pass. Mirrors
// src/instantiating/ast/ast.rs pattern: Temporary-state structs hold real
// fields where downstream code uses them; otherwise bare-placeholder
// PhantomData shells with `<'s, 'h>` lifetimes for forward compatibility.
//
// IdH is currently a bare-placeholder (same precedent as IdI in instantiating);
// real fields restored when slice-pipeline reaches it.

#[allow(unused_imports)]
use std::marker::PhantomData;

use crate::interner::StrI;
use crate::final_ast::types::*;
use crate::final_ast::instructions::ExpressionH;
use crate::simplifying::hammer_interner::MustIntern;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::final_ast::types::InterfaceHT;
use crate::final_ast::types::InterfaceHTValH;
use crate::final_ast::types::StructHT;
use crate::final_ast::types::StructHTValH;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::code_hierarchy::PackageCoordinateMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ptr::eq;



// mig: case class RegionH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionH;


// mig: case class Export
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Export<'s, 'h> where 's: 'h {
    pub name_h: &'h IdH<'s>,
    pub exported_name: StrI<'s>,
}


// mig: case class PackageH
/// Temporary state
//
// PartialEq/Eq/Hash dropped because ArenaIndexMap doesn't implement them
// (also matches Scala's `override def equals(obj: Any): Boolean = vcurious()`).
#[derive(Copy, Clone, Debug)]
pub struct PackageH<'s, 'h> where 's: 'h {
    pub interfaces: &'h [InterfaceDefinitionH<'s, 'h>],
    pub structs: &'h [StructDefinitionH<'s, 'h>],
    pub functions: &'h [FunctionH<'s, 'h>],
    pub static_sized_arrays: &'h [StaticSizedArrayDefinitionHT<'s, 'h>],
    pub runtime_sized_arrays: &'h [RuntimeSizedArrayDefinitionHT<'s, 'h>],
    pub export_name_to_function: &'h ArenaIndexMap<'h, StrI<'s>, &'h PrototypeH<'s, 'h>>,
    pub export_name_to_kind: &'h ArenaIndexMap<'h, StrI<'s>, KindHT<'s, 'h>>,
    pub prototype_to_extern: &'h ArenaIndexMap<'h, &'h PrototypeH<'s, 'h>, HamutsFunctionExtern<'s, 'h>>,
    pub kind_to_extern: &'h ArenaIndexMap<'h, &'h OpaqueHT<'s, 'h>, HamutsKindExtern<'s, 'h>>,
}

// mig: fn extern_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn extern_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: extern_functions");
  }

// mig: fn abstract_functions
  pub fn abstract_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: abstract_functions");
  }

// mig: fn get_all_user_implemented_functions
  pub fn get_all_user_implemented_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: get_all_user_implemented_functions");
  }

// mig: fn non_extern_functions
  pub fn non_extern_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: non_extern_functions");
  }

// mig: fn get_all_user_functions
  pub fn get_all_user_functions(&self) -> Vec<&FunctionH<'s, 'h>> {
    self.functions.iter().filter(|f| f.is_user_function()).collect()
  }

// mig: fn lookup_function
  pub fn lookup_function(&self, readable_name: &str) -> &'h FunctionH<'s, 'h> {
    let from_exports: Vec<&'h PrototypeH<'s, 'h>> = self.export_name_to_function.iter().filter(|(k, _)| k.0 == readable_name).map(|(_, v)| *v).collect();
    let from_functions: Vec<&'h PrototypeH<'s, 'h>> = self.functions.iter().filter(|f| f.prototype.id.local_name.0 == readable_name).map(|f| f.prototype).collect();
    let mut matches: Vec<&'h PrototypeH<'s, 'h>> = Vec::new();
    for p in from_exports.into_iter().chain(from_functions.into_iter()) {
        if !matches.iter().any(|q| eq(*q as *const _, p as *const _)) {
            matches.push(p);
        }
    }
    assert!(!matches.is_empty());
    assert!(matches.len() <= 1);
    let first = matches[0];
    self.functions.iter().find(|f| eq(f.prototype as *const _, first as *const _)).expect("lookup_function: function with matching prototype")
  }

// mig: fn lookup_struct
  pub fn lookup_struct(&self, human_name: &str) -> &'h StructDefinitionH<'s, 'h> {
    let matches: Vec<&StructDefinitionH<'s, 'h>> = self.structs.iter().filter(|s| s.id.local_name.0 == human_name).collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }

// mig: fn lookup_interface
  pub fn lookup_interface(&self, human_name: &str) -> &'h InterfaceDefinitionH<'s, 'h> {
    let matches: Vec<&InterfaceDefinitionH<'s, 'h>> = self.interfaces.iter().filter(|i| i.id.shortened_name.0 == human_name).collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
}


// mig: case class ProgramH
/// Temporary state
pub struct ProgramH<'s, 'h> where 's: 'h {
    pub packages: PackageCoordinateMap<'s, PackageH<'s, 'h>>,
}

// mig: fn lookup_package
impl<'s, 'h> ProgramH<'s, 'h> where 's: 'h {
  pub fn lookup_package(&self, package_coordinate: PackageCoordinate<'s>) -> PackageH<'s, 'h> {
    *self.packages.get(&package_coordinate).expect("lookup_package: missing")
  }

// mig: fn lookup_function
    pub fn lookup_function(&self, prototype: &PrototypeH<'s, 'h>) -> &'h FunctionH<'s, 'h> {
        let paackage = self.lookup_package(prototype.id.package_coordinate);
        let result = paackage.functions.iter().find(|f| f.prototype.id == prototype.id).expect("lookup_function: missing");
        assert!(prototype == result.prototype);
        result
    }

// mig: fn lookup_struct
    pub fn lookup_struct(&self, interner: &HammerInterner<'s, 'h>, struct_ref_h: &StructHT<'s, 'h>) -> &'h StructDefinitionH<'s, 'h> {
        let paackage = self.lookup_package(struct_ref_h.id.package_coordinate);
        paackage.structs.iter().find(|s| *s.get_ref(interner) == *struct_ref_h).expect("lookup_struct: missing")
    }

// mig: fn lookup_interface
    pub fn lookup_interface(&self, interner: &HammerInterner<'s, 'h>, interface_ref_h: &InterfaceHT<'s, 'h>) -> &'h InterfaceDefinitionH<'s, 'h> {
        let paackage = self.lookup_package(interface_ref_h.id.package_coordinate);
        paackage.interfaces.iter().find(|i| *i.get_ref(interner) == *interface_ref_h).expect("lookup_interface: missing")
    }

// mig: fn lookup_static_sized_array
    pub fn lookup_static_sized_array(&self, ssa_th: &StaticSizedArrayHT<'s, 'h>) -> &'h StaticSizedArrayDefinitionHT<'s, 'h> {
        let paackage = self.lookup_package(ssa_th.id.package_coordinate);
        paackage.static_sized_arrays.iter().find(|s| s.name == ssa_th.id).expect("vassertSome: lookup_static_sized_array")
    }

// mig: fn lookup_runtime_sized_array
    pub fn lookup_runtime_sized_array(&self, rsa_th: &RuntimeSizedArrayHT<'s, 'h>) -> &'h RuntimeSizedArrayDefinitionHT<'s, 'h> {
        let paackage = self.lookup_package(rsa_th.name.package_coordinate);
        paackage.runtime_sized_arrays.iter().find(|s| s.name == rsa_th.name).expect("vassertSome: lookup_runtime_sized_array")
    }
}


// mig: case class StructDefinitionH
/// Temporary state
//
// PartialEq/Eq/Hash dropped because the edges field's EdgeH transitively
// holds an ArenaIndexMap (which lacks those impls). Matches Scala's `vcurious`.
#[derive(Copy, Clone, Debug)]
pub struct StructDefinitionH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub weakable: bool,
    pub extern_: bool,
    pub mutability: Mutability,
    pub edges: &'h [EdgeH<'s, 'h>],
    pub members: &'h [StructMemberH<'s, 'h>],
}
// mig: fn get_ref (on StructDefinitionH — see Scala `def getRef: StructHT = StructHT(id)` in the StructDefinitionH audit block below)
impl<'s, 'h> StructDefinitionH<'s, 'h> where 's: 'h {
    pub fn get_ref(&self, interner: &HammerInterner<'s, 'h>) -> &'h StructHT<'s, 'h> {
        interner.intern_struct_ht(StructHTValH { id: self.id })
    }
}


// mig: case class StructMemberH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructMemberH<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s>,
    pub variability: Variability,
    pub tyype: CoordH<'s, 'h>,
}


// mig: case class InterfaceDefinitionH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceDefinitionH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub weakable: bool,
    pub mutability: Mutability,
    pub super_interfaces: &'h [&'h InterfaceHT<'s, 'h>],
    pub methods: &'h [InterfaceMethodH<'s, 'h>],
}
// mig: fn get_ref (on InterfaceDefinitionH — see Scala `def getRef = InterfaceHT(id)` in audit-trail below)
impl<'s, 'h> InterfaceDefinitionH<'s, 'h> where 's: 'h {
    pub fn get_ref(&self, interner: &HammerInterner<'s, 'h>) -> &'h InterfaceHT<'s, 'h> {
        interner.intern_interface_ht(InterfaceHTValH { id: self.id })
    }
}


// mig: case class InterfaceMethodH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceMethodH<'s, 'h> where 's: 'h {
    pub prototype_h: &'h PrototypeH<'s, 'h>,
    pub virtual_param_index: i32,
}


// mig: case class EdgeH
/// Temporary state
//
// PartialEq/Eq/Hash dropped because ArenaIndexMap doesn't implement them
// (also matches Scala's `override def equals(obj: Any): Boolean = vcurious()`).
#[derive(Copy, Clone, Debug)]
pub struct EdgeH<'s, 'h> where 's: 'h {
    pub struct_: &'h StructHT<'s, 'h>,
    pub interface: &'h InterfaceHT<'s, 'h>,
    pub struct_prototypes_by_interface_method: &'h ArenaIndexMap<'h, InterfaceMethodH<'s, 'h>, &'h PrototypeH<'s, 'h>>,
}


// mig: sealed trait IFunctionAttributeH + case objects UserFunctionH, PureH
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionAttributeH {
    UserFunctionH,
    PureH,
}


// mig: case class FunctionH
/// Temporary state
//
// Drops PartialEq/Eq/Hash because the `body: ExpressionH` field opts out
// (per typing-pass parity — Scala uses `vcurious` on FunctionH).
#[derive(Copy, Clone, Debug)]
pub struct FunctionH<'s, 'h> where 's: 'h {
    pub prototype: &'h PrototypeH<'s, 'h>,
    pub is_abstract: bool,
    pub is_extern: bool,
    pub attributes: &'h [IFunctionAttributeH],
    pub body: ExpressionH<'s, 'h>,
}

// mig: fn is_user_function
impl<'s, 'h> FunctionH<'s, 'h> where 's: 'h {
    pub fn is_user_function(&self) -> bool {
        self.attributes.contains(&IFunctionAttributeH::UserFunctionH)
    }
}


// mig: case class PrototypeH
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub params: &'h [CoordH<'s, 'h>],
    pub return_type: CoordH<'s, 'h>,
    pub _must_intern: MustIntern,
}


// mig: case class PrototypeH (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeHValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub params: &'h [CoordH<'s, 'h>],
    pub return_type: CoordH<'s, 'h>,
}

// mig: case class IdH
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdH<'s> {
    pub local_name: StrI<'s>,
    pub package_coordinate: PackageCoordinate<'s>,
    pub shortened_name: StrI<'s>,
    pub fully_qualified_name: StrI<'s>,
    pub _must_intern: MustIntern,
}
// Realizes Scala's case-class auto-toString for IdH:
//   IdH(<local_name>,<package_coordinate>,<shortened_name>,<fully_qualified_name>)
// Per Scala convention: no space between case-class fields. StrI fields print as bare strings
// per Rust StrI's existing Display canon (interner.rs:40); Scala would wrap each as StrI(<v>).
impl<'s> Display for IdH<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "IdH({},{},{},{})", self.local_name, self.package_coordinate, self.shortened_name, self.fully_qualified_name)
    }
}



// mig: case class IdH (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdHValH<'s> {
    pub local_name: StrI<'s>,
    pub package_coordinate: PackageCoordinate<'s>,
    pub shortened_name: StrI<'s>,
    pub fully_qualified_name: StrI<'s>,
}

// --- Auxiliary types referenced by hammer files ---

// FunctionRefH lives in Scala's `Hammer.scala`, but it's a pure data type
// (just a wrapper around a PrototypeH ref) so we colocate it with the H-side
// AST here. Not interned in Scala (no `MustIntern` needed).
/// Temporary state
#[derive(Copy, Clone, Debug)]
pub struct FunctionRefH<'s, 'h> where 's: 'h {
    pub prototype: &'h PrototypeH<'s, 'h>,
}

// VariableIdH and Local live in instructions.rs (per Scala instructions.scala layout).
