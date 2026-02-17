// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::statement_tests

/*
package dev.vale.parsing

import dev.vale.{Collector, Interner, StrI, vimpl, vwat}
import dev.vale.parsing.ast.{AugmentPE, BlockPE, BorrowP, ConsecutorPE, ConstantBoolPE, ConstantIntPE, ConstantStrPE, DestructPE, DestructureP, DotPE, EachPE, FunctionCallPE, IExpressionPE, IfPE, LetPE, LocalNameDeclarationP, LookupNameP, LookupPE, MutatePE, NameOrRunePT, NameP, PatternPP, Patterns, ReturnPE, TuplePE, UnletPE, VoidPE}
import dev.vale.parsing.ast._
import dev.vale.lexing._
import dev.vale.options.GlobalOptions
import org.scalatest._

class StatementTests extends FunSuite with Collector with TestParseUtils {
*/
use bumpalo::Bump;
use crate::interner::{Interner, StrI};
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_let() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "x = 4;");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Let(LetPE {
          pattern: PatternPP {
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
              ..
            }),
            templex: None,
            destructure: None,
            ..
          },
          source: IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
          ..
        }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected x = 4; structure"),
  }
}
/*
  test("Simple let") {
    compileBlockContentsExpect( "x = 4;") shouldHave {
      case LetPE(_, PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)), None, None), ConstantIntPE(_, 4, _)) =>
    }
  }
*/

#[test]
fn multiple_statements() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "4");
  match &expr {
    IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }) => {}
    _ => panic!("expected 4"),
  }

  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "4;");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected 4;"),
  }

  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "4; 3");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected 4; 3"),
  }

  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "4; 3;");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected 4; 3;"),
  }
}
/*
  test("multiple statements") {
    compileBlockContentsExpect(
      """4""".stripMargin) shouldHave {
      case ConstantIntPE(_, 4, _) =>
    }

    compileBlockContentsExpect(
      """4;""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), VoidPE(_))) =>
    }

    compileBlockContentsExpect(
      """4; 3""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 3, _))) =>
    }

    compileBlockContentsExpect(
      """4; 3;""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 3, _), VoidPE(_))) =>
    }
  }
*/

#[test]
fn test_8() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "[x, y] = (4, 5);");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP {
          patterns: [
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("y"))),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            ..
          ],
          ..
        }),
        ..
      },
      source: IExpressionPE::Tuple(TuplePE {
        elements: [
          IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
          IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
          ..
        ],
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected [x, y] = (4, 5); structure"),
  }
}
/*
  test("8") {
    compileStatementExpect("[x, y] = (4, 5);") shouldHave {
      case LetPE(_,
          PatternPP(_,
            None,
            None,
            Some(
              DestructureP(_,
                Vector(
                  PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),
                  PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("y"))), None)),None,None))))),
          TuplePE(_,Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 5, _)))) =>
    }
  }
*/

#[test]
fn test_9() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "set x.a = 5;");
  match &expr {
    IExpressionPE::Mutate(MutatePE {
      mutatee: IExpressionPE::Dot(DotPE {
        left: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
          template_args: None,
        }),
        member: NameP(_, StrI("a")),
        ..
      }),
      source: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      ..
    }) => {}
    _ => panic!("expected set x.a = 5; structure"),
  }
}
/*
  test("9") {
    compileStatementExpect("set x.a = 5;") shouldHave {
      case MutatePE(_, DotPE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))), None), _, NameP(_, StrI("a"))), ConstantIntPE(_, 5, _)) =>
    }
  }
*/

