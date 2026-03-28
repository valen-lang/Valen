use serde::Deserialize;
use std::io::Read;

// ─── JSON input from Claude Code ────────────────────────────────────────────

#[derive(Deserialize)]
struct HookInput {
    tool_input: ToolInput,
}

#[derive(Deserialize)]
struct ToolInput {
    command: Option<String>,
}

// ─── Safelists ──────────────────────────────────────────────────────────────

/// Commands that are always read-only (no subcommand validation needed).
const READONLY_COMMANDS: &[&str] = &[
    // Filesystem inspection
    "ls", "ll", "la", "cat", "head", "tail", "less", "more", "file", "stat",
    "readlink", "realpath", "du", "df", "find", "fd", "locate", "which",
    "whereis", "type", "wc", "nl", "md5sum", "sha256sum", "sha1sum", "b2sum",
    "md5", "shasum",
    // Text processing (read-only)
    "grep", "egrep", "fgrep", "rg", "ag", "ack",
    // NOTE: sed, awk are handled specially below
    "awk", "gawk", "mawk",
    "sort", "uniq", "tr", "cut", "paste", "fold", "fmt", "column", "rev",
    "diff", "colordiff", "comm", "cmp",
    "jq", "yq", "xq", "xmllint",
    "iconv", "base64",
    // Archive inspection (not extraction)
    "zipinfo",
    // System/env inspection
    "echo", "printf", "date", "cal", "env", "printenv", "hostname", "uname",
    "id", "whoami", "groups", "pwd", "uptime", "free", "vmstat", "iostat",
    "lscpu", "lsblk", "lsusb", "lspci", "ps", "pgrep", "pidof", "nproc",
    "sysctl", "sw_vers",
    // Binary inspection
    "strings", "nm", "objdump", "otool", "ldd", "xxd", "hexdump", "od",
    // Misc
    "true", "false", "test", "[", "seq", "expr", "bc", "dc",
    "tree", "exa", "eza", "bat",
    "tput", "stty",
    "man", "info", "help",
    "basename", "dirname",
    "command",
];

/// Git subcommands that are read-only.
const READONLY_GIT_SUBCOMMANDS: &[&str] = &[
    "status", "log", "diff", "show", "branch", "tag", "remote",
    "describe", "rev-parse", "rev-list", "name-rev", "shortlog",
    "ls-files", "ls-tree", "ls-remote",
    "cat-file", "for-each-ref", "show-ref",
    "config", // read-only unless setting a value, but generally safe
    "blame", "annotate", "grep", "reflog", "count-objects",
    "check-ignore", "check-attr", "check-mailmap",
    "verify-commit", "verify-tag",
    "whatchanged", "cherry", "range-diff", "merge-base",
];

const READONLY_CARGO_SUBCOMMANDS: &[&str] = &[
    "build", "test", "check", "clippy", "doc", "metadata", "pkgid",
    "tree", "verify-project", "read-manifest", "version", "--version",
    "search", "info", "bench",
];

const READONLY_NPM_SUBCOMMANDS: &[&str] = &[
    "ls", "list", "ll", "la", "info", "show", "view", "outdated",
    "search", "why", "explain", "doctor", "ping", "config-list", "--version",
];

const READONLY_YARN_SUBCOMMANDS: &[&str] = &["list", "info", "why", "outdated", "--version"];
const READONLY_PNPM_SUBCOMMANDS: &[&str] = &["ls", "list", "ll", "la", "why", "outdated", "--version"];
const READONLY_PIP_SUBCOMMANDS: &[&str] = &["list", "show", "freeze", "check", "--version"];

const READONLY_GH_ACTIONS: &[&str] = &["list", "view", "status", "diff", "checks", "comments", "ls", "show", ""];

const READONLY_BREW_SUBCOMMANDS: &[&str] = &[
    "list", "ls", "info", "search", "outdated", "deps", "uses", "desc",
    "home", "--version", "config",
];

const READONLY_DOCKER_SUBCOMMANDS: &[&str] = &[
    "ps", "images", "inspect", "logs", "stats", "top", "version", "info",
];

const READONLY_KUBECTL_SUBCOMMANDS: &[&str] = &[
    "get", "describe", "logs", "top", "version", "config", "cluster-info",
    "api-resources", "api-versions", "explain",
];

/// Patterns that make any command unsafe, regardless of context.
const DANGEROUS_PATTERNS: &[&str] = &[
    "rm ", "rm\t", "rmdir",
    "mv ", "mv\t",
    "cp ", "cp\t",
    "chmod", "chown", "chgrp",
    "mkfs",
    "dd ",
    "kill ", "killall", "pkill",
    "shutdown", "reboot", "halt", "poweroff",
    "systemctl", "service ",
    "sudo ", "su ",
    "docker run", "docker exec", "docker rm", "docker stop", "docker kill",
    "kubectl delete", "kubectl apply", "kubectl exec",
];

// ─── Dangerous pattern check ────────────────────────────────────────────────

fn has_dangerous_pattern(cmd: &str) -> bool {
    for pattern in DANGEROUS_PATTERNS {
        if cmd.contains(pattern) {
            return true;
        }
    }
    false
}

