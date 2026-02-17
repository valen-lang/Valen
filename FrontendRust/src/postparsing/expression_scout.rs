use crate::parsing::ast::{
  IExpressionPE, IImpreciseNameP, INameDeclarationP, ITemplexPT, LoadAsP, OwnershipP,
};
use crate::interner::StrI;
use crate::postparsing::ast::LocationInDenizenBuilder;
use crate::postparsing::ast::IExpressionSE as IExpressionSETrait;
use crate::postparsing::expressions::{
  ConstantIntSE, DotSE, ExprMutateSE, FunctionCallSE, IExpressionSE, LetSE, LocalLoadSE,
  LocalMutateSE, OutsideLoadSE, OwnershippedSE, ReturnSE, RuneLookupSE, VoidSE,
};
use crate::postparsing::names::{CodeNameS, CodeRuneS, IImpreciseNameS, IImpreciseNameValS, IRuneValS, IVarNameS};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::post_parser::{CouldntFindVarToMutateS, ICompileErrorS, PostParser, StackFrame};
use crate::postparsing::variable_uses::{VariableDeclarationS, VariableDeclarations, VariableUses};
use crate::utils::range::RangeS;

/*
package dev.vale.postparsing

import dev.vale.postparsing.patterns.PatternScout
import dev.vale.postparsing.rules.{IRulexSR, IntLiteralSL, LiteralSR, MutabilityLiteralSL, RuleScout, RuneUsage, TemplexScout, VariabilityLiteralSL}
import dev.vale.parsing.ast._
import dev.vale.parsing.{ast, _}
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, postparsing, vassert, vassertSome, vcurious, vfail, vwat}
import PostParser.{evalRange, noDeclarations, noVariableUses}
import dev.vale
import dev.vale.lexing.RangeL
import dev.vale.parsing.ast
import dev.vale.parsing.ast.{AndPE, AugmentPE, BinaryCallPE, BlockPE, BorrowP, BraceCallPE, BreakPE, ConsecutorPE, ConstantBoolPE, ConstantFloatPE, ConstantIntPE, ConstantStrPE, ConstructArrayPE, DestructPE, DotPE, EachPE, FinalP, FunctionCallPE, FunctionP, IExpressionPE, ITemplexPT, IfPE, IndexPE, LambdaPE, LetPE, LoadAsBorrowP, LoadAsP, LoadAsWeakP, LookupNameP, LookupPE, MagicParamLookupPE, MethodCallPE, MoveP, MutableP, MutatePE, NameP, NotPE, OrPE, PackPE, RangePE, ReturnPE, RuntimeSizedP, ShortcallPE, StaticSizedP, StrInterpolatePE, SubExpressionPE, TemplateArgsP, TuplePE, UnletPE, UseP, VoidPE, WeakP, WhilePE}
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
  def scoutLambda(
    parentStackFrame: StackFrame,
    lambdaFunction0: FunctionP):
  (FunctionS, VariableUses)
}
*/
#[derive(Clone, Debug, PartialEq)]
enum IScoutResult<'a, 'p> {
  LocalLookupResult(LocalLookupResultS<'a>),
  OutsideLookupResult(OutsideLookupResultS<'a, 'p>),
  NormalResult(NormalResultS<'a>),
}
/*
// MIGALLOW: Rust IScoutResult doesn't need to be generic, because we never made use of that in
// Scala.
sealed trait IScoutResult[+T <: IExpressionSE]
*/
#[derive(Clone, Debug, PartialEq)]
struct LocalLookupResultS<'a> {
  range: RangeS<'a>,
  name: IVarNameS<'a>,
}
/*
// Will contain the address of a local.
case class LocalLookupResult(range: RangeS, name: IVarNameS) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
struct OutsideLookupResultS<'a, 'p> {
  range: RangeS<'a>,
  name: StrI<'a>,
  template_args: Option<&'p [ITemplexPT<'a, 'p>]>,
}
/*
// Looks up something that's not a local.
// Should be just a function, but its also super likely that the user just forgot
// to declare a variable, and we interpreted it as an outside lookup.
case class OutsideLookupResult(
  range: RangeS,
  name: StrI,
  templateArgs: Option[Vector[ITemplexPT]]
) extends IScoutResult[IExpressionSE] {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
struct NormalResultS<'a> {
  expr: IExpressionSE<'a>,
}

/*
// Anything else, such as:
// - Result of a function call
// - Address inside a struct
case class NormalResult[+T <: IExpressionSE](expr: T) extends IScoutResult[T] {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  def range: RangeS = expr.range
}
*/
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
/*
  def endsWithReturn(exprSE: IExpressionSE): Boolean = {
    exprSE match {
      case ReturnSE(_, _) => true
      case ConsecutorSE(exprs) => endsWithReturn(exprs.last)
      case _ => false
    }
  }
*/
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
              throw CompileErrorExceptionS(CouldntFindRuneS(rangeS, vassertSome(name).str.as_str())) // impl isolates
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
fn find_local<'a>(
  stack_frame: &StackFrame<'a>,
  range: RangeS<'a>,
  imprecise_name: &IImpreciseNameS<'a>,
) -> Option<LocalLookupResultS<'a>> {
  stack_frame
    .find_variable(imprecise_name)
    .map(|full_name| LocalLookupResultS::<'a> { range, name: full_name })
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
// MIGTODO: rename all "scout" to "post parse" or something.
impl<'a, 'ctx> PostParser<'a, 'ctx>
where
  'a: 'ctx,
{
fn scout_expression<'p>(
  &self,
  stack_frame: StackFrame<'a>,
  lidb: &mut LocationInDenizenBuilder,
  expression: &IExpressionPE<'a, 'p>,
) -> Result<(StackFrame<'a>, IScoutResult<'a, 'p>, VariableUses<'a>, VariableUses<'a>), ICompileErrorS<'a>>
where
  'a: 'p,
{
/*
  // Returns:
  // - new seq num
  // - declared variables
  // - new expression
  // - variable uses by self
  // - variable uses by child blocks
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
        expr: IExpressionSE::Void(VoidSE {
          range: PostParser::eval_range(file_coordinate, void.range),
        }),
      }),
      VariableUses::empty(),
      VariableUses::empty(),
    )),
    /*
    case VoidPE(range) => (stackFrame0, NormalResult(VoidSE(evalRange(range))), noVariableUses, noVariableUses)
    */
    IExpressionPE::Return(ret) => {
      let mut ret_expr_lidb = lidb.child();
      let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses) = self.scout_expression_and_coerce(
        stack_frame.clone(),
        &mut ret_expr_lidb,
        ret.expr.as_ref(),
        LoadAsP::Use,
      )?;
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: IExpressionSE::Return(ReturnSE {
            range: PostParser::eval_range(&file_coordinate, ret.range),
            inner: Box::new(inner_expr_s),
          }),
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
    IExpressionPE::Augment(augment) => {
      let load_as = match augment.target_ownership {
        OwnershipP::Borrow => LoadAsP::LoadAsBorrow,
        OwnershipP::Weak => LoadAsP::LoadAsWeak,
        OwnershipP::Own => panic!("POSTPARSER_AUGMENT_OWN_NOT_YET_IMPLEMENTED"),
        OwnershipP::Live => panic!("POSTPARSER_AUGMENT_LIVE_NOT_YET_IMPLEMENTED"),
        OwnershipP::Share => panic!("POSTPARSER_AUGMENT_SHARE_NOT_YET_IMPLEMENTED"),
      };
      let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses) = {
        let mut inner_lidb = lidb.child();
        let (stack_frame1, inner_expr_s, inner_self_uses, inner_child_uses) = self.scout_expression_and_coerce(
          stack_frame.clone(),
          &mut inner_lidb,
          augment.inner.as_ref(),
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
        IExpressionSE::OutsideLoad(outside_load) => {
          assert_eq!(outside_load.target_ownership, load_as);
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
        case OutsideLoadSE(_, _, _, _, innerLoadAs) => vassert(loadAs == innerLoadAs)
        case _ => vwat()
      }
      (stackFrame1, NormalResult(inner1), innerSelfUses, innerChildUses)
    }
    */
    IExpressionPE::Dot(dot) => {
      if let IExpressionPE::Lookup(lookup) = dot.left.as_ref() {
        if let IImpreciseNameP::LookupName(lookup_name) = &lookup.name {
          // Here, we're special casing lookups of this.x when we're in a constructor.
          // We know we're in a constructor if there's no `this` variable yet. After all,
          // in a constructor, `this` is just an imaginary concept until we actually
          // fill all the variables.
          if lookup_name.str().as_str() == "self"
            && stack_frame
              .find_variable(&self.interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
                name: lookup_name.str(),
              })))
              .is_none()
          {
            return Ok((
              stack_frame,
              IScoutResult::LocalLookupResult(LocalLookupResultS {
                range: PostParser::eval_range(&file_coordinate, lookup.name.range()),
                name: IVarNameS::ConstructingMemberName(dot.member.str()),
              }),
              VariableUses::empty(),
              VariableUses::empty(),
            ));
          }
        }
      }
      let (stack_frame1, container_expr_s, self_uses, child_uses) = {
        let mut dot_left_lidb = lidb.child();
        let (stack_frame1, container_expr_s, self_uses, child_uses) = self.scout_expression_and_coerce(
          stack_frame,
          &mut dot_left_lidb,
          dot.left.as_ref(),
          LoadAsP::LoadAsBorrow,
        )?;
        (stack_frame1, container_expr_s, self_uses, child_uses)
      };
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: IExpressionSE::Dot(DotSE {
            range: PostParser::eval_range(&file_coordinate, dot.range),
            left: Box::new(container_expr_s),
            member: dot.member.str(),
            borrow_container: true,
          }),
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
          val result = vale.postparsing.LocalLookupResult(evalRange(range), interner.intern(ConstructingMemberNameS(memberName)))
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
    IExpressionPE::Lookup(lookup) => {
      match (&lookup.name, &lookup.template_args) {
        (IImpreciseNameP::LookupName(lookup_name), None) => {
          let range = PostParser::eval_range(&file_coordinate, lookup.name.range());
          let imprecise_name = self.interner.intern_imprecise_name(IImpreciseNameValS::CodeName(
            CodeNameS { name: lookup_name.str() },
          ));
          if let Some(local_lookup_result) = find_local(&stack_frame, range.clone(), &imprecise_name) {
            return Ok((
              stack_frame,
              IScoutResult::LocalLookupResult(local_lookup_result),
              VariableUses::empty(),
              VariableUses::empty(),
            ));
          } else {
            let code_rune = self.interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
              name: lookup_name.str(),
            }));
            let lookup_result = if stack_frame.parent_env.all_declared_runes().contains(&code_rune) {
              IScoutResult::NormalResult(NormalResultS {
                expr: IExpressionSE::RuneLookup(RuneLookupSE { range, rune: code_rune }),
              })
            } else {
              IScoutResult::OutsideLookupResult(OutsideLookupResultS {
                range,
                name: lookup_name.str(),
                template_args: None,
              })
            };
            Ok((
              stack_frame,
              lookup_result,
              VariableUses::empty(),
              VariableUses::empty(),
            ))
          }
        }
        (IImpreciseNameP::LookupName(lookup_name), Some(template_args)) => Ok((
          stack_frame,
          IScoutResult::OutsideLookupResult(OutsideLookupResultS {
            range: PostParser::eval_range(&file_coordinate, lookup.name.range()),
            name: lookup_name.str(),
            template_args: Some(template_args.args),
          }),
          VariableUses::empty(),
          VariableUses::empty(),
        )),
        _ => panic!("POSTPARSER_SCOUT_NON_CODE_LOOKUP_NOT_YET_IMPLEMENTED"),
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
                  (OutsideLookupResult(rangeS, name, None))
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
        vale.postparsing.OutsideLookupResult(
          evalRange(range),
          templateName,
          Some(templateArgs.toVector))
      (stackFrame0, result, noVariableUses, noVariableUses)
    }
    */
    IExpressionPE::FunctionCall(function_call) => {
      let (stack_frame1, callable_expr_s, callable_self_uses, callable_child_uses) = {
        let mut callable_lidb = lidb.child();
        let (stack_frame1, callable_expr_s, callable_self_uses, callable_child_uses) = self.scout_expression_and_coerce(
          stack_frame,
          &mut callable_lidb,
          function_call.callable_expr.as_ref(),
          LoadAsP::LoadAsBorrow,
        )?;
        (stack_frame1, callable_expr_s, callable_self_uses, callable_child_uses)
      };
      let mut args_lidb = lidb.child();
      let (stack_frame2, arg_exprs_s, args_self_uses, args_child_uses) =
        self.scout_elements_as_expressions(stack_frame1, &mut args_lidb, &function_call.arg_exprs)?;
      let result =
        IScoutResult::NormalResult(NormalResultS {
          expr: IExpressionSE::FunctionCall(FunctionCallSE {
            range: PostParser::eval_range(&file_coordinate, function_call.range),
            location: lidb.child().consume(),
            callable_expr: Box::new(callable_expr_s),
            arg_exprs: arg_exprs_s,
          }),
        });
      Ok((
        stack_frame2,
        result,
        callable_self_uses.then_merge(&args_self_uses),
        callable_child_uses.then_merge(&args_child_uses),
      ))
    }
    /*
    case FunctionCallPE(range, _, callablePE, args) => {
      val loadCallableAs = LoadAsBorrowP
      val (stackFrame1, callable1, callableSelfUses, callableChildUses) =
        scoutExpressionAndCoerce(stackFrame0, lidb.child(), callablePE, loadCallableAs)
      val (stackFrame2, args1, argsSelfUses, argsChildUses) =
        scoutElementsAsExpressions(stackFrame1, lidb.child(), args)
      val result = NormalResult(vale.postparsing.FunctionCallSE(evalRange(range), lidb.child().consume(), callable1, args1.toVector))
      (stackFrame2, result, callableSelfUses.thenMerge(argsSelfUses), callableChildUses.thenMerge(argsChildUses))
    }
    */
    IExpressionPE::Let(lett) => {
      let destination = lett
        .pattern
        .destination
        .as_ref()
        .unwrap_or_else(|| panic!("POSTPARSER_SCOUT_LET_DESTINATION_NOT_YET_IMPLEMENTED"));
      assert!(
        destination.mutate.is_none(),
        "POSTPARSER_SCOUT_LET_MUTATING_DESTINATION_NOT_YET_IMPLEMENTED"
      );
      assert!(
        lett.pattern.templex.is_none(),
        "POSTPARSER_SCOUT_LET_TEMPLEX_NOT_YET_IMPLEMENTED"
      );
      assert!(
        lett.pattern.destructure.is_none(),
        "POSTPARSER_SCOUT_LET_DESTRUCTURE_NOT_YET_IMPLEMENTED"
      );
      let declared_name = match &destination.decl {
        INameDeclarationP::LocalNameDeclaration(local_name) => IVarNameS::CodeVarName(local_name.str()),
        // WARNING: This is inaccurate and doesn't match scala, and we need to change this to call out to translatePattern ASAP!
        INameDeclarationP::ConstructingMemberNameDeclaration(member_name) => {
          IVarNameS::ConstructingMemberName(member_name.str())
        }
        _ => panic!("POSTPARSER_SCOUT_LET_DECL_NOT_YET_IMPLEMENTED"),
      };
      let (stack_frame1, source_expr_s, source_self_uses, source_child_uses) = {
        let mut source_expr_lidb = lidb.child();
        let (stack_frame1, source_expr_s, source_self_uses, source_child_uses) = self.scout_expression_and_coerce(
          stack_frame,
          &mut source_expr_lidb,
          lett.source.as_ref(),
          LoadAsP::Use,
        )?;
        (stack_frame1, source_expr_s, source_self_uses, source_child_uses)
      };
      let name_already_exists = stack_frame1.locals.vars.iter().any(|decl| decl.name == declared_name);
      let declarations_from_pattern = if name_already_exists {
        panic!("POSTPARSER_SCOUT_LET_VARIABLE_NAME_ALREADY_EXISTS_NOT_YET_IMPLEMENTED")
      } else {
        VariableDeclarations {
          vars: vec![VariableDeclarationS {
            name: declared_name.clone(),
          }],
        }
      };
      Ok((
        stack_frame1.plus(&declarations_from_pattern),
        IScoutResult::NormalResult(NormalResultS {
          expr: IExpressionSE::Let(LetSE {
            range: PostParser::eval_range(&file_coordinate, lett.range),
            rules: Vec::new(),
            pattern: AtomSP {
              range: PostParser::eval_range(&file_coordinate, lett.range),
              name: Some(CaptureS {
                name: declared_name,
                mutate: false,
              }),
              coord_rune: None,
              destructure: None,
            },
            expr: Box::new(source_expr_s),
          }),
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

      val declarationsFromPattern = vale.postparsing.VariableDeclarations(patternScout.getParameterCaptures(patternS))

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
      let (stack_frame1, source_expr_s, source_inner_self_uses, source_child_uses) = {
        let mut source_expr_lidb = lidb.child();
        // MIGTODO: consider doing &mut StackFrame instead of clone, everywhere.
        self.scout_expression_and_coerce(
          stack_frame,
          &mut source_expr_lidb,
          mutate.source.as_ref(),
          LoadAsP::Use,
        )?
      };
      let (stack_frame2, destination_result_s, destination_self_uses, destination_child_uses) = {
        let mut destination_expr_lidb = lidb.child();
        self.scout_expression(
          stack_frame1,
          &mut destination_expr_lidb,
          mutate.mutatee.as_ref(),
        )?
      };
      let (mutate_expr_s, source_self_uses) = match destination_result_s {
        IScoutResult::LocalLookupResult(LocalLookupResultS { range, name }) => {
          (
            IExpressionSE::LocalMutate(LocalMutateSE {
              range,
              name: name.clone(),
              expr: Box::new(source_expr_s),
            }),
            source_inner_self_uses.mark_mutated(name),
          )
        }
        IScoutResult::OutsideLookupResult(OutsideLookupResultS { range, name, .. }) => {
          return Err(ICompileErrorS::CouldntFindVarToMutateS(CouldntFindVarToMutateS {
            range,
            name: name.as_str().to_string(),
          }));
        }
        IScoutResult::NormalResult(NormalResultS { expr: destination_expr_s }) => {
          (
            IExpressionSE::ExprMutate(ExprMutateSE {
              range: destination_expr_s.range(),
              mutatee: Box::new(destination_expr_s),
              expr: Box::new(source_expr_s),
            }),
            source_inner_self_uses,
          )
        }
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
    IExpressionPE::ConstantInt(constant_int) => Ok((
      stack_frame,
      IScoutResult::NormalResult(NormalResultS {
        expr: IExpressionSE::ConstantInt(ConstantIntSE {
          range: PostParser::eval_range(&file_coordinate, constant_int.range),
          value: constant_int.value,
          bits: constant_int.bits.unwrap_or(32) as i32,
        }),
      }),
      VariableUses::empty(),
      VariableUses::empty(),
    )),
    /*
    case ConstantIntPE(range, value, bitsP) => {
      val bits = bitsP.getOrElse(32L).toInt
      (stackFrame0, NormalResult(ConstantIntSE(evalRange(range), value, bits)), noVariableUses, noVariableUses)
    }
    */
    IExpressionPE::Consecutor(consecutor) => {
      let (stack_frame1, unfiltered_exprs, self_uses, child_uses) =
        self.scout_elements_as_expressions(stack_frame, lidb, &consecutor.inners)?;

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
            panic!("POSTPARSER_SCOUT_STATEMENT_AFTER_RETURN_NOT_YET_IMPLEMENTED");
          }
        }
      }
      assert!(
        !filtered_exprs.is_empty(),
        "POSTPARSER_SCOUT_CONSECUTOR_EMPTY_AFTER_VOID_STRIP"
      );
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS {
          expr: PostParser::consecutive(filtered_exprs),
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
    IExpressionPE::Block(block) => {
      assert!(
        block.maybe_default_region.is_none(),
        "POSTPARSER_SCOUT_BLOCK_DEFAULT_REGION_NOT_YET_IMPLEMENTED"
      );
      assert!(
        block.maybe_pure.is_none(),
        "POSTPARSER_SCOUT_BLOCK_PURE_NOT_YET_IMPLEMENTED"
      );
      self.scout_expression(
        stack_frame,
        lidb,
        block.inner.as_ref(),
      )
    }
    /*
    case b @ BlockPE(_, _, maybeNewDefaultRegion, _) => {
      vassert(maybeNewDefaultRegion.isEmpty)
      val (resultSE, selfUses, childUses) =
        scoutBlock(stackFrame0, lidb.child(), noDeclarations, b)
      (stackFrame0, NormalResult(resultSE), selfUses, childUses)
    }
    */
    IExpressionPE::SubExpression(sub_expression) => {
      let mut sub_expression_lidb = lidb.child();
      let (stack_frame1, sub_expression_s, sub_self_uses, sub_child_uses) = self.scout_expression_and_coerce(
        stack_frame.clone(),
        &mut sub_expression_lidb,
        sub_expression.inner.as_ref(),
        LoadAsP::Use,
      )?;
      Ok((
        stack_frame1,
        IScoutResult::NormalResult(NormalResultS { expr: sub_expression_s }),
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
                val callableExpr =
                  vale.postparsing.OutsideLoadSE(addCallRange, Vector(), interner.intern(CodeNameS(keywords.plus)), None, LoadAsBorrowP)
                FunctionCallSE(addCallRange, lidb.child().consume(), callableExpr, Vector(prevExpr, partSE))
              }
            })
          (stackFrame1, NormalResult(addedExpr), partsSelfUses, partsChildUses)
        }
        */
        /*
        case BreakPE(range) => {
          (stackFrame0, NormalResult(BreakSE(evalRange(range))), noVariableUses, noVariableUses)
        }
        */
        /*
        case NotPE(range, innerPE) => {
          val callableSE = vale.postparsing.OutsideLoadSE(evalRange(range), Vector(), interner.intern(CodeNameS(keywords.not)), None, LoadAsBorrowP)

          val (stackFrame1, innerSE, innerSelfUses, innerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), innerPE, UseP)

          val result =
            NormalResult(
              vale.postparsing.FunctionCallSE(evalRange(range), lidb.child().consume(), callableSE, Vector(innerSE)))

          (stackFrame1, result, innerSelfUses, innerChildUses)
        }
        */
        /*
        case RangePE(range, beginPE, endPE) => {
          val callableSE = vale.postparsing.OutsideLoadSE(evalRange(range), Vector(), interner.intern(CodeNameS(keywords.range)), None, LoadAsBorrowP)

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
              vale.postparsing.FunctionCallSE(evalRange(range), lidb.child().consume(), callableSE, Vector(beginSE, endSE))

          (stackFrame2, NormalResult(resultSE), beginSelfUses.thenMerge(endSelfUses), beginChildUses.thenMerge(endChildUses))
        }
        */
        /*
        case ConstantBoolPE(range,value) => (stackFrame0, NormalResult(vale.postparsing.ConstantBoolSE(evalRange(range), value)), noVariableUses, noVariableUses)
        */
        /*
        case ConstantStrPE(range, value) => {
          (stackFrame0, NormalResult(vale.postparsing.ConstantStrSE(evalRange(range), value)), noVariableUses, noVariableUses)
        }
        case ConstantFloatPE(range,value) => (stackFrame0, NormalResult(ConstantFloatSE(evalRange(range), value)), noVariableUses, noVariableUses)
        */
        /*
        case MagicParamLookupPE(range) => {
          val name = interner.intern(MagicParamNameS(PostParser.evalPos(stackFrame0.file, range.begin)))
          val lookup = vale.postparsing.LocalLookupResult(evalRange(range), name)
          // We dont declare it here, because then scoutBlock will think its a local and
          // hide it from those above.
          //   val declarations = VariableDeclarations(Vector(VariableDeclaration(lookup.name, FinalP)))
          // Leave it to scoutLambda to declare it.
          (stackFrame0, lookup, noVariableUses.markMoved(name), noVariableUses)
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
        /*
        case BinaryCallPE(range, namePE, leftPE, rightPE) => {
          val callableSE = vale.postparsing.OutsideLoadSE(evalRange(range), Vector(), interner.intern(CodeNameS(namePE.str)), None, LoadAsBorrowP)

          val (stackFrame1, leftSE, leftSelfUses, leftChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), leftPE, LoadAsBorrowP)
          val (stackFrame2, rightSE, rightSelfUses, rightChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), rightPE, LoadAsBorrowP)

          val result =
            NormalResult(
              vale.postparsing.FunctionCallSE(evalRange(range), lidb.child().consume(), callableSE, Vector(leftSE, rightSE)))
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

          val resultSE = vale.postparsing.IndexSE(evalRange(range), callableSE, argSE)
          (stackFrame2, NormalResult(resultSE), callableSelfUses.thenMerge(argSelfUses), callableChildUses.thenMerge(argChildUses))
        }
        */
        /*
        case MethodCallPE(range, subjectExpr, operatorRange, memberLookup, methodArgs) => {
          val loadSubjectAs =
            subjectExpr match {
              // For locals, just borrow.
              case LookupPE(_, _) => LoadAsBorrowP
              // For anything else, default to moving.
              case _ => UseP
            }
          val (stackFrame1, callable1, callableSelfUses, callableChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), memberLookup, LoadAsBorrowP)
          val (stackFrame2, subject1, subjectSelfUses, subjectChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), subjectExpr, loadSubjectAs)
          val (stackFrame3, tailArgs1, tailArgsSelfUses, tailArgsChildUses) =
            scoutElementsAsExpressions(stackFrame2, lidb.child(), methodArgs)

          val selfUses = callableSelfUses.thenMerge(subjectSelfUses).thenMerge(tailArgsSelfUses)
          val childUses = callableChildUses.thenMerge(subjectChildUses).thenMerge(tailArgsChildUses)
          val args = Vector(subject1) ++ tailArgs1

          val result = NormalResult(vale.postparsing.FunctionCallSE(evalRange(range), lidb.child().consume(), callable1, args))
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

                val ifSE = vale.postparsing.IfSE(evalRange(range), condSE, thenSE, elseSE)
                (stackFrame2, ifSE, selfUses, childUses)
              })
          (stackFrame0, NormalResult(resultSE), selfUses, childUses)
        }
        */
        /*
        case WhilePE(range, conditionPE, uncombinedBodyPE) => {
          val (loopSE, loopSelfUses, loopChildUses) =
            loopPostParser.scoutWhile(
              this, stackFrame0, lidb, range, conditionPE, uncombinedBodyPE)

          (stackFrame0, NormalResult(loopSE), loopSelfUses, loopChildUses)
        }
        */
        /*
        case EachPE(range, maybePure, entryPatternPP, inKeywordRange, iterableExpr, body) => {
          val (loopSE, selfUses, childUses) =
            loopPostParser.scoutEach(this, stackFrame0, lidb, range, maybePure.nonEmpty, entryPatternPP, inKeywordRange, iterableExpr, body)
          (stackFrame0, NormalResult(loopSE), selfUses, childUses)
        }
        */
        /*
        case IndexPE(range, containerExprPE, Vector(indexExprPE)) => {
          val (stackFrame1, containerExpr1, containerSelfUses, containerChildUses) =
            scoutExpressionAndCoerce(stackFrame0, lidb.child(), containerExprPE, LoadAsBorrowP)
          val (stackFrame2, indexExpr1, indexSelfUses, indexChildUses) =
            scoutExpressionAndCoerce(stackFrame1, lidb.child(), indexExprPE, UseP);
          val dot1 = vale.postparsing.IndexSE(evalRange(range), containerExpr1, indexExpr1)
          (stackFrame2, NormalResult(dot1), containerSelfUses.thenMerge(indexSelfUses), containerChildUses.thenMerge(indexChildUses))
        }
        */
        /*
        case ShortcallPE(range, argExprs) => {
          throw CompileErrorExceptionS(UnimplementedExpression(evalRange(range), "shortcalling"));
        }
        */
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
      vale.postparsing.IfSE(PostParser.evalRange(stackFrame0.file, range), condSE, thenSE, elseSE)
    (stackFrame3, ifSE, selfUses, childUses)
  }
*/
pub(crate) fn scout_expression_and_coerce<'p>(
    &self,
    stack_frame: StackFrame<'a>,
    lidb: &mut LocationInDenizenBuilder,
    expression_p: &IExpressionPE<'a, 'p>,
    load_as_p: LoadAsP,
  ) -> Result<(StackFrame<'a>, IExpressionSE<'a>, VariableUses<'a>, VariableUses<'a>), ICompileErrorS<'a>>
  where
    'a: 'p,
  {
    let mut expression_lidb = lidb.child();
    let (next_stack_frame, first_result_s, first_inner_self_uses, first_child_uses) = self.scout_expression(
      stack_frame,
      &mut expression_lidb,
      expression_p,
    )?;
    let (first_expr_s, first_self_uses) = match first_result_s {
      IScoutResult::LocalLookupResult(LocalLookupResultS { range, name }) => {
        let uses = match load_as_p {
          LoadAsP::LoadAsBorrow => first_inner_self_uses.mark_borrowed(name.clone()),
          LoadAsP::LoadAsWeak => first_inner_self_uses.mark_borrowed(name.clone()),
          LoadAsP::Use | LoadAsP::Move => first_inner_self_uses.mark_moved(name.clone()),
        };
        (IExpressionSE::LocalLoad(LocalLoadSE { range, name, target_ownership: load_as_p }), uses)
      }
      IScoutResult::OutsideLookupResult(OutsideLookupResultS { range, name, template_args }) => {
        assert!(
          template_args.is_none(),
          "POSTPARSER_SCOUT_LOOKUP_TEMPLATE_ARGS_NOT_YET_IMPLEMENTED"
        );
        (
          IExpressionSE::OutsideLoad(OutsideLoadSE {
            range,
            rules: Vec::new(),
            name: self.interner.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
              name,
            })),
            maybe_template_args: None,
            target_ownership: load_as_p,
          }),
          first_inner_self_uses,
        )
      }
      IScoutResult::NormalResult(NormalResultS { expr: inner_expr_s }) => {
        if load_as_p == LoadAsP::Use {
          (inner_expr_s, first_inner_self_uses)
        } else {
          (
            IExpressionSE::Ownershipped(OwnershippedSE {
              range: inner_expr_s.range(),
              inner_expr: Box::new(inner_expr_s),
              target_ownership: load_as_p,
            }),
            first_inner_self_uses,
          )
        }
      }
    };
    Ok((next_stack_frame, first_expr_s, first_self_uses, first_child_uses))
  }

