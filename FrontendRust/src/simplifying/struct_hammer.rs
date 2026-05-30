// From Frontend/SimplifyingPass/src/dev/vale/simplifying/StructHammer.scala
//
// Per typing-pass `Compiler` precedent, `StructHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::ast::{EdgeH, InterfaceDefinitionH, InterfaceMethodH, StructDefinitionH, StructMemberH};
use crate::final_ast::types::{CoordH, InterfaceHT, Mutability, OpaqueHT, StructHT};
use crate::instantiating::ast::ast::EdgeI;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::citizens::{
    AddressMemberTypeI, InterfaceDefinitionI, ReferenceMemberTypeI, StructDefinitionI, StructMemberI,
};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::types::{cI, CoordI, InterfaceIT, StructIT, VariabilityI};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;

/*
package dev.vale.simplifying

import dev.vale._
import dev.vale.finalast._
import dev.vale.instantiating.ast._

import scala.collection.immutable.ListMap


class StructHammer(
    interner: Interner,
    keywords: Keywords,
    nameHammer: NameHammer,
    translatePrototype: (HinputsI, HamutsBox, PrototypeI[cI]) => PrototypeH,
    translateReference: (HinputsI, HamutsBox, CoordI[cI]) => CoordH[KindHT]) {
*/

// mig: fn translate_interfaces
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_interfaces(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
    )
    {
        for interface in hinputs.interfaces.iter() {
            self.translate_interface(hinputs, hamuts, interface.instantiated_interface);
        }
    }
}
/*
  def translateInterfaces(hinputs: HinputsI, hamuts: HamutsBox): Unit = {
    hinputs.interfaces.foreach(interface => translateInterface(hinputs, hamuts, interface.instantiatedInterface))
  }
*/

// mig: fn translate_interface_methods
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_interface_methods(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        interface_tt: &'i InterfaceIT<'s, 'i, cI>,
    ) -> Vec<InterfaceMethodH<'s, 'h>>
    {
        panic!("Unimplemented: translate_interface_methods");
    }
}
/*
  def translateInterfaceMethods(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      interfaceTT: InterfaceIT[cI]):
  Vector[InterfaceMethodH] = {

    val edgeBlueprint = vassertSome(hinputs.interfaceToEdgeBlueprints.get(interfaceTT.id))

    val methodsH =
      edgeBlueprint.superFamilyRootHeaders.map({ case (superFamilyPrototype, virtualParamIndex) =>
//        val header = vassertSome(hinputs.lookupFunction(superFamilyPrototype.toSignature)).header
        val prototypeH = translatePrototype(hinputs, hamuts, superFamilyPrototype)
        InterfaceMethodH(prototypeH, virtualParamIndex)
      })

    methodsH
  }
*/

// mig: fn translate_interface
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_interface(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        interface_it: &'i InterfaceIT<'s, 'i, cI>,
    ) -> &'h InterfaceHT<'s, 'h>
    {
        panic!("Unimplemented: translate_interface");
    }
}
/*
  def translateInterface(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    interfaceIT: InterfaceIT[cI]):
  InterfaceHT = {
    hamuts.interfaceTToInterfaceH.get(interfaceIT) match {
      case Some(structRefH) => structRefH
      case None => {
        val fullNameH = nameHammer.translateFullName(hinputs, hamuts, interfaceIT.id)
        // This is the only place besides InterfaceDefinitionH that can make a InterfaceRefH
        val temporaryInterfaceRefH = InterfaceHT(fullNameH);
        hamuts.forwardDeclareInterface(interfaceIT, temporaryInterfaceRefH)
        val interfaceDefI = hinputs.lookupInterface(interfaceIT.id);


        val methodsH = translateInterfaceMethods(hinputs, hamuts, interfaceIT)

        val interfaceDefH =
          InterfaceDefinitionH(
            fullNameH,
            interfaceDefI.weakable,
            Conversions.evaluateMutabilityTemplata(interfaceDefI.mutability),
            Vector.empty, // super interfaces
            methodsH)
        hamuts.addInterface(interfaceIT, interfaceDefH)
        vassert(interfaceDefH.getRef == temporaryInterfaceRefH)

        // Make sure there's a destructor for this shared interface.
        interfaceDefI.mutability match {
          case MutableI => None
          case ImmutableI => {
//            vassert(
//              hinputs.functions.exists(function => {
//                function.header.fullName match {
//                  case FullNameI(_, _, FreeNameI(_, _, k)) if k.kind == interfaceDefI.instantiatedInterface => true
//                  case _ => false
//                }
//              }))
          }
        }

        (interfaceDefH.getRef)
      }
    }
  }
*/

