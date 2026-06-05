use crate::lexing::ast::RangeL;
use crate::parsing::ast::{
  BlockPE, DotPE, FunctionCallPE, IArraySizeP, IExpressionPE, IImpreciseNameP, ITemplexPT, LoadAsP,
  LookupPE, MutabilityP, NameP, OwnershipP, StaticSizedArraySizeP, VariabilityP,
};
use crate::interner::StrI;
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::ast::IExpressionSE as IExpressionSETrait;
use crate::postparsing::expressions::{
  BlockSE, ConstantBoolSE, ConstantIntSE, ConstantStrSE, DestructSE, DotSE, ExprMutateSE, FunctionCallSE, FunctionSE,
  IExpressionSE, IfSE, IndexSE, LetSE, LoadPartSE, LocalLoadSE, LocalMutateSE, LocalS, NewRuntimeSizedArraySE, OutsideLoadSE, OverloadSetSE,
  OwnershippedSE, PureSE, ReturnSE, RuneLookupSE, StaticArrayFromCallableSE, StaticArrayFromValuesSE, TemplataLoadSE,
  TupleSE, VoidSE,
};
use crate::postparsing::names::ImplicitRuneValS;
use crate::postparsing::rules::rules::{
  ILiteralSL, IRulexSR, IntLiteralSL, LiteralSR, MutabilityLiteralSL, RuneUsage, VariabilityLiteralSL,
};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, IFunctionDeclarationNameS, IImpreciseNameS,
  IImpreciseNameValS, IRuneS, IRuneValS, IVarNameS,
};
use crate::postparsing::post_parser::{
  CouldntFindRuneS, CouldntFindVarToMutateS, FunctionEnvironmentS, ICompileErrorS, IEnvironmentS,
  InitializingRuntimeSizedArrayRequiresSizeAndCallable,
  InitializingStaticSizedArrayRequiresSizeAndCallable, PostParser, StackFrame, StatementAfterReturnS,
  VariableNameAlreadyExists,
};
use crate::postparsing::post_parser::translate_imprecise_name;
use crate::postparsing::patterns::pattern_scout::{get_parameter_captures, translate_pattern};
use crate::postparsing::rules::rule_scout::translate_rulexes;
use crate::postparsing::rules::templex_scout::translate_templex;
use crate::postparsing::loop_post_parser::{scout_each, scout_while};
use crate::postparsing::variable_uses::{VariableDeclarations, VariableUses};
use crate::utils::range::RangeS;
use crate::postparsing::expressions::ConstantFloatSE;

/*
package dev.vale.postparsing

import dev.vale.postparsing.patterns.PatternScout
import dev.vale.postparsing.rules.{IRulexSR, IntLiteralSL, LiteralSR, MaybeCoercingCallSR, MutabilityLiteralSL, RuleScout, RuneUsage, TemplexScout, VariabilityLiteralSL}
import dev.vale.parsing.ast._
import dev.vale.parsing.{ast, _}
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, postparsing, vassert, vassertSome, vcurious, vfail, vimpl, vwat}
import PostParser.{evalRange, noDeclarations, noVariableUses}
import dev.vale
import dev.vale.lexing.RangeL
import dev.vale.parsing.ast
import dev.vale.parsing.ast.{AndPE, AugmentPE, BinaryCallPE, BlockPE, BorrowP, BraceCallPE, BreakPE, CallPT, ConsecutorPE, ConstantBoolPE, ConstantFloatPE, ConstantIntPE, ConstantStrPE, ConstructArrayPE, DestructPE, DotPE, EachPE, FinalP, FunctionCallPE, FunctionP, IExpressionPE, ITemplexPT, IfPE, IndexPE, LambdaPE, LetPE, LoadAsBorrowP, LoadAsP, LoadAsWeakP, LookupNameP, LookupPE, MagicParamLookupPE, MethodCallPE, MoveP, MutableP, MutatePE, NameOrRunePT, NameP, NotPE, OrPE, PackPE, RangePE, ReturnPE, RuntimeSizedP, ShortcallPE, StaticSizedP, StrInterpolatePE, SubExpressionPE, TemplateArgsP, TuplePE, UnletPE, UseP, VoidPE, WeakP, WhilePE}
import dev.vale.postparsing.patterns.PatternScout
//import dev.vale.postparsing.predictor.{Conclusions, PredictorEvaluator}
import dev.vale.postparsing.rules.TemplexScout

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
//import dev.vale.postparsing.predictor.Conclusions
import dev.vale.postparsing.rules.RuleScout
//import dev.vale.postparsing.templatepredictor.PredictorEvaluator
*/
/*
trait IExpressionScoutDelegate {
  // MIGALLOW: dont need to bring this trait method into rust.
  def scoutLambda(
    parentStackFrame: StackFrame,
    lambdaFunction0: FunctionP):
  (FunctionS, VariableUses)
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum IScoutResult<'s, 'p> {
  LocalLookupResult(LocalLookupResultS<'s>),
  OutsideLookupResult(OutsideLookupResultS<'s, 'p>),
  NormalResult(NormalResultS<'s>),
}
/*
// MIGALLOW: Rust IScoutResult doesn't need to be generic, because we never made use of that in
// Scala.
sealed trait IScoutResult[+T <: IExpressionSE]
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct LocalLookupResultS<'s> {
  range: RangeS<'s>,
  name: IVarNameS<'s>,
}
/*
// Will contain the address of a local.
case class LocalLookupResult(range: RangeS, name: IVarNameS) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct OutsideLookupResultS<'s, 'p> {
  range: RangeS<'s>,
  name: StrI<'s>,
  template_args: Option<&'p [&'p ITemplexPT<'p>]>,
}
/*
// Looks up something that's not a local.
// Should be just a function, but its also super likely that the user just forgot
// to declare a variable, and we interpreted it as an outside lookup.
case class OutsideLookupResult(
  range: RangeS,
  name: StrI,
  maybeTemplateArgs: Option[Vector[ITemplexPT]]
) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct NormalResultS<'s> {
  pub(crate) expr: &'s IExpressionSE<'s>,
}

/*
// Anything else, such as:
// - Result of a function call
// - Address inside a struct
case class NormalResult[+T <: IExpressionSE](expr: T) extends IScoutResult[T] {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  def range: RangeS = expr.range
}
*/
impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
/*
class ExpressionScout(
    delegate: IExpressionScoutDelegate,
    templexScout: TemplexScout,
    ruleScout: RuleScout,
    patternScout: PatternScout,
    interner: Interner,
    keywords: Keywords) {
  val loopPostParser = new LoopPostParser(interner, keywords)
*/
fn ends_with_return(_expr_se: &IExpressionSE<'s>) -> bool {
  panic!("Unimplemented ends_with_return");
}
/*
  def endsWithReturn(exprSE: IExpressionSE): Boolean = {
    exprSE match {
      case ReturnSE(_, _) => true
      case ConsecutorSE(exprs) => endsWithReturn(exprs.last)
      case _ => false
    }
  }
*/
pub(crate) fn scout_block(
  &self,
  parent_stack_frame: StackFrame<'s>,
  lidb: &mut LocationInDenizenBuilder,
  // When we scout a function, it might hand in things here because it wants them to be considered part of
  // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
  initial_locals: VariableDeclarations<'s>,
  block_pe: &'p BlockPE<'p>,
) -> Result<(&'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
{
  let file = parent_stack_frame.file;
  let range_s = PostParser::eval_range(file, block_pe.range);
  assert!(block_pe.maybe_default_region.is_none());
  let context_region: IRuneS<'s> = match &block_pe.maybe_default_region {
    None => parent_stack_frame.context_region.clone(),
    Some(region_rune_pt) => {
      let region_rune_name = region_rune_pt
        .name
        .as_ref()
        .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_BLOCK_DEFAULT_REGION_NAME_MISSING"));
      // Re-intern string from 'p into 's for cross-arena translation
      let region_rune_name_s: StrI<'s> = self.scout_arena.intern_str(region_rune_name.str().as_str());
      let region_rune_s: IRuneS<'s> = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS::<'s> {
        name: region_rune_name_s,
      }));
      if !parent_stack_frame.parent_env.all_declared_runes().contains(&region_rune_s) {
        return Err(ICompileErrorS::CouldntFindRuneS(CouldntFindRuneS {
          range: range_s.clone(),
          name: region_rune_name.str().as_str().to_string(),
        }));
      }
      region_rune_s
    }
  };
  let function_body_env: FunctionEnvironmentS<'s> = parent_stack_frame.parent_env.clone();
  let mut child_lidb = lidb.child();
  let (block_s, self_uses_of_things_from_above, child_uses_of_things_from_above) = self.new_block(
    function_body_env,
    Some(parent_stack_frame),
    &mut child_lidb,
    range_s,
    context_region,
    initial_locals,
    |stack_frame1, block_lidb| {
      let (stack_frame2, inner_expr_s, self_uses, child_uses) =
        self.scout_expression_and_coerce(stack_frame1, block_lidb, block_pe.inner, LoadAsP::Use)?;
      Ok((stack_frame2, inner_expr_s, self_uses, child_uses))
    },
  )?;
  let resulting_expr_s = if block_pe.maybe_pure.is_some() {
    let block_s = &*self.scout_arena.alloc(block_s);
    &*self.scout_arena.alloc(IExpressionSE::Pure(PureSE {
      range: PostParser::eval_range(file, block_pe.range),
      location: lidb.child().consume_in_arena(self.scout_arena),
      inner: &*self.scout_arena.alloc(IExpressionSE::Block(block_s)),
    }))
  } else {
    let block_s = &*self.scout_arena.alloc(block_s);
    &*self.scout_arena.alloc(IExpressionSE::Block(block_s))
  };
  Ok((
    resulting_expr_s,
    self_uses_of_things_from_above,
    child_uses_of_things_from_above,
  ))
}
/*
  def scoutBlock(
    parentStackFrame: StackFrame,
    lidb: LocationInDenizenBuilder,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    blockPE: BlockPE):
  (IExpressionSE, VariableUses, VariableUses) = {
    val BlockPE(rangeP, pure, maybeNewDefaultRegion, inner) = blockPE
    val rangeS = PostParser.evalRange(parentStackFrame.file, rangeP)
    vassert(maybeNewDefaultRegion.isEmpty)
    val (blockSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove) =
      newBlock(
        parentStackFrame.parentEnv,
        Some(parentStackFrame),
        lidb.child(),
        rangeS,
        maybeNewDefaultRegion match {
          case None => parentStackFrame.contextRegion
          case Some(RegionRunePT(range, name)) => {
            val regionRuneS = CodeRuneS(vassertSome(name).str) // impl isolates
            if (!parentStackFrame.parentEnv.allDeclaredRunes().contains(regionRuneS)) {
              throw CompileErrorExceptionS(CouldntFindRuneS(rangeS, vassertSome(name).str.str)) // impl isolates
            }
            regionRuneS
          }
        },
        initialLocals,
        (stackFrame1, lidb) => {
          val (stackFrame2, innerExprSE, selfUses, childUses) =
            scoutExpressionAndCoerce(
              stackFrame1, lidb, inner, UseP)
          (stackFrame2, innerExprSE, selfUses, childUses)
        })
    val resultingExprSE =
      if (blockPE.maybePure.nonEmpty) {
        PureSE(evalRange(parentStackFrame.file, blockPE.range), lidb.child().consume(), blockSE)
      } else {
        blockSE
      }
    (resultingExprSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
  }
*/
fn scout_impure_block(
  &self,
  parent_stack_frame: StackFrame<'s>,
  lidb: &mut LocationInDenizenBuilder,
  initial_locals: VariableDeclarations<'s>,
  block_pe: &'p BlockPE<'p>,
) -> Result<(&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
{
  let (expr_s, self_uses_of_things_from_above, child_uses_of_things_from_above) = self.scout_block(
    parent_stack_frame,
    lidb,
    initial_locals,
    block_pe,
  )?;
  match expr_s {
    IExpressionSE::Block(block_s) => Ok((
      block_s,
      self_uses_of_things_from_above,
      child_uses_of_things_from_above,
    )),
    _ => panic!("POSTPARSER_SCOUT_IMPURE_BLOCK_EXPECTED_BLOCK"),
  }
}
/*
  def scoutImpureBlock(
    parentStackFrame: StackFrame,
    lidb: LocationInDenizenBuilder,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    blockPE: BlockPE):
  (BlockSE, VariableUses, VariableUses) = {
    val (exprSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove) =
      scoutBlock(parentStackFrame, lidb, initialLocals, blockPE)
    exprSE match {
      case b @ BlockSE(_, _, _) => (b, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
      case other => vfail("Expected impure block!")
    }
  }
*/
  pub(crate) fn new_block<F>(
    &self,
    function_body_env: FunctionEnvironmentS<'s>,
    parent_stack_frame: Option<StackFrame<'s>>,
    lidb: &mut LocationInDenizenBuilder,
    range_s: RangeS<'s>,
    context_region: IRuneS<'s>,
    initial_locals: VariableDeclarations<'s>,
    scout_contents: F,
  ) -> Result<(&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
  where
    F: FnOnce(
      StackFrame<'s>,
      &mut LocationInDenizenBuilder,
    ) -> Result<
      (
        StackFrame<'s>,
        &'s IExpressionSE<'s>,
        VariableUses<'s>,
        VariableUses<'s>,
      ),
      ICompileErrorS<'s>,
    >,
  {
    let maybe_parent = parent_stack_frame.clone().map(Box::new);
    let pure_height = parent_stack_frame
      .map(|parent| parent.pure_height + 1)
      .unwrap_or(0);
    let initial_stack_frame = StackFrame::<'s> {
      file: function_body_env.file,
      name: function_body_env.name.clone(),
      parent_env: function_body_env,
      maybe_parent,
      context_region,
      pure_height,
      locals: initial_locals,
    };
    let mut inner_lidb = lidb.child();
    let (
      stack_frame_before_constructing,
      expr_without_constructing_without_void,
      self_uses_before_constructing,
      child_uses_before_constructing,
    ) = scout_contents(initial_stack_frame, &mut inner_lidb)?;

    // If we had for example:
    //   func MyStruct() {
    //     this.a = 5;
    //     println("flamscrankle");
    //     this.b = true;
    //   }
    // then here's where we insert the final
    //     MyStruct(`this.a`, `this.b`);
    let constructing_member_names: Vec<StrI<'s>> = stack_frame_before_constructing
      .locals
      .vars
      .iter()
      .filter_map(|declared| match &declared.name {
        IVarNameS::ConstructingMemberName(member_name) => Some(*member_name),
        _ => None,
      })
      .collect();
    let (
      stack_frame_after_constructing,
      expr_with_constructing_if_necessary,
      self_uses,
      child_uses,
    ): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = if constructing_member_names.is_empty() {
      // Add a void to the end, unless:
      // - we're constructing
      // - we end with a return
      // - a result was requested.
      (
        stack_frame_before_constructing,
        expr_without_constructing_without_void,
        self_uses_before_constructing,
        child_uses_before_constructing,
      )
    } else {
      // Per @PPSPASTNZ, synthesize a constructor call as parser AST, then scout it.
      let function_name = match &stack_frame_before_constructing.parent_env.name {
        IFunctionDeclarationNameS::FunctionName(function_name_s) => {
          // Re-intern into 'p arena for synthetic parser node (see @PPSPASTNZ)
          self.parse_arena.intern_str(function_name_s.name.as_str())
        }
        _ => panic!("POSTPARSER_NEW_BLOCK_EXPECTED_FUNCTION_NAME"),
      };
      let range_at_end = RangeL(range_s.end.offset, range_s.end.offset);
      // Per @PPSPASTNZ, allocate synthetic parser node in parse_arena
      let callable_expr_p = &*self.parse_arena.alloc(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(range_at_end, function_name)),
        template_args: None,
      })));
      // Per @PPSPASTNZ, all synthetic parser nodes allocated in parse_arena ('p)
      let self_keyword_p = self.parse_arena.intern_str(self.keywords.self_.as_str());
      let arg_exprs_p: Vec<&'p IExpressionPE<'p>> = constructing_member_names
        .iter()
        .map(|member_name: &StrI<'s>| -> &'p IExpressionPE<'p> {
          let self_lookup_p = &*self.parse_arena.alloc(IExpressionPE::Lookup(self.parse_arena.alloc(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(range_at_end, self_keyword_p)),
            template_args: None,
          })));
          let member_name_p = self.parse_arena.intern_str(member_name.as_str());
          &*self.parse_arena.alloc(IExpressionPE::Dot(DotPE {
            range: range_at_end,
            left: self_lookup_p,
            operator_range: RangeL::zero(),
            member: NameP(range_at_end, member_name_p),
          }))
        })
        .collect();
      // Per @PPSPASTNZ, allocate synthetic parser node in parse_arena
      let constructor_call_p: &'p IExpressionPE<'p> = &*self.parse_arena.alloc(IExpressionPE::FunctionCall(FunctionCallPE {
        range: range_at_end,
        operator_range: RangeL::zero(),
        callable_expr: callable_expr_p,
        arg_exprs: self.parse_arena.alloc_slice_from_vec(arg_exprs_p),
      }));
      let mut constructor_lidb = lidb.child();
      let (
        stack_frame_after_constructing,
        constructor_result,
        self_uses_after_constructing,
        child_uses_after_constructing,
      ) = self.scout_expression(
        stack_frame_before_constructing,
        &mut constructor_lidb,
        constructor_call_p,
      )?;
      let construct_expression = match constructor_result {
        IScoutResult::NormalResult(NormalResultS { expr }) => expr,
        _ => panic!("POSTPARSER_NEW_BLOCK_CONSTRUCTOR_SCOUT_RESULT_NOT_NORMAL"),
      };
      let expr_after_constructing =
        self.consecutive(vec![expr_without_constructing_without_void, construct_expression]);
      (
        stack_frame_after_constructing,
        expr_after_constructing,
        self_uses_before_constructing.then_merge(&self_uses_after_constructing),
        child_uses_before_constructing.then_merge(&child_uses_after_constructing),
      )
    };
    let locals: Vec<LocalS<'s>> = stack_frame_after_constructing
      .locals
      .vars
      .iter()
      .map(|declared| LocalS {
        var_name: declared.name.clone(),
        self_borrowed: self_uses.is_borrowed(&declared.name),
        self_moved: self_uses.is_moved(&declared.name),
        self_mutated: self_uses.is_mutated(&declared.name),
        child_borrowed: child_uses.is_borrowed(&declared.name),
        child_moved: child_uses.is_moved(&declared.name),
        child_mutated: child_uses.is_mutated(&declared.name),
      })
      .collect();
    let self_uses_of_things_from_above = VariableUses {
      uses: self_uses
        .uses
        .iter()
        .filter(|use_| !locals.iter().any(|local| local.var_name == use_.name))
        .cloned()
        .collect(),
    };
    let child_uses_of_things_from_above = VariableUses {
      uses: child_uses
        .uses
        .iter()
        .filter(|use_| !locals.iter().any(|local| local.var_name == use_.name))
        .cloned()
        .collect(),
    };
    Ok((
      &*self.scout_arena.alloc(
      BlockSE::<'s> {
        range: range_s,
        locals: self.scout_arena.alloc_slice_from_vec(locals),
        expr: expr_with_constructing_if_necessary,
      }),
      self_uses_of_things_from_above,
      child_uses_of_things_from_above,
    ))
  }
