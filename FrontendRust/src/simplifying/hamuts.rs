// From Frontend/SimplifyingPass/src/dev/vale/simplifying/Hamuts.scala
//
// Mutable bookkeeping state threaded through every simplifying-pass translation.
// Per the Slab 17 architect directive, the Rust port mirrors typing pass's
// `CompilerOutputs`: a single mutable struct with HashMap fields and `&mut self`
// methods. There is no HamutsBox/Hamuts split — Scala's `HamutsBox` (the mutable
// wrapper) and `Hamuts` (the immutable value) collapse into one `Hamuts` struct.
// The collapsed struct keeps HamutsBox's mutating API (`&mut self` methods) plus
// its field-getters (`&self` accessors). Scala's immutable `Hamuts.foo`
// functional-update methods are preserved in the audit trail under
// `// mig: ... (collapsed)` markers — they map onto the same `&mut self` methods
// above, so they get no separate Rust fn.

use std::collections::HashMap;

use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::instantiating::ast::types::{
    cI, InterfaceIT, RuntimeSizedArrayIT, StaticSizedArrayIT, StructIT,
};
use crate::instantiating::ast::ast::PrototypeI;
use crate::final_ast::ast::{
    FunctionH, FunctionRefH, InterfaceDefinitionH, PrototypeH, StructDefinitionH,
};
use crate::final_ast::types::{
    HamutsFunctionExtern, HamutsKindExtern, InterfaceHT, KindHT, OpaqueHT,
    RuntimeSizedArrayDefinitionHT, RuntimeSizedArrayHT, SimpleId, StaticSizedArrayDefinitionHT,
    StaticSizedArrayHT, StructHT,
};

/*
package dev.vale.simplifying

import dev.vale.{PackageCoordinate, StrI, vassert, vcurious, vfail, vimpl}
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.von.IVonData
*/

// mig: case class HamutsBox (collapsed into Hamuts; see Slab 17 architect directive)
// Scala's HamutsBox was a mutable wrapper around an immutable Hamuts. Per
// architect directive, the Rust port mirrors typing pass's `CompilerOutputs`:
// a single mutable struct (`Hamuts` below) with HashMap fields and `&mut self`
// methods. No HamutsBox/Hamuts split. The HamutsBox members below become the
// collapsed struct's accessors (`&self`) and mutating methods (`&mut self`).
/*
case class HamutsBox(var inner: Hamuts) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable
*/

// mig: fn package_coord_to_export_name_to_function (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn package_coord_to_export_name_to_function(&self) -> &HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, &'h PrototypeH<'s, 'h>>> {
        &self.package_coord_to_export_name_to_function
    }
}
/*
  def packageCoordToExportNameToFunction: Map[PackageCoordinate, Map[StrI, PrototypeH]] = inner.packageCoordToExportNameToFunction
*/

// mig: fn package_coord_to_export_name_to_kind (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn package_coord_to_export_name_to_kind(&self) -> &HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, KindHT<'s, 'h>>> {
        &self.package_coord_to_export_name_to_kind
    }
}
/*
  def packageCoordToExportNameToKind: Map[PackageCoordinate, Map[StrI, KindHT]] = inner.packageCoordToExportNameToKind
*/

// mig: fn package_coord_to_prototype_to_extern (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn package_coord_to_prototype_to_extern(&self) -> &HashMap<PackageCoordinate<'s>, HashMap<&'h PrototypeH<'s, 'h>, HamutsFunctionExtern<'s, 'h>>> {
        &self.package_coord_to_prototype_to_extern
    }
}
/*
  def packageCoordToPrototypeToExtern: Map[PackageCoordinate, Map[PrototypeH, HamutsFunctionExtern]] = inner.packageCoordToPrototypeToExtern
*/

// mig: fn package_coord_to_kind_to_extern (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn package_coord_to_kind_to_extern(&self) -> &HashMap<PackageCoordinate<'s>, HashMap<&'h OpaqueHT<'s, 'h>, HamutsKindExtern<'s, 'h>>> {
        &self.package_coord_to_kind_to_extern
    }
}
/*
  def packageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]] = inner.packageCoordToKindToExtern
*/