#[test]
fn test_1_pe() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, r#"set board.PE.PE.symbol = "v";"#);
  match &expr {
    IExpressionPE::Mutate(MutatePE {
      mutatee: IExpressionPE::Dot(DotPE {
        left: IExpressionPE::Dot(DotPE {
          left: IExpressionPE::Dot(DotPE {
            left: IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("board"))),
              template_args: None,
            }),
            member: NameP(_, StrI("PE")),
            ..
          }),
          member: NameP(_, StrI("PE")),
          ..
        }),
        member: NameP(_, StrI("symbol")),
        ..
      }),
      source: IExpressionPE::ConstantStr(ConstantStrPE { value: StrI("v"), .. }),
      ..
    }) =>
    {}
    _ => panic!(r#"expected set board.PE.PE.symbol = "v"; structure"#),
  }
}
/*
  test("1PE") {
    compileStatementExpect("""set board.PE.PE.symbol = "v";""") shouldHave {
      case MutatePE(_, DotPE(_, DotPE(_, DotPE(_, LookupPE(LookupNameP(NameP(_, StrI("board"))), None), _, NameP(_, StrI("PE"))), _, NameP(_, StrI("PE"))), _, NameP(_, StrI("symbol"))), ConstantStrPE(_, "v")) =>
    }
  }
*/

#[test]
fn test_simple_let() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "x = 3;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      source: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
      ..
    }) => {}
    _ => panic!("expected x = 3; structure"),
  }
}
/*
  test("Test simple let") {
    compileStatementExpect("x = 3;") shouldHave {
      case LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),ConstantIntPE(_, 3, _)) =>
    }
  }
*/

#[test]
fn test_simple_mut() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "set x = 5;");
  match &expr {
    IExpressionPE::Mutate(MutatePE {
      mutatee: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
        template_args: None,
      }),
      source: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      ..
    }) => {}
    _ => panic!("expected set x = 5; structure"),
  }
}
/*
  test("Test simple mut") {
    compileStatementExpect("set x = 5;") shouldHave {
      case MutatePE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))), None),ConstantIntPE(_, 5, _)) =>
    }
  }
*/

#[test]
fn test_expr_starting_with_return() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "retcode()");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("retcode"))),
        template_args: None,
      }),
      arg_exprs: [],
      ..
    }) => {}
    _ => panic!("expected retcode() structure"),
  }
}
/*
  test("Test expr starting with return") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect("retcode()") shouldHave {
      case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("retcode"))),None),Vector()) =>
    }
  }
*/

#[test]
fn test_inner_set() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "oldArray = set list.array = newArray;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("oldArray"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      source: IExpressionPE::Mutate(MutatePE {
        mutatee: IExpressionPE::Dot(DotPE {
          left: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("list"))),
            template_args: None,
          }),
          member: NameP(_, StrI("array")),
          ..
        }),
        source: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("newArray"))),
          template_args: None,
        }),
        ..
      }),
      ..
    }) =>
    {}
    _ => panic!("expected oldArray = set list.array = newArray; structure"),
  }
}
/*
  test("Test inner set") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect(
      "oldArray = set list.array = newArray;") shouldHave {
      case LetPE(_,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("oldArray"))), None)),None,None),
        MutatePE(_,
          DotPE(_,LookupPE(LookupNameP(NameP(_, StrI("list"))),None),_,NameP(_, StrI("array"))),
          LookupPE(LookupNameP(NameP(_, StrI("newArray"))),None))) =>
    }
  }
*/

#[test]
fn test_if_statement_producing() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "if true { 3 } else { 4 }");
  match &expr {
    IExpressionPE::If(IfPE {
      condition: IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }),
      then_body: BlockPE {
        inner: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        ..
      },
      else_body: BlockPE {
        inner: IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected if true {{ 3 }} else {{ 4 }} structure"),
  }
}
/*
  test("Test if-statement producing") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect(
      "if true { 3 } else { 4 }") shouldHave {
      case IfPE(_,
        ConstantBoolPE(_,true),
        BlockPE(_,None,None,ConstantIntPE(_,3,_)),
        BlockPE(_,None,None,ConstantIntPE(_,4,_))) =>
    }
  }
*/

#[test]
fn test_destruct() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "destruct x;");
  match &expr {
    IExpressionPE::Destruct(DestructPE {
      inner: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected destruct x; structure"),
  }
}
/*
  test("Test destruct") {
    compileStatementExpect("destruct x;") shouldHave {
      case DestructPE(_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None)) =>
    }
  }
*/

#[test]
fn test_unlet() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "unlet x");
  match &expr {
    IExpressionPE::Unlet(UnletPE {
      name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
      ..
    }) => {}
    _ => panic!("expected unlet x structure"),
  }
}
/*
  test("Test unlet") {
    compileStatementExpect("unlet x") shouldHave {
      case UnletPE(_,LookupNameP(NameP(_, StrI("x")))) =>
    }
  }
*/

