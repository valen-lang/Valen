#!/usr/bin/env python3
"""
Verify that all original Scala code from Frontend/ is still present
as block comments (/* ... */) in the corresponding FrontendRust/ files.

For each mapped file pair:
  1. Extract all block comment contents from the Rust file
  2. Filter out Guardian: lines (annotations added during migration)
  3. Normalize both sides (strip leading whitespace, collapse blank lines)
  4. Diff them

Lines present in the original Scala but missing from block comments = Scala code was removed.
Lines present in block comments but missing from the original = additions (MIGALLOW, etc).
Lines that differ = Scala code was modified in the block comment.
"""

import re
import sys
import os
import difflib

FRONTEND = os.path.join(os.path.dirname(__file__), '..', '..', 'Frontend')
FRONTEND_RUST = os.path.join(os.path.dirname(__file__), '..')

# (rust_file_relative_to_FrontendRust, scala_file_relative_to_Frontend)
FILE_MAP = [
    # === Lexing ===
    ("src/lexing/ast.rs", "LexingPass/src/dev/vale/lexing/ast.scala"),
    ("src/lexing/errors.rs", "LexingPass/src/dev/vale/lexing/errors.scala"),
    ("src/lexing/lex_and_explore.rs", "LexingPass/src/dev/vale/lexing/LexAndExplore.scala"),
    ("src/lexing/lexer.rs", "LexingPass/src/dev/vale/lexing/Lexer.scala"),
    ("src/lexing/lexing_iterator.rs", "LexingPass/src/dev/vale/lexing/LexingIterator.scala"),

    # === Parsing (src) ===
    ("src/parsing/ast/ast.rs", "ParsingPass/src/dev/vale/parsing/ast/ast.scala"),
    ("src/parsing/ast/expressions.rs", "ParsingPass/src/dev/vale/parsing/ast/expressions.scala"),
    ("src/parsing/ast/pattern.rs", "ParsingPass/src/dev/vale/parsing/ast/pattern.scala"),
    ("src/parsing/ast/rules.rs", "ParsingPass/src/dev/vale/parsing/ast/rules.scala"),
    ("src/parsing/ast/templex.rs", "ParsingPass/src/dev/vale/parsing/ast/templex.scala"),
    ("src/parsing/expression_parser.rs", "ParsingPass/src/dev/vale/parsing/ExpressionParser.scala"),
    ("src/parsing/formatter.rs", "ParsingPass/src/dev/vale/parsing/Formatter.scala"),
    ("src/parsing/parse_and_explore.rs", "ParsingPass/src/dev/vale/parsing/ParseAndExplore.scala"),
    ("src/parsing/parse_error_humanizer.rs", "ParsingPass/src/dev/vale/parsing/ParseErrorHumanizer.scala"),
    ("src/parsing/parse_utils.rs", "ParsingPass/src/dev/vale/parsing/ParseUtils.scala"),
    ("src/parsing/parsed_loader.rs", "ParsingPass/src/dev/vale/parsing/ParsedLoader.scala"),
    ("src/parsing/parser.rs", "ParsingPass/src/dev/vale/parsing/Parser.scala"),
    ("src/parsing/pattern_parser.rs", "ParsingPass/src/dev/vale/parsing/PatternParser.scala"),
    ("src/parsing/string_parser.rs", "ParsingPass/src/dev/vale/parsing/expressions/StringParser.scala"),
    ("src/parsing/templex_parser.rs", "ParsingPass/src/dev/vale/parsing/templex/TemplexParser.scala"),
    ("src/parsing/vonifier.rs", "ParsingPass/src/dev/vale/parsing/ParserVonifier.scala"),

    # === Parsing (tests) ===
    ("src/parsing/tests/after_regions_tests.rs", "ParsingPass/test/dev/vale/parsing/AfterRegionsTests.scala"),
    ("src/parsing/tests/expression_tests.rs", "ParsingPass/test/dev/vale/parsing/ExpressionTests.scala"),
    ("src/parsing/tests/if_tests.rs", "ParsingPass/test/dev/vale/parsing/IfTests.scala"),
    ("src/parsing/tests/impl_tests.rs", "ParsingPass/test/dev/vale/parsing/ImplTests.scala"),
    ("src/parsing/tests/load_tests.rs", "ParsingPass/test/dev/vale/parsing/LoadTests.scala"),
    ("src/parsing/tests/parse_samples_tests.rs", "ParsingPass/test/dev/vale/parsing/ParseSamplesTests.scala"),
    ("src/parsing/tests/parser_test_compilation.rs", "ParsingPass/test/dev/vale/parsing/ParserTestCompilation.scala"),
    ("src/parsing/tests/statement_tests.rs", "ParsingPass/test/dev/vale/parsing/StatementTests.scala"),
    ("src/parsing/tests/struct_tests.rs", "ParsingPass/test/dev/vale/parsing/StructTests.scala"),
    ("src/parsing/tests/top_level_tests.rs", "ParsingPass/test/dev/vale/parsing/TopLevelTests.scala"),
    ("src/parsing/tests/while_tests.rs", "ParsingPass/test/dev/vale/parsing/WhileTests.scala"),
    ("src/parsing/tests/functions/after_regions_function_tests.rs", "ParsingPass/test/dev/vale/parsing/functions/AfterRegionsFunctionTests.scala"),
    ("src/parsing/tests/functions/function_tests.rs", "ParsingPass/test/dev/vale/parsing/functions/FunctionTests.scala"),
    ("src/parsing/tests/patterns/capture_and_destructure_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/CaptureAndDestructureTests.scala"),
    ("src/parsing/tests/patterns/capture_and_type_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/CaptureAndTypeTests.scala"),
    ("src/parsing/tests/patterns/destructure_parser_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/DestructureParserTests.scala"),
    ("src/parsing/tests/patterns/pattern_parser_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/PatternParserTests.scala"),
    ("src/parsing/tests/patterns/type_and_destructure_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/TypeAndDestructureTests.scala"),
    ("src/parsing/tests/patterns/type_tests.rs", "ParsingPass/test/dev/vale/parsing/patterns/TypeTests.scala"),
    ("src/parsing/tests/rules/coord_rule_tests.rs", "ParsingPass/test/dev/vale/parsing/rules/CoordRuleTests.scala"),
    ("src/parsing/tests/rules/kind_rule_tests.rs", "ParsingPass/test/dev/vale/parsing/rules/KindRuleTests.scala"),
    ("src/parsing/tests/rules/rule_tests.rs", "ParsingPass/test/dev/vale/parsing/rules/RuleTests.scala"),
    ("src/parsing/tests/rules/rules_enums_tests.rs", "ParsingPass/test/dev/vale/parsing/rules/RulesEnumsTests.scala"),

    # === PostParsing (src) ===
    ("src/postparsing/ast.rs", "PostParsingPass/src/dev/vale/postparsing/ast.scala"),
    ("src/postparsing/expression_scout.rs", "PostParsingPass/src/dev/vale/postparsing/ExpressionScout.scala"),
    ("src/postparsing/expressions.rs", "PostParsingPass/src/dev/vale/postparsing/expressions.scala"),
    ("src/postparsing/function_scout.rs", "PostParsingPass/src/dev/vale/postparsing/FunctionScout.scala"),
    ("src/postparsing/identifiability_solver.rs", "PostParsingPass/src/dev/vale/postparsing/IdentifiabilitySolver.scala"),
    ("src/postparsing/itemplatatype.rs", "PostParsingPass/src/dev/vale/postparsing/ITemplataType.scala"),
    ("src/postparsing/loop_post_parser.rs", "PostParsingPass/src/dev/vale/postparsing/LoopPostParser.scala"),
    ("src/postparsing/names.rs", "PostParsingPass/src/dev/vale/postparsing/names.scala"),
    ("src/postparsing/post_parser.rs", "PostParsingPass/src/dev/vale/postparsing/PostParser.scala"),
    ("src/postparsing/post_parser_error_humanizer.rs", "PostParsingPass/src/dev/vale/postparsing/PostParserErrorHumanizer.scala"),
    ("src/postparsing/rune_type_solver.rs", "PostParsingPass/src/dev/vale/postparsing/RuneTypeSolver.scala"),
    ("src/postparsing/variable_uses.rs", "PostParsingPass/src/dev/vale/postparsing/VariableUses.scala"),
    ("src/postparsing/patterns/pattern_scout.rs", "PostParsingPass/src/dev/vale/postparsing/patterns/PatternScout.scala"),
    ("src/postparsing/patterns/patterns.rs", "PostParsingPass/src/dev/vale/postparsing/patterns/patterns.scala"),
    ("src/postparsing/rules/rule_scout.rs", "PostParsingPass/src/dev/vale/postparsing/rules/RuleScout.scala"),
    ("src/postparsing/rules/rules.rs", "PostParsingPass/src/dev/vale/postparsing/rules/rules.scala"),
    ("src/postparsing/rules/templex_scout.rs", "PostParsingPass/src/dev/vale/postparsing/rules/TemplexScout.scala"),

    # === PostParsing (tests) ===
    ("src/postparsing/test/post_parser_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserTests.scala"),
    ("src/postparsing/test/post_parser_variable_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserVariableTests.scala"),
    ("src/postparsing/test/post_parsing_parameters_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParsingParametersTests.scala"),
    ("src/postparsing/test/post_parsing_rule_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParsingRuleTests.scala"),
    ("src/postparsing/test/post_parser_error_humanizer_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserErrorHumanizerTests.scala"),
    ("src/postparsing/test/after_regions_error_tests.rs", "PostParsingPass/test/dev/vale/postparsing/AfterRegionsErrorTests.scala"),
    ("src/postparsing/test/post_parser_test_compilation.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserTestCompilation.scala"),

    # === Higher Typing ===
    ("src/higher_typing/ast.rs", "HigherTypingPass/src/dev/vale/highertyping/ast.scala"),
    ("src/higher_typing/higher_typing_pass.rs", "HigherTypingPass/src/dev/vale/highertyping/HigherTypingPass.scala"),
    ("src/higher_typing/astronomer_error_reporter.rs", "HigherTypingPass/src/dev/vale/highertyping/AstronomerErrorReporter.scala"),
    ("src/higher_typing/higher_typing_error_humanizer.rs", "HigherTypingPass/src/dev/vale/highertyping/HigherTypingErrorHumanizer.scala"),
    ("src/higher_typing/patterns.rs", "HigherTypingPass/src/dev/vale/highertyping/patterns.scala"),
    ("src/higher_typing/textifier.rs", "HigherTypingPass/src/dev/vale/highertyping/Textifier.scala"),
    ("src/higher_typing/tests/error_tests.rs", "HigherTypingPass/test/dev/vale/highertyping/ErrorTests.scala"),
    ("src/higher_typing/tests/higher_typing_pass_tests.rs", "HigherTypingPass/test/dev/vale/highertyping/HigherTypingPassTests.scala"),
    ("src/higher_typing/tests/test_compilation.rs", "HigherTypingPass/test/dev/vale/highertyping/HigherTypingTestCompilation.scala"),

    # === Solver ===
    ("src/solver/solver.rs", "Solver/src/dev/vale/solver/Solver.scala"),
    ("src/solver/i_solver_state.rs", "Solver/src/dev/vale/solver/ISolverState.scala"),
    ("src/solver/optimized_solver_state.rs", "Solver/src/dev/vale/solver/OptimizedSolverState.scala"),
    ("src/solver/simple_solver_state.rs", "Solver/src/dev/vale/solver/SimpleSolverState.scala"),
    ("src/solver/solver_error_humanizer.rs", "Solver/src/dev/vale/solver/SolverErrorHumanizer.scala"),

    # === Utils ===
    ("src/utils/code_hierarchy.rs", "Utils/src/dev/vale/CodeHierarchy.scala"),
    ("src/utils/collector.rs", "Utils/src/dev/vale/Collector.scala"),
    ("src/utils/vassert.rs", "Utils/src/dev/vale/vassert.scala"),
    ("src/utils/vpass.rs", "Utils/src/dev/vale/vpass.scala"),
    ("src/utils/accumulator.rs", "Utils/src/dev/vale/Accumulator.scala"),
    ("src/utils/result.rs", "Utils/src/dev/vale/Result.scala"),
    ("src/utils/range.rs", "Utils/src/dev/vale/Range.scala"),
    ("src/utils/repeat_str.rs", "Utils/src/dev/vale/repeatStr.scala"),
    ("src/utils/source_code_utils.rs", "Utils/src/dev/vale/SourceCodeUtils.scala"),
    ("src/utils/timer.rs", "Utils/src/dev/vale/Timer.scala"),
    ("src/utils/utils.rs", "Utils/src/dev/vale/Utils.scala"),
    ("src/utils/profiler.rs", "Utils/src/dev/vale/Profiler.scala"),
    ("src/utils/interner.rs", "Utils/src/dev/vale/Interner.scala"),
    ("src/utils/keywords.rs", "Utils/src/dev/vale/Keywords.scala"),

    # === Von ===
    ("src/von/ast.rs", "Von/src/dev/vale/von/VonAst.scala"),
    ("src/von/printer.rs", "Von/src/dev/vale/von/VonPrinter.scala"),

    # === Pass Manager ===
    ("src/pass_manager/full_compilation.rs", "PassManager/src/dev/vale/passmanager/FullCompilation.scala"),
    ("src/pass_manager/pass_manager.rs", "PassManager/src/dev/vale/passmanager/PassManager.scala"),

    # === Compile Options ===
    ("src/compile_options/compile_options.rs", "CompileOptions/src/dev/vale/options/GlobalOptions.scala"),

    # === Builtins ===
    ("src/builtins/builtins.rs", "Builtins/src/dev/vale/Builtins.scala"),

    # === Highlighter ===
    ("src/highlighter/highlighter.rs", "Highlighter/src/dev/vale/highlighter/Highlighter.scala"),
    ("src/highlighter/spanner.rs", "Highlighter/src/dev/vale/highlighter/Spanner.scala"),
    ("src/highlighter/tests/highlighter_tests.rs", "Highlighter/test/dev/vale/highlighter/HighlighterTests.scala"),
    ("src/highlighter/tests/spanner_tests.rs", "Highlighter/test/dev/vale/highlighter/SpannerTests.scala"),

    # === Final AST ===
    ("src/final_ast/ast.rs", "FinalAST/src/dev/vale/finalast/ast.scala"),
    ("src/final_ast/instructions.rs", "FinalAST/src/dev/vale/finalast/instructions.scala"),
    ("src/final_ast/types.rs", "FinalAST/src/dev/vale/finalast/types.scala"),
    ("src/final_ast/metal_printer.rs", "FinalAST/src/dev/vale/finalast/MetalPrinter.scala"),

    # === Instantiating ===
    ("src/instantiating/instantiated_compilation.rs", "InstantiatingPass/src/dev/vale/instantiating/InstantiatedCompilation.scala"),
    ("src/instantiating/instantiated_humanizer.rs", "InstantiatingPass/src/dev/vale/instantiating/InstantiatedHumanizer.scala"),
    ("src/instantiating/instantiator.rs", "InstantiatingPass/src/dev/vale/instantiating/Instantiator.scala"),
    ("src/instantiating/region_collapser_consistent.rs", "InstantiatingPass/src/dev/vale/instantiating/RegionCollapserConsistent.scala"),
    ("src/instantiating/region_collapser_individual.rs", "InstantiatingPass/src/dev/vale/instantiating/RegionCollapserIndividual.scala"),
    ("src/instantiating/region_counter.rs", "InstantiatingPass/src/dev/vale/instantiating/RegionCounter.scala"),
    ("src/instantiating/ast/ast.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/ast.scala"),
    ("src/instantiating/ast/citizens.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/citizens.scala"),
    ("src/instantiating/ast/expressions.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/expressions.scala"),
    ("src/instantiating/ast/hinputs.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/HinputsI.scala"),
    ("src/instantiating/ast/names.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/names.scala"),
    ("src/instantiating/ast/templata.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/templata.scala"),
    ("src/instantiating/ast/templata_utils.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/TemplataUtils.scala"),
    ("src/instantiating/ast/types.rs", "InstantiatingPass/src/dev/vale/instantiating/ast/types.scala"),
    ("src/instantiating/tests/instantiated_tests.rs", "InstantiatingPass/test/dev/vale/instantiating/InstantiatedTests.scala"),

    # === Simplifying ===
    ("src/simplifying/block_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/BlockHammer.scala"),
    ("src/simplifying/conversions.rs", "SimplifyingPass/src/dev/vale/simplifying/Conversions.scala"),
    ("src/simplifying/expression_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/ExpressionHammer.scala"),
    ("src/simplifying/function_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/FunctionHammer.scala"),
    ("src/simplifying/hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/Hammer.scala"),
    ("src/simplifying/hammer_compilation.rs", "SimplifyingPass/src/dev/vale/simplifying/HammerCompilation.scala"),
    ("src/simplifying/hamuts.rs", "SimplifyingPass/src/dev/vale/simplifying/Hamuts.scala"),
    ("src/simplifying/let_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/LetHammer.scala"),
    ("src/simplifying/load_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/LoadHammer.scala"),
    ("src/simplifying/mutate_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/MutateHammer.scala"),
    ("src/simplifying/name_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/NameHammer.scala"),
    ("src/simplifying/struct_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/StructHammer.scala"),
    ("src/simplifying/type_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/TypeHammer.scala"),
    ("src/simplifying/von_hammer.rs", "SimplifyingPass/src/dev/vale/simplifying/VonHammer.scala"),

    # === Typing (src) ===
    ("src/typing/array_compiler.rs", "TypingPass/src/dev/vale/typing/ArrayCompiler.scala"),
    ("src/typing/compilation.rs", "TypingPass/src/dev/vale/typing/Compilation.scala"),
    ("src/typing/compiler.rs", "TypingPass/src/dev/vale/typing/Compiler.scala"),
    ("src/typing/compiler_error_humanizer.rs", "TypingPass/src/dev/vale/typing/CompilerErrorHumanizer.scala"),
    ("src/typing/compiler_error_reporter.rs", "TypingPass/src/dev/vale/typing/CompilerErrorReporter.scala"),
    ("src/typing/compiler_outputs.rs", "TypingPass/src/dev/vale/typing/CompilerOutputs.scala"),
    ("src/typing/convert_helper.rs", "TypingPass/src/dev/vale/typing/ConvertHelper.scala"),
    ("src/typing/edge_compiler.rs", "TypingPass/src/dev/vale/typing/EdgeCompiler.scala"),
    ("src/typing/hinputs_t.rs", "TypingPass/src/dev/vale/typing/HinputsT.scala"),
    ("src/typing/infer_compiler.rs", "TypingPass/src/dev/vale/typing/InferCompiler.scala"),
    ("src/typing/overload_resolver.rs", "TypingPass/src/dev/vale/typing/OverloadResolver.scala"),
    ("src/typing/reachability.rs", "TypingPass/src/dev/vale/typing/Reachability.scala"),
    ("src/typing/sequence_compiler.rs", "TypingPass/src/dev/vale/typing/SequenceCompiler.scala"),
    ("src/typing/templata_compiler.rs", "TypingPass/src/dev/vale/typing/TemplataCompiler.scala"),
    ("src/typing/ast/ast.rs", "TypingPass/src/dev/vale/typing/ast/ast.scala"),
    ("src/typing/ast/citizens.rs", "TypingPass/src/dev/vale/typing/ast/citizens.scala"),
    ("src/typing/ast/expressions.rs", "TypingPass/src/dev/vale/typing/ast/expressions.scala"),
    ("src/typing/citizen/impl_compiler.rs", "TypingPass/src/dev/vale/typing/citizen/ImplCompiler.scala"),
    ("src/typing/citizen/struct_compiler.rs", "TypingPass/src/dev/vale/typing/citizen/StructCompiler.scala"),
    ("src/typing/citizen/struct_compiler_core.rs", "TypingPass/src/dev/vale/typing/citizen/StructCompilerCore.scala"),
    ("src/typing/citizen/struct_compiler_generic_args_layer.rs", "TypingPass/src/dev/vale/typing/citizen/StructCompilerGenericArgsLayer.scala"),
    ("src/typing/env/environment.rs", "TypingPass/src/dev/vale/typing/env/Environment.scala"),
    ("src/typing/env/function_environment_t.rs", "TypingPass/src/dev/vale/typing/env/FunctionEnvironmentT.scala"),
    ("src/typing/env/i_env_entry.rs", "TypingPass/src/dev/vale/typing/env/IEnvEntry.scala"),
    ("src/typing/expression/block_compiler.rs", "TypingPass/src/dev/vale/typing/expression/BlockCompiler.scala"),
    ("src/typing/expression/call_compiler.rs", "TypingPass/src/dev/vale/typing/expression/CallCompiler.scala"),
    ("src/typing/expression/expression_compiler.rs", "TypingPass/src/dev/vale/typing/expression/ExpressionCompiler.scala"),
    ("src/typing/expression/local_helper.rs", "TypingPass/src/dev/vale/typing/expression/LocalHelper.scala"),
    ("src/typing/expression/pattern_compiler.rs", "TypingPass/src/dev/vale/typing/expression/PatternCompiler.scala"),
    ("src/typing/function/destructor_compiler.rs", "TypingPass/src/dev/vale/typing/function/DestructorCompiler.scala"),
    ("src/typing/function/function_body_compiler.rs", "TypingPass/src/dev/vale/typing/function/FunctionBodyCompiler.scala"),
    ("src/typing/function/function_compiler.rs", "TypingPass/src/dev/vale/typing/function/FunctionCompiler.scala"),
    ("src/typing/function/function_compiler_closure_or_light_layer.rs", "TypingPass/src/dev/vale/typing/function/FunctionCompilerClosureOrLightLayer.scala"),
    ("src/typing/function/function_compiler_core.rs", "TypingPass/src/dev/vale/typing/function/FunctionCompilerCore.scala"),
    ("src/typing/function/function_compiler_middle_layer.rs", "TypingPass/src/dev/vale/typing/function/FunctionCompilerMiddleLayer.scala"),
    ("src/typing/function/function_compiler_solving_layer.rs", "TypingPass/src/dev/vale/typing/function/FunctionCompilerSolvingLayer.scala"),
    ("src/typing/function/virtual_compiler.rs", "TypingPass/src/dev/vale/typing/function/VirtualCompiler.scala"),
    ("src/typing/infer/compiler_solver.rs", "TypingPass/src/dev/vale/typing/infer/CompilerSolver.scala"),
    ("src/typing/macros/abstract_body_macro.rs", "TypingPass/src/dev/vale/typing/macros/AbstractBodyMacro.scala"),
    ("src/typing/macros/anonymous_interface_macro.rs", "TypingPass/src/dev/vale/typing/macros/AnonymousInterfaceMacro.scala"),
    ("src/typing/macros/as_subtype_macro.rs", "TypingPass/src/dev/vale/typing/macros/AsSubtypeMacro.scala"),
    ("src/typing/macros/functor_helper.rs", "TypingPass/src/dev/vale/typing/macros/FunctorHelper.scala"),
    ("src/typing/macros/lock_weak_macro.rs", "TypingPass/src/dev/vale/typing/macros/LockWeakMacro.scala"),
    ("src/typing/macros/macros.rs", "TypingPass/src/dev/vale/typing/macros/macros.scala"),
    ("src/typing/macros/rsa_len_macro.rs", "TypingPass/src/dev/vale/typing/macros/RSALenMacro.scala"),
    ("src/typing/macros/same_instance_macro.rs", "TypingPass/src/dev/vale/typing/macros/SameInstanceMacro.scala"),
    ("src/typing/macros/struct_constructor_macro.rs", "TypingPass/src/dev/vale/typing/macros/StructConstructorMacro.scala"),
    ("src/typing/macros/citizen/interface_drop_macro.rs", "TypingPass/src/dev/vale/typing/macros/citizen/InterfaceDropMacro.scala"),
    ("src/typing/macros/citizen/struct_drop_macro.rs", "TypingPass/src/dev/vale/typing/macros/citizen/StructDropMacro.scala"),
    ("src/typing/macros/rsa/rsa_drop_into_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSADropIntoMacro.scala"),
    ("src/typing/macros/rsa/rsa_immutable_new_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSAImmutableNewMacro.scala"),
    ("src/typing/macros/rsa/rsa_len_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSALenMacro.scala"),
    ("src/typing/macros/rsa/rsa_mutable_capacity_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSAMutableCapacityMacro.scala"),
    ("src/typing/macros/rsa/rsa_mutable_new_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSAMutableNewMacro.scala"),
    ("src/typing/macros/rsa/rsa_mutable_pop_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSAMutablePopMacro.scala"),
    ("src/typing/macros/rsa/rsa_mutable_push_macro.rs", "TypingPass/src/dev/vale/typing/macros/rsa/RSAMutablePushMacro.scala"),
    ("src/typing/macros/ssa/ssa_drop_into_macro.rs", "TypingPass/src/dev/vale/typing/macros/ssa/SSADropIntoMacro.scala"),
    ("src/typing/macros/ssa/ssa_len_macro.rs", "TypingPass/src/dev/vale/typing/macros/ssa/SSALenMacro.scala"),
    ("src/typing/names/name_translator.rs", "TypingPass/src/dev/vale/typing/names/NameTranslator.scala"),
    ("src/typing/names/names.rs", "TypingPass/src/dev/vale/typing/names/names.scala"),
    ("src/typing/templata/conversions.rs", "TypingPass/src/dev/vale/typing/templata/Conversions.scala"),
    ("src/typing/templata/templata.rs", "TypingPass/src/dev/vale/typing/templata/templata.scala"),
    ("src/typing/templata/templata_utils.rs", "TypingPass/src/dev/vale/typing/templata/TemplataUtils.scala"),
    ("src/typing/types/types.rs", "TypingPass/src/dev/vale/typing/types/types.scala"),

    # === TestVM ===
    ("src/TestVM/call.rs", "TestVM/src/dev/vale/testvm/Call.scala"),
    ("src/TestVM/expression_vivem.rs", "TestVM/src/dev/vale/testvm/ExpressionVivem.scala"),
    ("src/TestVM/function_vivem.rs", "TestVM/src/dev/vale/testvm/FunctionVivem.scala"),
    ("src/TestVM/heap.rs", "TestVM/src/dev/vale/testvm/Heap.scala"),
    ("src/TestVM/values.rs", "TestVM/src/dev/vale/testvm/Values.scala"),
    ("src/TestVM/vivem.rs", "TestVM/src/dev/vale/testvm/Vivem.scala"),
    ("src/TestVM/vivem_externs.rs", "TestVM/src/dev/vale/testvm/VivemExterns.scala"),

    # === Integration Tests ===
    ("src/tests/tests.rs", "Tests/src/dev/vale/Tests.scala"),
]


