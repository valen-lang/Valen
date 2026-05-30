// From Frontend/SimplifyingPass/src/dev/vale/simplifying/TypeHammer.scala
//
// Per typing-pass `Compiler` precedent, `TypeHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::ast::{PrototypeH, PrototypeHValH, RegionH};
use crate::final_ast::types::{CoordH, KindHT, LocationH, RuntimeSizedArrayHT, StaticSizedArrayHT};
use crate::instantiating::ast::types::OwnershipI;
use crate::simplifying::conversions::evaluate_ownership;
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::templata::RegionTemplataI;
use crate::instantiating::ast::types::{cI, CoordI, IntIT, KindIT, NeverIT, RuntimeSizedArrayIT, StaticSizedArrayIT};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;

/*
package dev.vale.simplifying

import dev.vale.finalast.{BoolHT, CoordH, FloatHT, InlineH, IntHT, KindHT, NeverHT, PrototypeH, RuntimeSizedArrayDefinitionHT, RuntimeSizedArrayHT, StaticSizedArrayDefinitionHT, StaticSizedArrayHT, StrHT, VoidHT, YonderH}
import dev.vale.{Interner, Keywords, PackageCoordinate, StrI, vassert, vfail, vimpl, vregionmut, vwat, finalast => m}
import dev.vale.finalast._
import dev.vale.instantiating.ast._

class TypeHammer(
    interner: Interner,
    keywords: Keywords,
    nameHammer: NameHammer,
    structHammer: StructHammer) {
*/

// mig: fn translate_kind
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_kind(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        tyype: KindIT<'s, 'i, cI>,
    ) -> KindHT<'s, 'h>
    {
        match tyype {
            KindIT::NeverIT(NeverIT { from_break, .. }) => KindHT::NeverHT(crate::final_ast::types::NeverHT { from_break }),
            KindIT::IntIT(IntIT { bits, .. }) => KindHT::IntHT(crate::final_ast::types::IntHT { bits }),
            KindIT::BoolIT(_) => KindHT::BoolHT(crate::final_ast::types::BoolHT),
            KindIT::FloatIT(_) => KindHT::FloatHT(crate::final_ast::types::FloatHT),
            KindIT::StrIT(_) => KindHT::StrHT(crate::final_ast::types::StrHT),
            KindIT::VoidIT(_) => KindHT::VoidHT(crate::final_ast::types::VoidHT),
            KindIT::StructIT(s) => {
                if hinputs.kind_externs.contains_key(s) {
                    panic!("translate_kind: StructIT kindExterns translateOpaqueI branch")
                } else {
                    panic!("translate_kind: StructIT translateStructI branch")
                }
            }
            KindIT::InterfaceIT(_) => panic!("translate_kind: InterfaceIT translateInterface branch"),
            KindIT::StaticSizedArrayIT(_) => panic!("translate_kind: StaticSizedArrayIT branch"),
            KindIT::RuntimeSizedArrayIT(_) => panic!("translate_kind: RuntimeSizedArrayIT branch"),
        }
    }
}
/*
  def translateKind(hinputs: HinputsI, hamuts: HamutsBox, tyype: KindIT[cI]):
  (KindHT) = {
    tyype match {
      case NeverIT(fromBreak) => NeverHT(fromBreak)
      case IntIT(bits) => IntHT(bits)
      case BoolIT() => BoolHT()
      case FloatIT() => FloatHT()
      case StrIT() => StrHT()
      case VoidIT() => VoidHT()
      case s @ StructIT(id) => {
        if (hinputs.kindExterns.contains(s)) {
          structHammer.translateOpaqueI(hinputs, hamuts, s)
        } else {
          structHammer.translateStructI(hinputs, hamuts, s)
        }
      }

      case i @ InterfaceIT(_) => structHammer.translateInterface(hinputs, hamuts, i)

//      case OverloadSetI(_, _) => VoidHT()

      case a @ contentsStaticSizedArrayIT(_, _, _, _, _) => translateStaticSizedArray(hinputs, hamuts, a)
      case a @ contentsRuntimeSizedArrayIT(_, _, _) => translateRuntimeSizedArray(hinputs, hamuts, a)
//      case KindPlaceholderI(fullName) => {
//        // this is a bit of a hack. sometimes lambda templates like to remember their original
//        // defining generics, and we dont translate those in the instantiator, so it can later
//        // use them to find those original templates.
//        // because of that, they make their way into the hammer, right here.
//        // long term, we should probably find a way to tostring templatas cleanly rather than
//        // converting them to hammer first.
//        // See DMPOGN for why these make it into the hammer.
//        VoidHT()
//      }
    }
  }
*/