#[test]
fn dot_on_function_calls_result() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "Wizard(8).charges");
  match &expr {
    IExpressionPE::Dot(DotPE {
      left: IExpressionPE::FunctionCall(FunctionCallPE {
        callable_expr: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("Wizard"))),
          template_args: None,
        }),
        arg_exprs: [IExpressionPE::ConstantInt(ConstantIntPE { value: 8, .. }), ..],
        ..
      }),
      member: NameP(_, StrI("charges")),
      ..
    }) => {}
    _ => panic!("expected Wizard(8).charges structure"),
  }
}
/*
  test("Dot on function call's result") {
    compileStatementExpect("Wizard(8).charges") shouldHave {
      case DotPE(_,
          FunctionCallPE(_,_,
            LookupPE(LookupNameP(NameP(_, StrI("Wizard"))), None),
            Vector(ConstantIntPE(_, 8, _))),
        _,
      NameP(_, StrI("charges"))) =>
    }
  }
*/

#[test]
fn let_with_pattern_with_only_a_capture() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "a = m;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      source: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("m"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected a = m; structure"),
  }
}
/*
  test("Let with pattern with only a capture") {
    compileStatementExpect("a = m;") shouldHave {
      case LetPE(_,Patterns.capture("a"),LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_simple_pattern() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "a Moo = m;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
          ..
        }),
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("Moo"))))),
        destructure: None,
        ..
      },
      source: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("m"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected a Moo = m; structure"),
  }
}
/*
  test("Let with simple pattern") {
    compileStatementExpect("a Moo = m;") shouldHave {
      case LetPE(_,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),Some(NameOrRunePT(NameP(_, StrI("Moo")))),None),
        LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_simple_pattern_in_destructure() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "[a Moo] = m;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP {
          patterns: [PatternPP {
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
              ..
            }),
            templex: Some(ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("Moo"))))),
            destructure: None,
            ..
          }, ..],
          ..
        }),
        ..
      },
      source: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("m"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected [a Moo] = m; structure"),
  }
}
/*
  test("Let with simple pattern in destructure") {
    compileStatementExpect("[a Moo] = m;") shouldHave {
      case LetPE(_,
          PatternPP(_,_,
            None,
            Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),Some(NameOrRunePT(NameP(_, StrI("Moo")))),None))))),
          LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_destructuring_pattern() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "Muta[ ] = m;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: None,
        templex: Some(ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("Muta"))))),
        destructure: Some(DestructureP { patterns: [], .. }),
        ..
      },
      source: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("m"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected Muta[ ] = m; structure"),
  }
}
/*
  test("Let with destructuring pattern") {
    compileStatementExpect("Muta[ ] = m;") shouldHave {
      case LetPE(_,PatternPP(_,None,Some(NameOrRunePT(NameP(_, StrI("Muta")))),Some(DestructureP(_,Vector()))),LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn destructure_pattern_with_let_and_set() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "[a, set x] = m;");
  match &expr {
    IExpressionPE::Let(LetPE {
      pattern: PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP {
          patterns: [
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
                mutate: None,
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                mutate: Some(_),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            ..
          ],
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected [a, set x] = m; structure"),
  }
}
/*
  test("Destructure pattern with let and set") {
    compileStatementExpect("[a, set x] = m;") shouldHave {
      case LetPE(_,
        PatternPP(_,
          None,None,
          Some(
            DestructureP(_,
              Vector(
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("a"))),None)),None,None),
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))),Some(_))),None,None))))),
        _) =>
    }
  }
*/

#[test]
fn ret() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "return 3;");
  match &expr {
    IExpressionPE::Return(ReturnPE {
      expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
      ..
    }) => {}
    _ => panic!("expected return 3; structure"),
  }
}
/*
  test("Ret") {
    compileStatementExpect("return 3;") shouldHave {
      case ReturnPE(_,ConstantIntPE(_, 3, _)) =>
    }
  }
*/

