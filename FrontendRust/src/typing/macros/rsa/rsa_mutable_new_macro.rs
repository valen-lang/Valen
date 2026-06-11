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
use crate::postparsing::names::{IImpreciseNameValS, RuneNameValS, CodeRuneS, IRuneValS};
use crate::typing::env::environment::{ILookupContext, IInDenizenEnvironmentT};
use crate::typing::templata::templata::{ITemplataT, expect_mutability};
use crate::typing::types::types::{IRegionT, RegionT};
use std::collections::HashSet;

/*
package dev.vale.typing.macros.rsa

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CompilerOutputs, ast}

import dev.vale.typing.ast._
import dev.vale.typing.env.{FunctionEnvironmentT, TemplataLookupContext}

import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, vassert, vassertSome, vimpl}

import dev.vale.postparsing.CodeRuneS
import dev.vale.typing.ast._
import dev.vale.typing.env.TemplataLookupContext
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.templata.MutabilityTemplataT
import dev.vale.typing.types.RuntimeSizedArrayTT

*/
// (Scala `class RSAMutableNewMacro(interner, keywords, arrayCompiler, destructorCompiler)`
//  absorbed onto `Compiler`; the method body lives at
//  `Compiler::generate_function_body_rsa_mutable_new` below.)
/*
class RSAMutableNewMacro(
  interner: Interner,
  keywords: Keywords,
  arrayCompiler: ArrayCompiler,
  destructorCompiler: DestructorCompiler
) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_mut_new

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_rsa_mutable_new(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'t>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {

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

        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE {
            inner: ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE {
                source_expr: ReferenceExpressionTE::NewMutRuntimeSizedArray(self.typing_interner.alloc(NewMutRuntimeSizedArrayTE {
                    array_type: self.typing_interner.alloc(array_tt),
                    region: RegionT { region: IRegionT::Default },
                    capacity_expr: ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE {
                        param_index: 0,
                        coord: param_coords[0].tyype,
                    })),
                })),
            })),
        }));
        (header, body)
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

    val body =
      BlockTE(
        ReturnTE(
          NewMutRuntimeSizedArrayTE(
            arrayTT,
            RegionT(DefaultRegionT),
            ArgLookupTE(0, paramCoords(0).tyype))))
//            freePrototype)))
    (header, body)
  }

}
*/
}
