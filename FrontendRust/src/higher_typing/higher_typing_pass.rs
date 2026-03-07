use crate::compile_options::GlobalOptions;
use crate::higher_typing::ast::{
    ExportAsA, FunctionA, ImplA, InterfaceA, ProgramA, StructA,
};
use crate::higher_typing::astronomer_error_reporter::{
    ICompileErrorA, ILookupFailedErrorA,
};
use crate::interner::{Interner, StrI};
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::FileP;
use crate::postparsing::ast::{
    ExportAsS, FunctionS, IFunctionAttributeS, ImplS, ImportS, InterfaceS, ParameterS, ProgramS,
    StructS, UserFunctionS,
};
use crate::postparsing::itemplatatype::{
    CoordTemplataType, ITemplataType, IntegerTemplataType, KindTemplataType,
    MutabilityTemplataType, TemplateTemplataType, VariabilityTemplataType,
};
use crate::postparsing::names::{IImpreciseNameS, INameS, IRuneS};
use crate::postparsing::rune_type_solver::{
    IRuneTypeSolverEnv, IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError,
};
use crate::postparsing::rules::rules::{IRulexSR, RuneUsage};
use crate::postparsing::post_parser::ICompileErrorS;
use crate::postparsing::ScoutCompilation;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate, PackageCoordinateMap};
use crate::utils::range::RangeS;
use crate::utils::range::CodeLocationS;
use std::collections::HashMap;
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
pub struct Astrouts<'a, 's> {
  code_location_to_maybe_type: std::collections::HashMap<CodeLocationS<'a>, Option<ITemplataType>>,
  code_location_to_struct: std::collections::HashMap<CodeLocationS<'a>, StructA<'a, 's>>,
  code_location_to_interface: std::collections::HashMap<CodeLocationS<'a>, InterfaceA<'a, 's>>,
}

// mig: impl Astrouts
impl<'a, 's> Astrouts<'a, 's> {
}
/*
case class Astrouts(
  codeLocationToMaybeType: mutable.HashMap[CodeLocationS, Option[ITemplataType]],
  codeLocationToStruct: mutable.HashMap[CodeLocationS, StructA],
  codeLocationToInterface: mutable.HashMap[CodeLocationS, InterfaceA])

*/
// mig: struct EnvironmentA
pub struct EnvironmentA<'a, 's> {
  maybe_name: Option<&'a INameS<'a>>,
  maybe_parent_env: Option<&'s EnvironmentA<'a, 's>>,
  code_map: PackageCoordinateMap<'a, ProgramS<'a, 's>>,
  rune_to_type: std::collections::HashMap<&'a IRuneS<'a>, ITemplataType>,
}

// mig: impl EnvironmentA
impl<'a, 's> EnvironmentA<'a, 's> {
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
fn equals(&self, _obj: &dyn std::any::Any) -> bool {
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
*/
  pub fn structs_s(&self) -> Vec<&'s StructS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.structs.iter()).collect()
  }
/*
  val structsS: Vector[StructS] = codeMap.packageCoordToContents.values.flatMap(_.structs).toVector
*/
  pub fn interfaces_s(&self) -> Vec<&'s InterfaceS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.interfaces.iter()).collect()
  }
/*
    val interfacesS: Vector[InterfaceS] = codeMap.packageCoordToContents.values.flatMap(_.interfaces).toVector
*/
  pub fn impls_s(&self) -> Vec<&'s ImplS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.impls.iter()).collect()
  }
/*
    val implsS: Vector[ImplS] = codeMap.packageCoordToContents.values.flatMap(_.impls).toVector
*/
  pub fn functions_s(&self) -> Vec<&'s &'s FunctionS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.implemented_functions.iter()).collect()
  }
/*
    val functionsS: Vector[FunctionS] = codeMap.packageCoordToContents.values.flatMap(_.implementedFunctions).toVector
*/
  pub fn exports_s(&self) -> Vec<&'s ExportAsS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.exports.iter()).collect()
  }
/*
    val exportsS: Vector[ExportAsS] = codeMap.packageCoordToContents.values.flatMap(_.exports).toVector
*/
  pub fn imports_s(&self) -> Vec<&'s ImportS<'a, 's>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.imports.iter()).collect()
  }
