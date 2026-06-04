// AFTERM: instead of get_astrouts, lets make a .build() method that consumes self and returns
// the compiled data. that will nicely destroy the compilation struct which is holding a bunch of
// other things hostage via reference.
// AFTERM: rename Astrouts

use crate::compile_options::GlobalOptions;
use crate::higher_typing::ast::{
    ExportAsA, FunctionA, ImplA, InterfaceA, ProgramA, StructA,
};
use crate::higher_typing::astronomer_error_reporter::{
    CouldntFindTypeA, ICompileErrorA, ILookupFailedErrorA, TooManyMatchingTypesA,
};
use crate::interner::StrI;
use crate::scout_arena::ScoutArena;
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
use crate::postparsing::names::{IImpreciseNameS, IImplDeclarationNameS, INameS, IRuneS, IStructDeclarationNameS};
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
use crate::postparsing::rune_type_solver::PrimitiveRuneTypeSolverLookupResult;
use crate::postparsing::rules::rules::{MaybeCoercingLookupSR, MaybeCoercingCallSR, LookupSR, CallSR, CoerceToCoordSR};
use crate::postparsing::names::{IRuneValS, ImplicitCoercionKindRuneValS};
use crate::postparsing::names::ImplicitCoercionTemplateRuneValS;
use crate::postparsing::rune_type_solver::CitizenRuneTypeSolverLookupResult;
use crate::postparsing::rune_type_solver::TemplataLookupResult;
use crate::postparsing::rune_type_solver::{RuneTypingTooManyMatchingTypes, RuneTypingCouldntFindType};
use crate::postparsing::rune_type_solver::RuneTypeSolver;
use crate::parse_arena::ParseArena;
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
pub struct Astrouts<'s> {
  code_location_to_maybe_type: std::collections::HashMap<CodeLocationS<'s>, Option<ITemplataType<'s>>>,
  code_location_to_struct: std::collections::HashMap<CodeLocationS<'s>, &'s StructA<'s>>,
  code_location_to_interface: std::collections::HashMap<CodeLocationS<'s>, &'s InterfaceA<'s>>,
}

// mig: impl Astrouts
impl<'s> Astrouts<'s> {
}
/*
case class Astrouts(
  codeLocationToMaybeType: mutable.HashMap[CodeLocationS, Option[ITemplataType]],
  codeLocationToStruct: mutable.HashMap[CodeLocationS, StructA],
  codeLocationToInterface: mutable.HashMap[CodeLocationS, InterfaceA])

*/
// mig: struct EnvironmentA
pub struct EnvironmentA<'s> {
  maybe_name: Option<&'s INameS<'s>>,
  maybe_parent_env: Option<&'s EnvironmentA<'s>>,
  code_map: &'s PackageCoordinateMap<'s, ProgramS<'s>>,
  rune_to_type: std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>,
}

// mig: impl EnvironmentA
impl<'s> EnvironmentA<'s> {
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
  pub fn structs_s(&self) -> Vec<&'s StructS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.structs.iter().copied()).collect()
  }
/*
  val structsS: Vector[StructS] = codeMap.packageCoordToContents.values.flatMap(_.structs).toVector
*/
  pub fn interfaces_s(&self) -> Vec<&'s InterfaceS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.interfaces.iter().copied()).collect()
  }
/*
    val interfacesS: Vector[InterfaceS] = codeMap.packageCoordToContents.values.flatMap(_.interfaces).toVector
*/
  pub fn impls_s(&self) -> Vec<&'s ImplS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.impls.iter().copied()).collect()
  }
/*
    val implsS: Vector[ImplS] = codeMap.packageCoordToContents.values.flatMap(_.impls).toVector
*/
  pub fn functions_s(&self) -> Vec<&'s FunctionS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.implemented_functions.iter().copied()).collect()
  }
/*
    val functionsS: Vector[FunctionS] = codeMap.packageCoordToContents.values.flatMap(_.implementedFunctions).toVector
*/
  pub fn exports_s(&self) -> Vec<&'s ExportAsS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.exports.iter().copied()).collect()
  }
/*
    val exportsS: Vector[ExportAsS] = codeMap.packageCoordToContents.values.flatMap(_.exports).toVector
*/
  pub fn imports_s(&self) -> Vec<&'s ImportS<'s>> {
    self.code_map.package_coord_to_contents.values().flat_map(|p| p.imports.iter().copied()).collect()
  }
/*
    val imports: Vector[ImportS] = codeMap.packageCoordToContents.values.flatMap(_.imports).toVector
*/

