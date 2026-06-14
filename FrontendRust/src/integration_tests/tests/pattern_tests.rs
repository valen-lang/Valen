use crate::instantiating::ast::citizens::IMemberTypeI;
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::names::IStructTemplateNameI;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::names::StructNameI;
use crate::instantiating::ast::names::StructTemplateNameI;
use crate::instantiating::ast::types::CoordI;
use crate::instantiating::ast::types::IntIT;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::ast::types::OwnershipI;
use crate::instantiating::ast::types::StructIT;
use crate::integration_tests::tests::run_compilation::test;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::typing::types::types::CoordT;
use crate::typing::types::types::IRegionT;
use crate::typing::types::types::IntT;
use crate::typing::types::types::KindT;
use crate::typing::types::types::OwnershipT;
use crate::typing::types::types::RegionT;
use crate::typing::typing_interner::TypingInterner;
use crate::von::ast::IVonData;
use crate::von::ast::VonInt;
// mig: struct PatternTests
pub struct PatternTests;

/*
package dev.vale

import dev.vale.parsing.ast.FinalP
import dev.vale.postparsing.CodeRuneS
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.types._
import dev.vale.typing._
import dev.vale.instantiating.ast._
import dev.vale.typing.ast.{NormalStructMemberT, ReferenceMemberTypeT}
import dev.vale.typing.names.{IdT, KindPlaceholderNameT, KindPlaceholderTemplateNameT, StructNameT, StructTemplateNameT}
import dev.vale.typing.types.IntT
import dev.vale.von.VonInt
import org.scalatest._

class PatternTests extends FunSuite with Matchers {
  // To get something like this to work would be rather involved.
  //test("Test matching a single-member pack") {
  //  val compile = RunCompilation.test( "exported func main() int { [x] = (4); = x; }")
  //  compile.getCompilerOutputs()
  //  val main = coutputs.lookupFunction("main")
  //  main.header.returnType shouldEqual Coord(Share, Readonly, Int2())
  //  compile.evalForKind(Vector()) match { case VonInt(4) => }
  //}
*/
// mig: fn test_matching_a_multiple_member_seq_of_immutables
#[test]
fn test_matching_a_multiple_member_seq_of_immutables() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "exported func main() int { [x, y] = (4, 5); return y; }",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type, CoordT {
            ownership: OwnershipT::Share,
            region: RegionT { region: IRegionT::Default },
            kind: KindT::Int(IntT::I32),
        });
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}