/*
  def newBlock(
    functionBodyEnv: FunctionEnvironmentS,
    parentStackFrame: Option[StackFrame],
    lidb: LocationInDenizenBuilder,
    rangeS: RangeS,
    contextRegion: IRuneS,
    // When we scout a function, it might hand in things here because it wants them to be considered part of
    // the body's block, so that we get to reuse the code at the bottom of function, tracking uses etc.
    initialLocals: VariableDeclarations,
    // If there's anything else we'd like to put at the end of the block, we can pass it in here
    scoutContents: (StackFrame, LocationInDenizenBuilder) => (StackFrame, IExpressionSE, VariableUses, VariableUses)):
  (BlockSE, VariableUses, VariableUses) = {
    val initialStackFrame =
      StackFrame(
        functionBodyEnv.file,
        functionBodyEnv.name,
        functionBodyEnv,
        parentStackFrame,
        contextRegion,
        parentStackFrame.map(_.pureHeight + 1).getOrElse(0),
        initialLocals)
//    val rangeS = evalRange(functionBodyEnv.file, blockPE.range)
//
//    val (stackFrameBeforeExtrasAndConstructing, exprsWithoutExtrasWithoutConstructingWithoutVoidS, selfUsesBeforeExtrasAndConstructing, childUsesBeforeExtrasAndConstructing) =
//      scoutElementsAsExpressions(initialStackFrame, lidb.child(), blockPE.elements)

    val (stackFrameBeforeConstructing, exprWithoutConstructingWithoutVoidS, selfUsesBeforeConstructing, childUsesBeforeConstructing) =
      scoutContents(initialStackFrame, lidb.child())

    // If we had for example:
    //   func MyStruct() {
    //     this.a = 5;
    //     println("flamscrankle");
    //     this.b = true;
    //   }
    // then here's where we insert the final
    //     MyStruct(`this.a`, `this.b`);
    val constructedMembersNames =
      stackFrameBeforeConstructing.locals.vars.map(_.name).collect({ case ConstructingMemberNameS(n) => n })

    val (stackFrame, exprWithConstructingIfNecessary, selfUses, childUses) =
      if (constructedMembersNames.isEmpty) {
        // Add a void to the end, unless:
        // - we're constructing
        // - we end with a return
        // - a result was requested.
        val exprsS = exprWithoutConstructingWithoutVoidS
        (stackFrameBeforeConstructing, exprsS, selfUsesBeforeConstructing, childUsesBeforeConstructing)
      } else {
        val rangeAtEnd = RangeL(rangeS.end.offset, rangeS.end.offset)
        val constructorCallP =
          FunctionCallPE(
            rangeAtEnd,
            RangeL.zero,
            LookupPE(
              stackFrameBeforeConstructing.parentEnv.name match {
                case FunctionNameS(n, _) => LookupNameP(NameP(rangeAtEnd, n))
                case _ => vwat()
              }, None),
            constructedMembersNames.map(n => DotPE(rangeAtEnd, LookupPE(LookupNameP(NameP(rangeAtEnd, keywords.self)), None), RangeL.zero, NameP(rangeAtEnd, n))))

        val (stackFrameAfterConstructing, NormalResult(constructExpression), selfUsesAfterConstructing, childUsesAfterConstructing) =
          scoutExpression(stackFrameBeforeConstructing, lidb.child(), constructorCallP)
        val exprAfterConstructing =
          PostParser.consecutive(
            Vector(
              exprWithoutConstructingWithoutVoidS,
              constructExpression))
        (stackFrameAfterConstructing, exprAfterConstructing, selfUsesBeforeConstructing.thenMerge(selfUsesAfterConstructing), childUsesBeforeConstructing.thenMerge(childUsesAfterConstructing))
      }

    val locals =
        stackFrame.locals.vars.map({ declared =>
        LocalS(
          declared.name,
          selfUses.isBorrowed(declared.name),
          selfUses.isMoved(declared.name),
          selfUses.isMutated(declared.name),
          childUses.isBorrowed(declared.name),
          childUses.isMoved(declared.name),
          childUses.isMutated(declared.name))
      })

    val selfUsesOfThingsFromAbove =
      VariableUses(selfUses.uses.filter(selfUseName => !locals.map(_.varName).contains(selfUseName.name)))
    val childUsesOfThingsFromAbove =
      VariableUses(childUses.uses.filter(selfUseName => !locals.map(_.varName).contains(selfUseName.name)))

    val blockSE = BlockSE(rangeS, locals, exprWithConstructingIfNecessary)
    (blockSE, selfUsesOfThingsFromAbove, childUsesOfThingsFromAbove)
  }
*/
fn find_local(
  &self,
  stack_frame: &StackFrame<'s>,
  range: RangeS<'s>,
  imprecise_name: &IImpreciseNameS<'s>,
) -> Option<LocalLookupResultS<'s>> {
  stack_frame
    .find_variable(imprecise_name)
    .map(|full_name| LocalLookupResultS::<'s> { range, name: full_name })
}

/*
  def findLocal(stackFrame0: StackFrame, rangeS: RangeS, impreciseNameS: IImpreciseNameS):
  Option[LocalLookupResult] = {
    stackFrame0.findVariable(impreciseNameS) match {
      case Some(fullName) => Some(LocalLookupResult(rangeS, fullName))
      case None => None
    }
  }
*/

