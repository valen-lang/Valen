use std::collections::HashSet;

use crate::postparsing::ast::{IBodyS, IFunctionAttributeS, LocationInDenizen};
use crate::postparsing::names::*;
use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::{ArgLookupTE, BlockTE, ExternFunctionCallTE, GenericParametersInheritance, ReferenceExpressionTE, ReturnTE};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::{CompilerOutputs, DeferredActionT};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::utils::range::RangeS;
use std::marker::PhantomData;

/*
package dev.vale.typing.function

import dev.vale.highertyping.FunctionA
import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, ConvertHelper, DeferredEvaluatingFunctionBody, RangedInternalErrorT, TemplataCompiler, TypingPassOptions, ast}
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.highertyping._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.citizen.ImplCompiler
import dev.vale.typing.env._

import scala.collection.immutable.{List, Set}

*/
pub struct ResultTypeMismatchError<'s, 't> {
    pub expected_type: CoordT<'s, 't>,
    pub actual_type: CoordT<'s, 't>,
}
/*
case class ResultTypeMismatchError(expectedType: CoordT, actualType: CoordT) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }

*/
/*
class FunctionCompilerCore(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,

    templataCompiler: TemplataCompiler,
    convertHelper: ConvertHelper,
    delegate: IFunctionCompilerDelegate) {
  val bodyCompiler = new BodyCompiler(opts, nameTranslator, templataCompiler, convertHelper, new IBodyCompilerDelegate {
    override def evaluateBlockStatements(
      coutputs: CompilerOutputs,
      startingNenv: NodeEnvironmentT,
      nenv: NodeEnvironmentBox,
      life: LocationInFunctionEnvironmentT,
      parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
      region: RegionT,
      exprs: BlockSE
    ): (ReferenceExpressionTE, Set[CoordT]) = {
      delegate.evaluateBlockStatements(
        coutputs, startingNenv, nenv, life, parentRanges, callLocation, region, exprs)
    }

    override def translatePatternList(
      coutputs: CompilerOutputs,
      nenv: NodeEnvironmentBox,
      life: LocationInFunctionEnvironmentT,
      parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
        region: RegionT,
      patterns1: Vector[AtomSP],
      patternInputExprs2: Vector[ReferenceExpressionTE]
    ): ReferenceExpressionTE = {
      delegate.translatePatternList(coutputs, nenv, life, parentRanges, callLocation, region, patterns1, patternInputExprs2)
    }
  })

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    // Preconditions:
    // - already spawned local env
    // - either no template args, or they were already added to the env.
    // - either no closured vars, or they were already added to the env.
    pub fn evaluate_function_for_header_core(
        &self,
        full_env: &'t FunctionEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        params2: &[ParameterT<'s, 't>],
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        // fullEnv.id match { case IdT(...drop...) => vpass(); case _ => }
        // (debug pattern match, not functionally needed)

        // val life = LocationInFunctionEnvironmentT(Vector())
        let life = LocationInFunctionEnvironmentT { path: self.typing_interner.alloc_slice_from_vec(Vec::new())};

        // val isDestructor = params2.nonEmpty && params2.head.tyype.ownership == OwnT && ...
        let is_destructor =
            !params2.is_empty() &&
            params2[0].tyype.ownership == OwnershipT::Own &&
            match full_env.id.local_name {
                INameT::Function(func_name) if func_name.template.human_name == self.keywords.drop => true,
                _ => false,
            };

        // val maybeExport = fullEnv.function.attributes.collectFirst { case e@ExportS(_) => e }
        let _maybe_export =
            full_env.function.attributes.iter().find_map(|a| {
                match a {
                    IFunctionAttributeS::Export(e) => Some(e),
                    _ => None,
                }
            });

        // val signature2 = SignatureT(fullEnv.id)
        let signature2: &'t SignatureT<'s, 't> = self.typing_interner.alloc(SignatureT { id: full_env.id });

        // val maybeRetTemplata = fullEnv.function.maybeRetCoordRune match { ... }
        let maybe_ret_templata =
            match &full_env.function.maybe_ret_coord_rune {
                None => None,
                Some(ret_coord_rune) => {
                    let imprecise_name = self.scout_arena.intern_imprecise_name(
                        IImpreciseNameValS::RuneName(RuneNameValS { rune: ret_coord_rune.rune }));
                    let mut lookup_filter = HashSet::new();
                    lookup_filter.insert(ILookupContext::TemplataLookupContext);
                    let full_env_as_i = IEnvironmentT::Function(full_env);
                    full_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner)
                }
            };

        // val maybeRetCoord = maybeRetTemplata match { ... }
        let maybe_ret_coord =
            match maybe_ret_templata {
                None => None,
                Some(ITemplataT::Coord(coord_templata)) => {
                    let ret_coord = coord_templata.coord;
                    coutputs.declare_function_return_type(signature2, ret_coord);
                    Some(ret_coord)
                }
                _ => panic!("Must be a coord!"),
            };

        // val header = fullEnv.function.body match { ... }
        let header =
            match &full_env.function.body {
                IBodyS::CodeBody(_body) => {
                    // val attributesWithoutExport = ...
                    let attributes_without_export: Vec<&IFunctionAttributeS<'s>> =
                        full_env.function.attributes.iter().filter(|a| {
                            !matches!(a, IFunctionAttributeS::Export(_))
                        }).collect();
                    let attributes_t = self.translate_attributes(&attributes_without_export);

                    match maybe_ret_coord {
                        Some(return_coord) => {
                            // val header = finalizeHeader(...)
                            let header =
                                self.finalize_header(full_env, coutputs, attributes_t.clone(), params2, return_coord);

                            // coutputs.deferEvaluatingFunctionBody(DeferredEvaluatingFunctionBody(...))
                            let attributes_t_arena: &'t [IFunctionAttributeT<'s>] =
                                self.typing_interner.alloc_slice_from_vec(attributes_t);
                            let call_range_arena: &'t [RangeS<'s>] =
                                self.typing_interner.alloc_slice_copy(call_range);
                            let params_t_arena: &'t [ParameterT<'s, 't>] =
                                self.typing_interner.alloc_slice_from_vec(params2.to_vec());

                            coutputs.defer_evaluating_function_body(
                                DeferredActionT::EvaluateFunctionBody {
                                    prototype: self.typing_interner.alloc(header.to_prototype()),
                                    full_env_snapshot: full_env,
                                    call_range: call_range_arena,
                                    call_location,
                                    life,
                                    attributes_t: attributes_t_arena,
                                    params_t: params_t_arena,
                                    is_destructor,
                                    maybe_explicit_return_coord: Some(return_coord),
                                    instantiation_bound_params,
                                });

                            header
                        }
                        None => {
                            let attributes_t_arena: &'t [IFunctionAttributeT<'s>] =
                                self.typing_interner.alloc_slice_from_vec(attributes_t);
                            let call_range_arena: &'t [RangeS<'s>] =
                                self.typing_interner.alloc_slice_copy(call_range);
                            let params_t_arena: &'t [ParameterT<'s, 't>] =
                                self.typing_interner.alloc_slice_from_vec(params2.to_vec());
                            let header =
                                self.finish_function_maybe_deferred(
                                    coutputs, full_env, call_range_arena, call_location, life, attributes_t_arena, params_t_arena, is_destructor, None, instantiation_bound_params)?;
                            header
                        }
                    }
                }
                IBodyS::ExternBody(_) => {
                    let ret_coord = maybe_ret_coord.unwrap();
                    let header =
                        self.make_extern_function(
                            coutputs,
                            full_env,
                            full_env.function.range,
                            self.translate_function_attributes(full_env.function.attributes),
                            params2,
                            ret_coord,
                            Some(FunctionTemplataT { outer_env: full_env.parent_env, function: full_env.function }));
                    header
                }
                IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => {
                    let generator_id = match &full_env.function.body {
                        IBodyS::AbstractBody(_) => self.keywords.abstract_body,
                        IBodyS::GeneratedBody(g) => g.generator_id,
                        _ => unreachable!(),
                    };

                    assert!(coutputs.lookup_function(signature2).is_none());

                    let generator = full_env.global_env.name_to_function_body_macro
                        .get(&generator_id)
                        .expect("generator not found in name_to_function_body_macro");
                    let (header, body) = generator.generate_function_body(
                        self, coutputs, full_env, generator_id, life, call_range, call_location,
                        Some(full_env.function), params2, maybe_ret_coord)?;

                    let header: &'t FunctionHeaderT<'s, 't> =
                        self.typing_interner.alloc(header);

                    coutputs.declare_function_return_type(
                        self.typing_interner.alloc(header.to_signature()), header.return_type);

                    let header_sig = self.typing_interner.alloc(header.to_signature());
                    coutputs.add_function(
                        header_sig,
                        self.typing_interner.alloc(FunctionDefinitionT {
                            header,
                            instantiation_bound_params,
                            body,
                        }));

                    if header.to_signature() != *signature2 {
                        panic!("Generator made a function whose signature doesn't match the expected one!");
                    }
                    header
                }
            };

        // if (header.attributes.exists({ case PureT => true case _ => false })) { ... }
        if header.attributes.iter().any(|a| matches!(a, IFunctionAttributeT::Pure)) {
            // (Scala has commented-out purity checks here)
        }

        Ok(header)
    }
/*
  // Preconditions:
  // - already spawned local env
  // - either no template args, or they were already added to the env.
  // - either no closured vars, or they were already added to the env.
  def evaluateFunctionForHeader(
    fullEnv: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    params2: Vector[ParameterT],
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]):
  (FunctionHeaderT) = {
    fullEnv.id match {
      case IdT(_, Vector(), FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), Vector(CoordTemplataT(CoordT(_, RegionT(DefaultRegionT), KindPlaceholderT(IdT(_, Vector(FunctionTemplateNameT(StrI("drop"), _)), KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, CodeRuneS(StrI("T0")))))))), CoordTemplataT(CoordT(_, RegionT(DefaultRegionT), KindPlaceholderT(IdT(_, Vector(FunctionTemplateNameT(StrI("drop"), _)), KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, CodeRuneS(StrI("T1"))))))))), Vector(CoordT(_, RegionT(DefaultRegionT), StructTT(IdT(_, Vector(), StructNameT(StructTemplateNameT(StrI("Tup2")), Vector(CoordTemplataT(CoordT(_, RegionT(DefaultRegionT), KindPlaceholderT(IdT(_, Vector(FunctionTemplateNameT(StrI("drop"), _)), KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, CodeRuneS(StrI("T0")))))))), CoordTemplataT(CoordT(_, RegionT(DefaultRegionT), KindPlaceholderT(IdT(_, Vector(FunctionTemplateNameT(StrI("drop"), _)), KindPlaceholderNameT(KindPlaceholderTemplateNameT(1, CodeRuneS(StrI("T1")))))))))))))))) => {
        vpass()
      }
      case _ =>
    }

//    opts.debugOut("Evaluating function " + fullEnv.fullName)

//    val functionTemplateName = TemplataCompiler.getFunctionTemplate(fullEnv.fullName)
    val functionTemplateName = fullEnv.id

    val life = LocationInFunctionEnvironmentT(Vector())

    val isDestructor =
      params2.nonEmpty &&
        params2.head.tyype.ownership == OwnT &&
        (fullEnv.id.localName match {
          case FunctionNameT(humanName, _, _) if humanName == keywords.drop => true
          case _ => false
        })

    val maybeExport =
      fullEnv.function.attributes.collectFirst { case e@ExportS(_) => e }

    val signature2 = SignatureT(fullEnv.id);
    val maybeRetTemplata =
      fullEnv.function.maybeRetCoordRune match {
        case None => (None)
        case Some(retCoordRune) => {
          fullEnv.lookupNearestWithImpreciseName(interner.intern(RuneNameS(retCoordRune.rune)), Set(TemplataLookupContext)).headOption
        }
      }
    val maybeRetCoord =
      maybeRetTemplata match {
        case None => (None)
        case Some(CoordTemplataT(retCoord)) => {
          coutputs.declareFunctionReturnType(signature2, retCoord)
          (Some(retCoord))
        }
        case _ => throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Must be a coord!"))
      }

    val header =
      fullEnv.function.body match {
        case CodeBodyS(body) => {
          val attributesWithoutExport =
            fullEnv.function.attributes.filter({
              case ExportS(_) => false
              case _ => true
            })
          val attributesT = translateAttributes(attributesWithoutExport)

          maybeRetCoord match {
            case Some(returnCoord) => {
              val header =
                finalizeHeader(fullEnv, coutputs, attributesT, params2, returnCoord)

              coutputs.deferEvaluatingFunctionBody(
                DeferredEvaluatingFunctionBody(
                  header.toPrototype,
                  (coutputs) => {
                    finishFunctionMaybeDeferred(
                      coutputs, fullEnv, callRange, callLocation, life, attributesT, params2, isDestructor, Some(returnCoord), instantiationBoundParams)
                  }))

              (header)
            }
            case None => {
              val header =
                finishFunctionMaybeDeferred(
                  coutputs, fullEnv, callRange, callLocation, life, attributesT, params2, isDestructor, None, instantiationBoundParams)
              (header)
            }
          }
        }
        case ExternBodyS => {
          val retCoord = vassertSome(maybeRetCoord)
          val header =
            makeExternFunction(
              coutputs,
              fullEnv,
              fullEnv.function.range,
              translateFunctionAttributes(fullEnv.function.attributes),
              params2,
              retCoord,
              Some(FunctionTemplataT(fullEnv.parentEnv, fullEnv.function)))
          (header)
        }
        case AbstractBodyS | GeneratedBodyS(_) => {
          val generatorId =
            fullEnv.function.body match {
              case AbstractBodyS => keywords.abstractBody
              case GeneratedBodyS(generatorId) => generatorId
            }

          // Funny story... let's say we're current instantiating a constructor,
          // for example MySome<T>().
          // The constructor returns a MySome<T>, which means when we do the above
          // evaluating of the function body, we stamp the MySome<T> struct.
          // That ends up stamping the entire struct, including the constructor.
          // That's what we were originally here for, and evaluating the body above
          // just did it for us O_o
          // So, here we check to see if we accidentally already did it.
          //   opts.debugOut("doesnt this mean we have to do this in every single generated function?")
          //   coutputs.lookupFunction(signature2) match {
          //     case Some(function2) => {
          //       (function2.header)
          //     }
          //     case None => {
          //       val generator = vassertSome(fullEnv.globalEnv.nameToFunctionBodyMacro.get(generatorId))
          //       val (header, body) =
          //         generator.generateFunctionBody(
          //           fullEnv, coutputs, generatorId, life, callRange,
          //           Some(fullEnv.function), params2, maybeRetCoord)
          //
          //       coutputs.declareFunctionReturnType(header.toSignature, header.returnType)
          //       val runeToFunctionBound = TemplataCompiler.assembleFunctionBoundToRune(fullEnv.templatas)
          //       coutputs.addFunction(FunctionT(header, runeToFunctionBound, body))
          //
          //       if (header.toSignature != signature2) {
          //         throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Generator made a function whose signature doesn't match the expected one!\n" +
          //           "Expected:  " + signature2 + "\n" +
          //           "Generated: " + header.toSignature))
          //       }
          //       (header)
          //     }
          //   }
          // Note from later: This might not be true anymore, since we have real generics.
          vassert(coutputs.lookupFunction(signature2).isEmpty)

          val generator = vassertSome(fullEnv.globalEnv.nameToFunctionBodyMacro.get(generatorId))
          val (header, body) =
            generator.generateFunctionBody(
              fullEnv, coutputs, generatorId, life, callRange, callLocation,
              Some(fullEnv.function), params2, maybeRetCoord)

          coutputs.declareFunctionReturnType(header.toSignature, header.returnType)
          coutputs.addFunction(
            FunctionDefinitionT(
              header,
              instantiationBoundParams,
              body))

          if (header.toSignature != signature2) {
            throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Generator made a function whose signature doesn't match the expected one!\n" +
              "Expected:  " + signature2 + "\n" +
              "Generated: " + header.toSignature))
          }
          header
        }
      }

    if (header.attributes.contains(PureT)) {
      //      header.params.foreach(param => {
      //        if (param.tyype.permission != ReadonlyT) {
      //          throw CompileErrorExceptionT(NonReadonlyReferenceFoundInPureFunctionParameter(fullEnv.function.range, param.name))
      //        }
      //      })
    }

    header
  }

*/
    pub fn get_function_prototype_for_call(
        &self,
        full_env: &'t FunctionEnvironmentT<'s, 't>,
        _coutputs: &CompilerOutputs<'s, 't>,
        _call_range: &[RangeS<'s>],
        _params2: &[ParameterT<'s, 't>],
    ) -> PrototypeT<'s, 't> {
        self.get_function_prototype_inner_for_call(full_env, full_env.id)
    }
/*
  // Preconditions:
  // - already spawned local env
  // - either no template args, or they were already added to the env.
  // - either no closured vars, or they were already added to the env.
  def getFunctionPrototypeForCall(
    fullEnv: FunctionEnvironmentT,
      coutputs: CompilerOutputs,
    callRange: List[RangeS],
      params2: Vector[ParameterT]):
  (PrototypeT[IFunctionNameT]) = {
    getFunctionPrototypeInnerForCall(
      fullEnv, fullEnv.id)
  }

*/
    pub fn get_function_prototype_inner_for_call(
        &self,
        full_env: &'t FunctionEnvironmentT<'s, 't>,
        id: IdT<'s, 't>,
    ) -> PrototypeT<'s, 't> {
        let ret_coord_rune = full_env.function.maybe_ret_coord_rune.unwrap();
        let imprecise_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::RuneName(RuneNameValS { rune: ret_coord_rune.rune }));
        let mut lookup_filter = HashSet::new();
        lookup_filter.insert(ILookupContext::TemplataLookupContext);
        let full_env_as_i = IInDenizenEnvironmentT::Function(full_env);
        let return_coord = match full_env_as_i.lookup_nearest_with_imprecise_name(imprecise_name, lookup_filter, self.typing_interner) {
            Some(ITemplataT::Coord(coord_templata)) => coord_templata.coord,
            other => panic!("vwat: unexpected in getFunctionPrototypeInnerForCall: {:?}", other),
        };
        PrototypeT { id, return_type: return_coord }
    }