// mig: fn translate_region
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_region(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        region: &RegionTemplataI<'s, 'i, cI>,
    ) -> RegionH
    {
        panic!("Unimplemented: translate_region");
    }
}
/*
  def translateRegion(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    region: RegionTemplataI[cI]):
  RegionH = {
    RegionH()
  }
*/

// mig: fn translate_coord
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_coord(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        coord: CoordI<'s, 'i, cI>,
    ) -> CoordH<'s, 'h>
    {
        let CoordI { ownership, kind: inner_type } = coord;
        let location = match (ownership, inner_type) {
            (OwnershipI::Own, _) => LocationH::YonderH,
            (OwnershipI::ImmutableBorrow | OwnershipI::MutableBorrow, _) => LocationH::YonderH,
            (OwnershipI::Weak, _) => LocationH::YonderH,
            (_, KindIT::StructIT(s)) if hinputs.kind_externs.contains_key(&s) => {
                panic!("translate_coord: kindExterns InlineH branch")
            }
            (OwnershipI::ImmutableShare | OwnershipI::MutableShare, KindIT::VoidIT(_) | KindIT::IntIT(_) | KindIT::BoolIT(_) | KindIT::FloatIT(_) | KindIT::NeverIT(_)) => LocationH::InlineH,
            (OwnershipI::ImmutableShare | OwnershipI::MutableShare, KindIT::StrIT(_)) => LocationH::YonderH,
            (OwnershipI::ImmutableShare | OwnershipI::MutableShare, _) => LocationH::YonderH,
        };
        let inner_h = self.translate_kind(hinputs, hamuts, inner_type);
        CoordH { ownership: evaluate_ownership(ownership), location, kind: inner_h }
    }
}
/*
  def translateCoord(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      coord: CoordI[cI]):
  (CoordH[KindHT]) = {
    val CoordI(ownership, innerType) = coord;
    val location = {
      (ownership, innerType) match {
        case (OwnI, _) => YonderH
        case (ImmutableBorrowI | MutableBorrowI, _) => YonderH
        case (WeakI, _) => YonderH
        case (_, kind @ StructIT(_)) if hinputs.kindExterns.contains(kind) => {
          InlineH
        }
        case (ImmutableShareI | MutableShareI, VoidIT() | IntIT(_) | BoolIT() | FloatIT() | NeverIT(_)) => InlineH
        case (ImmutableShareI | MutableShareI, StrIT()) => YonderH
        case (ImmutableShareI | MutableShareI, _) => YonderH
      }
    }
    val (innerH) = translateKind(hinputs, hamuts, innerType);
    (CoordH(Conversions.evaluateOwnership(ownership), location, innerH))
  }
*/

// mig: fn translate_coords
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_coords(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        references2: &[CoordI<'s, 'i, cI>],
    ) -> Vec<CoordH<'s, 'h>>
    {
        references2.iter().map(|c| self.translate_coord(hinputs, hamuts, *c)).collect()
    }
}
/*
  def translateCoords(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      references2: Vector[CoordI[cI]]):
  (Vector[CoordH[KindHT]]) = {
    references2.map(translateCoord(hinputs, hamuts, _))
  }
*/

// mig: fn check_conversion
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn check_conversion(
        &self,
        expected: CoordH<'s, 'h>,
        actual: CoordH<'s, 'h>,
    ) {
        panic!("Unimplemented: check_conversion");
    }
}
/*
  def checkConversion(expected: CoordH[KindHT], actual: CoordH[KindHT]): Unit = {
    if (actual != expected) {
      vfail("Expected a " + expected + " but was a " + actual);
    }
  }
*/

