// From Frontend/SimplifyingPass/src/dev/vale/simplifying/Hammer.scala
//
// Central `Hammer` god struct (typing-pass `Compiler` precedent — sub-hammers
// like `BlockHammer`/`ExpressionHammer`/etc. are NOT held as Rust struct
// fields. Their methods become `impl Hammer { ... }` blocks colocated in
// per-area files for organization).
//
// `LocalsBox` collapsed into single mutable `Locals` (architect-blessed
// pattern, same as `HamutsBox` → `Hamuts`).

use crate::final_ast::ast::{IdH, ProgramH, PrototypeH};
use crate::final_ast::instructions::{
    ConsecutorH, ConstantVoidH, ExpressionH, Local, StackifyH, VariableIdH,
};
use crate::final_ast::types::{CoordH, KindHT, NeverHT, Variability, VoidHT};
use crate::instantiating::ast::ast::{FunctionDefinitionI, FunctionExportI, FunctionExternI};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::{IdI, INameI, IVarNameI};
use crate::instantiating::ast::templata::ITemplataI;
use crate::instantiating::ast::types::{cI, CoordI, KindIT};
use crate::keywords::Keywords;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::simplifying::hamuts::Hamuts;
use std::collections::{HashMap, HashSet};

/*
package dev.vale.simplifying

import dev.vale.{Builtins, FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Profiler, Result, finalast, vassert, vcurious, vfail, vimpl, vwat}
import dev.vale.finalast.{ConsecutorH, ConstantVoidH, CoordH, ExpressionH, Final, IdH, KindHT, Local, NeverHT, PackageH, ProgramH, PrototypeH, StackifyH, Variability, VariableIdH, VoidHT}
import dev.vale.highertyping.ICompileErrorA
import dev.vale.finalast._
import dev.vale.instantiating.ast.{IdI, _}
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.List
*/

// mig: case class FunctionRefH (moved to src/final_ast/ast.rs — Scala has it
// in `Hammer.scala`, but it's a pure data type that colocates with the other
// finalast H-side AST. Field shape `{ prototype: &'h PrototypeH<...> }` per Scala.)
/*
case class FunctionRefH(prototype: PrototypeH) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
  //  def functionType = prototype.functionType
  def fullName = prototype.id
}
*/

// mig: case class LocalsBox (collapsed into Locals; see architect directive)
// Scala's LocalsBox was a mutable wrapper around an immutable Locals. Per
// architect directive (matching typing-pass `CompilerOutputs` precedent and
// the `HamutsBox` → `Hamuts` collapse), the Rust port has a single mutable
// `Locals` accumulator. The LocalsBox methods become &mut self/&self methods on
// Locals (below); the immutable `Locals.foo` functional-update methods collapse
// into those (preserved as audit-trail under `(collapsed)` markers).
/*
case class LocalsBox(var inner: Locals) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable
*/

// mig: fn snapshot
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn snapshot(&self) -> Locals<'s, 'i, 'h> {
        Locals {
            typing_pass_locals: self.typing_pass_locals.clone(),
            unstackified_vars: self.unstackified_vars.clone(),
            locals: self.locals.clone(),
            next_local_id_number: self.next_local_id_number,
        }
    }
}
/*
  def snapshot = inner
*/

// mig: fn typing_pass_locals
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn typing_pass_locals(&self) -> &HashMap<&'i IVarNameI<'s, 'i, cI>, VariableIdH<'s, 'h>> {
        panic!("Unimplemented: typing_pass_locals");
    }
}
/*
  def typingPassLocals: Map[IVarNameI[cI], VariableIdH] = inner.typingPassLocals
*/

// mig: fn unstackified_vars
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn unstackified_vars(&self) -> &HashSet<VariableIdH<'s, 'h>> {
        panic!("Unimplemented: unstackified_vars");
    }
}
/*
  def unstackifiedVars: Set[VariableIdH] = inner.unstackifiedVars
*/

// mig: fn locals
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn locals(&self) -> &HashMap<VariableIdH<'s, 'h>, Local<'s, 'h>> {
        panic!("Unimplemented: locals");
    }
}
/*
  def locals: Map[VariableIdH, Local] = inner.locals
*/

// mig: fn next_local_id_number
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn next_local_id_number(&self) -> i32 {
        panic!("Unimplemented: next_local_id_number");
    }
}
/*
  def nextLocalIdNumber: Int = inner.nextLocalIdNumber
*/

