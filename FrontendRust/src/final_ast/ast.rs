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

/*
package dev.vale.finalast

import dev.vale.{PackageCoordinate, PackageCoordinateMap, StrI, vassert, vassertSome, vcurious, vfail, vimpl, vpass}
import dev.vale.von.IVonData

import scala.collection.immutable.ListMap

object ProgramH {
//  val emptyTupleStructRef =
//    // If the typingpass ever decides to change this things name, update this to match typingpass's.
////    StructRefH(FullNameH("Tup0", 0, PackageCoordinate.BUILTIN, Vector(VonObject("Tup",None,Vector(VonMember("members",VonArray(None,Vector())))))))
//    StructRefH(FullNameH("Tup",0, PackageCoordinate.BUILTIN, Vector(VonObject("CitizenName",None,Vector(VonMember("humanName",VonObject("CitizenTemplateName",None,Vector(VonMember("Tup",VonStr("Tup"))))), VonMember("templateArgs",VonArray(None,Vector(VonObject("CoordListTemplata",None,Vector(VonMember("coords",VonArray(None,Vector()))))))))))))
//  def emptyTupleStructType = ReferenceH(ShareH, InlineH, ReadonlyH, emptyTupleStructRef)

  val mainRegionName = "main"
  val externRegionName = "host"
}

*/

// mig: case class RegionH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionH;
/*
case class RegionH() {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

// mig: case class Export
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Export<'s, 'h> where 's: 'h {
    pub name_h: &'h IdH<'s, 'h>,
    pub exported_name: StrI<'s>,
}
/*
case class Export(
  nameH: IdH,
  exportedName: String
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

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
/*
case class PackageH(
    // All the interfaces in the program.
    interfaces: Vector[InterfaceDefinitionH],
    // All the structs in the program.
    structs: Vector[StructDefinitionH],
    // All of the user defined functions (and some from the compiler itself).
    functions: Vector[FunctionH],
    staticSizedArrays: Vector[StaticSizedArrayDefinitionHT],
    runtimeSizedArrays: Vector[RuntimeSizedArrayDefinitionHT],
    // Used for native compilation only, not JVM/CLR/JS/iOS.
    // These are pointing into the specific functions (in the `functions` field)
    // which should be called when we drop a reference to an immutable object.
//    immDestructorsByKind: Map[KindH, PrototypeH],
    // Translations for backends to use if they need to export a name.
    exportNameToFunction: Map[StrI, PrototypeH],
    // Translations for backends to use if they need to export a name.
    exportNameToKind: Map[StrI, KindHT],
    // Translations for backends to use if they need to export a name.
    prototypeToExtern: Map[PrototypeH, HamutsFunctionExtern],
    // Translations for backends to use if they need to export a name.
    kindToExtern: Map[OpaqueHT, HamutsKindExtern]
) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big
*/
// mig: fn extern_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn extern_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: extern_functions");
  }
}
/*
  // These are convenience functions for the tests to look up various functions.
  def externFunctions = functions.filter(_.isExtern)
*/
// mig: fn abstract_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn abstract_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: abstract_functions");
  }
}
/*
  def abstractFunctions = functions.filter(_.isAbstract)
*/
// mig: fn get_all_user_implemented_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn get_all_user_implemented_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: get_all_user_implemented_functions");
  }
}
/*
  // Functions that are neither extern nor abstract
  def getAllUserImplementedFunctions = functions.filter(f => f.isUserFunction && !f.isExtern && !f.isAbstract)
*/
// mig: fn non_extern_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn non_extern_functions(&self) -> Vec<&'h FunctionH<'s, 'h>> {
    panic!("Unimplemented: non_extern_functions");
  }
}
/*
  // Abstract or implemented
  def nonExternFunctions = functions.filter(!_.isExtern)
*/
// mig: fn get_all_user_functions
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn get_all_user_functions(&self) -> Vec<&FunctionH<'s, 'h>> {
    self.functions.iter().filter(|f| f.is_user_function()).collect()
  }
}
/*
  def getAllUserFunctions = functions.filter(_.isUserFunction)
*/
// mig: fn lookup_function
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn lookup_function(&self, readable_name: &str) -> &'h FunctionH<'s, 'h> {
    let from_exports: Vec<&'h PrototypeH<'s, 'h>> = self.export_name_to_function.iter().filter(|(k, _)| k.0 == readable_name).map(|(_, v)| *v).collect();
    let from_functions: Vec<&'h PrototypeH<'s, 'h>> = self.functions.iter().filter(|f| f.prototype.id.local_name.0 == readable_name).map(|f| f.prototype).collect();
    let mut matches: Vec<&'h PrototypeH<'s, 'h>> = Vec::new();
    for p in from_exports.into_iter().chain(from_functions.into_iter()) {
        if !matches.iter().any(|q| std::ptr::eq(*q as *const _, p as *const _)) {
            matches.push(p);
        }
    }
    assert!(!matches.is_empty());
    assert!(matches.len() <= 1);
    let first = matches[0];
    self.functions.iter().find(|f| std::ptr::eq(f.prototype as *const _, first as *const _)).expect("lookup_function: function with matching prototype")
  }
}
/*
  // Convenience function for the tests to look up a function.
  // Function must be at the top level of the program.
  def lookupFunction(readableName: String) = {
    val matches =
      (Vector.empty ++
        exportNameToFunction.find(_._1.str == readableName).map(_._2).toVector ++
        functions.filter(_.prototype.id.localName == readableName).map(_.prototype))
        .distinct
    vassert(matches.nonEmpty)
    vassert(matches.size <= 1)
    functions.find(_.prototype == matches.head).get
  }
*/
// mig: fn lookup_struct
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn lookup_struct(&self, human_name: &str) -> &'h StructDefinitionH<'s, 'h> {
    let matches: Vec<&StructDefinitionH<'s, 'h>> = self.structs.iter().filter(|s| s.id.local_name.0 == human_name).collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
}
/*
  // Convenience function for the tests to look up a struct.
  // Struct must be at the top level of the program.
  def lookupStruct(humanName: String) = {
    val matches = structs.filter(_.id.localName == humanName)
    vassert(matches.size == 1)
    matches.head
  }
*/
// mig: fn lookup_interface
impl<'s, 'h> PackageH<'s, 'h> where 's: 'h {
  pub fn lookup_interface(&self, human_name: &str) -> &'h InterfaceDefinitionH<'s, 'h> {
    let matches: Vec<&InterfaceDefinitionH<'s, 'h>> = self.interfaces.iter().filter(|i| i.id.shortened_name.0 == human_name).collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
}
/*
  // Convenience function for the tests to look up an interface.
  // Interface must be at the top level of the program.
  def lookupInterface(humanName: String) = {
    val matches = interfaces.filter(_.id.shortenedName == humanName)
    vassert(matches.size == 1)
    matches.head
  }
}
*/