/*
  test("Test matching a multiple-member seq of immutables") {
    // Checks that the 5 made it into y, and it was an int
    val compile = RunCompilation.test( "exported func main() int { [x, y] = (4, 5); return y; }")
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    main.header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn test_matching_a_multiple_member_seq_of_mutables
#[test]
fn test_matching_a_multiple_member_seq_of_mutables() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
struct Marine { hp int; }
exported func main() int { [x, y] = (Marine(6), Marine(8)); return y.hp; }
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type, CoordT {
            ownership: OwnershipT::Share,
            region: RegionT { region: IRegionT::Default },
            kind: KindT::Int(IntT::I32),
        });
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}

/*
  test("Test matching a multiple-member seq of mutables") {
    // Checks that the 5 made it into y, and it was an int
    val compile = RunCompilation.test(
      """
        |struct Marine { hp int; }
        |exported func main() int { [x, y] = (Marine(6), Marine(8)); return y.hp; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    main.header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn test_matching_a_multiple_member_pack_of_immutable_and_own
#[test]
fn test_matching_a_multiple_member_pack_of_immutable_and_own() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
struct Marine { hp int; }
exported func main() int { [x, y] = (7, Marine(8)); return y.hp; }
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        // BUG: Scala uses `==` (a pure expression with discarded result) instead of `shouldEqual`; the assertion is dead.
        let _ = coutputs.functions[0].header.return_type == CoordT {
            ownership: OwnershipT::Share,
            region: RegionT { region: IRegionT::Default },
            kind: KindT::Int(IntT::I32),
        };
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}

/*
  test("Test matching a multiple-member pack of immutable and own") {
    // Checks that the 5 made it into y, and it was an int
    val compile = RunCompilation.test(
      """
        |struct Marine { hp int; }
        |exported func main() int { [x, y] = (7, Marine(8)); return y.hp; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    coutputs.functions.head.header.returnType == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn test_matching_a_multiple_member_pack_of_immutable_and_borrow
#[test]
fn test_matching_a_multiple_member_pack_of_immutable_and_borrow() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    // Checks that the 5 made it into y, and it was an int
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
struct Marine { hp int; }
exported func main() int {
  m = Marine(8);
  [x, y] = (7, &m);
  return y.hp;
}
",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        // BUG: Scala uses `==` (a pure expression with discarded result) instead of `shouldEqual`; the assertion is dead.
        let _ = coutputs.functions[0].header.return_type == CoordT {
            ownership: OwnershipT::Share,
            region: RegionT { region: IRegionT::Default },
            kind: KindT::Int(IntT::I32),
        };
    }
    {
        let monouts = compile.get_monouts();
        let tup_def = monouts.lookup_struct_by_name("Tup2");
        let tup_def_member_types: Vec<CoordI<'_, '_>> = tup_def.members.iter().filter_map(|m| match m.tyype {
            IMemberTypeI::AddressMemberTypeI(t) => Some(t.reference),
            IMemberTypeI::ReferenceMemberTypeI(t) => Some(t.reference),
        }).collect();
        match tup_def_member_types.as_slice() {
            [
                CoordI {
                    ownership: OwnershipI::MutableShare,
                    kind: KindIT::IntIT(IntIT { bits: 32, .. }),
                },
                CoordI {
                    ownership: OwnershipI::MutableBorrow,
                    kind: KindIT::StructIT(StructIT {
                        id: IdI {
                            init_steps: &[],
                            local_name: INameI::StructName(StructNameI {
                                template: IStructTemplateNameI::StructTemplate(StructTemplateNameI {
                                    human_name: StrI("Marine"),
                                    ..
                                }),
                                template_args: &[],
                            }),
                            ..
                        },
                        ..
                    }),
                },
            ] => {}
            _ => panic!("tup_def_member_types shape mismatch"),
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}

/*
  test("Test matching a multiple-member pack of immutable and borrow") {
    // Checks that the 5 made it into y, and it was an int
    val compile = RunCompilation.test(
      """
        |struct Marine { hp int; }
        |exported func main() int {
        |  m = Marine(8);
        |  [x, y] = (7, &m);
        |  return y.hp;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    coutputs.functions.head.header.returnType == CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)

    val monouts = compile.getMonouts()
    val tupDef = monouts.lookupStruct("Tup2")
    val tupDefMemberTypes =
      tupDef.members.collect({
        case StructMemberI(_, _, AddressMemberTypeI(tyype)) => tyype
        case StructMemberI(_, _, ReferenceMemberTypeI(tyype)) => tyype
      })
    tupDefMemberTypes match {
      case Vector(
        CoordI(MutableShareI,IntIT(32)),
        CoordI(MutableBorrowI,StructIT(IdI(_,Vector(),StructNameI(StructTemplateNameI(StrI("Marine")),Vector()))))) =>
      case null =>
//      case Vector(
//        ReferenceMemberTypeT(CoordT(own,PlaceholderT(IdT(_,Vector(StructTemplateNameT(StrI("Tup"))),PlaceholderNameT(PlaceholderTemplateNameT(0,CodeRuneS(StrI("T1")))))))),
//        ReferenceMemberTypeT(CoordT(OwnT,PlaceholderT(IdT(_,Vector(StructTemplateNameT(StrI("Tup"))),PlaceholderNameT(PlaceholderTemplateNameT(1,CodeRuneS(StrI("T2"))))))))) =>
    }
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn test_destructuring_a_shared
#[test]
fn test_destructuring_a_shared() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
import array.iter.*;
exported func main() int {
  sm = #[#](#[#](42, 73, 73));
  foreach [i, m1] in sm {
    return i;
  }
}
",
    );
    {
        let _coutputs = compile.expect_compiler_outputs();
    }
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Test destructuring a shared") {
    val compile = RunCompilation.test(
      """
        |import array.iter.*;
        |exported func main() int {
        |  sm = #[#](#[#](42, 73, 73));
        |  foreach [i, m1] in sm {
        |    return i;
        |  }
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
/*



//  test("Test if-let") {
//    // Checks that the 5 made it into y, and it was an int
//    val compile = RunCompilation.test(
//      """
//        |interface ISpaceship { }
//        |
//        |struct Firefly { fuel int; }
//        |impl ISpaceship for Firefly;
//        |
//        |exported func main() int {
//        |  s ISpaceship = Firefly(42);
//        |  return if (Firefly(fuel) = *s) {
//        |      fuel
//        |    } else {
//        |      73
//        |    }
//        |}
//      """.stripMargin)
//    val coutputs = compile.expectCompilerOutputs()
//    coutputs.functions.head.header.returnType == CoordT(ShareT, IntT.i32)
//    compile.evalForKind(Vector()) match { case VonInt(8) => }
//  }

  // Intentional known failure 2021.02.28, we never implemented pattern destructuring
//  test("Test imm struct param destructure") {
//    // Checks that the 5 made it into y, and it was an int
//    val compile = RunCompilation.test(
//      """
//        |
//        |struct Vec3 { x int; y int; z int; } exported func main() { refuelB(Vec3(1, 2, 3), 2); }
//        |// Using above Vec3
//        |
//        |// Without destructuring:
//        |func refuelA(
//        |    vec Vec3,
//        |    len int) {
//        |  Vec3(
//        |      vec.x * len,
//        |      vec.y * len,
//        |      vec.z * len)
//        |}
//        |
//        |// With destructuring:
//        |func refuelB(
//        |    Vec3(x, y, z),
//        |    len int) {
//        |  Vec3(x * len, y * len, z * len)
//        |}
//        |""".stripMargin)
//    val coutputs = compile.expectCompilerOutputs()
//    coutputs.functions.head.header.returnType == CoordT(ShareT, IntT.i32)
//    compile.evalForKind(Vector()) match { case VonInt(8) => }
//  }

*/
// mig: fn ignore_destructure
#[test]
fn ignore_destructure() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r"
struct Marine {
  hp int;
}
exported func main() int {
  m = Marine(4);
  Marine[_] = m;
  return 42;
}
",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Ignore destructure") {
    val compile = RunCompilation.test(
      """
        |struct Marine {
        |  hp int;
        |}
        |exported func main() int {
        |  m = Marine(4);
        |  Marine[_] = m;
        |  return 42;
        |}
  """.stripMargin)

    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }
*/

/*
}

*/