// mig: fn struct_t_to_opaque_h (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn struct_t_to_opaque_h(&self) -> &HashMap<&'i StructIT<'s, 'i, cI>, &'h OpaqueHT<'s, 'h>> {
        &self.struct_t_to_opaque_h
    }
}
/*
  def structTToOpaqueH: Map[StructIT[cI], OpaqueHT] = inner.structTToOpaqueH
*/

// mig: fn struct_t_to_struct_h (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn struct_t_to_struct_h(&self) -> &HashMap<&'i StructIT<'s, 'i, cI>, &'h StructHT<'s, 'h>> {
        &self.struct_t_to_struct_h
    }
}
/*
  def structTToStructH: Map[StructIT[cI], StructHT] = inner.structTToStructH
*/

// mig: fn struct_t_to_struct_def_h (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn struct_t_to_struct_def_h(&self) -> &HashMap<&'i StructIT<'s, 'i, cI>, StructDefinitionH<'s, 'h>> {
        &self.struct_t_to_struct_def_h
    }
}
/*
  def structTToStructDefH: Map[StructIT[cI], StructDefinitionH] = inner.structTToStructDefH
*/

// mig: fn struct_defs (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn struct_defs(&self) -> &Vec<StructDefinitionH<'s, 'h>> {
        &self.struct_defs
    }
}
/*
  def structDefs: Vector[StructDefinitionH] = inner.structDefs
*/

// mig: fn interface_t_to_interface_h (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn interface_t_to_interface_h(&self) -> &HashMap<&'i InterfaceIT<'s, 'i, cI>, &'h InterfaceHT<'s, 'h>> {
        &self.interface_t_to_interface_h
    }
}
/*
  def interfaceTToInterfaceH: Map[InterfaceIT[cI], InterfaceHT] = inner.interfaceTToInterfaceH
*/

// mig: fn interface_t_to_interface_def_h (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn interface_t_to_interface_def_h(&self) -> &HashMap<&'i InterfaceIT<'s, 'i, cI>, InterfaceDefinitionH<'s, 'h>> {
        &self.interface_t_to_interface_def_h
    }
}
/*
  def interfaceTToInterfaceDefH: Map[InterfaceIT[cI], InterfaceDefinitionH] = inner.interfaceTToInterfaceDefH
*/

// mig: fn function_refs (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn function_refs(&self) -> &HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionRefH<'s, 'h>> {
        &self.function_refs
    }
}
/*
  def functionRefs: Map[PrototypeI[cI], FunctionRefH] = inner.functionRefs
*/

// mig: fn function_defs (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn function_defs(&self) -> &HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionH<'s, 'h>> {
        &self.function_defs
    }
}
/*
  def functionDefs: Map[PrototypeI[cI], FunctionH] = inner.functionDefs
*/

// mig: fn static_sized_arrays (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn static_sized_arrays(&self) -> &HashMap<&'i StaticSizedArrayIT<'s, 'i, cI>, StaticSizedArrayDefinitionHT<'s, 'h>> {
        &self.static_sized_arrays
    }
}
/*
  def staticSizedArrays: Map[StaticSizedArrayIT[cI], StaticSizedArrayDefinitionHT] = inner.staticSizedArrays
*/

// mig: fn runtime_sized_arrays (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn runtime_sized_arrays(&self) -> &HashMap<&'i RuntimeSizedArrayIT<'s, 'i, cI>, RuntimeSizedArrayDefinitionHT<'s, 'h>> {
        &self.runtime_sized_arrays
    }
}
/*
  def runtimeSizedArrays: Map[RuntimeSizedArrayIT[cI], RuntimeSizedArrayDefinitionHT] = inner.runtimeSizedArrays
*/

// mig: fn forward_declare_struct (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn forward_declare_struct(&mut self, struct_it: &'i StructIT<'s, 'i, cI>, struct_ref_h: &'h StructHT<'s, 'h>) {
        panic!("Unimplemented: forward_declare_struct");
    }
}
/*
  def forwardDeclareStruct(structIT: StructIT[cI], structRefH: StructHT): Unit = {
    inner = inner.forwardDeclareStruct(structIT, structRefH)
  }
*/