// mig: fn get_by_var_name (Scala overload `get(IVarNameI[cI])` —
// disambiguated per instantiating overload-suffix pattern.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn get_by_var_name(&self, id: &IVarNameI<'s, 'i, cI>) -> Option<Local<'s, 'h>> {
        self.typing_pass_locals.get(id).copied().and_then(|var_id| self.locals.get(&var_id).copied())
    }
}
/*
Guardian: temp-disable: SPDMX — Per documented file-top architecture (Locals collapsed LocalsBox+Locals into a single struct, same as HamutsBox/Hamuts collapse documented in hamuts.rs lines 4-12): Scala's outer `def get(id) = inner.get(id)` delegated to the inner Locals' two-step lookup. Rust collapsed both into a single mutable Locals. The "inner" version's body (typingPassLocals.get → locals.get) IS the collapsed implementation; see the audit-trail `def get(varId: IVarNameI[cI]): Option[Local]` Scala block at hammer.rs:411 which shows the two-step lookup. SPDMX Exception Q (god-struct merging) applies. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1873-1780116603775/hook-1873/next_local_id_number--122.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def get(id: IVarNameI[cI]) = inner.get(id)
*/

// mig: fn get (Scala overload `get(VariableIdH)`.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn get(&self, id: VariableIdH<'s, 'h>) -> Option<Local<'s, 'h>> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(id: VariableIdH) = inner.get(id)
*/

// mig: fn mark_unstackified_by_var_name (Scala overload disambiguated.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_unstackified_by_var_name(&mut self, var_id: &'i IVarNameI<'s, 'i, cI>) {
        panic!("Unimplemented: mark_unstackified_by_var_name");
    }
}
/*
  def markUnstackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markUnstackified(varId2)
  }
*/

// mig: fn mark_restackified_by_var_name (Scala overload disambiguated.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_restackified_by_var_name(&mut self, var_id: &'i IVarNameI<'s, 'i, cI>) {
        panic!("Unimplemented: mark_restackified_by_var_name");
    }
}
/*
  def markRestackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markRestackified(varId2)
  }
*/

// mig: fn mark_unstackified
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_unstackified(&mut self, var_id_h: VariableIdH<'s, 'h>) {
        panic!("Unimplemented: mark_unstackified");
    }
}
/*
  def markUnstackified(varIdH: VariableIdH): Unit = {
    inner = inner.markUnstackified(varIdH)
  }
*/

// mig: fn set_next_local_id_number
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn set_next_local_id_number(&mut self, next_local_id_number: i32) {
        panic!("Unimplemented: set_next_local_id_number");
    }
}
/*
  def setNextLocalIdNumber(nextLocalIdNumber: Int): Unit = {
    inner = inner.copy(nextLocalIdNumber = nextLocalIdNumber)
  }
*/

// mig: fn add_hammer_local
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn add_hammer_local(
        &mut self,
        tyype: CoordH<'s, 'h>,
        variability: Variability,
    ) -> Local<'s, 'h> {
        panic!("Unimplemented: add_hammer_local");
    }
}
/*
  def addHammerLocal(
    tyype: CoordH[KindHT],
    variability: Variability):
  Local = {
    val (newInner, local) = inner.addHammerLocal(tyype, variability)
    inner = newInner
    local
  }
*/

// mig: fn add_typing_pass_local (Scala name; matches Scala `addTypingPassLocal`.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn add_typing_pass_local(
        &mut self,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        var_id_name_h: &'h IdH<'s, 'h>,
        variability: Variability,
        tyype: CoordH<'s, 'h>,
    ) -> Local<'s, 'h> {
        self.add_compiler_local(var_id, var_id_name_h, variability, tyype)
    }
}
/*
  def addTypingPassLocal(
    varId2: IVarNameI[cI],
    varIdNameH: IdH,
    variability: Variability,
    tyype: CoordH[KindHT]):
  Local = {
    val (newInner, local) = inner.addCompilerLocal(varId2, varIdNameH, variability, tyype)
    inner = newInner
    local
  }

}
*/