// mig: fn add_runes
fn add_runes(&self, new_rune_to_type: std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>) -> EnvironmentA<'s> {
  let mut merged = self.rune_to_type.clone();
  merged.extend(new_rune_to_type);
  EnvironmentA {
    maybe_name: self.maybe_name.clone(),
    maybe_parent_env: self.maybe_parent_env,
    code_map: self.code_map,
    rune_to_type: merged,
  }
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
pub fn explicify_lookups<'s: 's, E: IRuneTypeSolverEnv<'s>>(env: &E, scout_arena: &ScoutArena<'s>, rune_a_to_type: &mut HashMap<IRuneS<'s>, ITemplataType<'s>>, rule_builder: &mut Vec<IRulexSR<'s>>, all_rules_with_implicitly_coercing_lookups_s: Vec<IRulexSR<'s>>) -> Result<(), IRuneTypingLookupFailedError<'s>> {
  // Only two rules' results can be coerced: LookupSR and CallSR.
  // Let's look for those and rewrite them to put an explicit coercion in there.
  for rule in all_rules_with_implicitly_coercing_lookups_s {
    match rule {
      IRulexSR::MaybeCoercingCall(MaybeCoercingCallSR { range, result_rune, template_rune, args }) => {
        let expected_type = rune_a_to_type.get(&result_rune.rune).expect("vassertSome").clone();
        let actual_type = match rune_a_to_type.get(&template_rune.rune).expect("vassertSome") {
          ITemplataType::TemplateTemplataType(ttt) => (*ttt.return_type).clone(),
          _ => panic!("vwat"),
        };
        if actual_type == expected_type {
          rule_builder.push(IRulexSR::Call(CallSR { range, result_rune, template_rune, args }));
        } else {
          match (&actual_type, &expected_type) {
            (ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => {
              let kind_rune_s = scout_arena.intern_rune(IRuneValS::ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS {
                range: range.clone(),
                original_coord_rune: result_rune.rune.clone(),
              }));
              let kind_rune = RuneUsage { range: range.clone(), rune: kind_rune_s.clone() };
              rune_a_to_type.insert(kind_rune_s, ITemplataType::KindTemplataType(KindTemplataType {}));
              rule_builder.push(IRulexSR::Call(CallSR { range: range.clone(), result_rune: kind_rune.clone(), template_rune, args }));
              rule_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR { range, coord_rune: result_rune, kind_rune }));
            }
            _ => panic!("vimpl"),
          }
        }
      }
      IRulexSR::MaybeCoercingLookup(MaybeCoercingLookupSR { range, rune: result_rune, name }) => {
        let desired_type = rune_a_to_type.get(&result_rune.rune).expect("vassertSome").clone();
        let actual_lookup_result = env.lookup(range.clone(), name.clone())?;

        match actual_lookup_result {
          IRuneTypeSolverLookupResult::Primitive(PrimitiveRuneTypeSolverLookupResult { tyype: _ }) => {
            match &desired_type {
              ITemplataType::CoordTemplataType(_) => {
                coerce_kind_lookup_to_coord(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name);
              }
              ITemplataType::KindTemplataType(_) => {
                rule_builder.push(IRulexSR::Lookup(LookupSR { range, rune: result_rune, name }));
              }
              ITemplataType::TemplateTemplataType(ttt) => {
                assert!(!ttt.param_types.is_empty());
                rule_builder.push(IRulexSR::Lookup(LookupSR { range, rune: result_rune, name }));
              }
              _ => panic!("FoundPrimitiveDidntMatchExpectedType not yet migrated as IRuneTypingLookupFailedError variant")
            }
          }
          IRuneTypeSolverLookupResult::Citizen(citizen) => {
            let citizen_template_type = match citizen.tyype {
              ITemplataType::TemplateTemplataType(ttt) => ttt,
              _ => panic!("CitizenRuneTypeSolverLookupResult tyype should be TemplateTemplataType"),
            };
            match &desired_type {
              ITemplataType::KindTemplataType(_) => {
                coerce_kind_template_lookup_to_kind(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name, citizen_template_type.clone());
              }
              ITemplataType::CoordTemplataType(_) => {
                coerce_kind_template_lookup_to_coord(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name, citizen_template_type.clone());
              }
              ITemplataType::TemplateTemplataType(ttt) => {
                assert!(!ttt.param_types.is_empty());
                rule_builder.push(IRulexSR::Lookup(LookupSR { range, rune: result_rune, name }));
              }
              _ => panic!("FoundTemplataDidntMatchExpectedTypeA not yet migrated as IRuneTypingLookupFailedError variant")
            }
          }
          IRuneTypeSolverLookupResult::Templata(t) => {
            let actual_type = t.templata;
            match (&actual_type, &desired_type) {
              (x, y) if x == y => {
                rule_builder.push(IRulexSR::Lookup(LookupSR { range, rune: result_rune, name }));
              }
              (ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => {
                coerce_kind_lookup_to_coord(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name);
              }
              (ITemplataType::TemplateTemplataType(ttt), ITemplataType::KindTemplataType(_))
                  if matches!(ttt.return_type, ITemplataType::KindTemplataType(_)) => {
                coerce_kind_template_lookup_to_kind(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name, ttt.clone());
              }
              (ITemplataType::TemplateTemplataType(ttt), ITemplataType::CoordTemplataType(_))
                  if matches!(ttt.return_type, ITemplataType::KindTemplataType(_)) => {
                coerce_kind_template_lookup_to_coord(scout_arena, rune_a_to_type, rule_builder, range, result_rune, &name, ttt.clone());
              }
              _ => panic!("explicify_lookups TemplataLookupResult: unexpected coercion from {:?} to {:?}", actual_type, desired_type),
            }
          }
        }
      }
      rule => {
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
fn coerce_kind_lookup_to_coord<'s>(scout_arena: &ScoutArena<'s>, rune_a_to_type: &mut std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>, rule_builder: &mut Vec<IRulexSR<'s>>, range: RangeS<'s>, result_rune: RuneUsage<'s>, name: &IImpreciseNameS<'s>) {
  let kind_rune_s = scout_arena.intern_rune(IRuneValS::ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS {
    range: range.clone(),
    original_coord_rune: result_rune.rune.clone(),
  }));
  let kind_rune = RuneUsage { range: range.clone(), rune: kind_rune_s.clone() };
  rune_a_to_type.insert(kind_rune_s, ITemplataType::KindTemplataType(KindTemplataType {}));
  rule_builder.push(IRulexSR::Lookup(LookupSR { range: range.clone(), rune: kind_rune.clone(), name: name.clone() }));
  rule_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR { range, coord_rune: result_rune, kind_rune }));
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
fn coerce_kind_template_lookup_to_kind<'s>(scout_arena: &ScoutArena<'s>, rune_a_to_type: &mut std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>, rule_builder: &mut Vec<IRulexSR<'s>>, range: RangeS<'s>, result_rune: RuneUsage<'s>, name: &IImpreciseNameS<'s>, actual_template_type: TemplateTemplataType<'s>) {
  let template_rune_s = scout_arena.intern_rune(IRuneValS::ImplicitCoercionTemplateRune(ImplicitCoercionTemplateRuneValS {
    range: range.clone(),
    original_kind_rune: result_rune.rune.clone(),
  }));
  let template_rune = RuneUsage { range: range.clone(), rune: template_rune_s.clone() };
  rune_a_to_type.insert(template_rune_s, ITemplataType::TemplateTemplataType(actual_template_type));
  rule_builder.push(IRulexSR::Lookup(LookupSR { range: range.clone(), rune: template_rune.clone(), name: name.clone() }));
  rule_builder.push(IRulexSR::Call(CallSR { range, result_rune, template_rune, args: &[] }));
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
fn coerce_kind_template_lookup_to_coord<'s>(scout_arena: &ScoutArena<'s>, rune_a_to_type: &mut std::collections::HashMap<IRuneS<'s>, ITemplataType<'s>>, rule_builder: &mut Vec<IRulexSR<'s>>, range: RangeS<'s>, result_rune: RuneUsage<'s>, name: &IImpreciseNameS<'s>, ttt: TemplateTemplataType<'s>) {

  let template_rune_s = scout_arena.intern_rune(IRuneValS::ImplicitCoercionTemplateRune(ImplicitCoercionTemplateRuneValS {
    range: range.clone(),
    original_kind_rune: result_rune.rune.clone(),
  }));
  let template_rune = RuneUsage { range: range.clone(), rune: template_rune_s.clone() };

  let kind_rune_s = scout_arena.intern_rune(IRuneValS::ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS {
    range: range.clone(),
    original_coord_rune: result_rune.rune.clone(),
  }));
  let kind_rune = RuneUsage { range: range.clone(), rune: kind_rune_s.clone() };

  rune_a_to_type.insert(template_rune_s, ITemplataType::TemplateTemplataType(ttt));
  rune_a_to_type.insert(kind_rune_s, ITemplataType::KindTemplataType(KindTemplataType {}));
  rule_builder.push(IRulexSR::Lookup(LookupSR { range: range.clone(), rune: template_rune.clone(), name: name.clone() }));
  rule_builder.push(IRulexSR::Call(CallSR { range: range.clone(), result_rune: kind_rune.clone(), template_rune: template_rune.clone(), args: &[] }));
  rule_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR { range, coord_rune: result_rune, kind_rune }));
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
pub struct HigherTypingPass<'s, 'ctx> {
  global_options: GlobalOptions,
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  primitives: std::collections::HashMap<StrI<'s>, ITemplataType<'s>>,
}