def extract_block_comments(rust_content):
    """Extract contents of all /* ... */ block comments from Rust source."""
    comments = re.findall(r'/\*(.*?)\*/', rust_content, re.DOTALL)
    return comments


def filter_migration_annotations(text):
    """Remove Guardian: and MIGALLOW lines (including continuations) added during migration.

    MIGALLOW comments can span multiple lines. After the initial "// MIGALLOW" line,
    any subsequent "//" lines are treated as continuations until a non-"//" line is seen.
    """
    lines = text.split('\n')
    filtered = []
    in_migallow = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('Guardian:'):
            in_migallow = False
            continue
        if re.match(r'^//\s*MIGALLOW', stripped):
            in_migallow = True
            continue
        # Filter bare MIGALLOW: lines (not starting with //)
        if re.match(r'^MIGALLOW:', stripped):
            in_migallow = True
            continue
        # Filter AFTERM lines
        if re.match(r'^//?\s*AFTERM:', stripped) or stripped.startswith('AFTERM:'):
            continue
        if in_migallow:
            if stripped.startswith('//'):
                # Continuation of the MIGALLOW comment
                continue
            else:
                in_migallow = False
        # Strip trailing // MIGALLOW... annotations from Scala lines
        line = re.sub(r'\s*//\s*MIGALLOW:?.*$', '', line)
        filtered.append(line)
    return '\n'.join(filtered)