// mig: case class Locals
/// Temporary state
pub struct Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub typing_pass_locals: HashMap<&'i IVarNameI<'s, 'i, cI>, VariableIdH<'s, 'h>>,
    pub unstackified_vars: HashSet<VariableIdH<'s, 'h>>,
    pub locals: HashMap<VariableIdH<'s, 'h>, Local<'s, 'h>>,
    pub next_local_id_number: i32,
}
/*
// This represents the locals for the entire function.
// Note, some locals will have the same index, that just means they're in
// different blocks.
case class Locals(
     // This doesn't have all the locals that are in the locals list, this just
     // has any locals added by typingpass.
     typingPassLocals: Map[IVarNameI[cI], VariableIdH],

     unstackifiedVars: Set[VariableIdH],

     // This has all the locals for the function, a superset of typingpassLocals.
     locals: Map[VariableIdH, Local],

     nextLocalIdNumber: Int) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/

// mig: fn add_compiler_local (Scala name on Locals — distinct from
// add_typing_pass_local on LocalsBox in Scala. With LocalsBox collapsed, both
// methods live here.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn add_compiler_local(
        &mut self,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        var_id_name_h: &'h IdH<'s, 'h>,
        variability: Variability,
        tyype: CoordH<'s, 'h>,
    ) -> Local<'s, 'h> {
        if self.typing_pass_locals.contains_key(&var_id) {
            panic!("There's already a typingpass local named: {:?}", var_id);
        }
        let new_local_height = self.locals.len() as i32;
        let new_local_id_number = self.next_local_id_number;
        let new_local_id = VariableIdH { number: new_local_id_number, height: new_local_height, name: Some(var_id_name_h) };
        let new_local = Local { id: new_local_id, variability, type_h: tyype };
        self.typing_pass_locals.insert(var_id, new_local_id);
        self.locals.insert(new_local_id, new_local);
        self.next_local_id_number = new_local_id_number + 1;
        new_local
    }
}
/*
  def addCompilerLocal(
    varId2: IVarNameI[cI],
    varIdNameH: IdH,
    variability: Variability,
    tyype: CoordH[KindHT]):
  (Locals, Local) = {
    if (typingPassLocals.contains(varId2)) {
      vfail("There's already a typingpass local named: " + varId2)
    }
    val newLocalHeight = locals.size
    val newLocalIdNumber = nextLocalIdNumber
    val newLocalId = VariableIdH(newLocalIdNumber, newLocalHeight, Some(varIdNameH))
//    // Temporary until catalyst fills in stuff here
//    val keepAlive = newLocalId.name.map(_.readableName).getOrElse("").endsWith("__tether");
    val newLocal = Local(newLocalId, variability, tyype)
    val newLocals =
      Locals(
        typingPassLocals + (varId2 -> newLocalId),
        unstackifiedVars,
        locals + (newLocalId -> newLocal),
        newLocalIdNumber + 1)
    (newLocals, newLocal)
  }
*/

// mig: fn add_hammer_local (collapsed into the &mut self method above)
/*
  def addHammerLocal(
    tyype: CoordH[KindHT],
    variability: Variability):
  (Locals, Local) = {
    val newLocalHeight = locals.size
    val newLocalIdNumber = nextLocalIdNumber
    val newLocalId = VariableIdH(newLocalIdNumber, newLocalHeight, None)
    val newLocal = Local(newLocalId, variability, tyype)
    val newLocals =
      Locals(
        typingPassLocals,
        unstackifiedVars,
        locals + (newLocalId -> newLocal),
        newLocalIdNumber + 1)
    (newLocals, newLocal)
  }
*/

// mig: fn mark_unstackified_by_var_name (collapsed into the &mut self method above)
/*
  def markUnstackified(varId2: IVarNameI[cI]): Locals = {
    markUnstackified(typingPassLocals(varId2))
  }
*/

// mig: fn mark_restackified_by_var_name (collapsed into the &mut self method above)
/*
  def markRestackified(varId2: IVarNameI[cI]): Locals = {
    markRestackified(typingPassLocals(varId2))
  }
*/

// mig: fn mark_unstackified (collapsed into the &mut self method above)
/*
  def markUnstackified(varIdH: VariableIdH): Locals = {
    // Make sure it existed and wasnt already unstackified
    vassert(locals.contains(varIdH))
    if (unstackifiedVars.contains(varIdH)) {
      vfail("Already unstackified " + varIdH)
    }
    Locals(typingPassLocals, unstackifiedVars + varIdH, locals, nextLocalIdNumber)
  }
*/

