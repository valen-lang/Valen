// VISTODO: rename Hinputs everywhere
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{cI, sI, StructIT};
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::names::{IStructTemplateNameI, IInterfaceTemplateNameI, IImplTemplateNameI, IFunctionTemplateNameI};
use crate::instantiating::ast::ast::{
    EdgeI, FunctionDefinitionI, FunctionExportI, FunctionExternI, InterfaceEdgeBlueprintI,
    KindExportI, KindExternI, PrototypeI,
};
use crate::instantiating::ast::citizens::{ICitizenDefinitionI, InterfaceDefinitionI, StructDefinitionI};

/*
package dev.vale.instantiating.ast

import dev.vale.postparsing.IRuneS
import dev.vale.typing.ast._
import dev.vale.typing.names.{CitizenNameT, CitizenTemplateNameT, FunctionNameT, IFunctionNameT, IdT, LambdaCitizenNameT}
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{StrI, vassert, vassertOne, vassertSome, vcurious, vfail, vimpl}
import dev.vale.typing.ast._
import dev.vale.typing.names._
import dev.vale.typing.types._

import scala.collection.mutable
*/
// mig: struct InstantiationBoundArgumentsI
/// Temporary state (see @TFITCX)
pub struct InstantiationBoundArgumentsI<'s, 'i> where 's: 'i {
    pub rune_to_function_bound_arg: ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>>,
    pub caller_rune_to_callee_rune_to_reachable_func:
        ArenaIndexMap<'i, IRuneS<'s>, ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>>>,
    pub rune_to_impl_bound_arg: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, sI>>,
}

// mig: impl InstantiationBoundArgumentsI
/*
case class InstantiationBoundArgumentsI(
  runeToFunctionBoundArg: Map[IRuneS, PrototypeI[sI]],
  callerRuneToCalleeRuneToReachableFunc: Map[IRuneS, Map[IRuneS, PrototypeI[sI]]],
  runeToImplBoundArg: Map[IRuneS, IdI[sI, IImplNameI[sI]]])

*/
// mig: struct HinputsI
/// Temporary state (see @TFITCX) — top-level container for instantiated output.
pub struct HinputsI<'s, 'i> where 's: 'i {
    pub interfaces: &'i [InterfaceDefinitionI<'s, 'i, cI>],
    pub structs: &'i [&'i StructDefinitionI<'s, 'i, cI>],
    pub functions: &'i [&'i FunctionDefinitionI<'s, 'i>],
    pub interface_to_edge_blueprints:
        ArenaIndexMap<'i, IdI<'s, 'i, cI>, InterfaceEdgeBlueprintI<'s, 'i>>,
    pub interface_to_sub_citizen_to_edge:
        ArenaIndexMap<'i, IdI<'s, 'i, cI>, ArenaIndexMap<'i, IdI<'s, 'i, cI>, EdgeI<'s, 'i>>>,
    pub kind_exports: &'i [KindExportI<'s, 'i>],
    pub function_exports: &'i [FunctionExportI<'s, 'i>],
    pub kind_externs: ArenaIndexMap<'i, &'i StructIT<'s, 'i, cI>, KindExternI<'s, 'i>>,
    pub function_externs: &'i [FunctionExternI<'s, 'i>],
}
/*
case class HinputsI(
  interfaces: Vector[InterfaceDefinitionI],
  structs: Vector[StructDefinitionI],
//  emptyPackStructRef: StructTT,
  functions: Vector[FunctionDefinitionI],
//  immKindToDestructor: Map[KindT, PrototypeI],

  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interfaceToEdgeBlueprints: Map[IdI[cI, IInterfaceNameI[cI]], InterfaceEdgeBlueprintI],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interfaceToSubCitizenToEdge: Map[IdI[cI, IInterfaceNameI[cI]], Map[IdI[cI, ICitizenNameI[cI]], EdgeI]],

//  instantiationNameToInstantiationBounds: Map[IdI[cI, IInstantiationNameI[cI]], InstantiationBoundArgumentsI],

  kindExports: Vector[KindExportI],
  functionExports: Vector[FunctionExportI],
  kindExterns: Map[StructIT[cI], KindExternI],
  functionExterns: Vector[FunctionExternI],
) {
*/
// mig: fn to_string
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn to_string(&self) -> String {
        panic!("Unimplemented: to_string")
    }
}