// mig: fn add_struct_originating_from_typing_pass (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_struct_originating_from_typing_pass(&mut self, struct_it: &'i StructIT<'s, 'i, cI>, struct_def_h: StructDefinitionH<'s, 'h>) {
        panic!("Unimplemented: add_struct_originating_from_typing_pass");
    }
}
/*
  def addStructOriginatingFromTypingPass(structIT: StructIT[cI], structDefH: StructDefinitionH): Unit = {
    inner = inner.addStructOriginatingFromTypingPass(structIT, structDefH)
  }
*/

// mig: fn add_opaque (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_opaque(&mut self, struct_it: &'i StructIT<'s, 'i, cI>, opaque_h: &'h OpaqueHT<'s, 'h>) {
        panic!("Unimplemented: add_opaque");
    }
}
/*
  def addOpaque(structIT: StructIT[cI], opaqueH: OpaqueHT): Unit = {
    inner = inner.addOpaque(structIT, opaqueH)
  }
*/

// mig: fn add_struct_originating_from_hammer (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_struct_originating_from_hammer(&mut self, struct_def_h: StructDefinitionH<'s, 'h>) {
        panic!("Unimplemented: add_struct_originating_from_hammer");
    }
}
/*
  def addStructOriginatingFromHammer(structDefH: StructDefinitionH): Unit = {
    inner = inner.addStructOriginatingFromHammer(structDefH)
  }
*/

// mig: fn forward_declare_interface (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn forward_declare_interface(&mut self, interface_it: &'i InterfaceIT<'s, 'i, cI>, interface_ref_h: &'h InterfaceHT<'s, 'h>) {
        panic!("Unimplemented: forward_declare_interface");
    }
}
/*
  def forwardDeclareInterface(interfaceIT: InterfaceIT[cI], interfaceRefH: InterfaceHT): Unit = {
    inner = inner.forwardDeclareInterface(interfaceIT, interfaceRefH)
  }
*/

// mig: fn add_interface (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_interface(&mut self, interface_it: &'i InterfaceIT<'s, 'i, cI>, interface_def_h: InterfaceDefinitionH<'s, 'h>) {
        panic!("Unimplemented: add_interface");
    }
}
/*
  def addInterface(interfaceIT: InterfaceIT[cI], interfaceDefH: InterfaceDefinitionH): Unit = {
    inner = inner.addInterface(interfaceIT, interfaceDefH)
  }
*/

// mig: fn add_static_sized_array (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_static_sized_array(&mut self, ssa_it: &'i StaticSizedArrayIT<'s, 'i, cI>, static_sized_array_definition_th: StaticSizedArrayDefinitionHT<'s, 'h>) {
        panic!("Unimplemented: add_static_sized_array");
    }
}
/*
  def addStaticSizedArray(ssaIT: StaticSizedArrayIT[cI], staticSizedArrayDefinitionTH: StaticSizedArrayDefinitionHT): Unit = {
    inner = inner.addStaticSizedArray(ssaIT, staticSizedArrayDefinitionTH)
  }
*/

// mig: fn add_runtime_sized_array (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_runtime_sized_array(&mut self, rsa_it: &'i RuntimeSizedArrayIT<'s, 'i, cI>, runtime_sized_array_definition_th: RuntimeSizedArrayDefinitionHT<'s, 'h>) {
        panic!("Unimplemented: add_runtime_sized_array");
    }
}
/*
  def addRuntimeSizedArray(rsaIT: RuntimeSizedArrayIT[cI], runtimeSizedArrayDefinitionTH: RuntimeSizedArrayDefinitionHT): Unit = {
    inner = inner.addRuntimeSizedArray(rsaIT, runtimeSizedArrayDefinitionTH)
  }
*/

