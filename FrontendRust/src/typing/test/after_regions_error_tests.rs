// mig: struct AfterRegionsErrorTests
pub struct AfterRegionsErrorTests {}
/*
package dev.vale.typing

import dev.vale.solver.{FailedSolve, RuleError}
import dev.vale.typing.OverloadResolver.{FindFunctionResolveFailure, InferFailure, SpecificParamDoesntSend}
import dev.vale.typing.ResolvingSolveFailedOrIncomplete
import dev.vale.typing.ast._
import dev.vale.typing.infer.{BadIsaSubKind, SendingNonCitizen}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.citizen._
import dev.vale.typing.expression._
import dev.vale.{Collector, Err, Ok, vwat, _}
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

class AfterRegionsErrorTests extends FunSuite with Matchers {
*/
// mig: fn prints_bread_crumb_trail
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn prints_bread_crumb_trail() { panic!("Unmigrated test: prints_bread_crumb_trail"); }
/*
  ignore("Prints bread crumb trail") {
    val compile = CompilerTestCompilation.test(
      """
        |import printutils.*;
        |import v.builtins.panicutils.*;
        |
        |#!DeriveInterfaceDrop
        |sealed interface Opt<T> where T Ref { }
        |#!DeriveStructDrop
        |struct Some<T> where T Ref { value T; }
        |
        |impl<T> Opt<T> for Some<T>;
        |#!DeriveStructDrop
        |struct None<T> where T Ref { }
        |
        |impl<T> Opt<T> for None<T>;
        |
        |abstract func drop<T>(virtual opt Opt<T>)
        |where func drop(T)void;
        |
        |func drop<T>(opt Some<T>)
        |where func drop(T)void
        |{
        |  [x] = opt;
        |}
        |
        |func drop<T>(opt None<T>) {
        |  [ ] = opt;
        |}
        |
        |abstract func isEmpty<T>(virtual opt &Opt<T>) bool;
        |func isEmpty<T>(opt &None<T>) bool { return true; }
        |func isEmpty<T>(opt &Some<T>) bool { return false; }
        |
        |abstract func isEmpty<T>(virtual opt Opt<T>) bool;
        |func isEmpty<T>(opt None<T>) bool { return true; }
        |func isEmpty<T>(opt Some<T>) bool
        |where func drop(T)void
        |{ return false; }
        |
        |abstract func get<T>(virtual opt Opt<T>) T;
        |func get<T>(opt None<T>) T { panic("Called get() on a None!"); }
        |func get<T>(opt Some<T>) T {
        |  [value] = opt;
        |  return value;
        |}
        |
        |abstract func get<T>(virtual opt &Opt<T>) &T;
        |func get<T>(opt &None<T>) &T { panic("Called get() on a None!"); }
        |func get<T>(opt &Some<T>) &T { return &opt.value; }
        |
        |
        |#!DeriveStructDrop
        |struct MyList<T Ref> {
        |  value T;
        |  next Opt<MyList<T>>;
        |}
        |
        |func drop<T>(this MyList<T>)
        |where func drop(T)void {
        |  [value, next] = this;
        |}
        |
        |func printValues(list &MyList<int>) void {
        |  print(list.value);
        |  printNextValue(list.next);
        |}
        |
        |func printNextValue(virtual opt &Opt<MyList<int>>) void { }
        |func printNextValue(opt &None<MyList<int>>) void { }
        |func printNextValue(opt &Some<MyList<int>>) void {
        |  printValues(opt.value);
        |}
        |
        |
        |exported func main() int {
        |  list = MyList<int>(10, Some<MyList<int>>(MyList<int>(20, Some<MyList<int>>(MyList<int>(30, None<MyList<int>>())))));
        |  printValues(&list);
        |  return 0;
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    // Ensure it properly prints out that the original error is from isEmpty
    // Also prune it down a bit
    vimpl()
  }
*/
// mig: fn reports_error
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn reports_error() { panic!("Unmigrated test: reports_error"); }
/*
  // https://github.com/ValeLang/Vale/issues/548
  // Real bug: impl-ing a mut interface with an imm struct is silently accepted, then
  // explodes mid-override-search with BadIsaSuperKind(B). Fix attempt was an impl-time
  // mutability check in ImplCompiler — but it broke IFunction1.anonymous (interface and
  // anonymous-substruct both carry placeholder M's whose IdTs differ but are conceptually
  // the same). Punted; needs a substitution-based comparison. See investigations/reports_error_1_3.md.
  ignore("Reports error") {
    val compile = CompilerTestCompilation.test(
      """
        |interface A {
        |	func foo(virtual a &A) int;
        |}
        |
        |struct B imm {
  val int; }
        |impl A for B;
        |
        |func foo(b &B) int { return b.val; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    vimpl()
  }
*/
// mig: fn reports_error_imm_interface_imm_struct
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn reports_error_imm_interface_imm_struct() { panic!("Unmigrated test: reports_error_imm_interface_imm_struct"); }
/*
  ignore("Reports error (imm interface + imm struct)") {
    // Fails with "Immutable struct ("A.anonymous") cannot have mutable member",
    // because the anonymous substruct made from interface A contains a mutable
    // thing. We'll want to fix that by making those contained things immutable
    // when the interface is immutable.
    val compile = CompilerTestCompilation.test(
      """
        |interface A imm {
        |	func foo(virtual a &A) int;
        |}
        |
        |struct B imm { val int; }
        |impl A for B;
        |
        |func foo(b &B) int { return b.val; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    vimpl()
  }

  // right now there is no collision because they have different template names.
  // The old declaredSignatures mechanism (SignatureT -> RangeS map in CompilerOutputs)
  // was commented out. The replacement functionDeclaredNames uses IdT which includes
  // FunctionTemplateNameT.codeLocation, so two functions at different source locations
  // are treated as different. Need to restore signature-level duplicate detection.
//  test("Reports when two functions with same signature") {
//    val compile = CompilerTestCompilation.test(
//      """
//        |exported func moo() int { return 1337; }
//        |exported func moo() int { return 1448; }
//        |""".stripMargin)
//    compile.getCompilerOutputs() match {
//      case Err(FunctionAlreadyExists(_, _, IdT(_, Vector(), null))) =>
////      case Err(FunctionAlreadyExists(_, _, FullNameT(_, Vector(), FunctionTemplateNameT(StrI("moo"), _)))) =>
//    }
//  }
*/
// mig: fn report_when_downcasting_between_unrelated_types
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn report_when_downcasting_between_unrelated_types() { panic!("Unmigrated test: report_when_downcasting_between_unrelated_types"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Report when downcasting between unrelated types") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.as.*;
        |import panicutils.*;
        |
        |interface ISpaceship { }
        |struct Spoon { }
        |
        |exported func main() {
        |  ship = __pretend<ISpaceship>();
        |  ship.as<Spoon>();
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantDowncastUnrelatedTypes(_, _, _, _)) =>
    }
  }
