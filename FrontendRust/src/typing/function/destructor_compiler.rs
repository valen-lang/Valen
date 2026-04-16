/*
package dev.vale.typing.function


import dev.vale.postparsing._
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.expression.CallCompiler
import dev.vale.{Err, Interner, Keywords, Ok, PackageCoordinate, RangeS, vassert, vfail, vimpl}
import dev.vale.highertyping._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules.OwnershipLiteralSL
import dev.vale.postparsing.GlobalFunctionFamilyNameS
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, CouldntFindFunctionToCallT, OverloadResolver, RangedInternalErrorT, TypingPassOptions}
import dev.vale.typing.ast.{DiscardTE, FunctionCallTE, PrototypeT, ReferenceExpressionTE}
import dev.vale.typing.env.{GlobalEnvironment, IInDenizenEnvironmentT, PackageEnvironmentT}
import dev.vale.typing.names.{IdT, PackageTopLevelNameT}
import dev.vale.typing.types._
import dev.vale.typing.{ast, _}
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.typing.names.PackageTopLevelNameT

import scala.collection.immutable.List

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

// mig: struct DestructorCompiler
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct DestructorCompiler<'s, 'ctx, 't>(pub std::marker::PhantomData<(&'s (), &'ctx (), &'t ())>);
// mig: impl DestructorCompiler
impl<'s, 'ctx, 't> DestructorCompiler<'s, 'ctx, 't> {}
/*
class DestructorCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    structCompiler: StructCompiler,
    overloadCompiler: OverloadResolver) {
*/
// mig: fn get_drop_function
fn get_drop_function() {
  panic!("Unimplemented: get_drop_function");
}
/*
  def getDropFunction(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    type2: CoordT):
  StampFunctionSuccess = {
    val name = interner.intern(CodeNameS(keywords.drop))
    val args = Vector(type2)
    overloadCompiler.findFunction(
      env, coutputs, callRange, callLocation, name, Vector.empty, Vector.empty, contextRegion, args, Vector(), true) match {
      case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
      case Ok(x) => x
    }
  }
*/
// mig: fn drop
fn drop() {
  panic!("Unimplemented: drop");
}
/*
  def drop(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    undestructedExpr2: ReferenceExpressionTE):
  (ReferenceExpressionTE) = {
    val resultExpr2 =
      undestructedExpr2.result.coord match {
        case CoordT(ShareT, _, NeverT(_)) => undestructedExpr2
        case CoordT(ShareT, _, _) => DiscardTE(undestructedExpr2)
        case r@CoordT(OwnT, _, _) => {
          val StampFunctionSuccess(destructorPrototype, _) =
            getDropFunction(env, coutputs, callRange, callLocation, RegionT(), r)
          vassert(coutputs.getInstantiationBounds(destructorPrototype.id).nonEmpty)
          val resultTT =
            destructorPrototype.returnType
          FunctionCallTE(destructorPrototype, Vector(undestructedExpr2), resultTT)
        }
        case CoordT(BorrowT, _, _) => (DiscardTE(undestructedExpr2))
        case CoordT(WeakT, _, _) => (DiscardTE(undestructedExpr2))
      }
    resultExpr2.result.coord.kind match {
      case VoidT() | NeverT(_) =>
      case _ => {
        throw CompileErrorExceptionT(
          RangedInternalErrorT(
            callRange,
            "Unexpected return type for drop autocall.\nReturn: " + resultExpr2.result.coord.kind + "\nParam: " + undestructedExpr2.result.coord))
      }
    }
    resultExpr2
  }
}
*/