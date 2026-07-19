use std::io::{self, Read};

#[derive(serde::Deserialize)]
struct ProgramInput {
    #[serde(default)]
    file_path: String,
}

const ALLOWED_EXTENSIONS: &[&str] = &[".rs", ".md", ".cpp", ".c", ".h", ".vale"];

fn check(file_path: &str) -> Vec<String> {
    if file_path.is_empty() || ALLOWED_EXTENSIONS.iter().any(|ext| file_path.ends_with(ext)) {
        vec![]
    } else {
        vec![format!("File extension not allowed (only .rs, .md, .cpp, .c, .h, .vale may be edited): {}", file_path)]
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
        assert!(run(&make_input("/Volumes/V/Vale1/FrontendRust/src/lib.rs")).is_empty());
    }

    #[test]
    fn allow_md_file() {
        assert!(run(&make_input("/Volumes/V/Vale1/docs/README.md")).is_empty());
    }

    #[test]
    fn allow_cpp_file() {
        assert!(run(&make_input("/Volumes/V/Vale1/Backend/src/externs.cpp")).is_empty());
    }

    #[test]
    fn allow_c_file() {
        assert!(run(&make_input("/Volumes/V/Vale1/Backend/builtins/strings.c")).is_empty());
    }

    #[test]
    fn allow_h_file() {
        assert!(run(&make_input("/Volumes/V/Vale1/Backend/builtins/ValeBuiltins.h")).is_empty());
    }

    #[test]
    fn allow_vale_file() {
        assert!(run(&make_input("/Volumes/V/Vale1/FrontendRust/src/tests/programs/virtuals/interfaceimm.vale")).is_empty());
    }

    #[test]
    fn allow_empty_path() {
        assert!(run(&make_input("")).is_empty());
    }

    #[test]
    fn deny_py_file() {
        let v = run(&make_input("/Volumes/V/Vale1/scripts/build.py"));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn deny_toml_file() {
        assert!(!run(&make_input("/Volumes/V/Vale1/Cargo.toml")).is_empty());
    }

    #[test]
    fn deny_sh_file() {
        assert!(!run(&make_input("/Volumes/V/Vale1/scripts/deploy.sh")).is_empty());
    }

    #[test]
    fn deny_hpp_file_not_in_allowlist() {
        // .hpp is a distinct extension from .h and must not slip through
        assert!(!run(&make_input("/Volumes/V/Vale1/Backend/src/foo.hpp")).is_empty());
    }

    #[test]
    fn deny_violation_message_contains_path() {
        let violations = run(&make_input("/Volumes/V/Vale1/scripts/build.py"));
        assert!(violations[0].contains("/Volumes/V/Vale1/scripts/build.py"));
    }
}
