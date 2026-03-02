use crate::compile_options::GlobalOptions;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::postparsing::ScoutCompilation;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;

// From HigherTypingPass.scala lines 793-836: HigherTypingCompilation class
pub struct HigherTypingCompilation<'a, 'ctx, 'p> {
  scout_compilation: ScoutCompilation<'a, 'ctx, 'p>,
  #[allow(dead_code)]
  astrouts_cache: Option<()>, // PackageCoordinateMap[ProgramA] not yet ported
}

impl<'a, 'ctx, 'p> HigherTypingCompilation<'a, 'ctx, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  // From HigherTypingPass.scala lines 793-799
  pub fn new(
    interner: &'ctx Interner<'a>,
    keywords: &'ctx Keywords<'a>,
    packages_to_build: Vec<&'a PackageCoordinate<'a>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'a, HashMap<String, String>>,
    global_options: GlobalOptions,
    arena: &'p bumpalo::Bump,
  ) -> Self {
    let scout_compilation = ScoutCompilation::new(
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      global_options,
      arena,
    );

    HigherTypingCompilation {
      scout_compilation,
      astrouts_cache: None,
    }
  }

  // From HigherTypingPass.scala line 802: getCodeMap
  pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
    self.scout_compilation.get_code_map()
  }

  // From HigherTypingPass.scala line 803: getParseds
  pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'a, (FileP<'a, 'p>, Vec<RangeL>)>, FailedParse<'a>> {
    self.scout_compilation.get_parseds()
  }

  // From HigherTypingPass.scala line 804: getVpstMap
  pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
    self.scout_compilation.get_vpst_map()
  }

  // From HigherTypingPass.scala line 805: getScoutput
  pub fn get_scoutput(&mut self) -> Result<(), String> {
    panic!("HigherTypingCompilation.get_scoutput not yet implemented - see HigherTypingPass.scala line 805")
  }

  // From HigherTypingPass.scala lines 807-820: getAstrouts
  pub fn get_astrouts(&mut self) -> Result<(), String> {
    panic!("HigherTypingCompilation.get_astrouts not yet implemented - see HigherTypingPass.scala lines 807-820")
  }

  // From HigherTypingPass.scala lines 821-835: expectAstrouts
  pub fn expect_astrouts(&mut self) -> () {
    panic!("HigherTypingCompilation.expect_astrouts not yet implemented - see HigherTypingPass.scala lines 821-835")
  }
}
/*
package dev.vale.highertyping

import scala.sys.process.Process
import dev.vale
import dev.vale.highertyping.HigherTypingPass.explicifyLookups
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FileP, OwnP}
import dev.vale.postparsing.rules.{IRulexSR, RuleScout}
import dev.vale.postparsing._
import dev.vale.postparsing.RuneTypeSolver
import dev.vale.postparsing.rules._
import dev.vale.solver.RuleError

import scala.collection.immutable.List
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

*/
// mig: struct Astrouts
pub struct Astrouts<'a> {
  code_location_to_maybe_type: std::collections::HashMap<CodeLocationS, Option<ITemplataType>>,
  code_location_to_struct: std::collections::HashMap<CodeLocationS, StructA<'a>>,
  code_location_to_interface: std::collections::HashMap<CodeLocationS, InterfaceA<'a>>,
}

// mig: impl Astrouts
impl<'a> Astrouts<'a> {
}
/*
case class Astrouts(
  codeLocationToMaybeType: mutable.HashMap[CodeLocationS, Option[ITemplataType]],
  codeLocationToStruct: mutable.HashMap[CodeLocationS, StructA],
  codeLocationToInterface: mutable.HashMap[CodeLocationS, InterfaceA])

*/
// mig: struct EnvironmentA
pub struct EnvironmentA<'a> {
  maybe_name: Option<&'a INameS<'a>>,
  maybe_parent_env: Option<&'a EnvironmentA<'a>>,
  code_map: PackageCoordinateMap<'a, ProgramS<'a>>,
  rune_to_type: std::collections::HashMap<&'a IRuneS<'a>, ITemplataType>,
}