/*
  def getFunctionPrototypeInnerForCall(
    fullEnv: FunctionEnvironmentT,
    id: IdT[IFunctionNameT]):
  PrototypeT[IFunctionNameT] = {
    val retCoordRune = vassertSome(fullEnv.function.maybeRetCoordRune)
    val returnCoord =
      fullEnv.lookupNearestWithImpreciseName(
        interner.intern(RuneNameS(retCoordRune.rune)),
        Set(TemplataLookupContext))  match {
        case Some(CoordTemplataT(retCoord)) => retCoord
        case other => vwat(other)
      }
    PrototypeT(id, returnCoord)
  }

*/
    pub fn finalize_header(
        &self,
        full_env: &'t FunctionEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        attributes_t: Vec<IFunctionAttributeT<'s>>,
        params_t: &[ParameterT<'s, 't>],
        return_coord: CoordT<'s, 't>,
    ) -> &'t FunctionHeaderT<'s, 't> {
        let header = self.typing_interner.alloc(FunctionHeaderT {
            id: full_env.id,
            attributes: self.typing_interner.alloc_slice_from_vec(attributes_t),
            params: self.typing_interner.alloc_slice_from_vec(params_t.to_vec()),
            return_type: return_coord,
            maybe_origin_function_templata: Some(FunctionTemplataT {
                outer_env: full_env.parent_env,
                function: full_env.function,
            }),
        });
        let sig_ref = self.typing_interner.alloc(header.to_signature());
        coutputs.declare_function_return_type(sig_ref, return_coord);
        header
    }
