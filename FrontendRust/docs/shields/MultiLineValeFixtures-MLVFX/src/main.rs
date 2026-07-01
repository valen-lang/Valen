use serde::Deserialize;
use shield_utils::{read_stdin, output_violations, parse_diff, is_in_block_comment, DiffLine};

#[derive(Deserialize)]
struct ProgramInput {
    #[serde(default)]
    diff: String,
}

const VALE_KEYWORDS: &[&str] = &[
    "func", "struct", "interface", "impl", "import", "exported", "abstract", "where",
];

const VIOLATION_MSG: &str =
    "One-line raw string with embedded Vale source; break the fixture into a multi-line raw string.";

/// Scan `line` for raw-string literals whose opening and closing delimiters both
/// appear on this line. For each such literal, yields the string body (the text
/// between the delimiters, exclusive).
fn iter_single_line_raw_strings(line: &str) -> impl Iterator<Item = &str> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes.len() {
        // Look for `r`, optional `#`s, then `"`.
        if bytes[i] == b'r' && (i == 0 || !is_ident_char(bytes[i - 1])) {
            let mut hashes = 0usize;
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] == b'#' {
                hashes += 1;
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'"' {
                let body_start = j + 1;
                // Search for closing `"` followed by exactly `hashes` `#`s.
                let mut k = body_start;
                let mut found_end: Option<usize> = None;
                while k < bytes.len() {
                    if bytes[k] == b'"' {
                        let close_hash_end = k + 1 + hashes;
                        if close_hash_end <= bytes.len()
                            && bytes[k + 1..close_hash_end].iter().all(|&b| b == b'#')
                        {
                            // Must not be followed by another `#` (would mean the actual close
                            // needed even more hashes than `hashes`).
                            let next_is_hash = close_hash_end < bytes.len()
                                && bytes[close_hash_end] == b'#';
                            if !next_is_hash {
                                found_end = Some(k);
                                break;
                            }
                        }
                    }
                    k += 1;
                }
                if let Some(end) = found_end {
                    out.push(&line[body_start..end]);
                    i = end + 1 + hashes;
                    continue;
                }
            }
        }
        i += 1;
    }
    out.into_iter()
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn body_contains_vale_keyword(body: &str) -> bool {
    let bytes = body.as_bytes();
    for &kw in VALE_KEYWORDS {
        let kw_bytes = kw.as_bytes();
        let mut i = 0;
        while i + kw_bytes.len() <= bytes.len() {
            if &bytes[i..i + kw_bytes.len()] == kw_bytes {
                let before_ok = i == 0 || !is_ident_char(bytes[i - 1]);
                let after_ok = i + kw_bytes.len() == bytes.len()
                    || !is_ident_char(bytes[i + kw_bytes.len()]);
                if before_ok && after_ok {
                    return true;
                }
            }
            i += 1;
        }
    }
    false
}

/// Returns true when `body` contains an opening `{` and, after skipping matching
/// pairs, some `{...}` region has non-whitespace inner content. Empty `{}` (with
/// only whitespace inside) does NOT qualify.
fn body_has_nonempty_block(body: &str) -> bool {
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            // Find matching `}` at the same nesting level; check for non-whitespace inside.
            let mut depth: i32 = 1;
            let mut j = i + 1;
            let mut saw_nonws = false;
            while j < bytes.len() && depth > 0 {
                match bytes[j] {
                    b'{' => {
                        depth += 1;
                        saw_nonws = true;
                    }
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        } else {
                            saw_nonws = true;
                        }
                    }
                    b => {
                        if !b.is_ascii_whitespace() {
                            saw_nonws = true;
                        }
                    }
                }
                j += 1;
            }
            if saw_nonws {
                return true;
            }
            i = j + 1;
        } else {
            i += 1;
        }
    }
    false
}

fn run(input: &ProgramInput) -> Vec<String> {
    let lines = parse_diff(&input.diff);
    let mut violations = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let DiffLine::Added(content) = line else { continue };
        let trimmed = content.trim_start();
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }
        if is_in_block_comment(&lines, i) {
            continue;
        }
        for body in iter_single_line_raw_strings(content) {
            if body_contains_vale_keyword(body) && body_has_nonempty_block(body) {
                violations.push(VIOLATION_MSG.to_string());
            }
        }
    }
    violations
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

    fn make_input(diff: &str) -> ProgramInput {
        ProgramInput { diff: diff.to_string() }
    }

    fn assert_deny(diff: &str) {
        let violations = run(&make_input(diff));
        assert!(!violations.is_empty(), "Expected violations for:\n{}", diff);
    }

    fn assert_allow(diff: &str) {
        let violations = run(&make_input(diff));
        assert!(
            violations.is_empty(),
            "Unexpected violations {:?} for:\n{}",
            violations, diff
        );
    }

    #[test]
    fn tracer_bullet_one_liner_with_body_fires() {
        assert_deny("+let code = r#\"exported func main() int { return 42; }\"#;\n");
    }

    #[test]
    fn multi_line_raw_string_is_allowed() {
        assert_allow(
            "+let code = r#\"\n+exported func main() int { return 42; }\n+\"#;\n",
        );
    }

    #[test]
    fn compact_import_only_is_allowed() {
        assert_allow("+let code = r#\"import v.builtins.tup0.*;\"#;\n");
    }

    #[test]
    fn empty_struct_body_one_liner_is_allowed() {
        assert_allow("+let code = r#\"struct X {}\"#;\n");
    }

    #[test]
    fn non_vale_raw_string_with_braces_is_allowed() {
        assert_allow("+let sql = r#\"SELECT * FROM t WHERE {c};\"#;\n");
    }

    #[test]
    fn word_boundary_avoids_substring_match_of_struct() {
        assert_allow("+let s = r#\"the structure {inside} value\"#;\n");
    }

    #[test]
    fn line_comment_containing_pattern_is_ignored() {
        assert_allow("+// let code = r#\"func main() { return 1; }\"#;\n");
    }

    #[test]
    fn block_comment_line_is_ignored() {
        assert_allow(
            "+/* let code = r#\"func main() { return 1; }\"#;\n+   more inside the comment\n+*/\n",
        );
    }

    #[test]
    fn removed_lines_never_fire() {
        // A `-` line removing an old bad pattern must not itself count as a violation.
        let violations = run(&make_input(
            "-let code = r#\"exported func main() int { return 42; }\"#;\n",
        ));
        assert!(violations.is_empty(), "Removed lines produced {:?}", violations);
    }
}
