// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::functions::function_tests

/*
package dev.vale.parsing.functions

import dev.vale.{Collector, StrI, vassertOne, vimpl}
import dev.vale.parsing.ast._
import dev.vale.parsing._
import dev.vale.lexing.{BadFunctionBodyError, LightFunctionMustHaveParamTypes}
import org.scalatest._
import org.scalatest._


class FunctionTests extends FunSuite with Collector with TestParseUtils {
*/
use bumpalo::Bump;
use crate::cast;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;
use crate::parsing::tests::utils::{
  assert_destination_local_name, assert_templex_name, expect_1, expect_2, find_func_named,
};
#[test]
fn simple_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "func main() { }");
  let function = find_func_named(&program, "main");
  assert!(function.header.attributes.is_empty());
  assert!(function.header.generic_parameters.is_none());
  assert!(function.header.template_rules.is_none());
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert!(function.header.ret.ret_type.is_none());
  let body = function.body.as_ref().unwrap();
  assert!(body.maybe_pure.is_none());
  assert!(body.maybe_default_region.is_none());
  assert!(matches!(body.inner, IExpressionPE::Void(_)));
}
/*
  test("Simple function") {
    vassertOne(compileFileExpect("""func main() { }""").denizens) match {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,
            Some(NameP(_,StrI("main"))),
            Vector(),None,None,Some(ParamsP(_,Vector())),
            FunctionReturnP(_,None)),
          Some(BlockPE(_,None,None,VoidPE(_))))) =>
    }
  }
*/
#[test]
fn functions_with_weird_names() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "func !=() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, "!=");
  let program = compile(&parse_arena, &keywords, "func <=() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, "<=");
  let program = compile(&parse_arena, &keywords, "func >=() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, ">=");
  let program = compile(&parse_arena, &keywords, "func <() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, "<");
  let program = compile(&parse_arena, &keywords, "func >() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, ">");
  let program = compile(&parse_arena, &keywords, "func ==() { }");
  assert_eq!(program.denizens.len(), 1);
  find_func_named(&program, "==");
}
/*
  test("Functions with weird names") {
    vassertOne(compileFileExpect("""func !=() { }""").denizens)
    vassertOne(compileFileExpect("""func <=() { }""").denizens)
    vassertOne(compileFileExpect("""func >=() { }""").denizens)
    vassertOne(compileFileExpect("""func <() { }""").denizens)
    vassertOne(compileFileExpect("""func >() { }""").denizens)
    vassertOne(compileFileExpect("""func ==() { }""").denizens)
  }
*/
#[test]
fn function_then_struct() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(
    &parse_arena,
    &keywords,
    r#"
      exported func main() int {}

      struct mork { }
    "#,
  );
  assert_eq!(program.denizens.len(), 2);
  assert!(matches!(
    program.denizens[0],
    IDenizenP::TopLevelFunction(_)
  ));
  assert!(matches!(program.denizens[1], IDenizenP::TopLevelStruct(_)));
}
/*
  test("Function then struct") {
    val program =
      compileFile(
        """
          |exported func main() int {}
          |
          |struct mork { }
          |""".stripMargin).getOrDie()
    program.denizens(0) match { case TopLevelFunctionP(_) => }
    program.denizens(1) match { case TopLevelStructP(_) => }
  }
*/
#[test]
fn simple_function_with_return() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum() int {3}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(function.header.attributes.is_empty());
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "int");
  let body = function.body.as_ref().unwrap();
  assert_eq!(
    cast!(body.inner, IExpressionPE::ConstantInt).value,
    3
  );
}
/*
  test("Simple function with return") {
    compileDenizen("func sum() int {3}").getOrDie() match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))), Vector(), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, Some(_))),
        Some(BlockPE(_, None, None, ConstantIntPE(_, 3, _))))) =>
    }
  }
*/
#[test]
fn pure_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "pure func sum() {3}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::PureAttribute(_)]
  ));
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert!(function.header.ret.ret_type.is_none());
  let body = function.body.as_ref().unwrap();
  assert_eq!(
    cast!(body.inner, IExpressionPE::ConstantInt).value,
    3
  );
}
/*
  test("Pure function") {
    compileDenizen("pure func sum() {3}").getOrDie() match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))), Vector(PureAttributeP(_)), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, None)),
        Some(BlockPE(_, None, None, ConstantIntPE(_, 3, _))))) =>
    }
  }
*/
#[test]
fn extern_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "extern func sum();");
  let function = find_func_named(&program, "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::ExternAttribute(_)]
  ));
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert!(function.header.ret.ret_type.is_none());
  assert!(function.body.is_none());
}
/*
  test("Extern function") {
    vassertOne(compileFile("extern func sum();").getOrDie().denizens) match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))), Vector(ExternAttributeP(_)), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, None)),
        None)) =>
    }
  }
*/
#[test]
fn function_ending_with_set() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    r#"
      func moo() {
        set bork = value
      }
    "#,
  );
  cast!(denizen, IDenizenP::TopLevelFunction);
}
/*
  test("Function ending with set") {
    compileDenizenExpect(
      """
        |func moo() {
        |  set bork = value
        |}
        |""".stripMargin)
  }
*/
#[test]
fn extern_function_generated() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, r#"extern("bork") func sum();"#);
  let function = find_func_named(&program, "sum");
  let builtin = cast!(
    expect_1(&function.header.attributes),
    IAttributeP::BuiltinAttribute
  );
  assert_eq!(builtin.generator_name.as_str(), "bork");
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert!(function.header.ret.ret_type.is_none());
  assert!(function.body.is_none());
}
/*
  test("Extern function generated") {
    vassertOne(compileFile("extern(\"bork\") func sum();").getOrDie().denizens) match {
      case TopLevelFunctionP(FunctionP(_,
      FunctionHeaderP(_,
      Some(NameP(_, StrI("sum"))), Vector(BuiltinAttributeP(_, NameP(_, StrI("bork")))), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, None)),
      None)) =>
    }
  }
*/
#[test]
fn extern_function_with_return() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "extern func sum() int;");
  let function = find_func_named(&program, "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::ExternAttribute(_)]
  ));
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "int");
  assert!(function.body.is_none());
}
/*
  test("Extern function with return") {
    vassertOne(compileFile("extern func sum() int;").getOrDie().denizens) match {
      case TopLevelFunctionP(FunctionP(_,
      FunctionHeaderP(_,
      Some(NameP(_, StrI("sum"))), Vector(ExternAttributeP(_)), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, Some(NameOrRunePT(NameP(_, StrI("int")))))),
      None)) =>
    }
  }
*/
#[test]
fn abstract_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "abstract func sum();");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::AbstractAttribute(_)]
  ));
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  assert!(function.header.ret.ret_type.is_none());
  assert!(function.body.is_none());
}
/*
  test("Abstract function") {
    compileDenizen("abstract func sum();").getOrDie() match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))), Vector(AbstractAttributeP(_)), None, None, Some(ParamsP(_,Vector())), FunctionReturnP(_, None)),
        None)) =>
    }
  }
*/
#[test]
fn pure_and_default_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "pure func findNearbyUnits() i'int i'{ }",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(
    function.header.name.as_ref().unwrap().as_str(),
    "findNearbyUnits"
  );
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::PureAttribute(_)]
  ));
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  let ret_type = cast!(
    function.header.ret.ret_type.as_ref().unwrap(),
    ITemplexPT::Interpreted
  );
  assert!(ret_type.maybe_ownership.is_none());
  let ret_region = ret_type.maybe_region.as_ref().unwrap();
  assert_eq!(ret_region.name.as_ref().unwrap().as_str(), "i");
  assert_templex_name(ret_type.inner, "int");
  let body = function.body.as_ref().unwrap();
  let default_region = body.maybe_default_region.as_ref().unwrap();
  assert_eq!(default_region.name.as_ref().unwrap().as_str(), "i");
  assert!(matches!(body.inner, IExpressionPE::Void(_)));
}
/*
  test("Pure and default region") {
    compileDenizen("""pure func findNearbyUnits() i'int i'{ }""").getOrDie() match {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,
            Some(NameP(_,StrI("findNearbyUnits"))),
            Vector(PureAttributeP(_)),
            None,None,Some(ParamsP(_,Vector())),
            FunctionReturnP(_,Some(InterpretedPT(_,None,Some(RegionRunePT(_,Some(NameP(_,StrI("i"))))),NameOrRunePT(NameP(_,StrI("int"))))))),
          Some(BlockPE(_,None,Some(RegionRunePT(_,Some(NameP(_,StrI("i"))))),VoidPE(_))))) =>
    }
  }
*/
#[test]
fn return_isolate() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func findNearbyUnits() 'int { }");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(
    function.header.name.as_ref().unwrap().as_str(),
    "findNearbyUnits"
  );
  assert!(function.header.attributes.is_empty());
  assert!(function.header.params.as_ref().unwrap().params.is_empty());
  let ret_type = cast!(
    function.header.ret.ret_type.as_ref().unwrap(),
    ITemplexPT::Interpreted
  );
  assert!(ret_type.maybe_ownership.is_none());
  assert!(ret_type.maybe_region.is_some());
  assert!(ret_type.maybe_region.as_ref().unwrap().name.is_none());
  assert_templex_name(ret_type.inner, "int");
}
/*
  test("Return isolate") {
    compileDenizen("""func findNearbyUnits() 'int { }""").getOrDie() match {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,
            Some(NameP(_,StrI("findNearbyUnits"))),
            Vector(),
            None,None,Some(ParamsP(_,Vector())),
            FunctionReturnP(_,Some(InterpretedPT(_,None,Some(RegionRunePT(_,None)),NameOrRunePT(NameP(_,StrI("int"))))))),
          _)) =>
    }
  }
*/
#[test]
fn coord_generic_with_associated_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func findNearbyUnits<t', t'T>(x T) { }",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(
    function.header.name.as_ref().unwrap().as_str(),
    "findNearbyUnits"
  );
  let generic_params = &function.header.generic_parameters.as_ref().unwrap().params;
  let (first_param, second_param) = expect_2(generic_params);
  assert_eq!(first_param.name.as_str(), "t");
  assert_eq!(
    first_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(first_param.coord_region.is_none());
  assert!(first_param.attributes.is_empty());
  assert!(first_param.maybe_default.is_none());
  assert_eq!(second_param.name.as_str(), "T");
  assert!(second_param.maybe_type.is_none());
  assert_eq!(
    second_param
      .coord_region
      .as_ref()
      .unwrap()
      .name
      .as_ref()
      .unwrap()
      .str()
      .as_str(),
    "t"
  );
  assert!(second_param.attributes.is_empty());
  assert!(second_param.maybe_default.is_none());
}
/*
  test("Coord generic with associated region") {
    compileDenizen("""func findNearbyUnits<t', t'T>(x T) { }""").getOrDie() match {
      case TopLevelFunctionP(
        FunctionP(_,
        FunctionHeaderP(_,
        Some(NameP(_,StrI("findNearbyUnits"))),
        Vector(),
        Some(
          GenericParametersP(_,
          Vector(
          GenericParameterP(_,NameP(_,StrI("t")),Some(GenericParameterTypeP(_,RegionTypePR)),None,Vector(),None),
          GenericParameterP(_,NameP(_,StrI("T")),None,Some(RegionRunePT(_,Some(NameP(_,StrI("t"))))),Vector(),None)))),
        None,
        _,_),_)) =>
    }
  }
*/
#[test]
fn attribute_after_return() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "abstract func sum() int;");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::AbstractAttribute(_)]
  ));
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "int");
  assert!(function.body.is_none());
}
/*
  test("Attribute after return") {
    compileDenizen("abstract func sum() int;").getOrDie() match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))),
          Vector(AbstractAttributeP(_)),
          None,
          None, Some(ParamsP(_,Vector())),
          FunctionReturnP(_, Some(NameOrRunePT(NameP(_, StrI("int")))))),
        None)) =>
    }
  }
*/
#[test]
fn attribute_before_return() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "abstract func sum() Int;");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(matches!(
    function.header.attributes,
    [IAttributeP::AbstractAttribute(_)]
  ));
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "Int");
  assert!(function.body.is_none());
}
/*
  test("Attribute before return") {
    compileDenizen("abstract func sum() Int;").getOrDie() match {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))),
          Vector(AbstractAttributeP(_)),
          None,
          None, Some(ParamsP(_,Vector())),
          FunctionReturnP(_, Some(NameOrRunePT(NameP(_, StrI("Int")))))),
        None)) =>
    }
  }
*/
#[test]
fn simple_function_with_identifying_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<A>(a A){a}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "A");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Simple function with identifying rune") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<A>(a A){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_, NameP(_, StrI("A")), None, None, Vector(), None) =>
    }
  }
