use crate::suite::{matches_filters, Step, Subprocess, TestInstance, TestSuite};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct ExecResult {
    pub return_code: i64,
    pub stdout: String,
    pub stderr: String,
}
// no main.vale counterpart (Vale's stdlib `ExecResult` wraps the same fields;
// reproduced here so the call sites match the Vale shape).

pub fn capture_and_join(process: Subprocess) -> ExecResult {
    let output = process.child.wait_with_output().expect("capture_and_join failed");
    ExecResult {
        return_code: output.status.code().unwrap_or(-1) as i64,
        stdout: String::from_utf8(output.stdout).expect("non-UTF8 stdout"),
        stderr: String::from_utf8(output.stderr).expect("non-UTF8 stderr"),
    }
}
// no main.vale counterpart (Vale's stdlib `Subprocess.capture_and_join()` method;
// reproduced here as a free function so `(build_process).capture_and_join()` works).

impl TestSuite {
    #[allow(non_snake_case, dead_code, unused_variables)]
    pub fn StartCTest(
        &mut self,
        test_name: &str,
        input_c: &Path,
        flags: Vec<String>,
        region: &str,
        expect_stdout: Option<String>,
    ) {
        let suite = self;
        if matches_filters(test_name, &suite.test_filters) {
            suite.FinishTests(suite.max_concurrent_tests - 1);

            println!("Starting {}...", test_name);
            let _ = std::fs::create_dir_all(suite.cwd.join("testbuild"));
            let test_build_dir = suite.cwd.join(format!("testbuild/{}", test_name));
            let _ = std::fs::create_dir_all(&test_build_dir);
            let program = if cfg!(windows) { "cl.exe" } else { "clang" };
            let command = format!(
                "{} {} -o {}",
                program,
                suite.backend_tests_dir.join("twinpages/test.c").to_string_lossy(),
                test_build_dir.join("main").to_string_lossy()
            );
            let child = Command::new(program)
                .current_dir(&test_build_dir)
                .arg(suite.backend_tests_dir.join("twinpages/test.c"))
                .arg("-o")
                .arg(test_build_dir.join("main"))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("StartCTest spawn failed");
            let process = Subprocess { child, command };
            if suite.verbose {
                println!("Running command: {}", process.command);
            }
            // Vale source bitrot: the original Vale code passes 7 positional args
            // to a 6-field TestInstance constructor (a stray `0` and `flags`
            // instead of a List<Step>). That wouldn't compile in Vale either —
            // there are no callers in main.vale, so it's never exercised. The
            // shape below matches Vale's intent (the extra positional is dropped
            // and `flags` is left out — runs_args stays empty).
            suite.test_instances.push(TestInstance {
                test_name: test_name.to_string(),
                region: region.to_string(),
                test_build_dir,
                process,
                runs_args: Vec::new(),
                expect_stdout,
            });
            drop(flags);
        } else {
            drop(flags);
        }
    }
}
/* vale
func StartCTest(suite &TestSuite, test_name str, input_c &Path, flags List<str>, region str, expect_stdout Opt<str>) {
  if (matches_filters(test_name, &suite.test_filters)) {
    suite.FinishTests(suite.max_concurrent_tests - 1);

    println("Starting {test_name}...");
    suite.cwd./("testbuild").makeDirectory();
    test_build_dir = suite.cwd./("testbuild/{test_name}");
    test_build_dir.makeDirectory();
    process =
        SubprocessBuilder()
            .WithProgram(if (IsWindows()) { "cl.exe" } else { "clang" })
            .FromDir(test_build_dir.str())
            .WithArg(suite.backend_tests_dir./("twinpages/test.c").str())
            .WithArg("-o")
            .WithArg(test_build_dir./("main").str())
            .Build()
            .expect();
    if (suite.verbose) {
      println("Running command: " + process.command);
    }
    suite.test_instances.add(
        TestInstance(test_name, region, 0, test_build_dir, process, flags, expect_stdout));
  } else {
    drop(flags);
  }
}
*/