#[test]
fn foreach() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "foreach i in myList { }");
  match &expr {
    IExpressionPE::Each(EachPE {
      maybe_pure: None,
      entry_pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("i"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      iterable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("myList"))),
        template_args: None,
      }),
      body: BlockPE {
        inner: IExpressionPE::Void(_),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected foreach i in myList {{ }} structure"),
  }
}
/*
  test("foreach") {
    compileStatementExpect("foreach i in myList { }") shouldHave {
      case EachPE(_,
      None,
      PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
      _,
      LookupPE(LookupNameP(NameP(_, StrI("myList"))),None),
      BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_with_borrow() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "foreach i in &myList { }");
  match &expr {
    IExpressionPE::Each(EachPE {
      maybe_pure: None,
      entry_pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("i"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      iterable_expr: IExpressionPE::Augment(AugmentPE {
        target_ownership: OwnershipP::Borrow,
        inner: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("myList"))),
          template_args: None,
        }),
        ..
      }),
      body: BlockPE {
        inner: IExpressionPE::Void(_),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected foreach i in &myList {{ }} structure"),
  }
}
/*
  test("foreach with borrow") {
    compileStatementExpect("foreach i in &myList { }") shouldHave {
      case EachPE(_,
      None,
      PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
      _,
      AugmentPE(_, BorrowP, LookupPE(LookupNameP(NameP(_, StrI("myList"))),None)),
      BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_with_two_receivers() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "foreach [a, b] in myList { }");
  match &expr {
    IExpressionPE::Each(EachPE {
      maybe_pure: None,
      entry_pattern: PatternPP {
        destination: None,
        templex: None,
        destructure: Some(DestructureP {
          patterns: [
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("b"))),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            ..
          ],
          ..
        }),
        ..
      },
      iterable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("myList"))),
        template_args: None,
      }),
      body: BlockPE {
        inner: IExpressionPE::Void(_),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected foreach [a, b] in myList {{ }} structure"),
  }
}
/*
  test("foreach with two receivers") {
    compileStatementExpect("foreach [a, b] in myList { }") shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,
          None,None,
          Some(
            DestructureP(_,
              Vector(
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),None,None),
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("b"))), None)),None,None))))),
        _,
        LookupPE(LookupNameP(NameP(_, StrI("myList"))),None),
        BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_complex_iterable() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "foreach i in myList = 3; myList { }");
  match &expr {
    IExpressionPE::Each(EachPE {
      maybe_pure: None,
      entry_pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("i"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      iterable_expr: IExpressionPE::Consecutor(ConsecutorPE {
        inners: [
          IExpressionPE::Let(LetPE {
            pattern: PatternPP {
              destination: Some(DestinationLocalP {
                decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("myList"))),
                ..
              }),
              templex: None,
              destructure: None,
              ..
            },
            source: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
            ..
          }),
          IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("myList"))),
            template_args: None,
          }),
          ..
        ],
        ..
      }),
      body: BlockPE {
        inner: IExpressionPE::Void(_),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected foreach i in myList = 3; myList {{ }} structure"),
  }
}
/*
  test("foreach complex iterable") {
    compileStatementExpect("foreach i in myList = 3; myList { }") shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
        _,
        ConsecutorPE(
          Vector(
            LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("myList"))), None)),None,None),ConstantIntPE(_,3,_)),
            LookupPE(LookupNameP(NameP(_, StrI("myList"))),None))),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
*/

#[test]
fn multiple_statements_2() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      42;
      43;
      ",
  );
}
/*
  test("Multiple statements") {
    compileBlockContentsExpect(
      """
        |42;
        |43;
        |""".stripMargin)
  }
*/

#[test]
fn if_and_another_statement() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      newCapacity = if (true) { 1 } else { 2 };
      newArray = 3;
      ",
  );
}
/*
  test("If and another statement") {
    compileBlockContentsExpect(
      """
        |newCapacity = if (true) { 1 } else { 2 };
        |newArray = 3;
        |""".stripMargin)
  }
*/