/*
  def finalizeHeader(
      fullEnv: FunctionEnvironmentT,
      coutputs: CompilerOutputs,
      attributesT: Vector[IFunctionAttributeT],
      paramsT: Vector[ParameterT],
    returnCoord: CoordT):
  FunctionHeaderT = {
    val header =
      FunctionHeaderT(
        fullEnv.id,
        attributesT,
//        vimpl(),
        paramsT,
        returnCoord,
        Some(FunctionTemplataT(fullEnv.parentEnv, fullEnv.function)));
    coutputs.declareFunctionReturnType(header.toSignature, returnCoord)
    header
  }

*/
    pub fn finish_function_maybe_deferred(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        full_env_snapshot: &'t FunctionEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        life: LocationInFunctionEnvironmentT<'t>,
        attributes_t: &'t [IFunctionAttributeT<'s>],
        params_t: &'t [ParameterT<'s, 't>],
        is_destructor: bool,
        maybe_explicit_return_coord: Option<CoordT<'s, 't>>,
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        // val (maybeEvaluatedRetCoord, body2) =
        //   bodyCompiler.declareAndEvaluateFunctionBody(
        //     fullEnvSnapshot, coutputs, life, callRange, callLocation,
        //     fullEnvSnapshot.function, maybeExplicitReturnCoord, paramsT, isDestructor)
        let (maybe_evaluated_ret_coord, body2) =
            self.declare_and_evaluate_function_body(
                full_env_snapshot, coutputs, life, call_range, call_location,
                full_env_snapshot.function, maybe_explicit_return_coord, params_t, is_destructor)?;

        let ret_coord = match (maybe_explicit_return_coord, maybe_evaluated_ret_coord) {
            (Some(c), None) => c,
            (None, Some(c)) => c,
            _ => panic!("Expected exactly one return coord"),
        };
        let header = self.finalize_header(
            full_env_snapshot, coutputs, attributes_t.to_vec(), params_t, ret_coord);

        let _needed_function_bounds = self.assemble_rune_to_function_bound(full_env_snapshot.templatas);
        let _needed_impl_bounds = self.assemble_rune_to_impl_bound(full_env_snapshot.templatas);

        let header_sig = self.typing_interner.alloc(header.to_signature());
        assert!(coutputs.lookup_function(header_sig).is_none());
        let function2 = self.typing_interner.alloc(FunctionDefinitionT {
            header,
            instantiation_bound_params,
            body: ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE { inner: body2.inner })),
        });
        coutputs.add_function(header_sig, function2);
        Ok(function2.header)
    }
