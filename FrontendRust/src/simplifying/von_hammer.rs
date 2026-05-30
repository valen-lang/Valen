// From Frontend/SimplifyingPass/src/dev/vale/simplifying/VonHammer.scala
//
// Per typing-pass `Compiler` precedent, `VonHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::ast::{
    EdgeH, FunctionH, FunctionRefH, IdH, InterfaceDefinitionH, InterfaceMethodH, PackageH,
    ProgramH, PrototypeH, RegionH, StructDefinitionH, StructMemberH,
};
use crate::final_ast::instructions::{ExpressionH, Local, VariableIdH};
use crate::final_ast::types::{
    CodeLocation, CoordH, InterfaceHT, KindHT, LocationH, Mutability, OwnershipH,
    RuntimeSizedArrayDefinitionHT, SimpleId, SimpleIdStep, StaticSizedArrayDefinitionHT, StructHT,
    Variability,
};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::von::ast::IVonData;
use crate::simplifying::hammer::Hammer;

/*
package dev.vale.simplifying

import dev.vale.{CodeLocationS, PackageCoordinate, RangeS, vimpl}
import dev.vale.finalast._
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.{finalast => m}
import dev.vale.postparsing._
import dev.vale.von.{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr}

class VonHammer(nameHammer: NameHammer, typeHammer: TypeHammer) {
*/

// mig: fn vonify_program
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_program(&self, program: &ProgramH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_program");
    }
}
/*
  def vonifyProgram(program: ProgramH): IVonData = {
    val ProgramH(packages) = program

    VonObject(
      "Program",
      None,
      Vector(
        VonMember(
          "packages",
          VonArray(
            None,
            packages.flatMap({ case (packageCoord, paackage) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoord)),
                  VonMember("package", vonifyPackage(packageCoord, paackage))
                ))
            }).toVector))))
  }
*/

// mig: fn vonify_simple_id
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_simple_id(&self, simple_id: SimpleId<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_simple_id");
    }
}
/*
  def vonifySimpleId(simpleId: SimpleId): IVonData = {
    VonObject(
      "Id",
      None,
      Vector(
        VonMember(
          "steps",
          VonArray(
            None,
            simpleId.steps.map(vonifySimpleIdStep)))))
  }
*/

// mig: fn vonify_simple_id_step
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_simple_id_step(&self, step: SimpleIdStep<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_simple_id_step");
    }
}
/*
  def vonifySimpleIdStep(step: SimpleIdStep): IVonData = {
    val SimpleIdStep(name, templateArgs) = step
    VonObject(
      "IdStep",
      None,
      Vector(
        VonMember("name", VonStr(name)),
        VonMember("templateArgs", VonArray(None, templateArgs.map(vonifySimpleId)))))
  }
*/

// mig: fn vonify_package
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_package(
        &self,
        package_coord: PackageCoordinate<'s>,
        paackage: &PackageH<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_package");
    }
}
/*
  def vonifyPackage(packageCoord: PackageCoordinate, paackage: PackageH): IVonData = {
    val PackageH(
      interfaces,
      structs,
      functions,
      staticSizedArrays,
      runtimeSizedArrays,
//      immDestructorsByKind,
      exportNameToFunction,
      exportNameToKind,
      externNameToFunction,
      externNameToKind,
    ) = paackage

    VonObject(
      "Package",
      None,
      Vector(
        VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoord)),
        VonMember("interfaces", VonArray(None, interfaces.map(vonifyInterface).toVector)),
        VonMember("structs", VonArray(None, structs.map(vonfiyStruct).toVector)),
        VonMember("functions", VonArray(None, functions.map(vonifyFunction).toVector)),
        VonMember("staticSizedArrays", VonArray(None, staticSizedArrays.map(vonifyStaticSizedArrayDefinition).toVector)),
        VonMember("runtimeSizedArrays", VonArray(None, runtimeSizedArrays.map(vonifyRuntimeSizedArrayDefinition).toVector)),
//        VonMember(
//          "immDestructorsByKind",
//          VonArray(
//            None,
//            immDestructorsByKind.toVector.map({ case (kind, destructor) =>
//              VonObject(
//                "Entry",
//                None,
//                Vector(
//                  VonMember("kind", vonifyKind(kind)),
//                  VonMember("destructor", vonifyPrototype(destructor))))
//            }))),
        VonMember(
          "exportNameToFunction",
          VonArray(
            None,
            exportNameToFunction.toVector.map({ case (exportName, prototype) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("exportName", VonStr(exportName.str)),
                  VonMember("prototype", vonifyPrototype(prototype))))
            }))),
        VonMember(
          "exportNameToKind",
          VonArray(
            None,
            exportNameToKind.toVector.map({ case (exportName, kind) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("exportName", VonStr(exportName.str)),
                  VonMember("kind", vonifyKind(kind))))
            }))),
        VonMember(
          "prototypeToExtern",
          VonArray(
            None,
            externNameToFunction.toVector.map({ case (_, HamutsFunctionExtern(maybeExternName, prototype, simpleId)) =>
              VonObject(
                "ExternFunction",
                None,
                Vector(
                  VonMember("mangledName", VonStr(maybeExternName)),
                  VonMember("id", vonifySimpleId(simpleId)),
                  VonMember("prototype", vonifyPrototype(prototype))))
            }))),
        VonMember(
          "kindToExtern", // DO NOT SUBMIT rename plz
          VonArray(
            None,
            externNameToKind.toVector.map({ case (_, HamutsKindExtern(maybeExternName, kind, simpleId)) =>
              VonObject(
                "ExternKind",
                None,
                Vector(
                  VonMember("mangledName", VonStr(maybeExternName)),
                  VonMember("id", vonifySimpleId(simpleId)),
                  VonMember("kind", vonifyKind(kind))))
            })))))
  }
*/

