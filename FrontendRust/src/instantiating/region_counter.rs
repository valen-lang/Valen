/*
package dev.vale.instantiating

import dev.vale.instantiating.RegionCounter._
import dev.vale.instantiating.ast._
import dev.vale.{U, vassert, vimpl, vwat}

import scala.collection.mutable

object RegionCounter {
*/
use crate::instantiating::ast::names::{IdI, INameI, IFunctionNameI, FunctionNameIX, ExternFunctionNameI};
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::types::{CoordI, KindIT};
use crate::instantiating::ast::types::sI;
use crate::instantiating::ast::templata::RegionTemplataI;

// mig: struct CounterI
pub struct CounterI {
    set: std::collections::HashSet<i32>,
}
// mig: impl CounterI
// mig: fn new
impl CounterI {
    pub fn new() -> Self {
        CounterI { set: std::collections::HashSet::new() }
    }
}
/*
  class Counter {
    // TODO(optimize): Use an array for this, with a minimum index and maximum index (similar to
    // what a circular queue uses)
    val set: mutable.HashSet[Int] = mutable.HashSet[Int]()

*/
// mig: fn count
impl CounterI {
    pub fn count<'s, 'i>(&mut self, region: RegionTemplataI<'s, 'i, sI>) where 's: 'i {
        self.set.insert(region.pure_height);
    }
}
/*
    def count(region: RegionTemplataI[sI]): Unit = {
      set.add(region.pureHeight)
    }

*/
// mig: fn assemble_map
impl CounterI {
    pub fn assemble_map(&self) -> std::collections::HashMap<i32, i32> {
        let num_regions = self.set.len();
        // Let's say we have a set that contains 3, 5, -2, 0, 4, it becomes...
        let mut sorted = self.set.iter().copied().collect::<Vec<i32>>();
        sorted.sort(); // -2, 0, 3, 4, 5
        sorted.into_iter().enumerate() // (-2, 0), (0, 1), (3, 2), (4, 3), (5, 4)
            .map(|(i, subjective_region)| {
                // If we have 4 regions, then they should go from -3 to 0
                (subjective_region, i as i32 - num_regions as i32 + 1)
            }) // (-2, -4), (0, -3), (3, -2), (4, -1), (5, 0)
            .collect()
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
pub fn count_prototype<'s, 'i>(counter: &mut CounterI, prototype: &PrototypeI<'s, 'i, sI>)
where 's: 'i {
    let PrototypeI { id, return_type, .. } = *prototype;
    count_function_id(counter, &id);
    count_coord(counter, &return_type);
}
/*
  def countPrototype(counter: Counter, prototype: PrototypeI[sI]): Unit = {
    val PrototypeI(id, returnType) = prototype
    countFunctionId(counter, id)
    countCoord(counter, returnType)
  }

*/
// mig: fn count_id
pub fn count_id<'s, 'i>(counter: &mut CounterI, id_i: &IdI<'s, 'i, sI>, func: impl Fn(&mut CounterI, &INameI<'s, 'i, sI>))
where 's: 'i {
    let IdI { package_coord: _package_coord, init_steps, local_name } = *id_i;
    for x in init_steps {
        count_name(counter, x);
    }
    func(counter, &local_name);
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
pub fn count_function_id<'s, 'i>(counter: &mut CounterI, id: &IdI<'s, 'i, sI>)
where 's: 'i {
    count_id(counter, id, |counter, x| count_function_name(counter, &IFunctionNameI::try_from(*x).unwrap()))
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
pub fn count_function_name<'s, 'i>(counter: &mut CounterI, name: &IFunctionNameI<'s, 'i, sI>)
where 's: 'i {
    match *name {
        IFunctionNameI::Function(n) => {
            let FunctionNameIX { template_args, parameters, .. } = *n;
            for template_arg in template_args { count_templata(counter, template_arg) }
            for param in parameters { count_coord(counter, param) }
        }
        IFunctionNameI::ExternFunction(n) => {
            let ExternFunctionNameI { template_args, parameters, .. } = *n;
            for _template_arg in template_args { panic!("Unimplemented: count_function_name ExternFunction countTemplata") }
            for param in parameters { count_coord(counter, param) }
        }
        IFunctionNameI::LambdaCallFunction(_) => panic!("Unimplemented: count_function_name LambdaCallFunction"),
        IFunctionNameI::AnonymousSubstructConstructor(_) => panic!("Unimplemented: count_function_name AnonymousSubstructConstructor"),
        IFunctionNameI::ForwarderFunction(_) => panic!("Unimplemented: count_function_name ForwarderFunction"),
        _ => panic!("Unimplemented: count_function_name other"),
    }
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
pub fn count_citizen_name<'s, 'i>(counter: &mut CounterI, name: &crate::instantiating::ast::names::ICitizenNameI<'s, 'i, sI>) {
    use crate::instantiating::ast::names::{ICitizenNameI, StructNameI, InterfaceNameI};
    match name {
        ICitizenNameI::Struct(StructNameI { template: _, template_args }) => {
            for t in template_args.iter() { count_templata(counter, t); }
        }
        ICitizenNameI::LambdaCitizen(_) => {}
        ICitizenNameI::Interface(InterfaceNameI { template: _, template_args }) => {
            for t in template_args.iter() { count_templata(counter, t); }
        }
        ICitizenNameI::AnonymousSubstruct(_) => panic!("count_citizen_name: AnonymousSubstruct branch"),
        ICitizenNameI::StaticSizedArray(_) => panic!("count_citizen_name: StaticSizedArray branch (no Scala counterpart)"),
        ICitizenNameI::RuntimeSizedArray(_) => panic!("count_citizen_name: RuntimeSizedArray branch (no Scala counterpart)"),
    }
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
pub fn count_name<'s, 'i>(counter: &mut CounterI, name: &INameI<'s, 'i, sI>)
where 's: 'i {
    match name {
        INameI::Export(export_name) => {
            counter.count(export_name.region);
        }
        INameI::Extern(extern_name) => {
            counter.count(extern_name.region);
        }
        _ => panic!("Unimplemented: count_name"),
    }
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
pub fn count_templata<'s, 'i>(_counter: &mut CounterI, _templata: &crate::instantiating::ast::templata::ITemplataI<'s, 'i, sI>) {
    use crate::instantiating::ast::templata::ITemplataI;
    match _templata {
        ITemplataI::Coord(c) => {
            count_templata(_counter, &ITemplataI::Region(c.region));
            count_coord(_counter, &c.coord);
        }
        ITemplataI::Kind(k) => count_kind(_counter, &k.kind),
        ITemplataI::Region(r) => _counter.count(*r),
        ITemplataI::Mutability(_) => {}
        ITemplataI::Integer(_) => {}
        ITemplataI::Variability(_) => {}
        _ => panic!("count_templata: unimplemented variant"),
    }
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
pub fn count_coord<'s, 'i>(counter: &mut CounterI, coord: &CoordI<'s, 'i, sI>)
where 's: 'i {
    let CoordI { ownership: _ownership, kind } = *coord;
    count_kind(counter, &kind);
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
pub fn count_kind<'s, 'i>(counter: &mut CounterI, kind: &KindIT<'s, 'i, sI>)
where 's: 'i {
    match kind {
        KindIT::NeverIT(_) => {}
        KindIT::VoidIT(_) => {}
        KindIT::IntIT(_) => {}
        KindIT::BoolIT(_) => {}
        KindIT::FloatIT(_) => {}
        KindIT::StrIT(_) => {}
        KindIT::StructIT(s) => count_struct_id(counter, &s.id),
        KindIT::InterfaceIT(i) => count_interface_id(counter, &i.id),
        KindIT::StaticSizedArrayIT(_) => panic!("Unimplemented: count_kind StaticSizedArray"),
        KindIT::RuntimeSizedArrayIT(_) => panic!("Unimplemented: count_kind RuntimeSizedArray"),
    }
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
pub fn count_citizen_id<'s, 'i>(counter: &mut CounterI, citizen_id: &IdI<'s, 'i, sI>)
where 's: 'i {
    match citizen_id.local_name {
        INameI::StructName(_) => count_struct_id(counter, citizen_id),
        INameI::InterfaceName(_) => count_interface_id(counter, citizen_id),
        _ => panic!("count_citizen_id: non-citizen local name"),
    }
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
pub fn count_struct_id<'s, 'i>(counter: &mut CounterI, struct_id: &IdI<'s, 'i, sI>)
where 's: 'i {
    count_id(counter, struct_id, |counter, x| count_struct_name(counter, &crate::instantiating::ast::names::IStructNameI::try_from(*x).unwrap()))
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
pub fn count_struct_template_name<'s, 'i>(_counter: &mut CounterI, struct_name: &crate::instantiating::ast::names::IStructTemplateNameI<'s, 'i, sI>)
where 's: 'i {
    use crate::instantiating::ast::names::IStructTemplateNameI;
    match struct_name {
        IStructTemplateNameI::StructTemplate(_) => {}
        _ => panic!("count_struct_template_name: other"),
    }
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
pub fn count_struct_name<'s, 'i>(counter: &mut CounterI, struct_name: &crate::instantiating::ast::names::IStructNameI<'s, 'i, sI>)
where 's: 'i {
    use crate::instantiating::ast::names::{IStructNameI, StructNameI};
    match struct_name {
        IStructNameI::Struct(StructNameI { template, template_args }) => {
            count_struct_template_name(counter, template);
            for t in template_args.iter() { count_templata(counter, t); }
        }
        IStructNameI::LambdaCitizen(_) => {}
        IStructNameI::AnonymousSubstruct(_) => panic!("count_struct_name: AnonymousSubstruct branch"),
    }
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
pub fn count_impl_id<'s, 'i>(counter: &mut CounterI, struct_id: &IdI<'s, 'i, sI>)
where 's: 'i {
    count_id(counter, struct_id, |counter, x| count_impl_name(counter, &crate::instantiating::ast::names::IImplNameI::try_from(*x).unwrap()))
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
pub fn count_impl_name<'s, 'i>(counter: &mut CounterI, impl_id: &crate::instantiating::ast::names::IImplNameI<'s, 'i, sI>)
where 's: 'i {
    use crate::instantiating::ast::names::{IImplNameI, ImplNameI};
    match impl_id {
        IImplNameI::Impl(ImplNameI { template, template_args, sub_citizen }) => {
            count_impl_template_name(counter, template);
            for t in template_args.iter() { count_templata(counter, t); }
            count_citizen_id(counter, &sub_citizen.id());
        }
        IImplNameI::AnonymousSubstructImpl(_) => panic!("count_impl_name: AnonymousSubstructImpl branch"),
        IImplNameI::ImplBound(_) => panic!("count_impl_name: ImplBound branch"),
    }
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
pub fn count_impl_template_name<'s, 'i>(_counter: &mut CounterI, name: &crate::instantiating::ast::names::IImplTemplateNameI<'s, 'i, sI>)
where 's: 'i {
    use crate::instantiating::ast::names::IImplTemplateNameI;
    match name {
        IImplTemplateNameI::ImplTemplate(_) => {}
        _ => panic!("count_impl_template_name: other"),
    }
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
pub fn count_export_id<'s, 'i>(id_i: &IdI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_id(&mut counter, id_i, |counter, x| count_name(counter, x));
    counter.assemble_map()
}
/*
  def countExportId(idI: IdI[sI, ExportNameI[sI]]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countId(counter, idI, (x: ExportNameI[sI]) => RegionCounter.countName(counter, x))
    counter.assembleMap()
  }

*/
// mig: fn count_extern_id
pub fn count_extern_id<'s, 'i>(id_i: &IdI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_id(&mut counter, id_i, |counter, x| count_name(counter, x));
    counter.assemble_map()
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
pub fn count_interface_id<'s, 'i>(counter: &mut CounterI, interface_id: &IdI<'s, 'i, sI>)
where 's: 'i {
    count_id(counter, interface_id, |counter, x| count_name(counter, x))
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
pub fn count_impl_id_map<'s, 'i>(id_i: &IdI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_id(&mut counter, id_i, |counter, x| count_impl_name(counter, &crate::instantiating::ast::names::IImplNameI::try_from(*x).unwrap()));
    counter.assemble_map()
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
pub fn count_prototype_map<'s, 'i>(prototype: &PrototypeI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_prototype(&mut counter, prototype);
    counter.assemble_map()
}
/*
  def countPrototype(prototype: PrototypeI[sI]): Map[Int, Int] = {
    val counter = new RegionCounter.Counter()
    RegionCounter.countPrototype(counter, prototype)
    counter.assembleMap()
  }

*/
// mig: fn count_function_name
pub fn count_function_name_map<'s, 'i>(name: &IFunctionNameI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_function_name(&mut counter, name);
    counter.assemble_map()
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
pub fn count_impl_name_map<'s, 'i>(name: &crate::instantiating::ast::names::IImplNameI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32>
where 's: 'i {
    let mut counter = CounterI::new();
    count_impl_name(&mut counter, name);
    counter.assemble_map()
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
pub fn count_citizen_name_map<'s, 'i>(name: &crate::instantiating::ast::names::ICitizenNameI<'s, 'i, sI>) -> std::collections::HashMap<i32, i32> {
    let mut counter = CounterI::new();
    count_citizen_name(&mut counter, name);
    counter.assemble_map()
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