// mig: fn mark_restackified
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_restackified(&mut self, var_id_h: VariableIdH<'s, 'h>) {
        panic!("Unimplemented: mark_restackified");
    }
}
/*
  def markRestackified(varIdH: VariableIdH): Locals = {
    // Make sure it existed and was unstackified
    vassert(locals.contains(varIdH))
    if (!unstackifiedVars.contains(varIdH)) {
      vfail("Already unstackified " + varIdH)
    }
    Locals(typingPassLocals, unstackifiedVars - varIdH, locals, nextLocalIdNumber)
  }
*/

// mig: fn get_by_var_name (collapsed into the &self method above)
/*
  def get(varId: IVarNameI[cI]): Option[Local] = {
    typingPassLocals.get(varId) match {
      case None => None
      case Some(index) => Some(locals(index))
    }
  }
*/

// mig: fn get (collapsed into the &self method above)
/*
  def get(varId: VariableIdH): Option[Local] = {
    locals.get(varId)
  }
}
*/

// mig: class Hammer
/// Temporary state
//
// Central god struct (typing-pass `Compiler` precedent). Sub-hammer
// fields from Scala (`nameHammer`, `structHammer`, `typeHammer`,
// `functionHammer`, `vonHammer`) NOT held as Rust fields — their methods
// become `impl Hammer { ... }` blocks colocated in per-area files.
pub struct Hammer<'s, 'i, 'h, 'ctx>
where 's: 'i, 's: 'h, 'i: 'h,
{
    pub interner: &'ctx HammerInterner<'s, 'h>,
    pub keywords: &'ctx Keywords<'s>,
    pub scout_arena: &'ctx crate::scout_arena::ScoutArena<'s>,
    pub instantiating_interner: &'ctx crate::instantiating::instantiating_interner::InstantiatingInterner<'s, 'i>,
}
/*
class Hammer(interner: Interner, keywords: Keywords) {
  val nameHammer: NameHammer = new NameHammer()
  val structHammer: StructHammer =
    new StructHammer(
      interner,
      keywords,
      nameHammer,
      (hinputs, hamuts, prototypeI) => typeHammer.translatePrototype(hinputs, hamuts, prototypeI),
      (hinputs, hamuts, referenceI) => typeHammer.translateCoord(hinputs, hamuts, referenceI))
  val typeHammer: TypeHammer = new TypeHammer(interner, keywords, nameHammer, structHammer)
  val functionHammer = new FunctionHammer(keywords, typeHammer, nameHammer, structHammer)
  val vonHammer = new VonHammer(nameHammer, typeHammer)
*/

// mig: fn mangle_func
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_func(&self, id: &IdI<'s, 'i, cI>) -> String {
        panic!("Unimplemented: mangle_func");
    }
}
/*
  def mangleFunc(id: IdI[cI, IFunctionNameI[cI]]): String = {
    if (id.packageCoord.module.str == "rust") {
      ""
    } else {
      val IdI(packageCoord, initSteps, localName) = id
      (localName match {
        case FunctionNameIX(FunctionTemplateNameI(humanName, _), templateArgs, _) => {
          packageCoord.packages.map(_.str + "_").mkString("") +
              initSteps.map(mangleName(_, true)).mkString("") +
              humanName.str +
              (if (templateArgs.nonEmpty) "_" + templateArgs.length else "") +
              templateArgs.map("__" + mangleTemplata(_)).mkString("")
        }
        case ExternFunctionNameI(humanName, templateArgs, _) => {
          packageCoord.packages.map(_.str + "_").mkString("") +
              initSteps.map(mangleName(_, true)).mkString("") +
              humanName.str +
              (if (templateArgs.nonEmpty) "_" + templateArgs.length else "") +
              templateArgs.map("__" + mangleTemplata(_)).mkString("")
        }
        case other => vimpl(other)
      })
    }
  }
*/

// mig: fn mangle_name
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_name(&self, name: &INameI<'s, 'i, cI>, stuff_after: bool) -> String {
        panic!("Unimplemented: mangle_name");
    }
}
/*
  def mangleName(name: INameI[cI], stuffAfter: Boolean): String = {
    ""
//    name match {
//      case StructNameI(StructTemplateNameI(humanName),templateArgs) => {
//        humanName.str +
//        (if (templateArgs.nonEmpty) "_" + templateArgs.length else "") +
//        templateArgs.map("__" + mangleTemplata(_)).mkString("") +
//        (if (stuffAfter) "__" else "")
//      }
//      case other => vimpl(other)
//    }
  }
*/