/*
    val imports: Vector[ImportS] = codeMap.packageCoordToContents.values.flatMap(_.imports).toVector
*/

// mig: fn add_runes
fn add_runes(&self, _new_rune_to_type: std::collections::HashMap<&'a IRuneS<'a>, ITemplataType>) -> EnvironmentA<'a, 's> {
  panic!("Unimplemented: add_runes");
}
/*
  def addRunes(newruneToType: Map[IRuneS, ITemplataType]): EnvironmentA = {
    EnvironmentA(maybeName, maybeParentEnv, codeMap, runeToType ++ newruneToType)
  }
}
*/
}
/*
object HigherTypingPass {
*/

// mig: fn explicify_lookups
fn explicify_lookups<'a, E: IRuneTypeSolverEnv<'a>>(_env: &E, _rune_a_to_type: &mut HashMap<IRuneS<'a>, ITemplataType>, rule_builder: &mut Vec<IRulexSR<'a>>, all_rules_with_implicitly_coercing_lookups_s: Vec<IRulexSR<'a>>) -> Result<(), IRuneTypingLookupFailedError<'a>> {
  // Scala: Only two rules' results can be coerced: LookupSR and CallSR.
  // Let's look for those and rewrite them to put an explicit coercion in there.
  for rule in all_rules_with_implicitly_coercing_lookups_s {
    match &rule {
      IRulexSR::MaybeCoercingLookup(_) => {
        panic!("explicify_lookups: MaybeCoercingLookup not yet migrated");
      }
      IRulexSR::MaybeCoercingCall(_) => {
        panic!("explicify_lookups: MaybeCoercingCall not yet migrated");
      }
      _ => {
        // All other rules pass through unchanged
        rule_builder.push(rule);
      }
    }
  }
  Ok(())
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
fn coerce_kind_lookup_to_coord(_rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, _rule_builder: &mut Vec<IRulexSR>, _range: RangeS, _result_rune: RuneUsage, _name: &IImpreciseNameS) {
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
fn coerce_kind_template_lookup_to_kind(_rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, _rule_builder: &mut Vec<IRulexSR>, _range: RangeS, _result_rune: RuneUsage, _name: &IImpreciseNameS, _actual_template_type: TemplateTemplataType) {
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
fn coerce_kind_template_lookup_to_coord(_rune_a_to_type: &mut std::collections::HashMap<&IRuneS, ITemplataType>, _rule_builder: &mut Vec<IRulexSR>, _range: RangeS, _result_rune: RuneUsage, _name: &IImpreciseNameS, _ttt: TemplateTemplataType) {
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
  pub fn new(
    global_options: GlobalOptions,
    interner: &'ctx Interner<'a>,
    keywords: &'ctx Keywords<'a>,
  ) -> Self {
    let mut primitives = HashMap::new();
    primitives.insert(keywords.int, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.i64, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.str, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.bool, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.float, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.void, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.__never, ITemplataType::KindTemplataType(KindTemplataType {}));
    primitives.insert(keywords.array, ITemplataType::TemplateTemplataType(TemplateTemplataType {
      param_types: vec![
        ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ],
      return_type: Box::new(ITemplataType::KindTemplataType(KindTemplataType {})),
    }));
    primitives.insert(keywords.static_array, ITemplataType::TemplateTemplataType(TemplateTemplataType {
      param_types: vec![
        ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
        ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ],
      return_type: Box::new(ITemplataType::KindTemplataType(KindTemplataType {})),
    }));
    HigherTypingPass {
      global_options,
      interner,
      keywords,
      primitives,
    }
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
fn imprecise_name_matches_absolute_name(&self, _needle_imprecise_name_s: &IImpreciseNameS, _absolute_name: &INameS) -> bool {
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
fn lookup_types<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _needle_imprecise_name_s: &IImpreciseNameS<'a>) -> Vec<IRuneTypeSolverLookupResult<'a>> {
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
fn lookup_type<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _range: RangeS<'a>, _name: &IImpreciseNameS<'a>) -> Result<IRuneTypeSolverLookupResult<'a>, Box<dyn ILookupFailedErrorA<'a> + 'a>> {
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
fn translate_struct<'s>(&self, astrouts: &mut Astrouts<'a, 's>, env: &EnvironmentA<'a, 's>, struct_s: &StructS<'a, 's>) -> StructA<'a, 's> {
  let StructS {
    range: range_s,
    name: name_s,
    attributes: attributes_s,
    weakable,
    generic_params: generic_parameters_s,
    mutability_rune: mutability_rune_s,
    maybe_predicted_mutability,
    tyype,
    header_rune_to_explicit_type,
    header_predicted_rune_to_type: _,
    header_rules: header_rules_with_implicitly_coercing_lookups_s,
    members_rune_to_explicit_type,
    members_predicted_rune_to_type: _,
    member_rules: member_rules_with_implicitly_coercing_lookups_s,
    members,
  } = struct_s;

  // Scala: val runeTypingEnv = new IRuneTypeSolverEnv { override def lookup(...) }
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    astrouts,
    env,
    range_s: range_s.clone(),
  };

  // Scala: astrouts.codeLocationToStruct.get(rangeS.begin) match { case Some(value) => return value }
  if let Some(value) = astrouts.code_location_to_struct.get(&range_s.begin) {
    return value.clone();
  }

  // Scala: astrouts.codeLocationToMaybeType.get(rangeS.begin) match { ... }
  match astrouts.code_location_to_maybe_type.get(&range_s.begin) {
    Some(Some(_)) => {
      panic!("translate_struct: already evaluated but not in struct map");
    }
    Some(None) => {
      panic!("Cycle in determining struct type!");
    }
    None => {}
  }

  // Scala: astrouts.codeLocationToMaybeType.put(rangeS.begin, None)
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), None);

  // Scala: val allRulesWithImplicitlyCoercingLookupsS = headerRulesWithImplicitlyCoercingLookupsS ++ memberRulesWithImplicitlyCoercingLookupsS
  let all_rules_with_implicitly_coercing_lookups_s: Vec<IRulexSR<'a>> =
    header_rules_with_implicitly_coercing_lookups_s.iter().cloned()
      .chain(member_rules_with_implicitly_coercing_lookups_s.iter().cloned())
      .collect();

  // Scala: val allRuneToExplicitType = headerRuneToExplicitType ++ membersRuneToExplicitType
  let mut all_rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType> = HashMap::new();
  all_rune_to_explicit_type.extend(header_rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())));
  all_rune_to_explicit_type.extend(members_rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())));

  // Scala: val runeAToTypeWithImplicitlyCoercingLookupsS = calculateRuneTypes(...)
  let rune_a_to_type_with_implicitly_coercing_lookups_s = self.calculate_rune_types(
    astrouts,
    range_s.clone(),
    generic_parameters_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
    all_rune_to_explicit_type,
    &[],
    &all_rules_with_implicitly_coercing_lookups_s,
    env,
  );

  // Scala: val runeAToType = mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
  let mut rune_a_to_type: HashMap<IRuneS<'a>, ITemplataType> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;

  // Scala: val headerRulesBuilder = ArrayBuffer[IRulexSR]()
  // Scala: explicifyLookups(runeTypingEnv, runeAToType, headerRulesBuilder, headerRulesWithImplicitlyCoercingLookupsS) match { ... }
  let mut header_rules_builder: Vec<IRulexSR<'a>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    &mut rune_a_to_type,
    &mut header_rules_builder,
    header_rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for header rules"),
  }
  let header_rules_explicit_s = header_rules_builder;

  // Scala: val memberRulesBuilder = ArrayBuffer[IRulexSR]()
  // Scala: explicifyLookups(runeTypingEnv, runeAToType, memberRulesBuilder, memberRulesWithImplicitlyCoercingLookupsS) match { ... }
  let mut member_rules_builder: Vec<IRulexSR<'a>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    &mut rune_a_to_type,
    &mut member_rules_builder,
    member_rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for member rules"),
  }
  let member_rules_explicit_s = member_rules_builder;

  // Scala: val runesInHeader: Set[IRuneS] = (genericParametersS.map(_.rune.rune) ++ ...)
  let mut runes_in_header: std::collections::HashSet<IRuneS<'a>> = std::collections::HashSet::new();
  for gp in generic_parameters_s.iter() {
    runes_in_header.insert(gp.rune.rune.clone());
  }
  for gp in generic_parameters_s.iter() {
    if let Some(ref default) = gp.default {
      for rule in &default.rules {
        for rune_usage in rule.rune_usages() {
          runes_in_header.insert(rune_usage.rune.clone());
        }
      }
    }
  }
  for rule in &header_rules_explicit_s {
    for rune_usage in rule.rune_usages() {
      runes_in_header.insert(rune_usage.rune.clone());
    }
  }

  // Scala: val headerRuneAToType = runeAToType.toMap.filter(x => runesInHeader.contains(x._1))
  let header_rune_a_to_type: HashMap<IRuneS<'a>, ITemplataType> =
    rune_a_to_type.iter()
      .filter(|(k, _)| runes_in_header.contains(k))
      .map(|(k, v)| (k.clone(), v.clone()))
      .collect();

  // Scala: val membersRuneAToType = runeAToType.toMap.filter(x => !runesInHeader.contains(x._1))
  let members_rune_a_to_type: HashMap<IRuneS<'a>, ITemplataType> =
    rune_a_to_type.iter()
      .filter(|(k, _)| !runes_in_header.contains(k))
      .map(|(k, v)| (k.clone(), v.clone()))
      .collect();

  // Scala: astrouts.codeLocationToMaybeType.put(rangeS.begin, Some(tyype))
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), Some(tyype.clone()));

  // Scala: headerRulesExplicitS.collect({ case MaybeCoercingCallSR(_, _, _, _) => vwat() })
  for rule in &header_rules_explicit_s {
    if let IRulexSR::MaybeCoercingCall(_) = rule {
      panic!("MaybeCoercingCallSR should have been explicified in header rules");
    }
  }
  // Scala: memberRulesExplicitS.collect({ case MaybeCoercingCallSR(_, _, _, _) => vwat() })
  for rule in &member_rules_explicit_s {
    if let IRulexSR::MaybeCoercingCall(_) = rule {
      panic!("MaybeCoercingCallSR should have been explicified in member rules");
    }
  }

  // Scala: val structA = highertyping.StructA(rangeS, nameS, ...)
  let struct_a = StructA {
    range: range_s.clone(),
    name: name_s.clone(),
    attributes: attributes_s.to_vec(),
    weakable: *weakable,
    mutability_rune: mutability_rune_s.clone(),
    maybe_predicted_mutability: maybe_predicted_mutability.clone(),
    tyype: tyype.clone(),
    generic_parameters: generic_parameters_s.to_vec(),
    header_rune_to_type: header_rune_a_to_type,
    header_rules: header_rules_explicit_s,
    members_rune_to_type: members_rune_a_to_type,
    member_rules: member_rules_explicit_s,
    members: members.to_vec(),
  };

  // Scala: astrouts.codeLocationToStruct.put(rangeS.begin, structA)
  astrouts.code_location_to_struct.insert(range_s.begin.clone(), struct_a.clone());

  // Scala: structA (return it)
  struct_a
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
fn get_interface_type<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _interface_s: &InterfaceS<'a, 's>) -> ITemplataType {
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
fn translate_interface<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _interface_s: &InterfaceS<'a, 's>) -> InterfaceA<'a, 's> {
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
fn translate_impl<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _impl_s: &ImplS<'a, 's>) -> ImplA<'a, 's> {
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
fn translate_export<'s>(&self, _astrouts: &mut Astrouts<'a, 's>, _env: &EnvironmentA<'a, 's>, _export_s: &ExportAsS<'a, 's>) -> ExportAsA<'a> {
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
fn translate_function<'s>(&self, astrouts: &mut Astrouts<'a, 's>, env: &EnvironmentA<'a, 's>, function_s: &'s FunctionS<'a, 's>) -> FunctionA<'a, 's> {
  let range_s = function_s.range.clone();
  let name_s = function_s.name.clone();
  let attributes_s = function_s.attributes;
  let identifying_runes_s = function_s.generic_params;
  let rune_to_explicit_type = &function_s.rune_to_predicted_type;
  let tyype = &function_s.tyype;
  let params_s = function_s.params;
  let maybe_ret_coord_rune = &function_s.maybe_ret_coord_rune;
  let rules_with_implicitly_coercing_lookups_s = function_s.rules;
  let body_s = function_s.body;

  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      identifying_runes_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      rune_to_explicit_type.clone(),
      params_s,
      rules_with_implicitly_coercing_lookups_s,
      env,
    );

  let mut rune_a_to_type: HashMap<IRuneS<'a>, ITemplataType> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;
  // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
  // loose. We intentionally ignored the types of the things they're looking up, so we could know
  // what types we *expect* them to be, so we could coerce.
  // That coercion is good, but lets make it more explicit.
  let mut rule_builder: Vec<IRulexSR<'a>> = Vec::new();
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    astrouts,
    env,
    range_s: range_s.clone(),
  };
  match explicify_lookups(
    &rune_typing_env,
    &mut rune_a_to_type,
    &mut rule_builder,
    rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed"),
  }

  let mut attributes: Vec<IFunctionAttributeS<'a>> = attributes_s.to_vec();
  attributes.push(IFunctionAttributeS::UserFunction(UserFunctionS));

  FunctionA {
    range: range_s,
    name: name_s,
    attributes,
    tyype: tyype.clone(),
    generic_parameters: identifying_runes_s.to_vec(),
    rune_to_type: rune_a_to_type,
    params: params_s.to_vec(),
    maybe_ret_coord_rune: maybe_ret_coord_rune.clone(),
    rules: rule_builder,
    body: body_s.clone(),
  }
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
fn calculate_rune_types<'s>(
  &self,
  astrouts: &mut Astrouts<'a, 's>,
  range_s: RangeS<'a>,
  identifying_runes_s: Vec<IRuneS<'a>>,
  rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  params_s: &[ParameterS<'a>],
  rules_s: &[IRulexSR<'a>],
  env: &EnvironmentA<'a, 's>,
) -> HashMap<IRuneS<'a>, ITemplataType> {
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    astrouts,
    env,
    range_s: range_s.clone(),
  };
  let mut rune_s_to_pre_known_type_a: HashMap<IRuneS<'a>, ITemplataType> = rune_to_explicit_type;
  for param in params_s {
    if let Some(ref coord_rune) = param.pattern.coord_rune {
      rune_s_to_pre_known_type_a.insert(coord_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {}));
    }
  }
  let rune_type_solver = crate::postparsing::rune_type_solver::RuneTypeSolver {
    interner: self.interner,
  };
  let rune_s_to_type = rune_type_solver.solve_rune_type(
    self.global_options.sanity_check,
    &rune_typing_env,
    vec![range_s.clone()],
    false,
    rules_s,
    &identifying_runes_s,
    true,
    rune_s_to_pre_known_type_a,
  );
  match rune_s_to_type {
    Ok(t) => t,
    Err(_e) => panic!("CouldntSolveRulesA"),
  }
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
fn translate_program<'s>(&self, code_map: PackageCoordinateMap<'a, ProgramS<'a, 's>>, supplied_functions: Vec<FunctionA<'a, 's>>, supplied_interfaces: Vec<InterfaceA<'a, 's>>) -> ProgramA<'a, 's> {
  let env = EnvironmentA {
    maybe_name: None,
    maybe_parent_env: None,
    code_map,
    rune_to_type: HashMap::new(),
  };

  // If something is absent from the map, we haven't started evaluating it yet
  // If there is a None in the map, we started evaluating it
  // If there is a Some in the map, we know the type
  // If we are asked to evaluate something but there is already a None in the map, then we are
  // caught in a cycle.
  let mut astrouts = Astrouts {
    code_location_to_maybe_type: HashMap::new(),
    code_location_to_struct: HashMap::new(),
    code_location_to_interface: HashMap::new(),
  };

  let structs_a: Vec<StructA<'a, 's>> = env.structs_s().into_iter().map(|s| self.translate_struct(&mut astrouts, &env, s)).collect();

  let interfaces_a: Vec<InterfaceA<'a, 's>> = env.interfaces_s().into_iter().map(|i| self.translate_interface(&mut astrouts, &env, i)).collect();

  let impls_a: Vec<ImplA<'a, 's>> = env.impls_s().into_iter().map(|im| self.translate_impl(&mut astrouts, &env, im)).collect();

  let functions_a: Vec<FunctionA<'a, 's>> = env.functions_s().into_iter().map(|f| self.translate_function(&mut astrouts, &env, f)).collect();

  let exports_a: Vec<ExportAsA<'a>> = env.exports_s().into_iter().map(|e| self.translate_export(&mut astrouts, &env, e)).collect();

  ProgramA {
    structs: structs_a,
    interfaces: supplied_interfaces.into_iter().chain(interfaces_a).collect(),
    impls: impls_a,
    functions: supplied_functions.into_iter().chain(functions_a).collect(),
    exports: exports_a,
  }
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
pub fn run_pass<'s>(
  &self,
  separate_programs_s: FileCoordinateMap<'a, ProgramS<'a, 's>>,
) -> Result<PackageCoordinateMap<'a, ProgramA<'a, 's>>, Box<dyn ICompileErrorA<'a> + 'a>> {
  // Merge FileCoordinateMap into PackageCoordinateMap by flattening files per package
  let mut merged_program_s = PackageCoordinateMap::<ProgramS<'a, 's>>::new();
  for (package_coord, file_coords) in &separate_programs_s.package_coord_to_file_coords {
    let programs_s: Vec<&ProgramS<'a, 's>> = file_coords
      .iter()
      .map(|fc| separate_programs_s.file_coord_to_contents.get(fc).unwrap())
      .collect();
    // Flatten all files' contents into one ProgramS per package
    let structs: Vec<StructS<'a, 's>> = programs_s.iter().flat_map(|p| p.structs.iter().cloned()).collect();
    let interfaces: Vec<InterfaceS<'a, 's>> = programs_s.iter().flat_map(|p| p.interfaces.iter().cloned()).collect();
    let impls: Vec<ImplS<'a, 's>> = programs_s.iter().flat_map(|p| p.impls.iter().cloned()).collect();
    let functions: Vec<&'s FunctionS<'a, 's>> = programs_s.iter().flat_map(|p| p.implemented_functions.iter().cloned()).collect();
    let exports: Vec<ExportAsS<'a, 's>> = programs_s.iter().flat_map(|p| p.exports.iter().cloned()).collect();
    let imports: Vec<crate::postparsing::ast::ImportS<'a, 's>> = programs_s.iter().flat_map(|p| p.imports.iter().cloned()).collect();
    // Leak vecs into slices since ProgramS holds slices
    let merged = ProgramS {
      structs: structs.leak(),
      interfaces: interfaces.leak(),
      impls: impls.leak(),
      implemented_functions: functions.leak(),
      exports: exports.leak(),
      imports: imports.leak(),
    };
    merged_program_s.put(package_coord, merged);
  }

  let supplied_functions: Vec<FunctionA<'a, 's>> = Vec::new();
  let supplied_interfaces: Vec<InterfaceA<'a, 's>> = Vec::new();
  let _program_a =
    self.translate_program(merged_program_s, supplied_functions, supplied_interfaces);

  // Group results by package coordinate
  // In Scala, each item type is grouped by its package coordinate from range/name fields:
  //   structsA.groupBy(_.range.begin.file.packageCoordinate)
  //   interfacesA.groupBy(_.name.range.begin.file.packageCoordinate)
  //   functionsA.groupBy(_.name.packageCoordinate)
  //   implsA.groupBy(_.name.packageCoordinate)
  //   exportsA.groupBy(_.range.file.packageCoordinate)
  // This will be implemented when translate_program is migrated.
  panic!("Unimplemented: run_pass grouping logic (translate_program returned, which is unexpected since it's a stub)")
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
}
/*
*/
// mig: struct HigherTypingCompilation
pub struct HigherTypingCompilation<'a, 'ctx, 'p, 's> {
  global_options: GlobalOptions,
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  scout_compilation: ScoutCompilation<'a, 'ctx, 'p, 's>,
  astrouts_cache: Option<PackageCoordinateMap<'a, ProgramA<'a, 's>>>,
}