// mig: fn forward_declare_function (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn forward_declare_function(&mut self, function_ref2: &'i PrototypeI<'s, 'i, cI>, function_ref_h: FunctionRefH<'s, 'h>) {
        assert!(!self.function_refs.contains_key(&function_ref2));
        self.function_refs.insert(function_ref2, function_ref_h);
    }
}
/*
Guardian: temp-disable: SPDMX — Per documented file-top architecture (hamuts.rs lines 4-12, "There is no HamutsBox/Hamuts split"): Scala's HamutsBox.forwardDeclareFunction delegated to inner.forwardDeclareFunction because Scala had a mutable Box wrapping immutable Hamuts; Rust collapsed both into a single mutable Hamuts. Same precedent as add_function_export (line 330) and add_function_extern (line 360) which already have SPDMX temp-disables for this exact pattern. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1676-1780111880574/hook-1676/forward_declare_function--290.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def forwardDeclareFunction(functionRef2: PrototypeI[cI], functionRefH: FunctionRefH): Unit = {
    inner = inner.forwardDeclareFunction(functionRef2, functionRefH)
  }
*/

// mig: fn add_function (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_function(&mut self, function_ref2: &'i PrototypeI<'s, 'i, cI>, function_def_h: FunctionH<'s, 'h>) {
        panic!("Unimplemented: add_function");
    }
}
/*
  def addFunction(functionRef2: PrototypeI[cI], functionDefH: FunctionH): Unit = {
    inner = inner.addFunction(functionRef2, functionDefH)
  }
*/

// mig: fn add_kind_export (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_kind_export(&mut self, kind: KindHT<'s, 'h>, package_coordinate: PackageCoordinate<'s>, exported_name: StrI<'s>) {
        panic!("Unimplemented: add_kind_export");
    }
}
/*
  def addKindExport(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
    inner = inner.addKindExport(kind, packageCoordinate, exportedName)
  }

//  def addKindExtern(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
//    inner = inner.addKindExtern(kind, packageCoordinate, exportedName)
//  }
*/

// mig: fn add_function_export (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_function_export(&mut self, prototype: &'h PrototypeH<'s, 'h>, package_coordinate: PackageCoordinate<'s>, exported_name: StrI<'s>) {
        let export_name_to_function = self.package_coord_to_export_name_to_function.entry(package_coordinate).or_insert_with(HashMap::new);
        if let Some(existing_full_name) = export_name_to_function.get(&exported_name) {
            panic!("Already exported a `{:?}` from package `{:?} : {:?}", exported_name, package_coordinate, existing_full_name);
        }
        export_name_to_function.insert(exported_name, prototype);
    }
}
/*
Guardian: temp-disable: SPDMX — Per documented file-top architecture (hamuts.rs lines 4-12, "There is no HamutsBox/Hamuts split"): Scala's `HamutsBox.addFunctionExport` delegated to `inner.addFunctionExport` because Scala had a mutable Box wrapping immutable Hamuts; Rust collapsed both into a single mutable Hamuts. The "inner" delegation has nowhere to go — the mutator IS the collapsed `Hamuts.addFunctionExport` (architect-blessed god-struct, same pattern as the file's other 14 HamutsBox mutators). SPDMX Exception Q (god-struct merging) applies. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1501-1780107270452/hook-1501/add_function_export--330.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def addFunctionExport(prototype: PrototypeH, packageCoordinate: PackageCoordinate, exportedName: StrI): Unit = {
    inner = inner.addFunctionExport(prototype, packageCoordinate, exportedName)
  }
*/

// mig: fn add_kind_extern (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_kind_extern(&mut self, opaque_h: &'h OpaqueHT<'s, 'h>, simple_id: SimpleId<'s, 'h>, exported_name: StrI<'s>) {
        panic!("Unimplemented: add_kind_extern");
    }
}
/*
  def addKindExtern(opaqueH: OpaqueHT, simpleId: SimpleId, exportedName: String): Unit = {
    inner = inner.addKindExtern(opaqueH, simpleId, exportedName)
  }
*/

