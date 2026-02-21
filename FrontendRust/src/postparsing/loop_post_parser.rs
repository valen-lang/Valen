/*
package dev.vale.postparsing

import dev.vale.parsing.ast._
import PostParser.{noDeclarations, noVariableUses}
import dev.vale.lexing.RangeL
import dev.vale.parsing.ast.{AugmentPE, BlockPE, BorrowP, ConsecutorPE, FunctionCallPE, IExpressionPE, IterableNameDeclarationP, IterableNameP, IterationOptionNameDeclarationP, IterationOptionNameP, IteratorNameDeclarationP, IteratorNameP, LetPE, LookupNameP, LookupPE, NameP, PatternPP, UseP}
import dev.vale.{Interner, Keywords, StrI, postparsing}
*/
/*
class LoopPostParser(interner: Interner, keywords: Keywords) {
*/
use crate::parsing::ast::{
  AugmentPE, BlockPE, ConsecutorPE, DestinationLocalP, FunctionCallPE, IExpressionPE,
  IImpreciseNameP, INameDeclarationP, LetPE, LoadAsP, LookupPE, NameP, OwnershipP, PatternPP,
};
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::expressions::{
  BlockSE, BreakSE, IExpressionSE, MapSE, VoidSE, WhileSE,
};
use crate::postparsing::post_parser::{ICompileErrorS, PostParser, StackFrame};
use crate::postparsing::variable_uses::VariableUses;
use crate::lexing::ast::RangeL;

