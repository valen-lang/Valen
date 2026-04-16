/*
package dev.vale.typing.macros

import dev.vale.highertyping.{FunctionA, StructA}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules._
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CompileErrorExceptionT, CompilerOutputs, CouldntFindFunctionToCallT, InheritBoundsFromTypeItself, OverloadResolver, TemplataCompiler, TypingPassOptions, UseBoundsFromContainer, ast}
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, ConstructTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{FunctionEnvEntry, FunctionEnvironmentT}
import dev.vale.typing.names.{CitizenNameT, CitizenTemplateNameT, FunctionNameT, ICitizenNameT, ICitizenTemplateNameT, IFunctionNameT, IFunctionTemplateNameT, INameT, ITemplateNameT, IdT, NameTranslator, KindPlaceholderNameT}
import dev.vale.{Err, Interner, Keywords, Ok, PackageCoordinate, Profiler, RangeS, StrI, vassert, vassertSome, vcurious, vimpl}
import dev.vale.typing.types._
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.ConstructorNameS
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.ast._
import dev.vale.typing.env.PackageEnvironmentT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function.{DestructorCompiler, FunctionCompilerCore}
import dev.vale.typing.infer.CouldntFindFunction
import dev.vale.typing.templata.ITemplataT.expectMutability
import dev.vale.typing.templata._
import dev.vale.typing.types.InterfaceTT

import scala.collection.mutable

*/
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compilation::*;
use crate::interner::Interner;
use crate::typing::array_compiler::*;
use crate::typing::overload_resolver::*;
use crate::typing::templata_compiler::*;
use crate::typing::function::destructor_compiler::*;
use crate::typing::citizen::struct_compiler::*;
use crate::typing::names::name_translator::*;
use crate::typing::infer::compiler_solver::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rules::rules::*;

// mig: struct StructConstructorMacro
pub struct StructConstructorMacro<'s, 'ctx, 't> {
  pub opts: TypingPassOptions<'s>,
  pub interner: Interner<'s>,
  pub keywords: Keywords<'s>,
  pub name_translator: NameTranslator<'s, 't>,
  pub destructor_compiler: DestructorCompiler<'s, 't>,
}

// mig: impl StructConstructorMacro
impl<'s, 'ctx, 't> StructConstructorMacro<'s, 'ctx, 't> {}

/*
class StructConstructorMacro(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
  nameTranslator: NameTranslator,
  destructorCompiler: DestructorCompiler,
) extends IOnStructDefinedMacro with IFunctionBodyMacro {

  val generatorId: StrI = keywords.structConstructorGenerator

  val macroName: StrI = keywords.DeriveStructConstructor

*/
// mig: fn get_struct_sibling_entries
pub fn get_struct_sibling_entries<'p, 's>(
  &self,
  struct_name: IdT<INameT>,
  struct_a: StructA<'s>,
) -> Vec<(IdT<INameT>, FunctionEnvEntry)> {
  panic!("Unimplemented: get_struct_sibling_entries");
}