#[test]
fn test_blocks_trailing_void_presence() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "moo()");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("moo"))),
        template_args: None,
      }),
      arg_exprs: [],
      ..
    }) => {}
    _ => panic!("expected moo() structure"),
  }

  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "moo();");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::FunctionCall(FunctionCallPE {
          callable_expr: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("moo"))),
            template_args: None,
          }),
          arg_exprs: [],
          ..
        }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected moo(); structure"),
  }
}
/*
  test("Test block's trailing void presence") {
    compileBlockContentsExpect(
      "moo()") shouldHave {
      case FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("moo"))), None), Vector()) =>
    }

    compileBlockContentsExpect(
      "moo();") shouldHave {
      case ConsecutorPE(Vector(FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("moo"))), None), Vector()), VoidPE(_))) =>
    }
  }
*/

#[test]
fn block_with_statement_and_result() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      b;
      a
    ",
  );
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("b"))),
          template_args: None,
        }),
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
          template_args: None,
        }),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected b; a structure"),
  }
}
/*
  test("Block with statement and result") {
    compileBlockContentsExpect(
      """
        |b;
        |a
      """.stripMargin) shouldHave {
      case Vector(LookupPE(LookupNameP(NameP(_, StrI("b"))), None), LookupPE(LookupNameP(NameP(_, StrI("a"))), None)) =>
    }
  }
*/

#[test]
fn block_with_result() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, &parse_arena, "3");
  match &expr {
    IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }) => {}
    _ => panic!("expected 3"),
  }
}
/*
  test("Block with result") {
    compileStatementExpect("3") shouldHave {
      case ConstantIntPE(_, 3, _) =>
    }
  }
*/

#[test]
fn block_with_result_that_could_be_an_expr() {
  // = doThings(a); could be misinterpreted as an expression doThings(=, a) if we're
  // not careful.
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      a = 2;
      doThings(a)
    ",
  );
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Let(LetPE {
          pattern: PatternPP {
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
              ..
            }),
            templex: None,
            destructure: None,
            ..
          },
          source: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
          ..
        }),
        IExpressionPE::FunctionCall(FunctionCallPE {
          callable_expr: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("doThings"))),
            template_args: None,
          }),
          arg_exprs: [IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
            template_args: None,
          }), ..],
          ..
        }),
        ..
      ],
      ..
    }) =>
    {}
    _ => panic!("expected a = 2; doThings(a) structure"),
  }
}
/*
  test("Block with result that could be an expr") {
    // = doThings(a); could be misinterpreted as an expression doThings(=, a) if we're
    // not careful.
    compileBlockContentsExpect(
      """
        |a = 2;
        |doThings(a)
      """.stripMargin) shouldHave {
      case Vector(
        LetPE(_, PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)), None, None), ConstantIntPE(_, 2, _)),
        FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("doThings"))), None), Vector(LookupPE(LookupNameP(NameP(_, StrI("a"))), None)))) =>
    }
  }
*/

#[test]
fn mutating_as_statement() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, &parse_arena, "set x = 6;");
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Mutate(MutatePE {
          mutatee: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          source: IExpressionPE::ConstantInt(ConstantIntPE { value: 6, .. }),
          ..
        }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected set x = 6; structure"),
  }
}
/*
  test("Mutating as statement") {
    val program =
      compileBlockContentsExpect(
        "set x = 6;")
    program shouldHave {
      case MutatePE(_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None),ConstantIntPE(_, 6, _)) =>
    }
  }
*/

#[test]
fn lone_block() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      block {
        a
      }
    ",
  );
  match &expr {
    IExpressionPE::Block(BlockPE {
      inner: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected block {{ a }} structure"),
  }
}
/*
  test("Lone block") {
    compileBlockContentsExpect(
      """
        |block {
        |  a
        |}
      """.stripMargin) shouldHave {
      case BlockPE(_,None,None,LookupPE(LookupNameP(NameP(_, StrI("a"))),None)) =>
    }
  }
*/

#[test]
fn pure_block() {
  // Just make sure it parses, so that we can highlight it.
  // The pure block feature doesn't actually exist yet.
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      pure block {
        a
      }
    ",
  );
}
/*
  test("Pure block") {
    // Just make sure it parses, so that we can highlight it.
    // The pure block feature doesn't actually exist yet.
    compileBlockContentsExpect(
      """
        |pure block {
        |  a
        |}
      """.stripMargin)
  }
*/