// mig: fn translate_structs
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_structs(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
    )
    {
        for struct_def_i in hinputs.structs.iter() {
            if hinputs.kind_externs.contains_key(struct_def_i.instantiated_citizen) {
                self.translate_opaque_i(hinputs, hamuts, struct_def_i.instantiated_citizen);
            } else {
                self.translate_struct_i(hinputs, hamuts, struct_def_i.instantiated_citizen);
            }
        }
    }
}
/*
  def translateStructs(hinputs: HinputsI, hamuts: HamutsBox): Unit = {
    hinputs.structs.foreach(structDefI => {
      if (hinputs.kindExterns.contains(structDefI.instantiatedCitizen)) {
        translateOpaqueI(hinputs, hamuts, structDefI.instantiatedCitizen)
      } else {
        translateStructI(hinputs, hamuts, structDefI.instantiatedCitizen)
      }
    })
  }
*/

// mig: fn translate_struct_i
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_struct_i(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_it: &'i StructIT<'s, 'i, cI>,
    ) -> &'h StructHT<'s, 'h>
    {
        panic!("Unimplemented: translate_struct_i");
    }
}
/*
  def translateStructI(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      structIT: StructIT[cI]):
  (StructHT) = {
    hamuts.structTToStructH.get(structIT) match {
      case Some(structRefH) => structRefH
      case None => {
        val (fullNameH) = nameHammer.translateFullName(hinputs, hamuts, structIT.id)
        // This is the only place besides StructDefinitionH that can make a StructRefH
        val temporaryStructRefH = StructHT(fullNameH);
        hamuts.forwardDeclareStruct(structIT, temporaryStructRefH)
        val structDefI = hinputs.lookupStruct(structIT.id);
        val mutabilityH = Conversions.evaluateMutabilityTemplata(structDefI.mutability)
        val (membersH) =
          translateMembers(hinputs, hamuts, structDefI.instantiatedCitizen.id, mutabilityH, structDefI.members)

        val (edgesH) = translateEdgesForStruct(hinputs, hamuts, temporaryStructRefH, structIT)

        val structDefH =
          StructDefinitionH(
            fullNameH,
            structDefI.weakable,
            structDefI.attributes.exists({ case ExternI(_) => true case _ => false }),
            mutabilityH,
            edgesH,
            membersH);
        hamuts.addStructOriginatingFromTypingPass(structIT, structDefH)
        vassert(structDefH.getRef == temporaryStructRefH)

        // Make sure there's a destructor for this shared struct.
        structDefI.mutability match {
          case MutableI => None
          case ImmutableI => {
//            vassert(
//              hinputs.functions.exists(function => {
//                function.header.fullName match {
//                  case FullNameI(_, _, FreeNameI(_, _, k)) if k.kind == structDefI.instantiatedCitizen => true
//                  case _ => false
//                }
//              }))
          }
        }


        (structDefH.getRef)
      }
    }
  }
*/

// mig: fn translate_opaque_i
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_opaque_i(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_it: &'i StructIT<'s, 'i, cI>,
    ) -> &'h OpaqueHT<'s, 'h>
    {
        panic!("Unimplemented: translate_opaque_i");
    }
}
/*
  def translateOpaqueI(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      structIT: StructIT[cI]):
  (OpaqueHT) = {
    hamuts.structTToOpaqueH.get(structIT) match {
      case Some(opaqueH) => opaqueH
      case None => {
        val (fullNameH) = nameHammer.translateFullName(hinputs, hamuts, structIT.id)
        // This is the only place besides StructDefinitionH that can make a StructRefH
        val temporaryStructRefH = StructHT(fullNameH);
        hamuts.forwardDeclareStruct(structIT, temporaryStructRefH)
        val structDefI = hinputs.lookupStruct(structIT.id);
        val mutabilityH = Conversions.evaluateMutabilityTemplata(structDefI.mutability)
        vassert(structDefI.members.isEmpty)

        val (edgesH) = translateEdgesForStruct(hinputs, hamuts, temporaryStructRefH, structIT)

//        hamuts.addStructOriginatingFromTypingPass(structIT, structDefH)
//        vassert(structDefH.getRef == temporaryStructRefH)

        val opaqueH =
          OpaqueHT(
            structIT.id.packageCoord,
            nameHammer.translateFullName(hinputs, hamuts, structIT.id),
            NameHammer.simplifyId(structIT.id))

        hamuts.addOpaque(structIT, opaqueH)

        opaqueH
      }
    }
  }
*/

