// From Frontend/SimplifyingPass/src/dev/vale/simplifying/Hammer.scala
/*
package dev.vale.simplifying

import dev.vale.{Builtins, FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate, PackageCoordinateMap, Profiler, Result, finalast, vassert, vcurious, vfail, vwat}
import dev.vale.finalast.{ConsecutorH, ConstantVoidH, CoordH, ExpressionH, Final, IdH, KindHT, Local, NeverHT, PackageH, ProgramH, PrototypeH, StackifyH, Variability, VariableIdH, VoidHT}
import dev.vale.highertyping.ICompileErrorA
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.List
*/
// mig: struct FunctionRefH
pub struct FunctionRefH<'h> {
    prototype: PrototypeH<'h>,
    _must_intern: MustIntern,
}
// mig: impl FunctionRefH
/*
case class FunctionRefH(prototype: PrototypeH) {
  val hash = runtime.ScalaRunTime._hashCode(this)
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionRefH` below.)
/*
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionRefH` below.)
/*
override def equals(obj: Any): Boolean = vcurious();
  //  def functionType = prototype.functionType
*/
// mig: fn full_name
impl<'h> FunctionRefH<'h> {
    pub fn full_name(prototype: PrototypeH) -> IdH {
        panic!("Unimplemented: full_name");
    }
}
/*
  def fullName = prototype.id
}

*/
// mig: struct LocalsBoxH
pub struct LocalsBoxH<'h> {
    inner: LocalsH<'h>,
}
// mig: impl LocalsBoxH
/*
case class LocalsBox(var inner: Locals) {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LocalsBoxH` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocalsBoxH` below.)
/*
override def hashCode(): Int = vfail() // Shouldnt hash, is mutable
*/
// mig: fn snapshot
impl<'h> LocalsBoxH<'h> {
    pub fn snapshot(&self) -> LocalsH {
        panic!("Unimplemented: snapshot");
    }
}
/*
  def snapshot = inner
*/
// mig: fn typing_pass_locals
impl<'h> LocalsBoxH<'h> {
    pub fn typing_pass_locals(&self) -> ArenaIndexMap<IVarNameI, VariableIdH> {
        panic!("Unimplemented: typing_pass_locals");
    }
}
/*
  def typingPassLocals: Map[IVarNameI[cI], VariableIdH] = inner.typingPassLocals
*/
// mig: fn unstackified_vars
impl<'h> LocalsBoxH<'h> {
    pub fn unstackified_vars(&self) -> std::collections::HashSet<VariableIdH> {
        panic!("Unimplemented: unstackified_vars");
    }
}
/*
  def unstackifiedVars: Set[VariableIdH] = inner.unstackifiedVars
*/
// mig: fn locals
impl<'h> LocalsBoxH<'h> {
    pub fn locals(&self) -> ArenaIndexMap<VariableIdH, Local> {
        panic!("Unimplemented: locals");
    }
}
/*
  def locals: Map[VariableIdH, Local] = inner.locals
*/
// mig: fn next_local_id_number
impl<'h> LocalsBoxH<'h> {
    pub fn next_local_id_number(&self) -> i32 {
        panic!("Unimplemented: next_local_id_number");
    }
}
/*
  def nextLocalIdNumber: Int = inner.nextLocalIdNumber
*/
// mig: fn get
impl<'h> LocalsBoxH<'h> {
    pub fn get(&self, id: IVarNameI) -> Option<Local> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(id: IVarNameI[cI]) = inner.get(id)
*/
// mig: fn get
impl<'h> LocalsBoxH<'h> {
    pub fn get(&self, id: VariableIdH) -> Option<Local> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(id: VariableIdH) = inner.get(id)
*/
// mig: fn mark_unstackified
impl<'h> LocalsBoxH<'h> {
    pub fn mark_unstackified(&mut self, var_id: IVarNameI) {
        panic!("Unimplemented: mark_unstackified");
    }
}
/*
  def markUnstackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markUnstackified(varId2)
  }
*/
// mig: fn mark_restackified
impl<'h> LocalsBoxH<'h> {
    pub fn mark_restackified(&mut self, var_id: IVarNameI) {
        panic!("Unimplemented: mark_restackified");
    }
}
/*
  def markRestackified(varId2: IVarNameI[cI]): Unit = {
    inner = inner.markRestackified(varId2)
  }
*/
// mig: fn mark_unstackified
impl<'h> LocalsBoxH<'h> {
    pub fn mark_unstackified(&mut self, var_id_h: VariableIdH) {
        panic!("Unimplemented: mark_unstackified");
    }
}
/*
  def markUnstackified(varIdH: VariableIdH): Unit = {
    inner = inner.markUnstackified(varIdH)
  }
*/
// mig: fn set_next_local_id_number
impl<'h> LocalsBoxH<'h> {
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
impl<'h> LocalsBoxH<'h> {
    pub fn add_hammer_local(&mut self, tyype: CoordH<KindHT>, variability: Variability) -> Local {
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
// mig: fn add_typing_pass_local
impl<'h> LocalsBoxH<'h> {
    pub fn add_typing_pass_local(&mut self, var_id: IVarNameI, var_id_name_h: IdH, variability: Variability, tyype: CoordH<KindHT>) -> Local {
        panic!("Unimplemented: add_typing_pass_local");
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

// This represents the locals for the entire function.
// Note, some locals will have the same index, that just means they're in
// different blocks.
*/
// mig: struct LocalsH
pub struct LocalsH<'h> {
    typing_pass_locals: ArenaIndexMap<'h, IVarNameI<'h>, VariableIdH>,
    unstackified_vars: std::collections::HashSet<VariableIdH>,
    locals: ArenaIndexMap<'h, VariableIdH, Local<'h>>,
    next_local_id_number: i32,
    _must_intern: MustIntern,
}
// mig: impl LocalsH
/*
case class Locals(
     // This doesn't have all the locals that are in the locals list, this just
     // has any locals added by typingpass.
     typingPassLocals: Map[IVarNameI[cI], VariableIdH],

     unstackifiedVars: Set[VariableIdH],

     // This has all the locals for the function, a superset of typingpassLocals.
     locals: Map[VariableIdH, Local],

     nextLocalIdNumber: Int) {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for LocalsH` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocalsH` below.)
/*
override def hashCode(): Int = vcurious()
*/
// mig: fn add_compiler_local
impl<'h> LocalsH<'h> {
    pub fn add_compiler_local(&self, interner: &'ctx HammerInterner<'h>, var_id: IVarNameI<'h>, var_id_name_h: IdH<'h>, variability: Variability, tyype: CoordH<KindHT>) -> (LocalsH<'h>, Local<'h>) {
        // Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass
        // arena-allocates where Scala used GC.
        panic!("Unimplemented: add_compiler_local");
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
// mig: fn add_hammer_local
impl<'h> LocalsH<'h> {
    pub fn add_hammer_local(&self, interner: &'ctx HammerInterner<'h>, tyype: CoordH<KindHT>, variability: Variability) -> (LocalsH<'h>, Local<'h>) {
        // Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass
        // arena-allocates where Scala used GC.
        panic!("Unimplemented: add_hammer_local");
    }
}
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
// mig: fn mark_unstackified
impl<'h> LocalsH<'h> {
    pub fn mark_unstackified(&self, var_id: IVarNameI) -> LocalsH {
        panic!("Unimplemented: mark_unstackified");
    }
}
/*
  def markUnstackified(varId2: IVarNameI[cI]): Locals = {
    markUnstackified(typingPassLocals(varId2))
  }
*/
// mig: fn mark_restackified
impl<'h> LocalsH<'h> {
    pub fn mark_restackified(&self, var_id: IVarNameI) -> LocalsH {
        panic!("Unimplemented: mark_restackified");
    }
}
/*
  def markRestackified(varId2: IVarNameI[cI]): Locals = {
    markRestackified(typingPassLocals(varId2))
  }
*/
// mig: fn mark_unstackified
impl<'h> LocalsH<'h> {
    pub fn mark_unstackified(&self, var_id_h: VariableIdH) -> LocalsH {
        panic!("Unimplemented: mark_unstackified");
    }
}
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
impl<'h> LocalsH<'h> {
    pub fn mark_restackified(&self, var_id_h: VariableIdH) -> LocalsH {
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
// mig: fn get
impl<'h> LocalsH<'h> {
    pub fn get(&self, var_id: IVarNameI) -> Option<Local> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(varId: IVarNameI[cI]): Option[Local] = {
    typingPassLocals.get(varId) match {
      case None => None
      case Some(index) => Some(locals(index))
    }
  }
*/
// mig: fn get
impl<'h> LocalsH<'h> {
    pub fn get(&self, var_id: VariableIdH) -> Option<Local> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(varId: VariableIdH): Option[Local] = {
    locals.get(varId)
  }
}
*/
// mig: struct HammerH
pub struct HammerH<'h> {
    interner: &'h HammerInterner<'h>,
    keywords: Keywords<'h>,
    _must_intern: MustIntern,
}
// mig: impl HammerH
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
// mig: fn translate
impl<'h> HammerH<'h> {
    pub fn translate(&self, interner: &'ctx HammerInterner<'h>, hinputs: HinputsI) -> ProgramH<'h> {
        // Rust adaptation (SPDMX-B): interner threaded explicitly because the Rust pass
        // arena-allocates where Scala used GC.
        panic!("Unimplemented: translate");
    }
}
/*
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
*/
// mig: fn flatten_and_filter_voids
pub fn flatten_and_filter_voids(unfiltered_exprs_h: &[ExpressionH]) -> Vec<ExpressionH> {
    panic!("Unimplemented: flatten_and_filter_voids");
}
/*
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
pub fn consecutive(unfiltered_exprs_h: &[ExpressionH]) -> ExpressionH {
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
pub fn consecrash(locals_box: &mut LocalsBoxH, unfiltered_exprs_h: &[ExpressionH]) -> ExpressionH {
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
