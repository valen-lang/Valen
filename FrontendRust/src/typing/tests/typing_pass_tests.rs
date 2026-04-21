use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::typing::TypingPassCompilation;
use crate::utils::code_hierarchy::{self, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;

/*
package dev.vale.typing

import dev.vale.typing.TypingPassCompilation
import org.scalatest._

class TypingPassTests extends FunSuite with Matchers  {
*/

fn compile_program_to_hinputs<'s, 'ctx, 't, 'p>(
    compilation: &mut TypingPassCompilation<'s, 'ctx, 't, 'p>,
) -> ()
{
    match compilation.get_compiler_outputs() {
        Ok(_result) => { /* test passed */ },
        Err(err) => panic!("Expected to compile successfully, but got error:\n{:?}", err),
    }
}

/*
  def compileProgramToHinputs(compilation: TypingPassCompilation): HinputsT = {
    compilation.getCompilerOutputs() match {
      case Ok(result) => result
      case Err(err) => vfail("Expected to compile successfully, but got error:\n" + err)
    }
  }
*/

fn setup_test<'s, 'ctx, 'p>(
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
) -> TypingPassCompilation<'s, 'ctx, 's, 'p> {
    let options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: false,
        debug_output: false,
    };
    let test_module = parse_arena.intern_str("test");
    let test_package_ref = parse_arena.intern_package_coordinate(test_module, &[]);

    TypingPassCompilation::new(
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        vec![test_package_ref],
        resolver,
        options,
        crate::instantiating::InstantiatorCompilationOptions {
            debug_out: std::sync::Arc::new(|_| {}),
        },
    )
}

/*
  def setupTest(): (HigherTypingCompilation, FileCoordinateMap) = {
    val parseArena = new ParseArena()
    val scoutArena = new ScoutArena()
    val keywords = Keywords.ENGLISH

    val options = GlobalOptions(sanityCheck = true, useOverloadIndex = true, useOptimizedSolver = true, verboseErrors = false)
    val testModule = parseArena.intern_str("test")
    val testTldRef = parseArena.intern_package_coordinate(testModule, Vector.empty)
    HigherTypingCompilation.new(
      options, testTldRef, parseArena, scoutArena, keywords, Vector(testTldRef), new IPackageResolver[Map[String, String]] {
        override def getPackageContents(packageCoord: PackageCoordinate): Result[Map[String, String], String] = {
          Ok(Map())
        }
      })
  }
*/

#[test]
fn test_simple_void_function() {
    let bump = Bump::new();
    let parse_arena = ParseArena::new(&bump);
    let scout_bump = Bump::new();
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);

    struct DummyResolver;
    impl<'a> IPackageResolver<'a, HashMap<String, String>> for DummyResolver {
        fn resolve(&self, _package_coord: &'a PackageCoordinate<'a>) -> Option<HashMap<String, String>> {
            Some(HashMap::new())
        }
    }

    let resolver = &DummyResolver;
    let mut compilation = setup_test(
        &scout_arena,
        &keywords,
        &parser_keywords,
        &parse_arena,
        resolver,
    );

    compile_program_to_hinputs(&mut compilation);
}

/*
  test("Simple void function") {
    val (compilation, codeMap) = setupTest()

    val hinputs = compileProgramToHinputs(compilation)
  }
*/
