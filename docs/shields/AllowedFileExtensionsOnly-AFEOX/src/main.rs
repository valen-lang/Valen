use std::io::{self, Read};

#[derive(serde::Deserialize)]
struct ProgramInput {
    #[serde(default)]
    file_path: String,
}

const ALLOWED_EXTENSIONS: &[&str] = &[".rs", ".md", ".cpp", ".c", ".h", ".vale"];

// Python scripts staged for safe-script-runner live in tmp/scripts/ (see docs/skills/scripting.md);
// the hook may pass the path as either relative or absolute.
fn is_safe_script_runner_script(file_path: &str) -> bool {
    file_path.ends_with(".py")
        && (file_path.starts_with("tmp/scripts/") || file_path.contains("/tmp/scripts/"))
}

fn check(file_path: &str) -> Vec<String> {
    if file_path.is_empty()
        || ALLOWED_EXTENSIONS.iter().any(|ext| file_path.ends_with(ext))
        || is_safe_script_runner_script(file_path)
    {
        vec![]
    } else {
        vec![format!("File extension not allowed (only .rs, .md, .cpp, .c, .h, .vale may be edited; .py only under tmp/scripts/): {}", file_path)]
    }
}

fn run(input: &ProgramInput) -> Vec<String> {
    check(&input.file_path)
}

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    input
}

fn output_violations(violations: Vec<String>) {
    if violations.is_empty() {
        println!("{{\"violations\":[]}}");
    } else {
        let result = serde_json::json!({
            "violations": violations.iter()
                .map(|r| serde_json::json!({"reason": r}))
                .collect::<Vec<_>>()
        });
        println!("{}", result);
    }
}

fn main() {
    let raw = read_stdin();
    let input: ProgramInput = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("Failed to parse ProgramInput JSON: {}", e));
    output_violations(run(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input(file_path: &str) -> ProgramInput {
        ProgramInput { file_path: file_path.to_string() }
    }

    #[test]
    fn allow_rs_file() {
        assert!(run(&make_input("src/lib.rs")).is_empty());
    }

    #[test]
    fn allow_md_file() {
        assert!(run(&make_input("docs/README.md")).is_empty());
    }

    #[test]
    fn allow_cpp_file() {
        assert!(run(&make_input("Backend/src/externs.cpp")).is_empty());
    }

    #[test]
    fn allow_c_file() {
        assert!(run(&make_input("Backend/builtins/strings.c")).is_empty());
    }

    #[test]
    fn allow_h_file() {
        assert!(run(&make_input("Backend/builtins/ValeBuiltins.h")).is_empty());
    }

    #[test]
    fn allow_vale_file() {
        assert!(run(&make_input("src/tests/programs/virtuals/interfaceimm.vale")).is_empty());
    }

    #[test]
    fn allow_empty_path() {
        assert!(run(&make_input("")).is_empty());
    }

    #[test]
    fn allow_py_in_tmp_scripts_relative() {
        assert!(run(&make_input("tmp/scripts/migrate.py")).is_empty());
    }

    #[test]
    fn allow_py_in_tmp_scripts_absolute() {
        assert!(run(&make_input("/some/checkout/tmp/scripts/migrate-corpus-imports.py")).is_empty());
    }

    #[test]
    fn deny_py_in_tmp_outside_scripts() {
        assert_eq!(run(&make_input("tmp/migrate.py")).len(), 1);
    }

    #[test]
    fn deny_non_py_in_tmp_scripts() {
        assert_eq!(run(&make_input("tmp/scripts/helper.sh")).len(), 1);
    }

    #[test]
    fn deny_py_file() {
        let v = run(&make_input("scripts/build.py"));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn deny_toml_file() {
        assert!(!run(&make_input("Cargo.toml")).is_empty());
    }

    #[test]
    fn deny_sh_file() {
        assert!(!run(&make_input("scripts/deploy.sh")).is_empty());
    }

    #[test]
    fn deny_hpp_file_not_in_allowlist() {
        // .hpp is a distinct extension from .h and must not slip through
        assert!(!run(&make_input("Backend/src/foo.hpp")).is_empty());
    }

    #[test]
    fn deny_violation_message_contains_path() {
        let violations = run(&make_input("scripts/build.py"));
        assert!(violations[0].contains("scripts/build.py"));
    }
}
