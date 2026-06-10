use std::collections::HashMap;
use crate::typing::compiler::Compiler;
use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::compiler_outputs::*;
use crate::typing::ast::ast::InterfaceEdgeBlueprintT;
use std::collections::HashSet;

/*
package dev.vale.typing

import dev.vale.typing.ast.{AsSubtypeTE, DestroyImmRuntimeSizedArrayTE, DestroyStaticSizedArrayIntoFunctionTE, EdgeT, FunctionCallTE, InterfaceEdgeBlueprintT, LockWeakTE, NewImmRuntimeSizedArrayTE, PrototypeT, SignatureT, StaticArrayFromCallableTE}
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names.{IdT, FunctionNameT}
import dev.vale.typing.templata.CoordTemplataT
import dev.vale.typing.types._
import dev.vale.{Collector, StrI, vassertOne, vassertSome, vcurious, vpass}
import dev.vale.typing.ast._
import dev.vale.typing.names._
import dev.vale.typing.types._

import scala.collection.mutable

*/
pub struct Reachables<'s, 't> {
    pub functions: HashSet<SignatureT<'s, 't>>,
    pub structs: HashSet<StructTT<'s, 't>>,
    pub static_sized_arrays: HashSet<StaticSizedArrayTT<'s, 't>>,
    pub runtime_sized_arrays: HashSet<RuntimeSizedArrayTT<'s, 't>>,
    pub interfaces: HashSet<InterfaceTT<'s, 't>>,
    pub edges: HashSet<EdgeT<'s, 't>>,
}