// mig: fn mangle_struct
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_struct(&self, id: &IdI<'s, 'i, cI>) -> String {
        panic!("Unimplemented: mangle_struct");
    }
}
/*
  def mangleStruct(id: IdI[cI, IStructNameI[cI]]): String = {
    ""
//    val IdI(packageCoord, initSteps, localName) = id
//    mangleName(localName, false)
  }
*/

// mig: fn mangle_kind
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_kind(&self, kind: &KindIT<'s, 'i, cI>) -> String {
        panic!("Unimplemented: mangle_kind");
    }
}
/*
  def mangleKind(kind: KindIT[cI]): String = {
    ""
//    kind match {
//      case IntIT(bits) => "i" + bits
//      case other => vimpl(other)
//    }
  }
*/

// mig: fn mangle_coord
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_coord(&self, coord: &CoordI<'s, 'i, cI>) -> String {
        panic!("Unimplemented: mangle_coord");
    }
}
/*
  def mangleCoord(coord: CoordI[cI]): String = {
    ""
//    val CoordI(ownership, kind) = coord
//    (ownership match {
//      case ImmutableShareI => "1_IRef_"
//      case MutableShareI => ""
//      case OwnI => ""
//      case WeakI => "1_Weak_"
//      case ImmutableBorrowI => "1_IRef_"
//      case MutableBorrowI => "1_Ref_"
//    }) + mangleKind(kind)
  }
*/

// mig: fn mangle_templata
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn mangle_templata(&self, templata: &ITemplataI<'s, 'i, cI>) -> String {
        panic!("Unimplemented: mangle_templata");
    }
}
/*
  def mangleTemplata(templata: ITemplataI[cI]): String = {
    ""
//    templata match {
//      case CoordTemplataI(region, coord) => mangleCoord(coord)
//      case other => vimpl(other)
//    }
  }
*/

