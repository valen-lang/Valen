mod flagger;
mod suite;
mod builder;
mod runner;
mod rust_interop;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::flagger::{parse_all_flags, FLAG_BOOL, FLAG_INT, FLAG_STR, Flag};
use crate::suite::TestSuite;

fn main() -> ExitCode {
    let all_args: Vec<String> = std::env::args().collect();

    let tester_path = std::fs::canonicalize(PathBuf::from(&all_args[0]))
        .expect("resolve tester_path failed");
    let cwd = tester_path
        .parent()
        .expect("tester_path has no parent")
        .to_path_buf();
    let _compiler_dir = cwd.clone();


    let flags: Vec<Flag> = vec![
        Flag("--backend_path",       FLAG_STR(),  "Path to Midas executable.", "../Midas/build", "Alternate path for Midas, the codegen phase binary."),
        Flag("--frontend_path",      FLAG_STR(),  "Path to Frontend jar.",     "../Frontend",    "Alternate path for Frontend, the frontend phase binary."),
        Flag("--builtins_dir",       FLAG_STR(),  "Directory containing Midas builtins.", "../Midas/src/builtins", "Alternate path for the temporary C builtin functions."),
        Flag("--valec_path",         FLAG_STR(),  "Path to valec executable.", "../Driver/build/valec", "Path to valec executable."),
        Flag("--clang_path",         FLAG_STR(),  "Path to clang executable.", "/usr/bin/clang", "Path to clang executable."),
        Flag("--libc_path",          FLAG_STR(),  "Path to libc folder.",      "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr", "Path to libc folder which contains lib and include."),
        Flag("--backend_tests_dir",  FLAG_STR(),  "Directory containing Midas tests.", "../Midas/build", "Directory containing Midas tests."),
        Flag("--frontend_tests_dir", FLAG_STR(),  "Directory containing Frontend's tests.", "../Frontend", "Directory containing Frontend's tests."),
        Flag("--stdlib_dir",         FLAG_STR(),  "Directory containing the stdlib to use in the tests.", "../Frontend", "Directory containing the stdlib to use in the tests."),
        Flag("--concurrent",         FLAG_INT(),  "How many tests to run in parallel.", "10", "How many tests to run in parallel."),
        Flag("--verbose",            FLAG_BOOL(), "Print more progress details.", "true", "Print more progress details."),
        Flag("--flares",             FLAG_BOOL(), "Add flare output to stderr.", "false", "Add flare output to stderr."),
        // ====== beyond-port: rust-interop flags ======
        Flag("--vale_ruster_path",   FLAG_STR(),  "Path to ValeRuster executable.", "", "Path to the ValeRuster pass binary (beyond-port)."),
        Flag("--divination_path",    FLAG_STR(),  "Path to Divination executable.", "", "Path to the Divination pass binary (beyond-port)."),
        Flag("--rust_cargo_toml",    FLAG_STR(),  "Path to a Rust Cargo.toml.",     "", "Path to the rust-interop Cargo.toml (beyond-port)."),
        Flag("--rust_interop_tests_dir", FLAG_STR(), "Directory containing rust-interop tests.", "tests/rust-interop", "Directory scanned for rust-interop tests (beyond-port)."),
        // =============================================
    ];
    let parsed_flags = parse_all_flags(&flags, &all_args);


    let mut common_build_args: Vec<String> = Vec::new();
    common_build_args.push("build".to_string());


    let verbose = parsed_flags.get_bool_flag("--verbose", false);
    if verbose {
        println!("Tester processing flags");
    }
    let flares = parsed_flags.get_bool_flag("--flares", false);

    let max_concurrent_tests = parsed_flags.get_int_flag("--concurrent", 10);

    let frontend_path = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--frontend_path"));
    common_build_args.push("--frontend_path_override".to_string());
    common_build_args.push(frontend_path.to_string_lossy().into_owned());

    let backend_path = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--backend_path"));
    common_build_args.push("--backend_path_override".to_string());
    common_build_args.push(backend_path.to_string_lossy().into_owned());

    let builtins_dir = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--builtins_dir"));
    common_build_args.push("--builtins_dir_override".to_string());
    common_build_args.push(builtins_dir.to_string_lossy().into_owned());

    let valec_path = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--valec_path"));

    let maybe_clang_path = parsed_flags.get_string_flag("--clang_path");
    if maybe_clang_path.is_some() {
        common_build_args.push("--clang_override".to_string());
        common_build_args.push(maybe_clang_path.clone().unwrap());
    }

    let maybe_libc_path = parsed_flags.get_string_flag("--libc_path");
    if maybe_libc_path.is_some() {
        common_build_args.push("--libc_override".to_string());
        common_build_args.push(maybe_libc_path.clone().unwrap());
    }

    let stdlib_dir = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--stdlib_dir"));

    let backend_tests_dir = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--backend_tests_dir"));
    let frontend_tests_dir = PathBuf::from(parsed_flags.expect_string_flag(&flags, "--frontend_tests_dir"));

    let mut include_regions: Vec<String> = Vec::new();
    let mut test_filters: Vec<String> = Vec::new();


    if verbose {
        println!("Parsing command line inputs...");
    }

    let mut input_i = 0;
    while input_i < parsed_flags.unrecognized_inputs.len() {
        let unrecognized_input = parsed_flags.unrecognized_inputs[input_i].clone();
        if unrecognized_input.starts_with("--") {
            panic!("Unrecognized input: {}", unrecognized_input);
        } else if unrecognized_input.starts_with("@") {
            let region = unrecognized_input[1..].to_string();
            include_regions.push(region);
        } else {
            test_filters.push(unrecognized_input);
        }
        input_i = input_i + 1;
    }

    let samples_path = std::fs::canonicalize(
        frontend_tests_dir.join("Tests/test/main/resources"),
    )
    .expect("resolve samples_path failed");


    if include_regions.len() == 0 {
        include_regions.push("naive-rc".to_string());
        include_regions.push("unsafe-fast".to_string());
    }

    if verbose {
        println!("Parsed command line inputs...");
    }

    let mut suite = TestSuite {
        cwd,
        verbose,
        flares,
        valec_path,
        common_build_args,
        test_filters,
        max_concurrent_tests,
        backend_tests_dir: backend_tests_dir.clone(),
        stdlib_dir: stdlib_dir.clone(),
        test_instances: Vec::new(),
        num_successes: 0,
        num_failures: 0,
    };

    let _test_instances: Vec<crate::suite::TestInstance> = Vec::new();


    //if (include_regions.exists({ _ == "assist" })) {
    //  region = "assist";
    //  suite.StartTest(42, "weakDropThenLockStruct", samples_path./("programs/weaks/dropThenLockStruct.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakLockWhileLiveStruct", samples_path./("programs/weaks/lockWhileLiveStruct.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakFromLocalCRefStruct", samples_path./("programs/weaks/weakFromLocalCRefStruct.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakFromCRefStruct", samples_path./("programs/weaks/weakFromCRefStruct.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "loadFromWeakable", samples_path./("programs/weaks/loadFromWeakable.vale"), &List<str>(), region, false);
    //  suite.StartTest(42, "weakDropThenLockInterface", samples_path./("programs/weaks/dropThenLockInterface.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakLockWhileLiveInterface", samples_path./("programs/weaks/lockWhileLiveInterface.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakFromLocalCRefInterface", samples_path./("programs/weaks/weakFromLocalCRefInterface.vale"), &List<str>(), region, false);
    //  suite.StartTest(7, "weakFromCRefInterface", samples_path./("programs/weaks/weakFromCRefInterface.vale"), &List<str>(), region, false);
    //  suite.StartTest(42, "weakSelfMethodCallWhileLive", samples_path./("programs/weaks/callWeakSelfMethodWhileLive.vale"), &List<str>(), region, false);
    //  suite.StartTest(42, "weakSelfMethodCallAfterDrop", samples_path./("programs/weaks/callWeakSelfMethodAfterDrop.vale"), &List<str>(), region, false);
    //}

    //if (include_regions.exists({ _ == "resilient-v4" })) {
    //  region = "resilient-v4";
    //  suite.StartTest(14, "tethercrash", backend_tests_dir./("tethercrash.vale"), &List<str>(), region, false);
    //}

    for region in include_regions.iter() {
        let region = region.as_str();
        suite.StartTest(42, "mutswaplocals", &samples_path.join("programs/mutswaplocals.vale"), &[], region, false, None);
        suite.StartTest(5, "structimm", &samples_path.join("programs/structs/structimm.vale"), &[], region, false, None);
        suite.StartTest(5, "memberrefcount", &samples_path.join("programs/structs/memberrefcount.vale"), &[], region, false, None);
        suite.StartTest(42, "bigstructimm", &samples_path.join("programs/structs/bigstructimm.vale"), &[], region, false, None);
        suite.StartTest(8, "structmut", &samples_path.join("programs/structs/structmut.vale"), &[], region, false, None);
        suite.StartTest(42, "restackify", &samples_path.join("programs/restackify.vale"), &[], region, false, None);
        suite.StartTest(42, "destructure_restackify", &samples_path.join("programs/destructure_restackify.vale"), &[], region, false, None);
        suite.StartTest(42, "loop_restackify", &samples_path.join("programs/loop_restackify.vale"), &[], region, false, None);
        suite.StartTest(42, "lambda", &samples_path.join("programs/lambdas/lambda.vale"), &[], region, false, None);
        suite.StartTest(42, "if", &samples_path.join("programs/if/if.vale"), &[], region, false, None);
        suite.StartTest(42, "upcastif", &samples_path.join("programs/if/upcastif.vale"), &[], region, false, None);
        suite.StartTest(42, "ifnevers", &samples_path.join("programs/if/ifnevers.vale"), &[], region, false, None);
        suite.StartTest(42, "mutlocal", &samples_path.join("programs/mutlocal.vale"), &[], region, false, None);
        suite.StartTest(42, "while", &samples_path.join("programs/while/while.vale"), &[], region, false, None);
        suite.StartTest(8, "constraintRef", &samples_path.join("programs/constraintRef.vale"), &[], region, false, None);
        suite.StartTest(42, "ssamutfromcallable", &samples_path.join("programs/arrays/ssamutfromcallable.vale"), &[], region, false, None);
        suite.StartTest(42, "ssamutfromvalues", &samples_path.join("programs/arrays/ssamutfromvalues.vale"), &[], region, false, None);
        suite.StartTest(42, "interfaceimm", &samples_path.join("programs/virtuals/interfaceimm.vale"), &[], region, false, None);
        suite.StartTest(42, "interfacemut", &samples_path.join("programs/virtuals/interfacemut.vale"), &[], region, false, None);
        suite.StartTest(42, "structmutstore", &samples_path.join("programs/structs/structmutstore.vale"), &[], region, false, None);
        suite.StartTest(42, "structmutstoreinner", &samples_path.join("programs/structs/structmutstoreinner.vale"), &[], region, false, None);
        suite.StartTest(3, "rsaimm", &samples_path.join("programs/arrays/rsaimm.vale"), &[], region, false, None);
        suite.StartTest(3, "rsamut", &samples_path.join("programs/arrays/rsamut.vale"), &[], region, false, None);
        suite.StartTest(42, "rsamutdestroyintocallable", &samples_path.join("programs/arrays/rsamutdestroyintocallable.vale"), &[], region, false, None);
        suite.StartTest(42, "ssamutdestroyintocallable", &samples_path.join("programs/arrays/ssamutdestroyintocallable.vale"), &[], region, false, None);
        suite.StartTest(5, "rsamutlen", &samples_path.join("programs/arrays/rsamutlen.vale"), &[], region, false, None);
        suite.StartTest(42, "rsamutcapacity", &samples_path.join("programs/arrays/rsamutcapacity.vale"), &[], region, false, None);
        suite.StartTest(42, "stradd", &samples_path.join("programs/strings/stradd.vale"), &[], region, false, None);
        suite.StartTest(42, "strneq", &samples_path.join("programs/strings/strneq.vale"), &[], region, false, None);
        suite.StartTest(42, "lambdamut", &samples_path.join("programs/lambdas/lambdamut.vale"), &[], region, false, None);
        suite.StartTest(42, "strprint", &samples_path.join("programs/strings/strprint.vale"), &[], region, false, None);
        suite.StartTest(4, "inttostr", &samples_path.join("programs/strings/inttostr.vale"), &[], region, false, None);
        suite.StartTest(4, "i64tostr", &samples_path.join("programs/strings/i64tostr.vale"), &[], region, false, None);
        suite.StartTest(42, "nestedif", &samples_path.join("programs/if/nestedif.vale"), &[], region, false, None);
        suite.StartTest(42, "unstackifyret", &samples_path.join("programs/unstackifyret.vale"), &[], region, false, None);
        suite.StartTest(42, "swaprsamutdestroy", &samples_path.join("programs/arrays/swaprsamutdestroy.vale"), &[], region, false, None);
        suite.StartTest(42, "downcastBorrowSuccessful", &samples_path.join("programs/downcast/downcastBorrowSuccessful.vale"), &[], region, false, None);
        suite.StartTest(42, "downcastBorrowFailed", &samples_path.join("programs/downcast/downcastBorrowFailed.vale"), &[], region, false, None);
        suite.StartTest(42, "downcastOwningSuccessful", &samples_path.join("programs/downcast/downcastOwningSuccessful.vale"), &[], region, false, None);
        suite.StartTest(42, "downcastOwningFailed", &samples_path.join("programs/downcast/downcastOwningFailed.vale"), &[], region, false, None);
        suite.StartTest(42, "unreachablemoot", &samples_path.join("programs/unreachablemoot.vale"), &[], region, false, None);
        suite.StartTest(1, "panic", &samples_path.join("programs/panic.vale"), &[], region, false, None);
        suite.StartTest(42, "panicnot", &samples_path.join("programs/panicnot.vale"), &[], region, false, None);
        suite.StartTest(42, "nestedblocks", &samples_path.join("programs/nestedblocks.vale"), &[], region, false, None);
        suite.StartTest(42, "restackify", &samples_path.join("programs/restackify.vale"), &[], region, false, None);

        if region != "naive-rc" {
            suite.StartTest(42, "interfacemutreturnexport", &samples_path.join("programs/externs/interfacemutreturnexport"), &[], region, false, None);
            suite.StartTest(42, "interfacemutparamexport", &samples_path.join("programs/externs/interfacemutparamexport"), &[], region, false, None);
            suite.StartTest(42, "structmutreturnexport", &samples_path.join("programs/externs/structmutreturnexport"), &[], region, false, None);
            suite.StartTest(42, "structmutparamexport", &samples_path.join("programs/externs/structmutparamexport"), &[], region, false, None);
            suite.StartTest(42, "structmutparamdeepexport", &samples_path.join("programs/externs/structmutparamdeepexport"), &[], region, false, None);
            suite.StartTest(10, "rsamutparamexport", &samples_path.join("programs/externs/rsamutparamexport"), &[], region, false, None);
            suite.StartTest(42, "rsamutreturnexport", &samples_path.join("programs/externs/rsamutreturnexport"), &[], region, false, None);
            suite.StartTest(10, "ssamutparamexport", &samples_path.join("programs/externs/ssamutparamexport"), &[], region, false, None);
            suite.StartTest(42, "ssamutreturnexport", &samples_path.join("programs/externs/ssamutreturnexport"), &[], region, false, None);
        }
    }

    // ====== beyond-port: rust-interop tests ======
    let maybe_vale_ruster_path = parsed_flags.get_string_flag("--vale_ruster_path");
    if maybe_vale_ruster_path.is_some() {
        let rust_interop_tests_dir = PathBuf::from(
            parsed_flags
                .get_string_flag("--rust_interop_tests_dir")
                .unwrap_or_else(|| "tests/rust-interop".to_string()),
        );
        let divination_path = parsed_flags.get_string_flag("--divination_path").map(PathBuf::from);
        let rust_cargo_toml = parsed_flags.get_string_flag("--rust_cargo_toml").map(PathBuf::from);
        let vale_ruster_path = PathBuf::from(maybe_vale_ruster_path.unwrap());
        rust_interop::register(
            &mut suite,
            &rust_interop_tests_dir,
            Some(&vale_ruster_path),
            divination_path.as_ref(),
            rust_cargo_toml.as_ref(),
        );
    }
    // =============================================

    suite.FinishTests(0);

    let successes = suite.num_successes;
    let failures = suite.num_failures;

    println!("Done! Passed {}/{}", successes, successes + failures);
    if failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
/* vale
exported func main() int {
  all_args_list = List<str>();
  i = 0;
  while (i < numMainArgs()) {
    all_args_list.add(getMainArg(i));
    set i = i + 1;
  }
  all_args = all_args_list.toImmArray();


  tester_path = Path(all_args_list.get(0))&.resolve();
  cwd = tester_path.directory();
  compiler_dir = cwd.clone();


  flags =
      [#][
        Flag("--backend_path",       FLAG_STR(),  "Path to Midas executable.", "../Midas/build", "Alternate path for Midas, the codegen phase binary."),
        Flag("--frontend_path",      FLAG_STR(),  "Path to Frontend jar.",     "../Frontend",    "Alternate path for Frontend, the frontend phase binary."),
        Flag("--builtins_dir",       FLAG_STR(),  "Directory containing Midas builtins.", "../Midas/src/builtins", "Alternate path for the temporary C builtin functions."),
        Flag("--valec_path",         FLAG_STR(),  "Path to valec executable.", "../Driver/build/valec", "Path to valec executable."),
        Flag("--clang_path",         FLAG_STR(),  "Path to clang executable.", "/usr/bin/clang", "Path to clang executable."),
        Flag("--libc_path",          FLAG_STR(),  "Path to libc folder.",      "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr", "Path to libc folder which contains lib and include."),
        Flag("--backend_tests_dir",  FLAG_STR(),  "Directory containing Midas tests.", "../Midas/build", "Directory containing Midas tests."),
        Flag("--frontend_tests_dir", FLAG_STR(),  "Directory containing Frontend's tests.", "../Frontend", "Directory containing Frontend's tests."),
        Flag("--stdlib_dir",         FLAG_STR(),  "Directory containing the stdlib to use in the tests.", "../Frontend", "Directory containing the stdlib to use in the tests."),
        Flag("--concurrent",         FLAG_INT(),  "How many tests to run in parallel.", "10", "How many tests to run in parallel."),
        Flag("--verbose",            FLAG_BOOL(), "Print more progress details.", "true", "Print more progress details."),
        Flag("--flares",             FLAG_BOOL(), "Add flare output to stderr.", "false", "Add flare output to stderr.")];
  parsed_flags = flags.parse_all_flags(all_args);


  common_build_args = List<str>();
  common_build_args.add("build");


  verbose = parsed_flags.get_bool_flag("--verbose", false);
  if verbose {
    println("Tester processing flags");
  }
  flares = parsed_flags.get_bool_flag("--flares", false);

  max_concurrent_tests = parsed_flags.get_int_flag("--concurrent", 10);

  frontend_path = Path(parsed_flags.expect_string_flag(&flags, "--frontend_path"));
  common_build_args.add("--frontend_path_override");
  common_build_args.add(frontend_path.str());

  backend_path = Path(parsed_flags.expect_string_flag(&flags, "--backend_path"));
  common_build_args.add("--backend_path_override");
  common_build_args.add(backend_path.str());

  builtins_dir = Path(parsed_flags.expect_string_flag(&flags, "--builtins_dir"));
  common_build_args.add("--builtins_dir_override");
  common_build_args.add(builtins_dir.str());

  valec_path = Path(parsed_flags.expect_string_flag(&flags, "--valec_path"));

  maybe_clang_path = parsed_flags.get_string_flag("--clang_path");
  if not maybe_clang_path.isEmpty() {
    common_build_args.add("--clang_override");
    common_build_args.add(maybe_clang_path.get());
  }

  maybe_libc_path = parsed_flags.get_string_flag("--libc_path");
  if not maybe_libc_path.isEmpty() {
    common_build_args.add("--libc_override");
    common_build_args.add(maybe_libc_path.get());
  }

  stdlib_dir = Path(parsed_flags.expect_string_flag(&flags, "--stdlib_dir"));

  backend_tests_dir = Path(parsed_flags.expect_string_flag(&flags, "--backend_tests_dir"));
  frontend_tests_dir = Path(parsed_flags.expect_string_flag(&flags, "--frontend_tests_dir"));

  include_regions = List<str>();
  test_filters = List<str>();


  if (verbose) {
    println("Parsing command line inputs...")
  }

  input_i = 0;
  while (input_i < parsed_flags.unrecognized_inputs.len()) {
    unrecognized_input = parsed_flags.unrecognized_inputs.get(input_i);
    if (unrecognized_input.startsWith("--")) {
      panic("Unrecognized input: " + unrecognized_input);
    } else if (unrecognized_input.startsWith("@")) {
      region = unrecognized_input.slice(1).str();
      include_regions.add(region);
    } else {
      test_filters.add(unrecognized_input);
    }
    set input_i = input_i + 1;
  }

  samples_path = frontend_tests_dir./("Tests/test/main/resources")&.resolve();


  if (include_regions.len() == 0) {
    include_regions.add("naive-rc");
    include_regions.add("unsafe-fast");
  }

  if (verbose) {
    println("Parsed command line inputs...");
  }

  suite =
      TestSuite(
          cwd,
          verbose,
          flares,
          valec_path,
          common_build_args,
          test_filters,
          max_concurrent_tests,
          backend_tests_dir.clone(),
          stdlib_dir.clone(),
          List<TestInstance>(),
          0,
          0);

  test_instances = List<TestInstance>();


  //if (include_regions.exists({ _ == "assist" })) {
  //  region = "assist";
  //  suite.StartTest(42, "weakDropThenLockStruct", samples_path./("programs/weaks/dropThenLockStruct.vale"), &List<str>(), region, false);
  //  ... (11 more)
  //}

  //if (include_regions.exists({ _ == "resilient-v4" })) {
  //  region = "resilient-v4";
  //  suite.StartTest(14, "tethercrash", backend_tests_dir./("tethercrash.vale"), &List<str>(), region, false);
  //}

  include_regions.each((region) => {
    suite.StartTest(42, "mutswaplocals", samples_path./("programs/mutswaplocals.vale"), &List<str>(), region, false, None<str>());
    // ... (45 more StartTest calls — see main.vale lines 256-301)
    suite.StartTest(42, "restackify", samples_path./("programs/restackify.vale"), &List<str>(), region, false, None<str>());

    if (region != "naive-rc") {
      suite.StartTest(42, "interfacemutreturnexport", samples_path./("programs/externs/interfacemutreturnexport"), &List<str>(), region, false, None<str>());
      // ... (8 more StartTest calls — see main.vale lines 303-313)
      suite.StartTest(42, "ssamutreturnexport", samples_path./("programs/externs/ssamutreturnexport"), &List<str>(), region, false, None<str>());
    }
  });

  suite.FinishTests(0);

  successes = suite.num_successes;
  failures = suite.num_failures;

  println("Done! Passed {successes}/{successes + failures}");
  if (failures == 0) {
    return 0;
  } else {
    return 1;
  }
}
*/