// mig: impl EnvironmentA
impl<'a> EnvironmentA<'a> {
}
/*
// Environments dont have an AbsoluteName, because an environment can span multiple
// files.
case class EnvironmentA(
    maybeName: Option[INameS],
    maybeParentEnv: Option[EnvironmentA],
    codeMap: PackageCoordinateMap[ProgramS],
    runeToType: Map[IRuneS, ITemplataType]) {
*/
// mig: fn equals
fn equals(&self, obj: &dyn std::any::Any) -> bool {
  panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
  panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

  val structsS: Vector[StructS] = codeMap.packageCoordToContents.values.flatMap(_.structs).toVector
  val interfacesS: Vector[InterfaceS] = codeMap.packageCoordToContents.values.flatMap(_.interfaces).toVector
  val implsS: Vector[ImplS] = codeMap.packageCoordToContents.values.flatMap(_.impls).toVector
  val functionsS: Vector[FunctionS] = codeMap.packageCoordToContents.values.flatMap(_.implementedFunctions).toVector
  val exportsS: Vector[ExportAsS] = codeMap.packageCoordToContents.values.flatMap(_.exports).toVector
  val imports: Vector[ImportS] = codeMap.packageCoordToContents.values.flatMap(_.imports).toVector

*/
// mig: fn add_runes
fn add_runes(&self, new_rune_to_type: std::collections::HashMap<&'a IRuneS<'a>, ITemplataType>) -> EnvironmentA<'a> {
  panic!("Unimplemented: add_runes");
}
/*
  def addRunes(newruneToType: Map[IRuneS, ITemplataType]): EnvironmentA = {
    EnvironmentA(maybeName, maybeParentEnv, codeMap, runeToType ++ newruneToType)
  }
}

object HigherTypingPass {
*/
// mig: fn explicify_lookups
fn explicify_lookups(env: &dyn IRuneTypeSolverEnv, rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, rule_builder: &mut Vec<IRulexSR>, all_rules_with_implicitly_coercing_lookups_s: Vec<IRulexSR>) -> Result<(), IRuneTypingLookupFailedError> {
  panic!("Unimplemented: explicify_lookups");
}
/*
  def explicifyLookups(
    // We take in this instead of an EnvironmentA because the typing pass calls this method too.
    env: IRuneTypeSolverEnv,
    runeAToType: mutable.HashMap[IRuneS, ITemplataType],
    ruleBuilder: ArrayBuffer[IRulexSR],
    allRulesWithImplicitlyCoercingLookupsS: Vector[IRulexSR]):
  Result[Unit, IRuneTypingLookupFailedError] = {
    // Only two rules' results can be coerced: LookupSR and CallSR.
    // Let's look for those and rewrite them to put an explicit coercion in there.
    allRulesWithImplicitlyCoercingLookupsS.foreach({
      case rule @ MaybeCoercingCallSR(range, resultRune, templateRune, args) => {
        val expectedType = vassertSome(runeAToType.get(resultRune.rune))
        val actualType =
          vassertSome(runeAToType.get(templateRune.rune)) match {
            case TemplateTemplataType(_, returnType) => returnType
            case _ => vwat()
          }
        (actualType, expectedType) match {
          case (x, y) if x == y => {
            ruleBuilder += CallSR(range, resultRune, templateRune, args)
          }
          case (KindTemplataType(), CoordTemplataType()) => {
            val kindRune = RuneUsage(range, ImplicitCoercionKindRuneS(range, resultRune.rune))
            runeAToType.put(kindRune.rune, KindTemplataType())
            ruleBuilder += CallSR(range, kindRune, templateRune, args)
            ruleBuilder += CoerceToCoordSR(range, resultRune, kindRune)
          }
          case _ => vimpl()
        }
      }
      case rule@MaybeCoercingLookupSR(range, resultRune, name) => {
        val desiredType = vassertSome(runeAToType.get(resultRune.rune))
        val actualLookupResult =
          env.lookup(range, name) match {
            case Err(e) => return Err(e)
            case Ok(x) => x
          }

        actualLookupResult match {
          case PrimitiveRuneTypeSolverLookupResult(tyype) => {
            desiredType match {
              case CoordTemplataType() => {
                coerceKindLookupToCoord(
                  runeAToType, ruleBuilder, range, resultRune, name)
              }
              case KindTemplataType() => {
                ruleBuilder += LookupSR(range, resultRune, name)
              }
              case TemplateTemplataType(paramTypes, returnType) => {
                vassert(paramTypes.nonEmpty) // impl, if it's empty we might need to do some coercing.
                ruleBuilder += LookupSR(range, resultRune, name)
              }
              case _ => return Err(FoundPrimitiveDidntMatchExpectedType(List(range), desiredType, tyype))
            }
          }
          case CitizenRuneTypeSolverLookupResult(tyype, genericParams) => {
            desiredType match {
              case KindTemplataType() => {
                coerceKindTemplateLookupToKind(
                  runeAToType, ruleBuilder, range, resultRune, name, tyype)
              }
              case CoordTemplataType() => {
                coerceKindTemplateLookupToCoord(
                  runeAToType, ruleBuilder, range, resultRune, name, tyype)
              }
              case TemplateTemplataType(paramTypes, returnType) => {
                vassert(paramTypes.nonEmpty) // impl, if it's empty we might need to do some coercing.
                ruleBuilder += LookupSR(range, resultRune, name)
              }
              case _ => return Err(FoundTemplataDidntMatchExpectedTypeA(List(range), desiredType, tyype))
            }
          }
          case TemplataLookupResult(actualType) => {
            (actualType, desiredType) match {
              case (x, y) if x == y => {
                ruleBuilder += LookupSR(range, resultRune, name)
              }
              case (KindTemplataType(), CoordTemplataType()) => {
                coerceKindLookupToCoord(
                  runeAToType, ruleBuilder, range, resultRune, name)
              }
              case (actualTemplateType @ TemplateTemplataType(_, KindTemplataType()), KindTemplataType()) => {
                coerceKindTemplateLookupToKind(
                  runeAToType, ruleBuilder, range, resultRune, name, actualTemplateType)
              }
              case (actualTemplateType @ TemplateTemplataType(_, KindTemplataType()), CoordTemplataType()) => {
                coerceKindTemplateLookupToCoord(
                  runeAToType, ruleBuilder, range, resultRune, name, actualTemplateType)
              }
              case _ => {
                throw CompileErrorExceptionA(
                  highertyping.RangedInternalErrorA(
                    range, "Unexpected coercion from " + actualType + " to " + desiredType))
              }
            }
          }
        }
      }
      case rule => {
        ruleBuilder += rule
      }
    })
    Ok(())
  }

*/
// mig: fn coerce_kind_lookup_to_coord
fn coerce_kind_lookup_to_coord(rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, rule_builder: &mut Vec<IRulexSR>, range: RangeS, result_rune: RuneUsage, name: &IImpreciseNameS) {
  panic!("Unimplemented: coerce_kind_lookup_to_coord");
}
/*
  private def coerceKindLookupToCoord(
    runeAToType: mutable.HashMap[IRuneS, ITemplataType],
    ruleBuilder: ArrayBuffer[IRulexSR],
    range: RangeS,
    resultRune: RuneUsage,
    name: IImpreciseNameS
  ) = {
    val kindRune = RuneUsage(range, ImplicitCoercionKindRuneS(range, resultRune.rune))
    runeAToType.put(kindRune.rune, KindTemplataType())
    ruleBuilder += LookupSR(range, kindRune, name)
    ruleBuilder += CoerceToCoordSR(range, resultRune, kindRune)
  }

*/
// mig: fn coerce_kind_template_lookup_to_kind
fn coerce_kind_template_lookup_to_kind(rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, rule_builder: &mut Vec<IRulexSR>, range: RangeS, result_rune: RuneUsage, name: &IImpreciseNameS, actual_template_type: TemplateTemplataType) {
  panic!("Unimplemented: coerce_kind_template_lookup_to_kind");
}
/*
  private def coerceKindTemplateLookupToKind(
    runeAToType: mutable.HashMap[IRuneS, ITemplataType],
    ruleBuilder: ArrayBuffer[IRulexSR],
    range: RangeS,
    resultRune: RuneUsage,
    name: IImpreciseNameS,
    actualTemplateType: TemplateTemplataType):
  Unit = {
    val templateRune = RuneUsage(range, ImplicitCoercionTemplateRuneS(range, resultRune.rune))
    runeAToType.put(templateRune.rune, actualTemplateType)
    ruleBuilder += LookupSR(range, templateRune, name)
    ruleBuilder += CallSR(range, resultRune, templateRune, Vector())
  }

*/
// mig: fn coerce_kind_template_lookup_to_coord
fn coerce_kind_template_lookup_to_coord(rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, rule_builder: &mut Vec<IRulexSR>, range: RangeS, result_rune: RuneUsage, name: &IImpreciseNameS, ttt: TemplateTemplataType) {
  panic!("Unimplemented: coerce_kind_template_lookup_to_coord");
}
/*
  private def coerceKindTemplateLookupToCoord(
    runeAToType: mutable.HashMap[IRuneS, ITemplataType],
    ruleBuilder: ArrayBuffer[IRulexSR],
    range: RangeS,
    resultRune: RuneUsage,
    name: IImpreciseNameS,
    ttt: TemplateTemplataType):
  Unit = {
    val templateRune = RuneUsage(range, ImplicitCoercionTemplateRuneS(range, resultRune.rune))
    val kindRune = RuneUsage(range, ImplicitCoercionKindRuneS(range, resultRune.rune))
    runeAToType.put(templateRune.rune, ttt)
    runeAToType.put(kindRune.rune, KindTemplataType())
    ruleBuilder += LookupSR(range, templateRune, name)
    ruleBuilder += CallSR(range, kindRune, templateRune, Vector())
    ruleBuilder += CoerceToCoordSR(range, resultRune, kindRune)
  }
}

*/
// mig: struct HigherTypingPass
pub struct HigherTypingPass<'a, 'ctx> {
  global_options: GlobalOptions,
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  primitives: std::collections::HashMap<StrI<'a>, ITemplataType>,
}