// mig: fn translate
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate(&self, hinputs: &HinputsI<'s, 'i>) -> &'h ProgramH<'s, 'h> {
        let HinputsI {
            interfaces,
            structs,
            functions,
            interface_to_edge_blueprints,
            interface_to_sub_citizen_to_edge: edges,
            kind_exports,
            function_exports,
            kind_externs,
            function_externs,
        } = hinputs;

        let mut hamuts = Hamuts {
            human_name_to_full_name_to_id: HashMap::new(),
            struct_t_to_opaque_h: HashMap::new(),
            struct_t_to_struct_h: HashMap::new(),
            struct_t_to_struct_def_h: HashMap::new(),
            struct_defs: Vec::new(),
            static_sized_arrays: HashMap::new(),
            runtime_sized_arrays: HashMap::new(),
            interface_t_to_interface_h: HashMap::new(),
            interface_t_to_interface_def_h: HashMap::new(),
            function_refs: HashMap::new(),
            function_defs: HashMap::new(),
            package_coord_to_export_name_to_function: HashMap::new(),
            package_coord_to_export_name_to_kind: HashMap::new(),
            package_coord_to_prototype_to_extern: HashMap::new(),
            package_coord_to_kind_to_extern: HashMap::new(),
        };

        for _ in kind_exports.iter() {
            panic!("Unimplemented: translate kindExports");
        }

        for FunctionExportI { range: _, prototype, export_id, exported_name } in function_exports.iter() {
            let prototype_h = self.translate_prototype(hinputs, &mut hamuts, prototype);
            hamuts.add_function_export(prototype_h, *export_id.package_coord, *exported_name);
        }

        for _ in kind_externs.iter() {
            panic!("Unimplemented: translate kindExterns");
        }

        for FunctionExternI { prototype, num_inherited_generic_parameters } in function_externs.iter() {
            let num_inherited = *num_inherited_generic_parameters;
            let export_name = if prototype.id.package_coord.module.0 == "rust" {
                panic!("translate functionExterns: rust-package empty-name branch")
            } else {
                match prototype.id.local_name {
                    crate::instantiating::ast::names::INameI::ExternFunction(extern_fn) => extern_fn.human_name,
                    other => panic!("translate functionExterns: unexpected local_name variant {:?}", std::mem::discriminant(&other)),
                }
            };
            let raw_simple_id = crate::simplifying::name_hammer::simplify_id(self.interner, &prototype.id);
            let export_simplified_id = if num_inherited == 0 {
                raw_simple_id
            } else {
                panic!("translate functionExterns: numInherited != 0 branch")
            };
            let prototype_h = self.translate_prototype(hinputs, &mut hamuts, prototype);
            hamuts.add_function_extern(prototype_h, export_simplified_id, export_name);
        }

        self.translate_interfaces(hinputs, &mut hamuts);
        self.translate_structs(hinputs, &mut hamuts);
        let user_functions: Vec<&FunctionDefinitionI> = functions.iter().filter(|f| f.header.is_user_function()).copied().collect();
        let non_user_functions: Vec<&FunctionDefinitionI> = functions.iter().filter(|f| !f.header.is_user_function()).copied().collect();
        self.translate_functions(hinputs, &mut hamuts, &user_functions);
        self.translate_functions(hinputs, &mut hamuts, &non_user_functions);

        panic!("Unimplemented: translate packaging");
    }
}
/*
Guardian: temp-disable: SPDMX — Per documented file-top architecture (hammer.rs lines 1-9): sub-hammers like TypeHammer are NOT held as Rust struct fields under the typing-pass `Compiler` god-struct precedent. Their methods are colocated `impl Hammer` blocks, so `typeHammer.translatePrototype(...)` correctly ports to `self.translate_prototype(...)`. SPDMX Exception Q (god-struct merging) applies. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1327-1780102559874/hook-1327/translate--571.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translate(hinputs: HinputsI): ProgramH = {
    val HinputsI(
    interfaces,
    structs,
    functions,
//    kindToDestructor,
    interfaceToEdgeBlueprints,
    edges,
    kindExports,
    functionExports,
    kindExterns,
    functionExterns) = hinputs


    val hamuts = HamutsBox(Hamuts(Map(), Map(), Map(), Map(), Vector.empty, Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map()))
    //    val emptyPackStructRefH = structHammer.translateStructRef(hinputs, hamuts, emptyPackStructRef)
    //    vassert(emptyPackStructRefH == ProgramH.emptyTupleStructRef)

    kindExports.foreach({ case KindExportI(_, tyype, exportId, exportName) =>
      val kindH = typeHammer.translateKind(hinputs, hamuts, tyype)
      hamuts.addKindExport(kindH, exportId.packageCoord, exportName)
    })

    functionExports.foreach({ case FunctionExportI(_, prototype, exportId, exportName) =>
      val prototypeH = typeHammer.translatePrototype(hinputs, hamuts, prototype)
      hamuts.addFunctionExport(prototypeH, exportId.packageCoord, exportName)
    })

//    kindExterns.foreach({ case KindExternI(tyype, packageCoordinate, exportName) =>
//      val kindH = typeHammer.translateKind(hinputs, hamuts, tyype)
//      hamuts.addKindExtern(kindH, packageCoordinate, exportName)
//    })

    kindExterns.foreach({ case (struct, KindExternI(_)) =>
      val exportName = mangleStruct(struct.id)
      val exportSimplifiedId = NameHammer.simplifyId(struct.id)
      val opaqueH = structHammer.translateOpaqueI(hinputs, hamuts, struct)
      hamuts.addKindExtern(opaqueH, exportSimplifiedId, exportName)
    })

    functionExterns.foreach({ case FunctionExternI(prototype, numInherited) =>
      // Non-rust externs (stdlib's fsqrt etc.) use humanName directly so the user-written
      // `#include "stdlib/fsqrt.h"` and the generated header path agree. Rust externs use
      // "" because ValeRuster reads exportSimplifiedId instead of exportName.
      val exportName =
        if (prototype.id.packageCoord.module.str == "rust") {
          ""
        } else {
          prototype.id.localName match {
            case ExternFunctionNameI(humanName, _, _) => humanName.str
            case other => vwat(other)
          }
        }
      val rawSimpleId = NameHammer.simplifyId(prototype.id)
      val exportSimplifiedId =
        if (numInherited == 0) {
          rawSimpleId
        } else {
          // Per @PRIIROZ, inherited generic params come AFTER own ones in the leaf step's
          // templateArgs (e.g. `zork<N, K, V>` where N is own and K, V are inherited).
          // Move the trailing `numInherited` args off the leaf step onto the immediately
          // preceding (parent citizen) step, producing e.g. `[..., Vec<i32>, capacity]`
          // instead of `[..., Vec, capacity<i32>]`. This is the shape Backend's
          // rustifySimpleId expects per @SMLRZ.
          val steps = rawSimpleId.steps
          val leaf = steps.last
          val parentIdx = steps.length - 2
          val parent = steps(parentIdx)
          val (own, inherited) = leaf.templateArgs.splitAt(leaf.templateArgs.size - numInherited)
          val newParent = SimpleIdStep(parent.name, parent.templateArgs ++ inherited)
          val newLeaf = SimpleIdStep(leaf.name, own)
          SimpleId(steps.updated(parentIdx, newParent).updated(steps.length - 1, newLeaf))
        }
      val prototypeH = typeHammer.translatePrototype(hinputs, hamuts, prototype)
      hamuts.addFunctionExtern(prototypeH, exportSimplifiedId, exportName)
    })

    // We generate the names here first, so that externs get the first chance at having
    // ID 0 for each name, which means we dont need to add _1 _2 etc to the end of them,
    // and they'll match up with the actual outside names.
    //          externNameToExtern.map({ case (externName, prototype2) =>
    //            val fullNameH = NameHammer.translateFullName(hinputs, hamuts, prototype2.fullName)
    //            val humanName =
    //              prototype2.fullName.last match {
    //                case ExternFunctionName2(humanName, _) => humanName
    //                case _ => vfail("Only human-named functions can be extern")
    //              }
    //            if (fullNameH.readableName != humanName) {
    //              vfail("Name conflict, two externs with the same name!")
    //            }
    //            val prototypeH = typeHammer.translatePrototype(hinputs, hamuts, prototype2)
    //            (packageCoordinate -> (externName, prototypeH))
    //          })
    //      })
    //    val externPrototypesH =
    //      packageToExternNameToPrototypeH.values.flatMap(_.values.toVector)

    structHammer.translateInterfaces(hinputs, hamuts);
    structHammer.translateStructs(hinputs, hamuts)
    val userFunctions = functions.filter(f => f.header.isUserFunction).toVector
    val nonUserFunctions = functions.filter(f => !f.header.isUserFunction).toVector
    functionHammer.translateFunctions(hinputs, hamuts, userFunctions)
    functionHammer.translateFunctions(hinputs, hamuts, nonUserFunctions)

//    val immDestructorPrototypesH =
//      kindToDestructor.map({ case (kind, destructor) =>
//        val kindH = typeHammer.translateKind(hinputs, hamuts, kind)
//        val immDestructorPrototypeH = typeHammer.translatePrototype(hinputs, hamuts, destructor)
//        (kindH -> immDestructorPrototypeH)
//      }).toMap
//
//    immDestructorPrototypesH.foreach({ case (kindH, immDestructorPrototypeH) =>
//      vassert(immDestructorPrototypeH.params.head.kind == kindH)
//    })

    val packageToInterfaceDefs = hamuts.interfaceTToInterfaceDefH.groupBy(_._1.id.packageCoord)
    val packageToStructDefs = hamuts.structDefs.groupBy(_.id.packageCoordinate)
    val packageToFunctionDefs = hamuts.functionDefs.groupBy(_._1.id.packageCoord).mapValues(_.values.toVector)
    val packageToStaticSizedArrays = hamuts.staticSizedArrays.values.toVector.groupBy(_.name.packageCoordinate)
    val packageToRuntimeSizedArrays = hamuts.runtimeSizedArrays.values.toVector.groupBy(_.name.packageCoordinate)
//    val packageToImmDestructorPrototypes = immDestructorPrototypesH.groupBy(_._1.packageCoord(interner, keywords))
    val packageToExportNameToKind = hamuts.packageCoordToExportNameToKind
    val packageToExportNameToFunction = hamuts.packageCoordToExportNameToFunction
    val packageToExternNameToKind = hamuts.packageCoordToKindToExtern
    val packageToExternNameToFunction = hamuts.packageCoordToPrototypeToExtern

    val allPackageCoords =
      packageToInterfaceDefs.keySet ++
        packageToStructDefs.keySet ++
        packageToFunctionDefs.keySet ++
        packageToStaticSizedArrays.keySet ++
        packageToRuntimeSizedArrays.keySet ++
//        packageToImmDestructorPrototypes.keySet ++
        packageToExportNameToFunction.keySet ++
        packageToExportNameToKind.keySet ++
        packageToExternNameToFunction.keySet ++
        packageToExternNameToKind.keySet

    val packages = new PackageCoordinateMap[PackageH]()
    allPackageCoords.toVector.foreach(packageCoord => {
      packages.put(
        packageCoord,
        PackageH(
          packageToInterfaceDefs.getOrElse(packageCoord, Map()).values.toVector,
          packageToStructDefs.getOrElse(packageCoord, Vector.empty),
          packageToFunctionDefs.getOrElse(packageCoord, Vector.empty),
          packageToStaticSizedArrays.getOrElse(packageCoord, Vector.empty),
          packageToRuntimeSizedArrays.getOrElse(packageCoord, Vector.empty),
//          packageToImmDestructorPrototypes.getOrElse(packageCoord, Map()),
          packageToExportNameToFunction.getOrElse(packageCoord, Map()),
          packageToExportNameToKind.getOrElse(packageCoord, Map()),
          packageToExternNameToFunction.getOrElse(packageCoord, Map()),
          packageToExternNameToKind.getOrElse(packageCoord, Map())))
    })

    finalast.ProgramH(packages)
  }
}
*/