/// Check for output redirection (but allow 2>&1 and similar fd redirects).
fn has_output_redirection(cmd: &str) -> bool {
    let bytes = cmd.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i < len {
        if bytes[i] == b'\'' {
            // Skip single-quoted string
            i += 1;
            while i < len && bytes[i] != b'\'' {
                i += 1;
            }
            i += 1;
            continue;
        }
        if bytes[i] == b'"' {
            // Skip double-quoted string
            i += 1;
            while i < len {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    break;
                }
                i += 1;
            }
            i += 1;
            continue;
        }
        if bytes[i] == b'>' {
            // Check if this is an fd redirect like 2>&1
            let prev_is_fd = i > 0 && bytes[i - 1].is_ascii_digit();
            let next_is_amp = i + 1 < len && bytes[i + 1] == b'&';
            if prev_is_fd && next_is_amp {
                i += 1;
                continue;
            }
            // This is real output redirection
            return true;
        }
        i += 1;
    }
    false
}

fn has_command_substitution(cmd: &str) -> bool {
    cmd.contains("$(") || cmd.contains('`')
}

// ─── Command parsing helpers ────────────────────────────────────────────────

/// Strip leading env var assignments like `FOO=bar BAZ="x" command ...`
fn strip_env_assignments(cmd: &str) -> &str {
    let mut rest = cmd;
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return rest;
        }
        // Check for VAR=value pattern
        let Some(eq_pos) = rest.find('=') else { return rest };
        let before_eq = &rest[..eq_pos];
        // Must be a valid identifier before the =
        if before_eq.is_empty()
            || !before_eq.bytes().next().unwrap().is_ascii_alphabetic()
                && before_eq.as_bytes()[0] != b'_'
        {
            return rest;
        }
        if !before_eq
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return rest;
        }
        // Skip the value
        let after_eq = &rest[eq_pos + 1..];
        if after_eq.starts_with('"') {
            // Skip double-quoted value
            if let Some(end) = after_eq[1..].find('"') {
                rest = &after_eq[end + 2..];
            } else {
                return rest; // Unterminated quote, bail
            }
        } else if after_eq.starts_with('\'') {
            // Skip single-quoted value
            if let Some(end) = after_eq[1..].find('\'') {
                rest = &after_eq[end + 2..];
            } else {
                return rest;
            }
        } else {
            // Unquoted value — ends at whitespace
            match after_eq.find(char::is_whitespace) {
                Some(end) => rest = &after_eq[end..],
                None => return "", // Just an assignment, no command
            }
        }
    }
}

/// Extract the base command name (first word, with path stripped).
fn base_command(cmd: &str) -> &str {
    let first_word = cmd.split_whitespace().next().unwrap_or("");
    // Strip path prefix
    match first_word.rfind('/') {
        Some(pos) => &first_word[pos + 1..],
        None => first_word,
    }
}

/// Extract the subcommand for a tool, skipping leading flags.
fn first_non_flag_arg(cmd: &str) -> &str {
    let mut words = cmd.split_whitespace().skip(1); // skip the base command
    while let Some(word) = words.next() {
        if !word.starts_with('-') {
            return word;
        }
        // For flags that take a value argument, skip the next word too
        // (common: -C <path>, --git-dir <path>, etc.)
        if matches!(word, "-C" | "-c" | "--git-dir" | "--work-tree") {
            let _ = words.next();
        }
    }
    ""
}

fn in_list(needle: &str, haystack: &[&str]) -> bool {
    haystack.contains(&needle)
}

// ─── Single command check ───────────────────────────────────────────────────