// mig: impl HigherTypingPass
impl<'a, 'ctx> HigherTypingPass<'a, 'ctx> {
}
/*
class HigherTypingPass(globalOptions: GlobalOptions, interner: Interner, keywords: Keywords) {
  val primitives =
    Map(
      keywords.int -> KindTemplataType(),
      keywords.i64 -> KindTemplataType(),
      keywords.str -> KindTemplataType(),
      keywords.bool -> KindTemplataType(),
      keywords.float -> KindTemplataType(),
      keywords.void -> KindTemplataType(),
      keywords.__Never -> KindTemplataType(),
      keywords.Array -> TemplateTemplataType(Vector(MutabilityTemplataType(), CoordTemplataType()), KindTemplataType()),
      keywords.StaticArray -> TemplateTemplataType(Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()), KindTemplataType())
      // Put in with regions
      // keywords.Array -> TemplateTemplataType(Vector(MutabilityTemplataType(), CoordTemplataType(), RegionTemplataType()), KindTemplataType()),
      // keywords.StaticArray -> TemplateTemplataType(Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType(), RegionTemplataType()), KindTemplataType())
      )

  vregionmut() // Change the above Array/StaticArray templata types to have regions in them



*/
// mig: fn imprecise_name_matches_absolute_name
fn imprecise_name_matches_absolute_name(needle_imprecise_name_s: &IImpreciseNameS, absolute_name: &INameS) -> bool {
  panic!("Unimplemented: imprecise_name_matches_absolute_name");
}
/*
  // Returns whether the imprecise name could be referring to the absolute name.
  // See MINAAN for what we're doing here.
  def impreciseNameMatchesAbsoluteName(
    needleImpreciseNameS: IImpreciseNameS,
    absoluteName: INameS):
  Boolean = {
    (needleImpreciseNameS, absoluteName) match {
      case (CodeNameS(humanNameB), TopLevelCitizenDeclarationNameS(humanNameA, _)) => humanNameA == humanNameB
      case (RuneNameS(a), _) => false
      case other => vimpl(other)
    }
  }

*/
// mig: fn lookup_types
fn lookup_types<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, needle_imprecise_name_s: &IImpreciseNameS) -> Vec<IRuneTypeSolverLookupResult> {
  panic!("Unimplemented: lookup_types");
}
/*
  def lookupTypes(
    astrouts: Astrouts,
    env: EnvironmentA,
    needleImpreciseNameS: IImpreciseNameS):
  Vector[IRuneTypeSolverLookupResult] = {
    // See MINAAN for what we're doing here.

    // When the scout comes across a lambda, it doesn't put the e.g. main:lam1:__Closure struct into
    // the environment or anything, it lets typingpass to do that (because typingpass knows the actual types).
    // However, this means that when the lambda function gets to the higher typer, the higher typer doesn't
    // know what to do with it.
    needleImpreciseNameS match {
      case CodeNameS(_) =>
      case RuneNameS(_) =>
    }

    needleImpreciseNameS match {
      case CodeNameS(nameStr) => {
        primitives.get(nameStr) match {
          case Some(x) => return Vector(PrimitiveRuneTypeSolverLookupResult(x))
          case None =>
        }
      }
      case _ =>
    }

    needleImpreciseNameS match {
      case RuneNameS(rune) => {
        env.runeToType.get(rune) match {
          case Some(tyype) => return Vector(TemplataLookupResult(tyype))
          case None =>
        }
      }
      case _ =>
    }

    val nearStructTypes =
      env.structsS
        .filter(interface => impreciseNameMatchesAbsoluteName(needleImpreciseNameS, interface.name))
        .map(x => CitizenRuneTypeSolverLookupResult(x.tyype, x.genericParams))
    val nearInterfaceTypes =
      env.interfacesS
        .filter(interface => impreciseNameMatchesAbsoluteName(needleImpreciseNameS, interface.name))
        .map(x => CitizenRuneTypeSolverLookupResult(x.tyype, x.genericParams))
    val result = nearStructTypes ++ nearInterfaceTypes

    if (result.nonEmpty) {
      result
    } else {
      env.maybeParentEnv match {
        case None => Vector.empty
        case Some(parentEnv) => lookupTypes(astrouts, parentEnv, needleImpreciseNameS)
      }
    }
  }

*/
// mig: fn lookup_type
fn lookup_type<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, range: RangeS, name: &IImpreciseNameS) -> Result<IRuneTypeSolverLookupResult, ILookupFailedErrorA> {
  panic!("Unimplemented: lookup_type");
}
/*
  def lookupType(
    astrouts: Astrouts,
    env: EnvironmentA,
    range: RangeS,
    name: IImpreciseNameS):
  Result[IRuneTypeSolverLookupResult, ILookupFailedErrorA] = {
    lookupTypes(astrouts, env, name).distinct match {
      case Vector() => Err(CouldntFindTypeA(range, name))
      case Vector(only) => Ok(only)
      case others => Err(TooManyMatchingTypesA(range, name))
    }
  }

*/
// mig: fn translate_struct
fn translate_struct<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, struct_s: &StructS<'a>) -> StructA<'a> {
  panic!("Unimplemented: translate_struct");
}
/*
  def translateStruct(
    astrouts: Astrouts,
    env: EnvironmentA,
    structS: StructS):
  StructA = {
    val StructS(rangeS, nameS, attributesS, weakable, genericParametersS, mutabilityRuneS, maybePredictedMutability, tyype, headerRuneToExplicitType, headerPredictedRuneToType, headerRulesWithImplicitlyCoercingLookupsS, membersRuneToExplicitType, membersPredictedRuneToType, memberRulesWithImplicitlyCoercingLookupsS, members) = structS

    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }

    astrouts.codeLocationToStruct.get(rangeS.begin) match {
      case Some(value) => return value
      case None =>
    }

    astrouts.codeLocationToMaybeType.get(rangeS.begin) match {
      // Weird because this means we already evaluated it, in which case we should have hit the above return
      case Some(Some(_)) => vwat()
      case Some(None) => {
        throw CompileErrorExceptionA(highertyping.RangedInternalErrorA(rangeS, "Cycle in determining struct type!"))
      }
      case None =>
    }
    astrouts.codeLocationToMaybeType.put(rangeS.begin, None)

    val allRulesWithImplicitlyCoercingLookupsS =
      headerRulesWithImplicitlyCoercingLookupsS ++ memberRulesWithImplicitlyCoercingLookupsS
    val allRuneToExplicitType = headerRuneToExplicitType ++ membersRuneToExplicitType
    val runeAToTypeWithImplicitlyCoercingLookupsS =
      calculateRuneTypes(
        astrouts,
        rangeS,
        genericParametersS.map(_.rune.rune),
        allRuneToExplicitType,
        Vector(),
        allRulesWithImplicitlyCoercingLookupsS,
        env)

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.

    val headerRulesBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv, runeAToType, headerRulesBuilder, headerRulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }
    val headerRulesExplicitS = headerRulesBuilder.toVector

    val memberRulesBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv, runeAToType, memberRulesBuilder, memberRulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }
    val memberRulesExplicitS = memberRulesBuilder.toVector

    val runesInHeader: Set[IRuneS] =
      (genericParametersS.map(_.rune.rune) ++
        genericParametersS.flatMap(_.default).flatMap(_.rules.map(_.runeUsages.map(_.rune))).flatten ++
        headerRulesExplicitS.flatMap(_.runeUsages.map(_.rune))).toSet
    val headerRuneAToType = runeAToType.toMap.filter(x => runesInHeader.contains(x._1))
    val membersRuneAToType = runeAToType.toMap.filter(x => !runesInHeader.contains(x._1))

    // Shouldnt fail because we got a complete solve earlier
    astrouts.codeLocationToMaybeType.put(rangeS.begin, Some(tyype))

    headerRulesExplicitS.collect({
      case MaybeCoercingCallSR(_, _, _, _) => vwat()
    })
    memberRulesExplicitS.collect({
      case MaybeCoercingCallSR(_, _, _, _) => vwat()
    })

    val structA =
      highertyping.StructA(
        rangeS,
        nameS,
        attributesS,
        weakable,
        mutabilityRuneS,
        maybePredictedMutability,
        tyype,
        genericParametersS,
        headerRuneAToType,
        headerRulesExplicitS,
        membersRuneAToType,
        memberRulesExplicitS,
        members)
    astrouts.codeLocationToStruct.put(rangeS.begin, structA)
    structA
  }

*/
// mig: fn get_interface_type
fn get_interface_type<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, interface_s: &InterfaceS<'a>) -> ITemplataType {
  panic!("Unimplemented: get_interface_type");
}
/*
  def getInterfaceType(
    astrouts: Astrouts,
    env: EnvironmentA,
    interfaceS: InterfaceS):
  ITemplataType = {
    interfaceS.tyype
  }

*/
// mig: fn translate_interface
fn translate_interface<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, interface_s: &InterfaceS<'a>) -> InterfaceA<'a> {
  panic!("Unimplemented: translate_interface");
}
/*
  def translateInterface(astrouts: Astrouts,  env: EnvironmentA, interfaceS: InterfaceS): InterfaceA = {
    val InterfaceS(rangeS, nameS, attributesS, weakable, genericParametersS, runeToExplicitType, mutabilityRuneS, maybePredictedMutability, predictedRuneToType, tyype, rulesWithImplicitlyCoercingLookupsS, internalMethodsS) = interfaceS

    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }

    astrouts.codeLocationToInterface.get(rangeS.begin) match {
      case Some(value) => return value
      case None =>
    }

    astrouts.codeLocationToMaybeType.get(rangeS.begin) match {
      // Weird because this means we already evaluated it, in which case we should have hit the above return
      case Some(Some(_)) => vwat()
      case Some(None) => {
        throw CompileErrorExceptionA(highertyping.RangedInternalErrorA(rangeS, "Cycle in determining interface type!"))
      }
      case None =>
    }
    astrouts.codeLocationToMaybeType.put(rangeS.begin, None)

    val runeAToTypeWithImplicitlyCoercingLookups =
      calculateRuneTypes(astrouts, rangeS, genericParametersS.map(_.rune.rune), runeToExplicitType, Vector(), rulesWithImplicitlyCoercingLookupsS, env)

    // getOrDie because we should have gotten a complete solve
    astrouts.codeLocationToMaybeType.put(rangeS.begin, Some(tyype))

    val methodsEnv =
      env
        .addRunes(runeAToTypeWithImplicitlyCoercingLookups)
    val internalMethodsA =
      internalMethodsS.map(method => {
        translateFunction(astrouts, methodsEnv, method)
      })


    val runeAToTypeWithImplicitlyCoercingLookupsS =
      calculateRuneTypes(
        astrouts, rangeS, genericParametersS.map(_.rune.rune), runeToExplicitType, Vector(), rulesWithImplicitlyCoercingLookupsS, env)

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }

    val interfaceA =
      highertyping.InterfaceA(
        rangeS,
        nameS,
        attributesS,
        weakable,
        mutabilityRuneS,
        maybePredictedMutability,
        tyype,
        //        knowableRunesS,
        genericParametersS,
        //        localRunesS,
        //        conclusions,
        runeAToType.toMap,
        ruleBuilder.toVector,
        internalMethodsA)
    astrouts.codeLocationToInterface.put(rangeS.begin, interfaceA)
    interfaceA
  }

*/
// mig: fn translate_impl
fn translate_impl<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, impl_s: &ImplS<'a>) -> ImplA<'a> {
  panic!("Unimplemented: translate_impl");
}
/*
  def translateImpl(astrouts: Astrouts,  env: EnvironmentA, implS: ImplS): ImplA = {
    val ImplS(rangeS, nameS, identifyingRunesS, rulesWithImplicitlyCoercingLookupsS, runeToExplicitType, tyype, structKindRuneS, subCitizenImpreciseName, interfaceKindRuneS, superInterfaceImpreciseName) = implS

    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      calculateRuneTypes(
        astrouts,
        rangeS,
        identifyingRunesS.map(_.rune.rune),
        runeToExplicitType + (structKindRuneS.rune -> KindTemplataType(), interfaceKindRuneS.rune -> KindTemplataType()),
        Vector(),
        rulesWithImplicitlyCoercingLookupsS,
        env)

    // getOrDie because we should have gotten a complete solve
    astrouts.codeLocationToMaybeType.put(rangeS.begin, Some(tyype))

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(runeTypingEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }

    highertyping.ImplA(
      rangeS,
      nameS,
      identifyingRunesS,
      ruleBuilder.toVector,
      runeAToType.toMap,
      structKindRuneS,
      subCitizenImpreciseName,
      interfaceKindRuneS,
      superInterfaceImpreciseName)
  }

*/
// mig: fn translate_export
fn translate_export<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, export_s: &ExportAsS<'a>) -> ExportAsA<'a> {
  panic!("Unimplemented: translate_export");
}
/*
  def translateExport(astrouts: Astrouts,  env: EnvironmentA, exportS: ExportAsS): ExportAsA = {
    val ExportAsS(rangeS, rulesWithImplicitlyCoercingLookupsS, exportName, rune, exportedName) = exportS

    val defaultRegionRune = ExportDefaultRegionRuneS(exportName)

    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      calculateRuneTypes(
        astrouts,
        rangeS,
        Vector(),
        Map(rune.rune -> KindTemplataType()),
        Vector(),
        rulesWithImplicitlyCoercingLookupsS,
        env)

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(runeTypingEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }

    ExportAsA(
      rangeS,
      exportedName,
      ruleBuilder.toVector,
      runeAToType.toMap,
      rune)
  }

*/
// mig: fn translate_function
fn translate_function<'a>(astrouts: &Astrouts<'a>, env: &EnvironmentA<'a>, function_s: &FunctionS<'a>) -> FunctionA<'a> {
  panic!("Unimplemented: translate_function");
}
/*
  def translateFunction(astrouts: Astrouts, env: EnvironmentA, functionS: FunctionS): FunctionA = {
    val FunctionS(rangeS, nameS, attributesS, identifyingRunesS, runeToExplicitType, tyype, paramsS, maybeRetCoordRune, rulesWithImplicitlyCoercingLookupsS, bodyS) = functionS

    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      calculateRuneTypes(
        astrouts, rangeS, identifyingRunesS.map(_.rune.rune), runeToExplicitType, paramsS, rulesWithImplicitlyCoercingLookupsS, env)

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(runeTypingEnv, runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionA(TooManyMatchingTypesA(range, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionA(CouldntFindTypeA(range, name))
      case Ok(()) =>
    }

    highertyping.FunctionA(
      rangeS,
      nameS,
      attributesS ++ Vector(UserFunctionS),
      tyype,
      identifyingRunesS,
      runeAToType.toMap,
      paramsS,
      maybeRetCoordRune,
      ruleBuilder.toVector,
      bodyS)
  }

*/
// mig: fn calculate_rune_types
fn calculate_rune_types<'a>(astrouts: &Astrouts<'a>, range_s: RangeS, identifying_runes_s: Vec<&'a IRuneS<'a>>, rune_to_explicit_type: std::collections::HashMap<&'a IRuneS<'a>, ITemplataType>, params_s: Vec<&ParameterS<'a>>, rules_s: Vec<IRulexSR>, env: &EnvironmentA<'a>) -> std::collections::HashMap<&'a IRuneS<'a>, ITemplataType> {
  panic!("Unimplemented: calculate_rune_types");
}
/*
  private def calculateRuneTypes(
    astrouts: Astrouts,
    rangeS: RangeS,
    identifyingRunesS: Vector[IRuneS],
    runeToExplicitType: Map[IRuneS, ITemplataType],
    paramsS: Vector[ParameterS],
    rulesS: Vector[IRulexSR],
    env: EnvironmentA):
  Map[IRuneS, ITemplataType] = {
    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(
          range: RangeS,
          name: IImpreciseNameS
        ): Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          lookupType(astrouts, env, rangeS, name).mapError({
            case TooManyMatchingTypesA(range, name) => RuneTypingTooManyMatchingTypes(range, name)
            case CouldntFindTypeA(range, name) => RuneTypingCouldntFindType(range, name)
          })
        }
      }
    val runeSToPreKnownTypeA =
      runeToExplicitType ++
        paramsS.flatMap(_.pattern.coordRune.map(_.rune -> CoordTemplataType())).toMap
    val runeSToType =
      new RuneTypeSolver(interner).solve(
        globalOptions.sanityCheck,
        globalOptions.useOptimizedSolver,
        runeTypingEnv,
        List(rangeS),
        false, rulesS, identifyingRunesS, true, runeSToPreKnownTypeA) match {
        case Ok(t) => t
        case Err(e) => throw CompileErrorExceptionA(CouldntSolveRulesA(rangeS, e))
      }
    runeSToType
  }

*/
// mig: fn translate_program
fn translate_program<'a>(code_map: PackageCoordinateMap<'a, ProgramS<'a>>, primitives: std::collections::HashMap<StrI<'a>, ITemplataType>, supplied_functions: Vec<FunctionA<'a>>, supplied_interfaces: Vec<InterfaceA<'a>>) -> ProgramA<'a> {
  panic!("Unimplemented: translate_program");
}
/*
  def translateProgram(
      codeMap: PackageCoordinateMap[ProgramS],
      primitives: Map[StrI, ITemplataType],
      suppliedFunctions: Vector[FunctionA],
      suppliedInterfaces: Vector[InterfaceA]):
  ProgramA = {
    val env = EnvironmentA(None, None, codeMap, Map())

    // If something is absence from the map, we haven't started evaluating it yet
    // If there is a None in the map, we started evaluating it
    // If there is a Some in the map, we know the type
    // If we are asked to evaluate something but there is already a None in the map, then we are
    // caught in a cycle.
    val astrouts =
      Astrouts(
        mutable.HashMap[CodeLocationS, Option[ITemplataType]](),
        mutable.HashMap[CodeLocationS, StructA](),
        mutable.HashMap[CodeLocationS, InterfaceA]())

    val structsA = env.structsS.map(translateStruct(astrouts, env, _))

    val interfacesA = env.interfacesS.map(translateInterface(astrouts, env, _))

    val implsA = env.implsS.map(translateImpl(astrouts, env, _))

    val functionsA = env.functionsS.map(translateFunction(astrouts, env, _))

    val exportsA = env.exportsS.map(translateExport(astrouts, env, _))

    ProgramA(structsA, suppliedInterfaces ++ interfacesA, implsA, suppliedFunctions ++ functionsA, exportsA)
  }

*/
// mig: fn run_pass
fn run_pass<'a>(separate_programs_s: FileCoordinateMap<'a, ProgramS<'a>>) -> Result<PackageCoordinateMap<'a, ProgramA<'a>>, ICompileErrorA> {
  panic!("Unimplemented: run_pass");
}
/*
  def runPass(separateProgramsS: FileCoordinateMap[ProgramS]):
  Either[PackageCoordinateMap[ProgramA], ICompileErrorA] = {
    Profiler.frame(() => {
      val mergedProgramS =
        PackageCoordinateMap[ProgramS]()
      separateProgramsS.packageCoordToFileCoords.foreach({ case (packageCoord, fileCoords) =>
        val programsS = fileCoords.map(separateProgramsS.fileCoordToContents)
        mergedProgramS.put(
          packageCoord,
          ProgramS(
            programsS.flatMap(_.structs).toVector,
            programsS.flatMap(_.interfaces).toVector,
            programsS.flatMap(_.impls).toVector,
            programsS.flatMap(_.implementedFunctions).toVector,
            programsS.flatMap(_.exports).toVector,
            programsS.flatMap(_.imports).toVector))
      })
      val imports = mergedProgramS.packageCoordToContents.values.flatMap(_.imports)
//      val rustImports = imports.filter(_.moduleName == keywords.rust)
//      rustImports.foreach({
//        case ImportS(_, moduleName, packageNames, importeeName) => {
//          val rustPackageString = packageNames.map(_.str).mkString(".")
//
//          // ask a rust process to generate the json
//          // DO NOT SUBMIT
//          val processBuilder = Process("glass", List("/Users/verdagon/.cargo/bin/rustc", rustPackageString, importeeName.str))
//          val process = processBuilder.run
//          // Blocks
//          process.exitValue()
//        }
//      })

      //    val orderedModules = orderModules(mergedProgramS)

      try {
        val suppliedFunctions = Vector()
        val suppliedInterfaces = Vector()
        val ProgramA(structsA, interfacesA, implsA, functionsA, exportsA) =
          translateProgram(
            mergedProgramS, primitives, suppliedFunctions, suppliedInterfaces)

        val packageToStructsA = structsA.groupBy(_.range.begin.file.packageCoordinate)
        val packageToInterfacesA = interfacesA.groupBy(_.name.range.begin.file.packageCoordinate)
        val packageToFunctionsA = functionsA.groupBy(_.name.packageCoordinate)
        val packageToImplsA = implsA.groupBy(_.name.packageCoordinate)
        val packageToExportsA = exportsA.groupBy(_.range.file.packageCoordinate)

        val allPackages =
          packageToStructsA.keySet ++
            packageToInterfacesA.keySet ++
            packageToFunctionsA.keySet ++
            packageToImplsA.keySet ++
            packageToExportsA.keySet
        val packageToContents = mutable.HashMap[PackageCoordinate, ProgramA]()
        allPackages.foreach(paackage => {
          val contents =
            ProgramA(
              packageToStructsA.getOrElse(paackage, Vector.empty),
              packageToInterfacesA.getOrElse(paackage, Vector.empty),
              packageToImplsA.getOrElse(paackage, Vector.empty),
              packageToFunctionsA.getOrElse(paackage, Vector.empty),
              packageToExportsA.getOrElse(paackage, Vector.empty))
          packageToContents.put(paackage, contents)
        })
        Left(vale.PackageCoordinateMap(packageToContents))
      } catch {
        case CompileErrorExceptionA(err) => {
          Right(err)
        }
      }
    })
  }
}

*/
// mig: struct HigherTypingCompilation
pub struct HigherTypingCompilation<'a, 'ctx, 'p> {
  global_options: GlobalOptions,
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  scout_compilation: ScoutCompilation<'a, 'ctx, 'p>,
  astrouts_cache: Option<PackageCoordinateMap<'a, ProgramA<'a>>>,
}

