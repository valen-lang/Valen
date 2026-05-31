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
        let edge_blueprint = hinputs.interface_to_edge_blueprints.get(&interface_tt.id).expect("vassertSome: interface_to_edge_blueprints");
        edge_blueprint.super_family_root_headers.iter().map(|(super_family_prototype, virtual_param_index)| {
            let prototype_h = self.translate_prototype(hinputs, hamuts, super_family_prototype);
            crate::final_ast::ast::InterfaceMethodH { prototype_h, virtual_param_index: *virtual_param_index }
        }).collect()
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
        match hamuts.interface_t_to_interface_h.get(&interface_it).copied() {
            Some(struct_ref_h) => struct_ref_h,
            None => {
                let full_name_h = self.translate_full_name(hinputs, hamuts, &interface_it.id);
                let temporary_interface_ref_h = self.interner.intern_interface_ht(crate::final_ast::types::InterfaceHTValH { id: full_name_h });
                hamuts.forward_declare_interface(interface_it, temporary_interface_ref_h);
                let interface_def_i = hinputs.lookup_interface(&interface_it.id);
                let methods_h = self.translate_interface_methods(hinputs, hamuts, interface_it);
                let interface_def_h = crate::final_ast::ast::InterfaceDefinitionH {
                    id: full_name_h,
                    weakable: interface_def_i.weakable,
                    mutability: crate::simplifying::conversions::evaluate_mutability_templata(interface_def_i.mutability),
                    super_interfaces: &[],
                    methods: self.interner.alloc_slice_from_vec(methods_h),
                };
                hamuts.add_interface(interface_it, interface_def_h);
                assert!(std::ptr::eq(interface_def_h.get_ref(self.interner) as *const _, temporary_interface_ref_h as *const _));
                match interface_def_i.mutability {
                    crate::instantiating::ast::types::MutabilityI::Mutable => {}
                    crate::instantiating::ast::types::MutabilityI::Immutable => {}
                }
                interface_def_h.get_ref(self.interner)
            }
        }
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
            if hinputs.kind_externs.contains_key(&struct_def_i.instantiated_citizen) {
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
        match hamuts.struct_t_to_struct_h.get(&struct_it).copied() {
            Some(struct_ref_h) => struct_ref_h,
            None => {
                let full_name_h = self.translate_full_name(hinputs, hamuts, &struct_it.id);
                let temporary_struct_ref_h = self.interner.intern_struct_ht(crate::final_ast::types::StructHTValH { id: full_name_h });
                hamuts.forward_declare_struct(struct_it, temporary_struct_ref_h);
                let struct_def_i = hinputs.lookup_struct(&struct_it.id);
                let mutability_h = crate::simplifying::conversions::evaluate_mutability_templata(struct_def_i.mutability);
                let members_h = self.translate_members(hinputs, hamuts, &struct_def_i.instantiated_citizen.id, mutability_h, struct_def_i.members);
                let edges_h_vec = self.translate_edges_for_struct(hinputs, hamuts, temporary_struct_ref_h, struct_it);
                let edges_h: &'h [crate::final_ast::ast::EdgeH<'s, 'h>] = self.interner.alloc_slice_from_vec(edges_h_vec);
                let extern_ = struct_def_i.attributes.iter().any(|a| matches!(a, crate::instantiating::ast::ast::ICitizenAttributeI::ExternI(_)));
                let struct_def_h = crate::final_ast::ast::StructDefinitionH {
                    id: full_name_h,
                    weakable: struct_def_i.weakable,
                    extern_,
                    mutability: mutability_h,
                    edges: edges_h,
                    members: self.interner.alloc_slice_from_vec(members_h),
                };
                hamuts.add_struct_originating_from_typing_pass(struct_it, struct_def_h);
                assert!(std::ptr::eq(struct_def_h.get_ref(self.interner) as *const _, temporary_struct_ref_h as *const _));
                match struct_def_i.mutability {
                    crate::instantiating::ast::types::MutabilityI::Mutable => {}
                    crate::instantiating::ast::types::MutabilityI::Immutable => {}
                }
                struct_def_h.get_ref(self.interner)
            }
        }
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
        match hamuts.struct_t_to_opaque_h.get(&struct_it).copied() {
            Some(opaque_h) => opaque_h,
            None => {
                let full_name_h = self.translate_full_name(hinputs, hamuts, &struct_it.id);
                let temporary_struct_ref_h = self.interner.intern_struct_ht(crate::final_ast::types::StructHTValH { id: full_name_h });
                hamuts.forward_declare_struct(struct_it, temporary_struct_ref_h);
                let struct_def_i = hinputs.lookup_struct(&struct_it.id);
                let _mutability_h = crate::simplifying::conversions::evaluate_mutability_templata(struct_def_i.mutability);
                assert!(struct_def_i.members.is_empty());

                let _edges_h = self.translate_edges_for_struct(hinputs, hamuts, temporary_struct_ref_h, struct_it);

                let opaque_h: &'h OpaqueHT<'s, 'h> = self.interner.bump().alloc(crate::final_ast::types::OpaqueHT {
                    package_coord: *struct_it.id.package_coord,
                    struct_id: self.translate_full_name(hinputs, hamuts, &struct_it.id),
                    simple_id: crate::simplifying::name_hammer::simplify_id(self.interner, self.scout_arena, &struct_it.id),
                });

                hamuts.add_opaque(struct_it, opaque_h);

                opaque_h
            }
        }
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
        members.iter().map(|m| self.translate_member(hinputs, hamuts, struct_name, struct_mutability_h, m)).collect()
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
        let (variability, member_type) = match member2.tyype {
            crate::instantiating::ast::citizens::IMemberTypeI::ReferenceMemberTypeI(r) => {
                (member2.variability, self.translate_coord(hinputs, hamuts, r.reference))
            }
            crate::instantiating::ast::citizens::IMemberTypeI::AddressMemberTypeI(_) => panic!("Unimplemented: translate_member AddressMemberTypeI"),
        };
        let added_name = crate::instantiating::ast::names::add_step(struct_name, member2.name.into());
        StructMemberH {
            name: self.translate_full_name(hinputs, hamuts, &added_name),
            variability: crate::simplifying::conversions::evaluate_variability(variability),
            tyype: member_type,
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Scala's `translateReference` is a closure parameter of StructHammer wired in Hammer.scala line 174 to `typeHammer.translateCoord(...)`. Under the SPDMX Exception Q god-struct collapse, the closure indirection vanishes and the call lands on translate_coord — exactly what Scala dispatches to. Same precedent as the existing temp-disable on `translate` in hammer.rs:725. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2833-1780146824579/hook-2833/translate_member--382.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
        let edges2: Vec<&EdgeI<'s, 'i>> = hinputs.interface_to_sub_citizen_to_edge.iter()
            .flat_map(|(_, sub_map)| sub_map.iter().map(|(_, e)| e))
            .filter(|e| e.sub_citizen.id() == struct_tt.id)
            .collect();
        self.translate_edges_for_struct_with_edges(hinputs, hamuts, struct_ref_h, &edges2)
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
        edges2: &[&EdgeI<'s, 'i>],
    ) -> Vec<EdgeH<'s, 'h>>
    {
        edges2.iter().map(|e| self.translate_edge(hinputs, hamuts, struct_ref_h, self.instantiating_interner.intern_interface_it_ci(crate::instantiating::ast::types::InterfaceITValI { id: e.super_interface }), e)).collect()
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
        // Purposefully not trying to translate the entire struct here, because we might hit a circular dependency
        let interface_ref_h = self.translate_interface(hinputs, hamuts, interface_it);
        let interface_prototypes_h = self.translate_interface_methods(hinputs, hamuts, interface_it);

        let prototypes_h: Vec<&'h crate::final_ast::ast::PrototypeH<'s, 'h>> = hinputs.interface_to_edge_blueprints.get(&interface_it.id).expect("vassertSome: interface_to_edge_blueprints")
            .super_family_root_headers.iter().map(|(super_family_prototype, _virtual_param_index)| {
                let override_prototype_i = *edge2.abstract_func_to_override_func.get(&super_family_prototype.id).expect("vassertSome: abstract_func_to_override_func");
                self.translate_prototype(hinputs, hamuts, override_prototype_i)
            }).collect();

        let struct_prototypes_by_interface_method: Vec<(InterfaceMethodH<'s, 'h>, &'h crate::final_ast::ast::PrototypeH<'s, 'h>)> = interface_prototypes_h.iter().zip(prototypes_h.iter()).map(|(im, p)| (*im, *p)).collect();
        EdgeH {
            struct_: struct_ref_h,
            interface: interface_ref_h,
            struct_prototypes_by_interface_method: self.interner.bump().alloc(crate::utils::arena_index_map::ArenaIndexMap::from_iter_in(struct_prototypes_by_interface_method.into_iter(), self.interner.bump())),
        }
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
