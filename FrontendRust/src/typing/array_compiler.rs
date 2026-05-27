use std::collections::HashMap;

use crate::utils::range::RangeS;

use crate::postparsing::names::*;

use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::postparsing::ast::{LocationInDenizen, IRegionMutabilityS};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::itemplatatype::{IntegerTemplataType, MutabilityTemplataType, VariabilityTemplataType, ITemplataType};
use crate::postparsing::rules::rules::*;
use crate::typing::compiler::Compiler;
use crate::typing::names::names::*;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::postparsing::itemplatatype::CoordTemplataType;
use crate::postparsing::rune_type_solver::solve_rune_type;
use crate::typing::infer_compiler::{CompleteResolveSolve, InferEnv, InitialKnown};
use crate::typing::templata::templata::{expect_integer, expect_mutability, expect_variability};
use std::collections::HashSet;
use crate::typing::types::types::KindT;
use crate::typing::ast::expressions::DestroyStaticSizedArrayIntoFunctionTE;
use crate::typing::templata::templata::expect_coord_templata;

/*
package dev.vale.typing

import dev.vale.parsing.ast.MutableP
import dev.vale.postparsing._
import dev.vale.postparsing.rules.{IRulexSR, RuneParentEnvLookupSR, RuneUsage}
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.types._
import dev.vale._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import OverloadResolver._
import dev.vale.highertyping.HigherTypingPass.explicifyLookups
import dev.vale.solver.FailedSolve
import dev.vale.typing.ast.{DestroyImmRuntimeSizedArrayTE, DestroyStaticSizedArrayIntoFunctionTE, FunctionCallTE, NewImmRuntimeSizedArrayTE, ReferenceExpressionTE, RuntimeSizedArrayLookupTE, StaticArrayFromCallableTE, StaticArrayFromValuesTE, StaticSizedArrayLookupTE}
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.citizen.StructCompilerCore
import dev.vale.typing.function._
import dev.vale.typing.types._
import dev.vale.typing.templata._

import scala.collection.immutable.{List, Set}
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
*/
/*
class ArrayCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    inferCompiler: InferCompiler,
    overloadResolver: OverloadResolver,
    destructorCompiler: DestructorCompiler,
    templataCompiler: TemplataCompiler) {

*/
/*
  val runeTypeSolver = new RuneTypeSolver(interner)

*/
/*
  vassert(overloadResolver != null)
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_static_sized_array_from_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        region: RegionT,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        rules_with_implicitly_coercing_lookups_s: &[IRulexSR<'s>],
        maybe_element_type_rune_a: Option<IRuneS<'s>>,
        size_rune_a: IRuneS<'s>,
        mutability_rune: IRuneS<'s>,
        variability_rune: IRuneS<'s>,
        callable_te: ReferenceExpressionTE<'s, 't>,
    ) -> StaticArrayFromCallableTE<'s, 't> {

        let rune_typing_env = self.create_rune_type_solver_env(calling_env);

        let mut initially_known_runes: HashMap<IRuneS<'s>, ITemplataType<'s>> = HashMap::new();
        initially_known_runes.insert(size_rune_a, ITemplataType::IntegerTemplataType(IntegerTemplataType {}));
        initially_known_runes.insert(mutability_rune, ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}));
        initially_known_runes.insert(variability_rune, ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}));
        if let Some(rune) = maybe_element_type_rune_a {
            initially_known_runes.insert(rune, ITemplataType::CoordTemplataType(CoordTemplataType {}));
        }
        let rune_a_to_type_with_implicitly_coercing_lookups_s =
            solve_rune_type(
                self.scout_arena,
                self.opts.global_options.sanity_check,
                &rune_typing_env,
                parent_ranges.to_vec(),
                false,
                rules_with_implicitly_coercing_lookups_s,
                &[],
                true,
                initially_known_runes,
            ).unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_callable — HigherTypingInferError"));

        let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            HashMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
        let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
        match crate::higher_typing::higher_typing_pass::explicify_lookups(
            &rune_typing_env,
            self.scout_arena,
            &mut rune_a_to_type,
            &mut rule_builder,
            rules_with_implicitly_coercing_lookups_s.to_vec(),
        ) {
            Err(_e) => panic!("implement: evaluate_static_sized_array_from_callable — TooManyTypesWithNameT/CouldntFindTypeT"),
            Ok(()) => {}
        }
        let rules_a = rule_builder;
        // We preprocess out the rune parent env lookups, see MKRFA. The fold here is duplicated
        // from OverloadResolver.scala:311-325; when adding a new expression-scoped solver call
        // site, copy this again (or, preferably, land the shared helper refactor queued in
        // docs/refactor-thoughts/mkrfa-protocol-leak.md so neither copy is needed).
        // (MKRFA preprocessing not yet wired into this Rust path; tracked alongside the audit-trail fold.)

        let initial_knowns: &[InitialKnown<'s, 't>] = &[];
        let initial_sends = &[];

        let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
        let envs = InferEnv {
            original_calling_env: calling_env,
            parent_ranges: parent_ranges_t,
            call_location,
            self_env: IEnvironmentT::from(calling_env),
            context_region: region,
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &rules_a, &rune_a_to_type, parent_ranges, initial_knowns, initial_sends);
        match self.incrementally_solve(envs, coutputs, &mut solver_state, |_coutputs, _solver| false) {
            Err(_f) => panic!("implement: evaluate_static_sized_array_from_callable — TypingPassSolverError"),
            Ok(true) => {}
            Ok(false) => {}
        }
        let CompleteResolveSolve { conclusions: templatas, .. } =
            self.check_resolving_conclusions_and_resolve(
                envs, coutputs, parent_ranges, call_location, &rune_a_to_type, &rules_a, &[], &mut solver_state)
            .unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from check_resolving_conclusions_and_resolve in evaluate_static_sized_array_from_callable"))
            .unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_callable — TypingPassResolvingError"));

        let size = expect_integer(templatas.get(&size_rune_a).copied().expect("vassertSome: sizeRuneA not in templatas"));
        let mutability = expect_mutability(templatas.get(&mutability_rune).copied().expect("vassertSome: mutabilityRune not in templatas"));
        let variability = expect_variability(templatas.get(&variability_rune).copied().expect("vassertSome: variabilityRune not in templatas"));
        let prototype = self.get_array_generator_prototype(
            coutputs, calling_env, parent_ranges, call_location, callable_te, region);
        let ssa_mt = self.resolve_static_sized_array(
            mutability, variability, size, prototype.return_type, region);

        if let Some(element_type_rune_a) = maybe_element_type_rune_a {
            let expected_element_type = self.get_array_element_type(&templatas, element_type_rune_a);
            if prototype.return_type != expected_element_type {
                panic!("implement: evaluate_static_sized_array_from_callable — UnexpectedArrayElementType");
            }
        }

        StaticArrayFromCallableTE {
            array_type: self.typing_interner.alloc(ssa_mt),
            region,
            generator: callable_te,
            generator_method: prototype,
        }
    }
/*
Guardian: temp-disable: SPDMX — MACTX mirror pass: same MKRFA comment + empty initial_knowns issue as the sibling _from_values fn. Adding the audit-trail mirror; surrounding simplification predates the edit. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1373-1779476516288/hook-1373/evaluate_static_sized_array_from_callable--79.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
Guardian: temp-disable: SPDMX — Both deviations are direct mirrors of the in-file twin evaluate_static_sized_array_from_values (array_compiler.rs:381-498): pre-populating initially_known_runes for size/mutability/variability (lines 403-409) and the make_solver_state/incrementally_solve/check_resolving_conclusions_and_resolve triple (lines 460-471) are the established Rust adaptation of solveForResolving in this file. Same author, same shape; this is the precedent. — /Volumes/V/Sylvan/FrontendRust/guardian-logs/request-680-1778981807111/hook-680/evaluate_static_sized_array_from_callable--71.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def evaluateStaticSizedArrayFromCallable(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    region: RegionT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    rulesWithImplicitlyCoercingLookupsS: Vector[IRulexSR],
    maybeElementTypeRuneA: Option[IRuneS],
    sizeRuneA: IRuneS,
    mutabilityRune: IRuneS,
    variabilityRune: IRuneS,
    callableTE: ReferenceExpressionTE):
  StaticArrayFromCallableTE = {
    val runeTypingEnv =
      new IRuneTypeSolverEnv {
        override def lookup(range: RangeS, nameS: IImpreciseNameS):
        Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
          vimpl()
          //          vassertOne(callingEnv.lookupNearestWithImpreciseName(nameS, Set(TemplataLookupContext))).tyype
        }
      }

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      runeTypeSolver.solve(
        opts.globalOptions.sanityCheck,
        opts.globalOptions.useOptimizedSolver,
        runeTypingEnv,
        parentRanges,
        false,
        rulesWithImplicitlyCoercingLookupsS,
        List(),
        true,
        Map()) match {
        case Ok(r) => r
        case Err(e) => throw CompileErrorExceptionT(HigherTypingInferError(parentRanges, e))
      }

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv,
      runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionT(TooManyTypesWithNameT(range :: parentRanges, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionT(CouldntFindTypeT(range :: parentRanges, name))
      case Ok(()) =>
    }
    val rulesA = ruleBuilder.toVector
    // We preprocess out the rune parent env lookups, see MKRFA. The fold here is duplicated
    // from OverloadResolver.scala:311-325; when adding a new expression-scoped solver call
    // site, copy this again (or, preferably, land the shared helper refactor queued in
    // docs/refactor-thoughts/mkrfa-protocol-leak.md so neither copy is needed).
    val (initialKnowns, rulesWithoutRuneParentEnvLookups) =
      rulesA.foldLeft((Vector[InitialKnown](), Vector[IRulexSR]()))({
        case ((previousConclusions, remainingRules), RuneParentEnvLookupSR(_, rune)) => {
          val templata =
            vassertSome(
              callingEnv.lookupNearestWithImpreciseName(
                interner.intern(RuneNameS(rune.rune)), Set(TemplataLookupContext)))
          val newConclusions = previousConclusions :+ InitialKnown(rune, templata)
          (newConclusions, remainingRules)
        }
        case ((previousConclusions, remainingRules), rule) => {
          (previousConclusions, remainingRules :+ rule)
        }
      })
    val CompleteResolveSolve(templatas, _) =
      inferCompiler.solveForResolving(
        InferEnv(callingEnv, parentRanges, callLocation, callingEnv, region),
        coutputs,
        rulesWithoutRuneParentEnvLookups,
        runeAToType.toMap,
        parentRanges,
        callLocation,
        Vector(),
        initialKnowns,
        Vector()) match {
        case Err(e) => throw CompileErrorExceptionT(TypingPassResolvingError(parentRanges, e))
        case Ok(c) => c
      }

    val size = ITemplataT.expectInteger(vassertSome(templatas.get(sizeRuneA)))
    val mutability = ITemplataT.expectMutability(vassertSome(templatas.get(mutabilityRune)))
    val variability = ITemplataT.expectVariability(vassertSome(templatas.get(variabilityRune)))
    val prototype =
      overloadResolver.getArrayGeneratorPrototype(
        coutputs, callingEnv, parentRanges, callLocation, callableTE, region)
    val ssaMT = resolveStaticSizedArray(mutability, variability, size, prototype.returnType, region)

    maybeElementTypeRuneA.foreach(elementTypeRuneA => {
      val expectedElementType = getArrayElementType(templatas, elementTypeRuneA)
      if (prototype.returnType != expectedElementType) {
        throw CompileErrorExceptionT(UnexpectedArrayElementType(parentRanges, expectedElementType, prototype.returnType))
      }
    })

    val expr2 = ast.StaticArrayFromCallableTE(ssaMT, region, callableTE, prototype)
    expr2
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_runtime_sized_array_from_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &NodeEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        rules_with_implicitly_coercing_lookups_s: &[IRulexSR<'s>],
        maybe_element_type_rune: Option<IRuneS<'s>>,
        mutability_rune: IRuneS<'s>,
        size_te: ReferenceExpressionTE<'s, 't>,
        maybe_callable_te: Option<ReferenceExpressionTE<'s, 't>>,
    ) -> ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: evaluate_runtime_sized_array_from_callable");
    }
/*
  def evaluateRuntimeSizedArrayFromCallable(
    coutputs: CompilerOutputs,
    callingEnv: NodeEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    rulesWithImplicitlyCoercingLookupsS: Vector[IRulexSR],
    maybeElementTypeRune: Option[IRuneS],
    mutabilityRune: IRuneS,
    sizeTE: ReferenceExpressionTE,
    maybeCallableTE: Option[ReferenceExpressionTE]):
  ReferenceExpressionTE = {

    val runeTypingEnv = TemplataCompiler.createRuneTypeSolverEnv(callingEnv)

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      runeTypeSolver.solve(
        opts.globalOptions.sanityCheck,
        opts.globalOptions.useOptimizedSolver,
        runeTypingEnv,
        parentRanges,
        false,
        rulesWithImplicitlyCoercingLookupsS,
        List(),
        true,
        Map(mutabilityRune -> MutabilityTemplataType()) ++
            maybeElementTypeRune.map(_ -> CoordTemplataType())) match {
        case Ok(r) => r
        case Err(e) => throw CompileErrorExceptionT(HigherTypingInferError(parentRanges, e))
      }

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv,
      runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionT(TooManyTypesWithNameT(range :: parentRanges, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionT(CouldntFindTypeT(range :: parentRanges, name))
      case Ok(()) =>
    }
    val rulesA = ruleBuilder.toVector
    // We preprocess out the rune parent env lookups, see MKRFA. The fold here is duplicated
    // from OverloadResolver.scala:311-325; when adding a new expression-scoped solver call
    // site, copy this again (or, preferably, land the shared helper refactor queued in
    // docs/refactor-thoughts/mkrfa-protocol-leak.md so neither copy is needed).
    val (initialKnowns, rulesWithoutRuneParentEnvLookups) =
      rulesA.foldLeft((Vector[InitialKnown](), Vector[IRulexSR]()))({
        case ((previousConclusions, remainingRules), RuneParentEnvLookupSR(_, rune)) => {
          val templata =
            vassertSome(
              callingEnv.lookupNearestWithImpreciseName(
                interner.intern(RuneNameS(rune.rune)), Set(TemplataLookupContext)))
          val newConclusions = previousConclusions :+ InitialKnown(rune, templata)
          (newConclusions, remainingRules)
        }
        case ((previousConclusions, remainingRules), rule) => {
          (previousConclusions, remainingRules :+ rule)
        }
      })
//    val CompleteCompilerSolve(_, templatas, _, Vector()) =
//      inferCompiler.solveExpectComplete(
//        InferEnv(callingEnv, parentRanges, callLocation, callingEnv, region),
//        coutputs, rulesA, runeAToType.toMap, parentRanges,
//        callLocation, initialKnowns, Vector(), true, true, Vector())
    val rules = rulesWithoutRuneParentEnvLookups
    val runeToType = runeAToType.toMap
    val invocationRange = parentRanges
    val initialSends = Vector()

    val envs = InferEnv(callingEnv, parentRanges, callLocation,callingEnv, region)
    val solver =
      inferCompiler.makeSolverState(
        envs, coutputs, rules, runeToType, invocationRange, initialKnowns, initialSends)

    // Incrementally solve and add default generic parameters (and context region).
    inferCompiler.incrementallySolve(
      envs, coutputs, solver,
      (solver) => {
        // TODO(regions): Sometimes add default region rune
        false
      }) match {
      case Err(f @ FailedSolve(_, _, _, _, err)) => {
        throw CompileErrorExceptionT(TypingPassSolverError(invocationRange, f))
      }
      case Ok(true) =>
      case Ok(false) => // Incomplete, will be detected as SolveIncomplete below.
    }

    val CompleteResolveSolve(templatas, _) =
      inferCompiler.checkResolvingConclusionsAndResolve(envs, coutputs, invocationRange, callLocation, runeToType, rules, Vector(), solver) match {
        case Err(e) => throw CompileErrorExceptionT(TypingPassResolvingError(invocationRange, e))
        case Ok(i) => (i)
      }

    val mutability = ITemplataT.expectMutability(vassertSome(templatas.get(mutabilityRune)))

//    val variability = getArrayVariability(templatas, variabilityRune)

    // MSAE guard removed per @BRRZ. When the user omits the element type,
    // maybeElementTypeRune is None; the ImmutableT branch below handles that via
    // `.foreach` (no-op on None, the element type comes directly from
    // `getArrayGeneratorPrototype`), and the MutableT branch's `findFunction("Array",...)`
    // goes through the stdlib Array<M,E,G>(n int, generator G) where func(&G,int)E
    // bound, whose return rune E is now resolved via the relaxed ResolveSR.

    mutability match {
      case PlaceholderTemplataT(_, MutabilityTemplataType()) => vimpl()
      case MutabilityTemplataT(ImmutableT) => {
        val callableTE =
          maybeCallableTE match {
            case None => {
              throw CompileErrorExceptionT(NewImmRSANeedsCallable(parentRanges))
            }
            case Some(c) => c
          }

        val prototype =
          overloadResolver.getArrayGeneratorPrototype(
            coutputs, callingEnv, parentRanges, callLocation, callableTE, region)
        val rsaMT = resolveRuntimeSizedArray(prototype.returnType, mutability, region)

        maybeElementTypeRune.foreach(elementTypeRuneA => {
          val expectedElementType = getArrayElementType(templatas, elementTypeRuneA)
          if (prototype.returnType != expectedElementType) {
            throw CompileErrorExceptionT(UnexpectedArrayElementType(parentRanges, expectedElementType, prototype.returnType))
          }
        })

        NewImmRuntimeSizedArrayTE(rsaMT, region, sizeTE, callableTE, prototype)
      }
      case MutabilityTemplataT(MutableT) => {
        val StampFunctionSuccess(prototype, conclusions) =
          overloadResolver.findFunction(
            callingEnv
              .addEntries(
                interner,
                Vector(
                  (interner.intern(RuneNameT(CodeRuneS(keywords.M))), TemplataEnvEntry(MutabilityTemplataT(MutableT)))) ++
              maybeElementTypeRune.map(e => {
                (interner.intern(RuneNameT(e)), TemplataEnvEntry(CoordTemplataT(getArrayElementType(templatas, e))))
              })),
            coutputs,
            parentRanges,
            callLocation,
            interner.intern(CodeNameS(keywords.Array)),
            Vector(
              RuneParentEnvLookupSR(parentRanges.head, RuneUsage(parentRanges.head, CodeRuneS(keywords.M)))) ++
            maybeElementTypeRune.map(e => {
              RuneParentEnvLookupSR(parentRanges.head, RuneUsage(parentRanges.head, e))
            }),
            Vector(CodeRuneS(keywords.M)) ++ maybeElementTypeRune,
            Vector.empty,
            region,
            Vector(sizeTE.result.coord) ++
              maybeCallableTE.map(c => c.result.coord),
            Vector(),
            true) match {
            case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(parentRanges, e))
            case Ok(x) => x
          }

        val elementType =
          prototype.returnType.kind match {
            case RuntimeSizedArrayTT(IdT(_, _, RuntimeSizedArrayNameT(_, RawArrayNameT(mutability, elementType, _)))) => {
              if (mutability != MutabilityTemplataT(MutableT)) {
                throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Array function returned wrong mutability!"))
              }
              elementType
            }
            case _ => {
              throw CompileErrorExceptionT(RangedInternalErrorT(parentRanges, "Array function returned wrong type!"))
            }
          }
        maybeElementTypeRune.foreach(elementTypeRuneA => {
          val expectedElementType = getArrayElementType(templatas, elementTypeRuneA)
          if (elementType != expectedElementType) {
            throw CompileErrorExceptionT(
              UnexpectedArrayElementType(parentRanges, expectedElementType, prototype.returnType))
          }
        })
        vassert(coutputs.getInstantiationBounds(prototype.id).nonEmpty)
        val resultTE =
          prototype.returnType
        val callTE =
          FunctionCallTE(prototype, Vector(sizeTE) ++ maybeCallableTE, resultTE)
        callTE
        //        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't construct a mutable runtime array from a callable!"))
      }
    }
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_static_sized_array_from_values(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        rules_with_implicitly_coercing_lookups_s: &[IRulexSR<'s>],
        maybe_element_type_rune_a: Option<IRuneS<'s>>,
        size_rune_a: IRuneS<'s>,
        mutability_rune_a: IRuneS<'s>,
        variability_rune_a: IRuneS<'s>,
        exprs_2: Vec<ReferenceExpressionTE<'s, 't>>,
        region: RegionT,
    ) -> Result<StaticArrayFromValuesTE<'s, 't>, ICompileErrorT<'s, 't>> {

        let rune_typing_env = self.create_rune_type_solver_env(calling_env);

        let mut initially_known_runes: HashMap<IRuneS<'s>, ITemplataType<'s>> = HashMap::new();
        initially_known_runes.insert(size_rune_a, ITemplataType::IntegerTemplataType(IntegerTemplataType {}));
        initially_known_runes.insert(mutability_rune_a, ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}));
        initially_known_runes.insert(variability_rune_a, ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}));
        if let Some(rune) = maybe_element_type_rune_a {
            initially_known_runes.insert(rune, ITemplataType::CoordTemplataType(CoordTemplataType {}));
        }
        // Note: Rust solve_rune_type doesn't accept useOptimizedSolver (pre-existing API difference)
        let rune_a_to_type_with_implicitly_coercing_lookups_s =
            solve_rune_type(
                self.scout_arena,
                self.opts.global_options.sanity_check,
                &rune_typing_env,
                parent_ranges.to_vec(),
                false,
                rules_with_implicitly_coercing_lookups_s,
                &[],
                true,
                initially_known_runes,
            ).unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_values — HigherTypingInferError"));

        let member_types: HashSet<CoordT<'s, 't>> =
            exprs_2.iter().map(|e| e.result().coord).collect();
        if member_types.len() > 1 {
            let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
            let types_t = self.typing_interner.alloc_slice_copy(&member_types.iter().copied().collect::<Vec<_>>());
            return Err(ICompileErrorT::ArrayElementsHaveDifferentTypes { range: parent_ranges_t, types: types_t });
        }
        let member_type = *member_types.iter().next().expect("vassert: memberTypes is empty");

        // VIOLATES @IIIOZ: still HashMap because explicify_lookups takes &mut HashMap.
        let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            HashMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
        let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
        match crate::higher_typing::higher_typing_pass::explicify_lookups(
            &rune_typing_env,
            self.scout_arena,
            &mut rune_a_to_type,
            &mut rule_builder,
            rules_with_implicitly_coercing_lookups_s.to_vec(),
        ) {
            Err(_e) => panic!("implement: evaluate_static_sized_array_from_values — TooManyTypesWithNameT/CouldntFindTypeT"),
            Ok(()) => {}
        }
        let rules_a = rule_builder;
        // We preprocess out the rune parent env lookups, see MKRFA. The fold here is duplicated
        // from OverloadResolver.scala:311-325; when adding a new expression-scoped solver call
        // site, copy this again (or, preferably, land the shared helper refactor queued in
        // docs/refactor-thoughts/mkrfa-protocol-leak.md so neither copy is needed).
        // (MKRFA preprocessing not yet wired into this Rust path; tracked alongside the audit-trail fold.)

        let initial_knowns: &[InitialKnown<'s, 't>] = &[];
        let initial_sends = &[];

        let parent_ranges_t = self.typing_interner.alloc_slice_copy(parent_ranges);
        let envs = InferEnv {
            original_calling_env: calling_env,
            parent_ranges: parent_ranges_t,
            call_location,
            self_env: IEnvironmentT::from(calling_env),
            context_region: region,
        };
        let mut solver_state = self.make_solver_state(
            envs, coutputs, &rules_a, &rune_a_to_type, parent_ranges, initial_knowns, initial_sends);
        match self.incrementally_solve(envs, coutputs, &mut solver_state, |_coutputs, _solver| false) {
            Err(_f) => panic!("implement: evaluate_static_sized_array_from_values — TypingPassSolverError"),
            Ok(true) => {}
            Ok(false) => {}
        }
        let CompleteResolveSolve { conclusions: templatas, .. } =
            self.check_resolving_conclusions_and_resolve(
                envs, coutputs, parent_ranges, call_location, &rune_a_to_type, &rules_a, &[], &mut solver_state)
            .unwrap_or_else(|_e| panic!("Unimplemented: ICompileErrorT from check_resolving_conclusions_and_resolve in evaluate_static_sized_array_from_values"))
            .unwrap_or_else(|_e| panic!("Unimplemented: evaluate_static_sized_array_from_values — TypingPassResolvingError"));

        if let Some(element_type_rune_a) = maybe_element_type_rune_a {
            let expected_element_type = self.get_array_element_type(&templatas, element_type_rune_a);
            if member_type != expected_element_type {
                panic!("implement: evaluate_static_sized_array_from_values — UnexpectedArrayElementType");
            }
        }

        let mutability = expect_mutability(templatas.get(&mutability_rune_a).copied().expect("vassertSome: mutabilityRuneA not in templatas"));
        let variability = expect_variability(templatas.get(&variability_rune_a).copied().expect("vassertSome: variabilityRuneA not in templatas"));

        let static_sized_array_type = self.resolve_static_sized_array(
            mutability, variability, ITemplataT::Integer(exprs_2.len() as i64), member_type, region);
        let ownership = match static_sized_array_type.mutability() {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(p) if matches!(p.tyype, ITemplataType::MutabilityTemplataType(_)) => OwnershipT::Own,
            _ => panic!("vwat"),
        };
        let ssa_ref = self.typing_interner.alloc(static_sized_array_type);
        let ssa_coord = CoordT { ownership, region, kind: KindT::StaticSizedArray(ssa_ref) };
        Ok(StaticArrayFromValuesTE {
            elements: self.typing_interner.alloc_slice_from_vec(exprs_2),
            result_reference: ssa_coord,
            array_type: ssa_ref,
        })
    }
/*
Guardian: temp-disable: SPDMX — MACTX mirror pass: adding the @MKRFA comment near rules_a/initial_knowns documents that this Rust path doesn't yet wire the RuneParentEnvLookupSR preprocessing fold present in canonical Scala. The surrounding empty-initial_knowns simplification predates this edit; the comment is the audit-trail mirror, not a behavioral change. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1373-1779476516288/hook-1373/evaluate_static_sized_array_from_values--510.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def evaluateStaticSizedArrayFromValues(
      coutputs: CompilerOutputs,
      callingEnv: IInDenizenEnvironmentT,
      parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
      rulesWithImplicitlyCoercingLookupsS: Vector[IRulexSR],
      maybeElementTypeRuneA: Option[IRuneS],
      sizeRuneA: IRuneS,
      mutabilityRuneA: IRuneS,
      variabilityRuneA: IRuneS,
      exprs2: Vector[ReferenceExpressionTE],
      region: RegionT):
   StaticArrayFromValuesTE = {

    val runeTypingEnv = TemplataCompiler.createRuneTypeSolverEnv(callingEnv)

    val runeAToTypeWithImplicitlyCoercingLookupsS =
      runeTypeSolver.solve(
        opts.globalOptions.sanityCheck,
        opts.globalOptions.useOptimizedSolver,
        runeTypingEnv,
        parentRanges,
        false,
        rulesWithImplicitlyCoercingLookupsS,
        List(),
        true,
        Map[IRuneS, ITemplataType](
          sizeRuneA -> IntegerTemplataType(),
          mutabilityRuneA -> MutabilityTemplataType(),
          variabilityRuneA -> VariabilityTemplataType()) ++
            (maybeElementTypeRuneA match {
              case Some(rune) => Map(rune -> CoordTemplataType())
              case None => Map()
            })) match {
        case Ok(r) => r
        case Err(e) => throw CompileErrorExceptionT(HigherTypingInferError(parentRanges, e))
      }
    val memberTypes = exprs2.map(_.result.coord).toSet
    if (memberTypes.size > 1) {
      throw CompileErrorExceptionT(ArrayElementsHaveDifferentTypes(parentRanges, memberTypes))
    }
    val memberType = memberTypes.head

    val runeAToType =
      mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
    // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
    // loose. We intentionally ignored the types of the things they're looking up, so we could know
    // what types we *expect* them to be, so we could coerce.
    // That coercion is good, but lets make it more explicit.
    val ruleBuilder = ArrayBuffer[IRulexSR]()
    explicifyLookups(
      runeTypingEnv,
      runeAToType, ruleBuilder, rulesWithImplicitlyCoercingLookupsS) match {
      case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionT(TooManyTypesWithNameT(range :: parentRanges, name))
      case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionT(CouldntFindTypeT(range :: parentRanges, name))
      case Ok(()) =>
    }
    val rulesA = ruleBuilder.toVector
    // We preprocess out the rune parent env lookups, see MKRFA. The fold here is duplicated
    // from OverloadResolver.scala:311-325; when adding a new expression-scoped solver call
    // site, copy this again (or, preferably, land the shared helper refactor queued in
    // docs/refactor-thoughts/mkrfa-protocol-leak.md so neither copy is needed).
    val (initialKnowns, rulesWithoutRuneParentEnvLookups) =
      rulesA.foldLeft((Vector[InitialKnown](), Vector[IRulexSR]()))({
        case ((previousConclusions, remainingRules), RuneParentEnvLookupSR(_, rune)) => {
          val templata =
            vassertSome(
              callingEnv.lookupNearestWithImpreciseName(
                interner.intern(RuneNameS(rune.rune)), Set(TemplataLookupContext)))
          val newConclusions = previousConclusions :+ InitialKnown(rune, templata)
          (newConclusions, remainingRules)
        }
        case ((previousConclusions, remainingRules), rule) => {
          (previousConclusions, remainingRules :+ rule)
        }
      })
//    val CompleteCompilerSolve(_, templatas, _, Vector()) =
//      inferCompiler.solveExpectComplete(
//        envs,
//        coutputs, rulesA, runeAToType.toMap, parentRanges,
//        callLocation, initialKnowns, Vector(), true, true, Vector())
    val rules = rulesWithoutRuneParentEnvLookups
    val runeToType = runeAToType.toMap
    val invocationRange = parentRanges
    val initialSends = Vector()

    val envs = InferEnv(callingEnv, parentRanges, callLocation,callingEnv, region)
    val solver =
      inferCompiler.makeSolverState(
        envs, coutputs, rules, runeToType, invocationRange, initialKnowns, initialSends)
    // Incrementally solve and add default generic parameters (and context region).
    inferCompiler.incrementallySolve(
      envs, coutputs, solver,
      (solver) => {
        // TODO(regions): Sometimes add default region
        false
      }) match {
      case Err(f @ FailedSolve(_, _, _, _, err)) => {
        throw CompileErrorExceptionT(TypingPassSolverError(invocationRange, f))
      }
      case Ok(true) =>
      case Ok(false) => // Incomplete, will be detected as SolveIncomplete below.
    }

    val CompleteResolveSolve(templatas, _) =
      inferCompiler.checkResolvingConclusionsAndResolve(envs, coutputs, invocationRange, callLocation, runeToType, rules, Vector(), solver) match {
        case Err(e) => throw CompileErrorExceptionT(TypingPassResolvingError(invocationRange, e))
        case Ok(i) => (i)
      }


    maybeElementTypeRuneA.foreach(elementTypeRuneA => {
      val expectedElementType = getArrayElementType(templatas, elementTypeRuneA)
      if (memberType != expectedElementType) {
        throw CompileErrorExceptionT(UnexpectedArrayElementType(parentRanges, expectedElementType, memberType))
      }
    })

//    val size = getArraySize(templatas, sizeRuneA)
    val mutability = ITemplataT.expectMutability(vassertSome(templatas.get(mutabilityRuneA)))
    val variability = ITemplataT.expectVariability(vassertSome(templatas.get(variabilityRuneA)))

    val staticSizedArrayType = resolveStaticSizedArray(mutability, variability, IntegerTemplataT(exprs2.size), memberType, region)
    val ownership =
      staticSizedArrayType.mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(_, MutabilityTemplataType()) => OwnT
      }

    val ssaCoord = CoordT(ownership, region, staticSizedArrayType)

    val finalExpr =
      StaticArrayFromValuesTE(
        exprs2, ssaCoord, staticSizedArrayType)
    (finalExpr)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_destroy_static_sized_array_into_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        fate: &'t FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        arr_te: ReferenceExpressionTE<'s, 't>,
        callable_te: ReferenceExpressionTE<'s, 't>,
        context_region: RegionT,
    ) -> Result<DestroyStaticSizedArrayIntoFunctionTE<'s, 't>, ICompileErrorT<'s, 't>> {
        let array_tt = match arr_te.result().coord.kind {
            KindT::StaticSizedArray(s) => s,
            other => panic!("Destroying a non-array with a callable! Destroying: {:?}", other),
        };
        let prototype = self.get_array_consumer_prototype(
            coutputs, fate, range, call_location, callable_te, array_tt.element_type(), context_region)?;
        Ok(DestroyStaticSizedArrayIntoFunctionTE {
            array_expr: arr_te,
            array_type: array_tt,
            consumer: callable_te,
            consumer_method: prototype,
        })
    }
/*
  def evaluateDestroyStaticSizedArrayIntoCallable(
    coutputs: CompilerOutputs,
    fate: FunctionEnvironmentBoxT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    arrTE: ReferenceExpressionTE,
    callableTE: ReferenceExpressionTE,
    contextRegion: RegionT):
  DestroyStaticSizedArrayIntoFunctionTE = {
    val arrayTT =
      arrTE.result.coord match {
        case CoordT(_, region, s @ contentsStaticSizedArrayTT(_, _, _, _, _)) => s
        case other => {
          throw CompileErrorExceptionT(RangedInternalErrorT(range, "Destroying a non-array with a callable! Destroying: " + other))
        }
      }

    val prototype =
      overloadResolver.getArrayConsumerPrototype(
        coutputs, fate, range, callLocation, callableTE, arrayTT.elementType, contextRegion)

    ast.DestroyStaticSizedArrayIntoFunctionTE(
      arrTE,
      arrayTT,
      callableTE,
      prototype)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_destroy_runtime_sized_array_into_callable(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        fate: &FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        arr_te: ReferenceExpressionTE<'s, 't>,
        callable_te: ReferenceExpressionTE<'s, 't>,
        context_region: RegionT,
    ) -> DestroyImmRuntimeSizedArrayTE<'s, 't> {
        panic!("Unimplemented: evaluate_destroy_runtime_sized_array_into_callable");
    }
/*
  def evaluateDestroyRuntimeSizedArrayIntoCallable(
    coutputs: CompilerOutputs,
    fate: FunctionEnvironmentBoxT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    arrTE: ReferenceExpressionTE,
    callableTE: ReferenceExpressionTE,
    contextRegion: RegionT):
  DestroyImmRuntimeSizedArrayTE = {
    val arrayTT =
      arrTE.result.coord match {
        case CoordT(_, region, s @ contentsRuntimeSizedArrayTT(_, _, _)) => s
        case other => {
          throw CompileErrorExceptionT(RangedInternalErrorT(range, "Destroying a non-array with a callable! Destroying: " + other))
        }
      }

    arrayTT.mutability match {
      case PlaceholderTemplataT(_, MutabilityTemplataType()) => {
        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't destroy an array whose mutability we don't know!"))
      }
      case MutabilityTemplataT(ImmutableT) =>
      case MutabilityTemplataT(MutableT) => {
        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't destroy a mutable array with a callable!"))
      }
    }

    val prototype =
      overloadResolver.getArrayConsumerPrototype(
        coutputs, fate, range, callLocation, callableTE, arrayTT.elementType, contextRegion)

//    val freePrototype =
//      destructorCompiler.getFreeFunction(
//        coutputs, fate, range, arrTE.result.reference)
//        .function.prototype
//    vassert(coutputs.getInstantiationBounds(freePrototype.fullName).nonEmpty)

    ast.DestroyImmRuntimeSizedArrayTE(
      arrTE,
      arrayTT,
      callableTE,
      prototype)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_static_sized_array(&self, global_env: &'t GlobalEnvironmentT<'s, 't>, coutputs: &mut CompilerOutputs<'s, 't>) {
        // val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        // val templateId =
        //   IdT(builtinPackage, Vector.empty, interner.intern(StaticSizedArrayTemplateNameT()))
        let template_name = self.typing_interner.intern_static_sized_array_template_name(
            StaticSizedArrayTemplateNameT { _phantom: std::marker::PhantomData }
        );
        let template_id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::StaticSizedArrayTemplate(template_name),
        });

        // See CSFMSEO and SAFHE.
        // val arrayOuterEnv =
        //   CitizenEnvironmentT(
        //     globalEnv,
        //     PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        //     templateId,
        //     templateId,
        //     TemplatasStore(templateId, Map(), Map()))
        let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
            global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
        let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
        let parent_env = self.typing_interner.alloc(PackageEnvironmentT {
            global_env,
            id: *template_id,
            global_namespaces,
        });
        let empty_templatas = TemplatasStoreBuilder::new(template_id).build_in(self.typing_interner);
        let array_outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Package(parent_env),
            template_id: *template_id,
            id: *template_id,
            templatas: empty_templatas,
        });
        // coutputs.declareType(templateId)
        coutputs.declare_type(template_id);
        // coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)
        let array_outer_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_outer_env);
        coutputs.declare_type_outer_env(template_id, array_outer_env_ref);

        // val TemplateTemplataType(types, _) = StaticSizedArrayTemplateTemplataT().tyype
        // val Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()) = types
        // (assertion only — types are verified by the placeholder calls below)

        // val sizePlaceholder =
        //   templataCompiler.createNonKindNonRegionPlaceholderInner(
        //     templateId, 0, CodeRuneS(interner.intern(StrI("N"))), IntegerTemplataType())
        let rune_n = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("N"),
        }));
        let size_placeholder = self.create_non_kind_non_region_placeholder_inner(
            *template_id, 0, rune_n, ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
        );
        // val mutabilityPlaceholder =
        //   templataCompiler.createNonKindNonRegionPlaceholderInner(
        //     templateId, 1, CodeRuneS(interner.intern(StrI("M"))), MutabilityTemplataType())
        let rune_m = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("M"),
        }));
        let mutability_placeholder = self.create_non_kind_non_region_placeholder_inner(
            *template_id, 1, rune_m, ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        );
        // val variabilityPlaceholder =
        //   templataCompiler.createNonKindNonRegionPlaceholderInner(
        //     templateId, 2, CodeRuneS(interner.intern(StrI("V"))), VariabilityTemplataType())
        let rune_v = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("V"),
        }));
        let variability_placeholder = self.create_non_kind_non_region_placeholder_inner(
            *template_id, 2, rune_v, ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
        );
        // val elementPlaceholder =
        //   templataCompiler.createCoordPlaceholderInner(
        //     coutputs, arrayOuterEnv, templateId, 3, CodeRuneS(interner.intern(StrI("E"))), None, ReadOnlyRegionS, OwnT, true)
        let rune_e = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("E"),
        }));
        let element_placeholder = self.create_coord_placeholder_inner(
            coutputs,
            array_outer_env_ref,
            *template_id, 3, rune_e, None,
            IRegionMutabilityS::ReadOnlyRegion, OwnershipT::Own, true,
        );

        // val placeholders =
        //   Vector(sizePlaceholder, mutabilityPlaceholder, variabilityPlaceholder, elementPlaceholder)
        let element_placeholder_templata = ITemplataT::Coord(
            self.typing_interner.alloc(element_placeholder));
        let placeholders = [
            size_placeholder, mutability_placeholder, variability_placeholder, element_placeholder_templata,
        ];
        // val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))
        let local_name = template_name.make_citizen_name(self.typing_interner, &placeholders);
        let id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name,
        });
        // vassert(TemplataCompiler.getTemplate(id) == templateId)
        assert!(*self.get_template(*id) == *template_id);

        // val arrayInnerEnv =
        //   arrayOuterEnv.copy(
        //     id = id,
        //     templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
        let inner_templatas = TemplatasStoreBuilder::new(id).build_in(self.typing_interner);
        let array_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: array_outer_env.parent_env,
            template_id: array_outer_env.template_id,
            id: *id,
            templatas: inner_templatas,
        });
        let array_inner_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_inner_env);
        // coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
        coutputs.declare_type_inner_env(template_id, array_inner_env_ref);
    }
/*
  def compileStaticSizedArray(globalEnv: GlobalEnvironment, coutputs: CompilerOutputs): Unit = {
    val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
    val templateId =
      IdT(builtinPackage, Vector.empty, interner.intern(StaticSizedArrayTemplateNameT()))

    // We declare the function into the environment that we use to compile the
    // struct, so that those who use the struct can reach into its environment
    // and see the function and use it.
    // See CSFMSEO and SAFHE.
    val arrayOuterEnv =
      CitizenEnvironmentT(
        globalEnv,
        PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        templateId,
        templateId,
        TemplatasStore(templateId, Map(), Map()))
    coutputs.declareType(templateId)
    coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)

    val TemplateTemplataType(types, _) = StaticSizedArrayTemplateTemplataT().tyype
    val Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()) = types
    val sizePlaceholder =
      templataCompiler.createNonKindNonRegionPlaceholderInner(
        templateId, 0, CodeRuneS(interner.intern(StrI("N"))), IntegerTemplataType())
    val mutabilityPlaceholder =
      templataCompiler.createNonKindNonRegionPlaceholderInner(
        templateId, 1, CodeRuneS(interner.intern(StrI("M"))), MutabilityTemplataType())
    val variabilityPlaceholder =
      templataCompiler.createNonKindNonRegionPlaceholderInner(
        templateId, 2, CodeRuneS(interner.intern(StrI("V"))), VariabilityTemplataType())
    val elementPlaceholder =
      templataCompiler.createCoordPlaceholderInner(
        coutputs, arrayOuterEnv, templateId, 3, CodeRuneS(interner.intern(StrI("E"))), None, ReadOnlyRegionS, OwnT, true)

    val placeholders =
      Vector(sizePlaceholder, mutabilityPlaceholder, variabilityPlaceholder, elementPlaceholder)

    val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))
    vassert(TemplataCompiler.getTemplate(id) == templateId)

    val arrayInnerEnv =
      arrayOuterEnv.copy(
        id = id,
        templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
    coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_static_sized_array(
        &self,
        mutability: ITemplataT<'s, 't>,
        variability: ITemplataT<'s, 't>,
        size: ITemplataT<'s, 't>,
        type_2: CoordT<'s, 't>,
        region: RegionT,
    ) -> StaticSizedArrayTT<'s, 't> {
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        let template_name = self.typing_interner.intern_static_sized_array_template_name(
            StaticSizedArrayTemplateNameT { _phantom: std::marker::PhantomData }
        );
        let arr_name = self.typing_interner.intern_raw_array_name(
            RawArrayNameT { mutability, element_type: type_2, self_region: region }
        );
        let ssa_name = self.typing_interner.intern_static_sized_array_name(
            StaticSizedArrayNameT { template: template_name, size, variability, arr: arr_name }
        );
        let id = *self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::StaticSizedArray(ssa_name),
        });
        *self.typing_interner.intern_static_sized_array_tt(StaticSizedArrayTTValT { name: id })
    }
/*
Guardian: temp-disable: SPDMX — The `self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[])` pattern IS the established Rust equivalent of `PackageCoordinate.BUILTIN(interner, keywords)` — this exact pattern is already used in `compile_runtime_sized_array`, `compile_static_sized_array`, and `resolve_runtime_sized_array` in this same file. This is a documented, consistent Rust adaptation, not a parity drift.
  def resolveStaticSizedArray(
    mutability: ITemplataT[MutabilityTemplataType],
    variability: ITemplataT[VariabilityTemplataType],
    size: ITemplataT[IntegerTemplataType],
    type2: CoordT,
    region: RegionT):
  (StaticSizedArrayTT) = {
    interner.intern(StaticSizedArrayTT(
      IdT(
        PackageCoordinate.BUILTIN(interner, keywords),
        Vector(),
        interner.intern(StaticSizedArrayNameT(
          interner.intern(StaticSizedArrayTemplateNameT()),
          size,
          variability,
          interner.intern(RawArrayNameT(mutability, type2, region)))))))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_runtime_sized_array(&self, global_env: &'t GlobalEnvironmentT<'s, 't>, coutputs: &mut CompilerOutputs<'s, 't>) {
        // val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        // val templateId =
        //   IdT(builtinPackage, Vector.empty, interner.intern(RuntimeSizedArrayTemplateNameT()))
        let template_name = self.typing_interner.intern_runtime_sized_array_template_name(
            RuntimeSizedArrayTemplateNameT { _phantom: std::marker::PhantomData }
        );
        let template_id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::RuntimeSizedArrayTemplate(template_name),
        });

        // See CSFMSEO and SAFHE.
        // val arrayOuterEnv =
        //   CitizenEnvironmentT(
        //     globalEnv,
        //     PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        //     templateId,
        //     templateId,
        //     TemplatasStore(templateId, Map(), Map()))
        let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
            global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
        let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
        let parent_env = self.typing_interner.alloc(PackageEnvironmentT {
            global_env,
            id: *template_id,
            global_namespaces,
        });
        let empty_templatas = TemplatasStoreBuilder::new(template_id).build_in(self.typing_interner);
        let array_outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: IEnvironmentT::Package(parent_env),
            template_id: *template_id,
            id: *template_id,
            templatas: empty_templatas,
        });
        // coutputs.declareType(templateId)
        coutputs.declare_type(template_id);
        // coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)
        let array_outer_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_outer_env);
        coutputs.declare_type_outer_env(template_id, array_outer_env_ref);

        // val TemplateTemplataType(types, _) = RuntimeSizedArrayTemplateTemplataT().tyype
        // val Vector(MutabilityTemplataType(), CoordTemplataType()) = types
        // (assertion only — types are verified by the placeholder calls below)

        // val mutabilityPlaceholder =
        //   templataCompiler.createNonKindNonRegionPlaceholderInner(
        //     templateId, 0, CodeRuneS(interner.intern(StrI("M"))), MutabilityTemplataType())
        let rune_m = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("M"),
        }));
        let mutability_placeholder = self.create_non_kind_non_region_placeholder_inner(
            *template_id, 0, rune_m, ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
        );
        // val elementPlaceholder =
        //   templataCompiler.createCoordPlaceholderInner(
        //     coutputs, arrayOuterEnv, templateId, 1, CodeRuneS(interner.intern(StrI("E"))), None, ReadOnlyRegionS, OwnT, true)
        let rune_e = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str("E"),
        }));
        let element_placeholder = self.create_coord_placeholder_inner(
            coutputs,
            array_outer_env_ref,
            *template_id, 1, rune_e, None,
            IRegionMutabilityS::ReadOnlyRegion, OwnershipT::Own, true,
        );

        // val placeholders =
        //   Vector(mutabilityPlaceholder, elementPlaceholder)
        let element_placeholder_templata = ITemplataT::Coord(
            self.typing_interner.alloc(element_placeholder));
        let placeholders = [mutability_placeholder, element_placeholder_templata];
        // val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))
        let local_name = template_name.make_citizen_name(self.typing_interner, &placeholders);
        let id = self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name,
        });

        // val arrayInnerEnv =
        //   arrayOuterEnv.copy(
        //     id = id,
        //     templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
        let inner_templatas = TemplatasStoreBuilder::new(id).build_in(self.typing_interner);
        let array_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env,
            parent_env: array_outer_env.parent_env,
            template_id: array_outer_env.template_id,
            id: *id,
            templatas: inner_templatas,
        });
        let array_inner_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(array_inner_env);
        // coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
        coutputs.declare_type_inner_env(template_id, array_inner_env_ref);
    }
/*
  def compileRuntimeSizedArray(globalEnv: GlobalEnvironment, coutputs: CompilerOutputs): Unit = {
    val builtinPackage = PackageCoordinate.BUILTIN(interner, keywords)
    val templateId =
      IdT(builtinPackage, Vector.empty, interner.intern(RuntimeSizedArrayTemplateNameT()))

    // We declare the function into the environment that we use to compile the
    // struct, so that those who use the struct can reach into its environment
    // and see the function and use it.
    // See CSFMSEO and SAFHE.
    val arrayOuterEnv =
      CitizenEnvironmentT(
        globalEnv,
        PackageEnvironmentT(globalEnv, templateId, globalEnv.nameToTopLevelEnvironment.values.toVector),
        templateId,
        templateId,
        TemplatasStore(templateId, Map(), Map()))
    coutputs.declareType(templateId)
    coutputs.declareTypeOuterEnv(templateId, arrayOuterEnv)



    val TemplateTemplataType(types, _) = RuntimeSizedArrayTemplateTemplataT().tyype
    val Vector(MutabilityTemplataType(), CoordTemplataType()) = types
    val mutabilityPlaceholder =
      templataCompiler.createNonKindNonRegionPlaceholderInner(
        templateId, 0, CodeRuneS(interner.intern(StrI("M"))), MutabilityTemplataType())
    val elementPlaceholder =
      templataCompiler.createCoordPlaceholderInner(
        coutputs, arrayOuterEnv, templateId, 1, CodeRuneS(interner.intern(StrI("E"))), None, ReadOnlyRegionS,OwnT, true)
    val placeholders =
      Vector(mutabilityPlaceholder, elementPlaceholder)

    val id = templateId.copy(localName = templateId.localName.makeCitizenName(interner, placeholders))

    val arrayInnerEnv =
      arrayOuterEnv.copy(
        id = id,
        templatas = arrayOuterEnv.templatas.copy(templatasStoreName = id))
    coutputs.declareTypeInnerEnv(templateId, arrayInnerEnv)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_runtime_sized_array(
        &self,
        type_2: CoordT<'s, 't>,
        mutability: ITemplataT<'s, 't>,
        region: RegionT,
    ) -> RuntimeSizedArrayTT<'s, 't> {
        let builtin_package: &'s PackageCoordinate<'s> =
            self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
        let template_name = self.typing_interner.intern_runtime_sized_array_template_name(
            RuntimeSizedArrayTemplateNameT { _phantom: std::marker::PhantomData }
        );
        let arr_name = self.typing_interner.intern_raw_array_name(
            RawArrayNameT { mutability, element_type: type_2, self_region: region }
        );
        let rsa_name = self.typing_interner.intern_runtime_sized_array_name(
            RuntimeSizedArrayNameT { template: template_name, arr: arr_name }
        );
        let id = *self.typing_interner.intern_id(IdValT {
            package_coord: builtin_package,
            init_steps: &[],
            local_name: INameT::RuntimeSizedArray(rsa_name),
        });
        *self.typing_interner.intern_runtime_sized_array_tt(RuntimeSizedArrayTTValT { name: id })
    }
/*
Guardian: temp-disable: SPDMX — The `self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[])` pattern IS the established Rust equivalent of `PackageCoordinate.BUILTIN(interner, keywords)` — this exact pattern is already used in `compile_runtime_sized_array` (line 846) and `compile_static_sized_array` (line 635) in this same file, both with the Scala `BUILTIN` comment alongside. This is a documented, consistent Rust adaptation, not a parity drift. — FrontendRust/guardian-logs/request-386-1778704896207/hook-386/resolve_runtime_sized_array--993.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def resolveRuntimeSizedArray(
    type2: CoordT,
    mutability: ITemplataT[MutabilityTemplataType],
    region: RegionT):
  (RuntimeSizedArrayTT) = {
    interner.intern(RuntimeSizedArrayTT(
      IdT(
        PackageCoordinate.BUILTIN(interner, keywords),
        Vector(),
        interner.intern(RuntimeSizedArrayNameT(
          interner.intern(RuntimeSizedArrayTemplateNameT()),
          interner.intern(RawArrayNameT(mutability, type2, region)))))))
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn get_array_size(&self, templatas: &HashMap<IRuneS<'s>, ITemplataT<'s, 't>>, size_rune_a: IRuneS<'s>) -> i32 {
        panic!("Unimplemented: get_array_size");
    }
/*
  private def getArraySize(templatas: Map[IRuneS, ITemplataT[ITemplataType]], sizeRuneA: IRuneS): Int = {
    val IntegerTemplataT(m) = vassertSome(templatas.get(sizeRuneA))
    m.toInt
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn get_array_element_type(&self, templatas: &HashMap<IRuneS<'s>, ITemplataT<'s, 't>>, type_rune_a: IRuneS<'s>) -> CoordT<'s, 't> {
        let coord_templata = expect_coord_templata(*templatas.get(&type_rune_a).expect("vassertSome: typeRuneA not in templatas"));
        coord_templata.coord
    }
/*
  private def getArrayElementType(templatas: Map[IRuneS, ITemplataT[ITemplataType]], typeRuneA: IRuneS): CoordT = {
    val CoordTemplataT(m) = vassertSome(templatas.get(typeRuneA))
    m
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn lookup_in_static_sized_array(
        &self,
        range: RangeS<'s>,
        container_expr_2: ReferenceExpressionTE<'s, 't>,
        index_expr_2: ReferenceExpressionTE<'s, 't>,
        at: StaticSizedArrayTT<'s, 't>,
    ) -> StaticSizedArrayLookupTE<'s, 't> {
        let variability_templata = at.variability();
        let variability = match variability_templata {
            ITemplataT::Placeholder(_) => VariabilityT::Final,
            ITemplataT::Variability(VariabilityTemplataT { variability }) => variability,
            _ => panic!("vwat"),
        };
        let member_type = at.element_type();
        StaticSizedArrayLookupTE {
            range,
            array_expr: container_expr_2,
            array_type: self.typing_interner.alloc(at),
            index_expr: index_expr_2,
            element_type: member_type,
            variability,
        }
    }
/*
  def lookupInStaticSizedArray(
      range: RangeS,
      containerExpr2: ReferenceExpressionTE,
      indexExpr2: ReferenceExpressionTE,
    at: StaticSizedArrayTT):
  StaticSizedArrayLookupTE = {
    val contentsStaticSizedArrayTT(size, mutability, variabilityTemplata, memberType, selfRegion) = at
    val variability =
      variabilityTemplata match {
        case PlaceholderTemplataT(_, _) => FinalT
        case VariabilityTemplataT(variability) => variability
      }
    StaticSizedArrayLookupTE(range, containerExpr2, at, indexExpr2, memberType,  variability)
  }
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn lookup_in_unknown_sized_array(
        &self,
        parent_ranges: &[RangeS<'s>],
        range: RangeS<'s>,
        container_expr_2: ReferenceExpressionTE<'s, 't>,
        index_expr_2: ReferenceExpressionTE<'s, 't>,
        rsa: &'t RuntimeSizedArrayTT<'s, 't>,
    ) -> RuntimeSizedArrayLookupTE<'s, 't> {
        if index_expr_2.result().coord.kind != KindT::Int(IntT::I32) {
            panic!("implement: lookup_in_unknown_sized_array — IndexedArrayWithNonInteger");
        }
        let variability = match rsa.mutability() {
            ITemplataT::Placeholder(_) => VariabilityT::Final,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => VariabilityT::Final,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => VariabilityT::Varying,
            _ => panic!("vwat"),
        };
        RuntimeSizedArrayLookupTE::new(range, container_expr_2, rsa, index_expr_2, variability)
    }
/*
  def lookupInUnknownSizedArray(
    parentRanges: List[RangeS],
    range: RangeS,
    containerExpr2: ReferenceExpressionTE,
    indexExpr2: ReferenceExpressionTE,
    rsa: RuntimeSizedArrayTT
  ): RuntimeSizedArrayLookupTE = {
    val contentsRuntimeSizedArrayTT(mutability, memberType, selfRegion) = rsa
    vregionmut(selfRegion)
    if (indexExpr2.result.coord.kind != IntT(32)) {
      throw CompileErrorExceptionT(IndexedArrayWithNonInteger(range :: parentRanges, indexExpr2.result.coord))
    }
    val variability =
      mutability match {
        case PlaceholderTemplataT(_, MutabilityTemplataType()) => FinalT
        case MutabilityTemplataT(ImmutableT) => FinalT
        case MutabilityTemplataT(MutableT) => VaryingT
      }
    RuntimeSizedArrayLookupTE(range, containerExpr2, rsa, indexExpr2, variability)
  }

}
*/
}