// mig: fn translate_members
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_members(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_name: &IdI<'s, 'i, cI>,
        struct_mutability_h: Mutability,
        members: &[StructMemberI<'s, 'i, cI>],
    ) -> Vec<StructMemberH<'s, 'h>>
    {
        panic!("Unimplemented: translate_members");
    }
}
/*
  def translateMembers(hinputs: HinputsI, hamuts: HamutsBox, structName: IdI[cI, INameI[cI]], structMutabilityH: Mutability, members: Vector[StructMemberI]):
  (Vector[StructMemberH]) = {
    members.map(translateMember(hinputs, hamuts, structName, structMutabilityH, _))
  }
*/

// mig: fn translate_member
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_member(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_name: &IdI<'s, 'i, cI>,
        struct_mutability_h: Mutability,
        member2: &StructMemberI<'s, 'i, cI>,
    ) -> StructMemberH<'s, 'h>
    {
        panic!("Unimplemented: translate_member");
    }
}
/*
  def translateMember(hinputs: HinputsI, hamuts: HamutsBox, structName: IdI[cI, INameI[cI]], structMutabilityH: Mutability, member2: StructMemberI):
  (StructMemberH) = {
    val (variability, memberType) =
      member2 match {
//        case StructMemberI(_, variability, OpaqueMemberTypeI()) => {
//          val opaqueHT =
//            OpaqueHT(
//              nameHammer.translateFullName(hinputs, hamuts, structName),
//              NameHammer.simplifyId(structName))
//          (variability, CoordH(vregionmut(MutableShareH), YonderH, opaqueHT))
//        }
        case StructMemberI(_, variability, ReferenceMemberTypeI(coord)) => {
          (variability, translateReference(hinputs, hamuts, coord))
        }
        case StructMemberI(_, variability, AddressMemberTypeI(coord)) => {
          val (referenceH) =
            translateReference(hinputs, hamuts, coord)
          val (boxStructRefH) =
            makeBox(hinputs, hamuts, variability, coord, referenceH)
          // The stack owns the box, closure structs just borrow it.
          (variability, CoordH(vregionmut(MutableBorrowH), YonderH, boxStructRefH))
        }
      }
    StructMemberH(
      nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(structName, member2.name)),
      Conversions.evaluateVariability(variability),
      memberType)
  }
*/

// mig: fn make_box
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn make_box(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        conceptual_variability: VariabilityI,
        type2: CoordI<'s, 'i, cI>,
        type_h: CoordH<'s, 'h>,
    ) -> &'h StructHT<'s, 'h>
    {
        panic!("Unimplemented: make_box");
    }
}
/*
  def makeBox(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    conceptualVariability: VariabilityI,
    type2: CoordI[cI],
    typeH: CoordH[KindHT]):
  (StructHT) = {
    val boxFullName2 =
      IdI(
        PackageCoordinate.BUILTIN(interner, keywords),
        Vector[INameI[cI]](),
        StructNameI[cI](
          StructTemplateNameI[cI](keywords.BOX_HUMAN_NAME),
          Vector(CoordTemplataI[cI](RegionTemplataI(0), type2))))
    val boxFullNameH = nameHammer.translateFullName(hinputs, hamuts, boxFullName2)
    hamuts.structDefs.find(_.id == boxFullNameH) match {
      case Some(structDefH) => (structDefH.getRef)
      case None => {
        val temporaryStructRefH = StructHT(boxFullNameH);

        // We don't actually care about the given variability, because even if it's final, we still need
        // the box to contain a varying reference, see VCBAAF.
        val _ = conceptualVariability
        val actualVariability = VaryingI

        val memberH =
          StructMemberH(
            nameHammer.addStep(hamuts, temporaryStructRefH.id, keywords.BOX_MEMBER_NAME.str),
            Conversions.evaluateVariability(actualVariability), typeH)

        val structDefH =
          StructDefinitionH(
            boxFullNameH,
            false,
            false,
            Mutable,
            Vector.empty,
            Vector(memberH));
        hamuts.addStructOriginatingFromHammer(structDefH)
        vassert(structDefH.getRef == temporaryStructRefH)
        (structDefH.getRef)
      }
    }
  }
*/