// mig: fn vonify_region
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_region(&self, region: RegionH) -> IVonData {
        panic!("Unimplemented: vonify_region");
    }
}
/*
  def vonifyRegion(region: RegionH): IVonData = {
    val RegionH() = region

    VonObject(
      "Region",
      None,
      Vector())
//      Vector(
//        VonMember(
//          "kinds",
//          VonArray(
//            None,
//            kinds.map(vonifyKind).toVector))))
  }
*/

// mig: fn vonify_struct_h
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct_h(&self, r#ref: &StructHT<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_struct_h");
    }
}
/*
  def vonifyStructH(ref: StructHT): IVonData = {
    val StructHT(fullName) = ref

    VonObject(
      "StructId",
      None,
      Vector(
        VonMember("name", vonifyName(fullName))))
  }
*/

// mig: fn vonify_interface (Scala overload — by InterfaceHT)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface(&self, r#ref: &InterfaceHT<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_interface");
    }
}
/*
  def vonifyInterface(ref: InterfaceHT): IVonData = {
    val InterfaceHT(fullName) = ref

    VonObject(
      "InterfaceId",
      None,
      Vector(
        VonMember("name", vonifyName(fullName))))
  }
*/

// mig: fn vonify_interface_method
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface_method(
        &self,
        interface_method_h: &InterfaceMethodH<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_interface_method");
    }
}
/*
  def vonifyInterfaceMethod(interfaceMethodH: InterfaceMethodH): IVonData = {
    val InterfaceMethodH(prototype, virtualParamIndex) = interfaceMethodH

    VonObject(
      "InterfaceMethod",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype)),
        VonMember("virtualParamIndex", VonInt(virtualParamIndex))))
  }
*/

// mig: fn vonify_interface_def (Scala overload — by InterfaceDefinitionH;
// disambiguated per overload-suffix pattern.)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_interface_def(&self, interface: &InterfaceDefinitionH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_interface_def");
    }
}
/*
  def vonifyInterface(interface: InterfaceDefinitionH): IVonData = {
    val InterfaceDefinitionH(fullName, weakable, mutability, superInterfaces, prototypes) = interface

    VonObject(
      "Interface",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("kind", vonifyInterface(interface.getRef)),
        VonMember("weakable", VonBool(weakable)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("superInterfaces", VonArray(None, superInterfaces.map(vonifyInterface).toVector)),
        VonMember("methods", VonArray(None, prototypes.map(vonifyInterfaceMethod).toVector))))
  }
*/

// mig: fn vonify_struct
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct(&self, r#struct: &StructDefinitionH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_struct");
    }
}
/*
  def vonfiyStruct(struct: StructDefinitionH): IVonData = {
    val StructDefinitionH(fullName, weakable, extern, mutability, edges, members) = struct

    VonObject(
      "Struct",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("kind", vonifyStructH(struct.getRef)),
        VonMember("weakable", VonBool(weakable)),
        VonMember("extern", VonBool(extern)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("edges", VonArray(None, edges.map(edge => vonifyEdge(edge)).toVector)),
        VonMember("members", VonArray(None, members.map(vonifyStructMember).toVector))))
  }
*/

// mig: fn vonify_mutability
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_mutability(&self, mutability: Mutability) -> IVonData {
        panic!("Unimplemented: vonify_mutability");
    }
}
/*
  def vonifyMutability(mutability: Mutability): IVonData = {
    mutability match {
      case Immutable => VonObject("Immutable", None, Vector())
      case Mutable => VonObject("Mutable", None, Vector())
    }
  }
*/