// mig: fn add_function_extern (HamutsBox mutator)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn add_function_extern(&mut self, prototype: &'h PrototypeH<'s, 'h>, simple_id: SimpleId<'s, 'h>, exported_name: StrI<'s>) {
        let package_coordinate = prototype.id.package_coordinate;
        let prototype_to_extern = self.package_coord_to_prototype_to_extern.entry(package_coordinate).or_insert_with(HashMap::new);
        if let Some(existing) = prototype_to_extern.get(prototype) {
            panic!("Already exported a `{:?}` from package `{:?} : {:?}", exported_name, package_coordinate, existing);
        }
        prototype_to_extern.insert(prototype, HamutsFunctionExtern { maybe_extern_name: exported_name, prototype, simple_id });
    }
}
/*
Guardian: temp-disable: SPDMX — Per documented file-top architecture (hamuts.rs lines 4-12, "There is no HamutsBox/Hamuts split"): Scala's `HamutsBox.addFunctionExtern` delegated to `inner.addFunctionExtern` because Scala had a mutable Box wrapping immutable Hamuts; Rust collapsed both into a single mutable Hamuts. Same precedent as `add_function_export` above (line 330) which already has an SPDMX temp-disable for this exact pattern. SPDMX Exception Q (god-struct merging) applies. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1539-1780108002041/hook-1539/add_function_extern--359.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def addFunctionExtern(prototype: PrototypeH, simpleId: SimpleId, exportedName: String): Unit = {
    inner = inner.addFunctionExtern(prototype, simpleId, exportedName)
  }

//  def getNameId(readableName: String, packageCoordinate: PackageCoordinate, parts: Vector[IVonData]): Int = {
//    val (newInner, id) = inner.getNameId(readableName, packageCoordinate, parts)
//    inner = newInner
//    id
//  }
*/

// mig: fn get_static_sized_array (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn get_static_sized_array(&self, static_sized_array_th: &'h StaticSizedArrayHT<'s, 'h>) -> StaticSizedArrayDefinitionHT<'s, 'h> {
        panic!("Unimplemented: get_static_sized_array");
    }
}
/*
  def getStaticSizedArray(staticSizedArrayTH: StaticSizedArrayHT): StaticSizedArrayDefinitionHT = {
    inner.getStaticSizedArray(staticSizedArrayTH)
  }
*/

// mig: fn get_runtime_sized_array (HamutsBox accessor)
impl<'s, 'i, 'h> Hamuts<'s, 'i, 'h> where 's: 'i, 'i: 'h {
    pub fn get_runtime_sized_array(&self, runtime_sized_array_th: &'h RuntimeSizedArrayHT<'s, 'h>) -> RuntimeSizedArrayDefinitionHT<'s, 'h> {
        panic!("Unimplemented: get_runtime_sized_array");
    }
}
/*
  def getRuntimeSizedArray(runtimeSizedArrayTH: RuntimeSizedArrayHT): RuntimeSizedArrayDefinitionHT = {
    inner.getRuntimeSizedArray(runtimeSizedArrayTH)
  }
}
*/

