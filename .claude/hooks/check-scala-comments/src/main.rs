use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use similar::TextDiff;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Deserialize)]
struct HookInput {
    tool_input: ToolInput,
}

#[derive(Deserialize)]
struct ToolInput {
    file_path: Option<String>,
    old_string: Option<String>,
    new_string: Option<String>,
    content: Option<String>,
}

/// Map of (rust_path_relative_to_FrontendRust, scala_path_relative_to_Frontend)
const FILE_MAP: &[(&str, &str)] = &[
    // === Lexing ===
    ("src/lexing/ast.rs", "LexingPass/src/dev/vale/lexing/ast.scala"),
    ("src/lexing/errors.rs", "LexingPass/src/dev/vale/lexing/errors.scala"),
    ("src/lexing/lex_and_explore.rs", "LexingPass/src/dev/vale/lexing/LexAndExplore.scala"),
    ("src/lexing/lexer.rs", "LexingPass/src/dev/vale/lexing/Lexer.scala"),
    ("src/lexing/lexing_iterator.rs", "LexingPass/src/dev/vale/lexing/LexingIterator.scala"),
    // === Parsing (src) ===
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
    // === Parsing (tests) ===
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
    // === PostParsing (src) ===
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
    // === PostParsing (tests) ===
    ("src/postparsing/test/post_parser_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserTests.scala"),
    ("src/postparsing/test/post_parser_variable_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserVariableTests.scala"),
    ("src/postparsing/test/post_parsing_parameters_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParsingParametersTests.scala"),
    ("src/postparsing/test/post_parsing_rule_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParsingRuleTests.scala"),
    ("src/postparsing/test/post_parser_error_humanizer_tests.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserErrorHumanizerTests.scala"),
    ("src/postparsing/test/after_regions_error_tests.rs", "PostParsingPass/test/dev/vale/postparsing/AfterRegionsErrorTests.scala"),
    ("src/postparsing/test/post_parser_test_compilation.rs", "PostParsingPass/test/dev/vale/postparsing/PostParserTestCompilation.scala"),
    // === Higher Typing ===
    ("src/higher_typing/ast.rs", "HigherTypingPass/src/dev/vale/highertyping/ast.scala"),
    ("src/higher_typing/higher_typing_pass.rs", "HigherTypingPass/src/dev/vale/highertyping/HigherTypingPass.scala"),
    ("src/higher_typing/astronomer_error_reporter.rs", "HigherTypingPass/src/dev/vale/highertyping/AstronomerErrorReporter.scala"),
    ("src/higher_typing/higher_typing_error_humanizer.rs", "HigherTypingPass/src/dev/vale/highertyping/HigherTypingErrorHumanizer.scala"),
    ("src/higher_typing/patterns.rs", "HigherTypingPass/src/dev/vale/highertyping/patterns.scala"),
    ("src/higher_typing/textifier.rs", "HigherTypingPass/src/dev/vale/highertyping/Textifier.scala"),
    ("src/higher_typing/tests/error_tests.rs", "HigherTypingPass/test/dev/vale/highertyping/ErrorTests.scala"),
    ("src/higher_typing/tests/higher_typing_pass_tests.rs", "HigherTypingPass/test/dev/vale/highertyping/HigherTypingPassTests.scala"),
    ("src/higher_typing/tests/test_compilation.rs", "HigherTypingPass/test/dev/vale/highertyping/HigherTypingTestCompilation.scala"),
    // === Solver ===
    ("src/solver/solver.rs", "Solver/src/dev/vale/solver/Solver.scala"),
    ("src/solver/i_solver_state.rs", "Solver/src/dev/vale/solver/ISolverState.scala"),
    ("src/solver/optimized_solver_state.rs", "Solver/src/dev/vale/solver/OptimizedSolverState.scala"),
    ("src/solver/simple_solver_state.rs", "Solver/src/dev/vale/solver/SimpleSolverState.scala"),
    ("src/solver/solver_error_humanizer.rs", "Solver/src/dev/vale/solver/SolverErrorHumanizer.scala"),
    // === Utils ===
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
    // === Von ===
    ("src/von/ast.rs", "Von/src/dev/vale/von/VonAst.scala"),
    ("src/von/printer.rs", "Von/src/dev/vale/von/VonPrinter.scala"),
    // === Pass Manager ===
    ("src/pass_manager/full_compilation.rs", "PassManager/src/dev/vale/passmanager/FullCompilation.scala"),
    ("src/pass_manager/pass_manager.rs", "PassManager/src/dev/vale/passmanager/PassManager.scala"),
    // === Compile Options ===
    ("src/compile_options/compile_options.rs", "CompileOptions/src/dev/vale/options/GlobalOptions.scala"),
    // === Builtins ===
    ("src/builtins/builtins.rs", "Builtins/src/dev/vale/Builtins.scala"),
    // === Highlighter ===
    ("src/highlighter/highlighter.rs", "Highlighter/src/dev/vale/highlighter/Highlighter.scala"),
    ("src/highlighter/spanner.rs", "Highlighter/src/dev/vale/highlighter/Spanner.scala"),
    ("src/highlighter/tests/highlighter_tests.rs", "Highlighter/test/dev/vale/highlighter/HighlighterTests.scala"),
    ("src/highlighter/tests/spanner_tests.rs", "Highlighter/test/dev/vale/highlighter/SpannerTests.scala"),
    // === Final AST ===
    ("src/final_ast/ast.rs", "FinalAST/src/dev/vale/finalast/ast.scala"),
    ("src/final_ast/instructions.rs", "FinalAST/src/dev/vale/finalast/instructions.scala"),
    ("src/final_ast/types.rs", "FinalAST/src/dev/vale/finalast/types.scala"),
    ("src/final_ast/metal_printer.rs", "FinalAST/src/dev/vale/finalast/MetalPrinter.scala"),
    // === Instantiating ===
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
    // === Simplifying ===
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
    // === Typing (src) ===
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
    // === TestVM ===
    ("src/TestVM/call.rs", "TestVM/src/dev/vale/testvm/Call.scala"),
    ("src/TestVM/expression_vivem.rs", "TestVM/src/dev/vale/testvm/ExpressionVivem.scala"),
    ("src/TestVM/function_vivem.rs", "TestVM/src/dev/vale/testvm/FunctionVivem.scala"),
    ("src/TestVM/heap.rs", "TestVM/src/dev/vale/testvm/Heap.scala"),
    ("src/TestVM/values.rs", "TestVM/src/dev/vale/testvm/Values.scala"),
    ("src/TestVM/vivem.rs", "TestVM/src/dev/vale/testvm/Vivem.scala"),
    ("src/TestVM/vivem_externs.rs", "TestVM/src/dev/vale/testvm/VivemExterns.scala"),
    // === Integration Tests ===
    ("src/tests/tests.rs", "Tests/src/dev/vale/Tests.scala"),
];

