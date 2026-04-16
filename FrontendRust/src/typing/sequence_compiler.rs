/*
package dev.vale.typing

import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.{Interner, Keywords, Profiler, RangeS, vassert, vassertSome, vimpl}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{IInDenizenEnvironmentT, TemplataLookupContext}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.typing.citizen.StructCompilerCore
import dev.vale.typing.env.PackageEnvironmentT
import dev.vale.typing.function.FunctionCompiler

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
use crate::typing::compilation::*;
use crate::interner::Interner;
use crate::typing::templata_compiler::*;
use crate::typing::citizen::struct_compiler::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::itemplatatype::ITemplataType;

// mig: struct SequenceCompiler
pub struct SequenceCompiler<'s, 'ctx, 't> {
    pub opts: TypingPassOptions,
    pub interner: &'ctx Interner,
    pub keywords: &'ctx Keywords,
    pub struct_compiler: StructCompiler<'s, 'ctx, 't>,
    pub templata_compiler: TemplataCompiler<'s, 'ctx, 't>,
}
// mig: impl SequenceCompiler
impl<'s, 'ctx, 't> SequenceCompiler<'s, 'ctx, 't> {}
/*
class SequenceCompiler(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
    structCompiler: StructCompiler,
    templataCompiler: TemplataCompiler) {
*/
// mig: fn resolve_tuple
fn resolve_tuple(
    &self,
    env: &IInDenizenEnvironmentT,
    coutputs: &mut CompilerOutputs,
    parent_ranges: &[RangeS],
    call_location: LocationInDenizen,
    exprs: Vec<ReferenceExpressionTE>,
) -> ReferenceExpressionTE {
    panic!("Unimplemented: resolve_tuple");
}
/*
  def resolveTuple(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
    exprs2: Vector[ReferenceExpressionTE]):
  (ReferenceExpressionTE) = {
    val types2 = exprs2.map(_.result.expectReference().coord)
    val region = RegionT()
    val finalExpr = TupleTE(exprs2, makeTupleCoord(env, coutputs, parentRanges, callLocation, region, types2))
    (finalExpr)
  }
*/
// mig: fn make_tuple_kind
fn make_tuple_kind(
    &self,
    env: &IInDenizenEnvironmentT,
    coutputs: &mut CompilerOutputs,
    parent_ranges: &[RangeS],
    call_location: LocationInDenizen,
    types: Vec<CoordT>,
) -> StructTT {
    panic!("Unimplemented: make_tuple_kind");
}
/*
  def makeTupleKind(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
    types2: Vector[CoordT]):
  StructTT = {
    val tupleTemplate @ StructDefinitionTemplataT(_, _) =
      vassertSome(
        env.lookupNearestWithName(
          interner.intern(StructTemplateNameT(keywords.tupleHumanName(types2.length))), Set(TemplataLookupContext)))
    structCompiler.resolveStruct(
      coutputs,
      env,
      RangeS.internal(interner, -17653) :: parentRanges,
      callLocation,
      tupleTemplate,
//      Vector(CoordListTemplata(types2))).kind
      types2.map(CoordTemplataT)).expect().kind
  }
*/
// mig: fn make_tuple_coord
fn make_tuple_coord(
    &self,
    env: &IInDenizenEnvironmentT,
    coutputs: &mut CompilerOutputs,
    parent_ranges: &[RangeS],
    call_location: LocationInDenizen,
    region: RegionT,
    types: Vec<CoordT>,
) -> CoordT {
    panic!("Unimplemented: make_tuple_coord");
}
/*
  def makeTupleCoord(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
    region: RegionT,
    types2: Vector[CoordT]):
  CoordT = {
    templataCompiler.coerceKindToCoord(
      coutputs, makeTupleKind(env, coutputs, parentRanges, callLocation, types2), region)
  }
}
*/