// mig: case class Hamuts
/// Temporary state
//
// Mutable accumulator. `'s` is the scout/parser arena (matches Scala's
// `PackageCoordinate`/`StrI`). `'i` is the instantiating arena (matches Scala's
// `StructIT[cI]`/`InterfaceIT[cI]`/`PrototypeI[cI]` keys). `'h` is the
// simplifying arena.
pub struct Hamuts<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub human_name_to_full_name_to_id: HashMap<String, HashMap<String, i32>>,
    pub struct_t_to_opaque_h: HashMap<&'i StructIT<'s, 'i, cI>, &'h OpaqueHT<'s, 'h>>,
    pub struct_t_to_struct_h: HashMap<&'i StructIT<'s, 'i, cI>, &'h StructHT<'s, 'h>>,
    pub struct_t_to_struct_def_h: HashMap<&'i StructIT<'s, 'i, cI>, StructDefinitionH<'s, 'h>>,
    pub struct_defs: Vec<StructDefinitionH<'s, 'h>>,
    pub static_sized_arrays: HashMap<&'i StaticSizedArrayIT<'s, 'i, cI>, StaticSizedArrayDefinitionHT<'s, 'h>>,
    pub runtime_sized_arrays: HashMap<&'i RuntimeSizedArrayIT<'s, 'i, cI>, RuntimeSizedArrayDefinitionHT<'s, 'h>>,
    pub interface_t_to_interface_h: HashMap<&'i InterfaceIT<'s, 'i, cI>, &'h InterfaceHT<'s, 'h>>,
    pub interface_t_to_interface_def_h: HashMap<&'i InterfaceIT<'s, 'i, cI>, InterfaceDefinitionH<'s, 'h>>,
    pub function_refs: HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionRefH<'s, 'h>>,
    pub function_defs: HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionH<'s, 'h>>,
    pub package_coord_to_export_name_to_function: HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, &'h PrototypeH<'s, 'h>>>,
    pub package_coord_to_export_name_to_kind: HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, KindHT<'s, 'h>>>,
    pub package_coord_to_prototype_to_extern: HashMap<PackageCoordinate<'s>, HashMap<&'h PrototypeH<'s, 'h>, HamutsFunctionExtern<'s, 'h>>>,
    pub package_coord_to_kind_to_extern: HashMap<PackageCoordinate<'s>, HashMap<&'h OpaqueHT<'s, 'h>, HamutsKindExtern<'s, 'h>>>,
}
/*
case class Hamuts(
    humanNameToFullNameToId: Map[String, Map[String, Int]],
    structTToOpaqueH: Map[StructIT[cI], OpaqueHT],
    structTToStructH: Map[StructIT[cI], StructHT],
    structTToStructDefH: Map[StructIT[cI], StructDefinitionH],
    structDefs: Vector[StructDefinitionH],
    staticSizedArrays: Map[StaticSizedArrayIT[cI], StaticSizedArrayDefinitionHT],
    runtimeSizedArrays: Map[RuntimeSizedArrayIT[cI], RuntimeSizedArrayDefinitionHT],
    interfaceTToInterfaceH: Map[InterfaceIT[cI], InterfaceHT],
    interfaceTToInterfaceDefH: Map[InterfaceIT[cI], InterfaceDefinitionH],
    functionRefs: Map[PrototypeI[cI], FunctionRefH],
    functionDefs: Map[PrototypeI[cI], FunctionH],
    packageCoordToExportNameToFunction: Map[PackageCoordinate, Map[StrI, PrototypeH]],
    packageCoordToExportNameToKind: Map[PackageCoordinate, Map[StrI, KindHT]],
    packageCoordToPrototypeToExtern: Map[PackageCoordinate, Map[PrototypeH, HamutsFunctionExtern]],
    packageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]]) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big

  vassert(functionDefs.values.map(_.id).toVector.distinct.size == functionDefs.values.size)
  vassert(structDefs.map(_.id).distinct.size == structDefs.size)
  vassert(runtimeSizedArrays.values.map(_.name).toVector.distinct.size == runtimeSizedArrays.size)
*/

// mig: fn forward_declare_struct (collapsed into the &mut self method above)
/*
  def forwardDeclareStruct(structIT: StructIT[cI], structRefH: StructHT): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH + (structIT -> structRefH),
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_struct_originating_from_typing_pass (collapsed into the &mut self method above)
/*
  def addStructOriginatingFromTypingPass(structTT: StructIT[cI], structDefH: StructDefinitionH): Hamuts = {
    vassert(structTToStructH.contains(structTT))
    // structTToStructDefH.get(structTT) match {
    //   case Some(existingDef) => {
    //     // Added all this to help VmdSiteGen. Apparently it calls this method twice with the same structs sometimes?
    //     vassert(existingDef.id == structDefH.id)
    //     vassert(existingDef.members.map(_.name) == structDefH.members.map(_.name))
    //     vassert(existingDef.members.map(_.tyype) == structDefH.members.map(_.tyype))
    //     vassert(structDefs.exists(_.id == structDefH.id))
    //     this
    //   }
    //   case None => {
        Hamuts(
          humanNameToFullNameToId,
          structTToOpaqueH,
          structTToStructH,
          structTToStructDefH + (structTT -> structDefH),
          structDefs :+ structDefH,
          staticSizedArrays,
          runtimeSizedArrays,
          interfaceTToInterfaceH,
          interfaceTToInterfaceDefH,
          functionRefs,
          functionDefs,
          packageCoordToExportNameToFunction,
          packageCoordToExportNameToKind,
          packageCoordToPrototypeToExtern,
          packageCoordToKindToExtern)
      // }
    // }
  }
*/