*/
#[test]
fn simple_function_with_coord_typed_identifying_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<A Ref>(a A){a}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "A");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::CoordType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Simple function with coord-typed identifying rune") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<A Ref>(a A){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_, NameP(_, StrI("A")), Some(GenericParameterTypeP(_, CoordTypePR)), None, Vector(), None) =>
    }
  }
*/
#[test]
fn simple_function_with_region_typed_identifying_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<a'>(){}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "a");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Simple function with region-typed identifying rune") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<a'>(){}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_, NameP(_, StrI("a")), Some(GenericParameterTypeP(_, RegionTypePR)), None, Vector(), None) =>
    }
  }
*/
#[test]
fn readonly_region_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<r' ro>(){}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "r");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(matches!(
    generic_param.attributes,
    [IRuneAttributeP::ReadOnlyRegionRuneAttribute(_)]
  ));
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Readonly region rune") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<r' ro>(){}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_, NameP(_, StrI("r")), Some(GenericParameterTypeP(_, RegionTypePR)), None, Vector(ReadOnlyRegionRuneAttributeP(_)), None) =>
    }
  }
*/
#[test]
fn simple_function_with_apostrophe_region_typed_identifying_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<r'>(a &r'Marine){a}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "r");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Simple function with apostrophe region-typed identifying rune") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<r'>(a &r'Marine){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_, NameP(_, StrI("r")), Some(GenericParameterTypeP(_, RegionTypePR)), None, Vector(), None) =>
    }
  }
