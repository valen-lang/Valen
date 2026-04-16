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
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;
use crate::keywords::Keywords;
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
use crate::interner::Interner;
use crate::typing::overload_resolver::*;
use crate::typing::citizen::struct_compiler::*;
use crate::typing::function::function_compiler::*;
use crate::postparsing::rules::rules::*;
// mig: struct FunctorHelper
pub struct FunctorHelper<'s, 'ctx, 't> {
    pub interner: Interner<'s>,
    pub keywords: Keywords<'s>,
}

// mig: impl FunctorHelper
impl<'s, 'ctx, 't> FunctorHelper<'s, 'ctx, 't> {}

/*
class FunctorHelper( interner: Interner, keywords: Keywords) {
*/
// mig: fn get_functor_for_prototype
fn get_functor_for_prototype(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    call_range: Vec<RangeS>,
    drop_function: PrototypeTemplataT<IFunctionNameT>,
) -> ReinterpretTE {
    panic!("Unimplemented: get_functor_for_prototype");
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