/*
  // By MaybeDeferred we mean that this function might be called later, to reduce reentrancy.
  private def finishFunctionMaybeDeferred(
      coutputs: CompilerOutputs,
      fullEnvSnapshot: FunctionEnvironmentT,
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      life: LocationInFunctionEnvironmentT,
      attributesT: Vector[IFunctionAttributeT],
      paramsT: Vector[ParameterT],
      isDestructor: Boolean,
      maybeExplicitReturnCoord: Option[CoordT],
      instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]):
  FunctionHeaderT = {
    val (maybeEvaluatedRetCoord, body2) =
      bodyCompiler.declareAndEvaluateFunctionBody(
        fullEnvSnapshot,
        coutputs, life, callRange, callLocation, fullEnvSnapshot.function, maybeExplicitReturnCoord, paramsT, isDestructor)

    val retCoord = vassertOne(maybeExplicitReturnCoord.toList ++ maybeEvaluatedRetCoord.toList)
    val header = finalizeHeader(fullEnvSnapshot, coutputs, attributesT, paramsT, retCoord)

    // Funny story... let's say we're current instantiating a constructor,
    // for example MySome<T>().
    // The constructor returns a MySome<T>, which means when we do the above
    // evaluating of the function body, we stamp the MySome<T> struct.
    // That ends up stamping the entire struct, including the constructor.
    // That's what we were originally here for, and evaluating the body above
    // just did it for us O_o
    // So, here we check to see if we accidentally already did it.
    // Note from later: this might not be true anymore now that we have real generics.
    //   coutputs.lookupFunction(header.toSignature) match {
    //     case None => {
    //       val functionBoundToRune = TemplataCompiler.assembleFunctionBoundToRune(fullEnv.templatas)
    //       val function2 = FunctionT(header, functionBoundToRune, body2);
    //       coutputs.addFunction(function2)
    //       (function2.header)
    //     }
    //     case Some(function2) => {
    //       (function2.header)
    //     }
    //   }
    vassert(coutputs.lookupFunction(header.toSignature).isEmpty)
    val neededFunctionBounds = TemplataCompiler.assembleRuneToFunctionBound(fullEnvSnapshot.templatas)
    val neededImplBounds = TemplataCompiler.assembleRuneToImplBound(fullEnvSnapshot.templatas)
    val function2 =
      FunctionDefinitionT(
        header,
        instantiationBoundParams,
        body2);
    coutputs.addFunction(function2)
    header
  }

*/
    pub fn translate_attributes(&self, attributes_a: &[&IFunctionAttributeS<'s>]) -> Vec<IFunctionAttributeT<'s>> {
        attributes_a.iter().map(|a| {
            match a {
                IFunctionAttributeS::UserFunction(_) => IFunctionAttributeT::UserFunction,
                IFunctionAttributeS::Pure(_) => IFunctionAttributeT::Pure,
                IFunctionAttributeS::Additive(_) => IFunctionAttributeT::Additive,
                _ => panic!("implement: translate other function attributes"),
            }
        }).collect()
    }
/*
  def translateAttributes(attributesA: Vector[IFunctionAttributeS]): Vector[IFunctionAttributeT] = {
    attributesA.map({
      //      case ExportA(packageCoord) => Export2(packageCoord)
      case UserFunctionS => UserFunctionT
      case PureS => PureT
      case AdditiveS => AdditiveT
    })
  }

*/
    pub fn make_extern_function(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        range: RangeS<'s>,
        attributes: Vec<IFunctionAttributeT<'s>>,
        params2: &[ParameterT<'s, 't>],
        return_type: CoordT<'s, 't>,
        maybe_origin: Option<FunctionTemplataT<'s, 't>>,
    ) -> &'t FunctionHeaderT<'s, 't> {
        match env.id.local_name {
            INameT::Function(FunctionNameT { template: FunctionTemplateNameT { human_name, .. }, template_args: template_params, parameters, .. }) => {
                let header = self.typing_interner.alloc(FunctionHeaderT {
                    id: env.id,
                    attributes: self.typing_interner.alloc_slice_from_vec(attributes),
                    params: self.typing_interner.alloc_slice_from_vec(params2.to_vec()),
                    return_type,
                    maybe_origin_function_templata: maybe_origin,
                });

                let extern_function_name = self.typing_interner.intern_name(
                    INameValT::ExternFunction(ExternFunctionNameValT { human_name: *human_name, template_args: template_params, parameters }));
                let extern_function_id = self.typing_interner.intern_id(IdValT {
                    package_coord: env.id.package_coord,
                    init_steps: env.id.init_steps,
                    local_name: extern_function_name,
                });
                let extern_prototype = self.typing_interner.alloc(PrototypeT {
                    id: *extern_function_id,
                    return_type: header.return_type,
                });

                coutputs.add_instantiation_bounds(
                    self.opts.global_options.sanity_check,
                    self.typing_interner,
                    env.template_id,
                    extern_prototype.id,
                    self.typing_interner.alloc(InstantiationBoundArgumentsT {
                        rune_to_bound_prototype: self.typing_interner.alloc_index_map(),
                        rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map(),
                        rune_to_bound_impl: self.typing_interner.alloc_index_map(),
                    }),
                );

                let arg_lookups: Vec<ReferenceExpressionTE<'s, 't>> =
                    header.params.iter().enumerate().map(|(index, param)| {
                        ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
                            param_index: index as i32,
                            coord: param.tyype,
                        }))
                    }).collect();

                let function2 = self.typing_interner.alloc(FunctionDefinitionT {
                    header,
                    instantiation_bound_params: self.typing_interner.alloc(InstantiationBoundArgumentsT {
                        rune_to_bound_prototype: self.typing_interner.alloc_index_map(),
                        rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map(),
                        rune_to_bound_impl: self.typing_interner.alloc_index_map(),
                    }),
                    body: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                        source_expr: ReferenceExpressionTE::ExternFunctionCall(self.typing_interner.alloc(ExternFunctionCallTE {
                            prototype2: extern_prototype,
                            args: self.typing_interner.alloc_slice_from_vec(arg_lookups),
                        })),
                    })),
                });

                let header_sig = self.typing_interner.alloc(function2.header.to_signature());
                coutputs.declare_function_return_type(header_sig, function2.header.return_type);
                coutputs.add_function(header_sig, function2);
                // Register the extern with coutputs so it survives to HinputsT.functionExterns and reaches
                // the Backend's pragma generation. The placeholderedExternId mirrors what Compiler.scala
                // used to construct: a top-level IdT with an ExternNameT carrying a fresh ExternTemplateNameT
                // keyed on the function's range. Fires for both top-level externs and externs declared
                // inside an extern struct (the latter wouldn't otherwise reach Compiler.scala's loop).
                let extern_template_name = self.typing_interner.intern_extern_template_name(ExternTemplateNameT {
                    code_loc: range.begin,
                });
                let placeholdered_extern_name = self.typing_interner.intern_extern_name(ExternNameT {
                    template: extern_template_name,
                    template_arg: RegionT { region: IRegionT::Default },
                });
                let placeholdered_extern_id = *self.typing_interner.intern_id(IdValT {
                    package_coord: env.id.package_coord,
                    init_steps: &[],
                    local_name: INameT::Extern(placeholdered_extern_name),
                });
                // Per @PRIIROZ, internal-method externs inherit the container's generic params at the end
                // of their templateArgs. Hammer uses this count to reshape the wire-format SimpleId so the
                // inherited args land on the citizen step instead of the function step (i.e.
                // `Vec<i32>::capacity` rather than `Vec::capacity<i32>`), which is what Backend's
                // rustifySimpleId expects per @SMLRZ.
                let maybe_inheritance = match ICitizenTemplateNameT::try_from(extern_prototype.id.init_id(self.typing_interner).local_name) {
                    Ok(_ctn) => {
                        let citizen = coutputs.lookup_citizen_by_template_name(extern_prototype.id.init_id(self.typing_interner));
                        Some(GenericParametersInheritance { num_inherited_generic_parameters: citizen.generic_param_types(self.scout_arena).len() as i32 })
                    }
                    Err(_) => None,
                };
                coutputs.add_function_extern(
                    range, placeholdered_extern_id, extern_prototype, *human_name, maybe_inheritance, self.typing_interner);
                function2.header
            }
            _ => {
                panic!("Only human-named function can be extern!");
            }
        }
    }