#[test]
fn unsafe_pure_block() {
  // Just make sure it parses, so that we can highlight it.
  // The unsafe pure block feature doesn't actually exist yet.
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      unsafe pure block {
        a
      }
    ",
  );
}
/*
  test("Unsafe pure block") {
    // Just make sure it parses, so that we can highlight it.
    // The unsafe pure block feature doesn't actually exist yet.
    compileBlockContentsExpect(
      """
        |unsafe pure block {
        |  a
        |}
      """.stripMargin)
  }
*/

#[test]
fn report_leaving_out_semicolon_or_ending_body_after_expression_for_square() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let err = compile_statement(
    &interner,
    &keywords,
    &parse_arena,
    "
      block {
        floop() ]
      }
      ",
  )
  .unwrap_err();
  assert!(matches!(err, ParseError::BadStartOfStatementError(_)));
}
/*
  test("Report leaving out semicolon or ending body after expression, for square") {
    compileStatement(
      """
        |block {
        |  floop() ]
        |}
        """.stripMargin).expectErr() match {
      case BadStartOfStatementError(_) =>
    }
  }
*/

#[test]
fn empty_block() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      block {
      }
      return 3;
      ",
  );
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Block(BlockPE {
          inner: IExpressionPE::Void(_),
          ..
        }),
        IExpressionPE::Return(ReturnPE {
          expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
          ..
        }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected block {{ }} return 3; structure"),
  }
}
/*
  test("Empty block") {
    compileBlockContentsExpect(
      """
        |block {
        |}
        |return 3;
    """.stripMargin) match {
      case ConsecutorPE(
        Vector(
          BlockPE(_,None,None,VoidPE(_)),
          ReturnPE(_,ConstantIntPE(_,3,None)), VoidPE(_))) =>
    }
  }
*/

#[test]
fn cant_use_set_as_a_local_name() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let err = compile_statement(&interner, &keywords, &parse_arena, "[set] = (6,)").unwrap_err();
  assert!(matches!(
    err,
    ParseError::CantUseThatLocalName { ref name, .. } if name == "set"
  ));
}
/*
  test("Cant use set as a local name") {
    val error = compileStatement(
      """[set] = (6,)""".stripMargin).expectErr()
    error match {
      case CantUseThatLocalName(_, "set") =>
    }
  }
*/

#[test]
fn foreach_2() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      foreach i in a {
        i
      }
      ",
  );
  match &expr {
    IExpressionPE::Each(EachPE {
      maybe_pure: None,
      entry_pattern: PatternPP {
        destination: Some(DestinationLocalP {
          decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("i"))),
          ..
        }),
        templex: None,
        destructure: None,
        ..
      },
      iterable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
        template_args: None,
      }),
      body: BlockPE {
        inner: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("i"))),
          template_args: None,
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected foreach i in a {{ i }} structure"),
  }
}
/*
  test("foreach 2") {
    val programS =
      compileBlockContentsExpect(
        """
          |foreach i in a {
          |  i
          |}
          |""".stripMargin)
    programS shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
        _,
        LookupPE(LookupNameP(NameP(_, StrI("a"))),None),
        BlockPE(_,None,None,
          LookupPE(LookupNameP(NameP(_, StrI("i"))),None))) =>
    }
  }
*/

#[test]
fn foreach_expr() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    &parse_arena,
    "
      a = foreach i in c { i };
      ",
  );
  match &expr {
    IExpressionPE::Consecutor(ConsecutorPE {
      inners: [
        IExpressionPE::Let(LetPE {
          pattern: PatternPP {
            destination: Some(DestinationLocalP {
              decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
              ..
            }),
            templex: None,
            destructure: None,
            ..
          },
          source: IExpressionPE::Each(_),
          ..
        }),
        IExpressionPE::Void(_),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected a = foreach i in c {{ i }}; structure"),
  }
}
/*
  test("foreach expr") {
    val programS =
      compileBlockContentsExpect(
        """
          |a = foreach i in c { i };
          |""".stripMargin)
    programS shouldHave {
      case ConsecutorPE(Vector(
        LetPE(_,
          PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("a"))), None)),None,None),
          EachPE(_,_,_,_,_,_)),
        VoidPE(_))) =>
    }
  }

}
*/