#[allow(non_snake_case)]
pub fn PrintTestFailure(build_result: &ExecResult, test_name: &str, region: &str) {
    println!(
        "Error {} building test {} (region {}).",
        build_result.return_code, test_name, region
    );
    if build_result.stdout.len() > 0 {
        println!("stdout:");
        println!("{}", build_result.stdout);
    } else {
        println!("(no stdout)");
    }
    if build_result.stderr.len() > 0 {
        println!("stderr:");
        println!("{}", build_result.stderr);
    } else {
        println!("(no stderr)");
    }
}
/* vale
func PrintTestFailure(build_result &ExecResult, test_name str, region str) {
  println("Error {build_result.return_code} building test {test_name} (region {region}).");
  if (build_result.stdout.len() > 0) {
    println("stdout:");
    println(build_result.stdout);
  } else {
    println("(no stdout)");
  }
  if (build_result.stderr.len() > 0) {
    println("stderr:");
    println(build_result.stderr);
  } else {
    println("(no stderr)");
  }
}
*/

impl TestSuite {
    #[allow(non_snake_case)]
    pub fn FinishTests(&mut self, until_this_many_left: i64) {
        let suite = self;
        while (suite.test_instances.len() as i64) > until_this_many_left {
            let build_instance = suite.test_instances.remove(0);
            let TestInstance {
                test_name,
                region,
                test_build_dir,
                process: build_process,
                runs_args: runs,
                expect_stdout,
            } = build_instance;

            let build_result = capture_and_join(build_process);
            if build_result.return_code != 0 {
                PrintTestFailure(&build_result, &test_name, &region);
                suite.num_failures = suite.num_failures + 1;
            } else {
                let program_name = if cfg!(windows) { "main.exe" } else { "main" };
                let run_program = test_build_dir.join(program_name).to_string_lossy().into_owned();

                let mut all_runs_succeeded = true;
                for run in &runs {
                    let Step { args, expected_return_code } = run;
                    let command = format!("{} {}", run_program, args.join(" "));
                    let child = Command::new(&run_program)
                        .current_dir(&test_build_dir)
                        .args(args)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("run spawn failed");
                    let run_process = Subprocess { child, command };
                    if suite.verbose {
                        println!("Running command: {}", run_process.command);
                    }
                    let run_result = capture_and_join(run_process);

                    let mut this_run_succeeded = true;
                    let mut print_stdout_stderr = false;
                    if suite.verbose {
                        print_stdout_stderr = true;
                    }
                    if run_result.return_code != *expected_return_code {
                        println!(
                            "Invalid result for test {} (region {}). Expected return {} but got {}.",
                            test_name, region, expected_return_code, run_result.return_code
                        );
                        print_stdout_stderr = true;
                        this_run_succeeded = false;
                    }
                    if expect_stdout.is_some() {
                        let expected_stdout_str = expect_stdout.as_ref().unwrap();
                        if run_result.stdout != *expected_stdout_str {
                            if run_result.stdout.len() > 0 {
                                println!(
                                    "Invalid result for test {} (region {}), expected stdout:",
                                    test_name, region
                                );
                                println!("{}", expected_stdout_str);
                            } else {
                                println!(
                                    "Invalid result for test {} (region {}), expected empty stdout.",
                                    test_name, region
                                );
                            }
                            print_stdout_stderr = true;
                            this_run_succeeded = false;
                        }
                    }

                    if print_stdout_stderr {
                        if run_result.stdout.len() > 0 {
                            println!("Observed stdout:");
                            println!("{}", run_result.stdout);
                        } else {
                            println!("(Observed empty stdout)");
                        }
                        if run_result.stderr.len() > 0 {
                            println!("Observed stderr:");
                            println!("{}", run_result.stderr);
                        } else {
                            println!("(Observed empty stderr)");
                        }
                    }

                    if this_run_succeeded {
                        // Success!
                    } else {
                        all_runs_succeeded = false;
                    }
                }

                if all_runs_succeeded {
                    println!("Test {} (region {}) succeeded!", test_name, region);
                    suite.num_successes = suite.num_successes + 1;
                } else {
                    suite.num_failures = suite.num_failures + 1;
                }
            }
        }
    }
}
/* vale
func FinishTests(suite &TestSuite, until_this_many_left int) {
  while (suite.test_instances.len() > until_this_many_left) {
    build_instance = suite.test_instances.remove(0);
    [test_name, region, test_build_dir, build_process, runs, expect_stdout] = build_instance;

    build_result = (build_process).capture_and_join();
    if (build_result.return_code != 0) {
      PrintTestFailure(&build_result, test_name, region);
      set suite.num_failures = suite.num_failures + 1;
    } else {
      program_name = if (IsWindows()) { "main.exe" } else { "main" };
      run_program = test_build_dir./(program_name).str();

      all_runs_succeeded = true;
      foreach run in &runs {
        [args, expected_return_code] = run;
        run_process =
            SubprocessBuilder()
            .FromDir(test_build_dir.str())
            .WithProgram(run_program)
            .WithArgs(&args)
            .Build()
            .expect();
        if (suite.verbose) {
          println("Running command: " + run_process.command);
        }
        run_result = (run_process).capture_and_join();

        this_run_succeeded = true;
        print_stdout_stderr = false;
        if suite.verbose {
          set print_stdout_stderr = true;
        }
        if run_result.return_code != expected_return_code {
          println("Invalid result for test {test_name} (region {region}). Expected return {expected_return_code} but got {run_result.return_code}.");
          set print_stdout_stderr = true;
          set this_run_succeeded = false;
        }
        if expect_stdout.nonEmpty() {
          expected_stdout_str = expect_stdout.get();
          if run_result.stdout != expected_stdout_str {
            if run_result.stdout.len() > 0 {
              println("Invalid result for test {test_name} (region {region}), expected stdout:");
              println(expected_stdout_str);
            } else {
              println("Invalid result for test {test_name} (region {region}), expected empty stdout.");
            }
            set print_stdout_stderr = true;
            set this_run_succeeded = false;
          }
        }

        if print_stdout_stderr {
          if (run_result.stdout.len() > 0) {
            println("Observed stdout:");
            println(run_result.stdout);
          } else {
            println("(Observed empty stdout)");
          }
          if (run_result.stderr.len() > 0) {
            println("Observed stderr:");
            println(run_result.stderr);
          } else {
            println("(Observed empty stderr)");
          }
        }

        if this_run_succeeded {
          // Success!
        } else {
          set all_runs_succeeded = false;
        }
      }

      if all_runs_succeeded {
        println("Test {test_name} (region {region}) succeeded!");
        set suite.num_successes = suite.num_successes + 1;
      } else {
        set suite.num_failures = suite.num_failures + 1;
      }
    }
  }
}
*/

