// mig: struct AfterRegionsIntegrationTests
pub struct AfterRegionsIntegrationTests;
/*
package dev.vale

import dev.vale.finalast._
import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.passmanager.{FullCompilation, FullCompilationOptions}
import dev.vale.postparsing._
import dev.vale.testvm._
import dev.vale.typing.ast._
import dev.vale.typing.citizen.WeakableImplingMismatch
import dev.vale.typing.expression.TookWeakRefOfNonWeakableError
import dev.vale.typing.names.{FunctionNameT, FunctionTemplateNameT}
import dev.vale.typing.templata.MutabilityTemplataT
import dev.vale.typing.types._
import dev.vale.typing.{HinputsT, ICompileErrorT}
import dev.vale.von.{IVonData, VonBool, VonFloat, VonInt}
import org.scalatest._


class AfterRegionsIntegrationTests extends FunSuite with Matchers {
*/
// mig: fn todo
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn todo() { panic!("Unmigrated test: todo"); }
/*
  ignore("TODO") {
    // only look at function bounds from the caller's environment, dont get any actual functions
    // from there. we can get actual functions from the type's environment, however.
    vimpl()

    // every time we do a templatas substitute, we do a substitutions for any of their bounds in the
    // coutputs. that's likely really expensive.
    // and it might be unnecessary? can the instantiator resolve those mappings themselves? perhaps
    // there's some in-between where templar can track merely that a bound *was* satisfied, but not
    // what satisfied it.
    // would that be enough for e.g. cases, which bring in bounds from the kind they're matching?
    // and parameters and stuff?
    vimpl()

    // had a bug when as was defined like this:
    //   extern("vale_as_subtype")
    //   func as<SubKind Kind, SuperType Ref>(left SuperType) Result<SubType, SuperType>
    //     where O Ownership,
    //   SuperKind Kind,
    //   SubType Ref = Ref[O, SubKind],
    //   SuperType Ref = Ref[O, SuperKind],
    //   implements(SubType, SuperType);
    // the definition assumed O was own, and the call inferred O to be borrow.
    // this cause some mayhem further down when a name didnt match.
    vimpl()
  }
*/
// mig: fn test_returning_empty_seq
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_returning_empty_seq() { panic!("Unmigrated test: test_returning_empty_seq"); }
/*
  test("Test returning empty seq") {
    val compile = RunCompilation.test(
      """
        |export () as Tup0;
        |exported func main() () {
        |  return ();
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    compile.run(Vector())
  }
*/
// mig: fn map_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn map_function() { panic!("Unmigrated test: map_function"); }
/*
  // Family 3: generic virtual dispatcher with abstract generics not reachable from
  // self-interface. Exercises `abstract func map<T, R>(virtual opt &Opt<T>, ...) Opt<R>`,
  // where `R` doesn't appear in `self`. The typing-pass → instantiator pipeline was
  // built around the invariant "every dispatcher placeholder mimics an impl placeholder";
  // this test breaks that. Three layered fixes already landed (FunctionCompilerSolvingLayer
  // vimpl removal, optutils.vale getOr signature rewrite, EdgeCompiler fresh-placeholder
  // inclusion), but the final Instantiator.translateOverride patch (Layer 4) was prototyped
  // and reverted pending owner review.
  //
  // See:
  //   - docs/historical/after-regions-test-fixing-quest.md (Family 3 section)
  //   - investigations/family3_map_function.md (collapsed call tree, instrumentation,
  //     architectural audit, git archaeology)
  //   - docs/Generics.md §§ GTCII, CDFGI, FODAIR, AFCTD, OMCNAGP
  ignore("Map function") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/mapFunc.vale"))
    compile.expectCompilerOutputs()

    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn imm_tuple_access
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn imm_tuple_access() { panic!("Unmigrated test: imm_tuple_access"); }
/*
  test("imm tuple access") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/tuples/immtupleaccess.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn interface_method_call_on_impl_bounded_generic_dispatches_through_interface
#[test]
fn interface_method_call_on_impl_bounded_generic_dispatches_through_interface() {
    // The scenario: genericGetFuel<T> takes &T with a `where implements(T, IShip)` bound
    // and calls x.getFuel() in its body. The user expects this to find IShip's abstract
    // getFuel, then dispatch virtually to Raza's override at runtime.
    //
    // Why this isn't automatic: Vale's interface abstract methods don't sit at the package
    // level. They live inside the interface's own outer env, reachable only via
    // coutputs.getOuterEnvForType(getInterfaceTemplate(IShip)). For a *concrete* &IShip
    // receiver, OverloadResolver.getParamEnvironments mechanically returns IShip's outer
    // env (because the receiver's type names IShip directly). For a *placeholder* &T
    // receiver, the type doesn't name IShip — IShip is one indirection away, declared via
    // the where-clause as an IsaTemplataT(T, IShip) entry in genericGetFuel's near-env.
    // Without something following that indirection, the lookup of getFuel finds only the
    // free function `getFuel(self &Raza)` (which type-mismatches T) and never reaches
    // IShip's outer env where the abstract method lives. Pre-fix, this produced
    // "No ancestors satisfy call" and the program failed to type-check.
    //
    // What we changed: OverloadResolver.getCandidateBanners now also calls
    // getPlaceholderImplBoundEnvs alongside getParamEnvironments. For each placeholder-
    // typed param, it looks up ambient impl bounds keyed by the placeholder's imprecise
    // name (ImplSubCitizenImpreciseNameS, populated automatically when addRunedDataToNearEnv
    // writes the IsaTemplataT into the near-env), pulls each IsaTemplataT, and adds each
    // super-interface's outer env to the candidate search. With that, the abstract
    // getFuel(virtual self &IShip) becomes a candidate; the inner per-call-site solve
    // verifies T isa IShip via the same IsaTemplataT (through ImplCompiler.isParent); the
    // call resolves; the instantiator monomorphizes genericGetFuel<Raza>; and the backend
    // dispatches getFuel virtually through Raza's vtable, returning 42.
    //
    // The fix is principle-aligned with @BDPFWDZ (By Default Pull From Where Declared):
    // IShip's methods stay in IShip's outer env where they were declared; the resolver
    // walks (via the where-clause's IsaTemplataT link) to find them; nothing is copied
    // into the calling function's near-env. See
    // docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md for the broader principle.
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "sealed interface IShip {\n  func getFuel(virtual self &IShip) int;\n}\nstruct Raza { fuel int; }\nimpl IShip for Raza;\nfunc getFuel(self &Raza) int { return self.fuel; }\n\nfunc genericGetFuel<T>(x &T) int\nwhere implements(T, IShip) {\n  return x.getFuel();\n}\n\nexported func main() int {\n  return genericGetFuel(&Raza(42));\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("Expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Interface Method call on impl-bounded generic dispatches through interface") {
    // The scenario: genericGetFuel<T> takes &T with a `where implements(T, IShip)` bound
    // and calls x.getFuel() in its body. The user expects this to find IShip's abstract
    // getFuel, then dispatch virtually to Raza's override at runtime.
    //
    // Why this isn't automatic: Vale's interface abstract methods don't sit at the package
    // level. They live inside the interface's own outer env, reachable only via
    // coutputs.getOuterEnvForType(getInterfaceTemplate(IShip)). For a *concrete* &IShip
    // receiver, OverloadResolver.getParamEnvironments mechanically returns IShip's outer
    // env (because the receiver's type names IShip directly). For a *placeholder* &T
    // receiver, the type doesn't name IShip — IShip is one indirection away, declared via
    // the where-clause as an IsaTemplataT(T, IShip) entry in genericGetFuel's near-env.
    // Without something following that indirection, the lookup of getFuel finds only the
    // free function `getFuel(self &Raza)` (which type-mismatches T) and never reaches
    // IShip's outer env where the abstract method lives. Pre-fix, this produced
    // "No ancestors satisfy call" and the program failed to type-check.
    //
    // What we changed: OverloadResolver.getCandidateBanners now also calls
    // getPlaceholderImplBoundEnvs alongside getParamEnvironments. For each placeholder-
    // typed param, it looks up ambient impl bounds keyed by the placeholder's imprecise
    // name (ImplSubCitizenImpreciseNameS, populated automatically when addRunedDataToNearEnv
    // writes the IsaTemplataT into the near-env), pulls each IsaTemplataT, and adds each
    // super-interface's outer env to the candidate search. With that, the abstract
    // getFuel(virtual self &IShip) becomes a candidate; the inner per-call-site solve
    // verifies T isa IShip via the same IsaTemplataT (through ImplCompiler.isParent); the
    // call resolves; the instantiator monomorphizes genericGetFuel<Raza>; and the backend
    // dispatches getFuel virtually through Raza's vtable, returning 42.
    //
    // The fix is principle-aligned with @BDPFWDZ (By Default Pull From Where Declared):
    // IShip's methods stay in IShip's outer env where they were declared; the resolver
    // walks (via the where-clause's IsaTemplataT link) to find them; nothing is copied
    // into the calling function's near-env. See
    // docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md for the broader principle.

    val compile =
      RunCompilation.test(
        """
          |sealed interface IShip {
          |  func getFuel(virtual self &IShip) int;
          |}
          |struct Raza { fuel int; }
          |impl IShip for Raza;
          |func getFuel(self &Raza) int { return self.fuel; }
          |
          |func genericGetFuel<T>(x &T) int
          |where implements(T, IShip) {
          |  return x.getFuel();
          |}
          |
          |exported func main() int {
          |  return genericGetFuel(&Raza(42));
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn test_overload_set
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_overload_set() { panic!("Unmigrated test: test_overload_set"); }
/*
  ignore("Test overload set") {
    // Search @POSIPP for why this doesn't work.
    val compile =
      RunCompilation.test(
        """
          |import array.each.*;
          |func myfunc(i int) { }
          |exported func main() int {
          |  mylist = [#](1, 3, 3, 7);
          |  mylist.each(myfunc);
          |  42
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn pass_overload_set_into_placeholder_parameter_posipp
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn pass_overload_set_into_placeholder_parameter_posipp() { panic!("Unmigrated test: pass_overload_set_into_placeholder_parameter_posipp"); }
/*
  ignore("Pass overload set into placeholder parameter (@POSIPP)") {
    // Search @POSIPP for why this doesn't work.
    val compile =
      RunCompilation.test(
        """
          |func myOtherFunc() { }
          |func myFunc<F>(f &F) void where func(&F)void { f() }
          |exported func main() int {
          |  myFunc(myOtherFunc);
          |  42
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn upcasting_in_a_generic_function
#[test]
#[ignore = "ignored upstream in Scala (see audit comment): pending CoordT redesign — make CoordT contain a placeholder and move Ownership to a generic param so the return type's ownership is calculated from the parameter"]
fn upcasting_in_a_generic_function() {
    // This is testing two things:
    //  - Upcasting inside a generic function
    //  - The return type's ownership is actually calculated from the parameter. This will
    //    fail as long as we still have CoordT(Ownership, ITemplata[KindTemplataType])
    //    because that ownership isn't a templata. The call site will correctly have that
    //    ownership as borrow, but the definition will think it's an own, *not* a placeholder
    //    or variable-thing or anything like that. So, when it gets to the instantiator, it
    //    will actually make the wrong return type. I think the solution will be to make CoordT
    //    contain a placeholder, and move O to be a generic param.
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "func upcast<SuperKind Kind, SubType Ref>(left SubType) SuperType\nwhere O Ownership,\n  SubKind Kind,\n  SuperType Ref = Ref[O, SuperKind],\n  SubType Ref = Ref[O, SubKind],\n  implements(SubType, SuperType)\n{\n  left\n}\n\nsealed interface IShip  {}\nstruct Serenity {}\nimpl IShip for Serenity;\n\nexported func main() {\n  ship &IShip = upcast<IShip>(&Serenity());\n}\n\n",
    );

    compile.eval_for_kind_primitive_args(Vec::new()).unwrap();
}
/*
  ignore("Upcasting in a generic function") {
    // This is testing two things:
    //  - Upcasting inside a generic function
    //  - The return type's ownership is actually calculated from the parameter. This will
    //    fail as long as we still have CoordT(Ownership, ITemplata[KindTemplataType])
    //    because that ownership isn't a templata. The call site will correctly have that
    //    ownership as borrow, but the definition will think it's an own, *not* a placeholder
    //    or variable-thing or anything like that. So, when it gets to the instantiator, it
    //    will actually make the wrong return type. I think the solution will be to make CoordT
    //    contain a placeholder, and move O to be a generic param.
    val compile = RunCompilation.test(
      """
        |func upcast<SuperKind Kind, SubType Ref>(left SubType) SuperType
        |where O Ownership,
        |  SubKind Kind,
        |  SuperType Ref = Ref[O, SuperKind],
        |  SubType Ref = Ref[O, SubKind],
        |  implements(SubType, SuperType)
        |{
        |  left
        |}
        |
        |sealed interface IShip  {}
        |struct Serenity {}
        |impl IShip for Serenity;
        |
        |exported func main() {
        |  ship &IShip = upcast<IShip>(&Serenity());
        |}
        |
        |""".stripMargin)

    compile.evalForKind(Vector())
  }
*/
// mig: fn diff_iter
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn diff_iter() { panic!("Unmigrated test: diff_iter"); }
/*
  ignore("Diff iter") {
    // When we try to compile this:
    //   HashSetDiffIterator<K>(a.table, b, 0)
    // it makes sure all the struct rules pass, including its members, including this:
    //   table &[]Opt<X>;
    // And here we get a conflict:
    //   Conflict, thought rune X was Kind$_0 but now concluding it's Kind$_0
    // because one is Share ownership, and one is Own. (they look similar dont they)
    // I think it's because HashSet<K Ref imm> has an imm there, and HashSetDiffIterator<X> doesn't.
    // We need a better error message.
    val compile = RunCompilation.test(
      """
        |
        |#!DeriveStructDrop
        |struct HashSet<K Ref imm> {
        |  table! Array<mut, Opt<K>>;
        |  size! int;
        |}
        |
        |struct HashSetDiffIterator<X> {
        |  table &[]Opt<X>;
        |  otherTable &HashSet<X>;
        |  pos! int;
        |}
        |
        |func diff_iter<K>(
        |  a &HashSet<K>,
        |  b &HashSet<K>)
        |HashSetDiffIterator<K> {
        |  HashSetDiffIterator<K>(a.table, b, 0)
        |}
        |
        |exported func main() int {
        |  hash = HashSet([]Opt<int>(0), 0);
        |  diff_iter(&hash, &hash);
        |  destruct hash;
        |  14
        |}
        |
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(14) => }
  }
*/
// mig: fn call_array_without_element_type
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn call_array_without_element_type() { panic!("Unmigrated test: call_array_without_element_type"); }
/*
  test("Call Array<> without element type") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = Array<imm>(3, {13 + _});
        |  sum = 0;
        |  drop_into(a, &(e) => { set sum = sum + e; });
        |  return sum;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn make_array_without_type
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_array_without_type() { panic!("Unmigrated test: make_array_without_type"); }
/*
  test("Make array without type") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = #[](10, {_});
        |  return a.3;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn borrowing_to_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn borrowing_to_array() { panic!("Unmigrated test: borrowing_to_array"); }
/*
  test("Borrowing toArray") {
    val compile = RunCompilation.test(
      """import list.*;
        |
        |func toArray<E>(list &List<E>) []<mut>&E {
        |  return []&E(list.len(), { list.get(_) });
        |}
        |
        |exported func main() int {
        |  l = List<int>();
        |  add(&l, 5);
        |  add(&l, 9);
        |  add(&l, 7);
        |  return l.toArray()[1];
        |}
        |
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn infinite_lambda_call
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_lambda_call() { panic!("Unmigrated test: infinite_lambda_call"); }
/*
  ignore("Infinite lambda call") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  lam = (f, z) => {
        |    f(f, z)
        |  };
        |  lam(lam, 7);
        |}
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
}
*/