// mig: fn vonify_location
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_location(&self, location: LocationH) -> IVonData {
        panic!("Unimplemented: vonify_location");
    }
}
/*
  def vonifyLocation(location: LocationH): IVonData = {
    location match {
      case InlineH => VonObject("Inline", None, Vector())
      case YonderH => VonObject("Yonder", None, Vector())
    }
  }
*/

// mig: fn vonify_variability
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_variability(&self, variability: Variability) -> IVonData {
        panic!("Unimplemented: vonify_variability");
    }
}
/*
  def vonifyVariability(variability: Variability): IVonData = {
    variability match {
      case Varying => VonObject("Varying", None, Vector())
      case Final => VonObject("Final", None, Vector())
    }
  }
*/

// mig: fn vonify_prototype
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_prototype(&self, prototype: &PrototypeH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_prototype");
    }
}
/*
  def vonifyPrototype(prototype: PrototypeH): IVonData = {
    val PrototypeH(fullName, params, returnType) = prototype

    VonObject(
      "Prototype",
      None,
      Vector(
        VonMember("name", vonifyName(fullName)),
        VonMember("params", VonArray(None, params.map(vonifyCoord).toVector)),
        VonMember("return", vonifyCoord(returnType))))
  }
*/

// mig: fn vonify_coord
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_coord(&self, coord: CoordH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_coord");
    }
}
/*
  def vonifyCoord(coord: CoordH[KindHT]): IVonData = {
    val CoordH(ownership, location, kind) = coord

    VonObject(
      "Ref",
      None,
      Vector(
        VonMember("ownership", vonifyOwnership(ownership)),
        VonMember("location", vonifyLocation(location)),
        VonMember("kind", vonifyKind(kind))))
  }
*/

// mig: fn vonify_edge
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_edge(&self, edge_h: &EdgeH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_edge");
    }
}
/*
  def vonifyEdge(edgeH: EdgeH): IVonData = {
    val EdgeH(struct, interface, structPrototypesByInterfacePrototype) = edgeH

    VonObject(
      "Edge",
      None,
      Vector(
        VonMember("structName", vonifyStructH(struct)),
        VonMember("interfaceName", vonifyInterface(interface)),
        VonMember(
          "methods",
          VonArray(
            None,
            structPrototypesByInterfacePrototype.toVector.map({ case (interfaceMethod, structPrototype) =>
              VonObject(
                "Entry",
                None,
                Vector(
                  VonMember("method", vonifyInterfaceMethod(interfaceMethod)),
                  VonMember("override", vonifyPrototype(structPrototype))))
            })))))
  }
*/

// mig: fn vonify_ownership
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_ownership(&self, ownership: OwnershipH) -> IVonData {
        panic!("Unimplemented: vonify_ownership");
    }
}
/*
  def vonifyOwnership(ownership: OwnershipH): IVonData = {
    ownership match {
      case OwnH => VonObject("Own", None, Vector())
      case ImmutableBorrowH => VonObject("ImmutableBorrow", None, Vector())
      case MutableBorrowH => VonObject("MutableBorrow", None, Vector())
      case ImmutableShareH => VonObject("ImmutableShare", None, Vector())
      case MutableShareH => VonObject("MutableShare", None, Vector())
      case WeakH => VonObject("Weak", None, Vector())
    }
  }
*/

// mig: fn vonify_struct_member
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_struct_member(&self, struct_member_h: &StructMemberH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_struct_member");
    }
}
/*
  def vonifyStructMember(structMemberH: StructMemberH): IVonData = {
    val StructMemberH(name, variability, tyype) = structMemberH

    VonObject(
      "StructMember",
      None,
      Vector(
        VonMember("fullName", vonifyName(name)),
        VonMember("name", VonStr(name.localName)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyCoord(tyype))))
  }
*/

// mig: fn vonify_runtime_sized_array_definition
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_runtime_sized_array_definition(
        &self,
        rsa_def: &RuntimeSizedArrayDefinitionHT<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_runtime_sized_array_definition");
    }
}
/*
  def vonifyRuntimeSizedArrayDefinition(rsaDef: RuntimeSizedArrayDefinitionHT): IVonData = {
    val RuntimeSizedArrayDefinitionHT(name, mutability, elementType) = rsaDef
    VonObject(
      "RuntimeSizedArrayDefinition",
      None,
      Vector(
        VonMember("name", vonifyName(name)),
        VonMember("kind", vonifyKind(rsaDef.kind)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("elementType", vonifyCoord(elementType))))
  }
*/