fn scout_loop<'a, 's, F>(
  _stack_frame0: crate::postparsing::post_parser::StackFrame<'a>,
  _lidb: &mut crate::postparsing::ast::LocationInDenizenBuilder,
  _range_p: crate::lexing::ast::RangeL,
  _pure: bool,
  _make_contents: F,
) -> (
  crate::postparsing::expressions::BlockSE<'a, 's>,
  crate::postparsing::variable_uses::VariableUses<'a>,
  crate::postparsing::variable_uses::VariableUses<'a>,
)
where
  F: FnOnce(
    crate::postparsing::post_parser::StackFrame<'a>,
    &mut crate::postparsing::ast::LocationInDenizenBuilder,
    bool,
  ) -> (
    crate::postparsing::post_parser::StackFrame<'a>,
    crate::postparsing::expressions::BlockSE<'a, 's>,
    crate::postparsing::variable_uses::VariableUses<'a>,
    crate::postparsing::variable_uses::VariableUses<'a>,
  ),
{
  panic!("Unimplemented scout_loop");
}
/*
  def scoutLoop(
    expressionScout: ExpressionScout,
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    rangeP: RangeL,
    pure: Boolean,
    makeContents: (StackFrame, LocationInDenizenBuilder, Boolean) => (StackFrame, BlockSE, VariableUses, VariableUses)):
  (BlockSE, VariableUses, VariableUses) = {
    // This just scopes the iterable's expression so its things dont outlive the foreach block.
    expressionScout.newBlock(
      stackFrame0.parentEnv, Some(stackFrame0), lidb.child(), PostParser.evalRange(stackFrame0.file, rangeP),
      stackFrame0.contextRegion,
      noDeclarations,
      (stackFrame1, lidb) => {
        val (stackFrame2, bodySE, selfUses, childUses) =
          makeContents(stackFrame1, lidb, true)
        val whileSE = postparsing.WhileSE(PostParser.evalRange(stackFrame0.file, rangeP), bodySE)
        (stackFrame2, whileSE, selfUses, childUses)
      })
  }
*/
pub(crate) fn scout_each<'a, 'p, 'ctx, 's>(
  post_parser: &PostParser<'a, 'p, 'ctx, 's>,
  stack_frame0: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeL,
  _pure: bool,
  entry_pattern_pp: &PatternPP<'a, 'p>,
  in_keyword_range: RangeL,
  iterable_expr: &IExpressionPE<'a, 'p>,
  body: &BlockPE<'a, 'p>,
) -> Result<(&'s BlockSE<'a, 's>, VariableUses<'a>, VariableUses<'a>), ICompileErrorS<'a>>
where
  'a: 'ctx,
  'a: 'p,
  'a: 'p,
  'a: 's,
{
  let each_range_s = PostParser::eval_range(stack_frame0.file, range);
  let parent_env0 = stack_frame0.parent_env.clone();
  let context_region0 = stack_frame0.context_region.clone();
  let mut each_lidb = lidb.child();
  let (each_block_s, self_uses, child_uses) = post_parser.new_block(
    parent_env0,
    Some(stack_frame0),
    &mut each_lidb,
    each_range_s.clone(),
    context_region0,
    PostParser::no_declarations(),
    |stack_frame1, each_contents_lidb| {
      let (stack_frame2, let_iterable_se, let_iterable_self_uses, let_iterable_child_uses) = {
        let let_iterable_expr_p = IExpressionPE::Let(LetPE {
          range: in_keyword_range,
          pattern: PatternPP {
            range: in_keyword_range,
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::IterableNameDeclaration(in_keyword_range),
              mutate: None,
            }),
            templex: None,
            destructure: None,
          },
          source: iterable_expr,
        });
        post_parser.scout_expression_and_coerce(
          stack_frame1,
          &mut each_contents_lidb.child(),
          &let_iterable_expr_p,
          LoadAsP::Use,
        )?
      };
      let (stack_frame3, let_iterator_se, let_iterator_self_uses, let_iterator_child_uses) = {
        let begin_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(in_keyword_range, post_parser.keywords.begin)),
          template_args: None,
        });
        let iterable_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::IterableName(in_keyword_range),
          template_args: None,
        });
        let iterable_borrow_expr_p = IExpressionPE::Augment(AugmentPE {
          range: in_keyword_range,
          target_ownership: OwnershipP::Borrow,
          inner: &iterable_lookup_expr_p,
        });
        let begin_args = [iterable_borrow_expr_p];
        let begin_call_expr_p = IExpressionPE::FunctionCall(FunctionCallPE {
          range: in_keyword_range,
          operator_range: in_keyword_range,
          callable_expr: &begin_lookup_expr_p,
          arg_exprs: &begin_args,
        });
        let let_iterator_expr_p = IExpressionPE::Let(LetPE {
          range: in_keyword_range,
          pattern: PatternPP {
            range: in_keyword_range,
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::IteratorNameDeclaration(in_keyword_range),
              mutate: None,
            }),
            templex: None,
            destructure: None,
          },
          source: &begin_call_expr_p,
        });
        post_parser.scout_expression_and_coerce(
          stack_frame2,
          &mut each_contents_lidb.child(),
          &let_iterator_expr_p,
          LoadAsP::Use,
        )?
      };
      let parent_env3 = stack_frame3.parent_env.clone();
      let context_region3 = stack_frame3.context_region.clone();
      let (loop_se, loop_body_self_uses, loop_body_child_uses) = post_parser.new_block(
        parent_env3,
        Some(stack_frame3.clone()),
        &mut each_contents_lidb.child(),
        each_range_s.clone(),
        context_region3,
        PostParser::no_declarations(),
        |stack_frame4, loop_lidb| {
          let parent_env4 = stack_frame4.parent_env.clone();
          let context_region4 = stack_frame4.context_region.clone();
          let (loop_body_se, loop_body_self_uses, loop_body_child_uses) = post_parser.new_block(
            parent_env4,
            Some(stack_frame4.clone()),
            &mut loop_lidb.child(),
            each_range_s.clone(),
            context_region4,
            PostParser::no_declarations(),
            |stack_frame5, loop_body_lidb| {
              scout_each_body(
                post_parser,
                stack_frame5,
                loop_body_lidb,
                range,
                in_keyword_range,
                entry_pattern_pp,
                body,
              )
            },
          )?;
          let loop_se = if body.inner.produces_result() {
            &*post_parser.scout_arena.alloc(IExpressionSE::Map(MapSE {
              range: each_range_s.clone(),
              body: loop_body_se,
            }))
          } else {
            &*post_parser.scout_arena.alloc(IExpressionSE::While(WhileSE {
              range: each_range_s.clone(),
              body: loop_body_se,
            }))
          };
          Ok((stack_frame4.clone(), loop_se, loop_body_self_uses, loop_body_child_uses))
        },
      )?;
      let loop_se = &*post_parser.scout_arena.alloc(IExpressionSE::Block(loop_se));
      let contents_se = post_parser.consecutive(vec![let_iterable_se, let_iterator_se, loop_se]);
      let self_uses = let_iterable_self_uses
        .then_merge(&let_iterator_self_uses)
        .then_merge(&loop_body_self_uses);
      let child_uses = let_iterable_child_uses
        .then_merge(&let_iterator_child_uses)
        .then_merge(&loop_body_child_uses);
      Ok((stack_frame3, contents_se, self_uses, child_uses))
    },
  )?;
  Ok((each_block_s, self_uses, child_uses))
}
/*
  def scoutEach(
    expressionScout: ExpressionScout,
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    pure: Boolean,
    entryPatternPP: PatternPP,
    inKeywordRange: RangeL,
    iterableExpr: IExpressionPE,
    body: BlockPE):
  (BlockSE, VariableUses, VariableUses) = {
    expressionScout.newBlock(
      stackFrame0.parentEnv, Some(stackFrame0), lidb.child(), PostParser.evalRange(stackFrame0.file, range),
      stackFrame0.contextRegion,
      noDeclarations,
      (stackFrame1, lidb) => {
        val (stackFrame2, letIterableSE, letIterableSelfUses, letIterableChildUses) =
          expressionScout.scoutExpressionAndCoerce(
            stackFrame1, lidb.child(),
            LetPE(
              inKeywordRange,
              PatternPP(inKeywordRange, Some(DestinationLocalP(IterableNameDeclarationP(inKeywordRange), None)), None, None),
              iterableExpr),
            UseP)
        val (stackFrame3, letIteratorSE, letIteratorSelfUses, letIteratorChildUses) =
          expressionScout.scoutExpressionAndCoerce(
            stackFrame2, lidb.child(),
            LetPE(
              inKeywordRange,
              PatternPP(inKeywordRange, Some(DestinationLocalP(IteratorNameDeclarationP(inKeywordRange), None)), None, None),
              FunctionCallPE(
                inKeywordRange, inKeywordRange,
                LookupPE(LookupNameP(NameP(inKeywordRange, keywords.begin)), None),
                Vector(
                  AugmentPE(
                    inKeywordRange, BorrowP,
                    LookupPE(IterableNameP(inKeywordRange), None))))),
            UseP)

        val (loopSE, loopBodySelfUses, loopBodyChildUses) =
          expressionScout.newBlock(
            stackFrame3.parentEnv, Some(stackFrame3), lidb.child(), PostParser.evalRange(stackFrame0.file, range),
            stackFrame3.contextRegion,
            noDeclarations,
            (stackFrame4, lidb) => {
              val (loopBodySE, loopBodySelfUses, loopBodyChildUses) =
                expressionScout.newBlock(
                  stackFrame4.parentEnv, Some(stackFrame4), lidb.child(), PostParser.evalRange(stackFrame0.file, range),
                  stackFrame4.contextRegion,
                  noDeclarations,
                  (stackFrame5, lidb) => {
                    scoutEachBody(expressionScout, stackFrame5, lidb, range, inKeywordRange, entryPatternPP, body)
                  })
              val loopSE =
                if (body.producesResult()) {
                  MapSE(PostParser.evalRange(stackFrame0.file, range), loopBodySE)
                } else {
                  postparsing.WhileSE(PostParser.evalRange(stackFrame0.file, range), loopBodySE)
                }
              (stackFrame4, loopSE, loopBodySelfUses, loopBodyChildUses)
            })

        val contentsSE = PostParser.consecutive(Vector(letIterableSE, letIteratorSE, loopSE))

        val selfUses = letIterableSelfUses.thenMerge(letIteratorSelfUses).thenMerge(loopBodySelfUses)
        val childUses = letIterableChildUses.thenMerge(letIteratorChildUses).thenMerge(loopBodyChildUses)

        (stackFrame3, contentsSE, selfUses, childUses)
      })
  }
*/
fn scout_each_body<'a, 'p, 'ctx, 's>(
  post_parser: &PostParser<'a, 'p, 'ctx, 's>,
  stack_frame0: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeL,
  in_keyword_range: RangeL,
  entry_pattern_pp: &PatternPP<'a, 'p>,
  body_pe: &BlockPE<'a, 'p>,
) -> Result<
  (
    StackFrame<'a>,
    &'s crate::postparsing::expressions::IExpressionSE<'a, 's>,
    VariableUses<'a>,
    VariableUses<'a>,
  ),
  ICompileErrorS<'a>,