// mig: case class ProgramH
/// Temporary state
pub struct ProgramH<'s, 'h> where 's: 'h {
    pub packages: crate::utils::code_hierarchy::PackageCoordinateMap<'s, PackageH<'s, 'h>>,
}
/*
case class ProgramH(
  packages: PackageCoordinateMap[PackageH]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big
*/
// mig: fn lookup_package
impl<'s, 'h> ProgramH<'s, 'h> where 's: 'h {
  pub fn lookup_package(&self, package_coordinate: crate::utils::code_hierarchy::PackageCoordinate<'s>) -> PackageH<'s, 'h> {
    *self.packages.get(&package_coordinate).expect("lookup_package: missing")
  }
}
/*
  def lookupPackage(packageCoordinate: PackageCoordinate): PackageH = {
    vassertSome(packages.get(packageCoordinate))
  }
  def lookupFunction(prototype: PrototypeH): FunctionH = {
    val paackage = lookupPackage(prototype.id.packageCoordinate)
    val result = vassertSome(paackage.functions.find(_.id == prototype.id))
    vassert(prototype == result.prototype)
    result
  }
  def lookupStruct(structRefH: StructHT): StructDefinitionH = {
    val paackage = lookupPackage(structRefH.id.packageCoordinate)
    vassertSome(paackage.structs.find(_.getRef == structRefH))
  }
  def lookupInterface(interfaceRefH: InterfaceHT): InterfaceDefinitionH = {
    val paackage = lookupPackage(interfaceRefH.id.packageCoordinate)
    vassertSome(paackage.interfaces.find(_.getRef == interfaceRefH))
  }
  def lookupStaticSizedArray(ssaTH: StaticSizedArrayHT): StaticSizedArrayDefinitionHT = {
    val paackage = lookupPackage(ssaTH.id.packageCoordinate)
    vassertSome(paackage.staticSizedArrays.find(_.name == ssaTH.id))
  }
  def lookupRuntimeSizedArray(rsaTH: RuntimeSizedArrayHT): RuntimeSizedArrayDefinitionHT = {
    val paackage = lookupPackage(rsaTH.name.packageCoordinate)
    vassertSome(paackage.runtimeSizedArrays.find(_.name == rsaTH.name))
  }
}
*/

