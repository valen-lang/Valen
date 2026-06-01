// From Frontend/SimplifyingPass/src/dev/vale/simplifying/NameHammer.scala
//
// Per typing-pass `Compiler` precedent, `NameHammer` is not a Rust struct.
// Its methods live as `impl Hammer { ... }` blocks colocated here; its free
// functions (in Scala's `object NameHammer`) become module-level Rust fns.

use crate::interner::StrI;
use crate::utils::range::CodeLocationS;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::von::ast::VonObject;
use crate::final_ast::ast::IdH;
use crate::final_ast::types::{SimpleId, SimpleIdStep};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::Hammer;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::{IdI, INameI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::{cI, CoordI, KindIT};

/*
package dev.vale.simplifying

import dev.vale._
import dev.vale.finalast.IdH
import dev.vale.finalast._
import dev.vale.postparsing.AnonymousSubstructParentInterfaceTemplateRuneS
import dev.vale.instantiating._
import dev.vale.instantiating.ast._
import dev.vale.von.{IVonData, VonArray, VonInt, VonMember, VonObject, VonStr}

import scala.collection.immutable.List

class NameHammer() {
*/

// mig: fn translate_full_name
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_full_name(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &Hamuts<'s, 'i, 'h>,
        full_name2: &IdI<'s, 'i, cI>,
    ) -> &'h IdH<'s, 'h>
    {
        let IdI { package_coord, init_steps: _, local_name: local_name_t } = full_name2;
        let code_map = |loc: crate::utils::range::CodeLocationS<'s>| format!("{:?}", loc);
        let long_name = crate::instantiating::instantiated_humanizer::humanize_id(&code_map, full_name2, None);
        let local_name = crate::instantiating::instantiated_humanizer::humanize_name(&code_map, *local_name_t, None);
        self.interner.intern_id_h(crate::final_ast::ast::IdHValH {
            local_name: self.scout_arena.intern_str(&local_name),
            package_coordinate: **package_coord,
            shortened_name: self.scout_arena.intern_str(&long_name),
            fully_qualified_name: self.scout_arena.intern_str(&long_name),
            _phantom_h: std::marker::PhantomData,
        })
    }
}
/*
  def translateFullName(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    fullName2: IdI[cI, INameI[cI]]
  ): IdH = {
    val IdI(packageCoord, _, localNameT) = fullName2
    val longName = InstantiatedHumanizer.humanizeId(_.toString, fullName2)
    val localName = InstantiatedHumanizer.humanizeName(_.toString, localNameT)
    finalast.IdH(localName, packageCoord, longName, longName)
  }
*/

// mig: fn add_step
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn add_step(
        &self,
        hamuts: &Hamuts<'s, 'i, 'h>,
        full_name: &IdH<'s, 'h>,
        s: StrI<'s>,
    ) -> &'h IdH<'s, 'h>
    {
        panic!("Unimplemented: add_step");
    }
}
/*
  // Adds a step to the name.
  def addStep(
    hamuts: HamutsBox,
    fullName: IdH,
    s: String):
  IdH = {
    val IdH(_, packageCoordinate, shortenedName, fullyQualifiedName) = fullName
    IdH(s, packageCoordinate, shortenedName + "." + s, fullyQualifiedName + "." + s)
  }
}
*/

/*
object NameHammer {
*/

// mig: fn translate_code_location (object NameHammer free function)
pub fn translate_code_location<'p>(location: &CodeLocationS<'p>) -> VonObject {
    panic!("Unimplemented: translate_code_location");
}
/*
  def translateCodeLocation(location: CodeLocationS): VonObject = {
    val CodeLocationS(fileCoord, offset) = location
    VonObject(
      "CodeLocation",
      None,
      Vector(
        VonMember("file", translateFileCoordinate(fileCoord)),
        VonMember("offset", VonInt(offset))))
  }
*/