def normalize(text):
    """Normalize text for comparison:
    - Strip leading whitespace from each line
    - Collapse multiple consecutive blank lines into one
    - Strip leading/trailing blank lines
    """
    lines = text.split('\n')
    # Strip leading whitespace from each line
    lines = [line.lstrip() for line in lines]
    # Collapse multiple consecutive blank lines into one
    result = []
    prev_blank = False
    for line in lines:
        is_blank = line.strip() == ''
        if is_blank and prev_blank:
            continue
        result.append(line)
        prev_blank = is_blank
    # Strip leading/trailing blank lines
    while result and result[0].strip() == '':
        result.pop(0)
    while result and result[-1].strip() == '':
        result.pop()
    return result


def diff_is_only_blank_lines(diff_lines):
    """Check if a unified diff contains only blank-line additions/removals.
    Returns True if every changed line (starting with + or -) is blank."""
    for line in diff_lines:
        if line.startswith('---') or line.startswith('+++'):
            continue
        if line.startswith('@@'):
            continue
        if line.startswith('+') or line.startswith('-'):
            content = line[1:]
            if content.strip() != '':
                return False
    return True


def check_file_pair(rust_path, scala_path):
    """Check one file pair. Returns (has_diff, diff_text)."""
    if not os.path.exists(rust_path):
        return (True, f"  Rust file missing: {rust_path}")
    if not os.path.exists(scala_path):
        return (True, f"  Scala file missing: {scala_path}")

    with open(rust_path) as f:
        rust_content = f.read()
    with open(scala_path) as f:
        scala_content = f.read()

    # Extract and combine block comments
    comments = extract_block_comments(rust_content)
    if not comments:
        return (True, f"  No block comments found in Rust file")

    extracted = '\n'.join(comments)
    extracted = filter_migration_annotations(extracted)

    # Normalize both
    scala_lines = normalize(scala_content)
    extracted_lines = normalize(extracted)

    if scala_lines == extracted_lines:
        return (False, None)

    # Generate unified diff
    diff = list(difflib.unified_diff(
        scala_lines,
        extracted_lines,
        fromfile=f"original: {os.path.basename(scala_path)}",
        tofile=f"extracted: {os.path.basename(rust_path)}",
        lineterm='',
        n=2,  # 2 lines of context
    ))

    # If the only differences are blank lines, treat as OK
    if diff_is_only_blank_lines(diff):
        return (False, None)

    return (True, '\n'.join(diff))