*/
// mig: fn lambda_body_type_mismatches_anonymous_interface_return_type
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn lambda_body_type_mismatches_anonymous_interface_return_type() { panic!("Unmigrated test: lambda_body_type_mismatches_anonymous_interface_return_type"); }
/*
  test("Lambda body type mismatches anonymous interface return type") {
    val compile = CompilerTestCompilation.test(
      """
        |interface AFunction1<P Ref> {
        |  func __call(virtual this &AFunction1<P>, a P) int;
        |}
        |exported func main() {
        |  arr = AFunction1<int>((_) => { true });
        |}
        |""".stripMargin)

    // The compiler rejects this not via a body-vs-return-type comparison on the
    // synthesized forwarder, but earlier: the substruct constructor's __call bound
    // (emitted by AnonymousInterfaceMacro) checks the lambda's __call return type
    // during inference and reports a ReturnTypeConflictInConclusionResolve. See
    // investigations/family1_4_body_result_doesnt_match_unreachable.md.
    compile.getCompilerOutputs() match {
      case Err(CouldntFindFunctionToCallT(_, fff)) => {
        val rejectionReasons = fff.rejectedCalleeToReason.map(_._2).toVector
        rejectionReasons match {
          case Vector(FindFunctionResolveFailure(
              ResolvingResolveConclusionError(
                ReturnTypeConflictInConclusionResolve(
                  _,
                  CoordT(ShareT, _, IntT(_)),
                  actualPrototype)))) => {
            actualPrototype.returnType match {
              case CoordT(ShareT, _, BoolT()) =>
              case other => vwat(other)
            }
          }
          case other => vwat(other)
        }
      }
      case Err(other) => {
        val codeMap = compile.getCodeMap().getOrDie()
        vwat(
          CompilerErrorHumanizer.humanize(
          true,
          SourceCodeUtils.humanizePos(codeMap, _),
          SourceCodeUtils.linesBetween(codeMap, _, _),
          SourceCodeUtils.lineRangeContaining(codeMap, _),
          SourceCodeUtils.lineContaining(codeMap, _),
          other))
      }
      case Ok(wat) => vwat(wat)
    }
  }
*/
// mig: fn detects_sending_non_citizen_to_citizen
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn detects_sending_non_citizen_to_citizen() { panic!("Unmigrated test: detects_sending_non_citizen_to_citizen"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Detects sending non-citizen to citizen") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |interface MyInterface {}
        |func moo<T>(a T)
        |where implements(T, MyInterface), func drop(T)void
        |{ }
        |exported func main() {
        |  moo(7);
        |}
        |""".stripMargin
    )
    compile.getCompilerOutputs() match {
      case Err(CouldntFindFunctionToCallT(range, fff)) => {
        fff.rejectedCalleeToReason.map(_._2).head match {
          case FindFunctionResolveFailure(ResolvingSolveFailedOrIncomplete(FailedSolve(_, _, _, _, RuleError(BadIsaSubKind(IntT(32)))))) =>
          case InferFailure(FailedSolve(_, _, _, _, RuleError(SendingNonCitizen(IntT(32))))) =>
          case other => vfail(other)
        }
      }
    }
  }
*/
// mig: fn accidentally_mention_type_rune
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn accidentally_mention_type_rune() { panic!("Unmigrated test: accidentally_mention_type_rune"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Accidentally mention type rune") {
    val compile = CompilerTestCompilation.test(
      """
        |func moo<Z>(z &Z) {
        |  drop(Z);
        |}
        |
        |exported func main() void {
        |  moo(4);
        |}
""".stripMargin)

    compile.getCompilerOutputs() match {
      case Err(CantUseRuneValueAsExpression(_, _)) =>
      case Err(e) => vfail(e)
      case Ok(_) => vfail()
    }
  }
*/
// mig: fn call_bound_with_wrong_arguments
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn call_bound_with_wrong_arguments() { panic!("Unmigrated test: call_bound_with_wrong_arguments"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Call bound with wrong arguments") {
    val compile = CompilerTestCompilation.test(
      """
        |func add<X>(i int, x &X) where func str(&X)str {
        |  str(true);
        |}
        |
  """.stripMargin)

    compile.getCompilerOutputs() match {
      case Err(CouldntFindFunctionToCallT(_, fff)) => {
        vassert(fff.rejectedCalleeToReason.size >= 1)
        fff.rejectedCalleeToReason.head._2 match {
          case SpecificParamDoesntSend(0, CoordT(ShareT, _, BoolT()), _) =>
          case other => vfail(other)
        }
      }
      case Err(e) => vfail(e)
      case Ok(_) => vfail()
    }
  }
*/
// mig: fn inherit_reachable_bounds_for_params_and_things_inside_params_too_irbfptipt
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn inherit_reachable_bounds_for_params_and_things_inside_params_too_irbfptipt() { panic!("Unmigrated test: inherit_reachable_bounds_for_params_and_things_inside_params_too_irbfptipt"); }
/*
  ignore("Inherit reachable bounds for params and things inside params too (IRBFPTIPT)") {
    val compile = CompilerTestCompilation.test(
      """
        |struct BoxA<T> where func drop(T)void { x T; }
        |struct BoxB<T> where func drop(T)void { x T; }
        |
        |sealed interface IBoxA<T> where func drop(T)void { }
        |impl<T> IBoxA<BoxB<T>> for BoxA<BoxB<T>>;
        |
        |abstract func bork<T>(virtual self IBoxA<BoxB<T>>);
        |func bork<T>(self BoxA<BoxB<T>>) { // should inherit drop(T) from BoxB<T>
        |  [b] = self;
        |  drop(b);
        |}
        |
""".stripMargin)

    compile.getCompilerOutputs() match {
      case Err(e) => vimpl(e)
      case Ok(_) => vfail()
    }
  }
*/
// mig: fn ambiguous_call
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn ambiguous_call() { panic!("Unmigrated test: ambiguous_call"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Ambiguous call") {
    val compile = CompilerTestCompilation.test(
      """
        |func add<X>(i int, x &X) { }
        |func add<X>(x &X, i int) { }
        |
        |exported func main() void {
        |  add(3, 4);
        |}
""".stripMargin)

    compile.getCompilerOutputs() match {
      case Err(CouldntNarrowDownCandidates(_, candidates)) => {
        vassert(candidates.size == 2)
      }
      case Err(e) => vfail(e)
      case Ok(_) => vfail()
    }
  }
*/
// mig: fn cant_make_non_weakable_extend_a_weakable
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn cant_make_non_weakable_extend_a_weakable() { panic!("Unmigrated test: cant_make_non_weakable_extend_a_weakable"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Cant make non-weakable extend a weakable") {
    val compile = CompilerTestCompilation.test(
      """
        |weakable interface IUnit {}
        |struct Muta { hp int; }
        |impl IUnit for Muta;
        |func main(muta Muta) int  { return 7; }
        |""".stripMargin)

    try {
      compile.expectCompilerOutputs().lookupFunction("main")
      vfail()
    } catch {
      case WeakableImplingMismatch(false, true) =>
      case other => {
        other.printStackTrace()
        vfail()
      }
    }
  }
*/
// mig: fn cant_make_weakable_extend_a_non_weakable
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn cant_make_weakable_extend_a_non_weakable() { panic!("Unmigrated test: cant_make_weakable_extend_a_non_weakable"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Cant make weakable extend a non-weakable") {
    val compile = CompilerTestCompilation.test(
      """
        |interface IUnit {}
        |weakable struct Muta { hp int; }
        |impl IUnit for Muta;
        |func main(muta Muta) int  { return 7; }
        |""".stripMargin)

    try {
      compile.expectCompilerOutputs().lookupFunction("main")
      vfail()
    } catch {
      case WeakableImplingMismatch(true, false) =>
      case _ => vfail()
    }
  }
*/
// mig: fn cant_make_weak_ref_to_non_weakable
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn cant_make_weak_ref_to_non_weakable() { panic!("Unmigrated test: cant_make_weak_ref_to_non_weakable"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Cant make weak ref to non-weakable") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Muta { hp int; }
        |exported func main() int {
        |  m = Muta(7);
        |  w = &&m;
        |  return m.hp;
        |}
        |""".stripMargin)

    try {
      compile.expectCompilerOutputs().lookupFunction("main")
      vfail()
    } catch {
      case TookWeakRefOfNonWeakableError() =>
      case other => vfail(other)
    }

  }
