use crate::utils::range::RangeS;

use crate::typing::templata::templata::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;

/*
package dev.vale.typing.macros

import dev.vale.postparsing.rules._
import dev.vale.postparsing.{CodeNameS, FunctorPrototypeRuneNameS}
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, vfail, vimpl, vwat}
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, CouldntEvaluateFunction, OverloadResolver}
import dev.vale.typing.ast.{ConstructTE, PrototypeT}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{ExpressionLookupContext, FunctionEnvironmentT, TemplataEnvEntry, TemplataLookupContext}
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function.FunctionCompiler
import dev.vale.typing.function._
import dev.vale.typing.names.{IFunctionNameT, RuneNameT}
import dev.vale.typing.templata.PrototypeTemplataT
import dev.vale.typing.types.CoordT
*/
// (Scala `class FunctorHelper(interner, keywords)` absorbed onto `Compiler`;
//  the method body lives at `Compiler::get_functor_for_prototype` below.)
/*
class FunctorHelper( interner: Interner, keywords: Keywords) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_functor_for_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        drop_function: PrototypeTemplataT<'s, 't>,
    ) -> ReinterpretTE<'s, 't> {
        panic!("Unimplemented: get_functor_for_prototype");
        // vfail()
    }
/*
  def getFunctorForPrototype(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    dropFunction: PrototypeTemplataT[IFunctionNameT]):
  ReinterpretTE = {
    vfail()
//    val functorTemplate =
//      env.lookupNearestWithImpreciseName(
//        interner.intern(CodeNameS(keywords.underscoresCall)), Set(ExpressionLookupContext)) match {
//        case Some(st@FunctionTemplata(_, _)) => st
//        case other => vwat(other)
//      }
//    val functorPrototypeTT =
//      functionCompiler.evaluateGenericLightFunctionFromCallForPrototype(
//          coutputs, callRange, env, functorTemplate, , args) match {
//        case (EvaluateFunctionSuccess(prototype)) => (prototype)
//        case (EvaluateFunctionFailure(fffr)) => {
//          throw CompileErrorExceptionT(CouldntEvaluateFunction(callRange, fffr))
//        }
//      }
//
//    ReinterpretTE(
//      VoidLiteralTE(),
//      CoordT(ShareT, FunctorT(functorPrototypeTT)))
  }
}
*/
}