*/
#[test]
fn pool_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func sum<r' = pool>(a &r'Marine){a}",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "r");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert_templex_name(generic_param.maybe_default.as_ref().unwrap(), "pool");
}
/*
  test("Pool region") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<r' = pool>(a &r'Marine){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_,
        NameP(_, StrI("r")),
        Some(GenericParameterTypeP(_, RegionTypePR)),
        None,
        Vector(),
        Some(NameOrRunePT(NameP(_,StrI("pool"))))) =>
    }
  }
*/
#[test]
fn pool_readonly_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func sum<r' ro = pool>(a &r'Marine){a}",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "r");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(matches!(
    generic_param.attributes,
    [IRuneAttributeP::ReadOnlyRegionRuneAttribute(_)]
  ));
  assert_templex_name(generic_param.maybe_default.as_ref().unwrap(), "pool");
}
/*
  test("Pool readonly region") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<r' ro = pool>(a &r'Marine){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_,
        NameP(_, StrI("r")),
        Some(GenericParameterTypeP(_, RegionTypePR)),
        None,
        Vector(ReadOnlyRegionRuneAttributeP(_)),
        Some(NameOrRunePT(NameP(_,StrI("pool"))))) =>
    }
  }
*/
#[test]
fn arena_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func sum<x' = arena>(a &x'Marine){a}",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "x");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert_templex_name(generic_param.maybe_default.as_ref().unwrap(), "arena");
}
/*
  test("Arena region") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<x' = arena>(a &x'Marine){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_,
        NameP(_, StrI("x")),
        Some(GenericParameterTypeP(_, RegionTypePR)),
        None,
        Vector(),
        Some(NameOrRunePT(NameP(_,StrI("arena"))))) =>
    }
  }
*/
#[test]
fn readonly_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum<x'>(a &x'Marine){a}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "x");
  assert_eq!(
    generic_param.maybe_type.as_ref().unwrap().tyype,
    ITypePR::RegionType
  );
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*

  test("Readonly region") {
    val TopLevelFunctionP(func) =
      compileDenizen("func sum<x'>(a &x'Marine){a}").getOrDie()
    func.header.genericParameters.get.params.head match {
      case GenericParameterP(_,
        NameP(_, StrI("x")),
        Some(GenericParameterTypeP(_, RegionTypePR)),
        None,
        Vector(),
        None) =>
    }
  }