*/
// mig: fn hash_map_style_return_type_inference_must_not_skip_caller_bound_args
#[test]
#[ignore = "unmigrated - pending typing-pass body migration"]
fn hash_map_style_return_type_inference_must_not_skip_caller_bound_args() { panic!("Unmigrated test: hash_map_style_return_type_inference_must_not_skip_caller_bound_args"); }
/*
  // Regression guard for @BRRZ. Reproduces the shape from docs/Generics.md:531-539
  // that motivated removing return-type inference. With the relaxed ResolveSR puzzle
  // the solver no longer stalls on K and V, but the post-solve bound-arg check
  // (InferCompiler.checkResolvingConclusionsAndResolve:295) must still reject this
  // because main doesn't supply enough to determine K and V. If this test ever
  // passes, the safety property of BRRZ has drifted and needs immediate investigation.
  test("HashMap-style return-type inference must not skip caller bound args") {
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct<K, V, H> { }
        |
        |func make<K, V, H>(h H) MyStruct<K, V, H>
        |where func drop(H)void {
        |  return MyStruct<K, V, H>();
        |}
        |
        |exported func main() int {
        |  m = make(7);
        |  return 0;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(_) => // expected — K and V cannot be inferred
      case Ok(_) => vfail("Expected HashMap-style K/V inference from return type to fail, but compilation succeeded.")
    }
  }

}
*/