// mig: impl HigherTypingCompilation
impl<'a, 'ctx, 'p, 's> HigherTypingCompilation<'a, 'ctx, 'p, 's>
where
    'a: 'ctx,
    'a: 'p,
    'a: 's,
{
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
  // From HigherTypingPass.scala lines 793-799
  pub fn new(
    interner: &'ctx Interner<'a>,
    keywords: &'ctx Keywords<'a>,
    packages_to_build: Vec<&'a PackageCoordinate<'a>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'a, HashMap<String, String>>,
    global_options: GlobalOptions,
    parser_arena: &'p bumpalo::Bump,
    scout_arena: &'s bumpalo::Bump,
  ) -> Self {
    let scout_compilation = ScoutCompilation::new(
      interner,
      keywords,
      packages_to_build,
      package_to_contents_resolver,
      global_options.clone(),
      parser_arena,
      scout_arena,
    );

    HigherTypingCompilation {
      global_options,
      interner,
      keywords,
      scout_compilation,
      astrouts_cache: None,
    }
  }

// mig: fn get_code_map
pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
  self.scout_compilation.get_code_map()
}

/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getCodeMap()
*/
// mig: fn get_parseds
pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'a, (FileP<'a, 'p>, Vec<RangeL>)>, FailedParse<'a>> {
  self.scout_compilation.get_parseds()
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = scoutCompilation.getParseds()
*/
// mig: fn get_vpst_map
pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'a, String>, FailedParse<'a>> {
  self.scout_compilation.get_vpst_map()
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getVpstMap()
*/
// mig: fn get_scoutput
fn get_scoutput(&mut self) -> Result<FileCoordinateMap<'a, ProgramS<'a, 's>>, ICompileErrorS<'a>> {
  Ok(self.scout_compilation.get_scoutput()?.clone())
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = scoutCompilation.getScoutput()

*/
// mig: fn get_astrouts
pub fn get_astrouts(&mut self) -> Result<&PackageCoordinateMap<'a, ProgramA<'a, 's>>, Box<dyn ICompileErrorA<'a> + 'a>> {
  if self.astrouts_cache.is_some() {
    return Ok(self.astrouts_cache.as_ref().unwrap());
  }
  let scoutput = self.scout_compilation.expect_scoutput().clone();
  let higher_typing_pass = HigherTypingPass::new(
    self.global_options.clone(),
    self.interner,
    self.keywords,
  );
  let astrouts = higher_typing_pass.run_pass(scoutput)?;
  self.astrouts_cache = Some(astrouts);
  Ok(self.astrouts_cache.as_ref().unwrap())
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
pub fn expect_astrouts(&mut self) -> &PackageCoordinateMap<'a, ProgramA<'a, 's>> {
  match self.get_astrouts() {
    Ok(x) => x,
    Err(_e) => {
      panic!("HigherTypingCompilation.expect_astrouts failed")
    }
  }
}
} // end impl HigherTypingCompilation
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

// Concrete IRuneTypeSolverEnv for the higher typing pass.
// All 6 Scala anonymous `new IRuneTypeSolverEnv` in this file close over (astrouts, env, rangeS)
// and delegate to lookupType. This struct captures those same fields.
struct HigherTypingRuneTypeSolverEnv<'a, 's, 'env> {
  astrouts: &'env Astrouts<'a, 's>,
  env: &'env EnvironmentA<'a, 's>,
  range_s: RangeS<'a>,
}

impl<'a, 's, 'env> IRuneTypeSolverEnv<'a> for HigherTypingRuneTypeSolverEnv<'a, 's, 'env> {
  fn lookup(
    &self,
    _range: RangeS<'a>,
    _name: IImpreciseNameS<'a>,
  ) -> Result<IRuneTypeSolverLookupResult<'a>, IRuneTypingLookupFailedError<'a>> {
    // Scala: lookupType(astrouts, env, rangeS, name).mapError({...})
    // lookup_type is still a panic stub on HigherTypingPass, so just panic here too.
    panic!("HigherTypingRuneTypeSolverEnv::lookup not yet implemented")
  }
}
