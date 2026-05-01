# Master Migration TODO

Items sourced from `// V:` review comments across the codebase.

## Architecture / Design Questions

- [ ] `parser_bump` should be a private part of `parse_arena` (`full_compilation.rs:69`)
- [ ] Clarify why we have both `parser_bump` and `parse_arena` (`instantiated_compilation.rs:41`)
- [ ] Where did all the parser interning stuff go? (`parse_arena.rs:139`)
- [ ] Can we put the `Keywords` struct into the arena so it doesn't have to be a separate thing? (`parse_and_explore.rs:9`)
- [ ] Should `PatternParser` be folded into the main `Parser` struct? (`pattern_parser.rs:30`)
- [ ] Suspicious that we can get the underlying arena from `scout_arena` (`post_parser.rs:1888`)
- [ ] Coherent story for when something should be `self`/impl'd vs free function (`rune_type_solver.rs:997`)
- [ ] Coherent story for when something is inline or in the arena (`code_hierarchy.rs:143`)
- [ ] How do slices work with interning? Dedup? Equality? (`expression_scout.rs:475`)
- [ ] Should we reference docs about how our arenas work (`pass_manager.rs:701`)

## Unnecessary Clone

- [ ] Why do we have cloning on `ImportL` etc.? (`lexing/ast.rs:130`)
- [ ] Why are all parsing AST types cloneable? (`parsing/ast/ast.rs:363`)
- [ ] Why are all expression types cloneable? (`parsing/ast/expressions.rs:230`)
- [ ] Why is `PatternParser` cloneable? (`pattern_parser.rs:29`)
- [ ] Why is `IRulexSR` cloneable? (`rules/rules.rs:76`)
- [ ] Does `PatternScout` need to be clone? (`patterns/patterns.rs:59`)

## Scala Parity Verification

- [ ] Are `LexingIterator` changes closer or further from Scala? (`lexing_iterator.rs:242`)
- [ ] Is `scout_generic_parameter` now closer or further from Scala? (`post_parser.rs:2284`)
- [ ] Is `translate_rulex` closer to Scala or further? (`rule_scout.rs:218`)
- [ ] Is `translate_rulex` `BuiltinCallPR` closer to Scala or further? (`rule_scout.rs:331`)
- [ ] Is `test_simple_function` a faithful translation? (`post_parser_tests.rs:72`)
- [ ] Is `test_empty_function` a faithful translation? (`post_parser_tests.rs:123`)
- [ ] Above changes consistent with below Scala? (`post_parsing_rule_tests.rs:59`)
- [ ] Above changes consistent with below Scala? (`post_parsing_parameters_tests.rs:69`)
- [ ] Above changes consistent with below Scala? (`post_parsing_parameters_tests.rs:119`)
- [ ] Is `translate_type_into_rune` a faithful translation? (`templex_scout.rs:570`)
- [ ] Does above comment match Scala? (`rules/rules.rs:75`)

## Shield / Process Improvements

- [ ] Make sure `equals` and `hashCode` are mentioned in shields as exceptions (`postparsing/ast.rs:61`)
- [ ] Combine the various "must match Scala" shields (`postparsing/ast.rs:62`, `rules/rules.rs:175`)
- [ ] Make `equals` and `hashCode` exceptions to the broad rule (`rules/rules.rs:174`)
- [ ] Make a tool that pulls out everything not in a block comment and compares to `Frontend/` (`expression_scout.rs:1`)
- [ ] Should have a rule that we cannot have any `todo!` in the codebase (`lex_and_explore.rs:49`)

## Enforcement / Constructors

- [ ] Do we have anything enforcing that we must go through constructors? (`ast/templex.rs:196`)
- [ ] Anything enforce that we actually have to call this constructor? (`post_parser.rs:2656`)

## Unexplained Changes

- [ ] Investigate the change in `function_scout.rs:1689`
- [ ] Why is this so verbose compared to Scala? (`post_parser.rs:2706`)
