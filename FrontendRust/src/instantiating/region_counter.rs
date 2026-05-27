/*
package dev.vale.instantiating

import dev.vale.instantiating.RegionCounter._
import dev.vale.instantiating.ast._
import dev.vale.{U, vassert, vimpl, vwat}

import scala.collection.mutable

object RegionCounter {
*/
// mig: struct CounterI
pub struct CounterI;
// TODO: populate fields when names.rs / templata.rs are fully migrated.
// mig: impl CounterI
/*
  class Counter {
    // TODO(optimize): Use an array for this, with a minimum index and maximum index (similar to
    // what a circular queue uses)
    val set: mutable.HashSet[Int] = mutable.HashSet[Int]()

*/
// mig: fn count
impl CounterI {
    pub fn count(&mut self) { panic!("Unimplemented: count"); }
}
/*
    def count(region: RegionTemplataI[sI]): Unit = {
      set.add(region.pureHeight)
    }

*/
// mig: fn assemble_map
impl CounterI {
    pub fn assemble_map(&self) -> std::collections::HashMap<i32, i32> {
        panic!("Unimplemented: assemble_map");
    }
}
/*
    def assembleMap(): Map[Int, Int] = {
      val numRegions = set.size
      // Let's say we have a set that contains 3, 5, -2, 0, 4, it becomes...
      set.toVector
        .sorted // -2, 0, 3, 4, 5
        .zipWithIndex // (-2, 0), (0, 1), (3, 2), (4, 3), (5, 4)
        .map({ case (subjectiveRegion, i) =>
          // If we have 4 regions, then they should go from -3 to 0
          subjectiveRegion -> (i - numRegions + 1)
        }) // (-2, -4), (0, -3), (3, -2), (4, -1), (5, 0)
        .toMap
    }

//    def assembleMap(counter: Counter): Vector[Int] = {
//      var numRegions = 0
//      U.foreach(counts.toVector, (hasRegion: Boolean) => {
//        if (hasRegion) {
//          numRegions = numRegions + 1
//        }
//      })
//      // If we have 4 regions, then they should go from -3 to 0
//      var nextRegion = -numRegions + 1
//      U.mapWithIndex(counts.toVector, (i, hasRegion: Boolean) => {
//        if (hasRegion) {
//          val region = nextRegion
//          nextRegion = nextRegion + 1
//          region
//        } else {
//          Int.MaxValue
//        }
//      })
//    }

    //    private def count(counter: Counter, region: RegionTemplataI[sI]): Unit = {
//      while (region.pureHeight >= counts.size) {
//        counts += false
//      }
//      counts(region.pureHeight) = true
//    }

  }

*/
// mig: fn count_prototype
pub fn count_prototype() {
    panic!("Unimplemented: count_prototype");
}
/*
  def countPrototype(counter: Counter, prototype: PrototypeI[sI]): Unit = {
    val PrototypeI(id, returnType) = prototype
    countFunctionId(counter, id)
    countCoord(counter, returnType)
  }

*/
// mig: fn count_id
pub fn count_id() {
    panic!("Unimplemented: count_id");
}
/*
  def countId[T <: INameI[sI]](
    counter: Counter,
    id: IdI[sI, T],
    func: T => Unit):
  Unit = {
    val IdI(packageCoord, initSteps, localName) = id
    initSteps.foreach(x => countName(counter, x))
    func(localName)
  }

*/
// mig: fn count_function_id
pub fn count_function_id() {
    panic!("Unimplemented: count_function_id");
}
/*
  def countFunctionId(
    counter: Counter,
    id: IdI[sI, IFunctionNameI[sI]]):
  Unit = {
    countId[IFunctionNameI[sI]](
      counter: Counter,
      id,
      x => countFunctionName(counter, x))
  }

*/
// mig: fn count_function_name
pub fn count_function_name() {
    panic!("Unimplemented: count_function_name");
}
/*
  def countFunctionName(
    counter: Counter,
    name: IFunctionNameI[sI]):
  Unit = {
    name match {
      case FunctionNameIX(FunctionTemplateNameI(humanName, codeLocation), templateArgs, parameters) => {
        templateArgs.foreach(countTemplata(counter, _))
        parameters.foreach(countCoord(counter, _))
      }
      case ExternFunctionNameI(humanName, templateArgs, parameters) => {
        templateArgs.foreach(countTemplata(counter, _))
        parameters.foreach(countCoord(counter, _))
      }
      case LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(codeLoc, paramsTT), templateArgs, parameters) => {
        templateArgs.foreach(countTemplata(counter, _))
        parameters.foreach(countCoord(counter, _))
      }
      case AnonymousSubstructConstructorNameI(AnonymousSubstructConstructorTemplateNameI(substruct), templateArgs, parameters) => {
        countName(counter, substruct)
        templateArgs.foreach(countTemplata(counter, _))
        parameters.foreach(countCoord(counter, _))
      }
      case ForwarderFunctionNameI(ForwarderFunctionTemplateNameI(funcTemplateName, index), funcName) => {
        countName(counter, funcTemplateName)
        countFunctionName(counter, funcName)
      }
    //   case OverrideDispatcherNameI(OverrideDispatcherTemplateNameI(implId), templateArgs, parameters) => {
    //     countId(counter, implId, x => countName(counter, x))
    //     templateArgs.foreach(countTemplata(counter, _))
    //     parameters.foreach(countCoord(counter, _))
    //   }
    //   case CaseFunctionFromImplNameI(CaseFunctionFromImplTemplateNameI(humanName, runeInImpl, runeInCitizen), templateArgs, parameters) => {
    //     templateArgs.foreach(countTemplata(counter, _))
    //     parameters.foreach(countCoord(counter, _))
    //   }
    }
  }

*/
// mig: fn count_citizen_name
pub fn count_citizen_name() {
    panic!("Unimplemented: count_citizen_name");
}
/*
  def countCitizenName(
      counter: Counter,
      name: ICitizenNameI[sI]):
  Unit = {
    name match {
      case StructNameI(StructTemplateNameI(humanName), templateArgs) => {
        templateArgs.foreach(countTemplata(counter, _))
      }
      case LambdaCitizenNameI(template) => {
      }
      case InterfaceNameI(InterfaceTemplateNameI(humanName), templateArgs) => {
        templateArgs.foreach(countTemplata(counter, _))
      }
      case AnonymousSubstructNameI(AnonymousSubstructTemplateNameI(interface), templateArgs) => {
        countName(counter, interface)
        templateArgs.foreach(countTemplata(counter, _))
      }
    }
  }

*/
// mig: fn count_var_name
pub fn count_var_name() {
    panic!("Unimplemented: count_var_name");
}
/*
  def countVarName(
    counter: Counter,
    name: IVarNameI[sI]):
  Unit = {
    name match {
      case CodeVarNameI(name) =>
      case TypingPassBlockResultVarNameI(life) =>
      case TypingPassTemporaryVarNameI(life) =>
      case TypingPassFunctionResultVarNameI() =>
    }
  }

*/
// mig: fn count_name
pub fn count_name() {
    panic!("Unimplemented: count_name");
}
/*
  def countName(
    counter: Counter,
    name: INameI[sI]):
  Unit = {
    name match {
      case z : IFunctionNameI[_] => {
        // Scala can't seem to match generics.
        val x = z.asInstanceOf[IFunctionNameI[sI]]
        countFunctionName(counter, x)
      }
      case ExportNameI(template, region) => {
        counter.count(region)
      }
      case ExternNameI(template, region) => {
        counter.count(region)
      }
      case c : ICitizenNameI[_] => {
        // Scala can't seem to match generics.
        val x = c.asInstanceOf[ICitizenNameI[sI]]
        countCitizenName(counter, x)
      }
      case StructNameI(template, templateArgs) => {
        templateArgs.foreach(arg => countTemplata(counter, arg))
      }
      case StructTemplateNameI(_) =>
      case LambdaCitizenTemplateNameI(_) =>
      case InterfaceTemplateNameI(humanNamee) =>
      case AnonymousSubstructTemplateNameI(interface) => {
        countName(counter, interface)
      }
      case FunctionTemplateNameI(humanName, codeLocation) =>
      // case AnonymousSubstructImplTemplateNameI(interface) => {
      //   countName(counter, interface)
      // }
      case other => vimpl(other)
    }
  }

*/
// mig: fn count_templata
pub fn count_templata() {
    panic!("Unimplemented: count_templata");
}
/*
  def countTemplata(
    counter: Counter,
    templata: ITemplataI[sI]):
  Unit = {
    templata match {
      case CoordTemplataI(region, coord) => {
        countTemplata(counter, region)
        countCoord(counter, coord)
      }
      case KindTemplataI(kind) => countKind(counter, kind)
      case r @ RegionTemplataI(_) => counter.count(r)
      case MutabilityTemplataI(mutability) =>
      case IntegerTemplataI(_) =>
      case VariabilityTemplataI(variability) =>
      case other => vimpl(other)
    }
  }

*/
// mig: fn count_coord
pub fn count_coord() {
    panic!("Unimplemented: count_coord");
}
/*
  def countCoord(
    counter: Counter,
    coord: CoordI[sI]):
  Unit = {
    val CoordI(ownership, kind) = coord
    countKind(counter, kind)
  }

*/
// mig: fn count_kind
pub fn count_kind_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_kind");
}
/*
  def countKind(kind: KindIT[sI]): Map[Int, Int] = {
    val map = new RegionCounter.Counter()
    RegionCounter.countKind(map, kind)
    map.assembleMap()
  }

*/
// mig: fn count_kind
pub fn count_kind() {
    panic!("Unimplemented: count_kind");
}
/*
  def countKind(
    counter: Counter,
    kind: KindIT[sI]):
  Unit = {
    kind match {
      case NeverIT(_) =>
      case VoidIT() =>
      case IntIT(_) =>
      case BoolIT() =>
      case FloatIT() =>
      case StrIT() =>
      case StructIT(id) => countStructId(counter, id)
      case InterfaceIT(id) => countInterfaceId(counter, id)
      case StaticSizedArrayIT(ssaId) => {
        countId[StaticSizedArrayNameI[sI]](
          counter,
          ssaId,
          { case StaticSizedArrayNameI(template, size, variability, RawArrayNameI(mutability, elementType, selfRegion)) =>
            countTemplata(elementType)
            counter.count(selfRegion)
          })
      }
      case RuntimeSizedArrayIT(ssaId) => {
        countId[RuntimeSizedArrayNameI[sI]](
          counter,
          ssaId,
          { case RuntimeSizedArrayNameI(template, RawArrayNameI(mutability, elementType, selfRegion)) =>
            countTemplata(elementType)
            counter.count(selfRegion)
          })
      }
    }
  }

*/
// mig: fn count_runtime_sized_array
pub fn count_runtime_sized_array() {
    panic!("Unimplemented: count_runtime_sized_array");
}
/*
  def countRuntimeSizedArray(
    counter: Counter,
    rsa: RuntimeSizedArrayIT[sI]):
  Unit = {
    val RuntimeSizedArrayIT(rsaId) = rsa
    countId[RuntimeSizedArrayNameI[sI]](
      counter,
      rsaId,
      { case RuntimeSizedArrayNameI(template, RawArrayNameI(mutability, elementType, selfRegion)) =>
        countTemplata(counter, elementType)
        counter.count(selfRegion)
      })
  }

*/
// mig: fn count_static_sized_array
pub fn count_static_sized_array() {
    panic!("Unimplemented: count_static_sized_array");
}
/*
  def countStaticSizedArray(
    counter: Counter,
    ssa: StaticSizedArrayIT[sI]):
  Unit = {
    val StaticSizedArrayIT(ssaId) = ssa
    countId[StaticSizedArrayNameI[sI]](
      counter,
      ssaId,
      { case StaticSizedArrayNameI(template, size, variability, RawArrayNameI(mutability, elementType, selfRegion)) =>
        countTemplata(elementType)
        counter.count(selfRegion)
      })
  }

*/
// mig: fn count_citizen_id
pub fn count_citizen_id() {
    panic!("Unimplemented: count_citizen_id");
}
/*
  def countCitizenId(
      counter: Counter,
      citizenId: IdI[sI, ICitizenNameI[sI]]):
  Unit = {
    citizenId match {
      case IdI(packageCoord, initSteps, localName: IStructNameI[_]) => {
        countStructId(IdI(packageCoord, initSteps, localName.asInstanceOf[IStructNameI[sI]]))
      }
      case IdI(packageCoord, initSteps, localName: IInterfaceNameI[_]) => {
        countInterfaceId(IdI(packageCoord, initSteps, localName.asInstanceOf[IInterfaceNameI[sI]]))
      }
    }
  }

*/
// mig: fn count_struct_id
pub fn count_struct_id() {
    panic!("Unimplemented: count_struct_id");
}
/*
  def countStructId(
    counter: Counter,
    structId: IdI[sI, IStructNameI[sI]]):
  Unit = {
    countId[IStructNameI[sI]](
      counter,
      structId,
      countStructName(counter, _))
  }

*/
// mig: fn count_struct_template_name
pub fn count_struct_template_name() {
    panic!("Unimplemented: count_struct_template_name");
}
/*
  def countStructTemplateName(
      counter: Counter,
      structName: IStructTemplateNameI[sI]):
  Unit = {
    structName match {
      case StructTemplateNameI(humanName) => StructTemplateNameI(humanName)
    }
  }

*/
// mig: fn count_struct_name
pub fn count_struct_name() {
    panic!("Unimplemented: count_struct_name");
}
/*
  def countStructName(
      counter: Counter,
      structName: IStructNameI[sI]):
  Unit = {
    structName match {
      case StructNameI(template, templateArgs) => {
        countStructTemplateName(counter, template)
        templateArgs.foreach(countTemplata(counter, _))
      }
      case LambdaCitizenNameI(template) => {
      }
      case AnonymousSubstructNameI(template, templateArgs) => {
        templateArgs.foreach(countTemplata(counter, _))
      }
    }
  }

*/
// mig: fn count_impl_id
pub fn count_impl_id() {
    panic!("Unimplemented: count_impl_id");
}
/*
  def countImplId(
    counter: Counter,
    structId: IdI[sI, IImplNameI[sI]]):
  Unit = {
    countId[IImplNameI[sI]](
      counter,
      structId,
      x => countImplName(counter, x))
  }

*/
// mig: fn count_impl_name
pub fn count_impl_name() {
    panic!("Unimplemented: count_impl_name");
}
/*
  def countImplName(
      counter: Counter,
      implId: IImplNameI[sI]):
  Unit = {
    implId match {
      case ImplNameI(template, templateArgs, subCitizen) => {
        countImplTemplateName(counter, template)
        templateArgs.foreach(countTemplata(counter, _))
        countCitizenId(subCitizen.id)
      }
      case AnonymousSubstructImplNameI(AnonymousSubstructImplTemplateNameI(interface), templateArgs, subCitizen) => {
        countName(counter, interface)
        templateArgs.foreach(countTemplata(counter, _))
        countCitizenId(subCitizen.id)
      }
      case ImplBoundNameI(ImplBoundTemplateNameI(codeLocationS), templateArgs) => {
        templateArgs.foreach(countTemplata(counter, _))
      }
    }
  }

*/
// mig: fn count_impl_template_name
pub fn count_impl_template_name() {
    panic!("Unimplemented: count_impl_template_name");
}
/*
  def countImplTemplateName(
    counter: Counter,
    structName: IImplTemplateNameI[sI]):
  Unit = {
    structName match {
      case ImplTemplateNameI(humanName) => ImplTemplateNameI(humanName)
    }
  }

*/
// mig: fn count_export_id
pub fn count_export_id() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_export_id");
}
/*
  def countExportId(idI: IdI[sI, ExportNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(counter, idI, (x: ExportNameI[sI]) => RegionCounter.countName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_extern_id
pub fn count_extern_id() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_extern_id");
}
/*
  def countExternId(idI: IdI[sI, ExternNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(counter, idI, (x: ExternNameI[sI]) => RegionCounter.countName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_struct_id
pub fn count_struct_id_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_struct_id");
}
/*
  def countStructId(idI: IdI[sI, IStructNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(counter, idI, (x: IStructNameI[sI]) => RegionCounter.countName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_interface_id
pub fn count_interface_id_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_interface_id");
}
/*
  def countInterfaceId(idI: IdI[sI, IInterfaceNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    countInterfaceId(counter, idI)
    counter.assembleMap()
  }

*/
// mig: fn count_interface_id
pub fn count_interface_id() {
    panic!("Unimplemented: count_interface_id");
}
/*
  def countInterfaceId(
      counter: Counter,
      interfaceId: IdI[sI, IInterfaceNameI[sI]]):
  Unit = {
    RegionCounter.countId(counter, interfaceId, (x: IInterfaceNameI[sI]) => RegionCounter.countName(counter, x))
  }

*/
// mig: fn count_function_id
pub fn count_function_id_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_function_id");
}
/*
  def countFunctionId(idI: IdI[sI, IFunctionNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(counter, idI, (x: IFunctionNameI[sI]) => RegionCounter.countName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_impl_id
pub fn count_impl_id_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_impl_id");
}
/*
  def countImplId(
    implId: IdI[sI, IImplNameI[sI]]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(
      counter, implId, (x: IImplNameI[sI]) => RegionCounter.countImplName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_coord
pub fn count_coord_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_coord");
}
/*
  def countCoord(coord: CoordI[sI]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countCoord(counter, coord)
    counter.assembleMap()
  }

*/
// mig: fn count_var_name
pub fn count_var_name_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_var_name");
}
/*
  def countVarName(
    name: IVarNameI[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countVarName(counter, name)
    counter.assembleMap()
  }

*/
// mig: fn count_static_sized_array
pub fn count_static_sized_array_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_static_sized_array");
}
/*
  def countStaticSizedArray(
    ssa: StaticSizedArrayIT[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countStaticSizedArray(counter, ssa)
    counter.assembleMap()
  }

*/
// mig: fn count_runtime_sized_array
pub fn count_runtime_sized_array_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_runtime_sized_array");
}
/*
  def countRuntimeSizedArray(
    rsa: RuntimeSizedArrayIT[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countRuntimeSizedArray(counter, rsa)
    counter.assembleMap()
  }

*/
// mig: fn count_prototype
pub fn count_prototype_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_prototype");
}
/*
  def countPrototype(prototype: PrototypeI[sI]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countPrototype(counter, prototype)
    counter.assembleMap()
  }

*/
// mig: fn count_function_name
pub fn count_function_name_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_function_name");
}
/*
  def countFunctionName(
    name: IFunctionNameI[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countFunctionName(counter, name)
    counter.assembleMap()
  }

*/
// mig: fn count_impl_name
pub fn count_impl_name_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_impl_name");
}
/*
  def countImplName(
      name: IImplNameI[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countImplName(counter, name)
    counter.assembleMap()
  }

*/
// mig: fn count_citizen_name
pub fn count_citizen_name_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_citizen_name");
}
/*
  def countCitizenName(
      name: ICitizenNameI[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countCitizenName(counter, name)
    counter.assembleMap()
  }

*/
// mig: fn count_citizen_id
pub fn count_citizen_id_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_citizen_id");
}
/*
  def countCitizenId(
      name: IdI[sI, ICitizenNameI[sI]]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countCitizenId(counter, name)
    counter.assembleMap()
  }

*/
// mig: fn count_templata
pub fn count_templata_map() -> std::collections::HashMap<i32, i32> {
    panic!("Unimplemented: count_templata");
}
/*
  def countTemplata(
      name: ITemplataI[sI]):
  Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countTemplata(counter, name)
    counter.assembleMap()
  }
}
*/