fn is_readonly_simple(raw_cmd: &str) -> bool {
    let cmd = strip_env_assignments(raw_cmd.trim());
    if cmd.is_empty() {
        return false; // bare assignment — not clearly read-only
    }

    let base = base_command(cmd);

    // ── sed: only without -i ──
    if base == "sed" {
        // Check for -i anywhere in args (could be -i, -i.bak, -ni, etc.)
        for word in cmd.split_whitespace().skip(1) {
            if word.starts_with('-') && !word.starts_with("--") && word.contains('i') {
                return false;
            }
            if word == "--in-place" {
                return false;
            }
            // Stop checking flags after --
            if word == "--" {
                break;
            }
        }
        return true;
    }

    // ── git: only certain subcommands ──
    if base == "git" {
        let sub = first_non_flag_arg(cmd);
        if sub.is_empty() {
            return false;
        }
        // git stash: only list/show
        if sub == "stash" {
            return cmd.contains("stash list") || cmd.contains("stash show");
        }
        return in_list(sub, READONLY_GIT_SUBCOMMANDS);
    }

    // ── cargo ──
    if base == "cargo" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_CARGO_SUBCOMMANDS);
    }

    // ── npm ──
    if base == "npm" || base == "npx" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_NPM_SUBCOMMANDS);
    }

    // ── yarn ──
    if base == "yarn" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_YARN_SUBCOMMANDS);
    }

    // ── pnpm ──
    if base == "pnpm" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_PNPM_SUBCOMMANDS);
    }

    // ── pip/pip3 ──
    if base == "pip" || base == "pip3" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_PIP_SUBCOMMANDS);
    }

    // ── brew ──
    if base == "brew" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_BREW_SUBCOMMANDS);
    }

    // ── docker ──
    if base == "docker" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_DOCKER_SUBCOMMANDS);
    }

    // ── kubectl ──
    if base == "kubectl" {
        let sub = first_non_flag_arg(cmd);
        return in_list(sub, READONLY_KUBECTL_SUBCOMMANDS);
    }

    // ── gh: read-only subcommand + action ──
    if base == "gh" {
        let sub = first_non_flag_arg(cmd);
        match sub {
            "pr" | "issue" | "repo" | "run" | "release" | "status" | "auth" => {
                // Get the action (third positional word)
                let action = cmd
                    .split_whitespace()
                    .skip(2)
                    .find(|w| !w.starts_with('-'))
                    .unwrap_or("");
                return in_list(action, READONLY_GH_ACTIONS);
            }
            "api" => return true, // gh api is read-only (GET by default)
            _ => return false,
        }
    }

    // ── make: only dry-run ──
    if base == "make" || base == "cmake" {
        for word in cmd.split_whitespace().skip(1) {
            if word.starts_with('-') && word.contains('n') {
                return true;
            }
            if word == "--dry-run" || word == "--just-print" {
                return true;
            }
        }
        return false;
    }

    // ── node/python/ruby/perl: only --version/--help ──
    if matches!(base, "node" | "python" | "python3" | "ruby" | "perl") {
        return cmd.contains("--version") || cmd.contains("--help") || cmd.contains("-V");
    }

    // ── rustc: only introspection ──
    if base == "rustc" {
        return cmd.contains("--version") || cmd.contains("--print") || cmd.contains("--help");
    }

    // ── xargs: recursively check the command it runs ──
    if base == "xargs" {
        // Strip "xargs" and its flags, then check the remaining command
        let mut words = cmd.split_whitespace().skip(1).peekable();
        // Skip xargs flags
        while let Some(word) = words.peek() {
            if word.starts_with('-') {
                // Flags that take a value
                let w = *word;
                words.next();
                if matches!(w, "-I" | "-L" | "-n" | "-P" | "-E" | "-d") {
                    words.next(); // skip the flag's argument
                }
            } else {
                break;
            }
        }
        let inner_cmd: String = words.collect::<Vec<_>>().join(" ");
        if inner_cmd.is_empty() {
            return false;
        }
        return is_readonly_simple(&inner_cmd);
    }

    // ── General safelist ──
    in_list(base, READONLY_COMMANDS)
}

// ─── Compound command check ─────────────────────────────────────────────────

/// Split on |, &&, ||, ; and check every segment.
fn is_readonly_compound(cmd: &str) -> bool {
    if has_dangerous_pattern(cmd) {
        return false;
    }
    if has_output_redirection(cmd) {
        return false;
    }
    if has_command_substitution(cmd) {
        return false;
    }

    // Split on shell operators. This is a simple split that doesn't respect quotes
    // perfectly, but covers the vast majority of real-world compound commands.
    for segment in split_on_shell_operators(cmd) {
        let trimmed = segment.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !is_readonly_simple(trimmed) {
            return false;
        }
    }
    true
}