// mig: fn lookup_function (humanName: String overload)
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_function_by_str(&self, human_name: &str) -> &'i crate::instantiating::ast::ast::FunctionDefinitionI<'s, 'i> {
        let matches: Vec<&&'i crate::instantiating::ast::ast::FunctionDefinitionI<'s, 'i>> = self.functions.iter().filter(|f| {
            match f.header.id.local_name {
                crate::instantiating::ast::names::INameI::FunctionNameIX(n) => n.template.human_name.0 == human_name,
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
}
/*
  override def toString: String = "HinputsI#()"

  private val subCitizenToInterfaceToEdgeMutable = mutable.HashMap[IdI[cI, ICitizenNameI[cI]], mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], EdgeI]]()
  interfaceToSubCitizenToEdge.foreach({ case (interface, subCitizenToEdge) =>
    subCitizenToEdge.foreach({ case (subCitizen, edge) =>
      subCitizenToInterfaceToEdgeMutable
        .getOrElseUpdate(subCitizen, mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], EdgeI]())
        .put(interface, edge)
    })
  })
  val subCitizenToInterfaceToEdge: Map[IdI[cI, ICitizenNameI[cI]], Map[IdI[cI, IInterfaceNameI[cI]], EdgeI]] =
    subCitizenToInterfaceToEdgeMutable.mapValues(_.toMap).toMap

*/
// mig: impl HinputsI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for HinputsI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for HinputsI` below.)
/*
override def hashCode(): Int = vfail() // Would need a really good reason to hash something this big

*/
// mig: fn lookup_struct
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_struct(
        &self,
        _struct_id: &IdI<'s, 'i, cI>,
    ) -> &'i StructDefinitionI<'s, 'i, cI> {
        self.structs.iter().find(|s| s.instantiated_citizen.id == *_struct_id).copied().expect("lookup_struct: not found")
    }
}
/*
  def lookupStruct(structId: IdI[cI, IStructNameI[cI]]): StructDefinitionI = {
    vassertSome(structs.find(_.instantiatedCitizen.id == structId))
  }

*/
// mig: fn lookup_interface
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_interface(
        &self,
        _interface_id: &IdI<'s, 'i, cI>,
    ) -> &'i InterfaceDefinitionI<'s, 'i, cI> {
        self.interfaces.iter().find(|i| i.instantiated_interface.id == *_interface_id).expect("lookup_interface: not found")
    }
}
/*
  def lookupInterface(interfaceId: IdI[cI, IInterfaceNameI[cI]]): InterfaceDefinitionI = {
    vassertSome(interfaces.find(_.instantiatedCitizen.id == interfaceId))
  }

*/
// mig: fn lookup_citizen
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_citizen(
        &self,
        _citizen_id: &IdI<'s, 'i, cI>,
    ) -> ICitizenDefinitionI<'s, 'i, cI> {
        panic!("Unimplemented: lookup_citizen")
    }
}
/*
  def lookupCitizen(citizenId: IdI[cI, ICitizenNameI[cI]]): CitizenDefinitionI = {
    vassertOne(
      structs.find(_.instantiatedCitizen.id == citizenId) ++ interfaces.find(_.instantiatedCitizen.id == citizenId))
  }

*/
// mig: fn lookup_struct_by_template
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_struct_by_template(
        &self,
        _struct_template_name: &IStructTemplateNameI<'s, 'i, cI>,
    ) -> &'i StructDefinitionI<'s, 'i, cI> {
        panic!("Unimplemented: lookup_struct_by_template")
    }
}
/*
  def lookupStructByTemplate(structTemplateName: IStructTemplateNameI[cI]): StructDefinitionI = {
    vassertSome(structs.find(_.instantiatedCitizen.id.localName.template == structTemplateName))
  }

*/
// mig: fn lookup_interface_by_template
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_interface_by_template(
        &self,
        _interface_template_name: &IInterfaceTemplateNameI<'s, 'i, cI>,
    ) -> &'i InterfaceDefinitionI<'s, 'i, cI> {
        panic!("Unimplemented: lookup_interface_by_template")
    }
}
/*
  def lookupInterfaceByTemplate(interfaceTemplateName: IInterfaceTemplateNameI[cI]): InterfaceDefinitionI = {
    vassertSome(interfaces.find(_.instantiatedCitizen.id.localName.template == interfaceTemplateName))
  }

*/
// mig: fn lookup_impl_by_template
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_impl_by_template(
        &self,
        _impl_template_name: &IImplTemplateNameI<'s, 'i, cI>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_impl_by_template")
    }
}
/*
  def lookupImplByTemplate(implTemplateName: IImplTemplateNameI[cI]): EdgeI = {
    vassertSome(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId.localName.template == implTemplateName))
  }

*/
// mig: fn lookup_edge
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_edge(
        &self,
        _impl_id: &IdI<'s, 'i, cI>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_edge")
    }
}
/*
  def lookupEdge(implId: IdI[cI, IImplNameI[cI]]): EdgeI = {
    vassertOne(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId == implId))
  }

//  def getInstantiationBoundArgs(instantiationName: IdI[cI, IInstantiationNameI[cI]]): InstantiationBoundArgumentsI = {
//    vassertSome(instantiationNameToInstantiationBounds.get(instantiationName))
//  }

//  def lookupStructByTemplateFullName(structTemplateId: IdI[IStructTemplateNameI]): StructDefinitionI = {
//    vassertSome(structs.find(_.templateName == structTemplateId))
//  }
//
//  def lookupInterfaceByTemplateFullName(interfaceTemplateId: IdI[IInterfaceTemplateNameI]): InterfaceDefinitionI = {
//    vassertSome(interfaces.find(_.templateName == interfaceTemplateId))
//  }
//
//  def lookupCitizenByTemplateFullName(interfaceTemplateId: IdI[ICitizenTemplateNameI]): CitizenDefinitionI = {
//    interfaceTemplateId match {
//      case IdI(packageCoord, initSteps, t: IStructTemplateNameI) => {
//        lookupStructByTemplateFullName(IdI(packageCoord, initSteps, t))
//      }
//      case IdI(packageCoord, initSteps, t: IInterfaceTemplateNameI) => {
//        lookupInterfaceByTemplateFullName(IdI(packageCoord, initSteps, t))
//      }
//    }
//  }
//
//  def lookupStructByTemplateName(structTemplateName: StructTemplateNameI): StructDefinitionI = {
//    vassertOne(structs.filter(_.templateName.localName == structTemplateName))
//  }
//
//  def lookupInterfaceByTemplateName(interfaceTemplateName: InterfaceTemplateNameI): InterfaceDefinitionI = {
//    vassertSome(interfaces.find(_.templateName.localName == interfaceTemplateName))
//  }

  // def lookupFunction(signature2: SignatureI[cI]): Option[FunctionDefinitionI] = {
  //   functions.find(_.header.toSignature == signature2).headOption
  // }

*/
// mig: fn lookup_function
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_function_by_template(
        &self,
        _func_template_name: &IFunctionTemplateNameI<'s, 'i, cI>,
    ) -> Option<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: lookup_function_by_template")
    }
}
/*
  def lookupFunction(funcTemplateName: IFunctionTemplateNameI[cI]): Option[FunctionDefinitionI] = {
    functions.find(_.header.id.localName.template == funcTemplateName).headOption
  }

*/
// mig: fn lookup_function
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_function(
        &self,
        _human_name: &str,
    ) -> &'i FunctionDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_function")
    }
}
/*
  def lookupFunction(humanName: String): FunctionDefinitionI = {
    val matches = functions.filter(f => {
      f.header.id.localName match {
        case FunctionNameIX(n, _, _) if n.humanName.str == humanName => true
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
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_struct_by_name(
        &self,
        human_name: &str,
    ) -> &'i StructDefinitionI<'s, 'i, cI> {
        let matches: Vec<&&'i StructDefinitionI<'s, 'i, cI>> = self.structs.iter().filter(|s| {
            match s.instantiated_citizen.id.local_name {
                crate::instantiating::ast::names::INameI::StructName(crate::instantiating::ast::names::StructNameI { template: crate::instantiating::ast::names::IStructTemplateNameI::StructTemplate(t), .. }) if t.human_name.0 == human_name => true,
                _ => false,
            }
        }).collect();
        if matches.is_empty() {
            panic!("Struct \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }
}
/*
  def lookupStruct(humanName: String): StructDefinitionI = {
    val matches = structs.filter(s => {
      s.instantiatedCitizen.id.localName match {
        case StructNameI(StructTemplateNameI(n), _) if n.str == humanName => true
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
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_impl(
        &self,
        _sub_citizen_it: &IdI<'s, 'i, cI>,
        _interface_it: &IdI<'s, 'i, cI>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_impl")
    }
}
/*
  def lookupImpl(
    subCitizenIT: IdI[cI, ICitizenNameI[cI]],
    interfaceIT: IdI[cI, IInterfaceNameI[cI]]):
  EdgeI = {
    vassertSome(
      vassertSome(interfaceToSubCitizenToEdge.get(interfaceIT))
        .get(subCitizenIT))
  }

*/
// mig: fn lookup_interface
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_interface_by_name(
        &self,
        _human_name: &str,
    ) -> &'i InterfaceDefinitionI<'s, 'i, cI> {
        panic!("Unimplemented: lookup_interface_by_name")
    }
}
/*
  def lookupInterface(humanName: String): InterfaceDefinitionI = {
    val matches = interfaces.filter(s => {
      s.instantiatedCitizen.id.localName match {
        case InterfaceNameI(InterfaceTemplateNameI(n), _) if n.str == humanName => true
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
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn lookup_user_function(
        &self,
        _human_name: &str,
    ) -> &'i FunctionDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_user_function")
    }
}
/*
  def lookupUserFunction(humanName: String): FunctionDefinitionI = {
    val matches =
      functions
        .filter(function => simpleNameI.unapply(function.header.id).contains(humanName))
        .filter(_.header.isUserFunction)
    if (matches.size == 0) {
      vfail("Not found!")
    } else if (matches.size > 1) {
      vfail("Multiple found!")
    }
    matches.head
  }

//  def nameIsLambdaIn(name: IdI[cI, IFunctionNameI[cI]], needleFunctionHumanName: String): Boolean = {
//    val first = name.steps.head
//    val lastTwo = name.steps.slice(name.steps.size - 2, name.steps.size)
//    (first, lastTwo) match {
//      case (
//        FunctionNameIX(FunctionTemplateNameI(StrI(hayFunctionHumanName), _), _, _),
//        Vector(
//          LambdaCitizenTemplateNameI(_),
//          LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(_, _), _, _)))
//        if hayFunctionHumanName == needleFunctionHumanName => true
//      case _ => false
//    }
//  }

//  def lookupLambdasIn(needleFunctionHumanName: String): Vector[FunctionDefinitionI] = {
//    functions.filter(f => nameIsLambdaIn(f.header.id, needleFunctionHumanName)).toVector
//  }

//  def lookupLambdaIn(needleFunctionHumanName: String): FunctionDefinitionI = {
//    vassertOne(lookupLambdasIn(needleFunctionHumanName))
//  }

*/
// mig: fn get_all_non_extern_functions
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn get_all_non_extern_functions(
        &self,
    ) -> Vec<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: get_all_non_extern_functions")
    }
}
/*
  def getAllNonExternFunctions: Iterable[FunctionDefinitionI] = {
    functions.filter(!_.header.isExtern)
  }

*/
// mig: fn get_all_user_functions
impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn get_all_user_functions(
        &self,
    ) -> Vec<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: get_all_user_functions")
    }
}
/*
  def getAllUserFunctions: Iterable[FunctionDefinitionI] = {
    functions.filter(_.header.isUserFunction)
  }
}
*/