impl<'s, 't> Reachables<'s, 't> {
/*
//class Reachables(
//  val functions: mutable.Set[SignatureT],
//  val structs: mutable.Set[StructTT],
//  val staticSizedArrays: mutable.Set[StaticSizedArrayTT],
//  val runtimeSizedArrays: mutable.Set[RuntimeSizedArrayTT],
//  val interfaces: mutable.Set[InterfaceTT],
//  val edges: mutable.Set[EdgeT]
//) {
*/
pub fn size(&self) -> usize {
    panic!("Unimplemented: Slab 15 — body migration");
}
/*
//  def size = functions.size + structs.size + staticSizedArrays.size + runtimeSizedArrays.size + interfaces.size + edges.size
//}
//
//object Reachability {
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn find_reachables(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
    ) -> Reachables<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def findReachables(program: CompilerOutputs, edgeBlueprints: Vector[InterfaceEdgeBlueprint], edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]]): Reachables = {
//    val structs = program.getAllStructs()
//    val interfaces = program.getAllInterfaces()
//    val functions = program.getAllFunctions()
//    val exportedKinds = program.getKindExports.map(_.tyype).toSet
//    val exportedFunctionSignatures = program.getFunctionExports.map(_.prototype.toSignature).toSet
//
//    val exposedFunctions =
//      functions.filter(func => {
//        (func.header.fullName.last match {
//          case FunctionNameT(FunctionTemplateNameT(StrI("main"), _), _, _) => true
//          case _ => false
//        }) ||
//        exportedFunctionSignatures.contains(func.header.toSignature)
//      })
//    val exposedStructs = structs.filter(struct => exportedKinds.contains(struct.getRef))
//    val exposedInterfaces = interfaces.filter(interface => exportedKinds.contains(interface.getRef))
//    val reachables = new Reachables(mutable.Set(), mutable.Set(), mutable.Set(), mutable.Set(), mutable.Set(), mutable.Set())
//    var sizeBefore = 0
//    do {
//      vcurious(sizeBefore == 0) // do we ever need multiple iterations, or is the DFS good enough?
//      sizeBefore = reachables.size
//      exposedFunctions.map(_.header.toSignature).foreach(visitFunction(program, edgeBlueprints, edges, reachables, _))
//      exposedStructs.map(_.getRef).foreach(visitStruct(program, edgeBlueprints, edges, reachables, _))
//      exposedInterfaces.map(_.getRef).foreach(visitInterface(program, edgeBlueprints, edges, reachables, _))
//    } while (reachables.size != sizeBefore)
//    reachables
//  }
*/
    pub fn visit_function(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        callee_signature: SignatureT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitFunction(program: CompilerOutputs, edgeBlueprints: Vector[InterfaceEdgeBlueprint], edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]], reachables: Reachables, calleeSignature: SignatureT): Unit = {
//    if (reachables.functions.contains(calleeSignature)) {
//      return
//    }
//    reachables.functions.add(calleeSignature)
//    val function = vassertSome(program.lookupFunction(calleeSignature))
//    Collector.all(function, {
//      case FunctionCallTE(calleePrototype, _) => {
//        vpass()
//        vpass()
//        visitFunction(program, edgeBlueprints, edges, reachables, calleePrototype.toSignature)
//      }
//      case NewImmRuntimeSizedArrayTE(_, _, _, calleePrototype) => visitFunction(program, edgeBlueprints, edges, reachables, calleePrototype.toSignature)
//      case StaticArrayFromCallableTE(_, _, calleePrototype) => visitFunction(program, edgeBlueprints, edges, reachables, calleePrototype.toSignature)
//      case DestroyStaticSizedArrayIntoFunctionTE(_, _, _, calleePrototype) => visitFunction(program, edgeBlueprints, edges, reachables, calleePrototype.toSignature)
//      case DestroyImmRuntimeSizedArrayTE(_, _, _, calleePrototype) => visitFunction(program, edgeBlueprints, edges, reachables, calleePrototype.toSignature)
//      case sr @ StructTT(_) => visitStruct(program, edgeBlueprints, edges, reachables, sr)
//      case ir @ InterfaceTT(_) => visitInterface(program, edgeBlueprints, edges, reachables, ir)
//      case ssa @ contentsStaticSizedArrayTT(_, _, _, _) => visitStaticSizedArray(program, edgeBlueprints, edges, reachables, ssa)
//      case rsa @ contentsRuntimeSizedArrayTT(_, _) => visitRuntimeSizedArray(program, edgeBlueprints, edges, reachables, rsa)
//      case LockWeakTE(_, _, someConstructor, noneConstructor) => {
//        visitFunction(program, edgeBlueprints, edges, reachables, someConstructor.toSignature)
//        visitFunction(program, edgeBlueprints, edges, reachables, noneConstructor.toSignature)
//      }
//      case AsSubtypeTE(_, _, _, someConstructor, noneConstructor) => {
//        visitFunction(program, edgeBlueprints, edges, reachables, someConstructor.toSignature)
//        visitFunction(program, edgeBlueprints, edges, reachables, noneConstructor.toSignature)
//      }
//    })
//  }
//
*/
    pub fn visit_struct(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        struct_tt: StructTT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitStruct(program: CompilerOutputs, edgeBlueprints: Vector[InterfaceEdgeBlueprint], edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]], reachables: Reachables, structTT: StructTT): Unit = {
//    if (reachables.structs.contains(structTT)) {
//      return
//    }
//    reachables.structs.add(structTT)
//    val structDef = program.lookupStruct(structTT)
//    // Make sure the destructor got in, because for immutables, it's implicitly called by lots of instructions
//    // that let go of a reference.
//    if (structDef.mutability == ImmutableT) {
//      val destructorSignature = program.findImmDestructor(structTT).toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//    Collector.all(structDef, {
//      case sr @ StructTT(_) => visitStruct(program, edgeBlueprints, edges, reachables, sr)
//      case ir @ InterfaceTT(_) => visitInterface(program, edgeBlueprints, edges, reachables, ir)
//      case ssa @ contentsStaticSizedArrayTT(_, _, _, _) => visitStaticSizedArray(program, edgeBlueprints, edges, reachables, ssa)
//      case rsa @ contentsRuntimeSizedArrayTT(_, _) => visitRuntimeSizedArray(program, edgeBlueprints, edges, reachables, rsa)
//    })
//    edges.foreach({ case (interface, structToMethods) =>
//      structToMethods.get(structTT) match {
//        case None =>
//        case Some(methods) => visitImpl(program, edgeBlueprints, edges, reachables, interface, structTT, methods)
//      }
//    })
//
//    if (structDef.mutability == ImmutableT) {
//      val destructorSignature =
//        vassertOne(
//          program.getAllFunctions().find(func => {
//            func.header.toSignature match {
//              case SignatureT(FullNameT(_, _, FreeNameT(_, kind))) if kind == structTT => true
////              case SignatureT(FullNameT(_, _, AbstractVirtualFreeNameT(_, kind))) if kind == structTT => true
////              case SignatureT(FullNameT(_, _, OverrideVirtualFreeNameT(_, kind))) if kind == structTT => true
//              case _ => false
//            }
//          })).header.toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//  }
//
*/
    pub fn visit_interface(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        interface_tt: InterfaceTT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitInterface(program: CompilerOutputs, edgeBlueprints: Vector[InterfaceEdgeBlueprint], edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]], reachables: Reachables, interfaceTT: InterfaceTT): Unit = {
//    if (reachables.interfaces.contains(interfaceTT)) {
//      return
//    }
//    reachables.interfaces.add(interfaceTT)
//    val interfaceDef = program.lookupInterface(interfaceTT)
//    // Make sure the destructor got in, because for immutables, it's implicitly called by lots of instructions
//    // that let go of a reference.
//    if (interfaceDef.mutability == ImmutableT) {
//      val destructorSignature = program.findImmDestructor(interfaceTT).toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//    Collector.all(interfaceDef, {
//      case sr @ StructTT(_) => visitStruct(program, edgeBlueprints, edges, reachables, sr)
//      case ir @ InterfaceTT(_) => visitInterface(program, edgeBlueprints, edges, reachables, ir)
//      case ssa @ contentsStaticSizedArrayTT(_, _, _, _) => visitStaticSizedArray(program, edgeBlueprints, edges, reachables, ssa)
//      case rsa @ contentsRuntimeSizedArrayTT(_, _) => visitRuntimeSizedArray(program, edgeBlueprints, edges, reachables, rsa)
//    })
//    edgeBlueprints.find(_.interface == interfaceTT).get.superFamilyRootBanners.foreach(f => {
//      visitFunction(program, edgeBlueprints, edges, reachables, f.toSignature)
//    })
//    vassertSome(edges.get(interfaceTT)).foreach({ case (structTT, methods) =>
//      visitImpl(program, edgeBlueprints, edges, reachables, interfaceTT, structTT, methods)
//    })
//
//    if (interfaceDef.mutability == ImmutableT) {
//      val destructorSignature =
//        vassertOne(
//          program.getAllFunctions().find(func => {
//            func.header.toSignature match {
//              case SignatureT(FullNameT(_, _, FreeNameT(_, kind))) if kind == interfaceTT => true
////              case SignatureT(FullNameT(_, _, AbstractVirtualFreeNameT(_, kind))) if kind == interfaceTT => true
////              case SignatureT(FullNameT(_, _, OverrideVirtualFreeNameT(_, kind))) if kind == interfaceTT => true
//              case _ => false
//            }
//          })).header.toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//  }
//
*/
    pub fn visit_impl(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        interface_tt: InterfaceTT<'s, 't>,
        struct_tt: StructTT<'s, 't>,
        methods: &[&'t PrototypeT<'s, 't>],
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitImpl(
//      program: CompilerOutputs,
//      edgeBlueprints: Vector[InterfaceEdgeBlueprint],
//      edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]],
//      reachables: Reachables,
//      interfaceTT: InterfaceTT,
//      structTT: StructTT,
//      methods: Vector[PrototypeT]):
//  Unit = {
//    val edge = ast.EdgeT(structTT, interfaceTT, methods)
//    if (reachables.edges.contains(edge)) {
//      return
//    }
//    reachables.edges.add(edge)
//    edges.foreach(edge => {
//      visitStruct(program, edgeBlueprints, edges, reachables, structTT)
//      visitInterface(program, edgeBlueprints, edges, reachables, interfaceTT)
//      methods.map(_.toSignature).foreach(visitFunction(program, edgeBlueprints, edges, reachables, _))
//    })
//  }
//
*/
    pub fn visit_static_sized_array(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        ssa: StaticSizedArrayTT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitStaticSizedArray(
//    program: CompilerOutputs,
//    edgeBlueprints: Vector[InterfaceEdgeBlueprint],
//    edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]],
//    reachables: Reachables,
//    ssa: StaticSizedArrayTT
//  ): Unit = {
//    if (reachables.staticSizedArrays.contains(ssa)) {
//      return
//    }
//    reachables.staticSizedArrays.add(ssa)
//
//    // Make sure the destructor got in, because for immutables, it's implicitly called by lots of instructions
//    // that let go of a reference.
//    if (ssa.mutability == ImmutableT) {
//      val destructorSignature =
//        vassertOne(
//          program.getAllFunctions().find(func => {
//            func.header.toSignature match {
//              case SignatureT(FullNameT(_, _, FreeNameT(_, kind))) if kind == ssa => true
////              case SignatureT(FullNameT(_, _, AbstractVirtualFreeNameT(_, kind))) if kind == ssa => true
////              case SignatureT(FullNameT(_, _, OverrideVirtualFreeNameT(_, kind))) if kind == ssa => true
//              case _ => false
//            }
//          })).header.toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//  }
//
*/
    pub fn visit_runtime_sized_array(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        rsa: RuntimeSizedArrayTT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
//  def visitRuntimeSizedArray(
//    program: CompilerOutputs,
//    edgeBlueprints: Vector[InterfaceEdgeBlueprint],
//    edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]],
//    reachables: Reachables,
//    rsa: RuntimeSizedArrayTT
//  ): Unit = {
//    if (reachables.runtimeSizedArrays.contains(rsa)) {
//      return
//    }
//    reachables.runtimeSizedArrays.add(rsa)
//
//    // Make sure the destructor got in, because for immutables, it's implicitly called by lots of instructions
//    // that let go of a reference.
//    if (rsa.mutability == ImmutableT) {
//      val destructorSignature =
//        vassertOne(
//          program.getAllFunctions().find(func => {
//            func.header.toSignature match {
//              case SignatureT(FullNameT(_, _, FreeNameT(_, kind))) if kind == rsa => true
////              case SignatureT(FullNameT(_, _, AbstractVirtualFreeNameT(_, kind))) if kind == rsa => true
////              case SignatureT(FullNameT(_, _, OverrideVirtualFreeNameT(_, kind))) if kind == rsa => true
//              case _ => false
//            }
//          })).header.toSignature
//      visitFunction(program, edgeBlueprints, edges, reachables, destructorSignature)
//    }
//  }
//}
*/
}