// mig: fn translate_static_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_static_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        ssa_it: &'i StaticSizedArrayIT<'s, 'i, cI>,
    ) -> &'h StaticSizedArrayHT<'s, 'h>
    {
        panic!("Unimplemented: translate_static_sized_array");
    }
}
/*
  def translateStaticSizedArray(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      ssaIT: StaticSizedArrayIT[cI]):
  StaticSizedArrayHT = {
    hamuts.staticSizedArrays.get(ssaIT) match {
      case Some(x) => x.kind
      case None => {
        val name = nameHammer.translateFullName(hinputs, hamuts, ssaIT.name)
        val contentsStaticSizedArrayIT(_, mutabilityI, variabilityI, memberType, arrRegion) = ssaIT
        vregionmut(arrRegion) // what do with arrRegion?
        val memberReferenceH = translateCoord(hinputs, hamuts, memberType.coord)
        val mutability = Conversions.evaluateMutabilityTemplata(mutabilityI)
        val variability = Conversions.evaluateVariabilityTemplata(variabilityI)
        val size = ssaIT.size
        val definition = StaticSizedArrayDefinitionHT(name, size, mutability, variability, memberReferenceH)
        val result = StaticSizedArrayHT(name)
        hamuts.staticSizedArrays.find(_._2.kind == result) match {
          case Some(x) => vwat(x)
          case None => hamuts.addStaticSizedArray(ssaIT, definition)
        }
        result
      }
    }
  }
*/

// mig: fn translate_runtime_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_runtime_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        rsa_it: &'i RuntimeSizedArrayIT<'s, 'i, cI>,
    ) -> &'h RuntimeSizedArrayHT<'s, 'h>
    {
        panic!("Unimplemented: translate_runtime_sized_array");
    }
}
/*
  def translateRuntimeSizedArray(hinputs: HinputsI, hamuts: HamutsBox, rsaIT: RuntimeSizedArrayIT[cI]): RuntimeSizedArrayHT = {
    hamuts.runtimeSizedArrays.get(rsaIT) match {
      case Some(x) => x.kind
      case None => {
        val nameH = nameHammer.translateFullName(hinputs, hamuts, rsaIT.name)
        val contentsRuntimeSizedArrayIT(mutabilityI, memberType, arrRegion) = rsaIT
        vregionmut(arrRegion) // what do with arrRegion?
        val memberReferenceH = translateCoord(hinputs, hamuts, memberType.coord)
        val mutability = Conversions.evaluateMutabilityTemplata(mutabilityI)
        //    val variability = Conversions.evaluateVariability(variabilityI)
        val definition = RuntimeSizedArrayDefinitionHT(nameH, mutability, memberReferenceH)
        val result = RuntimeSizedArrayHT(nameH)
        hamuts.runtimeSizedArrays.values.find(_.kind == result) match {
          case Some(x) => vwat(x)
          case None => hamuts.addRuntimeSizedArray(rsaIT, definition)
        }
        result
      }
    }
  }
*/

// mig: fn translate_prototype
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_prototype(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        prototype2: &'i PrototypeI<'s, 'i, cI>,
    ) -> &'h PrototypeH<'s, 'h>
    {
        let PrototypeI { id: full_name2, return_type: return_type2, _must_intern: _ } = prototype2;
        let params_types_h = self.translate_coords(hinputs, hamuts, &prototype2.param_types());
        let return_type_h = self.translate_coord(hinputs, hamuts, *return_type2);
        let full_name_h = self.translate_full_name(hinputs, hamuts, full_name2);
        let prototype_h = self.interner.intern_prototype(PrototypeHValH {
            id: full_name_h,
            params: self.interner.alloc_slice_from_vec(params_types_h),
            return_type: return_type_h,
        });
        prototype_h
    }
}
/*
  def translatePrototype(
    hinputs: HinputsI, hamuts: HamutsBox,
    prototype2: PrototypeI[cI]):
  (PrototypeH) = {
    val PrototypeI(fullName2, returnType2) = prototype2;
    val (paramsTypesH) = translateCoords(hinputs, hamuts, prototype2.paramTypes)
    val (returnTypeH) = translateCoord(hinputs, hamuts, returnType2)
    val (fullNameH) = nameHammer.translateFullName(hinputs, hamuts, fullName2)
    val prototypeH = PrototypeH(fullNameH, paramsTypesH, returnTypeH)
    (prototypeH)
  }

}
*/
