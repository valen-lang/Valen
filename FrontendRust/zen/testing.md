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

    // UIIOVCP
    let ret_type = func.header.ret.ret_type.as_ref().expect("Expected return type");
    let ret_call = cast!(ret_type, ITemplexPT::Call);
    let ret_name = &cast!(ret_call.template.as_ref(), ITemplexPT::NameOrRune);
    assert!(ret_name.name.str.str == "IDesire");
    assert!(ret_call.args.len() == 2);
    assert_eq!(cast!(&ret_call.args[0], ITemplexPT::RegionRune).name.as_ref().unwrap().str.str, "r");
    assert_eq!(cast!(&ret_call.args[1], ITemplexPT::RegionRune).name.as_ref().unwrap().str.str, "i");

And make sure to put a `// UIIOVCP` comment above them so it's clear.
Inside a UIIOVCP block like this, we shouldn't use any `match` statement, and
we should prioritize conciseness and prefer statements that can fit on one (100 character) line.


# Tests prefer macros instead of match-panic (TPMIOMP)

No need for a descriptive panic statement in tests, like:

    match err {
          ParseError::UnrecognizedDenizenError(_) => {}
      other => panic!("Expected UnrecognizedDenizenError, got {:?}", other),
    }

Just use a macro like:

    assert_matches!(err, ParseError::UnrecognizedDenizenError(_));


# Tests prefer unwrap to expect for conciseness (TPUTEFC)

Instead of using expect() for errors, use unwrap() for conciseness.

    let err = compile_for_error(code).expect_err("Should be error");

Just use unwrap() instead:

    let err = compile_for_error(code).unwrap_err();


# Never Have Conditionals In Tests (NHCIT)

A test should never react to what the implementation is doing. The test should be hard, rigid, and say exactly what it expects the implementation to do.

Concretely: **no if-statements in tests.**