// mig: fn translate_file_coordinate (object NameHammer free function)
pub fn translate_file_coordinate<'p>(coord: &FileCoordinate<'p>) -> VonObject {
    panic!("Unimplemented: translate_file_coordinate");
}
/*
  def translateFileCoordinate(coord: FileCoordinate): VonObject = {
    val FileCoordinate(PackageCoordinate(module, paackage), filename) = coord
    VonObject(
      "FileCoordinate",
      None,
      Vector(
        VonMember("module", VonStr(module.str)),
        VonMember("paackage", VonArray(None, paackage.map(_.str).map(VonStr).toVector)),
        VonMember("filename", VonStr(filename))))
  }
*/

// mig: fn translate_package_coordinate (object NameHammer free function)
pub fn translate_package_coordinate<'p>(coord: &PackageCoordinate<'p>) -> VonObject {
    let PackageCoordinate { module, packages: paackage } = coord;
    let non_empty_module_name = if module.0 == "" { "__vale".to_string() } else { module.0.to_string() };
    crate::von::ast::VonObject {
        tyype: "PackageCoordinate".to_string(),
        id: None,
        members: vec![
            crate::von::ast::VonMember {
                field_name: "project".to_string(),
                value: crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: non_empty_module_name }),
            },
            crate::von::ast::VonMember {
                field_name: "packageSteps".to_string(),
                value: crate::von::ast::IVonData::Array(crate::von::ast::VonArray {
                    id: None,
                    members: paackage.iter().map(|s| crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: s.0.to_string() })).collect(),
                }),
            },
        ],
    }
}
/*
  def translatePackageCoordinate(coord: PackageCoordinate): VonObject = {
    val PackageCoordinate(module, paackage) = coord
    val nonEmptyModuleName = if (module.str == "") "__vale" else module.str;
    VonObject(
      "PackageCoordinate",
      None,
      Vector(
        VonMember("project", VonStr(nonEmptyModuleName)),
        VonMember("packageSteps", VonArray(None, paackage.map(_.str).map(VonStr).toVector))))
  }
*/

// mig: fn simplify_id (object NameHammer free function)
pub fn simplify_id<'s, 'i, 'h>(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, id: &IdI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    let IdI { package_coord, init_steps, local_name } = id;
    let PackageCoordinate { module, packages } = **package_coord;
    let mut steps: Vec<SimpleIdStep<'s, 'h>> = Vec::new();
    steps.push(SimpleIdStep { name: module, template_args: &[] });
    for paackage in packages.iter() {
        steps.push(SimpleIdStep { name: *paackage, template_args: &[] });
    }
    for step in init_steps.iter() {
        steps.push(simplify_name(interner, scout_arena, step));
    }
    steps.push(simplify_name(interner, scout_arena, local_name));
    SimpleId { steps: interner.alloc_slice_from_vec(steps) }
}
/*
  def simplifyId(id: IdI[cI, INameI[cI]]): SimpleId = {
    val IdI(packageCoord, initSteps, localName) = id
    val PackageCoordinate(module, packages) = packageCoord
    SimpleId(
      (SimpleIdStep(module.str, Vector()) +:
          packages.map(paackage => SimpleIdStep(paackage.str, Vector()))) ++
          initSteps.map(step => simplifyName(step)) :+
          simplifyName(localName))
  }
*/

