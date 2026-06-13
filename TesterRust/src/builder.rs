use crate::suite::{Subprocess, TestSuite};
use std::path::Path;
use std::process::{Command, Stdio};

impl TestSuite {
    #[allow(non_snake_case)]
    pub fn StartBuild(
        &self,
        test_name: &str,
        vale_code_path: &Path,
        extra_build_flags: &[String],
        test_build_dir: &Path,
        region: &str,
        include_stdlib: bool,
    ) -> Subprocess {
        let suite = self;
        println!("Starting {}, region {}...", test_name, region);

        let mut build_args: Vec<String> = Vec::new();
        suite.common_build_args.iter().for_each(|arg| build_args.push(arg.clone()));
        build_args.push(format!("vtest={}", vale_code_path.to_string_lossy()));
        build_args.push(format!(
            "vtest={}",
            suite.backend_tests_dir.join("testbuiltins.c").to_string_lossy()
        ));
        build_args.push("--output_dir".to_string());
        build_args.push(test_build_dir.to_string_lossy().into_owned());
        build_args.push("--region_override".to_string());
        build_args.push(region.to_string());
        build_args.push("--llvm_ir".to_string());
        build_args.push("true".to_string());
        build_args.push("--opt_level".to_string());
        build_args.push("O0".to_string());
        if suite.flares {
            build_args.push("--flares".to_string());
            build_args.push("true".to_string());
        }
        //build_args.push("--asan".to_string());
        //build_args.push("true".to_string());
        //build_args.push("--census".to_string());
        //build_args.push("true".to_string());
        //build_args.push("--enable_side_calling".to_string());
        //build_args.push("true".to_string());

        // Never include the stdlib that came with the bootstrapping compiler.
        build_args.push("--no_std".to_string());
        build_args.push("true".to_string());
        // If we want a stdlib, we use the one that's at HEAD, in this repo.
        if include_stdlib {
            build_args.push(format!(
                "stdlib={}",
                suite.stdlib_dir.join("src").to_string_lossy()
            ));
        }

        extra_build_flags.iter().for_each(|flag| build_args.push(flag.clone()));

        if suite.verbose {
            println!("Starting subprocess...");
        }

        let command = format!(
            "{} {}",
            suite.valec_path.to_string_lossy(),
            build_args.join(" ")
        );
        let child = Command::new(&suite.valec_path)
            .args(&build_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("StartBuild spawn failed");
        let build_process = Subprocess { child, command };
        if suite.verbose {
            println!("Build command: {}", build_process.command);
        }
        build_process
    }
}
/* vale
func StartBuild(
    suite &TestSuite,
    test_name str,
    vale_code_path Path,
    extra_build_flags &List<str>,
    test_build_dir &Path,
    region str,
    include_stdlib bool)
Subprocess {
  println("Starting {test_name}, region {region}...");

  build_args = List<str>();
  suite.common_build_args.each((arg) => { build_args.add(arg); });
  build_args.add("vtest=" + vale_code_path.str());
  build_args.add("vtest=" + suite.backend_tests_dir./("testbuiltins.c").str());
  build_args.add("--output_dir");
  build_args.add(test_build_dir.str());
  build_args.add("--region_override");
  build_args.add(region);
  build_args.add("--llvm_ir");
  build_args.add("true");
  build_args.add("--opt_level");
  build_args.add("O0");
  if suite.flares {
    build_args.add("--flares");
    build_args.add("true");
  }
  //build_args.add("--asan");
  //build_args.add("true");
  //build_args.add("--census");
  //build_args.add("true");
  //build_args.add("--enable_side_calling");
  //build_args.add("true");

  // Never include the stdlib that came with the bootstrapping compiler.
  build_args.add("--no_std");
  build_args.add("true");
  // If we want a stdlib, we use the one that's at HEAD, in this repo.
  if include_stdlib {
    build_args.add("stdlib=" + suite.stdlib_dir./("src").str());
  }

  extra_build_flags.each((flag) => {
    build_args.add(flag);
  });

  if (suite.verbose) {
    println("Starting subprocess...");
  }

  build_process =
      SubprocessBuilder()
      .WithProgram(suite.valec_path.str())
      .WithArgs(&build_args)
      .Build()
      .expect();
  if (suite.verbose) {
    println("Build command: " + build_process.command);
  }
  return build_process;
}
*/
