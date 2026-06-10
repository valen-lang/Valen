/*
package dev.vale.typing

import dev.vale.postparsing.{IRuneS, ITemplataType}
import dev.vale.typing.ast._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{PackageCoordinate, StrI, vassert, vassertOne, vassertSome, vcurious, vfail, vimpl}
import dev.vale.typing.ast._
import dev.vale.typing.names._
import dev.vale.typing.types._

import scala.collection.mutable
*/
use std::collections::HashMap;
use crate::postparsing::names::IRuneS;
use crate::typing::ast::ast::{
    EdgeT, FunctionDefinitionT, FunctionExportT, FunctionExternT,
    InterfaceEdgeBlueprintT, KindExportT, KindExternT, PrototypeT, SignatureT,
};
use crate::typing::ast::citizens::{CitizenDefinitionT, InterfaceDefinitionT, StructDefinitionT};
use crate::typing::names::names::{
    FunctionTemplateNameT, INameT, IdT, ImplTemplateNameT, InterfaceTemplateNameT, StructTemplateNameT,
};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::arena_index_map::ArenaIndexMap;
// mig: struct InstantiationReachableBoundArgumentsT
/// Arena-allocated (see @TFITCX)
// Structural-equality opt-in: Scala uses case-class `==` on this type via
// `vassert(existing == instantiationBoundArgs)` in addInstantiationBounds.
// TFITCX/IEOIBZ ptr-eq is for identity types; this is a value-bag.
#[derive(PartialEq, Eq)]
pub struct InstantiationReachableBoundArgumentsT<'s, 't> {
    pub citizen_rune_to_reachable_prototype: ArenaIndexMap<'t, IRuneS<'s>, PrototypeT<'s, 't>>,
}
/*
case class InstantiationReachableBoundArgumentsT[R <: IFunctionNameT](
  citizenRuneToReachablePrototype: Map[IRuneS, PrototypeT[R]]
)
*/
/*

object InstantiationBoundArgumentsT {
*/
// mig: fn make
// Rust adaptation (SPDMX-B): interner threaded so the resulting InstantiationBoundArgumentsT
// is arena-allocated and shared by &'t reference (no Clone, per AASSNCMCX).
pub fn make<'s, 't>(
    interner: &TypingInterner<'s, 't>,
    rune_to_bound_prototype: Vec<(IRuneS<'s>, PrototypeT<'s, 't>)>,
    rune_to_citizen_rune_to_reachable_prototype: Vec<(IRuneS<'s>, &'t InstantiationReachableBoundArgumentsT<'s, 't>)>,
    rune_to_bound_impl: Vec<(IRuneS<'s>, IdT<'s, 't>)>,
) -> &'t InstantiationBoundArgumentsT<'s, 't> {
    interner.alloc(InstantiationBoundArgumentsT {
        rune_to_bound_prototype: interner.alloc_index_map_from_iter(rune_to_bound_prototype.into_iter()),
        rune_to_citizen_rune_to_reachable_prototype: interner.alloc_index_map_from_iter(rune_to_citizen_rune_to_reachable_prototype.into_iter()),
        rune_to_bound_impl: interner.alloc_index_map_from_iter(rune_to_bound_impl.into_iter()),
    })
}
/*
  def make[BF <: IFunctionNameT, BI <: IImplNameT](
      runeToBoundPrototype: Map[IRuneS, PrototypeT[BF]],
      runeToCitizenRuneToReachablePrototype: Map[IRuneS, InstantiationReachableBoundArgumentsT[BF]],
      runeToBoundImpl: Map[IRuneS, IdT[BI]]):
  InstantiationBoundArgumentsT[BF, BI] = {
    InstantiationBoundArgumentsT(
      (scala.collection.immutable.HashMap.newBuilder ++= runeToBoundPrototype).result(),
      (scala.collection.immutable.HashMap.newBuilder ++= runeToCitizenRuneToReachablePrototype).result(),
          (scala.collection.immutable.HashMap.newBuilder ++= runeToBoundImpl).result())
  }
}
*/
// mig: struct InstantiationBoundArgumentsT
/// Arena-allocated (see @TFITCX)
// Structural-equality opt-in: Scala uses case-class `==` on this type via
// `vassert(existing == instantiationBoundArgs)` in addInstantiationBounds.
// TFITCX/IEOIBZ ptr-eq is for identity types; this is a value-bag.
#[derive(PartialEq, Eq)]
pub struct InstantiationBoundArgumentsT<'s, 't> {
    pub rune_to_bound_prototype: ArenaIndexMap<'t, IRuneS<'s>, PrototypeT<'s, 't>>,
    pub rune_to_citizen_rune_to_reachable_prototype: ArenaIndexMap<'t, IRuneS<'s>, &'t InstantiationReachableBoundArgumentsT<'s, 't>>,
    pub rune_to_bound_impl: ArenaIndexMap<'t, IRuneS<'s>, IdT<'s, 't>>,
}
/*
case class InstantiationBoundArgumentsT[BF <: IFunctionNameT, BI <: IImplNameT](
  // This is the callee's rune to the prototype that satisfies it.
  // If this is at the call site, then this might be a real function like func drop(int)void.
  // If this is the instantiation bound params in the definition, then this will be a bound like func drop(T)void.
  runeToBoundPrototype: scala.collection.immutable.HashMap[IRuneS, PrototypeT[BF]],
  // This is empty for structs and interfaces.
  // For functions, this includes all the bounds that are inherited from structs and interfaces.
  runeToCitizenRuneToReachablePrototype: scala.collection.immutable.HashMap[IRuneS, InstantiationReachableBoundArgumentsT[BF]],
  // Same as runeToBoundPrototype but for impls.
  runeToBoundImpl: scala.collection.immutable.HashMap[IRuneS, IdT[BI]]) {

//  println("Made Inst bound args size:")
//  println(runeToBoundPrototype.size)
//  println(runeToCitizenRuneToReachablePrototype.size)
//  println(runeToBoundImpl.size)
*/
// mig: impl InstantiationBoundArgumentsT
impl<'s, 't> InstantiationBoundArgumentsT<'s, 't> {
    pub fn new() -> Self {
        panic!("Unimplemented: new");
    }
/*
  vassert(!runeToCitizenRuneToReachablePrototype.exists(_._2.citizenRuneToReachablePrototype.isEmpty))
}
*/
}
// mig: struct HinputsT
/// Temporary state (see @TFITCX)
pub struct HinputsT<'s, 't> {
    pub interfaces: Vec<&'t InterfaceDefinitionT<'s, 't>>,
    pub structs: Vec<&'t StructDefinitionT<'s, 't>>,
    pub functions: Vec<&'t FunctionDefinitionT<'s, 't>>,

    pub interface_to_edge_blueprints: HashMap<
        IdT<'s, 't>,
        &'t InterfaceEdgeBlueprintT<'s, 't>,
    >,
    pub interface_to_sub_citizen_to_edge: HashMap<
        IdT<'s, 't>,
        HashMap<IdT<'s, 't>, &'t EdgeT<'s, 't>>,
    >,

    pub instantiation_name_to_instantiation_bounds: HashMap<
        IdT<'s, 't>,
        &'t InstantiationBoundArgumentsT<'s, 't>,
    >,

    pub kind_exports: Vec<&'t KindExportT<'s, 't>>,
    pub function_exports: Vec<&'t FunctionExportT<'s, 't>>,
    pub kind_externs: Vec<&'t KindExternT<'s, 't>>,
    pub function_externs: Vec<&'t FunctionExternT<'s, 't>>,

    pub sub_citizen_to_interface_to_edge: HashMap<
        IdT<'s, 't>,
        HashMap<IdT<'s, 't>, &'t EdgeT<'s, 't>>,
    >,
}
/*
case class HinputsT(
  interfaces: Vector[InterfaceDefinitionT],
  structs: Vector[StructDefinitionT],
  functions: Vector[FunctionDefinitionT],

  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interfaceToEdgeBlueprints: Map[IdT[IInterfaceNameT], InterfaceEdgeBlueprintT],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interfaceToSubCitizenToEdge: Map[IdT[IInterfaceNameT], Map[IdT[ICitizenNameT], EdgeT]],

  instantiationNameToInstantiationBounds: Map[IdT[IInstantiationNameT], InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]],

  kindExports: Vector[KindExportT],
  functionExports: Vector[FunctionExportT],
  kindExterns: Vector[KindExternT],
  functionExterns: Vector[FunctionExternT],
) {
*/
// mig: impl HinputsT
impl<'s, 't> HinputsT<'s, 't> {
    pub fn new() -> Self {
        panic!("Unimplemented: new");
    }
    /*
      private val subCitizenToInterfaceToEdgeMutable = mutable.HashMap[IdT[ICitizenNameT], mutable.HashMap[IdT[IInterfaceNameT], EdgeT]]()
      interfaceToSubCitizenToEdge.foreach({ case (interface, subCitizenToEdge) =>
        subCitizenToEdge.foreach({ case (subCitizen, edge) =>
          subCitizenToInterfaceToEdgeMutable
            .getOrElseUpdate(subCitizen, mutable.HashMap[IdT[IInterfaceNameT], EdgeT]())
            .put(interface, edge)
        })
      })
      val subCitizenToInterfaceToEdge: Map[IdT[ICitizenNameT], Map[IdT[IInterfaceNameT], EdgeT]] =
        subCitizenToInterfaceToEdgeMutable.mapValues(_.toMap).toMap

    */
    // mig: fn equals
    /*
      override def equals(obj: Any): Boolean = vcurious();
    */
    // mig: fn hash_code
    /*
      override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big
    */
    // mig: fn lookup_struct
    pub fn lookup_struct(&self, struct_id: IdT<'s, 't>) -> &'t StructDefinitionT<'s, 't> {
        *self.structs.iter().find(|s| s.instantiated_citizen.id == struct_id).expect("lookup_struct: missing")
    }
    /*
      def lookupStruct(structId: IdT[IStructNameT]): StructDefinitionT = {
        vassertSome(structs.find(_.instantiatedCitizen.id == structId))
      }
    */
    // mig: fn lookup_struct_by_template
    pub fn lookup_struct_by_template(&self, struct_template_name: StructTemplateNameT) -> StructDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_struct_by_template");
    }
    /*
      def lookupStructByTemplate(structTemplateName: IStructTemplateNameT): StructDefinitionT = {
        vassertSome(structs.find(_.instantiatedCitizen.id.localName.template == structTemplateName))
      }
    */
    // mig: fn lookup_interface_by_template
    pub fn lookup_interface_by_template(&self, interface_template_name: InterfaceTemplateNameT) -> InterfaceDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_interface_by_template");
    }
    /*
      def lookupInterfaceByTemplate(interfaceTemplateName: IInterfaceTemplateNameT): InterfaceDefinitionT = {
        vassertSome(interfaces.find(_.instantiatedCitizen.id.localName.template == interfaceTemplateName))
      }
    */
    // mig: fn lookup_impl_by_template
    pub fn lookup_impl_by_template(&self, impl_template_name: ImplTemplateNameT) -> EdgeT<'s, 't> {
        panic!("Unimplemented: lookup_impl_by_template");
    }
    /*
      def lookupImplByTemplate(implTemplateName: IImplTemplateNameT): EdgeT = {
        vassertSome(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId.localName.template == implTemplateName))
      }
    */
    // mig: fn lookup_interface
    pub fn lookup_interface(&self, interface_id: IdT<'s, 't>) -> InterfaceDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_interface");
    }
    /*
      def lookupInterface(interfaceId: IdT[IInterfaceNameT]): InterfaceDefinitionT = {
        vassertSome(interfaces.find(_.instantiatedCitizen.id == interfaceId))
      }
    */
    // mig: fn lookup_edge
    pub fn lookup_edge(&self, impl_id: IdT<'s, 't>) -> &'t EdgeT<'s, 't> {
        let matches: Vec<&&'t EdgeT<'s, 't>> = self.interface_to_sub_citizen_to_edge
            .values()
            .flat_map(|m| m.values())
            .filter(|edge| edge.edge_id == impl_id)
            .collect();
        assert!(matches.len() == 1, "vassertOne: expected exactly one edge for impl_id {:?}, got {}", impl_id, matches.len());
        *matches[0]
    }
    /*
      def lookupEdge(implId: IdT[IImplNameT]): EdgeT = {
        vassertOne(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId == implId))
      }
    */
    // mig: fn get_instantiation_bound_args
    pub fn get_instantiation_bound_args(&self, instantiation_name: IdT<'s, 't>) -> &'t InstantiationBoundArgumentsT<'s, 't> {
        *self.instantiation_name_to_instantiation_bounds.get(&instantiation_name).unwrap()
    }
    /*
      def getInstantiationBoundArgs(instantiationName: IdT[IInstantiationNameT]): InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT] = {
        vassertSome(instantiationNameToInstantiationBounds.get(instantiationName))
      }
    */
    // mig: fn lookup_struct_by_template_id
    pub fn lookup_struct_by_template_id(&self, struct_template_id: IdT<'s, 't>) -> StructDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_struct_by_template_id");
    }
    /*
      def lookupStructByTemplateId(structTemplateId: IdT[IStructTemplateNameT]): StructDefinitionT = {
        vassertSome(structs.find(_.templateName == structTemplateId))
      }
    */
    // mig: fn lookup_interface_by_template_id
    pub fn lookup_interface_by_template_id(&self, interface_template_id: IdT<'s, 't>) -> InterfaceDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_interface_by_template_id");
    }
    /*
      def lookupInterfaceByTemplateId(interfaceTemplateId: IdT[IInterfaceTemplateNameT]): InterfaceDefinitionT = {
        vassertSome(interfaces.find(_.templateName == interfaceTemplateId))
      }
    */
    // mig: fn lookup_citizen_by_template_id
    pub fn lookup_citizen_by_template_id(&self, citizen_template_id: IdT<'s, 't>) -> CitizenDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_citizen_by_template_id");
    }
    /*
      def lookupCitizenByTemplateId(interfaceTemplateId: IdT[ICitizenTemplateNameT]): CitizenDefinitionT = {
        interfaceTemplateId match {
          case IdT(packageCoord, initSteps, t: IStructTemplateNameT) => {
            lookupStructByTemplateId(IdT(packageCoord, initSteps, t))
          }
          case IdT(packageCoord, initSteps, t: IInterfaceTemplateNameT) => {
            lookupInterfaceByTemplateId(IdT(packageCoord, initSteps, t))
          }
        }
      }
    */
    // mig: fn lookup_struct_by_template_name
    // Rust adaptation: Scala's `_.templateName.localName == structTemplateName`
    // compares directly because Scala's covariant `templateName.localName`
    // narrows to `IStructTemplateNameT`. Rust's `template_name.local_name`
    // stays in the wide `INameT` enum, so we must extract the struct-template
    // case before structural comparison. `vassertOne` is inlined as a match on
    // the result count.
    pub fn lookup_struct_by_template_name(&self, struct_template_name: StructTemplateNameT<'s, 't>) -> &'t StructDefinitionT<'s, 't> {
        let matches: Vec<&'t StructDefinitionT<'s, 't>> = self.structs.iter()
            .filter(|s| match s.template_name.local_name {
                INameT::StructTemplate(t) => *t == struct_template_name,
                _ => false,
            })
            .copied()
            .collect();
        match matches.len() {
            1 => matches[0],
            0 => panic!("lookup_struct_by_template_name: not found: {:?}", struct_template_name),
            _ => panic!("lookup_struct_by_template_name: multiple found: {:?}", struct_template_name),
        }
    }
    /*
      def lookupStructByTemplateName(structTemplateName: StructTemplateNameT): StructDefinitionT = {
        vassertOne(structs.filter(_.templateName.localName == structTemplateName))
      }
    */
    // mig: fn lookup_interface_by_template_name
    pub fn lookup_interface_by_template_name(&self, interface_template_name: &'t InterfaceTemplateNameT<'s, 't>) -> &'t InterfaceDefinitionT<'s, 't> {
        self.interfaces.iter().copied()
            .find(|i| i.template_name.local_name == INameT::InterfaceTemplate(interface_template_name))
            .unwrap_or_else(|| panic!("lookup_interface_by_template_name: not found"))
    }
    /*
      def lookupInterfaceByTemplateName(interfaceTemplateName: InterfaceTemplateNameT): InterfaceDefinitionT = {
        vassertSome(interfaces.find(_.templateName.localName == interfaceTemplateName))
      }
    */
    // mig: fn lookup_function
    pub fn lookup_function_by_signature(&self, signature2: SignatureT<'s, 't>) -> Option<&'t FunctionDefinitionT<'s, 't>> {
        self.functions.iter().copied().find(|f| f.header.to_signature() == signature2)
    }
    /*
      def lookupFunction(signature2: SignatureT): Option[FunctionDefinitionT] = {
        functions.find(_.header.toSignature == signature2).headOption
      }
    */
    // mig: fn lookup_function
    pub fn lookup_function_by_template(&self, func_template_name: FunctionTemplateNameT) -> Option<&'t FunctionDefinitionT<'s, 't>> {
        panic!("Unimplemented: lookup_function_by_template");
    }
    /*
      def lookupFunction(funcTemplateName: IFunctionTemplateNameT): Option[FunctionDefinitionT] = {
        functions.find(_.header.id.localName.template == funcTemplateName).headOption
      }
    */
    // mig: fn lookup_function
    pub fn lookup_function_by_str(&self, human_name: &str) -> &'t FunctionDefinitionT<'s, 't> {
        let matches: Vec<_> = self.functions.iter().filter(|f| {
            match &f.header.id.local_name {
                INameT::Function(func_name) if func_name.template.human_name.as_str() == human_name => true,
                _ => false,
            }
        }).collect();
        if matches.is_empty() {
            panic!("Function \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }
    /*
      def lookupFunction(humanName: String): FunctionDefinitionT = {
        val matches = functions.filter(f => {
          f.header.id.localName match {
            case FunctionNameT(n, _, _) if n.humanName.str == humanName => true
            case _ => false
          }
        })
        if (matches.size == 0) {
          vfail("Function \"" + humanName + "\" not found!")
        } else if (matches.size > 1) {
          vfail("Multiple found!")
        }
        matches.head
      }
    */
    // mig: fn lookup_struct
    pub fn lookup_struct_by_str(&self, human_name: &str) -> &'t StructDefinitionT<'s, 't> {
        let matches: Vec<_> = self.structs.iter().filter(|s| {
            match &s.template_name.local_name {
                INameT::StructTemplate(t) if t.human_name.as_str() == human_name => true,
                _ => false,
            }
        }).copied().collect();
        if matches.is_empty() {
            panic!("Struct \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }
    /*
      def lookupStruct(humanName: String): StructDefinitionT = {
        val matches = structs.filter(s => {
          s.templateName.localName match {
            case StructTemplateNameT(n) if n.str == humanName => true
            case _ => false
          }
        })
        if (matches.size == 0) {
          vfail("Struct \"" + humanName + "\" not found!")
        } else if (matches.size > 1) {
          vfail("Multiple found!")
        }
        matches.head
      }
    */
    // mig: fn lookup_impl
    pub fn lookup_impl(&self, sub_citizen_tt: IdT<'s, 't>, interface_tt: IdT<'s, 't>) -> &'t EdgeT<'s, 't> {
        self.interface_to_sub_citizen_to_edge
            .get(&interface_tt)
            .unwrap_or_else(|| panic!("lookup_impl: interface not found"))
            .get(&sub_citizen_tt)
            .unwrap_or_else(|| panic!("lookup_impl: sub citizen not found"))
    }
    /*
      def lookupImpl(
        subCitizenTT: IdT[ICitizenNameT],
        interfaceTT: IdT[IInterfaceNameT]):
      EdgeT = {
        vassertSome(
          vassertSome(interfaceToSubCitizenToEdge.get(interfaceTT))
            .get(subCitizenTT))
      }
    */
    // mig: fn lookup_interface
    pub fn lookup_interface_by_human_name(&self, human_name: &str) -> &'t InterfaceDefinitionT<'s, 't> {
        let matches: Vec<_> = self.interfaces.iter().filter(|i| {
            match &i.template_name.local_name {
                INameT::InterfaceTemplate(t) if t.human_namee.as_str() == human_name => true,
                _ => false,
            }
        }).copied().collect();
        if matches.is_empty() {
            panic!("Interface \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }
    /*
      def lookupInterface(humanName: String): InterfaceDefinitionT = {
        val matches = interfaces.filter(s => {
          s.templateName.localName match {
            case InterfaceTemplateNameT(n) if n.str == humanName => true
            case _ => false
          }
        })
        if (matches.size == 0) {
          vfail("Interface \"" + humanName + "\" not found!")
        } else if (matches.size > 1) {
          vfail("Multiple found!")
        }
        matches.head
      }
    */
    // mig: fn lookup_user_function
    pub fn lookup_user_function(&self, human_name: &str) -> FunctionDefinitionT<'s, 't> {
        panic!("Unimplemented: lookup_user_function");
    }
    /*
      def lookupUserFunction(humanName: String): FunctionDefinitionT = {
        val matches =
          functions
            .filter(function => simpleNameT.unapply(function.header.id).contains(humanName))
            .filter(_.header.isUserFunction)
        if (matches.size == 0) {
          vfail("Not found!")
        } else if (matches.size > 1) {
          vfail("Multiple found!")
        }
        matches.head
      }
    */
    // mig: fn name_is_lambda_in
    pub fn name_is_lambda_in(&self, name: IdT<'s, 't>, needle_function_human_name: &str) -> bool {
        let steps = name.steps();
        let first = steps[0];
        let last_two = &steps[steps.len().saturating_sub(2)..steps.len()];
        match (first, last_two) {
            (
                INameT::Function(f),
                [
                    INameT::LambdaCitizenTemplate(_),
                    INameT::LambdaCallFunction(_),
                ],
            ) if f.template.human_name.0 == needle_function_human_name => true,
            _ => false,
        }
    }
    /*
      def nameIsLambdaIn(name: IdT[IFunctionNameT], needleFunctionHumanName: String): Boolean = {
        val first = name.steps.head
        val lastTwo = name.steps.slice(name.steps.size - 2, name.steps.size)
        (first, lastTwo) match {
          case (
            FunctionNameT(FunctionTemplateNameT(StrI(hayFunctionHumanName), _), _, _),
            Vector(
              LambdaCitizenTemplateNameT(_),
              LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(_, _), _, _)))
            if hayFunctionHumanName == needleFunctionHumanName => true
          case _ => false
        }
      }
    */
    // mig: fn lookup_lambdas_in
    pub fn lookup_lambdas_in(&self, needle_function_human_name: &str) -> Vec<&'t FunctionDefinitionT<'s, 't>> {
        self.functions.iter().copied().filter(|f| self.name_is_lambda_in(f.header.id, needle_function_human_name)).collect()
    }
    /*
      def lookupLambdasIn(needleFunctionHumanName: String): Vector[FunctionDefinitionT] = {
        functions.filter(f => nameIsLambdaIn(f.header.id, needleFunctionHumanName)).toVector
      }
    */
    // mig: fn lookup_lambda_in
    pub fn lookup_lambda_in(&self, needle_function_human_name: &str) -> &'t FunctionDefinitionT<'s, 't> {
        let lambdas = self.lookup_lambdas_in(needle_function_human_name);
        assert_eq!(lambdas.len(), 1);
        lambdas[0]
    }
    /*
      def lookupLambdaIn(needleFunctionHumanName: String): FunctionDefinitionT = {
        vassertOne(lookupLambdasIn(needleFunctionHumanName))
      }
    */
    // mig: fn get_all_user_functions
    pub fn get_all_user_functions(&self) -> Vec<&'t FunctionDefinitionT<'s, 't>> {
        self.functions.iter().copied().filter(|f| f.header.is_user_function()).collect()
    }
    /*
      // def getAllNonExternFunctions: Iterable[FunctionDefinitionT] = {
      //   functions.filter(!_.header.isExtern)
      // }

      def getAllUserFunctions: Iterable[FunctionDefinitionT] = {
        functions.filter(_.header.isUserFunction)
      }
    }
    */
}