// Returns:
// - new seq num
// - declared variables
// - new expression
// - variable uses by self
// - variable uses by child blocks
// AFTERM: rename all "scout" to "post parse" or something.
fn scout_expression(
  &self,
  stack_frame: StackFrame<'s>,
  lidb: &mut LocationInDenizenBuilder,
  expression: &'p IExpressionPE<'p>,
) -> Result<(StackFrame<'s>, IScoutResult<'s, 'p>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
{
/*
  // Returns:
  // - new seq num
  // - declared variables
  // - new expression
  // - variable uses by self
  // - variable uses by child blocks
  private def translateMaybeTemplateArgs(
    parentEnv: IEnvironmentS,
    lidb: LocationInDenizenBuilder,
    ruleBuilder: ArrayBuffer[IRulexSR],
    contextRegion: IRuneS,
    maybeTemplateArgs: Option[Vector[ITemplexPT]]):
  Vector[RuneUsage] = {
    maybeTemplateArgs match {
      case None => Vector()
      case Some(templateArgs) => {
        templateArgs.map(templateArg => {
          templexScout.translateTemplex(parentEnv, lidb.child(), ruleBuilder, contextRegion, templateArg)
        })
      }
    }
  }
  private def scoutExpression(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    expr: IExpressionPE):
  (StackFrame, IScoutResult[IExpressionSE], VariableUses, VariableUses) = {
    Profiler.frame(() => {
      val evalRange = (range: RangeL) => PostParser.evalRange(stackFrame0.file, range)
*/
  let file_coordinate = stack_frame.file;
  match expression {
  /*
  expr match {
  */
    IExpressionPE::Void(void) => Ok((
      stack_frame,
      IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::Void(VoidSE {
          range: PostParser::eval_range(file_coordinate, void.range),
        })),
      }),
      VariableUses::<'s>::empty(),
      VariableUses::<'s>::empty(),
    )),
    /*
    case VoidPE(range) => (stackFrame0, NormalResult(VoidSE(evalRange(range))), noVariableUses, noVariableUses)
    */
    IExpressionPE::Lambda(lambda) => {
      let (function_s, child_uses) = self.scout_lambda(stack_frame.clone(), &lambda.function)?;
      Ok((
        stack_frame.clone(),
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self
              .scout_arena
              .alloc(IExpressionSE::Function(FunctionSE { function: function_s })),
        }),
        VariableUses::<'s>::empty(),
        child_uses,
      ))
    }
        /*
        case lam @ LambdaPE(captures,_) => {
          val (function1, childUses) =
            delegate.scoutLambda(stackFrame0, lam.function)

          (stackFrame0, NormalResult(FunctionSE(function1)), noVariableUses, childUses)
        }
        */
        /*
        case StrInterpolatePE(range, partsPE) => {
          val (stackFrame1, partsSE, partsSelfUses, partsChildUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), partsPE)

          val rangeS = evalRange(range)
          val startingExpr: IExpressionSE = ConstantStrSE(RangeS(rangeS.begin, rangeS.begin), "")
          val addedExpr =
            partsSE.foldLeft(startingExpr)({
              case (prevExpr, partSE) => {
                val addCallRange = RangeS(prevExpr.range.end, partSE.range.begin)
                val overloadSetSE =
                  OverloadSetSE(
                    OutsideLoadSE(
                      addCallRange,
                      Vector(),
                      Vector(LoadPartSE(interner.intern(CodeNameS(keywords.plus)), Vector()))))
                FunctionCallSE(addCallRange, lidb.child().consume(), overloadSetSE, Vector(prevExpr, partSE))
              }
            })
          (stackFrame1, NormalResult(addedExpr), partsSelfUses, partsChildUses)
        }
        */
    IExpressionPE::Augment(augment) => {
      let load_as = match augment.target_ownership {
        OwnershipP::Borrow => LoadAsP::LoadAsBorrow,
        OwnershipP::Weak => LoadAsP::LoadAsWeak,
        OwnershipP::Own => panic!("POSTPARSER_AUGMENT_OWN_NOT_YET_IMPLEMENTED"),
        OwnershipP::Live => panic!("POSTPARSER_AUGMENT_LIVE_NOT_YET_IMPLEMENTED"),
        OwnershipP::Share => panic!("POSTPARSER_AUGMENT_SHARE_NOT_YET_IMPLEMENTED"),
      };
      let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
        let mut inner_lidb = lidb.child();
        let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses) = self.scout_expression_and_coerce(
          stack_frame,
          &mut inner_lidb,
          augment.inner,
          load_as,
        )?;
        (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses)
      };
      match &inner_expr_s {
        IExpressionSE::Ownershipped(ownershipped) => {
          assert_eq!(ownershipped.target_ownership, load_as);
        }
        IExpressionSE::LocalLoad(local_load) => {
          assert_eq!(local_load.target_ownership, load_as);
        }
        _ => panic!("POSTPARSER_SCOUT_AUGMENT_UNEXPECTED_RESULT"),
      }
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS { expr: inner_expr_s }),
        inner_self_uses,
        inner_child_uses,
      ))
    }
      /*
      case AugmentPE(range, targetOwnership, innerPE) => {
        val loadAs =
          targetOwnership match {
            case BorrowP => LoadAsBorrowP
            case WeakP => LoadAsWeakP
          }
        val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
          scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, loadAs)
        inner1 match {
          case OwnershippedSE(_, _, innerLoadAs) => vassert(loadAs == innerLoadAs)
          case LocalLoadSE(_, _, innerLoadAs) => vassert(loadAs == innerLoadAs)
//            case OutsideLoadSE(_, _, _, innerLoadAs) => vassert(loadAs == innerLoadAs) DO NOT SUBMIT
          case _ => vwat()
        }
        (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
      }
      */
    IExpressionPE::Return(ret) => {
      let mut ret_expr_lidb = lidb.child();
      let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses) = self.scout_expression_and_coerce(
        stack_frame,
        &mut ret_expr_lidb,
        ret.expr,
        LoadAsP::Use,
      )?;
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Return(ReturnSE {
            range: PostParser::eval_range(&file_coordinate, ret.range),
            inner: inner_expr_s,
          })),
        }),
        inner_self_uses,
        inner_child_uses,
      ))
    }
      /*
      case ReturnPE(range, innerPE) => {
        val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
          scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
        (stackFrame1, NormalResult(ReturnSE(evalRange(range), inner1)), innerSelfUses, innerChildUses)
      }
      */
        /*
        case BreakPE(range) => {
          (stackFrame0, NormalResult(BreakSE(evalRange(range))), noVariableUses, noVariableUses)
        }
        */
        /*
        case NotPE(range, innerPE) => {
          val overloadSetSE =
            OverloadSetSE(
              OutsideLoadSE(
                evalRange(range),
                Vector(),
                Vector(LoadPartSE(interner.intern(CodeNameS(keywords.not)), Vector()))))

          val (stackFrame1, innerSE, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)

          val result =
            NormalResult(
              FunctionCallSE(evalRange(range), lidb.child().consume(), overloadSetSE, Vector(innerSE)))

          (stackFrame1, result, innerSelfUses, innerChildUses)
        }
        */
        /*
        case RangePE(range, beginPE, endPE) => {
          val callableSE =
            OverloadSetSE(
              OutsideLoadSE(
                evalRange(range),
                Vector(),
                Vector(LoadPartSE(interner.intern(CodeNameS(keywords.range)), Vector()))))

          val loadBeginAs =
            beginPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => LoadAsBorrowP
            }
          val (stackFrame1, beginSE, beginSelfUses, beginChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), beginPE, loadBeginAs)

          val loadEndAs =
            endPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => LoadAsBorrowP
            }
          val (stackFrame2, endSE, endSelfUses, endChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), endPE, loadEndAs)

          val resultSE =
              FunctionCallSE(evalRange(range), lidb.child().consume(), callableSE, Vector(beginSE, endSE))

          (stackFrame2, NormalResult(resultSE), beginSelfUses.thenMerge(endSelfUses), beginChildUses.thenMerge(endChildUses))
        }
        */
    IExpressionPE::SubExpression(sub_expression) => {
      let mut sub_expression_lidb = lidb.child();
      let (stack_frame1, sub_expression_s, sub_self_uses, sub_child_uses) = self.scout_expression_and_coerce(
        stack_frame,
        &mut sub_expression_lidb,
        sub_expression.inner,
        LoadAsP::Use,
      )?;
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: sub_expression_s,
        }),
        sub_self_uses,
        sub_child_uses,
      ))
    }
    /*
    case SubExpressionPE(range, innerPE) => {
      val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
        scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
      (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
    }
    */
    IExpressionPE::ConstantInt(constant_int) => Ok((
      stack_frame.clone(),
      IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::ConstantInt(ConstantIntSE {
          range: PostParser::eval_range(&file_coordinate, constant_int.range),
          value: constant_int.value,
          bits: constant_int.bits.unwrap_or(32) as i32,
        })),
      }),
      VariableUses::<'s>::empty(),
      VariableUses::<'s>::empty(),
    )),
    /*
    case ConstantIntPE(range, value, bitsP) => {
      val bits = bitsP.getOrElse(32L).toInt
      (stackFrame0, NormalResult(ConstantIntSE(evalRange(range), value, bits)), noVariableUses, noVariableUses)
    }
    */
    IExpressionPE::ConstantBool(constant_bool) => Ok((
      stack_frame.clone(),
      IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::ConstantBool(ConstantBoolSE {
          range: PostParser::eval_range(&file_coordinate, constant_bool.range),
          value: constant_bool.value,
        })),
      }),
      VariableUses::<'s>::empty(),
      VariableUses::<'s>::empty(),
    )),
        /*
        case ConstantBoolPE(range,value) => (stackFrame0, NormalResult(ConstantBoolSE(evalRange(range), value)), noVariableUses, noVariableUses)
        */
    IExpressionPE::ConstantStr(constant_str) => Ok((
      stack_frame.clone(),
      IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::ConstantStr(ConstantStrSE {
          range: PostParser::eval_range(&file_coordinate, constant_str.range),
          value: self.scout_arena.intern_str(constant_str.value.as_str()),
        })),
      }),
      VariableUses::<'s>::empty(),
      VariableUses::<'s>::empty(),
    )),
        /*
        case ConstantStrPE(range, value) => {
          (stackFrame0, NormalResult(ConstantStrSE(evalRange(range), value)), noVariableUses, noVariableUses)
        }
        */
    IExpressionPE::ConstantFloat(constant_float) => Ok((
      stack_frame.clone(),
      IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::ConstantFloat(ConstantFloatSE {
          range: PostParser::eval_range(&file_coordinate, constant_float.range),
          value: constant_float.value,
        })),
      }),
      VariableUses::<'s>::empty(),
      VariableUses::<'s>::empty(),
    )),
        /*
        case ConstantFloatPE(range,value) => (stackFrame0, NormalResult(ConstantFloatSE(evalRange(range), value)), noVariableUses, noVariableUses)
        */
    IExpressionPE::MagicParamLookup(magic_param_lookup) => {
      let range_s = PostParser::eval_range(&file_coordinate, magic_param_lookup.range);
      let name = IVarNameS::MagicParamName(PostParser::eval_pos(
        &file_coordinate,
        magic_param_lookup.range.begin(),
      ));
      Ok((
        stack_frame.clone(),
        IScoutResult::LocalLookupResult(LocalLookupResultS {
          range: range_s,
          name: name.clone(),
        }),
        VariableUses::<'s>::empty().mark_moved(name),
        VariableUses::<'s>::empty(),
      ))
    }
    /*
    case MagicParamLookupPE(range) => {
      val name = interner.intern(MagicParamNameS(PostParser.evalPos(stackFrame0.file, range.begin)))
      val lookup = LocalLookupResult(evalRange(range), name)
      // We dont declare it here, because then scoutBlock will think its a local and
      // hide it from those above.
      //   val declarations = VariableDeclarations(Vector(VariableDeclaration(lookup.name, FinalP)))
      // Leave it to scoutLambda to declare it.
      (stackFrame0, lookup, noVariableUses.markMoved(name), noVariableUses)
    }
    */
    IExpressionPE::Lookup(lookup) => {
      match &lookup.template_args {
        None => {
          let range = PostParser::eval_range(&file_coordinate, lookup.name.range());
          let imprecise_name_s = translate_imprecise_name(self.scout_arena, &file_coordinate, &lookup.name);
          let lookup_result = match self.find_local(&stack_frame, range.clone(), &imprecise_name_s) {
            Some(local_result) => IScoutResult::LocalLookupResult(local_result),
            None => match &imprecise_name_s {
              IImpreciseNameS::CodeName(code_name) => {
                let name = code_name.name;
                let code_rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name }));
                if stack_frame.parent_env.all_declared_runes().contains(&code_rune) {
                  IScoutResult::NormalResult(NormalResultS {
                    expr: &*self.scout_arena.alloc(IExpressionSE::RuneLookup(RuneLookupSE {
                      range,
                      rune: code_rune,
                    })),
                  })
                } else {
                  IScoutResult::OutsideLookupResult(OutsideLookupResultS {
                    range,
                    name,
                    template_args: None,
                  })
                }
              }
              _ => panic!("POSTPARSER_SCOUT_LOOKUP_IMPRECISE_NAME_NOT_CODE_NAME"),
            }
          };
          Ok((
            stack_frame.clone(),
            lookup_result,
            VariableUses::<'s>::empty(),
            VariableUses::<'s>::empty(),
          ))
        }
        Some(template_args) => {
          let (range, template_name) = match &lookup.name {
            IImpreciseNameP::LookupName(name_p) => (PostParser::eval_range(&file_coordinate, name_p.range()), self.scout_arena.intern_str(name_p.str().as_str())),
            _ => panic!("POSTPARSER_SCOUT_LOOKUP_TEMPLATE_ARGS_EXPECTED_LOOKUP_NAME"),
          };
          Ok((
            stack_frame.clone(),
            IScoutResult::OutsideLookupResult(OutsideLookupResultS {
              range,
              name: template_name,
              template_args: Some(template_args.args),
            }),
            VariableUses::<'s>::empty(),
            VariableUses::<'s>::empty(),
          ))
        }
      }
    }
    /*
    case LookupPE(lookupName, None) => {
      val rangeS = evalRange(lookupName.range)
      val impreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, lookupName)
      val lookup =
        findLocal(stackFrame0, rangeS, impreciseNameS) match {
          case Some(result) => result
          case None => {
            impreciseNameS match {
              case CodeNameS(name) => {
                if (stackFrame0.parentEnv.allDeclaredRunes().contains(CodeRuneS(name))) {
                  NormalResult(RuneLookupSE(rangeS, CodeRuneS(name)))
                } else {
                  OutsideLookupResult(rangeS, name, None)
                }
              }
              case _ => vwat(impreciseNameS)
            }
          }
        }
      (stackFrame0, lookup, noVariableUses, noVariableUses)
    }
    case LookupPE(impreciseName, Some(TemplateArgsP(_, templateArgs))) => {
      val (range, templateName) =
        impreciseName match {
          case LookupNameP(NameP(range, templateName)) => (range, templateName)
          case _ => vwat()
        }
      val result =
        OutsideLookupResult(
          evalRange(range),
          templateName,
          Some(templateArgs.toVector))
      (stackFrame0, result, noVariableUses, noVariableUses)
    }
    */
        /*
        case DestructPE(range, innerPE) => {
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)
          (stackFrame1, NormalResult(DestructSE(evalRange(range), inner1)), innerSelfUses, innerChildUses)
        }
        */
        /*
        case UnletPE(range, localNameP) => {
          val rangeS = evalRange(range)
          val impreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, localNameP)
          val varNameS =
            findLocal(stackFrame0, rangeS, impreciseNameS) match {
              case Some(LocalLookupResult(_, name)) => name
              case None => {
                throw CompileErrorExceptionS(RangedInternalErrorS(rangeS, "Can't unlet local: " + localNameP))
              }
            }
          val result = NormalResult(UnletSE(rangeS, varNameS))
          (stackFrame0, result, noVariableUses.markMoved(varNameS), noVariableUses)
        }
        */
        /*
  //      case ResultPE(range, innerPE) => {
  //        scoutExpression(stackFrame0, lidb.child(), innerPE, true)
  //      }
        case PackPE(range, innersPE) => {
          vassert(innersPE.size == 1)
          val (stackFrame1, inner1, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innersPE.head, UseP)
          (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
        }
        */
    IExpressionPE::FunctionCall(function_call) => {
      let parent_env = IEnvironmentS::FunctionEnvironment(stack_frame.parent_env.clone());
      let context_region = stack_frame.context_region.clone();
      let mut callable_lidb = lidb.child();
      let (stack_frame0a, subject_uncoerced_scout_result, uncoerced_callable_self_uses, uncoerced_callable_child_uses) =
        self.scout_expression(stack_frame, &mut callable_lidb, function_call.callable_expr)?;
      match subject_uncoerced_scout_result {
        IScoutResult::OutsideLookupResult(OutsideLookupResultS { range, name: container_name, template_args: container_maybe_template_args }) => {
          let mut rule_builder = Vec::new();
          let container_template_arg_rune_usages = self.translate_maybe_template_args(
            &parent_env,
            lidb,
            &mut rule_builder,
            context_region,
            container_maybe_template_args,
          );
          let load_part_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
            name: container_name,
          }));
          let load_part = self.scout_arena.alloc(LoadPartSE {
            name: load_part_name,
            explicit_template_args: container_template_arg_rune_usages,
          });
          let overload_set_se = &*self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
            lookup: OutsideLoadSE {
              range,
              rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
              parts: self.scout_arena.alloc_slice_copy(&[&*load_part]),
            },
          }));
          let mut args_lidb = lidb.child();
          let (stack_frame3, args_se, self_uses, child_uses) =
            self.scout_elements_as_expressions(stack_frame0a, &mut args_lidb, &function_call.arg_exprs)?;
          let result = IScoutResult::NormalResult(NormalResultS {
            expr: &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
              range,
              location: lidb.child().consume_in_arena(self.scout_arena),
              callable_expr: overload_set_se,
              arg_exprs: self.scout_arena.alloc_slice_from_vec(args_se),
            })),
          });
          Ok((stack_frame3, result, self_uses, child_uses))
        }
        _ => {
          let callable_child_uses = uncoerced_callable_child_uses;
          let (stack_frame2, callable1, callable_self_uses) = self.coerce(
            stack_frame0a,
            subject_uncoerced_scout_result,
            &mut lidb.child(),
            uncoerced_callable_self_uses,
            LoadAsP::LoadAsBorrow,
          )?;
          let mut args_lidb = lidb.child();
          let (stack_frame3, args1, args_self_uses, args_child_uses) =
            self.scout_elements_as_expressions(stack_frame2, &mut args_lidb, &function_call.arg_exprs)?;
          let result = IScoutResult::NormalResult(NormalResultS {
            expr: &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
              range: PostParser::eval_range(&file_coordinate, function_call.range),
              location: lidb.child().consume_in_arena(self.scout_arena),
              callable_expr: callable1,
              arg_exprs: self.scout_arena.alloc_slice_from_vec(args1),
            })),
          });
          Ok((stack_frame3, result, callable_self_uses.then_merge(&args_self_uses), callable_child_uses.then_merge(&args_child_uses)))
        }
      }
    }
    /*
    case FunctionCallPE(range, _, callablePE, argsPE) => {
      val (stackFrame0a, subjectUncoercedScoutResult, uncoercedCallableSelfUses, uncoercedCallableChildUses) =
        scoutExpression(stackFrame0, lidb.child(), callablePE)
      subjectUncoercedScoutResult match {
        case OutsideLookupResult(range, containerName, containerMaybeTemplateArgs) => {
          val ruleBuilder = ArrayBuffer[IRulexSR]()
          val containerTemplateArgRuneUsages =
            translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, containerMaybeTemplateArgs)
          val overloadSetSE =
            OverloadSetSE(
              OutsideLoadSE(
                range,
                ruleBuilder.toVector,
                Vector(LoadPartSE(interner.intern(CodeNameS(containerName)), containerTemplateArgRuneUsages))))
          val (stackFrame3, argsSE, selfUses, childUses) =
            scoutElementsAsExpressions(stackFrame0a, lidb.child(), argsPE)
          val result = NormalResult(FunctionCallSE(range, lidb.child().consume(), overloadSetSE, argsSE))
          (stackFrame3, result, selfUses, childUses)
        }
        case _ => {
          val callableChildUses = uncoercedCallableChildUses
          val (stackFrame2, callable1, callableSelfUses) =
            coerce(stackFrame0a, subjectUncoercedScoutResult, lidb.child(), uncoercedCallableSelfUses, LoadAsBorrowP)
          val (stackFrame3, args1, argsSelfUses, argsChildUses) =
            scoutElementsAsExpressions(stackFrame2, lidb.child(), argsPE)
          val result = NormalResult(FunctionCallSE(evalRange(range), lidb.child().consume(), callable1, args1.toVector))
          (stackFrame3, result, callableSelfUses.thenMerge(argsSelfUses), callableChildUses.thenMerge(argsChildUses))
        }
      }
    }
    */
    IExpressionPE::BinaryCall(binary_call) => {
      let part_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(binary_call.function_name.str().as_str()),
      }));
      let load_part = self.scout_arena.alloc(LoadPartSE {
        name: part_name,
        explicit_template_args: &[],
      });
      let overload_set_se = &*self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
        lookup: OutsideLoadSE {
          range: PostParser::eval_range(&file_coordinate, binary_call.range),
          rules: &[],
          parts: self.scout_arena.alloc_slice_copy(&[&*load_part]),
        },
      }));
      let (stack_frame1, left_expr_s, left_self_uses, left_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
        let mut left_lidb = lidb.child();
        self.scout_expression_and_coerce(
          stack_frame,
          &mut left_lidb,
          binary_call.left_expr,
          LoadAsP::LoadAsBorrow,
        )?
      };
      let (stack_frame2, right_expr_s, right_self_uses, right_child_uses) = {
        let mut right_lidb = lidb.child();
        self.scout_expression_and_coerce(
          stack_frame1,
          &mut right_lidb,
          binary_call.right_expr,
          LoadAsP::LoadAsBorrow,
        )?
      };
      let result = IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
          range: PostParser::eval_range(&file_coordinate, binary_call.range),
          location: lidb.child().consume_in_arena(self.scout_arena),
          callable_expr: overload_set_se,
          arg_exprs: self.scout_arena.alloc_slice_from_vec(
            vec![left_expr_s, right_expr_s],
          ),
        })),
      });
      Ok((
        stack_frame2,
        result,
        left_self_uses.then_merge(&right_self_uses),
        left_child_uses.then_merge(&right_child_uses),
      ))
    }
    /*
    case BinaryCallPE(range, namePE, leftPE, rightPE) => {
      val overloadSetSE =
        OverloadSetSE(
          OutsideLoadSE(
          evalRange(range),
          Vector(),
          Vector(LoadPartSE(interner.intern(CodeNameS(namePE.str)), Vector()))))

      val (stackFrame1, leftSE, leftSelfUses, leftChildUses) =
        scoutExpressionAndCoerce(stackFrame0, lidb.child(), leftPE, LoadAsBorrowP)
      val (stackFrame2, rightSE, rightSelfUses, rightChildUses) =
        scoutExpressionAndCoerce(stackFrame1, lidb.child(), rightPE, LoadAsBorrowP)

      val result =
        NormalResult(
          FunctionCallSE(evalRange(range), lidb.child().consume(), overloadSetSE, Vector(leftSE, rightSE)))
      (stackFrame2, result, leftSelfUses.thenMerge(rightSelfUses), leftChildUses.thenMerge(rightChildUses))
    }
    */
        /*
        case BraceCallPE(range, operatorRange, subjectPE, args, callableReadwrite) => {
          val loadSubjectAs =
            subjectPE match {
              // For subexpressions, just use what they give.
              case SubExpressionPE(_, _) => UseP
              // For anything else, default to borrowing.
              case _ => {
                LoadAsBorrowP
              }
            }

          vassert(args.size == 1)
          val argPE = args.head

          val (stackFrame1, callableSE, callableSelfUses, callableChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), subjectPE, loadSubjectAs)
          val (stackFrame2, argSE, argSelfUses, argChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), argPE, UseP)

          val resultSE = IndexSE(evalRange(range), callableSE, argSE)
          (stackFrame2, NormalResult(resultSE), callableSelfUses.thenMerge(argSelfUses), callableChildUses.thenMerge(argChildUses))
        }
        */
    IExpressionPE::MethodCall(method_call) => {
      let method_call_range_s = PostParser::eval_range(&file_coordinate, method_call.range);
      let parent_env = IEnvironmentS::FunctionEnvironment(stack_frame.parent_env.clone());
      let context_region = stack_frame.context_region.clone();
      let method_name_p = method_call.method_lookup.name;
      let maybe_method_template_args_args: Option<&'p [&'p ITemplexPT<'p>]> =
        method_call.method_lookup.template_args.as_ref().map(|t| t.args);
      let mut subject_lidb = lidb.child();
      let (stack_frame0a, subject_uncoerced_scout_result, uncoerced_subject_self_uses, uncoerced_subject_child_uses) =
        self.scout_expression(stack_frame, &mut subject_lidb, method_call.subject_expr)?;
      let method_imprecise_name_s = translate_imprecise_name(self.scout_arena, &file_coordinate, &method_name_p);
      let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
      let method_template_arg_rune_usages = self.translate_maybe_template_args(
        &parent_env,
        lidb,
        &mut rule_builder,
        context_region.clone(),
        maybe_method_template_args_args,
      );
      let (stack_frame3, args, self_uses, child_uses, container_parts): (StackFrame<'s>, Vec<&'s IExpressionSE<'s>>, VariableUses<'s>, VariableUses<'s>, Vec<&'s LoadPartSE<'s>>) =
        match subject_uncoerced_scout_result {
          IScoutResult::OutsideLookupResult(OutsideLookupResultS { range: _subject_range_s, name: container_name, template_args: container_maybe_template_args }) => {
            assert!(uncoerced_subject_self_uses.is_empty());
            assert!(uncoerced_subject_child_uses.is_empty());
            let container_template_arg_rune_usages = self.translate_maybe_template_args(
              &parent_env,
              lidb,
              &mut rule_builder,
              context_region.clone(),
              container_maybe_template_args,
            );
            let mut args_lidb = lidb.child();
            let (stack_frame3, args, self_uses, child_uses) =
              self.scout_elements_as_expressions(stack_frame0a, &mut args_lidb, method_call.arg_exprs)?;
            // We don't support more than 2 parts yet
            let container_load_part_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
              name: container_name,
            }));
            let container_part = self.scout_arena.alloc(LoadPartSE {
              name: container_load_part_name,
              explicit_template_args: container_template_arg_rune_usages,
            });
            (stack_frame3, args, self_uses, child_uses, vec![&*container_part])
          }
          _ => {
            let load_subject_as = match method_call.subject_expr {
              // For locals, just borrow.
              IExpressionPE::Lookup(_) => LoadAsP::LoadAsBorrow,
              // For anything else, default to moving.
              _ => LoadAsP::Use,
            };
            let subject_child_uses = uncoerced_subject_child_uses;
            let mut coerce_lidb = lidb.child();
            let (stack_frame1, subject1, subject_self_uses) = self.coerce(
              stack_frame0a,
              subject_uncoerced_scout_result,
              &mut coerce_lidb,
              uncoerced_subject_self_uses,
              load_subject_as,
            )?;
            let mut args_lidb = lidb.child();
            let (stack_frame3, tail_args1, tail_args_self_uses, tail_args_child_uses) =
              self.scout_elements_as_expressions(stack_frame1, &mut args_lidb, method_call.arg_exprs)?;
            let mut args = vec![subject1];
            args.extend(tail_args1);
            (stack_frame3, args, subject_self_uses.then_merge(&tail_args_self_uses), subject_child_uses.then_merge(&tail_args_child_uses), Vec::new())
          }
        };
      let method_load_part = self.scout_arena.alloc(LoadPartSE {
        name: method_imprecise_name_s,
        explicit_template_args: method_template_arg_rune_usages,
      });
      let mut parts: Vec<&'s LoadPartSE<'s>> = container_parts;
      parts.push(&*method_load_part);
      let parts_slice = self.scout_arena.alloc_slice_copy(&parts);
      let overload_set_se = &*self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
        lookup: OutsideLoadSE {
          range: method_call_range_s,
          rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
          parts: parts_slice,
        },
      }));
      let result = IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
          range: method_call_range_s,
          location: lidb.child().consume_in_arena(self.scout_arena),
          callable_expr: overload_set_se,
          arg_exprs: self.scout_arena.alloc_slice_from_vec(args),
        })),
      });
      Ok((stack_frame3, result, self_uses, child_uses))
    }
        /*
        case MethodCallPE(methodCallRangeP, subjectExpr, _, LookupPE(methodName, maybeMethodTemplateArgs), methodArgs) => {
          val methodCallRangeS = evalRange(methodCallRangeP)
          val (stackFrame0a, subjectUncoercedScoutResult, uncoercedSubjectSelfUses, uncoercedSubjectChildUses) =
            scoutExpression(stackFrame0, lidb.child(), subjectExpr)
          val methodImpreciseNameS = PostParser.translateImpreciseName(interner, stackFrame0.file, methodName)
          val ruleBuilder = ArrayBuffer[IRulexSR]()
          val methodTemplateArgRuneUsages =
            translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, maybeMethodTemplateArgs.map(_.args))
          val (stackFrame3, args, selfUses, childUses, containerParts) =
            subjectUncoercedScoutResult match {
              case OutsideLookupResult(subjectRangeS, containerName, containerMaybeTemplateArgs) => {
                vassert(uncoercedSubjectSelfUses.isEmpty)
                vassert(uncoercedSubjectChildUses.isEmpty)
                val containerTemplateArgRuneUsages =
                  translateMaybeTemplateArgs(stackFrame0.parentEnv, lidb, ruleBuilder, stackFrame0.contextRegion, containerMaybeTemplateArgs)
                val (stackFrame3, args, selfUses, childUses) =
                  scoutElementsAsExpressions(stackFrame0a, lidb.child(), methodArgs)
                // We don't support more than 2 parts yet
                val containerParts = Vector(
                  LoadPartSE(interner.intern(CodeNameS(containerName)), containerTemplateArgRuneUsages))
                (stackFrame3, args, selfUses, childUses, containerParts)
              }
              case _ => {
                val loadSubjectAs =
                  subjectExpr match {
                    // For locals, just borrow.
                    case LookupPE(_, _) => LoadAsBorrowP
                    // For anything else, default to moving.
                    case _ => UseP
                  }
                val subjectChildUses = uncoercedSubjectChildUses
                val (stackFrame1, subject1, subjectSelfUses) =
                  coerce(stackFrame0a, subjectUncoercedScoutResult, lidb.child(), uncoercedSubjectSelfUses, loadSubjectAs)
                val (stackFrame3, tailArgs1, tailArgsSelfUses, tailArgsChildUses) =
                  scoutElementsAsExpressions(stackFrame1, lidb.child(), methodArgs)
                val containerParts = Vector()
                (stackFrame3, Vector(subject1) ++ tailArgs1, subjectSelfUses.thenMerge(tailArgsSelfUses), subjectChildUses.thenMerge(tailArgsChildUses), containerParts)
              }
            }
          val parts = containerParts :+ LoadPartSE(methodImpreciseNameS, methodTemplateArgRuneUsages)
          val overloadSetSE =
            OverloadSetSE(OutsideLoadSE(methodCallRangeS, ruleBuilder.toVector, parts))
          val result = NormalResult(FunctionCallSE(methodCallRangeS, lidb.child().consume(), overloadSetSE, args))
          (stackFrame3, result, selfUses, childUses)
        }
        */
        /*
        case TuplePE(range, elementsPE) => {
          val (stackFrame1, elements1, selfUses, childUses) =
            scoutElementsAsExpressions(stackFrame0, lidb.child(), elementsPE)
          (stackFrame1, NormalResult(TupleSE(evalRange(range), elements1.toVector)), selfUses, childUses)
        }
        */
    IExpressionPE::ConstructArray(construct_array) => {
      let range_s = PostParser::eval_range(&file_coordinate, construct_array.range);
      let mut rule_builder = Vec::new();
      let parent_env = IEnvironmentS::FunctionEnvironment(stack_frame.parent_env.clone());
      let context_region = stack_frame.context_region.clone();
      let maybe_type_rune_s = construct_array.type_pt.as_ref().map(|type_pt| {
        translate_templex(
          self.scout_arena,
          self.keywords,
          parent_env.clone(),
          &mut lidb.child(),
          &mut rule_builder,
          context_region.clone(),
          type_pt,
        )
      });
      let mutability_rune_s = match &construct_array.mutability_pt {
        None => {
          let rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())));
          let rune_usage = RuneUsage { range: range_s, rune };
          rule_builder.push(IRulexSR::Literal(LiteralSR {
            range: range_s,
            rune: rune_usage,
            literal: ILiteralSL::MutabilityLiteral(MutabilityLiteralSL { mutability: MutabilityP::Mutable }),
          }));
          rune_usage
        }
        Some(mutability_pt) => {
          translate_templex(
            self.scout_arena,
            self.keywords,
            parent_env.clone(),
            &mut lidb.child(),
            &mut rule_builder,
            context_region.clone(),
            mutability_pt,
          )
        }
      };
      let variability_rune_s = match &construct_array.variability_pt {
        None => {
          let rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())));
          let rune_usage = RuneUsage { range: range_s, rune };
          rule_builder.push(IRulexSR::Literal(LiteralSR {
            range: range_s,
            rune: rune_usage,
            literal: ILiteralSL::VariabilityLiteral(VariabilityLiteralSL { variability: VariabilityP::Final }),
          }));
          rune_usage
        }
        Some(variability_pt) => {
          translate_templex(
            self.scout_arena,
            self.keywords,
            parent_env.clone(),
            &mut lidb.child(),
            &mut rule_builder,
            context_region.clone(),
            variability_pt,
          )
        }
      };
      let mut args_lidb = lidb.child();
      let (stack_frame1, args_s, self_uses, child_uses) =
          self.scout_elements_as_expressions(stack_frame, &mut args_lidb, &construct_array.args)?;
      let result = match &construct_array.size {
        IArraySizeP::RuntimeSized => {
          assert!(
            !construct_array.initializing_individual_elements,
            "POSTPARSER_SCOUT_CONSTRUCT_ARRAY_RUNTIME_INIT_INDIVIDUAL_ELEMENTS_NOT_YET_IMPLEMENTED"
          );
          if args_s.is_empty() || args_s.len() > 2 {
            return Err(
              ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(
                InitializingRuntimeSizedArrayRequiresSizeAndCallable { range: range_s },
              ),
            );
          }
          let size_se = args_s[0];
          let callable_se = args_s.get(1).copied();
          IExpressionSE::NewRuntimeSizedArray(NewRuntimeSizedArraySE {
            range: range_s,
            rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
            maybe_element_type_st: maybe_type_rune_s,
            mutability_st: mutability_rune_s,
            size: self.scout_arena.alloc(size_se),
            callable: callable_se,
          })
        }
        IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: maybe_size_pt }) => {
          let maybe_size_rune_s = maybe_size_pt.as_ref().map(|size_pt| {
            translate_templex(
              self.scout_arena,
              self.keywords,
              parent_env.clone(),
              &mut lidb.child(),
              &mut rule_builder,
              context_region.clone(),
              size_pt,
            )
          });
          if construct_array.initializing_individual_elements {
            let size_rune_s = match maybe_size_rune_s {
              Some(s) => s,
              None => {
                let rune = self.scout_arena.intern_rune(IRuneValS::ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())));
                let rune_usage = RuneUsage { range: range_s, rune };
                rule_builder.push(IRulexSR::Literal(LiteralSR {
                  range: range_s,
                  rune: rune_usage,
                  literal: ILiteralSL::IntLiteral(IntLiteralSL { value: args_s.len() as i64 }),
                }));
                rune_usage
              }
            };
            IExpressionSE::StaticArrayFromValues(StaticArrayFromValuesSE {
              range: range_s,
              rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
              maybe_element_type_st: maybe_type_rune_s,
              mutability_st: mutability_rune_s,
              variability_st: variability_rune_s,
              size_st: size_rune_s,
              elements: self.scout_arena.alloc_slice_from_vec(args_s),
            })
          } else {
            if args_s.len() != 1 {
              return Err(
                ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(
                  InitializingStaticSizedArrayRequiresSizeAndCallable { range: range_s },
                ),
              );
            }
            let size_rune_s = match maybe_size_rune_s {
              Some(s) => s,
              None => panic!("vassertSome: no size rune for static array from callable"),
            };
            let callable_se = args_s[0];
            IExpressionSE::StaticArrayFromCallable(StaticArrayFromCallableSE {
              range: range_s,
              rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
              maybe_element_type_st: maybe_type_rune_s,
              mutability_st: mutability_rune_s,
              variability_st: variability_rune_s,
              size_st: size_rune_s,
              callable: callable_se,
            })
          }
        }
      };
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS { expr: &*self.scout_arena.alloc(result) }),
        self_uses,
        child_uses,
      ))
    }
    /*
    case ConstructArrayPE(rangeP, maybeTypePT, maybeMutabilityPT, maybeVariabilityPT, size, initializingIndividualElements, argsPE) => {
      val rangeS = evalRange(rangeP)
      val ruleBuilder = mutable.ArrayBuffer[IRulexSR]()
      val maybeTypeRuneS =
        maybeTypePT.map(typePT => {
          templexScout.translateTemplex(
            stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, typePT)
        })
      val mutabilityRuneS =
        maybeMutabilityPT match {
          case None => {
            val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
            ruleBuilder += LiteralSR(rangeS, rune, MutabilityLiteralSL(MutableP))
            rune
          }
          case Some(mutabilityPT) => {
            templexScout.translateTemplex(
              stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, mutabilityPT)
          }
        }
      val variabilityRuneS =
        maybeVariabilityPT match {
          case None => {
            val rune = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
            ruleBuilder += rules.LiteralSR(rangeS, rune, VariabilityLiteralSL(FinalP))
            rune
          }
          case Some(variabilityPT) => {
            templexScout.translateTemplex(
              stackFrame0.parentEnv, lidb.child(), ruleBuilder, stackFrame0.contextRegion, variabilityPT)
          }
        }

      val (stackFrame1, argsSE, selfUses, childUses) =
        scoutElementsAsExpressions(stackFrame0, lidb.child(), argsPE)

      val result =
        size match {
          case RuntimeSizedP => {
            vassert(!initializingIndividualElements)
            if (argsSE.isEmpty || argsSE.size > 2) {
              throw CompileErrorExceptionS(InitializingRuntimeSizedArrayRequiresSizeAndCallable(rangeS))
            }
            val sizeSE = argsSE.head
            val callableSE = argsSE.lift(1)

            NewRuntimeSizedArraySE(rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, sizeSE, callableSE)
          }
          case StaticSizedP(maybeSizePT) => {
            val maybeSizeRuneS =
              maybeSizePT match {
                case None => None
                case Some(sizePT) => {
                  Some(
                    templexScout.translateTemplex(
                      stackFrame0.parentEnv,
                      lidb.child(),
                      ruleBuilder,
                      stackFrame0.contextRegion,
                      sizePT))
                }
              }

            if (initializingIndividualElements) {
              val sizeRuneS =
                maybeSizeRuneS match {
                  case Some(s) => s
                  case None => {
                    val runeS = rules.RuneUsage(rangeS, ImplicitRuneS(lidb.child().consume()))
                    ruleBuilder += rules.LiteralSR(rangeS, runeS, IntLiteralSL(argsSE.size))
                    runeS
                  }
                }

              StaticArrayFromValuesSE(
                rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, variabilityRuneS, sizeRuneS, argsSE.toVector)
            } else {
              if (argsSE.size != 1) {
                throw CompileErrorExceptionS(InitializingStaticSizedArrayRequiresSizeAndCallable(rangeS))
              }
              val sizeRuneS = vassertSome(maybeSizeRuneS)
              val Vector(callableSE) = argsSE
              StaticArrayFromCallableSE(
                rangeS, ruleBuilder.toVector, maybeTypeRuneS, mutabilityRuneS, variabilityRuneS, sizeRuneS, callableSE)
            }
          }
        }

      (stackFrame1, NormalResult(result), selfUses, childUses)
    }
        */
    IExpressionPE::Block(block) => {
      assert!(
        block.maybe_default_region.is_none(),
        "POSTPARSER_SCOUT_BLOCK_DEFAULT_REGION_NOT_YET_IMPLEMENTED"
      );
      let mut block_lidb = lidb.child();
      let (result_se, self_uses, child_uses) =
          self.scout_block(stack_frame.clone(), &mut block_lidb, PostParser::<'s, 'p, '_>::no_declarations(), block)?;
      Ok((
        stack_frame,
        IScoutResult::NormalResult(NormalResultS { expr: result_se }),
        self_uses,
        child_uses,
      ))
    }
    /*
    case b @ BlockPE(_, _, maybeNewDefaultRegion, _) => {
      vassert(maybeNewDefaultRegion.isEmpty)
      val (resultSE, selfUses, childUses) =
        scoutBlock(stackFrame0, lidb.child(), noDeclarations, b)
      (stackFrame0, NormalResult(resultSE), selfUses, childUses)
    }
    */
    IExpressionPE::Consecutor(consecutor) => {
      let mut consecutor_lidb = lidb.child();
      let (stack_frame1, unfiltered_exprs, self_uses, child_uses) =
          self.scout_elements_as_expressions(stack_frame, &mut consecutor_lidb, &consecutor.inners)?;

      // Match Scala's two-step behavior:
      // 1) recursively scout all inners
      // 2) strip voids that appear after a return; error on non-void after return
      let mut filtered_exprs = Vec::new();
      let mut saw_return = false;
      for expr_s in unfiltered_exprs {
        match (saw_return, &expr_s) {
          (false, IExpressionSE::Return(_)) => {
            saw_return = true;
            filtered_exprs.push(expr_s);
          }
          (false, _) => {
            filtered_exprs.push(expr_s);
          }
          (true, IExpressionSE::Void(_)) => {}
          (true, _) => {
            return Err(ICompileErrorS::StatementAfterReturnS(StatementAfterReturnS {
              range: expr_s.range(),
            }));
          }
        }
      }
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: self.consecutive(filtered_exprs),
        }),
        self_uses,
        child_uses,
      ))
    }
    /*
    case ConsecutorPE(inners) => {
      val (stackFrame1, unfilteredResultsSE, selfUses, childUses) =
        scoutElementsAsExpressions(stackFrame0, lidb.child(), inners)

      // Strip trailing voids after return
      // The boolean is whether we've seen a return yet
      val (_, filteredResultsSE) =
        unfilteredResultsSE.foldLeft((false, Vector[IExpressionSE]()))({
          case ((false, previous), r @ ReturnSE(_, _)) => (true, previous :+ r)
          case ((false, previous), next) => (false, previous :+ next)
          case ((true, previous), VoidSE(_)) => (true, previous)
          case ((true, previous), next) => {
            throw CompileErrorExceptionS(StatementAfterReturnS(next.range))
          }
        })

      (stackFrame1, NormalResult(PostParser.consecutive(filteredResultsSE)), selfUses, childUses)
    }
    */
        /*
        case AndPE(range, leftPE, rightPE) => {
          val rightRange = evalRange(rightPE.range)
          val endRange = RangeS(rightRange.end, rightRange.end)

          val (stackFrameZ, ifSE, selfUses, childUses) =
            newIf(
              stackFrame0, lidb, range,
              (stackFrame1, lidb) => {
                scoutExpressionAndCoerce(stackFrame1, lidb, leftPE, UseP)
              },
              (stackFrame2, lidb) => {
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, rightPE)
                (stackFrame2, thenSE, thenUses, thenChildUses)
              },
              (stackFrame3, lidb) => {
                val elseSE =
                  BlockSE(endRange, Vector.empty, ConstantBoolSE(endRange, false))
                (stackFrame3, elseSE, noVariableUses, noVariableUses)
              })

          (stackFrameZ, NormalResult(ifSE), selfUses, childUses)
        }
        */
        /*
        case OrPE(range, leftPE, rightPE) => {
          val rightRange = evalRange(rightPE.range)
          val endRange = RangeS(rightRange.end, rightRange.end)

          val (stackFrameZ, ifSE, selfUses, childUses) =
            newIf(
              stackFrame0, lidb, range,
              (stackFrame1, lidb) => {
                scoutExpressionAndCoerce(stackFrame1, lidb, leftPE, UseP)
              },
              (stackFrame2, lidb) => {
                val elseSE =
                  BlockSE(endRange, Vector.empty, ConstantBoolSE(endRange, true))
                (stackFrame2, elseSE, noVariableUses, noVariableUses)
              },
              (stackFrame3, lidb) => {
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame3, lidb.child(), noDeclarations, rightPE)
                (stackFrame3, thenSE, thenUses, thenChildUses)
              })

          (stackFrameZ, NormalResult(ifSE), selfUses, childUses)
        }
        */
    IExpressionPE::If(if_expr) => {
      let range_s = PostParser::eval_range(file_coordinate, if_expr.range);
      let mut block_lidb = lidb.child();
      let (result_se, self_uses, child_uses): (&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = self.new_block(
        stack_frame.parent_env.clone(),
        Some(stack_frame.clone()),
        &mut block_lidb,
        range_s,
        stack_frame.context_region.clone(),
        PostParser::<'s, 'p, '_>::no_declarations(),
        |stack_frame1: StackFrame<'s>, block_lidb: &mut LocationInDenizenBuilder| {
          let (stack_frame2, cond_se, cond_uses, cond_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
            let mut cond_lidb = block_lidb.child();
            self.scout_expression_and_coerce(
              stack_frame1,
              &mut cond_lidb,
              if_expr.condition,
              LoadAsP::Use,
            )?
          };
          let (then_se, then_uses, then_child_uses): (&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
            let mut then_lidb = block_lidb.child();
            self.scout_impure_block(
              stack_frame2.clone(),
              &mut then_lidb,
              PostParser::<'s, 'p, '_>::no_declarations(),
              if_expr.then_body,
            )?
          };
          let (else_se, else_uses, else_child_uses): (&'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
            let mut else_lidb = block_lidb.child();
            self.scout_impure_block(
              stack_frame2.clone(),
              &mut else_lidb,
              PostParser::<'s, 'p, '_>::no_declarations(),
              if_expr.else_body,
            )?
          };
          let self_case_uses = then_uses.branch_merge(&else_uses);
          let self_uses = cond_uses.then_merge(&self_case_uses);
          let child_case_uses = then_child_uses.branch_merge(&else_child_uses);
          let child_uses = cond_child_uses.then_merge(&child_case_uses);
          let if_se = &*self.scout_arena.alloc(IExpressionSE::If(IfSE {
            range: PostParser::eval_range(file_coordinate, if_expr.range),
            condition: cond_se,
            then_body: then_se,
            else_body: else_se,
          }));
          Ok((
            stack_frame2,
            if_se,
            self_uses,
            child_uses,
          ))
        },
      )?;
      Ok((
        stack_frame,
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Block(result_se)),
        }),
        self_uses,
        child_uses,
      ))
    }
    /*
        case IfPE(range, condition, thenBody, elseBody) => {
          val (resultSE, selfUses, childUses) =
            newBlock(
              stackFrame0.parentEnv,
              Some(stackFrame0),
              lidb.child(),
              evalRange(range),
              stackFrame0.contextRegion,
              noDeclarations,
              (stackFrame1, lidb) => {
                val (stackFrame2, condSE, condUses, condChildUses) =
                  scoutExpressionAndCoerce(stackFrame1, lidb.child(), condition, UseP)
                val (thenSE, thenUses, thenChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, thenBody)
                val (elseSE, elseUses, elseChildUses) =
                  scoutImpureBlock(stackFrame2, lidb.child(), noDeclarations, elseBody)

                val selfCaseUses = thenUses.branchMerge(elseUses)
                val selfUses = condUses.thenMerge(selfCaseUses);
                val childCaseUses = thenChildUses.branchMerge(elseChildUses)
                val childUses = condChildUses.thenMerge(childCaseUses);

                val ifSE = IfSE(evalRange(range), condSE, thenSE, elseSE)
                (stackFrame2, ifSE, selfUses, childUses)
              })
          (stackFrame0, NormalResult(resultSE), selfUses, childUses)
        }
        */
    IExpressionPE::While(while_expr) => {
      let (loop_s, loop_self_uses, loop_child_uses) = scout_while(
        self,
        stack_frame.clone(),
        lidb,
        while_expr.range,
        while_expr.condition,
        while_expr.body,
      )?;
      Ok((
        stack_frame,
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Block(loop_s)),
        }),
        loop_self_uses,
        loop_child_uses,
      ))
    }
        /*
        case WhilePE(range, conditionPE, uncombinedBodyPE) => {
          val (loopSE, loopSelfUses, loopChildUses) =
            loopPostParser.scoutWhile(
              this, stackFrame0, lidb, range, conditionPE, uncombinedBodyPE)

          (stackFrame0, NormalResult(loopSE), loopSelfUses, loopChildUses)
        }
        */
    IExpressionPE::Each(each) => {
      let (loop_s, self_uses, child_uses) = scout_each(
        self,
        stack_frame.clone(),
        lidb,
        each.range,
        each.maybe_pure.is_some(),
        &each.entry_pattern,
        each.in_keyword_range,
        each.iterable_expr,
        each.body,
      )?;
      Ok((
        stack_frame.clone(),
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Block(loop_s)),
        }),
        self_uses,
        child_uses,
      ))
    }
        /*
        case EachPE(range, maybePure, entryPatternPP, inKeywordRange, iterableExpr, body) => {
          val (loopSE, selfUses, childUses) =
            loopPostParser.scoutEach(this, stackFrame0, lidb, range, maybePure.nonEmpty, entryPatternPP, inKeywordRange, iterableExpr, body)
          (stackFrame0, NormalResult(loopSE), selfUses, childUses)
        }
        */

    IExpressionPE::Let(lett) => {
      let parent_env = IEnvironmentS::FunctionEnvironment(stack_frame.parent_env.clone());
      let (stack_frame1, source_expr_s, source_self_uses, source_child_uses) = {
        let mut source_expr_lidb = lidb.child();
        self.scout_expression_and_coerce(
          stack_frame,
          &mut source_expr_lidb,
          lett.source,
          LoadAsP::Use,
        )?
      };
      let mut rule_builder = Vec::new();
      let mut rune_to_explicit_type = Vec::new();
      {
        let mut rule_lidb = lidb.child();
        translate_rulexes(
          self.scout_arena,
          self.keywords,
          parent_env,
          &mut rule_lidb,
          &mut rule_builder,
          &mut rune_to_explicit_type,
          stack_frame1.context_region.clone(),
          &[],
        );
      }
      let pattern_s = {
        let mut pattern_lidb = lidb.child();
        let mut rune_to_explicit_type_map = rune_to_explicit_type
          .into_iter()
          .collect::<std::collections::HashMap<_, _>>();
        translate_pattern(
          self.scout_arena,
          self.keywords,
          stack_frame1.clone(),
          &mut pattern_lidb,
          &mut rule_builder,
          &mut rune_to_explicit_type_map,
          &lett.pattern,
        )
      };
      let declarations_from_pattern = VariableDeclarations {
        vars: get_parameter_captures(&pattern_s),
      };
      let maybe_name_conflict_var_name =
        stack_frame1
          .locals
          .vars
          .iter()
          .map(|decl| decl.name.clone())
          .find(|name| declarations_from_pattern.vars.iter().any(|decl| decl.name == *name));
      if let Some(name_conflict_var_name) = maybe_name_conflict_var_name {
        return Err(ICompileErrorS::VariableNameAlreadyExists(VariableNameAlreadyExists {
          range: PostParser::eval_range(&file_coordinate, lett.range),
          name: name_conflict_var_name,
        }));
      }
      Ok((
        stack_frame1.plus(&declarations_from_pattern),
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Let(LetSE {
            range: PostParser::eval_range(&file_coordinate, lett.range),
            rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
            pattern: pattern_s,
            expr: source_expr_s,
          })),
        }),
        source_self_uses,
        source_child_uses,
      ))
    }
    /*
    case LetPE(range, patternP, exprPE) => {
      val codeLocation = PostParser.evalPos(stackFrame0.file, range.begin)
      val (stackFrame1, expr1, selfUses, childUses) =
        scoutExpressionAndCoerce(stackFrame0, lidb.child(), exprPE, UseP);

      val ruleBuilder = ArrayBuffer[IRulexSR]()
      val runeToExplicitType = mutable.ArrayBuffer[(IRuneS, ITemplataType)]()

      ruleScout.translateRulexes(
        stackFrame0.parentEnv, lidb.child(), ruleBuilder, runeToExplicitType, stackFrame1.contextRegion, Vector())

      val patternS =
        patternScout.translatePattern(
          stackFrame1, lidb.child(), ruleBuilder, runeToExplicitType, patternP)

      val declarationsFromPattern = VariableDeclarations(patternScout.getParameterCaptures(patternS))

      val nameConflictVarNames =
        stackFrame1.locals.vars.map(_.name).intersect(declarationsFromPattern.vars.map(_.name))
      nameConflictVarNames.headOption match {
        case None =>
        case Some(nameConflictVarName) => {
          throw CompileErrorExceptionS(VariableNameAlreadyExists(evalRange(range), nameConflictVarName))
        }
      }

      val letSE = LetSE(evalRange(range), ruleBuilder.toVector, patternS, expr1)
      (stackFrame1 ++ declarationsFromPattern, NormalResult(letSE), selfUses, childUses)
    }
    */
    IExpressionPE::Mutate(mutate) => {
      let (stack_frame1, source_expr_s, source_inner_self_uses, source_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
        let mut source_expr_lidb = lidb.child();
        // AFTERM: consider doing &mut StackFrame instead of clone, everywhere.
        self.scout_expression_and_coerce(
          stack_frame,
          &mut source_expr_lidb,
          mutate.source,
          LoadAsP::Use,
        )?
      };
      let (stack_frame2, destination_result_s, destination_self_uses, destination_child_uses): (StackFrame<'s>, IScoutResult<'s, 'p>, VariableUses<'s>, VariableUses<'s>) = {
        let mut destination_expr_lidb = lidb.child();
        self.scout_expression(
          stack_frame1,
          &mut destination_expr_lidb,
          mutate.mutatee,
        )?
      };
      let (mutate_expr_s, source_self_uses): (&'s IExpressionSE<'s>, VariableUses<'s>) = match destination_result_s {
        IScoutResult::LocalLookupResult(LocalLookupResultS { range, name }) => (
          &*self.scout_arena.alloc(IExpressionSE::LocalMutate(LocalMutateSE {
            range,
            name: name.clone(),
            expr: source_expr_s,
          })),
          source_inner_self_uses.mark_mutated(name),
        ),
        IScoutResult::OutsideLookupResult(OutsideLookupResultS { range, name, .. }) => {
          return Err(ICompileErrorS::CouldntFindVarToMutateS(CouldntFindVarToMutateS {
            range,
            name: name.as_str().to_string(),
          }));
        }
        IScoutResult::NormalResult(NormalResultS { expr: destination_expr_s }) => (
          &*self.scout_arena.alloc(IExpressionSE::ExprMutate(ExprMutateSE {
            range: destination_expr_s.range(),
            mutatee: destination_expr_s,
            expr: source_expr_s,
          })),
          source_inner_self_uses,
        ),
      };
      Ok((
        stack_frame2,
        IScoutResult::NormalResult(NormalResultS { expr: mutate_expr_s }),
        source_self_uses.then_merge(&destination_self_uses),
        source_child_uses.then_merge(&destination_child_uses),
      ))
    }
    /*
    case MutatePE(mutateRange, destinationExprPE, sourceExprPE) => {
      val (stackFrame1, sourceExpr1, sourceInnerSelfUses, sourceChildUses) =
        scoutExpressionAndCoerce(stackFrame0, lidb.child(), sourceExprPE, UseP);
      val (stackFrame2, destinationResult1, destinationSelfUses, destinationChildUses) =
        scoutExpression(stackFrame1, lidb.child(), destinationExprPE);
      val (mutateExpr1, sourceSelfUses) =
        destinationResult1 match {
          case LocalLookupResult(range, name) => {
            (LocalMutateSE(range, name, sourceExpr1), sourceInnerSelfUses.markMutated(name))
          }
          case OutsideLookupResult(range, name, maybeTemplateArgs) => {
            throw CompileErrorExceptionS(CouldntFindVarToMutateS(range, name.str))
          }
          case NormalResult(destinationExpr1) => {
            (ExprMutateSE(destinationExpr1.range, destinationExpr1, sourceExpr1), sourceInnerSelfUses)
          }
        }
      (stackFrame2, NormalResult(mutateExpr1), sourceSelfUses.thenMerge(destinationSelfUses), sourceChildUses.thenMerge(destinationChildUses))
    }
    */
    IExpressionPE::Dot(dot) => {
      match dot.left {
        IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(lookup_name), .. })
        if lookup_name.str() == self.keywords.self_
            && stack_frame
            .find_variable(&self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
              name: self.keywords.self_,
            })))
            .is_none() =>
          {
            return Ok((
              stack_frame.clone(),
              IScoutResult::LocalLookupResult(LocalLookupResultS {
                range: PostParser::eval_range(&file_coordinate, lookup_name.range()),
                name: IVarNameS::ConstructingMemberName(self.scout_arena.intern_str(dot.member.str().as_str())),
              }),
              VariableUses::<'s>::empty(),
              VariableUses::<'s>::empty(),
            ));
          }
        _ => {}
      }
      let (stack_frame1, container_expr_s, self_uses, child_uses) = {
        let mut dot_left_lidb = lidb.child();
        let (stack_frame1, container_expr_s, self_uses, child_uses) = self.scout_expression_and_coerce(
          stack_frame.clone(),
          &mut dot_left_lidb,
          dot.left,
          LoadAsP::LoadAsBorrow,
        )?;
        (stack_frame1, container_expr_s, self_uses, child_uses)
      };
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: &*self.scout_arena.alloc(IExpressionSE::Dot(DotSE {
            range: PostParser::eval_range(&file_coordinate, dot.range),
            left: container_expr_s,
            member: self.scout_arena.intern_str(dot.member.str().as_str()),
            borrow_container: true,
          })),
        }),
        self_uses,
        child_uses,
      ))
    }
    /*
    case DotPE(rangeP, containerExprPE, _, NameP(_, memberName)) => {
      containerExprPE match {
        // Here, we're special casing lookups of this.x when we're in a constructor.
        // We know we're in a constructor if there's no `this` variable yet. After all,
        // in a constructor, `this` is just an imaginary concept until we actually
        // fill all the variables.
        case LookupPE(LookupNameP(NameP(range, s)), _) if s == keywords.self && (stackFrame0.findVariable(interner.intern(CodeNameS(interner.intern(StrI("self"))))).isEmpty) => {
          val result = LocalLookupResult(evalRange(range), interner.intern(ConstructingMemberNameS(memberName)))
          (stackFrame0, result, noVariableUses, noVariableUses)
        }
        case _ => {
          val (stackFrame1, containerExpr, selfUses, childUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), containerExprPE, LoadAsBorrowP)
          (stackFrame1, NormalResult(DotSE(evalRange(rangeP), containerExpr, memberName, true)), selfUses, childUses)
        }
      }
    }
    */
        /*
        case IndexPE(range, containerExprPE, Vector(indexExprPE)) => {
          val (stackFrame1, containerExpr1, containerSelfUses, containerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), containerExprPE, LoadAsBorrowP)
          val (stackFrame2, indexExpr1, indexSelfUses, indexChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), indexExprPE, UseP);
          val dot1 = IndexSE(evalRange(range), containerExpr1, indexExpr1)
          (stackFrame2, NormalResult(dot1), containerSelfUses.thenMerge(indexSelfUses), containerChildUses.thenMerge(indexChildUses))
        }
        */
        /*
        case ShortcallPE(range, argExprs) => {
          throw CompileErrorExceptionS(UnimplementedExpression(evalRange(range), "shortcalling"));
        }
        */
    IExpressionPE::Not(not) => {
      let not_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.keywords.not,
      }));
      let load_part = self.scout_arena.alloc(LoadPartSE {
        name: not_name,
        explicit_template_args: &[],
      });
      let callable_expr_s = &*self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
        lookup: OutsideLoadSE {
          range: PostParser::eval_range(&file_coordinate, not.range),
          rules: &[],
          parts: self.scout_arena.alloc_slice_copy(&[&*load_part]),
        },
      }));
      let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = {
        let mut inner_lidb = lidb.child();
        self.scout_expression_and_coerce(stack_frame, &mut inner_lidb, not.inner, LoadAsP::Use)?
      };
      let result = IScoutResult::NormalResult(NormalResultS {
        expr: &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
          range: PostParser::eval_range(&file_coordinate, not.range),
          location: lidb.child().consume_in_arena(self.scout_arena),
          callable_expr: callable_expr_s,
          arg_exprs: self.scout_arena.alloc_slice_from_vec(vec![inner_expr_s]),
        })),
      });
      Ok((stack_frame1, result, inner_self_uses, inner_child_uses))
    }
    IExpressionPE::BraceCall(brace_call) => {
      let load_subject_as = match brace_call.subject_expr {
        IExpressionPE::SubExpression(_) => LoadAsP::Use,
        _ => LoadAsP::LoadAsBorrow,
      };
      assert!(brace_call.arg_exprs.len() == 1);
      let arg_pe = brace_call.arg_exprs[0];
      let (stack_frame1, callable_se, callable_self_uses, callable_child_uses) =
        self.scout_expression_and_coerce(stack_frame, &mut lidb.child(), brace_call.subject_expr, load_subject_as)?;
      let (stack_frame2, arg_se, arg_self_uses, arg_child_uses) =
        self.scout_expression_and_coerce(stack_frame1, &mut lidb.child(), arg_pe, LoadAsP::Use)?;
      let result_se = IExpressionSE::Index(IndexSE {
        range: PostParser::eval_range(&file_coordinate, brace_call.range),
        left: callable_se,
        index_expr: arg_se,
      });
      Ok((stack_frame2, IScoutResult::NormalResult(NormalResultS { expr: self.scout_arena.alloc(result_se) }), callable_self_uses.then_merge(&arg_self_uses), callable_child_uses.then_merge(&arg_child_uses)))
    }
    IExpressionPE::Destruct(destruct_pe) => {
      let (stack_frame1, inner1, inner_self_uses, inner_child_uses) =
        self.scout_expression_and_coerce(stack_frame, &mut lidb.child(), destruct_pe.inner, LoadAsP::Use)?;
      let result_se = IExpressionSE::Destruct(DestructSE {
        range: PostParser::eval_range(&file_coordinate, destruct_pe.range),
        inner: inner1,
      });
      Ok((stack_frame1, IScoutResult::NormalResult(NormalResultS { expr: self.scout_arena.alloc(result_se) }), inner_self_uses, inner_child_uses))
    }
    IExpressionPE::And(and_pe) => {
      let right_range = PostParser::eval_range(&file_coordinate, and_pe.right.range);
      let end_range = RangeS { begin: right_range.end, end: right_range.end };
      let (stack_frame_z, if_se, self_uses, child_uses) = Self::new_if(
        stack_frame, lidb, and_pe.range,
        |sf1, lidb| {
          self.scout_expression_and_coerce(sf1, lidb, and_pe.left, LoadAsP::Use)
        },
        |sf2, lidb| {
          let (then_se, then_uses, then_child_uses) =
            self.scout_impure_block(sf2.clone(), &mut lidb.child(), PostParser::<'s, 'p, '_>::no_declarations(), and_pe.right)?;
          Ok((sf2, then_se, then_uses, then_child_uses))
        },
        |sf3, _lidb| {
          let false_const = self.scout_arena.alloc(IExpressionSE::ConstantBool(ConstantBoolSE {
            range: end_range,
            value: false,
          }));
          let else_se = self.scout_arena.alloc(BlockSE {
            range: end_range,
            locals: &[],
            expr: false_const,
          });
          Ok((sf3, &*else_se, VariableUses::<'s>::empty(), VariableUses::<'s>::empty()))
        },
      )?;
      let if_alloc = self.scout_arena.alloc(IExpressionSE::If(if_se));
      Ok((stack_frame_z, IScoutResult::NormalResult(NormalResultS { expr: if_alloc }), self_uses, child_uses))
    }
    IExpressionPE::Or(or_pe) => {
      let right_range = PostParser::eval_range(&file_coordinate, or_pe.right.range);
      let end_range = RangeS { begin: right_range.end, end: right_range.end };
      let (stack_frame_z, if_se, self_uses, child_uses) = Self::new_if(
        stack_frame, lidb, or_pe.range,
        |sf1, lidb| {
          self.scout_expression_and_coerce(sf1, lidb, or_pe.left, LoadAsP::Use)
        },
        |sf2, _lidb| {
          let true_const = self.scout_arena.alloc(IExpressionSE::ConstantBool(ConstantBoolSE {
            range: end_range,
            value: true,
          }));
          let else_se = self.scout_arena.alloc(BlockSE {
            range: end_range,
            locals: &[],
            expr: true_const,
          });
          Ok((sf2, &*else_se, VariableUses::<'s>::empty(), VariableUses::<'s>::empty()))
        },
        |sf3, lidb| {
          let (then_se, then_uses, then_child_uses) =
            self.scout_impure_block(sf3.clone(), &mut lidb.child(), PostParser::<'s, 'p, '_>::no_declarations(), or_pe.right)?;
          Ok((sf3, then_se, then_uses, then_child_uses))
        },
      )?;
      let if_alloc = self.scout_arena.alloc(IExpressionSE::If(if_se));
      Ok((stack_frame_z, IScoutResult::NormalResult(NormalResultS { expr: if_alloc }), self_uses, child_uses))
    }
    IExpressionPE::Tuple(tuple_pe) => {
      let (stack_frame1, elements1, self_uses, child_uses) =
        self.scout_elements_as_expressions(stack_frame, &mut lidb.child(), tuple_pe.elements)?;
      let result_se = IExpressionSE::Tuple(TupleSE {
        range: PostParser::eval_range(&file_coordinate, tuple_pe.range),
        elements: self.scout_arena.alloc_slice_from_vec(elements1),
      });
      Ok((stack_frame1, IScoutResult::NormalResult(NormalResultS { expr: self.scout_arena.alloc(result_se) }), self_uses, child_uses))
    }
    IExpressionPE::StrInterpolate(str_interp_pe) => {
      let (stack_frame1, parts_se, parts_self_uses, parts_child_uses) =
        self.scout_elements_as_expressions(stack_frame, &mut lidb.child(), str_interp_pe.parts)?;

      let range_s = PostParser::eval_range(&file_coordinate, str_interp_pe.range);
      let starting_expr: &'s IExpressionSE<'s> =
        self.scout_arena.alloc(IExpressionSE::ConstantStr(ConstantStrSE {
          range: RangeS { begin: range_s.begin, end: range_s.begin },
          value: self.scout_arena.intern_str(""),
        }));
      let added_expr = parts_se.iter().fold(starting_expr, |prev_expr, part_se| {
        let add_call_range = RangeS {
          begin: prev_expr.range().end,
          end: part_se.range().begin,
        };
        let plus_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
          name: self.keywords.plus,
        }));
        let load_part = self.scout_arena.alloc(LoadPartSE {
          name: plus_name,
          explicit_template_args: &[],
        });
        let callable_expr = self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
          lookup: OutsideLoadSE {
            range: add_call_range,
            rules: &[],
            parts: self.scout_arena.alloc_slice_copy(&[&*load_part]),
          },
        }));
        let args = self.scout_arena.alloc_slice_from_vec(vec![prev_expr, *part_se]);
        self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
          range: add_call_range,
          location: lidb.child().consume_in_arena(self.scout_arena),
          callable_expr,
          arg_exprs: args,
        }))
      });
      Ok((stack_frame1, IScoutResult::NormalResult(NormalResultS { expr: added_expr }), parts_self_uses, parts_child_uses))
    }
    IExpressionPE::Break(break_pe) => {
      let range_s = PostParser::eval_range(&file_coordinate, break_pe.range);
      let expr = self.scout_arena.alloc(IExpressionSE::Break(crate::postparsing::expressions::BreakSE { range: range_s }));
      Ok((stack_frame, IScoutResult::NormalResult(NormalResultS { expr }), VariableUses::empty(), VariableUses::empty()))
    }
    IExpressionPE::Range(range_pe) => {
      let range_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.keywords.range,
      }));
      let load_part = self.scout_arena.alloc(LoadPartSE {
        name: range_name,
        explicit_template_args: &[],
      });
      let callable_se = &*self.scout_arena.alloc(IExpressionSE::OverloadSet(OverloadSetSE {
        lookup: OutsideLoadSE {
          range: PostParser::eval_range(&file_coordinate, range_pe.range),
          rules: &[],
          parts: self.scout_arena.alloc_slice_copy(&[&*load_part]),
        },
      }));
      let load_begin_as = match range_pe.from_expr {
        IExpressionPE::SubExpression(_) => LoadAsP::Use,
        _ => LoadAsP::LoadAsBorrow,
      };
      let (stack_frame1, begin_se, begin_self_uses, begin_child_uses) =
        self.scout_expression_and_coerce(stack_frame, &mut lidb.child(), range_pe.from_expr, load_begin_as)?;
      let load_end_as = match range_pe.to_expr {
        IExpressionPE::SubExpression(_) => LoadAsP::Use,
        _ => LoadAsP::LoadAsBorrow,
      };
      let (stack_frame2, end_se, end_self_uses, end_child_uses) =
        self.scout_expression_and_coerce(stack_frame1, &mut lidb.child(), range_pe.to_expr, load_end_as)?;
      let result_se = &*self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
        range: PostParser::eval_range(&file_coordinate, range_pe.range),
        location: lidb.child().consume_in_arena(self.scout_arena),
        callable_expr: callable_se,
        arg_exprs: self.scout_arena.alloc_slice_from_vec(vec![begin_se, end_se]),
      }));
      Ok((stack_frame2, IScoutResult::NormalResult(NormalResultS { expr: result_se }), begin_self_uses.then_merge(&end_self_uses), begin_child_uses.then_merge(&end_child_uses)))
    }
    _ => panic!(
      "POSTPARSER_SCOUT_EXPRESSION_NOT_YET_IMPLEMENTED: {:?}",
      expression
    ),
  }
}
  /*
}
})
}
*/