/*
  // If we load an immutable with targetOwnershipIfLookupResult = Own or Borrow, it will just be Share.
  def scoutExpressionAndCoerce(
    stackFramePE: StackFrame,
    lidb: LocationInDenizenBuilder,
    exprPE: IExpressionPE,
    loadAsP: LoadAsP):
  (StackFrame, IExpressionSE, VariableUses, VariableUses) = {
    val (namesFromInsideFirst, firstResult1, firstInnerSelfUses, firstChildUses) =
      scoutExpression(stackFramePE, lidb.child(), exprPE);
    val (firstExpr1, firstSelfUses) =
      firstResult1 match {
        case LocalLookupResult(range, name) => {
          val uses =
            loadAsP match {
              case LoadAsBorrowP => firstInnerSelfUses.markBorrowed(name)
              case LoadAsWeakP => firstInnerSelfUses.markBorrowed(name)
              case UseP | MoveP => firstInnerSelfUses.markMoved(name)
            }
          (vale.postparsing.LocalLoadSE(range, name, loadAsP), uses)
        }
        case OutsideLookupResult(range, name, maybeTemplateArgs) => {
          val ruleBuilder = ArrayBuffer[IRulexSR]()
          val maybeTemplateArgRunes =
            maybeTemplateArgs.map(templateArgs => {
              templateArgs.map(templateArgPT => {
                templexScout.translateTemplex(
                  stackFramePE.parentEnv, lidb.child(), ruleBuilder, stackFramePE.contextRegion, templateArgPT)
              })
            })
          val load = vale.postparsing.OutsideLoadSE(range, ruleBuilder.toVector, interner.intern(CodeNameS(name)), maybeTemplateArgRunes, loadAsP)
          (load, firstInnerSelfUses)
        }
        case NormalResult(innerExpr1) => {
          loadAsP match {
            case UseP => (innerExpr1, firstInnerSelfUses)
            case _ => (vale.postparsing.OwnershippedSE(innerExpr1.range, innerExpr1, loadAsP), firstInnerSelfUses)
          }
        }
      }
    (namesFromInsideFirst, firstExpr1, firstSelfUses, firstChildUses)
  }
*/
fn scout_elements_as_expressions<'p>(
    &self,
    initial_stack_frame: StackFrame<'a>,
    lidb: &mut LocationInDenizenBuilder,
    exprs_p: &[IExpressionPE<'a, 'p>],
  ) -> Result<(StackFrame<'a>, Vec<IExpressionSE<'a>>, VariableUses<'a>, VariableUses<'a>), ICompileErrorS<'a>>
  where
    'a: 'p,
  {
    let mut self_uses = VariableUses::empty();
    let mut child_uses = VariableUses::empty();
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
/*
  def flattenExpressions(expr: IExpressionSE): Vector[IExpressionSE] = {
    expr match {
      case ConsecutorSE(exprs) => exprs.flatMap(flattenExpressions)
      case other => Vector(other)
    }
  }
}
*/