use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::names::{IImpreciseNameValS, RuneNameValS, CodeRuneS, IRuneValS, CodeNameS};
use crate::typing::env::environment::{ILookupContext, IInDenizenEnvironmentT};
use crate::typing::templata::templata::{ITemplataT, expect_mutability};
use crate::typing::types::types::RegionT;
use std::collections::HashSet;
use crate::typing::compiler_error_reporter::ICompileErrorT;

/*
package dev.vale.typing.macros.rsa

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CompileErrorExceptionT, CompilerErrorHumanizer, CompilerOutputs, CouldntFindFunctionToCallT, OverloadResolver, ast}
import dev.vale.typing.ast._
import dev.vale.typing.env.{FunctionEnvironmentT, TemplataLookupContext}
import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{Err, Interner, Keywords, Ok, Profiler, RangeS, StrI, vassert, vassertSome, vfail, vimpl, vwat}
import dev.vale.postparsing.CodeRuneS
import dev.vale.typing.ast._
import dev.vale.typing.env.TemplataLookupContext
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.templata.PrototypeTemplataT
import dev.vale.typing.types._
*/
// (Scala `class RSAImmutableNewMacro(interner, keywords, overloadResolver, arrayCompiler,
//  destructorCompiler)` absorbed onto `Compiler`; the method body lives at
//  `Compiler::generate_function_body_rsa_immutable_new` below.)
/*
class RSAImmutableNewMacro(
  interner: Interner,
  keywords: Keywords,
  overloadResolver: OverloadResolver,
  arrayCompiler: ArrayCompiler,
  destructorCompiler: DestructorCompiler
) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_imm_new
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_rsa_immutable_new(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> Result<(FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {

        let header = FunctionHeaderT {
            id: env.id,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            params: self.typing_interner.alloc_slice_from_vec(param_coords.to_vec()),
            return_type: maybe_ret_coord.expect("vassertSome: maybeRetCoord"),
            maybe_origin_function_templata: Some(env.templata()),
        };
        coutputs.declare_function_return_type(
            self.typing_interner.alloc(header.to_signature()),
            header.return_type,
        );

        let rune_e = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.e }));
        let rune_name_e = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS { rune: rune_e }));
        let element_type = match IInDenizenEnvironmentT::from(env).lookup_nearest_with_imprecise_name(rune_name_e, {
            let mut s = HashSet::new();
            s.insert(ILookupContext::TemplataLookupContext);
            s
        }, self.typing_interner).expect("vassertSome: E rune") {
            ITemplataT::Coord(ct) => ct.coord,
            _ => panic!("vwat"),
        };

        let rune_m = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.m }));
        let rune_name_m = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS { rune: rune_m }));
        let mutability = expect_mutability(
            IInDenizenEnvironmentT::from(env).lookup_nearest_with_imprecise_name(rune_name_m, {
                let mut s = HashSet::new();
                s.insert(ILookupContext::TemplataLookupContext);
                s
            }, self.typing_interner).expect("vassertSome: M rune"),
        );

        let array_tt = self.resolve_runtime_sized_array(element_type, mutability, RegionT { region: IRegionT::Default });

        let generator_arg_coord = match param_coords[1].tyype.ownership {
            OwnershipT::Share => CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: param_coords[1].tyype.kind },
            OwnershipT::Borrow => CoordT { ownership: OwnershipT::Borrow, region: RegionT { region: IRegionT::Default }, kind: param_coords[1].tyype.kind },
            OwnershipT::Own => panic!("vwat"), // shouldnt happen, signature takes in an &
            other => panic!("vwat: {:?}", other),
        };

        let func_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.underscores_call }));
        let generator_prototype = match self.find_function(
            IInDenizenEnvironmentT::from(env),
            coutputs,
            call_range,
            call_location,
            func_name,
            &[],
            &[],
            &[],
            RegionT { region: IRegionT::Default },
            &[generator_arg_coord, CoordT { ownership: OwnershipT::Share, region: RegionT { region: IRegionT::Default }, kind: KindT::Int(IntT::I32) }],
            &[],
            false,
        )? {
            Err(_e) => panic!("CouldntFindFunctionToCallT"),
            Ok(sfs) => sfs,
        };

        assert!(generator_prototype.prototype.return_type.ownership == OwnershipT::Share);

        let size_te = ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
            param_index: 0,
            coord: param_coords[0].tyype,
        }));
        let generator_te = ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
            param_index: 1,
            coord: param_coords[1].tyype,
        }));

        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE {
            inner: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                source_expr: ReferenceExpressionTE::NewImmRuntimeSizedArray(self.typing_interner.alloc(NewImmRuntimeSizedArrayTE {
                    array_type: self.typing_interner.alloc(array_tt),
                    region: RegionT { region: IRegionT::Default },
                    size_expr: size_te,
                    generator: generator_te,
                    generator_method: generator_prototype.prototype,
                })),
            })),
        }));
        Ok((header, body))
    }
/*
  def generateFunctionBody(
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
    val header =
      FunctionHeaderT(
        env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))
    coutputs.declareFunctionReturnType(header.toSignature, header.returnType)

    val CoordTemplataT(elementType) =
      vassertSome(
        env.lookupNearestWithImpreciseName(
          interner.intern(RuneNameS(CodeRuneS(keywords.E))), Set(TemplataLookupContext)))

    val mutability =
      ITemplataT.expectMutability(
        vassertSome(
          env.lookupNearestWithImpreciseName(
            interner.intern(RuneNameS(CodeRuneS(keywords.M))), Set(TemplataLookupContext))))

    val arrayTT = arrayCompiler.resolveRuntimeSizedArray(elementType, mutability, RegionT(DefaultRegionT))

    val generatorArgCoord =
      paramCoords(1).tyype match {
        case CoordT(ShareT, _, kind) => CoordT(ShareT, RegionT(DefaultRegionT), kind)
        case CoordT(BorrowT, _, kind) => CoordT(BorrowT, RegionT(DefaultRegionT), kind)
        case CoordT(OwnT, _, kind) => vwat() // shouldnt happen, signature takes in an &
      }

    val generatorPrototype =
      overloadResolver.findFunction(
        env,
        coutputs,
        callRange,
        callLocation,
        interner.intern(CodeNameS(keywords.underscoresCall)),
        Vector(),
        Vector(),
        Vector(),
        RegionT(DefaultRegionT),
        Vector(generatorArgCoord, CoordT(ShareT, RegionT(DefaultRegionT), IntT(32))),
        Vector(),
        false) match {
        case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
        case Ok(x) => x
      }

    vassert(generatorPrototype.prototype.returnType.ownership == ShareT)

    val sizeTE = ArgLookupTE(0, paramCoords(0).tyype)
    val generatorTE = ArgLookupTE(1, paramCoords(1).tyype)

    val body =
      BlockTE(
        ReturnTE(
          NewImmRuntimeSizedArrayTE(
            arrayTT,
            RegionT(DefaultRegionT),
            sizeTE,
            generatorTE,
            generatorPrototype.prototype)))
    (header, body)
  }
}
*/
}