impl TestSuite {
    #[allow(non_snake_case, dropping_references)]
    pub fn StartTest(
        &mut self,
        expected_return_code: i64,
        test_name: &str,
        vale_input: &Path,
        extra_build_flags: &[String],
        region: &str,
        include_stdlib: bool,
        expect_stdout: Option<String>,
    ) {
        let suite = self;
        if suite.verbose {
            println!("Considering test {}...", test_name);
        }

        if matches_filters(test_name, &suite.test_filters) {
            suite.FinishTests(suite.max_concurrent_tests - 1);

            let test_build_dir = suite.cwd.join(format!("testbuild/{}_{}", test_name, region));
            let build_process = suite.StartBuild(
                test_name,
                vale_input,
                extra_build_flags,
                &test_build_dir,
                region,
                include_stdlib,
            );
            suite.test_instances.push(TestInstance {
                test_name: test_name.to_string(),
                region: region.to_string(),
                test_build_dir,
                process: build_process,
                runs_args: vec![Step {
                    args: Vec::new(),
                    expected_return_code,
                }],
                expect_stdout,
            });
        } else {
            drop(expect_stdout);
            drop(vale_input);
        }
    }
}
/* vale
func StartTest(
    suite &TestSuite,
    expected_return_code int,
    test_name str,
    vale_input Path,
    extra_build_flags &List<str>,
    region str,
    include_stdlib bool,
    expect_stdout Opt<str>) {
  if (suite.verbose) {
    println("Considering test {test_name}...");
  }

  if (matches_filters(test_name, &suite.test_filters)) {
    suite.FinishTests(suite.max_concurrent_tests - 1);

    test_build_dir = suite.cwd./("testbuild/{test_name}_{region}");
    build_process =
        suite.StartBuild(
            test_name, vale_input, &extra_build_flags, &test_build_dir, region, include_stdlib);
    suite.test_instances.add(
        TestInstance(
            test_name, region, test_build_dir, build_process, List<Step>().add(Step(List<str>(), expected_return_code)), expect_stdout));
  } else {
    drop(expect_stdout);
    drop(vale_input);
  }
}
*/