// mig: impl HigherTypingCompilation
impl<'a, 'ctx, 'p> HigherTypingCompilation<'a, 'ctx, 'p> {
}
/*
class HigherTypingCompilation(
  globalOptions: GlobalOptions,
  val interner: Interner,
  val keywords: Keywords,
  packagesToBuild: Vector[PackageCoordinate],
  packageToContentsResolver: IPackageResolver[Map[String, String]]) {
  var scoutCompilation = new ScoutCompilation(globalOptions, interner, keywords, packagesToBuild, packageToContentsResolver)
  var astroutsCache: Option[PackageCoordinateMap[ProgramA]] = None

*/
// mig: fn get_code_map
fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
  panic!("Unimplemented: get_code_map");
}
/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getCodeMap()
*/
// mig: fn get_parseds
fn get_parseds(&mut self) -> Result<FileCoordinateMap<'a, (FileP<'a, 'p>, Vec<RangeL>)>, FailedParse<'a>> {
  panic!("Unimplemented: get_parseds");
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = scoutCompilation.getParseds()
*/
// mig: fn get_vpst_map
fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
  panic!("Unimplemented: get_vpst_map");
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getVpstMap()
*/
// mig: fn get_scoutput
fn get_scoutput(&mut self) -> Result<FileCoordinateMap<'a, ProgramS<'a>>, ICompileErrorS> {
  panic!("Unimplemented: get_scoutput");
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = scoutCompilation.getScoutput()

*/
// mig: fn get_astrouts
fn get_astrouts(&mut self) -> Result<PackageCoordinateMap<'a, ProgramA<'a>>, ICompileErrorA> {
  panic!("Unimplemented: get_astrouts");
}
/*
  def getAstrouts(): Result[PackageCoordinateMap[ProgramA], ICompileErrorA] = {
    astroutsCache match {
      case Some(astrouts) => Ok(astrouts)
      case None => {
        new HigherTypingPass(globalOptions, interner, keywords).runPass(scoutCompilation.expectScoutput()) match {
          case Right(err) => Err(err)
          case Left(astrouts) => {
            astroutsCache = Some(astrouts)
            Ok(astrouts)
          }
        }
      }
    }
  }
*/
// mig: fn expect_astrouts
fn expect_astrouts(&mut self) -> PackageCoordinateMap<'a, ProgramA<'a>> {
  panic!("Unimplemented: expect_astrouts");
}
/*
  def expectAstrouts(): PackageCoordinateMap[ProgramA] = {
    getAstrouts() match {
      case Ok(x) => x
      case Err(e) => {
        val codeMap = scoutCompilation.getCodeMap().getOrDie()
        vfail(
          HigherTypingErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
    }
  }
}

*/