pub(crate) fn new_if<FCond, FThen, FElse>(
  stack_frame0: StackFrame<'s>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeL,
  make_condition: FCond,
  make_then: FThen,
  make_else: FElse,
) -> Result<(StackFrame<'s>, IfSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
where
  FCond: FnOnce(
    StackFrame<'s>,
    &mut LocationInDenizenBuilder,
  ) -> Result<
    (
      StackFrame<'s>,
      &'s IExpressionSE<'s>,
      VariableUses<'s>,
      VariableUses<'s>,
    ),
    ICompileErrorS<'s>,
  >,
  FThen: FnOnce(
    StackFrame<'s>,
    &mut LocationInDenizenBuilder,
  ) -> Result<(StackFrame<'s>, &'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>,
  FElse: FnOnce(
    StackFrame<'s>,
    &mut LocationInDenizenBuilder,
  ) -> Result<(StackFrame<'s>, &'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>,
{
  let file = stack_frame0.file;
  let (stack_frame1, cond_se, cond_uses, cond_child_uses): (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>) = make_condition(stack_frame0, &mut lidb.child())?;
  let (stack_frame2, then_se, then_uses, then_child_uses): (StackFrame<'s>, &'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = make_then(stack_frame1, &mut lidb.child())?;
  let (stack_frame3, else_se, else_uses, else_child_uses): (StackFrame<'s>, &'s BlockSE<'s>, VariableUses<'s>, VariableUses<'s>) = make_else(stack_frame2, &mut lidb.child())?;

  let self_case_uses = then_uses.branch_merge(&else_uses);
  let self_uses = cond_uses.then_merge(&self_case_uses);
  let child_case_uses = then_child_uses.branch_merge(&else_child_uses);
  let child_uses = cond_child_uses.then_merge(&child_case_uses);

  let if_se = IfSE {
    range: PostParser::eval_range(file, range),
    condition: cond_se,
    then_body: then_se,
    else_body: else_se,
  };
  Ok((stack_frame3, if_se, self_uses, child_uses))
}
/*
  def newIf(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    makeCondition: (StackFrame, LocationInDenizenBuilder) => (StackFrame, IExpressionSE, VariableUses, VariableUses),
    makeThen: (StackFrame, LocationInDenizenBuilder) => (StackFrame, BlockSE, VariableUses, VariableUses),
    makeElse: (StackFrame, LocationInDenizenBuilder) => (StackFrame, BlockSE, VariableUses, VariableUses)):
  (StackFrame, IfSE, VariableUses, VariableUses) = {
    val (stackFrame1, condSE, condUses, condChildUses) =
      makeCondition(stackFrame0, lidb.child())

    val (stackFrame2, thenSE, thenUses, thenChildUses) =
      makeThen(stackFrame1, lidb.child())
    val (stackFrame3, elseSE, elseUses, elseChildUses) =
      makeElse(stackFrame2, lidb.child())

    val selfCaseUses = thenUses.branchMerge(elseUses)
    val selfUses = condUses.thenMerge(selfCaseUses);
    val childCaseUses = thenChildUses.branchMerge(elseChildUses)
    val childUses = condChildUses.thenMerge(childCaseUses);

    val ifSE =
      IfSE(PostParser.evalRange(stackFrame0.file, range), condSE, thenSE, elseSE)
    (stackFrame3, ifSE, selfUses, childUses)
  }
*/
pub(crate) fn translate_maybe_template_args(
    &self,
    parent_env: &IEnvironmentS<'s>,
    lidb: &mut LocationInDenizenBuilder,
    rule_builder: &mut Vec<IRulexSR<'s>>,
    context_region: IRuneS<'s>,
    maybe_template_args: Option<&'p [&'p ITemplexPT<'p>]>,
  ) -> &'s [RuneUsage<'s>] {
    match maybe_template_args {
      None => &[],
      Some(template_args) => {
        let runes: Vec<RuneUsage<'s>> = template_args.iter().map(|template_arg| {
          let mut child_lidb = lidb.child();
          translate_templex(
            self.scout_arena,
            self.keywords,
            parent_env.clone(),
            &mut child_lidb,
            rule_builder,
            context_region.clone(),
            template_arg,
          )
        }).collect();
        self.scout_arena.alloc_slice_from_vec(runes)
      }
    }
  }
// (audit-trail for translateMaybeTemplateArgs lives at canonical position in scoutExpression's
//  audit-trail block above; no duplicate adjacent block needed.)

// If we load an immutable with targetOwnershipIfLookupResult = Own or Borrow, it will just be Share.
pub(crate) fn scout_expression_and_coerce(
    &self,
    stack_frame: StackFrame<'s>,
    lidb: &mut LocationInDenizenBuilder,
    expression_p: &'p IExpressionPE<'p>,
    load_as_p: LoadAsP,
  ) -> Result<(StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
  {
    let mut expression_lidb = lidb.child();
    let (next_stack_frame, first_result_s, first_inner_self_uses, first_child_uses) = self.scout_expression(
      stack_frame,
      &mut expression_lidb,
      expression_p,
    )?;
    let mut coerce_lidb = lidb.child();
    let (next_stack_frame, first_expr_s, first_self_uses) =
      self.coerce(next_stack_frame, first_result_s, &mut coerce_lidb, first_inner_self_uses, load_as_p)?;
    Ok((next_stack_frame, first_expr_s, first_self_uses, first_child_uses))
  }
/*
  // If we load an immutable with targetOwnershipIfLookupResult = Own or Borrow, it will just be Share.
  def scoutExpressionAndCoerce(
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    exprPE: IExpressionPE,
    loadAsP: LoadAsP):
  (StackFrame, IExpressionSE, VariableUses, VariableUses) = {
    val (stackFrame1, uncoercedScoutResult, exprSelfUses, exprChildUses) =
      scoutExpression(stackFrame0, lidb.child(), exprPE)
    val (stackFrame2, coercedExpr, coercedSelfUses) =
      coerce(stackFrame1, uncoercedScoutResult, lidb.child(), exprSelfUses, loadAsP)
    (stackFrame2, coercedExpr, coercedSelfUses, exprChildUses)
  }
*/

pub(crate) fn coerce(
    &self,
    stack_frame: StackFrame<'s>,
    first_result: IScoutResult<'s, 'p>,
    _lidb: &mut LocationInDenizenBuilder,
    self_uses_before: VariableUses<'s>,
    load_as_p: LoadAsP,
  ) -> Result<(StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>), ICompileErrorS<'s>> {
    match first_result {
      IScoutResult::LocalLookupResult(LocalLookupResultS { range, name }) => {
        let self_uses_after = match load_as_p {
          LoadAsP::LoadAsBorrow => self_uses_before.mark_borrowed(name.clone()),
          LoadAsP::LoadAsWeak => self_uses_before.mark_borrowed(name.clone()),
          LoadAsP::Use | LoadAsP::Move => self_uses_before.mark_moved(name.clone()),
        };
        Ok((
          stack_frame,
          &*self.scout_arena.alloc(IExpressionSE::LocalLoad(LocalLoadSE {
            range,
            name,
            target_ownership: load_as_p,
          })),
          self_uses_after,
        ))
      }
      IScoutResult::OutsideLookupResult(OutsideLookupResultS { range, name: _, template_args: _ }) => {
        // When we have globals, here's where we'd check that the user declared they're accessing a global.
        // But since we don't have globals yet, we just throw an error...
        // V: open question... why don't we coerce to overload set here? Is that done somewhere else?
        Err(ICompileErrorS::CouldntFindVarToMutateS(CouldntFindVarToMutateS {
          range,
          name: "a".to_string(),
        }))
      }
      IScoutResult::NormalResult(NormalResultS { expr: inner_expr_s }) => {
        match load_as_p {
          LoadAsP::Use => Ok((stack_frame, inner_expr_s, self_uses_before)),
          _ => Ok((
            stack_frame,
            &*self.scout_arena.alloc(IExpressionSE::Ownershipped(OwnershippedSE {
              range: inner_expr_s.range(),
              inner_expr: inner_expr_s,
              target_ownership: load_as_p,
            })),
            self_uses_before,
          )),
        }
      }
    }
  }
/*
  def coerce(
      stackFrame0: StackFrame,
      firstResult1: IScoutResult[IExpressionSE],
      lidb: LocationInDenizenBuilder,
      selfUsesBefore: VariableUses,
      loadAsP: LoadAsP
  ): (StackFrame, IExpressionSE, VariableUses) = {
    firstResult1 match {
      case LocalLookupResult(range, name) => {
        val selfUsesAfter =
          loadAsP match {
            case LoadAsBorrowP => selfUsesBefore.markBorrowed(name)
            case LoadAsWeakP => selfUsesBefore.markBorrowed(name)
            case UseP | MoveP => selfUsesBefore.markMoved(name)
          }
        (stackFrame0, LocalLoadSE(range, name, loadAsP), selfUsesAfter)
      }
      case OutsideLookupResult(range, name, maybeTemplateArgs) => {
        // When we have globals, here's where we'd check that the user declared they're accessing a global.
        // But since we don't have globals yet, we just throw an error...
        // V: open question... why don't we coerce to overload set here? Is that done somewhere else?
        throw CompileErrorExceptionS(CouldntFindVarToMutateS(range, "a"))
      }
      case NormalResult(innerExpr1) => {
        loadAsP match {
          case UseP => (stackFrame0, innerExpr1, selfUsesBefore)
          case _ => (stackFrame0, OwnershippedSE(innerExpr1.range, innerExpr1, loadAsP), selfUsesBefore)
        }
      }
    }
  }
*/
pub(crate) fn scout_elements_as_expressions(
    &self,
    initial_stack_frame: StackFrame<'s>,
    lidb: &mut LocationInDenizenBuilder,
    exprs_p: &'p [&'p IExpressionPE<'p>],
  ) -> Result<(StackFrame<'s>, Vec<&'s IExpressionSE<'s>>, VariableUses<'s>, VariableUses<'s>), ICompileErrorS<'s>>
  {
    let mut self_uses = VariableUses::<'s>::empty();
    let mut child_uses = VariableUses::<'s>::empty();
    let mut exprs_s = Vec::new();
    let mut stack_frame = initial_stack_frame;
    for expr_p in exprs_p {
      let mut expr_lidb = lidb.child();
      let (next_stack_frame, expr_s, first_self_uses, first_child_uses) =
        self.scout_expression_and_coerce(stack_frame, &mut expr_lidb, expr_p, LoadAsP::Use)?;
      stack_frame = next_stack_frame;
      self_uses = self_uses.then_merge(&first_self_uses);
      child_uses = child_uses.then_merge(&first_child_uses);
      exprs_s.push(expr_s);
    }
    Ok((stack_frame, exprs_s, self_uses, child_uses))
  }

/*
  // Need a better name for this...
  // It's more like, scout elements as non-lookups, in other words,
  // if we get lookups then coerce them into moves.
  def scoutElementsAsExpressions(
    initialStackFramePE: StackFrame,
    lidb: LocationInDenizenBuilder,
    exprsPE: Vector[IExpressionPE]):
  (StackFrame, Vector[IExpressionSE], VariableUses, VariableUses) = {
    var selfUses = noVariableUses
    var childUses = noVariableUses
    val (finalStackFrame, reversedExprsSE) =
      exprsPE.foldLeft((initialStackFramePE, List[IExpressionSE]()))({
        case ((prevStackFrame, reversedPrevExprsSE), exprPE) => {
          val (nextStackFrame, exprSE, firstSelfUses, firstChildUses) =
            scoutExpressionAndCoerce(prevStackFrame, lidb.child(), exprPE, UseP)
          selfUses = selfUses.thenMerge(firstSelfUses)
          childUses = childUses.thenMerge(firstChildUses)
          (nextStackFrame, exprSE :: reversedPrevExprsSE)
        }
      })
    (finalStackFrame, reversedExprsSE.toVector.reverse, selfUses, childUses)
  }
}
*/
}

/*
object ExpressionScout {
*/
fn flatten_expressions<'s>(
  _expr: &IExpressionSE<'s>,
) -> Vec<&'s IExpressionSE<'s>> {
  panic!("Unimplemented flatten_expressions");
}
/*
  def flattenExpressions(expr: IExpressionSE): Vector[IExpressionSE] = {
    expr match {
      case ConsecutorSE(exprs) => exprs.flatMap(flattenExpressions)
      case other => Vector(other)
    }
  }
}
*/