/*
  def makeExternFunction(
      coutputs: CompilerOutputs,
      env: FunctionEnvironmentT,
      range: RangeS,
      attributes: Vector[IFunctionAttributeT],
      params2: Vector[ParameterT],
      returnType2: CoordT,
      maybeOrigin: Option[FunctionTemplataT]):
  (FunctionHeaderT) = {
    env.id.localName match {
      case FunctionNameT(FunctionTemplateNameT(humanName, _), templateParams, params) => {
        val header =
          ast.FunctionHeaderT(
            env.id,
            attributes,
//            Vector(RegionT(env.defaultRegion.localName, true)),
            params2,
            returnType2,
            maybeOrigin)

        val externFunctionId = IdT(env.id.packageCoord, env.id.initSteps, interner.intern(ExternFunctionNameT(humanName, templateParams, params)))
        val externPrototype = PrototypeT[ExternFunctionNameT](externFunctionId, header.returnType)

        coutputs.addInstantiationBounds(
          opts.globalOptions.sanityCheck,
          interner,
          env.templateId,
          externPrototype.id,
          InstantiationBoundArgumentsT.make(Map(), Map(), Map()))

        val argLookups =
          header.params.zipWithIndex.map({ case (param2, index) => ArgLookupTE(index, param2.tyype) })
        val function2 =
          FunctionDefinitionT(
            header,
            InstantiationBoundArgumentsT.make[FunctionBoundNameT, ImplBoundNameT](Map(), Map(), Map()),
            ReturnTE(ExternFunctionCallTE(externPrototype, argLookups)))

        coutputs.declareFunctionReturnType(header.toSignature, header.returnType)
        coutputs.addFunction(function2)
        // Register the extern with coutputs so it survives to HinputsT.functionExterns and reaches
        // the Backend's pragma generation. The placeholderedExternId mirrors what Compiler.scala
        // used to construct: a top-level IdT with an ExternNameT carrying a fresh ExternTemplateNameT
        // keyed on the function's range. Fires for both top-level externs and externs declared
        // inside an extern struct (the latter wouldn't otherwise reach Compiler.scala's loop).
        val placeholderedExternId =
          IdT(
            env.id.packageCoord,
            Vector(),
            interner.intern(
              ExternNameT(
                interner.intern(ExternTemplateNameT(range.begin)),
                RegionT(DefaultRegionT))))
        // Per @PRIIROZ, internal-method externs inherit the container's generic params at the end
        // of their templateArgs. Hammer uses this count to reshape the wire-format SimpleId so the
        // inherited args land on the citizen step instead of the function step (i.e.
        // `Vec<i32>::capacity` rather than `Vec::capacity<i32>`), which is what Backend's
        // rustifySimpleId expects per @SMLRZ.
        val maybeInheritance =
          externPrototype.id.initId(interner) match {
            case IdT(paackage, initSteps, ctn: ICitizenTemplateNameT) => {
              val citizen = coutputs.lookupCitizen(IdT(paackage, initSteps, ctn))
              Some(GenericParametersInheritance(citizen.genericParamTypes.size))
            }
            case _ => None
          }
        coutputs.addFunctionExtern(
          range, placeholderedExternId, externPrototype, humanName, maybeInheritance)
        (header)
      }
      case _ => {
        throw CompileErrorExceptionT(RangedInternalErrorT(List(range), "Only human-named function can be extern!"))
      }
    }
  }

*/
    pub fn translate_function_attributes(&self, a: &[IFunctionAttributeS<'s>]) -> Vec<IFunctionAttributeT<'s>> {
        a.iter().map(|attr| {
            match attr {
                IFunctionAttributeS::UserFunction(_) => IFunctionAttributeT::UserFunction,
                IFunctionAttributeS::Extern(extern_s) => IFunctionAttributeT::Extern(ExternT { package_coord: *extern_s.package_coord }),
                _ => panic!("implement: translateFunctionAttributes {:?}", attr),
            }
        }).collect()
    }
/*
  def translateFunctionAttributes(a: Vector[IFunctionAttributeS]): Vector[IFunctionAttributeT] = {
    U.map[IFunctionAttributeS, IFunctionAttributeT](a, {
      case UserFunctionS => UserFunctionT
      case ExternS(packageCoord) => ExternT(packageCoord)
      case x => vimpl(x)
    })
  }


//  def makeImplDestructor(
//    env: FunctionEnvironment,
//    coutputs: CompilerOutputs,
//    maybeOriginFunction1: Option[FunctionA],
//    structDefT: StructDefinitionT,
//    interfaceTT: InterfaceTT,
//    structDestructor: PrototypeT,
//  ):
//  (FunctionHeaderT) = {
//    val ownership = if (structDefT.mutability == MutableT) OwnT else ShareT
//    val permission = if (structDefT.mutability == MutableT) ReadwriteT else ReadonlyT
//    val structTT = structDefT.getRef
//    val structType2 = CoordT(ownership, permission, structTT)
//
//    val destructor2 =
//      ast.FunctionT(
//        ast.FunctionHeaderT(
//          env.fullName,
//          Vector.empty,
//          Vector(ast.ParameterT(interner.intern(CodeVarNameT("self")), None, structType2)),
//          CoordT(ShareT, VoidT()),
//          maybeOriginFunction1),
//        BlockTE(
//            ReturnTE(
//              FunctionCallTE(
//                structDestructor,
//                Vector(ArgLookupTE(0, structType2))))))
//
//    // If this fails, then the signature the FunctionCompilerMiddleLayer made for us doesn't
//    // match what we just made
//    vassert(
//      coutputs.getDeclaredSignatureOrigin(
//        destructor2.header.toSignature).nonEmpty)
//
//    // we cant make the destructor here because they might have a user defined one somewhere
//
//      coutputs
//        .declareFunctionReturnType(destructor2.header.toSignature, destructor2.header.returnType)
//      coutputs.addFunction(destructor2);
//
//    vassert(
//      coutputs.getDeclaredSignatureOrigin(
//        destructor2.header.toSignature).nonEmpty)
//
//    (destructor2.header)
//  }
}
*/
}