/*
  override def getStructSiblingEntries(structName: IdT[INameT], structA: StructA):
  Vector[(IdT[INameT], FunctionEnvEntry)] = {
    if (structA.members.collect({ case VariadicStructMemberS(_, _, _) => }).nonEmpty) {
      // Dont generate constructors for variadic structs, not supported yet.
      // Only one we have right now is tuple, which has its own special syntax for constructing.
      return Vector()
    }
    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()
    val rules = mutable.ArrayBuffer[IRulexSR]()

    // We dont need these, they really just contain bounds and stuff, which we'd inherit from our parameters anyway.
    // However, if we leave it out, then this (from an IRAGP test):
    //   struct Bork<T, Y> where T = Y { t T; y Y; }
    // thing's constructor would be:
    //   func Bork<T, Y>(t T, y Y) Bork<T, Y> { ... }
    // and it fails to resolve that return type there because it doesn't meet the struct's conditions, because it didn't
    // repeat the rules from the struct's header, specifically the T = Y rule.
    // So, we just include all the rules from the constructor's header.
    // If we ever need to drop that functionality (the T = Y nonsense) then we can probably take out the inheriting of
    // the header rules.
    runeToType ++= structA.headerRuneToType
    rules ++= structA.headerRules

    // We include these because they become our parameters. If a struct contains a Opt<^MyNode<T>> we want those two
    // CallSRs in our function rules too.
    runeToType ++= structA.membersRuneToType
    rules ++= structA.memberRules


    val retRune = RuneUsage(structA.name.range, ReturnRuneS())
    runeToType += (retRune.rune -> CoordTemplataType())
    val structNameRange = structA.name.range
    val structGenericRune = StructNameRuneS(structA.name)
    runeToType += (structGenericRune -> structA.tyype)
    rules += LookupSR(structNameRange, RuneUsage(structNameRange, structGenericRune), structA.name.getImpreciseName(interner))

    val structKindRune = RuneUsage(structNameRange, ImplicitCoercionKindRuneS(structNameRange, structGenericRune))
    runeToType += (structKindRune.rune -> KindTemplataType())
    rules += CallSR(structNameRange, structKindRune, RuneUsage(structNameRange, structGenericRune), structA.genericParameters.map(_.rune).toVector)

    rules += CoerceToCoordSR(structNameRange, retRune, structKindRune)

    val params =
      structA.members.zipWithIndex.flatMap({
        case (NormalStructMemberS(range, name, variability, typeRune), index) => {
          val capture = CaptureS(interner.intern(CodeVarNameS(name)), false)
          Vector(ParameterS(range, None, false, AtomSP(range, Some(capture), Some(typeRune), None)))
        }
        case (VariadicStructMemberS(range, variability, typeRune), index) => {
          Vector()
        }
      })
    runeToType ++= params.flatMap(_.pattern.coordRune.map(_.rune)).map(_ -> CoordTemplataType())

    val functionA =
      FunctionA(
        structA.range,
        interner.intern(ConstructorNameS(structA.name)),
        Vector(),
        TemplateTemplataType(structA.tyype.paramTypes, FunctionTemplataType()),
        structA.genericParameters,
        runeToType.toMap,
        params,
        Some(retRune),
        rules.toVector,
        GeneratedBodyS(generatorId))

    Vector(
      structName.copy(localName = nameTranslator.translateNameStep(functionA.name)) ->
        FunctionEnvEntry(functionA))
  }


*/
// mig: fn generate_function_body
pub fn generate_function_body<'s, 't>(
  &self,
  env: FunctionEnvironmentT<'s, 't>,
  coutputs: CompilerOutputs<'s, 't>,
  generator_id: StrI<'s>,
  life: LocationInFunctionEnvironmentT<'s>,
  call_range: Vec<RangeS<'s>>,
  call_location: LocationInDenizen<'s>,
  origin_function: Option<FunctionA<'s>>,
  param_coords: Vec<ParameterT<'s, 't>>,
  maybe_ret_coord: Option<CoordT<'s, 't>>,
) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
  panic!("Unimplemented: generate_function_body");
}

/*
  override def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
      callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val Some(CoordT(_, _, structTT @ StructTT(_))) = maybeRetCoord
    val definition = coutputs.lookupStruct(structTT.id)
    val placeholderSubstituter =
      TemplataCompiler.getPlaceholderSubstituter(
        opts.globalOptions.sanityCheck,
        interner,
        keywords,
        env.denizenTemplateId,
        structTT.id,
        // We only know about this struct from the return type, we don't get to inherit any of its
        // bounds or guarantees from. Satisfy them from our environment instead.
        UseBoundsFromContainer(
          definition.instantiationBoundParams,
          vassertSome(coutputs.getInstantiationBounds(structTT.id))))
    val members =
      definition.members.map({
        case NormalStructMemberT(name, _, ReferenceMemberTypeT(tyype)) => {
          (name, placeholderSubstituter.substituteForCoord(coutputs, tyype))
        }
        case NormalStructMemberT(name, variability, AddressMemberTypeT(tyype)) => vcurious()
        case VariadicStructMemberT(name, tyype) => vimpl()
      })

    val constructorId = env.id
    vassert(constructorId.localName.parameters.size == members.size)
    val constructorParams =
      members.map({ case (name, coord) => ParameterT(name, None, false, coord) })
    val mutability =
      StructCompiler.getMutability(
        opts.globalOptions.sanityCheck,
        interner, keywords, coutputs, env.denizenTemplateId, RegionT(), structTT,
        // Not entirely sure if this is right, but it's consistent with using it for the return kind
        // and its the more conservative option so we'll go with it for now.
        UseBoundsFromContainer(
          definition.instantiationBoundParams,
          vassertSome(coutputs.getInstantiationBounds(structTT.id))))
    val constructorReturnOwnership =
      mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, MutabilityTemplataType()) => OwnT
      }
    val constructorReturnType = CoordT(constructorReturnOwnership, RegionT(), structTT)

    // not virtual because how could a constructor be virtual
    val header =
      ast.FunctionHeaderT(
        constructorId,
        Vector.empty,
        constructorParams,
        constructorReturnType,
        Some(env.templata))

    val body =
      BlockTE(
        ReturnTE(
          ConstructTE(
            structTT,
            constructorReturnType,
            constructorParams.zipWithIndex.map({ case (p, index) => ArgLookupTE(index, p.tyype) }))))
    (header, body)
  }
}
*/
