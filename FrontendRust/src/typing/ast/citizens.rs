use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::hinputs_t::*;
use crate::typing::ast::ast::*;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::scout_arena::ScoutArena;

/*
package dev.vale.typing.ast

import dev.vale.postparsing._
import dev.vale.typing.{InstantiationBoundArgumentsT, TemplataCompiler}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{StrI, vcurious, vfail, vpass}

import scala.collection.immutable.Map

// A "citizen" is a struct or an interface.
*/

/// Value-type (see @TFITCX)
pub enum CitizenDefinitionT<'s, 't> {
    Struct(&'t StructDefinitionT<'s, 't>),
    Interface(&'t InterfaceDefinitionT<'s, 't>),
}
/*
trait CitizenDefinitionT {
*/
impl<'s, 't> CitizenDefinitionT<'s, 't> where 's: 't {
    pub fn template_name(&self) -> IdT<'s, 't> {
        match self {
            CitizenDefinitionT::Struct(s) => {
                panic!("Unimplemented: template_name Struct");
                // s.templateName
            }
            CitizenDefinitionT::Interface(i) => {
                panic!("Unimplemented: template_name Interface");
                // i.templateName
            }
        }
    }
    /*
      def templateName: IdT[ICitizenTemplateNameT]
    */
    pub fn generic_param_types(&self, scout_arena: &ScoutArena<'s>) -> Vec<ITemplataType<'s>> {
        match self {
            CitizenDefinitionT::Struct(s) => s.generic_param_types(scout_arena),
            CitizenDefinitionT::Interface(i) => {
                panic!("Unimplemented: generic_param_types Interface");
                // i.genericParamTypes
            }
        }
    }
    /*
      def genericParamTypes: Vector[ITemplataType]
    */
    pub fn instantiated_citizen(&self) -> ICitizenTT<'s, 't> {
        match self {
            CitizenDefinitionT::Struct(s) => ICitizenTT::Struct(&s.instantiated_citizen),
            CitizenDefinitionT::Interface(i) => ICitizenTT::Interface(&i.instantiated_interface),
        }
    }
    /*
      def instantiatedCitizen: ICitizenTT
    */
    pub fn default_region(&self) -> RegionT {
        match self {
            CitizenDefinitionT::Struct(s) => {
                panic!("Unimplemented: default_region Struct");
                // s.defaultRegion
            }
            CitizenDefinitionT::Interface(i) => {
                panic!("Unimplemented: default_region Interface");
                // i.defaultRegion
            }
        }
    }
    /*
      def defaultRegion: RegionT
    }
    */
}
/// Arena-allocated (see @TFITCX)
pub struct StructDefinitionT<'s, 't> {
    pub template_name: IdT<'s, 't>,
    pub instantiated_citizen: StructTT<'s, 't>,
    pub attributes: &'t [ICitizenAttributeT<'s>],
    pub weakable: bool,
    pub mutability: ITemplataT<'s, 't>,
    pub members: &'t [IStructMemberT<'s, 't>],
    pub is_closure: bool,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
}
/*
case class StructDefinitionT(
  templateName: IdT[IStructTemplateNameT],
  // In typing pass, this will have placeholders. Monomorphizing will give it a real name.
  instantiatedCitizen: StructTT,
  attributes: Vector[ICitizenAttributeT],
  weakable: Boolean,
  mutability: ITemplataT[MutabilityTemplataType],
  members: Vector[IStructMemberT],
  isClosure: Boolean,
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]
) extends CitizenDefinitionT {
*/
impl<'s, 't> StructDefinitionT<'s, 't> {
    fn default_region(&self) -> RegionT {
        panic!("Unimplemented: default_region");
        // RegionT(DefaultRegionT)
    }
/*
  def defaultRegion: RegionT = RegionT(DefaultRegionT)
*/
    fn generic_param_types(&self, scout_arena: &ScoutArena<'s>) -> Vec<ITemplataType<'s>> {
        IStructNameT::try_from(self.instantiated_citizen.id.local_name).unwrap().template_args().iter().map(|t| t.tyype(scout_arena)).collect()
    }
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

//  override def getRef: StructTT = ref
//
//  def getMember(memberName: StrI): NormalStructMemberT = {
//    members.find(p => p.name.equals(CodeVarNameT(memberName))) match {
//      case None => vfail("Couldn't find member " + memberName)
//      case Some(member) => member
//    }
//  }
//
//  private def getIndex(memberName: IVarNameT): Int = {
//    members.zipWithIndex.find(p => p._1.name.equals(memberName)) match {
//      case None => vfail("wat")
//      case Some((member, index)) => index
//    }
//  }
*/
    pub fn get_member_and_index(&self, needle_name: &IVarNameT<'s, 't>) -> Option<(&NormalStructMemberT<'s, 't>, usize)> {
        for (index, member) in self.members.iter().enumerate() {
            match member {
                IStructMemberT::Normal(m) if &m.name == needle_name => {
                    return Some((m, index));
                }
                _ => {}
            }
        }
        None
    }
/*
  def getMemberAndIndex(needleName: IVarNameT): Option[(NormalStructMemberT, Int)] = {
    members.zipWithIndex
      .foreach({
        case (m @ NormalStructMemberT(hayName, _, _), index) if hayName == needleName => {
          return Some((m, index))
        }
        case _ =>
      })
    None
  }
}
*/
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructMemberT<'s, 't> {
    Normal(NormalStructMemberT<'s, 't>),
    Variadic(VariadicStructMemberT<'s, 't>),
}
/*
sealed trait IStructMemberT {
*/
impl<'s, 't> IStructMemberT<'s, 't> where 's: 't {
    pub fn name(&self) -> &IVarNameT<'s, 't> {
        match self {
            IStructMemberT::Normal(m) => &m.name,
            IStructMemberT::Variadic(m) => &m.name,
        }
    }
    /*
      def name: IVarNameT
    }
    */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NormalStructMemberT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub variability: VariabilityT,
    pub tyype: IMemberTypeT<'s, 't>,
}
/*
case class NormalStructMemberT(
  name: IVarNameT,
  // In the case of address members, this refers to the variability of the pointee variable.
  variability: VariabilityT,
  tyype: IMemberTypeT
) extends IStructMemberT {
  vpass()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariadicStructMemberT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub tyype: PlaceholderTemplataT<'s, 't>,
}
/*
case class VariadicStructMemberT(
  name: IVarNameT,
  tyype: PlaceholderTemplataT[PackTemplataType]
) extends IStructMemberT {
  vpass()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IMemberTypeT<'s, 't> {
    Address(AddressMemberTypeT<'s, 't>),
    Reference(ReferenceMemberTypeT<'s, 't>),
}
/*
sealed trait IMemberTypeT  {
*/
impl<'s, 't> IMemberTypeT<'s, 't> where 's: 't {
    pub fn reference(&self) -> CoordT<'s, 't> {
        match self {
            IMemberTypeT::Address(m) => m.reference,
            IMemberTypeT::Reference(m) => m.reference,
        }
    }
    /*
      def reference: CoordT
    */
    pub fn expect_reference_member(&self) -> &ReferenceMemberTypeT<'s, 't> {
        match self {
            IMemberTypeT::Reference(r) => r,
            IMemberTypeT::Address(_) => panic!("Expected reference member, was address member!"),
        }
    }
    /*
      def expectReferenceMember(): ReferenceMemberTypeT = {
        this match {
          case r @ ReferenceMemberTypeT(_) => r
          case a @ AddressMemberTypeT(_) => vfail("Expected reference member, was address member!")
        }
      }
    */
    pub fn expect_address_member(&self) -> &AddressMemberTypeT<'s, 't> {
        match self {
            IMemberTypeT::Reference(_) => panic!("Expected address member, was reference member!"),
            IMemberTypeT::Address(a) => a,
        }
    }
    /*
      def expectAddressMember(): AddressMemberTypeT = {
        this match {
          case r @ ReferenceMemberTypeT(_) => vfail("Expected reference member, was address member!")
          case a @ AddressMemberTypeT(_) => a
        }
      }
    }
    */
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressMemberTypeT<'s, 't> {
    pub reference: CoordT<'s, 't>,
}
/*
case class AddressMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceMemberTypeT<'s, 't> {
    pub reference: CoordT<'s, 't>,
}
/*
case class ReferenceMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
/// Arena-allocated (see @TFITCX)
pub struct InterfaceDefinitionT<'s, 't> {
    pub template_name: IdT<'s, 't>,
    pub instantiated_interface: InterfaceTT<'s, 't>,
    pub ref_: InterfaceTT<'s, 't>,
    pub attributes: &'t [ICitizenAttributeT<'s>],
    pub weakable: bool,
    pub mutability: ITemplataT<'s, 't>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    pub internal_methods: &'t [(PrototypeT<'s, 't>, usize)],
}
/*
case class InterfaceDefinitionT(
  templateName: IdT[IInterfaceTemplateNameT],
  instantiatedInterface: InterfaceTT,
  ref: InterfaceTT,
  attributes: Vector[ICitizenAttributeT],
  weakable: Boolean,
  mutability: ITemplataT[MutabilityTemplataType],
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  // This does not include abstract functions declared outside the interface.
  // Note from later: Though, sometimes macros add functions into the inside.
  // See IMRFDI for why we need to remember only the internal methods here.
  internalMethods: Vector[(PrototypeT[IFunctionNameT], Int)]
) extends CitizenDefinitionT {
*/
impl<'s, 't> InterfaceDefinitionT<'s, 't> {
    fn default_region(&self) -> RegionT {
        panic!("Unimplemented: default_region");
        // RegionT(DefaultRegionT)
    }
/*
  def defaultRegion: RegionT = RegionT(DefaultRegionT)
*/
    fn generic_param_types(&self) -> Vec<ITemplataType<'s>> {
        panic!("Unimplemented: generic_param_types");
        // instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
    }
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
    fn instantiated_citizen(&self) -> ICitizenTT<'s, 't> {
        panic!("Unimplemented: instantiated_citizen");
        // instantiatedInterface
    }
/*
  override def instantiatedCitizen: ICitizenTT = instantiatedInterface
*/
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
//  override def getRef = ref
}
*/
}