// mig: case class StructDefinitionH
/// Temporary state
//
// PartialEq/Eq/Hash dropped because the edges field's EdgeH transitively
// holds an ArenaIndexMap (which lacks those impls). Matches Scala's `vcurious`.
#[derive(Copy, Clone, Debug)]
pub struct StructDefinitionH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub weakable: bool,
    pub extern_: bool,
    pub mutability: Mutability,
    pub edges: &'h [EdgeH<'s, 'h>],
    pub members: &'h [StructMemberH<'s, 'h>],
}
// mig: fn get_ref (on StructDefinitionH — see Scala `def getRef: StructHT = StructHT(id)` in the StructDefinitionH audit block below)
impl<'s, 'h> StructDefinitionH<'s, 'h> where 's: 'h {
    pub fn get_ref(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> &'h crate::final_ast::types::StructHT<'s, 'h> {
        interner.intern_struct_ht(crate::final_ast::types::StructHTValH { id: self.id })
    }
}
/*
// The struct definition, which defines a struct's name, members, and so on.
// There is only one of these per type of struct in the program.
case class StructDefinitionH(
    // Name of the struct. Guaranteed to be unique in the entire program.
    id: IdH,
    // Whether we can take weak references to this object.
    // On native, this means an extra "weak ref count" will be included for the object.
    // On JVM/CLR/JS, this means the object will have an extra tiny object pointing
    // back at itself.
    // On iOS, this can be ignored, all objects are weakable already.
    weakable: Boolean,
    // Whether it's defined by the outside world.
    extern: Boolean,
    // Whether this struct is deeply immutable or not.
    // This affects how the struct is deallocated.
    // On native, this means that we potentially call the destructor any time we let go
    // of a reference.
    // On JVM/CLR/JS/iOS, this can be ignored.
    mutability: Mutability,
    // All of the `impl`s, in other words, all of the vtables for this struct for all
    // the interfaces it implements.
    edges: Vector[EdgeH],
    // The members of the struct, in order.
    members: Vector[StructMemberH]) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
  def getRef: StructHT = StructHT(id)
}
*/

// mig: case class StructMemberH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructMemberH<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s, 'h>,
    pub variability: Variability,
    pub tyype: CoordH<'s, 'h>,
}
/*
// A member of a struct.
case class StructMemberH(
  // Name of the struct member. This is *not* guaranteed to be unique in the entire
  // program.
  name: IdH,
  // Whether this field can be changed or not.
  // This isn't wired up to anything, feel free to ignore it.
  variability: Variability,
  // The type of the member.
  tyype: CoordH[KindHT]) {

  vpass()

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
}
*/

// mig: case class InterfaceDefinitionH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceDefinitionH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub weakable: bool,
    pub mutability: Mutability,
    pub super_interfaces: &'h [&'h InterfaceHT<'s, 'h>],
    pub methods: &'h [InterfaceMethodH<'s, 'h>],
}
// mig: fn get_ref (on InterfaceDefinitionH — see Scala `def getRef = InterfaceHT(id)` in audit-trail below)
impl<'s, 'h> InterfaceDefinitionH<'s, 'h> where 's: 'h {
    pub fn get_ref(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> &'h crate::final_ast::types::InterfaceHT<'s, 'h> {
        interner.intern_interface_ht(crate::final_ast::types::InterfaceHTValH { id: self.id })
    }
}
/*
// An interface definition containing name, methods, etc.
case class InterfaceDefinitionH(
  id: IdH,
  // Whether we can take weak references to this interface.
  // On native, this means an extra "weak ref count" will be included for the object.
  // On JVM/CLR/JS, this means the object should extend the IWeakable interface,
  // and expose a tiny object pointing back at itself.
  // On iOS, this can be ignored, all objects are weakable already.
  weakable: Boolean,
  // Whether this interface is deeply immutable or not.
  // On native, this affects how we free the object.
  // This can be ignored on JVM/CLR/JS/iOS.
  mutability: Mutability,
  // The interfaces that this interface extends.
  // This isnt hooked up to anything, and can be safely ignored.
  // TODO: Change this to edges, since interfaces impl other interfaces.
  superInterfaces: Vector[InterfaceHT],
  // All the methods that we can call on this interface.
  methods: Vector[InterfaceMethodH]) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
  def getRef = InterfaceHT(id)
}
*/

