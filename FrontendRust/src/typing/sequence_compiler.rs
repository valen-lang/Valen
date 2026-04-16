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
    pub opts: TypingPassOptions<'s>,
    pub interner: &'ctx Interner<'s>,
    pub keywords: &'ctx Keywords<'s>,
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
impl<'s, 'ctx, 't> SequenceCompiler<'s, 'ctx, 't> {
fn resolve_tuple(
    &self,
    env: &IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    exprs: Vec<ReferenceExpressionTE<'s, 't>>,
) -> ReferenceExpressionTE<'s, 't> {
    panic!("Unimplemented: resolve_tuple");
}
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
impl<'s, 'ctx, 't> SequenceCompiler<'s, 'ctx, 't> {
fn make_tuple_kind(
    &self,
    env: &IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    types: Vec<CoordT<'s, 't>>,
) -> StructTT<'s, 't> {
    panic!("Unimplemented: make_tuple_kind");
}
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
impl<'s, 'ctx, 't> SequenceCompiler<'s, 'ctx, 't> {
fn make_tuple_coord(
    &self,
    env: &IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    types: Vec<CoordT<'s, 't>>,
) -> CoordT<'s, 't> {
    panic!("Unimplemented: make_tuple_coord");
}
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
