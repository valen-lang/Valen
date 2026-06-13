// ====== beyond-port: rust-interop test path ======
// None of this has a counterpart in Tester/src/main.vale. It scans
// `tests/rust-interop/*.vale`, reads `// expected_exit:` / `// expects_build_fail:`
// headers, and queues each as a test alongside the canonical Vale-corpus suite.

use crate::suite::{matches_filters, Step, TestInstance, TestSuite};
use std::fs;
use std::path::{Path, PathBuf};

pub enum RustInteropOutcome {
    ExpectedExit(i64),
    BuildFail,
}
// no main.vale counterpart (beyond-port: rust-interop addition).

pub struct RustInteropOpts {
    pub include_stdlib: bool,
    pub expected_stdout: Option<String>,
}
// no main.vale counterpart (beyond-port: rust-interop addition).

impl Default for RustInteropOpts {
    fn default() -> Self {
        RustInteropOpts { include_stdlib: false, expected_stdout: None }
    }
}
// no main.vale counterpart (beyond-port: rust-interop addition).

// no main.vale counterpart (beyond-port: rust-interop addition).
pub fn register(
    suite: &mut TestSuite,
    rust_interop_tests_dir: &Path,
    vale_ruster_path: Option<&PathBuf>,
    divination_path: Option<&PathBuf>,
    rust_cargo_toml: Option<&PathBuf>,
) {
    if !rust_interop_tests_dir.is_dir() {
        eprintln!(
            "rust-interop tests dir {} doesn't exist; skipping the rust-interop suite.",
            rust_interop_tests_dir.display()
        );
        return;
    }

    let mut entries: Vec<_> = match fs::read_dir(rust_interop_tests_dir) {
        Ok(it) => it.filter_map(|e| e.ok()).collect(),
        Err(e) => {
            eprintln!("read_dir({}) failed: {}", rust_interop_tests_dir.display(), e);
            return;
        }
    };
    entries.sort_by_key(|e| e.file_name());

    let region = "resilient-v3";

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("vale") {
            continue;
        }
        let test_name = match path.file_stem().and_then(|s| s.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        let (outcome, opts) = match parse_header(&path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Skipping {}: {}", path.display(), e);
                continue;
            }
        };

        start_rust_interop_test(
            suite,
            &test_name,
            &path,
            outcome,
            region,
            opts,
            vale_ruster_path,
            divination_path,
            rust_cargo_toml,
        );
    }
}

// no main.vale counterpart (beyond-port: rust-interop addition).
pub fn start_rust_interop_test(
    suite: &mut TestSuite,
    test_name: &str,
    vale_input: &Path,
    expected_exit_or_buildfail: RustInteropOutcome,
    region: &str,
    opts: RustInteropOpts,
    vale_ruster_path: Option<&PathBuf>,
    divination_path: Option<&PathBuf>,
    rust_cargo_toml: Option<&PathBuf>,
) {
    if suite.verbose {
        println!("Considering rust-interop test {}...", test_name);
    }

    if !matches_filters(test_name, &suite.test_filters) {
        return;
    }

    suite.FinishTests(suite.max_concurrent_tests - 1);

    let test_build_dir = suite.cwd.join(format!("testbuild/{}_{}", test_name, region));

    let mut extra: Vec<String> = Vec::new();
    if let Some(p) = vale_ruster_path {
        extra.push("--vale_ruster_path".to_string());
        extra.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = divination_path {
        extra.push("--divination_path".to_string());
        extra.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = rust_cargo_toml {
        extra.push("--rust_cargo_toml".to_string());
        extra.push(p.to_string_lossy().into_owned());
    }

    let build_process = suite.StartBuild(
        test_name,
        vale_input,
        &extra,
        &test_build_dir,
        region,
        opts.include_stdlib,
    );

    let runs_args = match expected_exit_or_buildfail {
        RustInteropOutcome::ExpectedExit(n) => vec![Step {
            args: Vec::new(),
            expected_return_code: n,
        }],
        RustInteropOutcome::BuildFail => Vec::new(),
    };

    suite.test_instances.push(TestInstance {
        test_name: test_name.to_string(),
        region: region.to_string(),
        test_build_dir,
        process: build_process,
        runs_args,
        expect_stdout: opts.expected_stdout,
    });
}

// no main.vale counterpart (beyond-port: rust-interop addition).
fn parse_header(path: &Path) -> Result<(RustInteropOutcome, RustInteropOpts), String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("read {}: {}", path.display(), e))?;

    let mut outcome: Option<RustInteropOutcome> = None;
    let mut opts = RustInteropOpts::default();

    for line in contents.lines().take(40) {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("// expected_exit:") {
            let n: i64 = rest.trim().parse()
                .map_err(|e| format!("bad expected_exit value in {}: {}", path.display(), e))?;
            outcome = Some(RustInteropOutcome::ExpectedExit(n));
        } else if let Some(rest) = trimmed.strip_prefix("// expects_build_fail:") {
            if rest.trim() == "true" {
                outcome = Some(RustInteropOutcome::BuildFail);
            }
        } else if let Some(rest) = trimmed.strip_prefix("// include_stdlib:") {
            if rest.trim() == "true" {
                opts.include_stdlib = true;
            }
        } else if let Some(rest) = trimmed.strip_prefix("// expected_stdout:") {
            opts.expected_stdout = Some(parse_quoted_string(rest.trim()).ok_or_else(|| {
                format!(
                    "bad expected_stdout in {}: expected a \"...\"-quoted string with optional \\n / \\t / \\\" escapes",
                    path.display()
                )
            })?);
        }
    }

    let outcome = outcome.ok_or_else(|| {
        format!(
            "{} has no '// expected_exit: N' or '// expects_build_fail: true' header",
            path.display()
        )
    })?;
    Ok((outcome, opts))
}

// no main.vale counterpart (beyond-port: rust-interop addition).
fn parse_quoted_string(raw: &str) -> Option<String> {
    let bytes = raw.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'"' || bytes[bytes.len() - 1] != b'"' {
        return None;
    }
    let inner = &raw[1..raw.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next()? {
                'n' => out.push('\n'),
                't' => out.push('\t'),
                'r' => out.push('\r'),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                _ => return None,
            }
        } else {
            out.push(c);
        }
    }
    Some(out)
}