static MIGALLOW_SUFFIX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s*//\s*MIGALLOW:?.*$").unwrap());

static MIGALLOW_START_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^//\s*MIGALLOW").unwrap());

static AFTERM_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^//?\s*AFTERM:").unwrap());

fn extract_block_comments(content: &str, file_path: &str) -> Vec<String> {
    let mut comments = Vec::new();
    let mut depth: usize = 0;
    let mut current = String::new();
    let mut chars = content.chars().peekable();
    let mut byte_pos: usize = 0;

    while let Some(&ch) = chars.peek() {
        if ch == '/' {
            chars.next();
            byte_pos += ch.len_utf8();
            if chars.peek() == Some(&'*') {
                chars.next();
                byte_pos += 1;
                depth += 1;
                if depth > 1 {
                    let line_num = content[..byte_pos].matches('\n').count() + 1;
                    panic!(
                        "Nested block comment found in {} at line {}. \
                         Nested block comments are not allowed in this codebase.",
                        file_path, line_num
                    );
                }
            } else if depth == 1 {
                current.push('/');
            }
        } else if ch == '*' {
            chars.next();
            byte_pos += 1;
            if chars.peek() == Some(&'/') {
                chars.next();
                byte_pos += 1;
                if depth == 1 {
                    comments.push(std::mem::take(&mut current));
                }
                depth = depth.saturating_sub(1);
            } else if depth == 1 {
                current.push('*');
            }
        } else {
            chars.next();
            byte_pos += ch.len_utf8();
            if depth == 1 {
                current.push(ch);
            }
        }
    }

    comments
}

