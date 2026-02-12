# Prefer find_func_named Over Name Matching (PFFNONM)

Instead of doing a collect_where to find a function by name, use find_func_named. Same with find_struct_named etc.


# Single String Matches Should Have Specific Variable Names (SSMSHSVN)

We often have long patterns that capture variables just to compare to a simple string, and suffix them with an underscore. Like:

    let program = compile("export int as NumberThing;");
    collect_only!(&program, NodeRefP::ExportAs(ExportAsP {
          struct_: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref s, .. } }),
      exported_name: NameP { str: ref e, .. },
      ..
    }) if s.str == "int" && e.str == "NumberThing" => Some(()));

For readability, use specific variable names for the strings. Like:

    let program = compile("export int as NumberThing;");
    collect_only!(&program, NodeRefP::ExportAs(ExportAsP {
          struct_: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref int_, .. } }),
      exported_name: NameP { str: ref NumberThing_, .. },
      ..
    }) if int_.str == "int" && NumberThing_.str == "NumberThing" => Some(()));

If the string uses characters that aren't valid in Rust identifiers, like "*", just make up something, like `star_`.


# Use Imperative Instructions Over Very Complex Patterns (UIIOVCP)

Instead of super long patterns like we see in this one:

    match func.header.ret.ret_type.as_ref() {
          Some(ITemplexPT::Call(CallPT {
              template: box ITemplexPT::NameOrRune(NameOrRunePT { name: NameP { str: ref ret_name, .. } }),
          args: ref args,
          ..
      })) if ret_name.str == "IDesire"
          && args.len() == 2
          && matches!(
                  &args[0],
              ITemplexPT::RegionRune(RegionRunePT { name: Some(NameP { str: ref r_, .. }), .. }) if r_.str == "r"
          )
          && matches!(
                  &args[1],
              ITemplexPT::RegionRune(RegionRunePT { name: Some(NameP { str: ref i_, .. }), .. }) if i_.str == "i"
          ) => {}
      _ => panic!("Expected return type IDesire<r', i'>"),
    }

Rule of thumb: if a pattern is 10 or more lines long, use imperative instructions instead. Like:

    let ret_type = func.header.ret.ret_type.as_ref().expect("Expected return type");
    let ret_call = cast!(ret_type, ITemplexPT::Call);
    let ret_name = &cast!(ret_call.template.as_ref(), ITemplexPT::NameOrRune);
    assert!(ret_name.name.str.str == "IDesire");
    assert!(ret_call.args.len() == 2);
    assert_eq!(cast!(&ret_call.args[0], ITemplexPT::RegionRune).name.as_ref().unwrap().str.str, "r");
    assert_eq!(cast!(&ret_call.args[1], ITemplexPT::RegionRune).name.as_ref().unwrap().str.str, "i");

Inside a block like this, we shouldn't use any `match` statement, and
we should prioritize conciseness and prefer statements that can fit on one (100 character) line.


# No match statements in tests (NMSIT)

Don't have match statements in tests.

For example, no need for a descriptive panic statement in tests, like:

    match err {
          ParseError::UnrecognizedDenizenError(_) => {}
      other => panic!("Expected UnrecognizedDenizenError, got {:?}", other),
    }

Use a macro instead like:

    assert_matches!(err, ParseError::UnrecognizedDenizenError(_));

If there's no obvious macro available, stop and ask for one to be implemented.


# Tests prefer unwrap to expect for conciseness (TPUTEFC)

Instead of using expect() for errors, use unwrap() for conciseness.

    let err = compile_for_error(code).expect_err("Should be error");

Just use unwrap() instead:

    let err = compile_for_error(code).unwrap_err();


# Never Have Conditionals In Tests (NHCIT)

A test should never react to what the implementation is doing. The test should be hard, rigid, and say exactly what it expects the implementation to do.

Concretely: **no if-statements in tests.**


# Use expect_ Functions Instead of Asserting Size then Indexing (UEFIAI)

Instead of asserting the length and indexing into a list like:

    let function_call = cast!(expr, IExpressionPE::FunctionCall);
    assert_eq!(function_call.arg_exprs.len(), 2);
    let first_arg = function_call.arg_exprs[0];
    let first_arg_lookup = cast!(&first_arg, IImpreciseNameP::LookupName);
    assert_eq!(first_arg_lookup.str.str, "ally");
    let second_arg = function_call.arg_exprs[1];
    let second_arg_lookup = cast!(&second_arg, IImpreciseNameP::LookupName);
    assert_eq!(second_arg_lookup.str.str, "enemy");

Use the expect_ functions, like expect_1, expect_2, expect_3, etc.:

    let function_call = cast!(expr, IExpressionPE::FunctionCall);
    let (first_arg, second_arg) = expect_2(function_call.arg_exprs);
    let first_arg_lookup = cast!(&first_arg, IImpreciseNameP::LookupName);
    assert_eq!(first_arg_lookup.str.str, "ally");
    let second_arg_lookup = cast!(&second_arg, IImpreciseNameP::LookupName);
    assert_eq!(second_arg_lookup.str.str, "enemy");


# Prefer Shortening Functions When Possible (PSFWP)

 * Use assert_destination_local_name when checking that a given DestinationLocalP contains an INameDeclarationP::LocalNameDeclaration and has a specific name.
 * Use assert_lookup_name when checking that an IExpressionPE is a Lookup with a specific name (and no template args).
 * Use assert_templex_name when checking that an ITemplexPT is a NameOrRune with a specific name.
 * Use assert_name when you already have an IImpreciseNameP and want to check that it's a LookupName with a specific string.


# Use collect_ macros to recursively search (UCMTRS)

Scala `shouldHave` means "this pattern exists somewhere in the traversed tree", not necessarily at the root node.

In Rust:

 * Use `collect_only!` / `collect_where!` when traversing from a `FileP`.
 * Use `collect_only_rulex!` / `collect_where_rulex!` when traversing from an `IRulexPR`.