// mig: fn translate_edges_for_struct (Scala overload — disambiguated.)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_edges_for_struct(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_ref_h: &'h StructHT<'s, 'h>,
        struct_tt: &'i StructIT<'s, 'i, cI>,
    ) -> Vec<EdgeH<'s, 'h>>
    {
        panic!("Unimplemented: translate_edges_for_struct");
    }
}
/*
  private def translateEdgesForStruct(
      hinputs: HinputsI, hamuts: HamutsBox,
      structRefH: StructHT,
      structTT: StructIT[cI]):
  (Vector[EdgeH]) = {
    val edges2 = hinputs.interfaceToSubCitizenToEdge.values.flatMap(_.values).filter(_.subCitizen.id == structTT.id)
    translateEdgesForStruct(hinputs, hamuts, structRefH, edges2.toVector)
  }
*/

// mig: fn translate_edges_for_struct_with_edges (Scala overload — disambiguated.)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_edges_for_struct_with_edges(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_ref_h: &'h StructHT<'s, 'h>,
        edges2: &[EdgeI<'s, 'i>],
    ) -> Vec<EdgeH<'s, 'h>>
    {
        panic!("Unimplemented: translate_edges_for_struct_with_edges");
    }
}
/*
  private def translateEdgesForStruct(
      hinputs: HinputsI, hamuts: HamutsBox,
      structRefH: StructHT,
      edges2: Vector[EdgeI]):
  (Vector[EdgeH]) = {
    edges2.map(e => translateEdge(hinputs, hamuts, structRefH, InterfaceIT(e.superInterface), e))
  }
*/

// mig: fn translate_edge
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_edge(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        struct_ref_h: &'h StructHT<'s, 'h>,
        interface_it: &'i InterfaceIT<'s, 'i, cI>,
        edge2: &EdgeI<'s, 'i>,
    ) -> EdgeH<'s, 'h>
    {
        panic!("Unimplemented: translate_edge");
    }
}
/*
  private def translateEdge(hinputs: HinputsI, hamuts: HamutsBox, structRefH: StructHT, interfaceIT: InterfaceIT[cI], edge2: EdgeI):
  (EdgeH) = {
    // Purposefully not trying to translate the entire struct here, because we might hit a circular dependency
    val interfaceRefH = translateInterface(hinputs, hamuts, interfaceIT)
    val interfacePrototypesH = translateInterfaceMethods(hinputs, hamuts, interfaceIT)

    val prototypesH =
      vassertSome(hinputs.interfaceToEdgeBlueprints.get(interfaceIT.id))
        .superFamilyRootHeaders.map({
        case (superFamilyPrototype, virtualParamIndex) =>
          val overridePrototypeI =
            vassertSome(edge2.abstractFuncToOverrideFunc.get(superFamilyPrototype.id))
          val overridePrototypeH = translatePrototype(hinputs, hamuts, overridePrototypeI)
          overridePrototypeH
      })

    val structPrototypesByInterfacePrototype = ListMap[InterfaceMethodH, PrototypeH](interfacePrototypesH.zip(prototypesH) : _*)
    (EdgeH(structRefH, interfaceRefH, structPrototypesByInterfacePrototype))
  }
*/

// mig: fn lookup_struct
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn lookup_struct(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &Hamuts<'s, 'i, 'h>,
        struct_tt: &'i StructIT<'s, 'i, cI>,
    ) -> &'i StructDefinitionI<'s, 'i, cI>
    {
        panic!("Unimplemented: lookup_struct");
    }
}
/*
  def lookupStruct(hinputs: HinputsI, hamuts: HamutsBox, structTT: StructIT[cI]): StructDefinitionI = {
    hinputs.lookupStruct(structTT.id)
  }
*/

// mig: fn lookup_interface
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn lookup_interface(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &Hamuts<'s, 'i, 'h>,
        interface_tt: &'i InterfaceIT<'s, 'i, cI>,
    ) -> &'i InterfaceDefinitionI<'s, 'i, cI>
    {
        panic!("Unimplemented: lookup_interface");
    }
}
/*
  def lookupInterface(hinputs: HinputsI, hamuts: HamutsBox, interfaceTT: InterfaceIT[cI]): InterfaceDefinitionI = {
    hinputs.lookupInterface(interfaceTT.id)
  }
}
*/