// mig: fn add_opaque (collapsed into the &mut self method above)
/*
  def addOpaque(structIT: StructIT[cI], opaqueH: OpaqueHT): Hamuts = {
    vassert(!structTToOpaqueH.contains(structIT)) // I think we only do this function once and use the cached thing
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH + (structIT -> opaqueH),
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_struct_originating_from_hammer (collapsed into the &mut self method above)
/*
  def addStructOriginatingFromHammer(structDefH: StructDefinitionH): Hamuts = {
    vassert(!structDefs.exists(_.id == structDefH.id))

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs :+ structDefH,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn forward_declare_interface (collapsed into the &mut self method above)
/*
  def forwardDeclareInterface(interfaceIT: InterfaceIT[cI], interfaceRefH: InterfaceHT): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH + (interfaceIT -> interfaceRefH),
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_interface (collapsed into the &mut self method above)
/*
  def addInterface(interfaceIT: InterfaceIT[cI], interfaceDefH: InterfaceDefinitionH): Hamuts = {
    vassert(interfaceTToInterfaceH.contains(interfaceIT))
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH + (interfaceIT -> interfaceDefH),
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn forward_declare_function (collapsed into the &mut self method above)
/*
  def forwardDeclareFunction(functionRef2: PrototypeI[cI], functionRefH: FunctionRefH): Hamuts = {
    vassert(!functionRefs.contains(functionRef2))

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs + (functionRef2 -> functionRefH),
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_function (collapsed into the &mut self method above)
/*
  def addFunction(functionRef2: PrototypeI[cI], functionDefH: FunctionH): Hamuts = {
    vassert(functionRefs.contains(functionRef2))
    functionDefs.find(_._2.id == functionDefH.id) match {
      case None =>
      case Some(existing) => {
        vfail("Internal error: Can't add function:\n" + functionRef2 + "\nbecause there's already a function with same hammer name:\b" + existing._1 + "\nHammer name:\n" + functionDefH.id)
      }
    }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs + (functionRef2 -> functionDefH),
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_kind_export (collapsed into the &mut self method above)
/*
  def addKindExport(kind: KindHT, packageCoordinate: PackageCoordinate, exportedName: StrI): Hamuts = {
    val newPackageCoordToExportNameToKind =
      packageCoordToExportNameToKind.get(packageCoordinate) match {
        case None => {
          packageCoordToExportNameToKind + (packageCoordinate -> Map(exportedName -> kind))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(exportedName) match {
            case None => {
              packageCoordToExportNameToKind + (packageCoordinate -> (exportNameToFullName + (exportedName -> kind)))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      newPackageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_function_export (collapsed into the &mut self method above)
/*
  def addFunctionExport(function: PrototypeH, packageCoordinate: PackageCoordinate, exportedName: StrI): Hamuts = {
    val newPackageCoordToExportNameToFunction =
      packageCoordToExportNameToFunction.get(packageCoordinate) match {
        case None => {
          packageCoordToExportNameToFunction + (packageCoordinate -> Map(exportedName -> function))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(exportedName) match {
            case None => {
              packageCoordToExportNameToFunction + (packageCoordinate -> (exportNameToFullName + (exportedName -> function)))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      newPackageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_kind_extern (collapsed into the &mut self method above)
/*
  def addKindExtern(opaqueH: OpaqueHT, simpleId: SimpleId, exportedName: String): Hamuts = {
    val packageCoordinate = opaqueH.packageCoord
    val newPackageCoordToKindToExtern: Map[PackageCoordinate, Map[OpaqueHT, HamutsKindExtern]] =
      packageCoordToKindToExtern.get(packageCoordinate) match {
        case None => {
          packageCoordToKindToExtern + (packageCoordinate -> Map(opaqueH -> HamutsKindExtern(exportedName, opaqueH, simpleId)))
        }
        case Some(exportNameToFullName) => {
          exportNameToFullName.get(opaqueH) match {
            case None => {
              packageCoordToKindToExtern + (packageCoordinate -> (exportNameToFullName + (opaqueH -> HamutsKindExtern(exportedName, opaqueH, simpleId))))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      newPackageCoordToKindToExtern)
  }
*/