def main():
    # Parse args
    filter_path = None
    if len(sys.argv) > 1:
        filter_path = sys.argv[1]

    files_checked = 0
    files_ok = 0
    files_diff = 0
    files_missing = 0

    for rust_rel, scala_rel in FILE_MAP:
        # Filter if specified
        if filter_path and filter_path not in rust_rel and filter_path not in scala_rel:
            continue

        rust_path = os.path.normpath(os.path.join(FRONTEND_RUST, rust_rel))
        scala_path = os.path.normpath(os.path.join(FRONTEND, scala_rel))

        files_checked += 1
        has_diff, diff_text = check_file_pair(rust_path, scala_path)

        if not has_diff:
            files_ok += 1
        else:
            if diff_text.startswith("  ") and ("missing" in diff_text or "No block comments" in diff_text):
                files_missing += 1
                print(f"\n{'='*70}")
                print(f"MISSING: {rust_rel}")
                print(diff_text)
            else:
                files_diff += 1
                print(f"\n{'='*70}")
                print(f"DIFF: {rust_rel} <-> {scala_rel}")
                print(diff_text)

    print(f"\n{'='*70}")
    print(f"Summary: {files_checked} checked, {files_ok} ok, {files_diff} with diffs, {files_missing} missing")


if __name__ == '__main__':
    main()