// mig: fn vonify_static_sized_array_definition
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_static_sized_array_definition(
        &self,
        ssa_def: &StaticSizedArrayDefinitionHT<'s, 'h>,
    ) -> IVonData {
        panic!("Unimplemented: vonify_static_sized_array_definition");
    }
}
/*
  def vonifyStaticSizedArrayDefinition(ssaDef: StaticSizedArrayDefinitionHT): IVonData = {
    val StaticSizedArrayDefinitionHT(name, size, mutability, variability, elementType) = ssaDef
    VonObject(
      "StaticSizedArrayDefinition",
      None,
      Vector(
        VonMember("name", vonifyName(name)),
        VonMember("kind", vonifyKind(ssaDef.kind)),
        VonMember("size", VonInt(size)),
        VonMember("mutability", vonifyMutability(mutability)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("elementType", vonifyCoord(elementType))))
  }
*/

// mig: fn vonify_kind
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_kind(&self, kind: KindHT<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_kind");
    }
}
/*
  def vonifyKind(kind: KindHT): IVonData = {
    kind match {
      case NeverHT(_) => VonObject("Never", None, Vector())
      case IntHT(bits) => VonObject("Int", None, Vector(VonMember("bits", VonInt(bits))))
      case BoolHT() => VonObject("Bool", None, Vector())
      case StrHT() => VonObject("Str", None, Vector())
      case VoidHT() => VonObject("Void", None, Vector())
      case FloatHT() => VonObject("Float", None, Vector())
      case ir @ InterfaceHT(_) => vonifyInterface(ir)
      case sr @ StructHT(_) => vonifyStructH(sr)
      case RuntimeSizedArrayHT(name) => {
        VonObject(
          "RuntimeSizedArray",
          None,
          Vector(
            VonMember("name", vonifyName(name))))
      }
      case StaticSizedArrayHT(name) => {
        VonObject(
          "StaticSizedArray",
          None,
          Vector(
            VonMember("name", vonifyName(name))))
      }
      case OpaqueHT(_, structId, simpleId) => {
        VonObject(
          "Opaque",
          None,
          Vector(
            VonMember("structId", vonifyName(structId)),
            VonMember("structSimpleId", vonifySimpleId(simpleId))))
      }
    }
  }
*/

// mig: fn vonify_function
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_function(&self, function_h: &FunctionH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_function");
    }
}
/*
  def vonifyFunction(functionH: FunctionH): IVonData = {
    val FunctionH(prototype, _, _, _, body) = functionH

    VonObject(
      "Function",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype)),
        // TODO: rename block to body
        VonMember("block", vonifyExpression(body))))
  }
*/

