// From Frontend/SimplifyingPass/src/dev/vale/simplifying/TypeHammer.scala
/*
package dev.vale.simplifying

import dev.vale.finalast.{BoolHT, CoordH, FloatHT, InlineH, IntHT, KindHT, NeverHT, PrototypeH, RuntimeSizedArrayDefinitionHT, RuntimeSizedArrayHT, StaticSizedArrayDefinitionHT, StaticSizedArrayHT, StrHT, VoidHT, YonderH}
import dev.vale.{Interner, Keywords, vassert, vfail, vimpl, vregionmut, vwat, finalast => m}
import dev.vale.finalast._
import dev.vale.instantiating.ast._
*/
// mig: struct TypeHammerH
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct TypeHammerH<'h> {
    pub interner: usize, // placeholder
    pub keywords: usize, // placeholder
    pub name_hammer: usize, // placeholder
    pub struct_hammer: usize, // placeholder
}
// mig: impl TypeHammerH
/*
class TypeHammer(
    interner: Interner,
    keywords: Keywords,
    nameHammer: NameHammer,
    structHammer: StructHammer) {
*/
// mig: fn translate_kind
impl<'h> TypeHammerH<'h> {
    pub fn translate_kind(&self, hinputs: usize, hamuts: usize, tyype: usize) -> usize {
        panic!("Unimplemented: translate_kind");
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
      case s @ StructIT(_) => structHammer.translateStructI(hinputs, hamuts, s)

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
impl<'h> TypeHammerH<'h> {
    pub fn translate_region(&self, hinputs: usize, hamuts: usize, region: usize) -> usize {
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
impl<'h> TypeHammerH<'h> {
    pub fn translate_coord(&self, hinputs: usize, hamuts: usize, coord: usize) -> usize {
        panic!("Unimplemented: translate_coord");
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
//        case (ImmutableShareI | MutableShareI, OverloadSetI(_, _)) => InlineH
//        case (ShareI, PackIT(_, _)) => InlineH
//        case (ShareI, TupleIT(_, _)) => InlineH
//        case (ShareI, StructIT(FullNameI(_, Vector(), CitizenNameI(CitizenTemplateNameI("Tup"), _)))) => InlineH
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
impl<'h> TypeHammerH<'h> {
    pub fn translate_coords(&self, hinputs: usize, hamuts: usize, references2: &'h [usize]) -> &'h [usize] {
        panic!("Unimplemented: translate_coords");
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
impl<'h> TypeHammerH<'h> {
    pub fn check_conversion(&self, expected: usize, actual: usize) -> () {
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
impl<'h> TypeHammerH<'h> {
    pub fn translate_static_sized_array(&self, hinputs: usize, hamuts: usize, ssaIT: usize) -> usize {
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
impl<'h> TypeHammerH<'h> {
    pub fn translate_runtime_sized_array(&self, hinputs: usize, hamuts: usize, rsaIT: usize) -> usize {
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
impl<'h> TypeHammerH<'h> {
    pub fn translate_prototype(&self, hinputs: usize, hamuts: usize, prototype2: usize) -> usize {
        panic!("Unimplemented: translate_prototype");
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