*/
#[test]
fn virtual_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func doCivicDance(virtual this Car) int;",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(
    function.header.name.as_ref().unwrap().as_str(),
    "doCivicDance"
  );
  assert!(function.header.attributes.is_empty());
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  assert!(matches!(param.virtuality.as_ref(), Some(AbstractP { .. })));
  let pattern = param.pattern.as_ref().unwrap();
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "this");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Car");
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "int");
  assert!(function.body.is_none());
}
/*
  test("Virtual function") {
    compileDenizen("func doCivicDance(virtual this Car) int;".stripMargin).getOrDie() shouldHave {
      case TopLevelFunctionP(FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("doCivicDance"))), Vector(), None,
          None,
          Some(
            ParamsP(_,
              Vector(
                ParameterP(_,
                  Some(AbstractP(_)),
                  _,
                  _,
                  Some(
                    PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("this"))), None)),Some(NameOrRunePT(NameP(_, StrI("Car")))), _)))))),
          FunctionReturnP(_, Some(NameOrRunePT(NameP(_, StrI("int")))))),
        None)) =>
    }
  }
*/
#[test]
fn bad_thing_for_body() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_denizen(
    &parse_arena,
    &keywords,
    r#"
      func doCivicDance(virtual this Car) moo blork
    "#,
  )
  .unwrap_err();
  assert!(matches!(err, ParseError::BadFunctionBodyError(_)));
}
/*
  test("Bad thing for body") {
    compileDenizen(
        """
          |func doCivicDance(virtual this Car) moo blork
        """.stripMargin).expectErr().error match {
      case BadFunctionBodyError(_) =>
    }
  }

*/
#[test]
fn function_with_parameter_and_return() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "func main(moo T) T { }");
  let function = find_func_named(&program, "main");
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  let pattern = param.pattern.as_ref().unwrap();
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "moo");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "T");
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "T");
  assert!(matches!(
    function.body.as_ref().unwrap().inner,
    IExpressionPE::Void(_)
  ));
}
/*
  test("Function with parameter and return") {
    vassertOne(compileFileExpect("""func main(moo T) T { }""").denizens) shouldHave {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,
            Some(NameP(_,StrI("main"))),Vector(),None,None,
            Some(ParamsP(_,Vector(ParameterP(_,_,_,_,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("moo"))), None)),Some(NameOrRunePT(NameP(_,StrI("T")))),None)))))),
            FunctionReturnP(_,Some(NameOrRunePT(NameP(_,StrI("T")))))),
          Some(BlockPE(_,None, None,VoidPE(_))))) =>
    }
  }
*/
#[test]
fn function_with_generics() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let program = compile(&parse_arena, &keywords, "func main<T>() { }");
  let function = find_func_named(&program, "main");
  assert!(function.header.attributes.is_empty());
  assert!(function.header.template_rules.is_none());
  let generic_param = expect_1(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(generic_param.name.as_str(), "T");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert!(generic_param.attributes.is_empty());
  assert!(generic_param.maybe_default.is_none());
}
/*
  test("Function with generics") {
    vassertOne(compileFileExpect("""func main<T>() { }""").denizens) shouldHave {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,
            Some(NameP(_,StrI("main"))),
            Vector(),
            Some(GenericParametersP(_,Vector(GenericParameterP(_,NameP(_,StrI("T")),None,None, Vector(), None)))),
            None,
            _,
            _),
          _)) =>
    }
  }
*/
#[test]
fn impl_function() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func maxHp(virtual this Marine) { return 5; }",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "maxHp");
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  assert!(matches!(param.virtuality.as_ref(), Some(AbstractP { .. })));
  let pattern = param.pattern.as_ref().unwrap();
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "this");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "Marine");
  assert!(function.header.ret.ret_type.is_none());
  assert!(function.body.is_some());
}
/*
  test("Impl function") {
    compileDenizenExpect(
      "func maxHp(virtual this Marine) { return 5; }") shouldHave {
      case FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("maxHp"))),Vector(), None, None,
          Some(
            ParamsP(
              _,
              Vector(
                ParameterP(_,
                  Some(AbstractP(_)),
                  _,
                  _,
                  Some(
                    PatternPP(_,
                      Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("this"))), None)),
                      Some(NameOrRunePT(NameP(_, StrI("Marine")))),
                      None)))))),
          FunctionReturnP(_, None)),
        Some(BlockPE(_, None, None, _))) =>
    }
  }
*/
#[test]
fn param() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func call(f F){f()}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  let pattern = param.pattern.as_ref().unwrap();
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "f");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "F");
}
/*
  test("Param") {
    val program = compileDenizenExpect("func call(f F){f()}")
    program shouldHave {
      case PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("f"))), None)),Some(NameOrRunePT(NameP(_, StrI("F")))),None) =>
    }
  }
*/
#[test]
fn func_with_rules() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func sum () where X Int {3}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "sum");
  assert!(function.header.attributes.is_empty());
  assert!(function.header.generic_parameters.is_none());
  assert!(function.header.template_rules.is_some());
  assert!(function.header.params.is_some());
  assert!(function.header.ret.ret_type.is_none());
  let body = function.body.as_ref().unwrap();
  assert_eq!(
    cast!(body.inner, IExpressionPE::ConstantInt).value,
    3
  );
}
/*
  test("Func with rules") {
    compileDenizenExpect(
      "func sum () where X Int {3}") shouldHave {
      case FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("sum"))), Vector(), None, Some(_), Some(_), FunctionReturnP(_, None)),
        Some(BlockPE(_, None, None, ConstantIntPE(_, 3, _)))) =>
    }
  }
*/
#[test]
fn func_with_func_bound() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    "func sum<T>() where func moo(&T)void {3}",
  );
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let template_rules = function.header.template_rules.as_ref().unwrap();
  let first_rule = expect_1(&template_rules.rules);
  let first_rule_templex = cast!(first_rule, IRulexPR::Templex);
  let function_bound = cast!(first_rule_templex, ITemplexPT::Func);
  assert_eq!(function_bound.name.as_str(), "moo");
  let first_param = expect_1(&function_bound.parameters);
  let interpreted = cast!(first_param, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().1,
    OwnershipP::Borrow
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner, "T");
  assert_templex_name(function_bound.return_type, "void");
}
/*
  test("Func with func bound") {
    compileDenizenExpect(
      "func sum<T>() where func moo(&T)void {3}") shouldHave {
      case TopLevelFunctionP(
        FunctionP(_,
          FunctionHeaderP(_,_,_,_,
            Some(
              TemplateRulesP(_,
                Vector(
                  TemplexPR(
                    FuncPT(_,
                      NameP(_,StrI("moo")),
                      _,
                      Vector(InterpretedPT(_,Some(OwnershipPT(_, BorrowP)),_,NameOrRunePT(NameP(_,StrI("T"))))),
                      NameOrRunePT(NameP(_,StrI("void")))))))),
          _,_),_)) =>
    }
  }
*/
#[test]
fn identifying_runes() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func wrap<A, F>(a A) { }");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  let (a_rune, f_rune) = expect_2(&function.header.generic_parameters.as_ref().unwrap().params);
  assert_eq!(a_rune.name.as_str(), "A");
  assert!(a_rune.maybe_type.is_none());
  assert!(a_rune.coord_region.is_none());
  assert!(a_rune.attributes.is_empty());
  assert!(a_rune.maybe_default.is_none());
  assert_eq!(f_rune.name.as_str(), "F");
  assert!(f_rune.maybe_type.is_none());
  assert!(f_rune.coord_region.is_none());
  assert!(f_rune.attributes.is_empty());
  assert!(f_rune.maybe_default.is_none());
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  let pattern = param.pattern.as_ref().unwrap();
  assert_destination_local_name(pattern.destination.as_ref().unwrap(), "a");
  assert_templex_name(pattern.templex.as_ref().unwrap(), "A");
}
/*


  test("Identifying runes") {
    compileDenizenExpect(
      "func wrap<A, F>(a A) { }") shouldHave {
      case FunctionP(_,
        FunctionHeaderP(_,
          Some(NameP(_, StrI("wrap"))), Vector(),
          Some(
            GenericParametersP(_,
              Vector(
              GenericParameterP(_, NameP(_, StrI("A")), None, None, Vector(), None),
              GenericParameterP(_, NameP(_, StrI("F")), None, None, Vector(), None)))),
          None,
          Some(ParamsP(_, Vector(ParameterP(_, _, _, _, Some(Patterns.capturedWithTypeRune("a", "A")))))),
          FunctionReturnP(_, None)),
        Some(BlockPE(_, None, None, VoidPE(_)))) =>
    }
  }
*/
#[test]
fn never_signature() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(&parse_arena, &keywords, "func __vbi_panic() __Never {}");
  let function = cast!(denizen, IDenizenP::TopLevelFunction);
  assert_templex_name(function.header.ret.ret_type.as_ref().unwrap(), "__Never");
}
/*
  test("Never signature") {
    // This test is here because we were parsing the first _ of __Never as an anonymous
    // rune then stopping.
    compileDenizenExpect(
      "func __vbi_panic() __Never {}") shouldHave {
      case NameOrRunePT(NameP(_, StrI("__Never"))) =>
    }
  }
*/
#[test]
fn should_require_identifying_runes() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_denizen(
    &parse_arena,
    &keywords,
    r#"
      func do(callable) int {callable()}
    "#,
  )
  .unwrap_err();
  assert!(matches!(
    err,
    ParseError::LightFunctionMustHaveParamTypes { param_index: 0, .. }
  ));
}
/*
  test("Should require identifying runes") {
    val error =
      compileDenizen(
        """
          |func do(callable) int {callable()}
          |""".stripMargin).expectErr().error
    error match {
      case LightFunctionMustHaveParamTypes(_, 0) =>
    }
  }
*/
#[test]
fn short_self() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let denizen = compile_denizen_expect(
    &parse_arena,
    &keywords,
    r#"
      interface IMoo {
        func moo(&self) {}
      }
    "#,
  );
  let interface = cast!(denizen, IDenizenP::TopLevelInterface);
  assert_eq!(interface.name.as_str(), "IMoo");
  let function = &interface.members[0];
  assert_eq!(function.header.name.as_ref().unwrap().as_str(), "moo");
  let param = expect_1(&function.header.params.as_ref().unwrap().params);
  assert!(param.virtuality.is_none());
  assert!(param.self_borrow.is_some());
  assert!(param.pattern.is_none());
}
/*
  test("Short self") {
    compileDenizenExpect(
      """
        |interface IMoo {
        |  func moo(&self) {}
        |}
        |""".stripMargin) shouldHave {
      case TopLevelInterfaceP(
        InterfaceP(_,
          NameP(_,StrI("IMoo")),Vector(),None,None,None,_,_,
          Vector(
            FunctionP(_,
              FunctionHeaderP(_,
                Some(NameP(_, StrI("moo"))),
                Vector(),None,None,
                Some(
                  ParamsP(_,
                    Vector(
                      ParameterP(_,None,None,Some(_),None)))),
                _),
              _)))) =>
    }
  }
}
*/