// mig: fn vonify_expression
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_expression(&self, node: ExpressionH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_expression");
    }
}
/*
  def vonifyExpression(node: ExpressionH[KindHT]): IVonData = {
    node match {
      case ConstantVoidH() => {
        VonObject("ConstantVoid", None, Vector())
      }
      case ConstantBoolH(value) => {
        VonObject(
          "ConstantBool",
          None,
          Vector(
            VonMember("value", VonBool(value))))
      }
      case ConstantIntH(value, bits) => {
        VonObject(
          "ConstantInt",
          None,
          Vector(
            VonMember("value", VonStr(value.toString)),
            VonMember("bits", VonInt(bits))))
      }
      case ConstantStrH(value) => {
        VonObject(
          "ConstantStr",
          None,
          Vector(
            VonMember("value", VonStr(value))))
      }
      case ConstantF64H(value) => {
        VonObject(
          "ConstantF64",
          None,
          Vector(
            VonMember("value", VonFloat(value))))
      }
      case ArrayCapacityH(sourceExpr) => {
        VonObject(
          "ArrayCapacity",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false))))
      }
      case ArrayLengthH(sourceExpr) => {
        VonObject(
          "ArrayLength",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false))))
      }
      case wa @ BorrowToWeakH(sourceExpr) => {
        VonObject(
          "BorrowToWeak",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKind", vonifyKind(sourceExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(wa.resultType)),
            VonMember("resultKind", vonifyKind(wa.resultType.kind))))
      }
      case AsSubtypeH(sourceExpr, targetType, resultResultType, okConstructor, errConstructor) => {
        VonObject(
          "AsSubtype",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("targetKind", vonifyKind(targetType)),
            VonMember("okConstructor", vonifyPrototype(okConstructor)),
            VonMember("okType", vonifyCoord(okConstructor.returnType)),
            VonMember("okKind", vonifyKind(okConstructor.returnType.kind)),
            VonMember("errConstructor", vonifyPrototype(errConstructor)),
            VonMember("errType", vonifyCoord(errConstructor.returnType)),
            VonMember("errKind", vonifyKind(errConstructor.returnType.kind)),
            VonMember("resultResultType", vonifyCoord(resultResultType)),
            VonMember("resultResultKind", vonifyKind(resultResultType.kind))))
      }
      case LockWeakH(sourceExpr, resultOptType, someConstructor, noneConstructor) => {
        VonObject(
          "LockWeak",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("someConstructor", vonifyPrototype(someConstructor)),
            VonMember("someType", vonifyCoord(someConstructor.returnType)),
            VonMember("someKind", vonifyKind(someConstructor.returnType.kind)),
            VonMember("noneConstructor", vonifyPrototype(noneConstructor)),
            VonMember("noneType", vonifyCoord(noneConstructor.returnType)),
            VonMember("noneKind", vonifyKind(noneConstructor.returnType.kind)),
            VonMember("resultOptType", vonifyCoord(resultOptType)),
            VonMember("resultOptKind", vonifyKind(resultOptType.kind))))
      }
      case ReturnH(sourceExpr) => {
        VonObject(
          "Return",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType))))
      }
      case BreakH() => {
        VonObject(
          "Break",
          None,
          Vector())
      }
      case DiscardH(sourceExpr) => {
        VonObject(
          "Discard",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceResultType", vonifyCoord(sourceExpr.resultType))))
      }
      case PreCheckBorrowH(sourceExpr) => {
        VonObject(
          "PreCheckBorrow",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceResultType", vonifyCoord(sourceExpr.resultType))))
      }
      case ArgumentH(resultReference, argumentIndex) => {
        VonObject(
          "Argument",
          None,
          Vector(
            VonMember("resultType", vonifyCoord(resultReference)),
            VonMember("argumentIndex", VonInt(argumentIndex))))
      }
      case NewArrayFromValuesH(resultType, sourceExprs) => {
        VonObject(
          "NewArrayFromValues",
          None,
          Vector(
            VonMember("sourceExprs", VonArray(None, sourceExprs.map(vonifyExpression).toVector)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("resultKind", vonifyKind(resultType.kind))))
      }
      case NewStructH(sourceExprs, targetMemberNames, resultType) => {
        VonObject(
          "NewStruct",
          None,
          Vector(
            VonMember(
              "sourceExprs",
              VonArray(None, sourceExprs.map(vonifyExpression).toVector)),
            VonMember(
              "memberNames",
              VonArray(None, targetMemberNames.map(n => vonifyName(n)).toVector)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case StackifyH(sourceExpr, local, name) => {
        VonObject(
          "Stackify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("local", vonifyLocal(local)),
            VonMember("knownLive", VonBool(false)),
            VonMember("optName", vonifyOptional[IdH](name, n => vonifyName(n)))))
      }
      case RestackifyH(sourceExpr, local, name) => {
        VonObject(
          "Restackify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("local", vonifyLocal(local)),
            VonMember("knownLive", VonBool(false)),
            VonMember("optName", vonifyOptional[IdH](name, n => vonifyName(n)))))
      }
      case UnstackifyH(local) => {
        VonObject(
          "Unstackify",
          None,
          Vector(
            VonMember("local", vonifyLocal(local))))
      }
      case DestroyStaticSizedArrayIntoFunctionH(arrayExpr, consumerExpr, consumerMethod, arrayElementType, arraySize) => {
        VonObject(
          "DestroyStaticSizedArrayIntoFunction",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("consumerExpr", vonifyExpression(consumerExpr)),
            VonMember("consumerType", vonifyCoord(consumerExpr.resultType)),
            VonMember("consumerMethod", vonifyPrototype(consumerMethod)),
            VonMember("consumerKnownLive", VonBool(false)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("arraySize", VonInt(arraySize))))
      }
      case DestroyStaticSizedArrayIntoLocalsH(structExpr, localTypes, localIndices) => {
        VonObject(
          "DestroyStaticSizedArrayIntoLocals",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(structExpr)),
            VonMember("arrayType", vonifyCoord(structExpr.resultType)),
            VonMember(
              "localTypes",
              VonArray(None, localTypes.map(localType => vonifyCoord(localType)).toVector)),
            VonMember(
              "localIndices",
              VonArray(None, localIndices.map(local => vonifyLocal(local))))))
      }
      case DestroyH(structExpr, localTypes, locals) => {
        VonObject(
          "Destroy",
          None,
          Vector(
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember(
              "localTypes",
              VonArray(None, localTypes.map(localType => vonifyCoord(localType)).toVector)),
            VonMember(
              "localIndices",
              VonArray(None, locals.map(local => vonifyLocal(local)))),
            VonMember(
              "localsKnownLives",
              VonArray(None, locals.map(local => VonBool(false))))))
      }
      case PushRuntimeSizedArrayH(arrayExpr, newcomerExpr) => {
        VonObject(
          "PushRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("newcomerExpr", vonifyExpression(newcomerExpr)),
            VonMember("newcomerType", vonifyCoord(newcomerExpr.resultType)),
            VonMember("newcomerKind", vonifyKind(newcomerExpr.resultType.kind)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case PopRuntimeSizedArrayH(arrayExpr, arrayElementType) => {
        VonObject(
          "PopRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case DestroyMutRuntimeSizedArrayH(arrayExpr) => {
        VonObject(
          "DestroyMutRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind))))
      }
      case DestroyImmRuntimeSizedArrayH(arrayExpr, consumerExpr, consumerMethod, arrayElementType) => {
        VonObject(
          "DestroyImmRuntimeSizedArray",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("consumerExpr", vonifyExpression(consumerExpr)),
            VonMember("consumerType", vonifyCoord(consumerExpr.resultType)),
            VonMember("consumerKind", vonifyKind(consumerExpr.resultType.kind)),
            VonMember("consumerMethod", vonifyPrototype(consumerMethod)),
            VonMember("arrayElementType", vonifyCoord(arrayElementType)),
            VonMember("consumerKnownLive", VonBool(false))))
      }
      case si @ StructToInterfaceUpcastH(sourceExpr, targetInterfaceRef) => {
        VonObject(
          "StructToInterfaceUpcast",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceStructType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceStructKind", vonifyStructH(sourceExpr.resultType.kind)),
            VonMember("targetInterfaceType", vonifyCoord(si.resultType)),
            VonMember("targetInterfaceKind", vonifyInterface(targetInterfaceRef))))
      }
      case InterfaceToInterfaceUpcastH(sourceExpr, targetInterfaceRef) => {
        vimpl()
      }
      case LocalStoreH(local, sourceExpr,localName) => {
        VonObject(
          "LocalStore",
          None,
          Vector(
            VonMember("local", vonifyLocal(local)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("localName", vonifyName(localName)),
            VonMember("knownLive", VonBool(false))))
      }
      case LocalLoadH(local, targetOwnership, localName) => {
        VonObject(
          "LocalLoad",
          None,
          Vector(
            VonMember("local", vonifyLocal(local)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("localName", vonifyName(localName))))
      }
      case MemberStoreH(resultType, structExpr, memberIndex, sourceExpr, memberName) => {
        VonObject(
          "MemberStore",
          None,
          Vector(
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember("structKnownLive", VonBool(false)),
            VonMember("memberIndex", VonInt(memberIndex)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("memberName", vonifyName(memberName))))
      }
      case ml @ MemberLoadH(structExpr, memberIndex, expectedMemberType, resultType, memberName) => {
        VonObject(
          "MemberLoad",
          None,
          Vector(
            VonMember("structExpr", vonifyExpression(structExpr)),
            VonMember("structId", vonifyStructH(structExpr.resultType.kind)),
            VonMember("structType", vonifyCoord(structExpr.resultType)),
            VonMember("structKnownLive", VonBool(false)),
            VonMember("memberIndex", VonInt(memberIndex)),
            VonMember("targetOwnership", vonifyOwnership(resultType.ownership)),
            VonMember("expectedMemberType", vonifyCoord(expectedMemberType)),
            VonMember("expectedResultType", vonifyCoord(resultType)),
            VonMember("memberName", vonifyName(memberName))))
      }
      case StaticSizedArrayStoreH(arrayExpr, indexExpr, sourceExpr, resultType) => {
        VonObject(
          "StaticSizedArrayStore",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case RuntimeSizedArrayStoreH(arrayExpr, indexExpr, sourceExpr, resultType) => {
        VonObject(
          "RuntimeSizedArrayStore",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("indexType", vonifyCoord(indexExpr.resultType)),
            VonMember("indexKind", vonifyKind(indexExpr.resultType.kind)),
            VonMember("sourceExpr", vonifyExpression(sourceExpr)),
            VonMember("sourceType", vonifyCoord(sourceExpr.resultType)),
            VonMember("sourceKind", vonifyKind(sourceExpr.resultType.kind)),
            VonMember("sourceKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case rsal @ RuntimeSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, resultType) => {
        VonObject(
          "RuntimeSizedArrayLoad",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("indexType", vonifyCoord(indexExpr.resultType)),
            VonMember("indexKind", vonifyKind(indexExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(rsal.resultType)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("expectedElementType", vonifyCoord(expectedElementType)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case NewMutRuntimeSizedArrayH(capacityExpr, elementType, resultType) => {
        VonObject(
          "NewMutRuntimeSizedArray",
          None,
          Vector(
            VonMember("capacityExpr", vonifyExpression(capacityExpr)),
            VonMember("capacityType", vonifyCoord(capacityExpr.resultType)),
            VonMember("capacityKind", vonifyKind(capacityExpr.resultType.kind)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(elementType))))
      }
      case NewImmRuntimeSizedArrayH(sizeExpr, generatorExpr, generatorMethod, elementType, resultType) => {
        VonObject(
          "NewImmRuntimeSizedArray",
          None,
          Vector(
            VonMember("sizeExpr", vonifyExpression(sizeExpr)),
            VonMember("sizeType", vonifyCoord(sizeExpr.resultType)),
            VonMember("sizeKind", vonifyKind(sizeExpr.resultType.kind)),
            VonMember("generatorExpr", vonifyExpression(generatorExpr)),
            VonMember("generatorType", vonifyCoord(generatorExpr.resultType)),
            VonMember("generatorKind", vonifyKind(generatorExpr.resultType.kind)),
            VonMember("generatorMethod", vonifyPrototype(generatorMethod)),
            VonMember("generatorKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(resultType))))
      }
      case StaticArrayFromCallableH(generatorExpr, generatorMethod, elementType, resultType) => {
        VonObject(
          "StaticArrayFromCallable",
          None,
          Vector(
            VonMember("generatorExpr", vonifyExpression(generatorExpr)),
            VonMember("generatorType", vonifyCoord(generatorExpr.resultType)),
            VonMember("generatorKind", vonifyKind(generatorExpr.resultType.kind)),
            VonMember("generatorMethod", vonifyPrototype(generatorMethod)),
            VonMember("generatorKnownLive", VonBool(false)),
            VonMember("resultType", vonifyCoord(resultType)),
            VonMember("elementType", vonifyCoord(resultType))))
      }
      case ssal @ StaticSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, arraySize, resultType) => {
        VonObject(
          "StaticSizedArrayLoad",
          None,
          Vector(
            VonMember("arrayExpr", vonifyExpression(arrayExpr)),
            VonMember("arrayType", vonifyCoord(arrayExpr.resultType)),
            VonMember("arrayKind", vonifyKind(arrayExpr.resultType.kind)),
            VonMember("arrayKnownLive", VonBool(false)),
            VonMember("indexExpr", vonifyExpression(indexExpr)),
            VonMember("resultType", vonifyCoord(ssal.resultType)),
            VonMember("targetOwnership", vonifyOwnership(targetOwnership)),
            VonMember("expectedElementType", vonifyCoord(expectedElementType)),
            VonMember("arraySize", VonInt(arraySize)),
            VonMember("resultType", vonifyCoord(resultType))))
      }
      case ExternCallH(functionExpr, argsExprs) => {
        VonObject(
          "ExternCall",
          None,
          Vector(
            VonMember("function", vonifyPrototype(functionExpr)),
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression))),
            VonMember("argTypes", VonArray(None, argsExprs.toVector.map(_.resultType).map(vonifyCoord)))))
      }
      case CallH(functionExpr, argsExprs) => {
        VonObject(
          "Call",
          None,
          Vector(
            VonMember("function", vonifyPrototype(functionExpr)),
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression)))))
      }
      case InterfaceCallH(argsExprs, virtualParamIndex, interfaceRefH, indexInEdge, functionType) => {
        VonObject(
          "InterfaceCall",
          None,
          Vector(
            VonMember("argExprs", VonArray(None, argsExprs.toVector.map(vonifyExpression))),
            VonMember("virtualParamIndex", VonInt(virtualParamIndex)),
            VonMember("interfaceRef", vonifyInterface(interfaceRefH)),
            VonMember("indexInEdge", VonInt(indexInEdge)),
            VonMember("functionType", vonifyPrototype(functionType))))
      }
      case IfH(conditionBlock, thenBlock, elseBlock, commonSupertype) => {
        VonObject(
          "If",
          None,
          Vector(
            VonMember("conditionBlock", vonifyExpression(conditionBlock)),
            VonMember("thenBlock", vonifyExpression(thenBlock)),
            VonMember("thenResultType", vonifyCoord(thenBlock.resultType)),
            VonMember("elseBlock", vonifyExpression(elseBlock)),
            VonMember("elseResultType", vonifyCoord(elseBlock.resultType)),
            VonMember("commonSupertype", vonifyCoord(commonSupertype))))
      }
      case WhileH(bodyBlock) => {
        VonObject(
          "While",
          None,
          Vector(
            VonMember("bodyBlock", vonifyExpression(bodyBlock))))
      }
      case ConsecutorH(nodes) => {
        VonObject(
          "Consecutor",
          None,
          Vector(
            VonMember("exprs", VonArray(None, nodes.map(node => vonifyExpression(node)).toVector))))
      }
      case m @ MutabilifyH(inner) => {
        VonObject(
          "Mutabilify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(inner)),
            VonMember("sourceType", vonifyCoord(inner.resultType)),
            VonMember("resultType", vonifyCoord(m.resultType))))
      }
      case m @ ImmutabilifyH(inner) => {
        VonObject(
          "Immutabilify",
          None,
          Vector(
            VonMember("sourceExpr", vonifyExpression(inner)),
            VonMember("sourceType", vonifyCoord(inner.resultType)),
            VonMember("resultType", vonifyCoord(m.resultType))))
      }
      case BlockH(inner) => {
        VonObject(
          "Block",
          None,
          Vector(
            VonMember("innerExpr", vonifyExpression(inner)),
            VonMember("innerType", vonifyCoord(inner.resultType))))
      }
      case IsSameInstanceH(left, right) => {
        VonObject(
          "Is",
          None,
          Vector(
            VonMember("leftExpr", vonifyExpression(left)),
            VonMember("leftExprType", vonifyCoord(left.resultType)),
            VonMember("rightExpr", vonifyExpression(right)),
            VonMember("rightExprType", vonifyCoord(right.resultType))))
      }
    }
  }
*/