// mig: fn simplify_name (object NameHammer free function)
pub fn simplify_name<'s, 'i, 'h>(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, name: &INameI<'s, 'i, cI>) -> SimpleIdStep<'s, 'h>
where 's: 'i, 'i: 'h,
{
    match name {
        INameI::StructName(crate::instantiating::ast::names::StructNameI { template: crate::instantiating::ast::names::IStructTemplateNameI::StructTemplate(t), template_args }) => SimpleIdStep {
            name: t.human_name,
            template_args: interner.alloc_slice_from_vec(template_args.iter().map(|t| simplify_templata(interner, scout_arena, t)).collect()),
        },
        INameI::StructName(_) => panic!("simplify_name: StructName non-StructTemplate inner"),
        INameI::StructTemplate(s) => SimpleIdStep {
            name: s.human_name,
            template_args: &[],
        },
        INameI::InterfaceName(i) => panic!("simplify_name: InterfaceName branch"),
        INameI::InterfaceTemplate(i) => panic!("simplify_name: InterfaceTemplate branch"),
        INameI::ExternFunction(f) => SimpleIdStep {
            name: f.human_name,
            template_args: interner.alloc_slice_from_vec(f.template_args.iter().map(|t| simplify_templata(interner, scout_arena, t)).collect()),
        },
        other => panic!("simplify_name: unimplemented variant {:?}", std::mem::discriminant(other)),
    }
}
/*
  def simplifyName(name: INameI[cI]): SimpleIdStep = {
    name match {
      case StructNameI(StructTemplateNameI(humanName), templateArgs) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case StructTemplateNameI(humanName) =>
        SimpleIdStep(humanName.str, Vector())
      case InterfaceNameI(InterfaceTemplateNameI(humanName), templateArgs) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case InterfaceTemplateNameI(humanName) =>
        SimpleIdStep(humanName.str, Vector())
      case ExternFunctionNameI(humanName, templateArgs, parameters) =>
        SimpleIdStep(humanName.str, templateArgs.map(simplifyTemplata))
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_templata (object NameHammer free function)
pub fn simplify_templata<'s, 'i, 'h>(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, templata: &ITemplataI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    match templata {
        ITemplataI::Coord(c) => simplify_coord(interner, scout_arena, &c.coord),
        other => panic!("simplify_templata: unimplemented variant {:?}", std::mem::discriminant(other)),
    }
}
/*
  def simplifyTemplata(templata: ITemplataI[cI]): SimpleId = {
    templata match {
      case CoordTemplataI(region, coord) => simplifyCoord(coord)
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_kind (object NameHammer free function)
pub fn simplify_kind<'s, 'i, 'h>(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, value: &KindIT<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    match value {
        KindIT::IntIT(crate::instantiating::ast::types::IntIT { bits, .. }) => {
            let name = scout_arena.intern_str(&format!("i{}", bits));
            SimpleId { steps: interner.alloc_slice_from_vec(vec![SimpleIdStep { name, template_args: &[] }]) }
        }
        KindIT::StrIT(_) => {
            let name = scout_arena.intern_str("str");
            SimpleId { steps: interner.alloc_slice_from_vec(vec![SimpleIdStep { name, template_args: &[] }]) }
        }
        other => panic!("simplify_kind: unimplemented variant {:?}", std::mem::discriminant(other)),
    }
}
/*
  def simplifyKind(value: KindIT[cI]): SimpleId = {
    value match {
      case IntIT(bits) => SimpleId(Vector(SimpleIdStep("i" + bits, Vector())))
      case StrIT() => SimpleId(Vector(SimpleIdStep("str", Vector())))
      case other => vimpl(other)
    }
  }
*/

// mig: fn simplify_coord (object NameHammer free function)
pub fn simplify_coord<'s, 'i, 'h>(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, value: &CoordI<'s, 'i, cI>) -> SimpleId<'s, 'h>
where 's: 'i, 'i: 'h,
{
    let CoordI { ownership, kind } = *value;
    let kind_id = simplify_kind(interner, scout_arena, &kind);
    match ownership {
        crate::instantiating::ast::types::OwnershipI::ImmutableShare => kind_id,
        crate::instantiating::ast::types::OwnershipI::MutableShare => kind_id,
        crate::instantiating::ast::types::OwnershipI::Own => kind_id,
        crate::instantiating::ast::types::OwnershipI::Weak => panic!("simplify_coord: Weak"),
        crate::instantiating::ast::types::OwnershipI::ImmutableBorrow => panic!("simplify_coord: ImmutableBorrow"),
        crate::instantiating::ast::types::OwnershipI::MutableBorrow => panic!("simplify_coord: MutableBorrow"),
    }
}
/*
  def simplifyCoord(value: CoordI[cI]): SimpleId = {
    val CoordI(ownership, kind) = value
    val kindId = simplifyKind(kind)
    (ownership match {
      case ImmutableShareI => kindId
      case MutableShareI => kindId
      case OwnI => kindId
      case WeakI => vimpl()
      case ImmutableBorrowI => SimpleId(Vector(SimpleIdStep("&", Vector(kindId))))
      case MutableBorrowI => SimpleId(Vector(SimpleIdStep("&mut", Vector(kindId))))
    })
  }
}
*/
