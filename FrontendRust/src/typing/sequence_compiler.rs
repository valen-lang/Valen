use crate::postparsing::ast::LocationInDenizen;
use crate::typing::compiler::Compiler;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::interner::Interner;
use crate::typing::citizen::struct_compiler::IResolveOutcome;
use std::collections::HashSet;
use std::iter::once;
use std::marker::PhantomData;

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
/*
class SequenceCompiler(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
    structCompiler: StructCompiler,
    templataCompiler: TemplataCompiler) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_tuple(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        exprs: Vec<ReferenceExpressionTE<'s, 't>>,
    ) -> ReferenceExpressionTE<'s, 't> {
        let types_2: Vec<CoordT<'s, 't>> = exprs.iter().map(|e| IExpressionResultT::Reference(e.result()).expect_reference().coord).collect();
        let region = RegionT { region: IRegionT::Default };
        let final_expr = ReferenceExpressionTE::Tuple(self.typing_interner.alloc(TupleTE {
            elements: self.typing_interner.alloc_slice_from_vec(exprs),
            result_reference: self.make_tuple_coord(env, coutputs, parent_ranges, call_location, region, types_2),
        }));
        final_expr
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
    val region = RegionT(DefaultRegionT)
    val finalExpr = TupleTE(exprs2, makeTupleCoord(env, coutputs, parentRanges, callLocation, region, types2))
    (finalExpr)
  }
*/
    pub fn make_tuple_kind(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        types: Vec<CoordT<'s, 't>>,
    ) -> StructTT<'s, 't> {
        let tuple_template_name = self.typing_interner.intern_struct_template_name(StructTemplateNameT { human_name: self.keywords.tuple_human_name[types.len()], _phantom: PhantomData });
        let tuple_template = match env.lookup_nearest_with_name(INameT::StructTemplate(tuple_template_name), {
            let mut s = HashSet::new();
            s.insert(ILookupContext::TemplataLookupContext);
            s
        }, self.typing_interner).unwrap() {
            ITemplataT::StructDefinition(t) => *t,
            _ => panic!("make_tuple_kind: expected StructDefinitionTemplataT"),
        };
        let new_parent_ranges: Vec<RangeS<'s>> = once(RangeS::internal(self.scout_arena, -17653)).chain(parent_ranges.iter().copied()).collect();
        let uncoerced_template_args: Vec<ITemplataT<'s, 't>> = types.iter().map(|c| ITemplataT::Coord(self.typing_interner.alloc(CoordTemplataT { coord: *c }))).collect();
        match self.resolve_struct(coutputs, env, self.typing_interner.alloc_slice_from_vec(new_parent_ranges), call_location, tuple_template, &uncoerced_template_args) {
            IResolveOutcome::ResolveSuccess(s) => s.kind,
            IResolveOutcome::ResolveFailure(_) => panic!("make_tuple_kind: resolve_struct failed"),
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
    pub fn make_tuple_coord(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        types: Vec<CoordT<'s, 't>>,
    ) -> CoordT<'s, 't> {
        let tuple_kind = self.make_tuple_kind(env, coutputs, parent_ranges, call_location, types);
        self.coerce_kind_to_coord(coutputs, KindT::Struct(self.typing_interner.alloc(tuple_kind)), region)
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
}