// mig: fn vonify_function_ref
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_function_ref(&self, r#ref: &FunctionRefH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_function_ref");
    }
}
/*
  def vonifyFunctionRef(ref: FunctionRefH): IVonData = {
    val FunctionRefH(prototype) = ref

    VonObject(
      "FunctionRef",
      None,
      Vector(
        VonMember("prototype", vonifyPrototype(prototype))))
  }
*/

// mig: fn vonify_local
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_local(&self, local: Local<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_local");
    }
}
/*
  def vonifyLocal(local: Local): IVonData = {
    val Local(id, variability, tyype) = local

    VonObject(
      "Local",
      None,
      Vector(
        VonMember("id", vonifyVariableId(id)),
        VonMember("variability", vonifyVariability(variability)),
        VonMember("type", vonifyCoord(tyype))))
  }
*/

// mig: fn vonify_variable_id
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_variable_id(&self, id: VariableIdH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_variable_id");
    }
}
/*
  def vonifyVariableId(id: VariableIdH): IVonData = {
    val VariableIdH(number, height, maybeName) = id

    VonObject(
      "VariableId",
      None,
      Vector(
        VonMember("number", VonInt(number)),
        VonMember("height", VonInt(number)),
        VonMember(
          "optName",
          vonifyOptional[IdH](maybeName, x => vonifyName(x)))))
  }
*/