// mig: impl HigherTypingPass
impl<'s, 'ctx> HigherTypingPass<'s, 'ctx> {
  pub fn new(
    global_options: GlobalOptions,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
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
      param_types: scout_arena.alloc_slice_copy(&[
        ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ]),
      return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    }));
    primitives.insert(keywords.static_array, ITemplataType::TemplateTemplataType(TemplateTemplataType {
      param_types: scout_arena.alloc_slice_copy(&[
        ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
        ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
        ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ]),
      return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    }));
    HigherTypingPass {
      global_options,
      scout_arena,
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
// Returns whether the imprecise name could be referring to the absolute name.
// See MINAAN for what we're doing here.
// mig: fn imprecise_name_matches_absolute_name
fn imprecise_name_matches_absolute_name(&self, needle_imprecise_name_s: &IImpreciseNameS, absolute_name: &INameS) -> bool {
  match (needle_imprecise_name_s, absolute_name) {
    (IImpreciseNameS::CodeName(code_name), INameS::TopLevelStructDeclaration(s)) => {
      s.name == code_name.name
    }
    (IImpreciseNameS::CodeName(code_name), INameS::TopLevelInterfaceDeclaration(i)) => {
      i.name == code_name.name
    }
    (IImpreciseNameS::RuneName(_), _) => false,
    _ => panic!("vimpl"),
  }
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
// See MINAAN for what we're doing here.
// mig: fn lookup_types
fn lookup_types(&self, astrouts: &Astrouts<'s>, env: &EnvironmentA<'s>, needle_imprecise_name_s: &IImpreciseNameS<'s>) -> Vec<IRuneTypeSolverLookupResult<'s>> {

  match needle_imprecise_name_s {
    IImpreciseNameS::CodeName(_) => {}
    IImpreciseNameS::RuneName(_) => {}
    _ => panic!("Unexpected imprecise name type in lookup_types"),
  }

  if let IImpreciseNameS::CodeName(code_name) = needle_imprecise_name_s {
    if let Some(x) = self.primitives.get(&code_name.name) {
      return vec![IRuneTypeSolverLookupResult::Primitive(PrimitiveRuneTypeSolverLookupResult { tyype: x.clone() })];
    }
  }

  if let IImpreciseNameS::RuneName(rune_name) = needle_imprecise_name_s {
    if let Some(tyype) = env.rune_to_type.get(&rune_name.rune) {
      return vec![IRuneTypeSolverLookupResult::Templata(TemplataLookupResult { templata: tyype.clone() })];
    }
  }

  let near_struct_types: Vec<_> = env.structs_s().iter()
    .filter(|s| self.imprecise_name_matches_absolute_name(needle_imprecise_name_s, &INameS::TopLevelStructDeclaration(s.name)))
    .map(|s| IRuneTypeSolverLookupResult::Citizen(CitizenRuneTypeSolverLookupResult { tyype: ITemplataType::TemplateTemplataType(s.tyype.clone()), generic_params: s.generic_params }))
    .collect();
  let near_interface_types: Vec<_> = env.interfaces_s().iter()
    .filter(|i| self.imprecise_name_matches_absolute_name(needle_imprecise_name_s, &INameS::TopLevelInterfaceDeclaration(i.name)))
    .map(|i| IRuneTypeSolverLookupResult::Citizen(CitizenRuneTypeSolverLookupResult { tyype: ITemplataType::TemplateTemplataType(i.tyype.clone()), generic_params: i.generic_params }))
    .collect();
  let result: Vec<IRuneTypeSolverLookupResult<'s>> = near_struct_types.into_iter().chain(near_interface_types).collect();

  if !result.is_empty() {
    result
  } else {
    match &env.maybe_parent_env {
      None => vec![],
      Some(parent_env) => self.lookup_types(astrouts, parent_env, needle_imprecise_name_s),
    }
  }
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
fn lookup_type(&self, astrouts: &Astrouts<'s>, env: &EnvironmentA<'s>, range: RangeS<'s>, name: &IImpreciseNameS<'s>) -> Result<IRuneTypeSolverLookupResult<'s>, ILookupFailedErrorA<'s>> {
  let results = self.lookup_types(astrouts, env, name);
  let mut distinct = Vec::new();
  for r in results {
    if !distinct.contains(&r) {
      distinct.push(r);
    }
  }
  match distinct.len() {
    0 => Err(ILookupFailedErrorA::CouldntFindType(CouldntFindTypeA { range, name: name.clone() })),
    1 => Ok(distinct.into_iter().next().unwrap()),
    _ => Err(ILookupFailedErrorA::TooManyMatchingTypes(TooManyMatchingTypesA { range, name: name.clone() })),
  }
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
fn translate_struct(&self, astrouts: &mut Astrouts<'s>, env: &EnvironmentA<'s>, struct_s: &StructS<'s>) -> &'s StructA<'s> {
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
    internal_methods: internal_methods_s,
  } = struct_s;

  // Check cache
  if let Some(value) = astrouts.code_location_to_struct.get(&range_s.begin) {
    return *value;
  }

  // Check for cycles
  match astrouts.code_location_to_maybe_type.get(&range_s.begin) {
    Some(Some(_)) => panic!("vwat: already evaluated struct type but missed cache"),
    Some(None) => {
      panic!("Cycle in determining struct type!");
    }
    None => {}
  }
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), None);

  let all_rules_with_implicitly_coercing_lookups_s: Vec<IRulexSR<'s>> =
    header_rules_with_implicitly_coercing_lookups_s.iter().chain(member_rules_with_implicitly_coercing_lookups_s.iter()).cloned().collect();
  let mut all_rune_to_explicit_type: HashMap<IRuneS<'s>, ITemplataType<'s>> = header_rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
  all_rune_to_explicit_type.extend(members_rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())));

  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      generic_parameters_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      all_rune_to_explicit_type,
      &[], // no params for structs
      &all_rules_with_implicitly_coercing_lookups_s,
      env,
    );

  let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;

  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };

  let mut header_rules_builder: Vec<IRulexSR<'s>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut header_rules_builder,
    header_rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for header rules"),
  }

  let mut member_rules_builder: Vec<IRulexSR<'s>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut member_rules_builder,
    member_rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for member rules"),
  }

  // Split rune_a_to_type into header vs member portions
  let mut runes_in_header: std::collections::HashSet<IRuneS<'s>> = std::collections::HashSet::new();
  for gp in generic_parameters_s.iter() {
    runes_in_header.insert(gp.rune.rune.clone());
    if let Some(ref default) = gp.default {
      for rule in default.rules.iter() {
        for ru in rule.rune_usages() {
          runes_in_header.insert(ru.rune.clone());
        }
      }
    }
  }
  for rule in header_rules_builder.iter() {
    for ru in rule.rune_usages() {
      runes_in_header.insert(ru.rune.clone());
    }
  }

  let header_rune_a_to_type = self.scout_arena.alloc_index_map_from_iter(
    rune_a_to_type.iter().filter(|(k, _)| runes_in_header.contains(k)).map(|(k, v)| (k.clone(), v.clone())),
  );
  let members_rune_a_to_type = self.scout_arena.alloc_index_map_from_iter(
    rune_a_to_type.iter().filter(|(k, _)| !runes_in_header.contains(k)).map(|(k, v)| (k.clone(), v.clone())),
  );

  // Shouldnt fail because we got a complete solve earlier
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), Some(ITemplataType::TemplateTemplataType(tyype.clone())));

  for rule in header_rules_builder.iter() {
    if matches!(rule, IRulexSR::MaybeCoercingCall(_)) { panic!("vwat: MaybeCoercingCallSR in header rules after explicify"); }
  }
  for rule in member_rules_builder.iter() {
    if matches!(rule, IRulexSR::MaybeCoercingCall(_)) { panic!("vwat: MaybeCoercingCallSR in member rules after explicify"); }
  }
  let methods_env = env.add_runes(rune_a_to_type.clone());
  let internal_methods_a: Vec<&'s FunctionA<'s>> = internal_methods_s.iter()
    .map(|method| self.translate_function(astrouts, &methods_env, *method))
    .collect();
  let struct_a = self.scout_arena.alloc(StructA::new(
    range_s.clone(),
    IStructDeclarationNameS::TopLevelStructDeclarationName((*name_s).clone()),
    attributes_s,
    *weakable,
    mutability_rune_s.clone(),
    *maybe_predicted_mutability,
    tyype.clone(),
    generic_parameters_s,
    header_rune_a_to_type,
    self.scout_arena.alloc_slice_from_vec(header_rules_builder),
    members_rune_a_to_type,
    self.scout_arena.alloc_slice_from_vec(member_rules_builder),
    members,
    self.scout_arena.alloc_slice_from_vec(internal_methods_a),
  ));
  astrouts.code_location_to_struct.insert(range_s.begin.clone(), struct_a);
  struct_a
}
/*
  def translateStruct(
    astrouts: Astrouts,
    env: EnvironmentA,
    structS: StructS):
  StructA = {
    val StructS(rangeS, nameS, attributesS, weakable, genericParametersS, mutabilityRuneS, maybePredictedMutability, tyype, headerRuneToExplicitType, headerPredictedRuneToType, headerRulesWithImplicitlyCoercingLookupsS, membersRuneToExplicitType, membersPredictedRuneToType, memberRulesWithImplicitlyCoercingLookupsS, members, internalMethodsS) = structS

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
    val methodsEnv =
      env.addRunes(runeAToType.toMap)
    val internalMethodsA =
      internalMethodsS.map(method => {
        translateFunction(astrouts, methodsEnv, method)
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
        members,
        internalMethodsA)
    astrouts.codeLocationToStruct.put(rangeS.begin, structA)
    structA
  }

*/
// mig: fn get_interface_type
fn get_interface_type(&self, _astrouts: &mut Astrouts<'s>, _env: &EnvironmentA<'s>, _interface_s: &InterfaceS<'s>) -> ITemplataType<'s> {
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
fn translate_interface(&self, astrouts: &mut Astrouts<'s>, env: &EnvironmentA<'s>, interface_s: &InterfaceS<'s>) -> &'s InterfaceA<'s> {
  let InterfaceS {
    range: range_s,
    name: name_s,
    attributes: attributes_s,
    weakable,
    generic_params: generic_parameters_s,
    rune_to_explicit_type,
    mutability_rune: mutability_rune_s,
    maybe_predicted_mutability,
    predicted_rune_to_type: _,
    tyype,
    rules: rules_with_implicitly_coercing_lookups_s,
    internal_methods: internal_methods_s,
  } = interface_s;

  // Check cache
  if let Some(value) = astrouts.code_location_to_interface.get(&range_s.begin) {
    return *value;
  }

  // Check for cycles
  match astrouts.code_location_to_maybe_type.get(&range_s.begin) {
    Some(Some(_)) => panic!("vwat: already evaluated interface type but missed cache"),
    Some(None) => {
      panic!("Cycle in determining interface type!");
    }
    None => {}
  }
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), None);

  let rune_a_to_type_with_implicitly_coercing_lookups =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      generic_parameters_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
      &[],
      rules_with_implicitly_coercing_lookups_s,
      env,
    );

  // getOrDie because we should have gotten a complete solve
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), Some(ITemplataType::TemplateTemplataType(tyype.clone())));

  let methods_env = env.add_runes(rune_a_to_type_with_implicitly_coercing_lookups.clone());
  let internal_methods_a: Vec<&'s FunctionA<'s>> =
    internal_methods_s.iter().map(|method| {
      self.translate_function(astrouts, &methods_env, method)
    }).collect();

  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      generic_parameters_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
      &[],
      rules_with_implicitly_coercing_lookups_s,
      env,
    );

  let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;
  // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit loose...

  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };

  let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut rule_builder,
    rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for interface"),
  }

  let interface_a = self.scout_arena.alloc(InterfaceA::new(
    range_s.clone(),
    name_s,
    attributes_s,
    *weakable,
    mutability_rune_s.clone(),
    *maybe_predicted_mutability,
    tyype.clone(),
    generic_parameters_s,
    self.scout_arena.alloc_index_map_from_iter(rune_a_to_type.into_iter()),
    self.scout_arena.alloc_slice_from_vec(rule_builder),
    self.scout_arena.alloc_slice_from_vec(internal_methods_a),
  ));
  astrouts.code_location_to_interface.insert(range_s.begin.clone(), interface_a);
  interface_a
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
fn translate_impl(&self, astrouts: &mut Astrouts<'s>, env: &EnvironmentA<'s>, impl_s: &ImplS<'s>) -> &'s ImplA<'s> {
  let ImplS {
    range: range_s,
    name: name_s,
    user_specified_identifying_runes: identifying_runes_s,
    rules: rules_with_implicitly_coercing_lookups_s,
    rune_to_explicit_type,
    tyype,
    struct_kind_rune: struct_kind_rune_s,
    sub_citizen_imprecise_name,
    interface_kind_rune: interface_kind_rune_s,
    super_interface_imprecise_name,
  } = impl_s;

  // Scala creates runeTypingEnv here, but Rust can't because it borrows astrouts immutably
  // while calculate_rune_types needs &mut astrouts. Created below after mutable borrows end.

  let mut rune_to_explicit_type_with_kinds: HashMap<IRuneS<'s>, ITemplataType<'s>> = rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
  rune_to_explicit_type_with_kinds.insert(struct_kind_rune_s.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {}));
  rune_to_explicit_type_with_kinds.insert(interface_kind_rune_s.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {}));

  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      identifying_runes_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      rune_to_explicit_type_with_kinds,
      &[], // Vector()
      rules_with_implicitly_coercing_lookups_s,
      env,
    );

  // getOrDie because we should have gotten a complete solve
  astrouts.code_location_to_maybe_type.insert(range_s.begin.clone(), Some(tyype.clone()));

  let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;
  // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
  // loose. We intentionally ignored the types of the things they're looking up, so we could know
  // what types we *expect* them to be, so we could coerce.
  // That coercion is good, but lets make it more explicit.

  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };

  let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut rule_builder,
    rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed for impl"),
  }

  self.scout_arena.alloc(ImplA::new(
    range_s.clone(),
    IImplDeclarationNameS::ImplDeclarationName(name_s.clone()),
    identifying_runes_s,
    self.scout_arena.alloc_slice_from_vec(rule_builder),
    self.scout_arena.alloc_index_map_from_iter(rune_a_to_type.into_iter()),
    struct_kind_rune_s.clone(),
    sub_citizen_imprecise_name.clone(),
    interface_kind_rune_s.clone(),
    super_interface_imprecise_name.clone(),
  ))
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
fn translate_export(&self, astrouts: &mut Astrouts<'s>, env: &EnvironmentA<'s>, export_s: &ExportAsS<'s>) -> &'s ExportAsA<'s> {
  let range_s = export_s.range.clone();
  let rules_with_implicitly_coercing_lookups_s = export_s.rules;
  let rune = export_s.rune.clone();
  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      Vec::new(),
      std::iter::once((rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {}))).collect(),
      &[],
      rules_with_implicitly_coercing_lookups_s,
      env,
    );
  let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;
  let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut rule_builder,
    rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed"),
  }
  self.scout_arena.alloc(ExportAsA {
    range: range_s,
    exported_name: export_s.exported_name,
    rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
    rune_to_type: self.scout_arena.alloc_index_map_from_iter(rune_a_to_type.into_iter()),
    type_rune: rune,
  })
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
fn translate_function(&self, astrouts: &mut Astrouts<'s>, env: &EnvironmentA<'s>, function_s: &'s FunctionS<'s>) -> &'s FunctionA<'s> {
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
  // Scala creates runeTypingEnv here, but Rust can't because it borrows astrouts immutably
  // while calculate_rune_types needs &mut astrouts. Created below after mutable borrows end.

  let rune_a_to_type_with_implicitly_coercing_lookups_s =
    self.calculate_rune_types(
      astrouts,
      range_s.clone(),
      identifying_runes_s.iter().map(|gp| gp.rune.rune.clone()).collect(),
      rune_to_explicit_type.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
      params_s,
      rules_with_implicitly_coercing_lookups_s,
      env,
    );

  let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
    rune_a_to_type_with_implicitly_coercing_lookups_s;
  // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
  // loose. We intentionally ignored the types of the things they're looking up, so we could know
  // what types we *expect* them to be, so we could coerce.
  // That coercion is good, but lets make it more explicit.
  let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };
  match explicify_lookups(
    &rune_typing_env,
    self.scout_arena,
    &mut rune_a_to_type,
    &mut rule_builder,
    rules_with_implicitly_coercing_lookups_s.to_vec(),
  ) {
    Ok(()) => {},
    Err(_e) => panic!("explicify_lookups failed"),
  }

  let mut attributes: Vec<IFunctionAttributeS<'s>> = attributes_s.to_vec();
  attributes.push(IFunctionAttributeS::UserFunction(UserFunctionS));

  self.scout_arena.alloc(FunctionA::new(
    range_s,
    name_s,
    self.scout_arena.alloc_slice_from_vec(attributes),
    tyype.clone(),
    identifying_runes_s,
    self.scout_arena.alloc_index_map_from_iter(rune_a_to_type.into_iter()),
    params_s,
    maybe_ret_coord_rune.clone(),
    self.scout_arena.alloc_slice_from_vec(rule_builder),
    *body_s,
  ))
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
fn calculate_rune_types(
  &self,
  astrouts: &mut Astrouts<'s>,
  range_s: RangeS<'s>,
  identifying_runes_s: Vec<IRuneS<'s>>,
  rune_to_explicit_type: HashMap<IRuneS<'s>, ITemplataType<'s>>,
  params_s: &[ParameterS<'s>],
  rules_s: &[IRulexSR<'s>],
  env: &EnvironmentA<'s>,
) -> HashMap<IRuneS<'s>, ITemplataType<'s>> {
  let rune_typing_env = HigherTypingRuneTypeSolverEnv {
    pass: self,
    astrouts,
    env,
    range_s: range_s.clone(),
  };
  let mut rune_s_to_pre_known_type_a: HashMap<IRuneS<'s>, ITemplataType<'s>> = rune_to_explicit_type;
  for param in params_s {
    if let Some(ref coord_rune) = param.pattern.coord_rune {
      rune_s_to_pre_known_type_a.insert(coord_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {}));
    }
  }
  let rune_type_solver = RuneTypeSolver {
    scout_arena: self.scout_arena,
  };
  // Violation: RSMSCPX: Scala passes globalOptions.useOptimizedSolver as 2nd arg to solve; Rust's solve_rune_type omits it
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
fn translate_program(&self, code_map: &'s PackageCoordinateMap<'s, ProgramS<'s>>, supplied_functions: Vec<&'s FunctionA<'s>>, supplied_interfaces: Vec<&'s InterfaceA<'s>>) -> ProgramA<'s> {
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

  let structs_a: Vec<&'s StructA<'s>> = env.structs_s().into_iter().map(|s| self.translate_struct(&mut astrouts, &env, s)).collect();

  let interfaces_a: Vec<&'s InterfaceA<'s>> = env.interfaces_s().into_iter().map(|i| self.translate_interface(&mut astrouts, &env, i)).collect();

  let impls_a: Vec<&'s ImplA<'s>> = env.impls_s().into_iter().map(|im| self.translate_impl(&mut astrouts, &env, im)).collect();

  let functions_a: Vec<&'s FunctionA<'s>> = env.functions_s().into_iter().map(|f| self.translate_function(&mut astrouts, &env, f)).collect();

  let exports_a: Vec<&'s ExportAsA<'s>> = env.exports_s().into_iter().map(|e| self.translate_export(&mut astrouts, &env, e)).collect();

  ProgramA {
    structs: self.scout_arena.alloc_slice_from_vec(structs_a),
    interfaces: self.scout_arena.alloc_slice_from_vec(supplied_interfaces.into_iter().chain(interfaces_a).collect()),
    impls: self.scout_arena.alloc_slice_from_vec(impls_a),
    functions: self.scout_arena.alloc_slice_from_vec(supplied_functions.into_iter().chain(functions_a).collect()),
    exports: self.scout_arena.alloc_slice_from_vec(exports_a),
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
pub fn run_pass(
  &self,
  separate_programs_s: FileCoordinateMap<'s, ProgramS<'s>>,
) -> Result<PackageCoordinateMap<'s, ProgramA<'s>>, ICompileErrorA<'s>> {
  // Merge FileCoordinateMap into PackageCoordinateMap by flattening files per package
  let mut merged_program_s = PackageCoordinateMap::<ProgramS<'s>>::new();
  for (package_coord, file_coords) in &separate_programs_s.package_coord_to_file_coords {
    let programs_s: Vec<&ProgramS<'s>> = file_coords
      .iter()
      .map(|fc| separate_programs_s.file_coord_to_contents.get(fc).unwrap())
      .collect();
    // Flatten all files' contents into one ProgramS per package
    let structs: Vec<&'s StructS<'s>> = programs_s.iter().flat_map(|p| p.structs.iter().copied()).collect();
    let interfaces: Vec<&'s InterfaceS<'s>> = programs_s.iter().flat_map(|p| p.interfaces.iter().copied()).collect();
    let impls: Vec<&'s ImplS<'s>> = programs_s.iter().flat_map(|p| p.impls.iter().copied()).collect();
    let functions: Vec<&'s FunctionS<'s>> = programs_s.iter().flat_map(|p| p.implemented_functions.iter().copied()).collect();
    let exports: Vec<&'s ExportAsS<'s>> = programs_s.iter().flat_map(|p| p.exports.iter().copied()).collect();
    let imports: Vec<&'s ImportS<'s>> = programs_s.iter().flat_map(|p| p.imports.iter().copied()).collect();
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

  let merged_program_s = self.scout_arena.alloc(merged_program_s);
  let supplied_functions: Vec<&'s FunctionA<'s>> = Vec::new();
  let supplied_interfaces: Vec<&'s InterfaceA<'s>> = Vec::new();
  let program_a =
    self.translate_program(merged_program_s, supplied_functions, supplied_interfaces);

  // Group results by package coordinate
  let ProgramA { structs: structs_a, interfaces: interfaces_a, impls: impls_a, functions: functions_a, exports: exports_a } = program_a;

  let mut package_to_structs_a: HashMap<&'s PackageCoordinate<'s>, Vec<&'s StructA<'s>>> = HashMap::new();
  for &s in structs_a {
    package_to_structs_a.entry(s.range.begin.file.package_coord).or_default().push(s);
  }

  let mut package_to_interfaces_a: HashMap<&'s PackageCoordinate<'s>, Vec<&'s InterfaceA<'s>>> = HashMap::new();
  for &i in interfaces_a {
    package_to_interfaces_a.entry(i.name.range.begin.file.package_coord).or_default().push(i);
  }

  let mut package_to_functions_a: HashMap<&'s PackageCoordinate<'s>, Vec<&'s FunctionA<'s>>> = HashMap::new();
  for &f in functions_a {
    package_to_functions_a.entry(f.name.package_coordinate()).or_default().push(f);
  }

  let mut package_to_impls_a: HashMap<&'s PackageCoordinate<'s>, Vec<&'s ImplA<'s>>> = HashMap::new();
  for &im in impls_a {
    package_to_impls_a.entry(im.name.package_coordinate()).or_default().push(im);
  }

  let mut package_to_exports_a: HashMap<&'s PackageCoordinate<'s>, Vec<&'s ExportAsA<'s>>> = HashMap::new();
  for &e in exports_a {
    package_to_exports_a.entry(e.range.begin.file.package_coord).or_default().push(e);
  }

  let mut all_packages: std::collections::HashSet<&'s PackageCoordinate<'s>> = std::collections::HashSet::new();
  all_packages.extend(package_to_structs_a.keys());
  all_packages.extend(package_to_interfaces_a.keys());
  all_packages.extend(package_to_functions_a.keys());
  all_packages.extend(package_to_impls_a.keys());
  all_packages.extend(package_to_exports_a.keys());

  let mut result = PackageCoordinateMap::<ProgramA<'s>>::new();
  for paackage in all_packages {
    let contents = ProgramA {
      structs: self.scout_arena.alloc_slice_from_vec(package_to_structs_a.remove(paackage).unwrap_or_default()),
      interfaces: self.scout_arena.alloc_slice_from_vec(package_to_interfaces_a.remove(paackage).unwrap_or_default()),
      impls: self.scout_arena.alloc_slice_from_vec(package_to_impls_a.remove(paackage).unwrap_or_default()),
      functions: self.scout_arena.alloc_slice_from_vec(package_to_functions_a.remove(paackage).unwrap_or_default()),
      exports: self.scout_arena.alloc_slice_from_vec(package_to_exports_a.remove(paackage).unwrap_or_default()),
    };
    result.put(paackage, contents);
  }
  Ok(result)
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
pub struct HigherTypingCompilation<'s, 'ctx, 'p> {
  global_options: GlobalOptions,
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub keywords: &'ctx Keywords<'s>,
  scout_compilation: ScoutCompilation<'s, 'ctx, 'p>,
  astrouts_cache: Option<PackageCoordinateMap<'s, ProgramA<'s>>>,
}

// mig: impl HigherTypingCompilation
impl<'s, 'ctx, 'p> HigherTypingCompilation<'s, 'ctx, 'p>
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
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    package_to_contents_resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
    global_options: GlobalOptions,
  ) -> Self {
    let scout_compilation = ScoutCompilation::new(
      scout_arena,
      keywords,
      parser_keywords,
      parse_arena,
      packages_to_build,
      package_to_contents_resolver,
      global_options.clone(),
    );

    HigherTypingCompilation {
      global_options,
      scout_arena,
      keywords,
      scout_compilation,
      astrouts_cache: None,
    }
  }

// mig: fn get_code_map
pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
  self.scout_compilation.get_code_map()
}

/*
  def getCodeMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getCodeMap()
*/
// mig: fn get_parseds
pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
  self.scout_compilation.get_parseds()
}
/*
  def getParseds(): Result[FileCoordinateMap[(FileP, Vector[RangeL])], FailedParse] = scoutCompilation.getParseds()
*/
// mig: fn get_vpst_map
pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
  self.scout_compilation.get_vpst_map()
}
/*
  def getVpstMap(): Result[FileCoordinateMap[String], FailedParse] = scoutCompilation.getVpstMap()
*/
// mig: fn get_scoutput
pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
  self.scout_compilation.get_scoutput()
}
/*
  def getScoutput(): Result[FileCoordinateMap[ProgramS], ICompileErrorS] = scoutCompilation.getScoutput()

*/
// mig: fn get_astrouts
pub fn get_astrouts(&mut self) -> Result<&PackageCoordinateMap<'s, ProgramA<'s>>, ICompileErrorA<'s>> {
  if self.astrouts_cache.is_some() {
    return Ok(self.astrouts_cache.as_ref().unwrap());
  }
  let scoutput = self.scout_compilation.expect_scoutput().clone();
  let higher_typing_pass = HigherTypingPass::new(
    self.global_options.clone(),
    self.scout_arena,
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
pub fn expect_astrouts(&mut self) -> &PackageCoordinateMap<'s, ProgramA<'s>> {
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
struct HigherTypingRuneTypeSolverEnv<'s, 'ctx, 'env> {
  pass: &'env HigherTypingPass<'s, 'ctx>,
  astrouts: &'env Astrouts<'s>,
  env: &'env EnvironmentA<'s>,
  range_s: RangeS<'s>,
}
/*
Guardian: disable-all
*/


impl<'s, 'ctx, 'env> IRuneTypeSolverEnv<'s> for HigherTypingRuneTypeSolverEnv<'s, 'ctx, 'env> {
  fn lookup(
    &self,
    _range: RangeS<'s>,
    name: IImpreciseNameS<'s>,
  ) -> Result<IRuneTypeSolverLookupResult<'s>, IRuneTypingLookupFailedError<'s>> {
    self.pass.lookup_type(self.astrouts, self.env, self.range_s.clone(), &name)
      .map_err(|e| match e {
        ILookupFailedErrorA::CouldntFindType(c) => {
          IRuneTypingLookupFailedError::CouldntFindType(RuneTypingCouldntFindType {
            range: c.range,
            name: c.name,
          })
        }
        ILookupFailedErrorA::TooManyMatchingTypes(t) => {
          IRuneTypingLookupFailedError::TooManyMatchingTypes(RuneTypingTooManyMatchingTypes {
            range: t.range,
            name: t.name,
          })
        }
      })
  }
  /*
  Guardian: disable-all
  */
}
/*
Guardian: disable-all
*/