fn filter_migration_annotations(text: &str) -> String {
    let mut filtered = Vec::new();
    let mut in_migallow = false;

    for line in text.lines() {
        let stripped = line.trim();

        if stripped.starts_with("Guardian:") {
            in_migallow = false;
            continue;
        }
        if MIGALLOW_START_RE.is_match(stripped) {
            in_migallow = true;
            continue;
        }
        if stripped.starts_with("MIGALLOW:") {
            in_migallow = true;
            continue;
        }
        if AFTERM_RE.is_match(stripped) || stripped.starts_with("AFTERM:") {
            continue;
        }
        if in_migallow {
            if stripped.starts_with("//") {
                continue;
            } else {
                in_migallow = false;
            }
        }
        let line = MIGALLOW_SUFFIX_RE.replace(line, "");
        filtered.push(line.into_owned());
    }

    filtered.join("\n")
}

fn normalize(text: &str) -> Vec<String> {
    text.lines()
        .map(|line| line.trim_start().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

fn diff_is_only_blank_lines(diff_text: &str) -> bool {
    for line in diff_text.lines() {
        if line.starts_with("---") || line.starts_with("+++") || line.starts_with("@@") {
            continue;
        }
        if line.starts_with('+') || line.starts_with('-') {
            let content = &line[1..];
            if !content.trim().is_empty() {
                return false;
            }
        }
    }
    true
}

fn check_file_pair(rust_content: &str, rust_path: &Path, scala_path: &Path) -> Option<String> {
    let scala_content = fs::read_to_string(scala_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", scala_path.display(), e));

    let comments = extract_block_comments(&rust_content, &rust_path.to_string_lossy());
    if comments.is_empty() {
        // Pre-migration file with no block comments yet — skip
        return None;
    }

    let extracted = comments.join("\n");
    let extracted = filter_migration_annotations(&extracted);

    let scala_lines = normalize(&scala_content);
    let extracted_lines = normalize(&extracted);

    if scala_lines == extracted_lines {
        return None;
    }

    let scala_text = scala_lines.join("\n");
    let extracted_text = extracted_lines.join("\n");

    let diff = TextDiff::from_lines(&scala_text, &extracted_text);
    let unified = diff
        .unified_diff()
        .context_radius(2)
        .header(
            &format!("original: {}", scala_path.file_name().unwrap().to_string_lossy()),
            &format!("extracted: {}", rust_path.file_name().unwrap().to_string_lossy()),
        )
        .to_string();

    if diff_is_only_blank_lines(&unified) {
        return None;
    }

    Some(unified)
}

fn find_scala_path(file_path: &str, project_dir: &Path) -> Option<(PathBuf, PathBuf)> {
    let frontend_rust = project_dir.join("FrontendRust");
    let frontend = project_dir.join("Frontend");

    let file_path = Path::new(file_path);

    // Check if file_path is under FrontendRust/
    let rust_rel = file_path.strip_prefix(&frontend_rust).ok()?;
    let rust_rel_str = rust_rel.to_str()?;

    // Look up in FILE_MAP
    for &(map_rust, map_scala) in FILE_MAP {
        if map_rust == rust_rel_str {
            let scala_abs = frontend.join(map_scala);
            if !scala_abs.exists() {
                panic!("Scala file not found: {}", scala_abs.display());
            }
            return Some((file_path.to_path_buf(), scala_abs));
        }
    }

    None
}

use std::io::Write as IoWrite;

fn open_log() -> fs::File {
    let log_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("logs");
    fs::create_dir_all(&log_dir)
        .unwrap_or_else(|e| panic!("Failed to create log dir {}: {}", log_dir.display(), e));
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let log_path = log_dir.join(format!("log-{}.log", timestamp));
    fs::File::create(&log_path)
        .unwrap_or_else(|e| panic!("Failed to create log file {}: {}", log_path.display(), e))
}

macro_rules! log {
    ($log:expr, $($arg:tt)*) => {
        writeln!($log, $($arg)*).unwrap();
    };
}

fn find_project_dir() -> PathBuf {
    // Try CLAUDE_PROJECT_DIR first
    if let Ok(dir) = std::env::var("CLAUDE_PROJECT_DIR") {
        return PathBuf::from(dir);
    }
    // Walk up from cwd looking for FrontendRust/ + Frontend/
    let mut dir = std::env::current_dir().expect("Failed to get current directory");
    loop {
        if dir.join("FrontendRust").is_dir() && dir.join("Frontend").is_dir() {
            return dir;
        }
        if !dir.pop() {
            panic!(
                "Could not find project root (directory containing FrontendRust/ and Frontend/). \
                 Set CLAUDE_PROJECT_DIR or run from within the project tree."
            );
        }
    }
}

fn run_check_all() {
    let project_dir = find_project_dir();
    let frontend_rust = project_dir.join("FrontendRust");
    let frontend = project_dir.join("Frontend");

    let mut checked = 0;
    let mut skipped = 0;
    let mut failures: Vec<(String, String)> = Vec::new();

    for &(rust_rel, scala_rel) in FILE_MAP {
        let rust_path = frontend_rust.join(rust_rel);
        let scala_path = frontend.join(scala_rel);

        if !rust_path.exists() {
            skipped += 1;
            continue;
        }
        if !scala_path.exists() {
            eprintln!("WARNING: Scala file missing: {}", scala_path.display());
            skipped += 1;
            continue;
        }

        let rust_content = fs::read_to_string(&rust_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", rust_path.display(), e));

        match check_file_pair(&rust_content, &rust_path, &scala_path) {
            None => {
                checked += 1;
            }
            Some(diff) => {
                checked += 1;
                failures.push((rust_rel.to_string(), diff));
            }
        }
    }

    if failures.is_empty() {
        println!("All {} files OK ({} skipped)", checked, skipped);
        process::exit(0);
    } else {
        for (rust_rel, diff) in &failures {
            eprintln!("MISMATCH: {}\n{}\n", rust_rel, diff);
        }
        eprintln!(
            "{} of {} files have mismatches ({} skipped)",
            failures.len(),
            checked,
            skipped
        );
        process::exit(1);
    }
}

fn run_hook() {
    let mut log = open_log();

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .unwrap_or_else(|e| panic!("Failed to read stdin: {}", e));

    log!(log, "stdin ({} bytes): {}", input.len(), &input[..input.len().min(500)]);

    let hook_input: HookInput = serde_json::from_str(&input)
        .unwrap_or_else(|e| panic!("Failed to parse hook input JSON: {}", e));

    let file_path = match hook_input.tool_input.file_path {
        Some(p) => p,
        None => {
            log!(log, "no file_path, exit 0");
            process::exit(0);
        }
    };

    log!(log, "file_path: {}", file_path);

    let project_dir = std::env::var("CLAUDE_PROJECT_DIR")
        .unwrap_or_else(|_| panic!("CLAUDE_PROJECT_DIR not set"));
    let project_dir = Path::new(&project_dir);

    let pair = match find_scala_path(&file_path, project_dir) {
        Some(pair) => pair,
        None => {
            log!(log, "not in FILE_MAP, exit 0");
            process::exit(0);
        }
    };

    let (rust_path, scala_path) = pair;
    log!(log, "rust_path: {}", rust_path.display());
    log!(log, "scala_path: {}", scala_path.display());
    log!(log, "has content: {}", hook_input.tool_input.content.is_some());
    log!(log, "has old_string: {}", hook_input.tool_input.old_string.is_some());
    log!(log, "has new_string: {}", hook_input.tool_input.new_string.is_some());

    let rust_content = if let Some(content) = hook_input.tool_input.content {
        log!(log, "Write tool: content ({} bytes)", content.len());
        content
    } else {
        let current = fs::read_to_string(&rust_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", rust_path.display(), e));
        let old = hook_input.tool_input.old_string
            .unwrap_or_else(|| panic!("Edit tool call missing old_string for {}", rust_path.display()));
        let new = hook_input.tool_input.new_string
            .unwrap_or_else(|| panic!("Edit tool call missing new_string for {}", rust_path.display()));
        log!(log, "Edit tool: old_string ({} bytes), new_string ({} bytes)", old.len(), new.len());
        if !current.contains(&old) {
            panic!("old_string not found in {}", rust_path.display());
        }
        current.replacen(&old, &new, 1)
    };

    match check_file_pair(&rust_content, &rust_path, &scala_path) {
        None => {
            log!(log, "PASS");
            process::exit(0);
        }
        Some(diff) => {
            let reason = format!(
                "Scala comment parity check FAILED for {}\n\n{}\n\n\
                 See FrontendRust/docs/usage/check-scala-comments-hook.md for how to fix this.",
                rust_path.display(),
                diff
            );
            log!(log, "FAIL:\n{}", reason);
            let response = serde_json::json!({
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": reason
                }
            });
            println!("{}", response);
            process::exit(2);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--check-all") {
        run_check_all();
    } else {
        run_hook();
    }
}