impl TestSuite {
    #[allow(non_snake_case, dead_code, dropping_references)]
    pub fn StartReplayTest(
        &mut self,
        first_expected_return_code: i64,
        repeated_expected_return_code: i64,
        test_name: &str,
        vale_input: &Path,
        specific_extra_build_flags: &[String],
        region: &str,
        include_stdlib: bool,
        expect_stdout: Option<String>,
    ) {
        let suite = self;
        if suite.verbose {
            println!("Considering test {}...", test_name);
        }

        if matches_filters(test_name, &suite.test_filters) {
            suite.FinishTests(suite.max_concurrent_tests - 1);

            let mut extra_build_flags: Vec<String> = Vec::new();
            for flag in specific_extra_build_flags {
                extra_build_flags.push(flag.clone());
            }
            extra_build_flags.push("--enable_replaying".to_string());
            extra_build_flags.push("true".to_string());

            let test_build_dir = suite.cwd.join(format!("testbuild/{}_{}", test_name, region));
            let build_process = suite.StartBuild(
                test_name,
                vale_input,
                &extra_build_flags,
                &test_build_dir,
                region,
                include_stdlib,
            );
            suite.test_instances.push(TestInstance {
                test_name: test_name.to_string(),
                region: region.to_string(),
                test_build_dir,
                process: build_process,
                // See AASETR for why we run the test three times.
                runs_args: vec![
                    Step {
                        args: Vec::new(),
                        expected_return_code: first_expected_return_code,
                    },
                    Step {
                        args: vec!["--vale_record".to_string(), "recording.bin".to_string()],
                        expected_return_code: repeated_expected_return_code,
                    },
                    Step {
                        args: vec!["--vale_replay".to_string(), "recording.bin".to_string()],
                        expected_return_code: repeated_expected_return_code,
                    },
                ],
                expect_stdout,
            });
        } else {
            drop(expect_stdout);
            drop(vale_input);
        }
    }
}
/* vale
func StartReplayTest(
    suite &TestSuite,
    first_expected_return_code int,
    repeated_expected_return_code int,
    test_name str,
    vale_input Path,
    specific_extra_build_flags &List<str>,
    region str,
    include_stdlib bool,
    expect_stdout Opt<str>) {
  if (suite.verbose) {
    println("Considering test {test_name}...");
  }

  if (matches_filters(test_name, &suite.test_filters)) {
    suite.FinishTests(suite.max_concurrent_tests - 1);

    extra_build_flags = List<str>();
    foreach flag in specific_extra_build_flags {
      extra_build_flags.add(flag);
    }
    extra_build_flags.add("--enable_replaying");
    extra_build_flags.add("true");

    test_build_dir = suite.cwd./("testbuild/{test_name}_{region}");
    build_process =
        suite.StartBuild(
            test_name, vale_input, &extra_build_flags, &test_build_dir, region, include_stdlib);
    suite.test_instances.add(
        TestInstance(
            test_name,
            region,
            test_build_dir,
            build_process,
            List<Step>()
                // See AASETR for why we run the test three times.
                .add(Step(List<str>(), first_expected_return_code))
                .add(Step(List<str>().add("--vale_record").add("recording.bin"), repeated_expected_return_code))
                .add(Step(List<str>().add("--vale_replay").add("recording.bin"), repeated_expected_return_code)),
            expect_stdout));
  } else {
    drop(expect_stdout);
    drop(vale_input);
  }
}
*/
