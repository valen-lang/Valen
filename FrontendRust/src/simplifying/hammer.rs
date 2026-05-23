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
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::IVarNameI;
use crate::instantiating::ast::types::cI;
use crate::keywords::Keywords;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::simplifying::hamuts::Hamuts;
use std::collections::{HashMap, HashSet};

/*
package dev.vale.simplifying

import dev.vale.{Builtins, FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Profiler, Result, finalast, vassert, vcurious, vfail, vwat}
import dev.vale.finalast.{ConsecutorH, ConstantVoidH, CoordH, ExpressionH, Final, IdH, KindHT, Local, NeverHT, PackageH, ProgramH, PrototypeH, StackifyH, Variability, VariableIdH, VoidHT}
import dev.vale.highertyping.ICompileErrorA
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.List

case class FunctionRefH(prototype: PrototypeH) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
  //  def functionType = prototype.functionType
  def fullName = prototype.id
}

case class LocalsBox(var inner: Locals) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable

  def snapshot = inner

  def typingPassLocals: Map[IVarNameI[cI], VariableIdH] = inner.typingPassLocals
  def unstackifiedVars: Set[VariableIdH] = inner.unstackifiedVars
  def locals: Map[VariableIdH, Local] = inner.locals
  def nextLocalIdNumber: Int = inner.nextLocalIdNumber

  def get(id: IVarNameI[cI]) = inner.get(id)
  def get(id: VariableIdH) = inner.get(id)

  def markUnstackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markUnstackified(varId2)
  }
  def markRestackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markRestackified(varId2)
  }

  def markUnstackified(varIdH: VariableIdH): Unit = {
    inner = inner.markUnstackified(varIdH)
  }
  def setNextLocalIdNumber(nextLocalIdNumber: Int): Unit = {
    inner = inner.copy(nextLocalIdNumber = nextLocalIdNumber)
  }

  def addHammerLocal(
    tyype: CoordH[KindHT],
    variability: Variability):
  Local = {
    val (newInner, local) = inner.addHammerLocal(tyype, variability)
    inner = newInner
    local
  }

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

  def markUnstackified(varId2: IVarNameI[cI]): Locals = {
    markUnstackified(typingPassLocals(varId2))
  }

  def markRestackified(varId2: IVarNameI[cI]): Locals = {
    markRestackified(typingPassLocals(varId2))
  }

  def markUnstackified(varIdH: VariableIdH): Locals = {
    // Make sure it existed and wasnt already unstackified
    vassert(locals.contains(varIdH))
    if (unstackifiedVars.contains(varIdH)) {
      vfail("Already unstackified " + varIdH)
    }
    Locals(typingPassLocals, unstackifiedVars + varIdH, locals, nextLocalIdNumber)
  }

  def markRestackified(varIdH: VariableIdH): Locals = {
    // Make sure it existed and was unstackified
    vassert(locals.contains(varIdH))
    if (!unstackifiedVars.contains(varIdH)) {
      vfail("Already unstackified " + varIdH)
    }
    Locals(typingPassLocals, unstackifiedVars - varIdH, locals, nextLocalIdNumber)
  }

  def get(varId: IVarNameI[cI]): Option[Local] = {
    typingPassLocals.get(varId) match {
      case None => None
      case Some(index) => Some(locals(index))
    }
  }

  def get(varId: VariableIdH): Option[Local] = {
    locals.get(varId)
  }
}

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
    functionExterns) = hinputs


    val hamuts = HamutsBox(Hamuts(Map(), Map(), Map(), Vector.empty, Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map(), Map()))
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

    functionExterns.foreach({ case FunctionExternI(prototype, exportName) =>
      val prototypeH = typeHammer.translatePrototype(hinputs, hamuts, prototype)
      hamuts.addFunctionExtern(prototypeH, exportName)
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
    val packageToExternNameToKind = hamuts.packageCoordToExternNameToKind
    val packageToExternNameToFunction = hamuts.packageCoordToExternNameToFunction

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

  def consecutive(unfilteredExprsHE: Vector[ExpressionH[KindHT]]): ExpressionH[KindHT] = {
    val filteredFlattenedExprsHE = flattenAndFilterVoids(unfilteredExprsHE)

    filteredFlattenedExprsHE match {
      case Vector() => vwat("Cant have empty consecutive")
      case Vector(only) => only
      case multiple => ConsecutorH(multiple)
    }
  }

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

// mig: case class FunctionRefH (moved to src/final_ast/ast.rs — Scala has it
// in `Hammer.scala`, but it's a pure data type that colocates with the other
// finalast H-side AST. Field shape `{ prototype: &'h PrototypeH<...> }` per Scala.)

// mig: case class LocalsBox (collapsed into Locals; see architect directive)
// Scala's LocalsBox was a mutable wrapper around an immutable Locals. Per
// architect directive (matching typing-pass `CompilerOutputs` precedent and
// the `HamutsBox` → `Hamuts` collapse), the Rust port has a single mutable
// `Locals` accumulator. The LocalsBox methods become &mut self methods on
// Locals.

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

// mig: fn snapshot
// (LocalsBox.snapshot returned `inner`; with the LocalsBox/Locals collapse,
// snapshot becomes a clone-equivalent. Not yet exposed — body migration.)

// mig: fn typing_pass_locals
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn typing_pass_locals(&self) -> &HashMap<&'i IVarNameI<'s, 'i, cI>, VariableIdH<'s, 'h>> {
        panic!("Unimplemented: typing_pass_locals");
    }
}

// mig: fn unstackified_vars
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn unstackified_vars(&self) -> &HashSet<VariableIdH<'s, 'h>> {
        panic!("Unimplemented: unstackified_vars");
    }
}

// mig: fn locals
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn locals(&self) -> &HashMap<VariableIdH<'s, 'h>, Local<'s, 'h>> {
        panic!("Unimplemented: locals");
    }
}

// mig: fn next_local_id_number
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn next_local_id_number(&self) -> i32 {
        panic!("Unimplemented: next_local_id_number");
    }
}

// mig: fn get_by_var_name (Scala overload `get(IVarNameI[cI])` —
// disambiguated per instantiating overload-suffix pattern.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn get_by_var_name(&self, id: &'i IVarNameI<'s, 'i, cI>) -> Option<Local<'s, 'h>> {
        panic!("Unimplemented: get_by_var_name");
    }
}

// mig: fn get (Scala overload `get(VariableIdH)`.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn get(&self, id: VariableIdH<'s, 'h>) -> Option<Local<'s, 'h>> {
        panic!("Unimplemented: get");
    }
}

// mig: fn mark_unstackified_by_var_name (Scala overload disambiguated.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_unstackified_by_var_name(&mut self, var_id: &'i IVarNameI<'s, 'i, cI>) {
        panic!("Unimplemented: mark_unstackified_by_var_name");
    }
}

// mig: fn mark_restackified_by_var_name (Scala overload disambiguated.)
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_restackified_by_var_name(&mut self, var_id: &'i IVarNameI<'s, 'i, cI>) {
        panic!("Unimplemented: mark_restackified_by_var_name");
    }
}

// mig: fn mark_unstackified
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_unstackified(&mut self, var_id_h: VariableIdH<'s, 'h>) {
        panic!("Unimplemented: mark_unstackified");
    }
}

// mig: fn mark_restackified
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn mark_restackified(&mut self, var_id_h: VariableIdH<'s, 'h>) {
        panic!("Unimplemented: mark_restackified");
    }
}

// mig: fn set_next_local_id_number
impl<'s, 'i, 'h> Locals<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub fn set_next_local_id_number(&mut self, next_local_id_number: i32) {
        panic!("Unimplemented: set_next_local_id_number");
    }
}

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
        panic!("Unimplemented: add_typing_pass_local");
    }
}

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
        panic!("Unimplemented: add_compiler_local");
    }
}

// mig: class Hammer
/// Temporary state
//
// Central god struct (typing-pass `Compiler` precedent). Sub-hammer
// fields from Scala (`nameHammer`, `structHammer`, `typeHammer`,
// `functionHammer`, `vonHammer`) NOT held as Rust fields — their methods
// become `impl Hammer { ... }` blocks colocated in per-area files.
pub struct Hammer<'s, 'h, 'ctx>
where 's: 'h,
{
    pub interner: &'ctx HammerInterner<'s, 'h>,
    pub keywords: &'ctx Keywords<'s>,
}

// mig: fn translate
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate(&self, hinputs: &HinputsI<'s, 'i>) -> &'h ProgramH<'s, 'h> {
        panic!("Unimplemented: translate");
    }
}

// mig: fn flatten_and_filter_voids
pub fn flatten_and_filter_voids<'s, 'h>(
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> Vec<ExpressionH<'s, 'h>>
where 's: 'h,
{
    panic!("Unimplemented: flatten_and_filter_voids");
}

// mig: fn consecutive
pub fn consecutive<'s, 'h>(
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> ExpressionH<'s, 'h>
where 's: 'h,
{
    panic!("Unimplemented: consecutive");
}

// mig: fn consecrash
pub fn consecrash<'s, 'i, 'h>(
    locals: &mut Locals<'s, 'i, 'h>,
    unfiltered_exprs_h: &[ExpressionH<'s, 'h>],
) -> ExpressionH<'s, 'h>
where 's: 'i, 'i: 'h,
{
    panic!("Unimplemented: consecrash");
}