// mig: fn vonify_optional
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_optional<I>(&self, opt: Option<I>, func: impl Fn(I) -> IVonData) -> IVonData {
        panic!("Unimplemented: vonify_optional");
    }
}
/*
  def vonifyOptional[I](opt: Option[I], func: (I) => IVonData): IVonData = {
    opt match {
      case None => VonObject("None", None, Vector())
      case Some(value) => VonObject("Some", None, Vector(VonMember("value", func(value))))
    }
  }
*/

// mig: fn vonify_code_location
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_code_location(&self, code_location: CodeLocation) -> IVonData {
        panic!("Unimplemented: vonify_code_location");
    }
}
/*
  def vonifyCodeLocation(codeLocation: CodeLocation): IVonData = {
    val CodeLocation(file, offset) = codeLocation
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", NameHammer.translateFileCoordinate(file)),
        VonMember("offset", VonInt(offset))))
  }
*/

// mig: fn vonify_code_location2
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_code_location2(&self, code_location: &CodeLocationS<'s>) -> IVonData {
        panic!("Unimplemented: vonify_code_location2");
    }
}
/*
  def vonifyCodeLocation2(codeLocation: CodeLocationS): IVonData = {
    val CodeLocationS(file, offset) = codeLocation
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", NameHammer.translateFileCoordinate(file)),
        VonMember("offset", VonInt(offset))))
  }
*/