// mig: case class InterfaceMethodH
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceMethodH<'s, 'h> where 's: 'h {
    pub prototype_h: &'h PrototypeH<'s, 'h>,
    pub virtual_param_index: i32,
}
/*
// A method in an interface.
case class InterfaceMethodH(
  // The name, params, and return type of the method.
  prototypeH: PrototypeH,
  // Describes which param is the one that will have the vtable.
  // Currently this is always assumed to be zero.
  virtualParamIndex: Int) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  vassert(virtualParamIndex >= 0)
}
*/

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
/*
// Represents how a struct implements an interface.
// Each edge has a vtable.
case class EdgeH(
  // The struct whose actual functions will be called.
  struct: StructHT,
  // The interface that this struct is conforming to.
  interface: InterfaceHT,
  // Map whose key is an interface method, and whose value is the method of the struct
  // that it's overriding.
  structPrototypesByInterfaceMethod: ListMap[InterfaceMethodH, PrototypeH]) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/

// mig: sealed trait IFunctionAttributeH + case objects UserFunctionH, PureH
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionAttributeH {
    UserFunctionH,
    PureH,
}
/*
sealed trait IFunctionAttributeH
case object UserFunctionH extends IFunctionAttributeH // Whether it was written by a human. Mostly for tests right now.
case object PureH extends IFunctionAttributeH
*/

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
/*
// A function's definition.
case class FunctionH(
  // Describes the function's name, params, and return type.
  prototype: PrototypeH,

  // Whether this has a body. If true, the body will simply contain an InterfaceCallH instruction.
  isAbstract: Boolean,
  // Whether this is an extern. If true, the body will simply contain an ExternCallH instruction to the same
  // prototype describing this function.
  isExtern: Boolean,

  attributes: Vector[IFunctionAttributeH],

  // The body of the function that contains the actual instructions.
  body: ExpressionH[KindHT]) {

  vpass()

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
  def id = prototype.id
  def isUserFunction = attributes.contains(UserFunctionH)
}
*/
// mig: fn is_user_function
impl<'s, 'h> FunctionH<'s, 'h> where 's: 'h {
    pub fn is_user_function(&self) -> bool {
        self.attributes.contains(&IFunctionAttributeH::UserFunctionH)
    }
}
/* Guardian: disable-all */

// mig: case class PrototypeH
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub params: &'h [CoordH<'s, 'h>],
    pub return_type: CoordH<'s, 'h>,
    pub _must_intern: MustIntern,
}
/*
// A wrapper around a function's name, which also has its params and return type.
case class PrototypeH(
  id: IdH,
  params: Vector[CoordH[KindHT]],
  returnType: CoordH[KindHT]
) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/

// mig: case class PrototypeH (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeHValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub params: &'h [CoordH<'s, 'h>],
    pub return_type: CoordH<'s, 'h>,
}

// mig: case class IdH
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdH<'s, 'h> where 's: 'h {
    pub local_name: StrI<'s>,
    pub package_coordinate: crate::utils::code_hierarchy::PackageCoordinate<'s>,
    pub shortened_name: StrI<'s>,
    pub fully_qualified_name: StrI<'s>,
    pub _must_intern: crate::simplifying::hammer_interner::MustIntern,
    pub _phantom_h: PhantomData<&'h ()>,
}
/*
// A unique name for something in the program.
case class IdH(
  // This is at the beginning so toString puts it at the start, for easier debugging
  localName: String, // Careful, has collisions.
  packageCoordinate: PackageCoordinate,
  // Should be the shortest possible string without collisions
  shortenedName: String,
  // Most precise name, without shortening.
  fullyQualifiedName: String) {
  vpass()

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;

  override def equals(obj: Any): Boolean = {
    obj match {
      case IdH(_, thatPackageCoord, thatShortenedName, thatFullyQualifiedName) => {
        if (shortenedName == thatShortenedName) {
          // These makes sure that the shortening never has any false collisions.
          vassert(fullyQualifiedName == thatFullyQualifiedName)
          vassert(packageCoordinate == thatPackageCoord)
          true
        } else {
          false
        }
      }
    }
  }
}

//object IdH {
//  def namePartsToString(packageCoordinate: PackageCoordinate, parts: Vector[IVonData]) = {
//    packageCoordinate.module.str + "::" + packageCoordinate.packages.map(_ + "::").mkString("") + parts.map(MetalPrinter.print).mkString(":")
//  }
//}
*/

// mig: case class IdH (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdHValH<'s, 'h> where 's: 'h {
    pub local_name: StrI<'s>,
    pub package_coordinate: crate::utils::code_hierarchy::PackageCoordinate<'s>,
    pub shortened_name: StrI<'s>,
    pub fully_qualified_name: StrI<'s>,
    pub _phantom_h: PhantomData<&'h ()>,
}
/*
Guardian: temp-disable: NCWSRX — False positive: `_phantom_h` is a Rust mechanical PhantomData required so the `'h` lifetime parameter isn't vacuous (no other field uses `'h`); Scala's `IdH` has no `'h` and no phantom — this is the documented Rust-only adaptation. Adding `pub` is needed so `hammer_interner` (the only legitimate constructor per the `MustIntern` seal) can fill the field at intern time. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1455-1780105540747/hook-1455/IdHValH--503.0.NoChangesWithoutScalaReference-NCWSRX.NoChangesWithoutScalaReference-NCWSRX.verdict.md
*/

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