// mig: fn flatten_and_filter_voids
pub fn flatten_and_filter_voids<'s, 'h>(
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> Vec<ExpressionH<'s, 'h>>
where 's: 'h,
{
    panic!("Unimplemented: flatten_and_filter_voids");
}
/*
object Hammer {

  private def flattenAndFilterVoids(unfilteredExprsHE: Vector[ExpressionH[KindHT]]): Vector[ExpressionH[KindHT]] = {
    val flattenedExprsHE =
      unfilteredExprsHE.flatMap({
        case ConsecutorH(innersHE) => innersHE
        case other => Vector(other)
      })
    flattenedExprsHE.init.foreach(exprHE => {
      exprHE.resultType.kind match {
        case NeverHT(_) => vwat()
        case _ =>
      }
    })

    // Filter out any Void that arent the last.
    val filteredFlattenedExprsHE =
      if (flattenedExprsHE.size <= 1) {
        flattenedExprsHE
      } else {
        flattenedExprsHE.init
          .filter({ case ConstantVoidH() => false case _ => true }) :+
          flattenedExprsHE.last
      }
    vassert(filteredFlattenedExprsHE.nonEmpty)
    filteredFlattenedExprsHE
  }
*/

// mig: fn consecutive
pub fn consecutive<'s, 'h>(
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> ExpressionH<'s, 'h>
where 's: 'h,
{
    panic!("Unimplemented: consecutive");
}
/*
  def consecutive(unfilteredExprsHE: Vector[ExpressionH[KindHT]]): ExpressionH[KindHT] = {
    val filteredFlattenedExprsHE = flattenAndFilterVoids(unfilteredExprsHE)

    filteredFlattenedExprsHE match {
      case Vector() => vwat("Cant have empty consecutive")
      case Vector(only) => only
      case multiple => ConsecutorH(multiple)
    }
  }
*/

// mig: fn consecrash
pub fn consecrash<'s, 'i, 'h>(
    locals: &mut Locals<'s, 'i, 'h>,
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> ExpressionH<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: consecrash");
}
/*
  // Like consecutive() but for expressions that were meant to go somewhere
  // but then the last one crashes.
  // We store them into locals really just so ConsecutorH doesn't complain
  // about some pre-last statements not producing voids.
  // See BRCOBS.
  def consecrash(
    locals: LocalsBox,
    unfilteredExprsHE: Vector[ExpressionH[KindHT]]):
  ExpressionH[KindHT] = {
    unfilteredExprsHE.last.resultType.kind match {
      case NeverHT(_) =>
      case _ => vwat()
    }

    val exprsHE = flattenAndFilterVoids(unfilteredExprsHE)

    // Make temporaries for all the previous things if we end in a never
    val exprsWithStackifiedInitHE =
      exprsHE.init
        .map(expr => {
          if (expr.resultType.kind == VoidHT()) {
            // Dont need a temporary if it's void, we can just drop it.
            expr
          } else {
            val local = locals.addHammerLocal(expr.resultType, Final)
            StackifyH(expr, local, None)
          }
        }) :+
        exprsHE.last
    // We'll never need to unstackify them because we're about to crash.

    exprsWithStackifiedInitHE match {
      case Vector() => vwat("Cant have empty consecutive")
      case Vector(only) => return only
      case multiple => ConsecutorH(multiple)
    }
  }
}
*/