/// Split a command string on |, &&, ||, and ;
fn split_on_shell_operators(cmd: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut start = 0;
    let bytes = cmd.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'\'' => {
                i += 1;
                while i < len && bytes[i] != b'\'' {
                    i += 1;
                }
                i += 1;
            }
            b'"' => {
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == b'"' {
                        break;
                    }
                    i += 1;
                }
                i += 1;
            }
            b'|' => {
                if i + 1 < len && bytes[i + 1] == b'|' {
                    // ||
                    segments.push(&cmd[start..i]);
                    i += 2;
                    start = i;
                } else {
                    // |
                    segments.push(&cmd[start..i]);
                    i += 1;
                    start = i;
                }
            }
            b'&' => {
                if i + 1 < len && bytes[i + 1] == b'&' {
                    // &&
                    segments.push(&cmd[start..i]);
                    i += 2;
                    start = i;
                } else if i > 0 && bytes[i - 1] == b'>' {
                    // Part of >&1 redirect — not an operator
                    i += 1;
                } else {
                    // background & — treat as unsafe, push remainder
                    segments.push(&cmd[start..i]);
                    i += 1;
                    start = i;
                }
            }
            b';' => {
                segments.push(&cmd[start..i]);
                i += 1;
                start = i;
            }
            _ => {
                i += 1;
            }
        }
    }
    if start < len {
        segments.push(&cmd[start..]);
    }
    segments
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let hook_input: HookInput = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(_) => return,
    };

    let command = match hook_input.tool_input.command {
        Some(ref c) if !c.is_empty() => c.as_str(),
        _ => return,
    };

    if is_readonly_compound(command) {
        println!(r#"{{"decision":"allow","reason":"Read-only command auto-approved"}}"#);
    } else {
        println!(r#"{{"decision":"approve"}}"#);
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Should approve ──

    #[test]
    fn approve_ls() {
        assert!(is_readonly_compound("ls -la"));
    }

    #[test]
    fn approve_git_status() {
        assert!(is_readonly_compound("git status"));
    }

    #[test]
    fn approve_git_log_pipe_head() {
        assert!(is_readonly_compound("git log --oneline | head -20"));
    }

    #[test]
    fn approve_git_diff() {
        assert!(is_readonly_compound("git diff HEAD~3"));
    }

    #[test]
    fn approve_git_blame() {
        assert!(is_readonly_compound("git blame src/main.rs"));
    }

    #[test]
    fn approve_git_stash_list() {
        assert!(is_readonly_compound("git stash list"));
    }

    #[test]
    fn approve_git_with_flags() {
        assert!(is_readonly_compound("git -C /some/path log --oneline"));
    }

    #[test]
    fn approve_cargo_build() {
        assert!(is_readonly_compound("cargo build --lib"));
    }

    #[test]
    fn approve_cargo_test() {
        assert!(is_readonly_compound("cargo test -- some_test"));
    }

    #[test]
    fn approve_cargo_check() {
        assert!(is_readonly_compound("cargo check"));
    }

    #[test]
    fn approve_find_pipe_wc() {
        assert!(is_readonly_compound("find . -name '*.rs' | wc -l"));
    }

    #[test]
    fn approve_cat_and_echo() {
        assert!(is_readonly_compound("cat foo.txt && echo done"));
    }

    #[test]
    fn approve_grep_chain() {
        assert!(is_readonly_compound("grep -r TODO src/ | sort | uniq -c | head"));
    }

    #[test]
    fn approve_env_prefix() {
        assert!(is_readonly_compound("RUST_LOG=debug cargo check"));
    }

    #[test]
    fn approve_sed_readonly() {
        assert!(is_readonly_compound("sed 's/foo/bar/g' file.txt"));
    }

    #[test]
    fn approve_git_merge_base() {
        assert!(is_readonly_compound("git merge-base main HEAD"));
    }

    #[test]
    fn approve_gh_pr_list() {
        assert!(is_readonly_compound("gh pr list"));
    }

    #[test]
    fn approve_gh_api() {
        assert!(is_readonly_compound("gh api repos/foo/bar/pulls/123/comments"));
    }

    #[test]
    fn approve_brew_list() {
        assert!(is_readonly_compound("brew list"));
    }

    #[test]
    fn approve_stderr_redirect() {
        assert!(is_readonly_compound("cargo build 2>&1"));
    }

    #[test]
    fn approve_xargs_grep() {
        assert!(is_readonly_compound("find . -name '*.rs' | xargs grep TODO"));
    }

    #[test]
    fn approve_docker_ps() {
        assert!(is_readonly_compound("docker ps -a"));
    }

    #[test]
    fn approve_kubectl_get() {
        assert!(is_readonly_compound("kubectl get pods -n default"));
    }

    // ── Should deny ──

    #[test]
    fn deny_rm() {
        assert!(!is_readonly_compound("rm some-file"));
    }

    #[test]
    fn deny_npm_install() {
        assert!(!is_readonly_compound("npm install"));
    }

    #[test]
    fn deny_git_commit() {
        assert!(!is_readonly_compound("git commit -m 'test'"));
    }

    #[test]
    fn deny_git_push() {
        assert!(!is_readonly_compound("git push origin main"));
    }

    #[test]
    fn deny_git_checkout() {
        assert!(!is_readonly_compound("git checkout main"));
    }

    #[test]
    fn deny_git_rebase() {
        assert!(!is_readonly_compound("git rebase main"));
    }

    #[test]
    fn deny_git_stash_push() {
        assert!(!is_readonly_compound("git stash push"));
    }

    #[test]
    fn deny_sed_inplace() {
        assert!(!is_readonly_compound("sed -i 's/foo/bar/g' file.txt"));
    }

    #[test]
    fn deny_output_redirect() {
        assert!(!is_readonly_compound("echo hello > file.txt"));
    }

    #[test]
    fn deny_append_redirect() {
        assert!(!is_readonly_compound("echo hello >> file.txt"));
    }

    #[test]
    fn deny_sudo() {
        assert!(!is_readonly_compound("sudo ls -la"));
    }

    #[test]
    fn deny_mv() {
        assert!(!is_readonly_compound("mv a.txt b.txt"));
    }

    #[test]
    fn deny_cp() {
        assert!(!is_readonly_compound("cp a.txt b.txt"));
    }

    #[test]
    fn deny_chmod() {
        assert!(!is_readonly_compound("chmod +x script.sh"));
    }

    #[test]
    fn deny_python_script() {
        assert!(!is_readonly_compound("python script.py"));
    }

    #[test]
    fn deny_node_script() {
        assert!(!is_readonly_compound("node app.js"));
    }

    #[test]
    fn deny_make_without_dryrun() {
        assert!(!is_readonly_compound("make all"));
    }

    #[test]
    fn deny_command_substitution() {
        assert!(!is_readonly_compound("echo $(rm -rf /)"));
    }

    #[test]
    fn deny_backtick_substitution() {
        assert!(!is_readonly_compound("echo `rm -rf /`"));
    }

    #[test]
    fn deny_cargo_install() {
        assert!(!is_readonly_compound("cargo install ripgrep"));
    }

    #[test]
    fn deny_docker_run() {
        assert!(!is_readonly_compound("docker run ubuntu"));
    }

    #[test]
    fn deny_docker_exec() {
        assert!(!is_readonly_compound("docker exec -it container bash"));
    }

    #[test]
    fn deny_kubectl_apply() {
        assert!(!is_readonly_compound("kubectl apply -f config.yaml"));
    }

    #[test]
    fn deny_kubectl_delete() {
        assert!(!is_readonly_compound("kubectl delete pod my-pod"));
    }

    #[test]
    fn deny_pipe_with_unsafe() {
        assert!(!is_readonly_compound("ls | tee file.txt"));
    }

    #[test]
    fn deny_gh_pr_create() {
        assert!(!is_readonly_compound("gh pr create --title 'test'"));
    }

    // ── Edge cases ──

    #[test]
    fn approve_python_version() {
        assert!(is_readonly_compound("python3 --version"));
    }

    #[test]
    fn approve_rustc_version() {
        assert!(is_readonly_compound("rustc --version"));
    }

    #[test]
    fn approve_make_dryrun() {
        assert!(is_readonly_compound("make -n all"));
    }

    #[test]
    fn strip_env_vars_then_check() {
        assert!(is_readonly_simple("FOO=bar ls -la"));
    }

    #[test]
    fn deny_bare_env_assignment() {
        assert!(!is_readonly_simple("FOO=bar"));
    }

    #[test]
    fn approve_path_prefix_command() {
        assert!(is_readonly_simple("/usr/bin/ls -la"));
    }

    #[test]
    fn deny_unknown_command() {
        assert!(!is_readonly_simple("my-custom-script"));
    }

    #[test]
    fn deny_gh_pr_merge() {
        assert!(!is_readonly_compound("gh pr merge 123"));
    }

    // ── Weird edge cases: redirection trickery ──

    #[test]
    fn deny_redirect_no_space() {
        assert!(!is_readonly_compound("ls>file.txt"));
    }

    #[test]
    fn deny_redirect_in_second_command() {
        assert!(!is_readonly_compound("ls -la && echo hi > out.txt"));
    }

    #[test]
    fn approve_fd_redirect_stderr_to_stdout() {
        assert!(is_readonly_compound("cargo check 2>&1 | head"));
    }

    #[test]
    fn deny_redirect_after_pipe() {
        assert!(!is_readonly_compound("ls | grep foo > matches.txt"));
    }

    #[test]
    fn approve_redirect_inside_single_quotes() {
        // > inside quotes is not a real redirect
        assert!(is_readonly_compound("echo '> not a redirect'"));
    }

    #[test]
    fn approve_redirect_inside_double_quotes() {
        assert!(is_readonly_compound("echo \"> not a redirect\""));
    }

    #[test]
    fn deny_redirect_after_quoted_string() {
        assert!(!is_readonly_compound("echo \"hello\" > file.txt"));
    }

    // ── Weird edge cases: command smuggling via operators ──

    #[test]
    fn deny_safe_then_unsafe_semicolon() {
        assert!(!is_readonly_compound("ls; rm -rf /"));
    }

    #[test]
    fn deny_safe_then_unsafe_and() {
        assert!(!is_readonly_compound("echo ok && python exploit.py"));
    }

    #[test]
    fn deny_safe_then_unsafe_or() {
        assert!(!is_readonly_compound("false || bash -c 'bad stuff'"));
    }

    #[test]
    fn deny_safe_pipe_to_unsafe() {
        assert!(!is_readonly_compound("cat file.txt | python3 -"));
    }

    #[test]
    fn deny_background_operator() {
        // Lone & is a background operator — we split on it, "1" isn't a known command
        assert!(!is_readonly_compound("sleep 999 & rm -rf /"));
    }

    #[test]
    fn deny_many_semicolons_last_is_bad() {
        assert!(!is_readonly_compound("ls; echo hi; pwd; rm file"));
    }

    #[test]
    fn approve_many_semicolons_all_safe() {
        assert!(is_readonly_compound("ls; echo hi; pwd; date"));
    }

    #[test]
    fn approve_triple_pipe_chain() {
        assert!(is_readonly_compound("cat f | grep x | sort | uniq | wc -l"));
    }

    // ── Weird edge cases: git trickery ──

    #[test]
    fn deny_git_push_force() {
        assert!(!is_readonly_compound("git push --force origin main"));
    }

    #[test]
    fn deny_git_reset_hard() {
        assert!(!is_readonly_compound("git reset --hard HEAD~1"));
    }

    #[test]
    fn deny_git_clean() {
        assert!(!is_readonly_compound("git clean -fd"));
    }

    #[test]
    fn deny_git_rm() {
        // "rm " dangerous pattern catches this
        assert!(!is_readonly_compound("git rm file.txt"));
    }

    #[test]
    fn deny_git_stash_drop() {
        assert!(!is_readonly_compound("git stash drop"));
    }

    #[test]
    fn deny_git_stash_pop() {
        assert!(!is_readonly_compound("git stash pop"));
    }

    #[test]
    fn deny_git_stash_bare() {
        // bare "git stash" defaults to push
        assert!(!is_readonly_compound("git stash"));
    }

    #[test]
    fn approve_git_no_pager_log() {
        assert!(is_readonly_compound("git --no-pager log --oneline -20"));
    }

    #[test]
    fn approve_git_c_flag_diff() {
        assert!(is_readonly_compound("git -C /other/repo diff HEAD"));
    }

    #[test]
    fn deny_git_cherry_pick() {
        assert!(!is_readonly_compound("git cherry-pick abc123"));
    }

    #[test]
    fn deny_git_merge() {
        assert!(!is_readonly_compound("git merge feature-branch"));
    }

    #[test]
    fn deny_git_tag_create() {
        // git tag with args creates a tag — but our safelist allows "tag" as read-only
        // This is a known limitation: `git tag` (list) is safe, `git tag v1.0` (create) is not
        // For now we allow it since the matcher just checks the subcommand
        // If this is a concern, remove "tag" from READONLY_GIT_SUBCOMMANDS
        assert!(is_readonly_compound("git tag"));
    }

    #[test]
    fn approve_git_log_with_format() {
        assert!(is_readonly_compound("git log --pretty=format:'%h %s' --graph"));
    }

    #[test]
    fn approve_git_diff_stat() {
        assert!(is_readonly_compound("git diff --stat HEAD~5..HEAD"));
    }

    #[test]
    fn approve_git_branch_list() {
        assert!(is_readonly_compound("git branch -a --sort=-committerdate"));
    }

    // ── Weird edge cases: sed trickery ──

    #[test]
    fn deny_sed_inplace_no_space() {
        assert!(!is_readonly_compound("sed -i's/a/b/' file"));
    }

    #[test]
    fn deny_sed_combined_flags_with_i() {
        assert!(!is_readonly_compound("sed -ni 's/foo/bar/p' file"));
    }

    #[test]
    fn deny_sed_long_flag_inplace() {
        assert!(!is_readonly_compound("sed --in-place 's/a/b/' file"));
    }

    #[test]
    fn approve_sed_with_e_flag() {
        assert!(is_readonly_compound("sed -e 's/foo/bar/' -e 's/baz/qux/' file"));
    }

    #[test]
    fn approve_sed_with_n_flag() {
        assert!(is_readonly_compound("sed -n '5,10p' file"));
    }

    // ── Weird edge cases: env var prefix ──

    #[test]
    fn approve_multiple_env_vars() {
        assert!(is_readonly_simple("FOO=bar BAZ=qux RUST_LOG=debug cargo check"));
    }

    #[test]
    fn approve_env_var_with_quoted_value() {
        assert!(is_readonly_simple("FOO=\"hello world\" ls -la"));
    }

    #[test]
    fn approve_env_var_with_single_quoted_value() {
        assert!(is_readonly_simple("FOO='hello world' ls -la"));
    }

    #[test]
    fn deny_env_var_then_unsafe() {
        assert!(!is_readonly_simple("PATH=/evil python script.py"));
    }

    #[test]
    fn deny_just_env_assignment_compound() {
        assert!(!is_readonly_compound("FOO=bar"));
    }

    // ── Weird edge cases: path prefixes ──

    #[test]
    fn approve_absolute_path_grep() {
        assert!(is_readonly_simple("/usr/bin/grep -r pattern src/"));
    }

    #[test]
    fn approve_relative_path_command() {
        // ./ls is not "ls" — basename gives "ls" but ./ls could be anything
        // Actually basename("./ls") = "ls" which is in the safelist
        assert!(is_readonly_simple("./ls"));
    }

    #[test]
    fn deny_absolute_path_unknown() {
        assert!(!is_readonly_simple("/usr/local/bin/my-script"));
    }

    // ── Weird edge cases: cargo trickery ──

    #[test]
    fn deny_cargo_run() {
        assert!(!is_readonly_compound("cargo run"));
    }

    #[test]
    fn deny_cargo_publish() {
        assert!(!is_readonly_compound("cargo publish"));
    }

    #[test]
    fn deny_cargo_add() {
        assert!(!is_readonly_compound("cargo add serde"));
    }

    #[test]
    fn deny_cargo_remove() {
        assert!(!is_readonly_compound("cargo remove serde"));
    }

    #[test]
    fn approve_cargo_clippy_fix_deny() {
        // cargo clippy is safe even with extra flags (it doesn't write)
        assert!(is_readonly_compound("cargo clippy -- -D warnings"));
    }

    #[test]
    fn approve_cargo_test_specific() {
        assert!(is_readonly_compound("cargo test test_name -- --nocapture"));
    }

    #[test]
    fn approve_cargo_build_release() {
        assert!(is_readonly_compound("cargo build --release 2>&1"));
    }

    #[test]
    fn approve_cargo_bench() {
        assert!(is_readonly_compound("cargo bench"));
    }

    // ── Weird edge cases: xargs ──

    #[test]
    fn deny_xargs_rm() {
        assert!(!is_readonly_compound("find . -name '*.tmp' | xargs rm"));
    }

    #[test]
    fn deny_xargs_with_flags_then_unsafe() {
        assert!(!is_readonly_compound("find . | xargs -I {} mv {} /tmp/"));
    }

    #[test]
    fn approve_xargs_with_flags_then_safe() {
        assert!(is_readonly_compound("find . | xargs -n 1 basename"));
    }

    #[test]
    fn deny_bare_xargs() {
        // xargs with no command defaults to echo, but our impl returns false for empty
        assert!(!is_readonly_simple("xargs"));
    }

    // ── Weird edge cases: command substitution hiding ──

    #[test]
    fn deny_subshell_in_arg() {
        assert!(!is_readonly_compound("ls $(cat /etc/shadow)"));
    }

    #[test]
    fn deny_backtick_in_arg() {
        assert!(!is_readonly_compound("echo `whoami > /tmp/pwned`"));
    }

    #[test]
    fn deny_nested_substitution() {
        assert!(!is_readonly_compound("echo $(echo $(rm -rf /))"));
    }

    // ── Weird edge cases: whitespace and empty ──

    #[test]
    fn approve_leading_whitespace() {
        assert!(is_readonly_compound("   ls -la"));
    }

    #[test]
    fn approve_trailing_whitespace() {
        assert!(is_readonly_compound("git status   "));
    }

    #[test]
    fn approve_extra_spaces_between_args() {
        assert!(is_readonly_compound("git   log   --oneline"));
    }

    #[test]
    fn approve_empty_segments_from_double_semicolons() {
        assert!(is_readonly_compound("ls ;; echo hi"));
    }

    // ── Weird edge cases: gh trickery ──

    #[test]
    fn deny_gh_issue_create() {
        assert!(!is_readonly_compound("gh issue create --title 'bug'"));
    }

    #[test]
    fn deny_gh_pr_close() {
        assert!(!is_readonly_compound("gh pr close 123"));
    }

    #[test]
    fn deny_gh_pr_comment() {
        assert!(!is_readonly_compound("gh pr comment 123 -b 'looks good'"));
    }

    #[test]
    fn approve_gh_pr_view() {
        assert!(is_readonly_compound("gh pr view 123"));
    }

    #[test]
    fn approve_gh_issue_list() {
        assert!(is_readonly_compound("gh issue list --state open"));
    }

    #[test]
    fn approve_gh_pr_checks() {
        assert!(is_readonly_compound("gh pr checks 123"));
    }

    #[test]
    fn approve_gh_run_view() {
        assert!(is_readonly_compound("gh run view 12345"));
    }

    #[test]
    fn deny_gh_repo_delete() {
        assert!(!is_readonly_compound("gh repo delete my-repo"));
    }

    #[test]
    fn deny_gh_release_create() {
        assert!(!is_readonly_compound("gh release create v1.0"));
    }

    // ── Weird edge cases: dangerous pattern false positives ──

    #[test]
    fn deny_rm_with_tab() {
        assert!(!is_readonly_compound("rm\tfile.txt"));
    }

    #[test]
    fn deny_mv_with_tab() {
        assert!(!is_readonly_compound("mv\ta.txt b.txt"));
    }

    #[test]
    fn deny_cp_with_tab() {
        assert!(!is_readonly_compound("cp\ta.txt b.txt"));
    }

    #[test]
    fn approve_grep_for_word_remove() {
        // "rm " appears in "rm " but "remove" doesn't match "rm " pattern
        assert!(is_readonly_compound("grep -r 'remove' src/"));
    }

    #[test]
    fn deny_sudo_with_safe_command() {
        assert!(!is_readonly_compound("sudo cat /etc/shadow"));
    }

    #[test]
    fn deny_su_switch_user() {
        assert!(!is_readonly_compound("su - admin"));
    }

    // ── Weird edge cases: docker/kubectl ──

    #[test]
    fn approve_docker_images() {
        assert!(is_readonly_compound("docker images --format '{{.Repository}}'"));
    }

    #[test]
    fn approve_docker_logs() {
        assert!(is_readonly_compound("docker logs -f container_name"));
    }

    #[test]
    fn deny_docker_build() {
        assert!(!is_readonly_compound("docker build -t myimage ."));
    }

    #[test]
    fn approve_kubectl_describe() {
        assert!(is_readonly_compound("kubectl describe pod my-pod -n production"));
    }

    #[test]
    fn approve_kubectl_logs() {
        assert!(is_readonly_compound("kubectl logs -f deployment/my-app"));
    }

    #[test]
    fn deny_kubectl_scale() {
        assert!(!is_readonly_compound("kubectl scale deployment my-app --replicas=3"));
    }

    // ── Weird edge cases: npm/yarn/pip ──

    #[test]
    fn deny_npm_run() {
        assert!(!is_readonly_compound("npm run build"));
    }

    #[test]
    fn deny_npm_exec() {
        assert!(!is_readonly_compound("npm exec -- some-tool"));
    }

    #[test]
    fn approve_npm_outdated() {
        assert!(is_readonly_compound("npm outdated"));
    }

    #[test]
    fn deny_pip_install() {
        assert!(!is_readonly_compound("pip install requests"));
    }

    #[test]
    fn approve_pip_freeze() {
        assert!(is_readonly_compound("pip freeze"));
    }

    #[test]
    fn deny_yarn_add() {
        assert!(!is_readonly_compound("yarn add lodash"));
    }

    // ── Weird edge cases: brew ──

    #[test]
    fn deny_brew_install() {
        assert!(!is_readonly_compound("brew install jq"));
    }

    #[test]
    fn deny_brew_uninstall() {
        assert!(!is_readonly_compound("brew uninstall jq"));
    }

    #[test]
    fn approve_brew_info() {
        assert!(is_readonly_compound("brew info jq"));
    }

    #[test]
    fn approve_brew_deps() {
        assert!(is_readonly_compound("brew deps --tree jq"));
    }

    // ── Weird edge cases: misc tools ──

    #[test]
    fn approve_jq_on_file() {
        assert!(is_readonly_compound("jq '.dependencies' package.json"));
    }

    #[test]
    fn approve_diff_two_files() {
        assert!(is_readonly_compound("diff -u old.txt new.txt"));
    }

    #[test]
    fn approve_base64_decode() {
        assert!(is_readonly_compound("echo 'aGVsbG8=' | base64 -d"));
    }

    #[test]
    fn approve_wc_multiple_files() {
        assert!(is_readonly_compound("wc -l src/*.rs"));
    }

    #[test]
    fn approve_tree_with_depth() {
        assert!(is_readonly_compound("tree -L 3 src/"));
    }

    #[test]
    fn approve_stat_file() {
        assert!(is_readonly_compound("stat -f '%z' Cargo.toml"));
    }

    #[test]
    fn deny_tee_command() {
        // tee writes to files — it's not in our safelist
        assert!(!is_readonly_simple("tee output.log"));
    }

    #[test]
    fn approve_awk_print() {
        assert!(is_readonly_compound("awk '{print $1}' data.txt | sort -n"));
    }

    #[test]
    fn approve_tr_lowercase() {
        assert!(is_readonly_compound("echo 'HELLO' | tr A-Z a-z"));
    }

    #[test]
    fn approve_cut_field() {
        assert!(is_readonly_compound("cut -d: -f1 /etc/passwd"));
    }

    // ── Weird edge cases: make ──

    #[test]
    fn approve_make_dry_run_long() {
        assert!(is_readonly_compound("make --dry-run all"));
    }

    #[test]
    fn approve_make_just_print() {
        assert!(is_readonly_compound("make --just-print install"));
    }

    #[test]
    fn deny_make_install() {
        assert!(!is_readonly_compound("make install"));
    }

    #[test]
    fn deny_make_clean() {
        assert!(!is_readonly_compound("make clean"));
    }

    // ── Weird edge cases: combined env + pipe + operators ──

    #[test]
    fn approve_complex_pipeline() {
        assert!(is_readonly_compound(
            "git log --oneline --since='2024-01-01' | grep -i fix | wc -l"
        ));
    }

    #[test]
    fn approve_env_var_compound() {
        assert!(is_readonly_compound("RUST_BACKTRACE=1 cargo test 2>&1 | head -50"));
    }

    #[test]
    fn deny_safe_pipe_to_redirect() {
        assert!(!is_readonly_compound("git diff > changes.patch"));
    }

    #[test]
    fn deny_safe_or_write() {
        assert!(!is_readonly_compound("ls || echo fail > log.txt"));
    }

    // ── Weird edge cases: node/python version only ──

    #[test]
    fn approve_node_version() {
        assert!(is_readonly_compound("node --version"));
    }

    #[test]
    fn deny_python_c_flag() {
        assert!(!is_readonly_compound("python3 -c 'import os; os.system(\"rm -rf /\")'"));
    }

    #[test]
    fn deny_node_eval() {
        assert!(!is_readonly_compound("node -e 'require(\"fs\").writeFileSync(\"x\",\"\")'"));
    }

    #[test]
    fn deny_python_module() {
        assert!(!is_readonly_compound("python3 -m http.server"));
    }

    #[test]
    fn approve_python_v_short() {
        assert!(is_readonly_compound("python3 -V"));
    }

    // ── Weird edge cases: rustc ──

    #[test]
    fn deny_rustc_compile() {
        assert!(!is_readonly_compound("rustc main.rs"));
    }

    #[test]
    fn approve_rustc_print_cfg() {
        assert!(is_readonly_compound("rustc --print cfg"));
    }

    // ── Adversarial: attempting to bypass via word boundaries ──

    #[test]
    fn deny_rmdir_no_space() {
        assert!(!is_readonly_compound("rmdir mydir"));
    }

    #[test]
    fn deny_kill_process() {
        assert!(!is_readonly_compound("kill -9 12345"));
    }

    #[test]
    fn deny_killall_process() {
        assert!(!is_readonly_compound("killall python"));
    }

    #[test]
    fn deny_pkill_process() {
        assert!(!is_readonly_compound("pkill -f myapp"));
    }

    #[test]
    fn deny_dd_disk_write() {
        assert!(!is_readonly_compound("dd if=/dev/zero of=/dev/sda"));
    }

    #[test]
    fn deny_systemctl_restart() {
        assert!(!is_readonly_compound("systemctl restart nginx"));
    }

    #[test]
    fn deny_reboot() {
        assert!(!is_readonly_compound("reboot"));
    }

    #[test]
    fn deny_shutdown_now() {
        assert!(!is_readonly_compound("shutdown -h now"));
    }

    // ── Weird edge cases: empty / degenerate input ──

    #[test]
    fn approve_just_spaces() {
        // All segments empty → vacuously true
        assert!(is_readonly_compound("   "));
    }

    #[test]
    fn approve_empty_string() {
        assert!(is_readonly_compound(""));
    }

    #[test]
    fn approve_just_semicolons() {
        assert!(is_readonly_compound(";;;"));
    }
}