>
where
  'a: 'ctx,
  'a: 'p,
  'a: 's,
{
  let each_range_s = PostParser::eval_range(stack_frame0.file, range);
  let (stack_frame4, if_se, if_self_uses, if_child_uses) = PostParser::new_if(
    stack_frame0,
    lidb,
    range,
    |stack_frame1, condition_lidb| {
      let next_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(in_keyword_range, post_parser.keywords.next)),
        template_args: None,
      });
      let iterator_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::IteratorName(in_keyword_range),
        template_args: None,
      });
      let iterator_borrow_expr_p = IExpressionPE::Augment(AugmentPE {
        range: in_keyword_range,
        target_ownership: OwnershipP::Borrow,
        inner: &iterator_lookup_expr_p,
      });
      let next_args = [iterator_borrow_expr_p];
      let next_call_expr_p = IExpressionPE::FunctionCall(FunctionCallPE {
        range: in_keyword_range,
        operator_range: in_keyword_range,
        callable_expr: &next_lookup_expr_p,
        arg_exprs: &next_args,
      });
      let let_iteration_option_expr_p = IExpressionPE::Let(LetPE {
        range: entry_pattern_pp.range,
        pattern: PatternPP {
          range: in_keyword_range,
          destination: Some(DestinationLocalP {
            decl: INameDeclarationP::IterationOptionNameDeclaration(in_keyword_range),
            mutate: None,
          }),
          templex: None,
          destructure: None,
        },
        source: &next_call_expr_p,
      });
      let is_empty_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(in_keyword_range, post_parser.keywords.is_empty)),
        template_args: None,
      });
      let iteration_option_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::IterationOptionName(in_keyword_range),
        template_args: None,
      });
      let iteration_option_borrow_expr_p = IExpressionPE::Augment(AugmentPE {
        range: in_keyword_range,
        target_ownership: OwnershipP::Borrow,
        inner: &iteration_option_lookup_expr_p,
      });
      let is_empty_args = [iteration_option_borrow_expr_p];
      let is_empty_call_expr_p = IExpressionPE::FunctionCall(FunctionCallPE {
        range: in_keyword_range,
        operator_range: in_keyword_range,
        callable_expr: &is_empty_lookup_expr_p,
        arg_exprs: &is_empty_args,
      });
      let condition_inners = [let_iteration_option_expr_p, is_empty_call_expr_p];
      let condition_expr_p = IExpressionPE::Consecutor(ConsecutorPE {
        inners: &condition_inners,
      });
      let (stack_frame3, cond_se, cond_self_uses, cond_child_uses) = post_parser.scout_expression_and_coerce(
        stack_frame1,
        condition_lidb,
        &condition_expr_p,
        LoadAsP::Use,
      )?;
      Ok((stack_frame3, cond_se, cond_self_uses, cond_child_uses))
    },
    |stack_frame1, then_lidb| {
      let parent_env1 = stack_frame1.parent_env.clone();
      let context_region1 = stack_frame1.context_region.clone();
      let (then_s, then_uses, then_child_uses) = post_parser.new_block(
        parent_env1,
        Some(stack_frame1.clone()),
        then_lidb,
        each_range_s.clone(),
        context_region1,
        PostParser::no_declarations(),
        |stack_frame2, then_inner_lidb| {
          let iteration_option_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::IterationOptionName(in_keyword_range),
            template_args: None,
          });
          let (stack_frame3, lookup_se, lookup_self_uses, lookup_child_uses) = post_parser
            .scout_expression_and_coerce(
              stack_frame2,
              then_inner_lidb,
              &iteration_option_lookup_expr_p,
              LoadAsP::Use,
            )?;
          let break_s = &*post_parser.scout_arena.alloc(IExpressionSE::Break(BreakSE {
            range: each_range_s.clone(),
          }));
          let lookup_and_break_se = post_parser.consecutive(vec![lookup_se, break_s]);
          Ok((stack_frame3, lookup_and_break_se, lookup_self_uses, lookup_child_uses))
        },
      )?;
      Ok((stack_frame1, then_s, then_uses, then_child_uses))
    },
    |stack_frame1, _else_lidb| {
      // Else does nothing
      let else_s = &*post_parser.scout_arena.alloc(BlockSE {
        range: each_range_s.clone(),
        locals: Vec::new(),
        expr: &*post_parser.scout_arena.alloc(IExpressionSE::Void(VoidSE {
          range: each_range_s.clone(),
        })),
      });
      Ok((
        stack_frame1,
        else_s,
        PostParser::no_variable_uses(),
        PostParser::no_variable_uses(),
      ))
    },
  )?;
  let if_se = &*post_parser.scout_arena.alloc(IExpressionSE::If(if_se));

  let (stack_frame5, consume_some_se, consume_some_self_uses, consume_some_child_uses) = {
    let get_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
      name: IImpreciseNameP::LookupName(NameP(in_keyword_range, post_parser.keywords.get)),
      template_args: None,
    });
    let iteration_option_lookup_expr_p = IExpressionPE::Lookup(LookupPE {
      name: IImpreciseNameP::IterationOptionName(in_keyword_range),
      template_args: None,
    });
    let get_args = [iteration_option_lookup_expr_p];
    let get_call_expr_p = IExpressionPE::FunctionCall(FunctionCallPE {
      range: in_keyword_range,
      operator_range: in_keyword_range,
      callable_expr: &get_lookup_expr_p,
      arg_exprs: &get_args,
    });
    let consume_some_expr_p = IExpressionPE::Let(LetPE {
      range: in_keyword_range,
      pattern: entry_pattern_pp.clone(),
      source: &get_call_expr_p,
    });
    let mut consume_some_lidb = lidb.child();
    post_parser.scout_expression_and_coerce(
      stack_frame4,
      &mut consume_some_lidb,
      &consume_some_expr_p,
      LoadAsP::Use,
    )?
  };

  let (user_body_se, user_body_self_uses, user_body_child_uses) = post_parser.scout_block(
    stack_frame5.clone(),
    &mut lidb.child(),
    PostParser::no_declarations(),
    body_pe,
  )?;

  let self_uses = if_self_uses
    .then_merge(&consume_some_self_uses)
    .then_merge(&user_body_self_uses);
  let child_uses = if_child_uses
    .then_merge(&consume_some_child_uses)
    .then_merge(&user_body_child_uses);
  let loop_body_se = post_parser.consecutive(vec![if_se, consume_some_se, user_body_se]);
  Ok((stack_frame5, loop_body_se, self_uses, child_uses))
}
/*
  def scoutEachBody(
    expressionScout: ExpressionScout,
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    inKeywordRange: RangeL,
    entryPatternPP: PatternPP,
    bodyPE: BlockPE,
  ): (StackFrame, IExpressionSE, VariableUses, VariableUses) = {
    val (stackFrame4, ifSE, ifSelfUses, ifChildUses) =
      expressionScout.newIf(
        stackFrame0, lidb, range,
        (stackFrame1, lidb) => {
          val (stackFrame3, condSE, condSelfUses, condChildUses) =
            expressionScout.scoutExpressionAndCoerce(
              stackFrame1,
              lidb,
              ConsecutorPE(
                Vector(
                  LetPE(
                    entryPatternPP.range,
                    PatternPP(inKeywordRange, Some(DestinationLocalP(IterationOptionNameDeclarationP(inKeywordRange), None)), None, None),
                    FunctionCallPE(
                      inKeywordRange,
                      inKeywordRange,
                      LookupPE(LookupNameP(NameP(inKeywordRange, keywords.next)), None),
                      Vector(
                        AugmentPE(
                          inKeywordRange,
                          BorrowP,
                          LookupPE(IteratorNameP(inKeywordRange), None))))),
                  FunctionCallPE(
                    inKeywordRange,
                    inKeywordRange,
                    LookupPE(LookupNameP(NameP(inKeywordRange, keywords.isEmpty)), None),
                    Vector(
                      AugmentPE(
                        inKeywordRange,
                        BorrowP,
                        LookupPE(IterationOptionNameP(inKeywordRange), None)))))),
              UseP)
          (stackFrame3, condSE, condSelfUses, condChildUses)
        },
        (stackFrame1, lidb) => {
          val (thenSE, thenUses, thenChildUses) =
            expressionScout.newBlock(
              stackFrame1.parentEnv, Some(stackFrame1), lidb.child(), PostParser.evalRange(stackFrame0.file, range),
              stackFrame1.contextRegion,
              noDeclarations,
              (stackFrame2, lidb) => {
                val (stackFrame3, lookupSE, lookupSelfUses, lookupChildUses) =
                  expressionScout.scoutExpressionAndCoerce(
                    stackFrame2,
                    lidb,
                    LookupPE(IterationOptionNameP(inKeywordRange), None),
                    UseP)
                val breakSE = postparsing.BreakSE(PostParser.evalRange(stackFrame3.file, range))
                val lookupAndBreakSE =
                  PostParser.consecutive(Vector(lookupSE, breakSE))
                (stackFrame3, lookupAndBreakSE, lookupSelfUses, lookupChildUses)
              })
          (stackFrame1, thenSE, thenUses, thenChildUses)
        },
        (stackFrame1, lidb) => {
          // Else does nothing
          val voidSE =
            postparsing.BlockSE(
              PostParser.evalRange(stackFrame1.file, range),
              Vector(),
              postparsing.VoidSE(PostParser.evalRange(stackFrame1.file, range)))
          (stackFrame1, voidSE, noVariableUses, noVariableUses)
        })

    val (stackFrame5, consumeSomeSE, consumeSomeSelfUses, consumeSomeChildUses) =
      expressionScout.scoutExpressionAndCoerce(
        stackFrame4, lidb.child(),
        LetPE(
          inKeywordRange,
          entryPatternPP,
          FunctionCallPE(
            inKeywordRange,
            inKeywordRange,
            LookupPE(LookupNameP(NameP(inKeywordRange, keywords.get)), None),
            Vector(
              LookupPE(IterationOptionNameP(inKeywordRange), None)))),
        UseP)

    val (userBodySE, userBodySelfUses, userBodyChildUses) =
      expressionScout.scoutBlock(stackFrame5, lidb.child(), noDeclarations, bodyPE)

    val selfUses = ifSelfUses.thenMerge(consumeSomeSelfUses).thenMerge(userBodySelfUses)
    val childUses = ifChildUses.thenMerge(consumeSomeChildUses).thenMerge(userBodyChildUses)
    val loopBodySE =
      PostParser.consecutive(Vector(ifSE, consumeSomeSE, userBodySE))

    (stackFrame5, loopBodySE, selfUses, childUses)
  }
*/
pub(crate) fn scout_while<'a, 'p, 'ctx, 's>(
  post_parser: &PostParser<'a, 'p, 'ctx, 's>,
  stack_frame0: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeL,
  condition_pe: &IExpressionPE<'a, 'p>,
  body: &BlockPE<'a, 'p>,
) -> Result<(&'s BlockSE<'a, 's>, VariableUses<'a>, VariableUses<'a>), ICompileErrorS<'a>>
where
  'a: 'ctx,
  'a: 'p,
  'a: 's,
{
  let while_range_s = PostParser::eval_range(stack_frame0.file, range);
  let parent_env0 = stack_frame0.parent_env.clone();
  let context_region0 = stack_frame0.context_region.clone();
  let (loop_s, loop_body_self_uses, loop_body_child_uses) = post_parser.new_block(
    parent_env0,
    Some(stack_frame0),
    &mut lidb.child(),
    while_range_s.clone(),
    context_region0,
    PostParser::no_declarations(),
    |stack_frame1, inner_lidb| {
      let parent_env1 = stack_frame1.parent_env.clone();
      let context_region1 = stack_frame1.context_region.clone();
      let (inner_loop_s, loop_body_self_uses, loop_body_child_uses) = post_parser.new_block(
        parent_env1,
        Some(stack_frame1.clone()),
        &mut inner_lidb.child(),
        while_range_s.clone(),
        context_region1,
        PostParser::no_declarations(),
        |stack_frame4, innermost_lidb| {
          let parent_env4 = stack_frame4.parent_env.clone();
          let context_region4 = stack_frame4.context_region.clone();
          let (loop_body_se, loop_body_self_uses, loop_body_child_uses) = post_parser.new_block(
            parent_env4,
            Some(stack_frame4.clone()),
            &mut innermost_lidb.child(),
            while_range_s.clone(),
            context_region4,
            PostParser::no_declarations(),
            |stack_frame5, body_lidb| {
              scout_while_body(
                post_parser,
                stack_frame5,
                body_lidb,
                range,
                condition_pe,
                body,
              )
            },
          )?;
          let while_se = &*post_parser.scout_arena.alloc(IExpressionSE::While(WhileSE {
            range: while_range_s.clone(),
            body: loop_body_se,
          }));
          Ok((stack_frame4, while_se, loop_body_self_uses, loop_body_child_uses))
        },
      )?;
      let inner_loop_expr =
        &*post_parser.scout_arena.alloc(IExpressionSE::Block(inner_loop_s));
      Ok((
        stack_frame1,
        inner_loop_expr,
        loop_body_self_uses,
        loop_body_child_uses,
      ))
    },
  )?;
  Ok((loop_s, loop_body_self_uses, loop_body_child_uses))
}
/*

  def scoutWhile(
    expressionScout: ExpressionScout,
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    conditionPE: IExpressionPE,
    body: BlockPE):
  (BlockSE, VariableUses, VariableUses) = {
    expressionScout.newBlock(
      stackFrame0.parentEnv, Some(stackFrame0), lidb.child(),
      PostParser.evalRange(stackFrame0.file, range),
      stackFrame0.contextRegion,
      noDeclarations,
      (stackFrame1, lidb) => {
        val (loopSE, loopBodySelfUses, loopBodyChildUses) =
          expressionScout.newBlock(
            stackFrame1.parentEnv, Some(stackFrame1), lidb.child(),
            PostParser.evalRange(stackFrame0.file, range),
            stackFrame1.contextRegion,
            noDeclarations,
            (stackFrame4, lidb) => {
              val (loopBodySE, loopBodySelfUses, loopBodyChildUses) =
                expressionScout.newBlock(
                  stackFrame4.parentEnv, Some(stackFrame4), lidb.child(),
                  PostParser.evalRange(stackFrame0.file, range),
                  stackFrame4.contextRegion,
                  noDeclarations,
                  (stackFrame5, lidb) => {
                    scoutWhileBody(expressionScout, stackFrame5, lidb, range, conditionPE, body)
                  })
              val whileSE = postparsing.WhileSE(PostParser.evalRange(stackFrame0.file, range), loopBodySE)
              (stackFrame4, whileSE, loopBodySelfUses, loopBodyChildUses)
            })
        (stackFrame1, loopSE, loopBodySelfUses, loopBodyChildUses)
      })
  }
*/
fn scout_while_body<'a, 'p, 'ctx, 's>(
  post_parser: &PostParser<'a, 'p, 'ctx, 's>,
  stack_frame0: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  range: RangeL,
  condition_pe: &IExpressionPE<'a, 'p>,
  body_pe: &BlockPE<'a, 'p>,
) -> Result<
  (
    StackFrame<'a>,
    &'s IExpressionSE<'a, 's>,
    VariableUses<'a>,
    VariableUses<'a>,
  ),
  ICompileErrorS<'a>,
>
where
  'a: 'ctx,
  'a: 'p,
  'a: 's,
{
  let while_range_s = PostParser::eval_range(stack_frame0.file, range);
  let (stack_frame4, if_se, if_self_uses, if_child_uses) = PostParser::new_if(
    stack_frame0,
    lidb,
    range,
    |stack_frame2, condition_lidb| {
      let (stack_frame3, cond_se, cond_self_uses, cond_child_uses) =
        post_parser.scout_expression_and_coerce(
          stack_frame2,
          condition_lidb,
          condition_pe,
          LoadAsP::Use,
        )?;
      Ok((stack_frame3, cond_se, cond_self_uses, cond_child_uses))
    },
    |stack_frame2, _then_lidb| {
      // Then does nothing, just continue on
      let void_s = &*post_parser.scout_arena.alloc(BlockSE {
        range: while_range_s.clone(),
        locals: Vec::new(),
        expr: &*post_parser.scout_arena.alloc(IExpressionSE::Void(VoidSE {
          range: while_range_s.clone(),
        })),
      });
      Ok((
        stack_frame2,
        void_s,
        PostParser::no_variable_uses(),
        PostParser::no_variable_uses(),
      ))
    },
    |stack_frame3, else_lidb| {
      let parent_env3 = stack_frame3.parent_env.clone();
      let context_region3 = stack_frame3.context_region.clone();
      let (then_s, then_uses, then_child_uses) = post_parser.new_block(
        parent_env3,
        Some(stack_frame3.clone()),
        else_lidb,
        while_range_s.clone(),
        context_region3,
        PostParser::no_declarations(),
        |stack_frame4, _break_lidb| {
          let break_s = &*post_parser.scout_arena.alloc(IExpressionSE::Break(BreakSE {
            range: while_range_s.clone(),
          }));
          Ok((stack_frame4, break_s, PostParser::no_variable_uses(), PostParser::no_variable_uses()))
        },
      )?;
      Ok((stack_frame3, then_s, then_uses, then_child_uses))
    },
  )?;
  let if_se = &*post_parser.scout_arena.alloc(IExpressionSE::If(if_se));

  let (user_body_se, user_body_self_uses, user_body_child_uses) = post_parser.scout_block(
    stack_frame4.clone(),
    &mut lidb.child(),
    PostParser::no_declarations(),
    body_pe,
  )?;

  let self_uses = if_self_uses.then_merge(&user_body_self_uses);
  let child_uses = if_child_uses.then_merge(&user_body_child_uses);
  let loop_body_se = post_parser.consecutive(vec![if_se, user_body_se]);
  Ok((stack_frame4, loop_body_se, self_uses, child_uses))
}
/*
  def scoutWhileBody(
    expressionScout: ExpressionScout,
    stackFrame0: StackFrame,
    lidb: LocationInDenizenBuilder,
    range: RangeL,
    conditionPE: IExpressionPE,
    bodyPE: BlockPE,
  ): (StackFrame, IExpressionSE, VariableUses, VariableUses) = {
    val (stackFrame4, ifSE, ifSelfUses, ifChildUses) =
      expressionScout.newIf(
        stackFrame0, lidb, range,
        (stackFrame2, lidb) => {
          val (stackFrame3, condSE, condSelfUses, condChildUses) =
            expressionScout.scoutExpressionAndCoerce(
              stackFrame2, lidb, conditionPE, UseP)
          (stackFrame3, condSE, condSelfUses, condChildUses)
        },
        (stackFrame2, lidb) => {
          // Then does nothing, just continue on
          val voidSE =
            postparsing.BlockSE(
              PostParser.evalRange(stackFrame2.file, range),
              Vector(),
              postparsing.VoidSE(PostParser.evalRange(stackFrame2.file, range)))
          (stackFrame2, voidSE, noVariableUses, noVariableUses)
        },
        (stackFrame3, _) => {
          val (thenSE, thenUses, thenChildUses) =
            expressionScout.newBlock(
              stackFrame3.parentEnv, Some(stackFrame3), lidb.child(), PostParser.evalRange(stackFrame0.file, range),
              stackFrame3.contextRegion,
              noDeclarations,
              (stackFrame4, lidb) => {
                val breakSE = postparsing.BreakSE(PostParser.evalRange(stackFrame4.file, range))
                (stackFrame4, breakSE, noVariableUses, noVariableUses)
              })
          (stackFrame3, thenSE, thenUses, thenChildUses)
        })

    val (userBodySE, userBodySelfUses, userBodyChildUses) =
      expressionScout.scoutBlock(stackFrame4, lidb.child(), noDeclarations, bodyPE)

    val selfUses = ifSelfUses.thenMerge(userBodySelfUses)
    val childUses = ifChildUses.thenMerge(userBodyChildUses)
    val loopBodySE =
      PostParser.consecutive(Vector(ifSE, userBodySE))

    (stackFrame4, loopBodySE, selfUses, childUses)
  }
*/
/*
}
*/