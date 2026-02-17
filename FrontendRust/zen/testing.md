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


# Prefer Single Match Over Nested Matches (PSMONM)

When matching on structure, use one match with nested patterns (including slice patterns) instead of multiple nested matches.

Avoid an outer match with an inner match:

    match &expr {
      IExpressionPE::FunctionCall(FunctionCallPE { arg_exprs, .. }) => {
        match (arg_exprs.get(0), arg_exprs.get(1)) {
          (Some(IExpressionPE::ConstantInt(..)), Some(IExpressionPE::Lookup(..))) => {}
          _ => panic!("expected structure"),
        }
      }
      _ => panic!("expected structure"),
    }

Prefer a single match with slice patterns and nested destructuring:

    match &expr {
      IExpressionPE::FunctionCall(FunctionCallPE {
        callable_expr: box IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("moo"))), .. }),
        arg_exprs: [IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
                    IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("=="))), template_args: None }), ..],
        ..
      }) => {}
      _ => panic!("expected moo(4, ==) structure"),
    }


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

Note that RSMSCP supersedes this. It's more important that Rust matches Scala.


# Use collect_ macros to recursively search (UCMTRS)

Scala `shouldHave` means "this pattern exists somewhere in the traversed tree", not necessarily at the root node.

In Rust:

 * Use `collect_only!` / `collect_where!` when traversing from a `FileP`.
 * Use `collect_only_rulex!` / `collect_where_rulex!` when traversing from an `IRulexPR`.