// mig: fn vonify_ranges
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_ranges(&self, ranges: &[RangeS<'s>]) -> IVonData {
        panic!("Unimplemented: vonify_ranges");
    }
}
/*
  def vonifyRanges(ranges: List[RangeS]): IVonData = {
    VonArray(
      None,
      ranges.map(vonifyRange).toVector)
  }
*/

// mig: fn vonify_range
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_range(&self, range: RangeS<'s>) -> IVonData {
        panic!("Unimplemented: vonify_range");
    }
}
/*
  def vonifyRange(range: RangeS): IVonData = {
    val RangeS(begin, end) = range
    VonObject(
      "Range",
      None,
      Vector(
        VonMember("begin", NameHammer.translateCodeLocation(begin)),
        VonMember("end", NameHammer.translateCodeLocation(end))))
  }
*/

// mig: fn vonify_name
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn vonify_name(&self, h: &IdH<'s, 'h>) -> IVonData {
        panic!("Unimplemented: vonify_name");
    }
}
/*
  def vonifyName(h: IdH): IVonData = {
    val IdH(localName, packageCoordinate, shortenedName, fullyQualifiedName) = h
    VonObject(
      "Name",
      None,
      Vector(
        VonMember("localName", VonStr(localName)),
        VonMember("packageCoordinate", NameHammer.translatePackageCoordinate(packageCoordinate)),
        VonMember("shortenedName", VonStr(shortenedName))))
  }
}
*/