// mig: fn add_function_extern (collapsed into the &mut self method above)
/*
  def addFunctionExtern(function: PrototypeH, simpleId: SimpleId, exportedName: String): Hamuts = {
    val packageCoordinate = function.id.packageCoordinate
    val newPackageCoordToPrototypeToExtern =
      packageCoordToPrototypeToExtern.get(packageCoordinate) match {
        case None => {
          packageCoordToPrototypeToExtern + (packageCoordinate -> Map(function -> HamutsFunctionExtern(exportedName, function, simpleId)))
        }
        case Some(prototypeToExtern) => {
          prototypeToExtern.get(function) match {
            case None => {
              packageCoordToPrototypeToExtern + (packageCoordinate -> (prototypeToExtern + (function -> HamutsFunctionExtern(exportedName, function, simpleId))))
            }
            case Some(existingFullName) => {
              vfail("Already exported a `" + exportedName + "` from package `" + packageCoordinate + " : " + existingFullName)
            }
          }
        }
      }

    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      newPackageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_static_sized_array (collapsed into the &mut self method above)
/*
  def addStaticSizedArray(
    ssaIT: StaticSizedArrayIT[cI],
    staticSizedArrayDefinitionHT: StaticSizedArrayDefinitionHT
  ): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays + (ssaIT -> staticSizedArrayDefinitionHT),
      runtimeSizedArrays,
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }
*/

// mig: fn add_runtime_sized_array (collapsed into the &mut self method above)
/*
  def addRuntimeSizedArray(
    rsaIT: RuntimeSizedArrayIT[cI],
    runtimeSizedArrayDefinitionHT: RuntimeSizedArrayDefinitionHT
  ): Hamuts = {
    Hamuts(
      humanNameToFullNameToId,
      structTToOpaqueH,
      structTToStructH,
      structTToStructDefH,
      structDefs,
      staticSizedArrays,
      runtimeSizedArrays + (rsaIT -> runtimeSizedArrayDefinitionHT),
      interfaceTToInterfaceH,
      interfaceTToInterfaceDefH,
      functionRefs,
      functionDefs,
      packageCoordToExportNameToFunction,
      packageCoordToExportNameToKind,
      packageCoordToPrototypeToExtern,
      packageCoordToKindToExtern)
  }

//  // This returns a unique ID for that specific human name.
//  // Two things with two different human names could result in the same ID here.
//  // This ID is meant to be concatenated onto the human name.
//  def getNameId(readableName: String, packageCoordinate: PackageCoordinate, parts: Vector[IVonData]): (Hamuts, Int) = {
//    val namePartsString = IdH.namePartsToString(packageCoordinate, parts)
//    val idByFullNameForHumanName =
//      humanNameToFullNameToId.get(readableName) match {
//        case None => Map[String, Int]()
//        case Some(x) => x
//      }
//    val id =
//      idByFullNameForHumanName.get(namePartsString) match {
//        case None => idByFullNameForHumanName.size
//        case Some(i) => i
//      }
//    val idByFullNameForHumanNameNew = idByFullNameForHumanName + (namePartsString -> id)
//    val idByFullNameByHumanNameNew = humanNameToFullNameToId + (readableName -> idByFullNameForHumanNameNew)
//    val newHamuts =
//      Hamuts(
//        idByFullNameByHumanNameNew,
//        structTToStructH,
//        structTToStructDefH,
//        structDefs,
//        staticSizedArrays,
//        runtimeSizedArrays,
//        interfaceTToInterfaceH,
//        interfaceTToInterfaceDefH,
//        functionRefs,
//        functionDefs,
//        packageCoordToExportNameToFunction,
//        packageCoordToExportNameToKind,
//        packageCoordToPrototypeToExtern,
//        packageCoordToKindToExtern)
//    (newHamuts, id)
//  }
*/

// mig: fn get_static_sized_array (collapsed into the &self method above)
/*
  def getStaticSizedArray(staticSizedArrayHT: StaticSizedArrayHT): StaticSizedArrayDefinitionHT = {
    staticSizedArrays.values.find(_.kind == staticSizedArrayHT).get
  }
*/

// mig: fn get_runtime_sized_array (collapsed into the &self method above)
/*
  def getRuntimeSizedArray(runtimeSizedArrayTH: RuntimeSizedArrayHT): RuntimeSizedArrayDefinitionHT = {
    runtimeSizedArrays.values.find(_.kind == runtimeSizedArrayTH).get
  }
}
*/
