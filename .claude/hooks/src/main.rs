use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use regex::Regex;
use rabble::{ClaudeCliRequester, Session, SessionConfig, ConversationRequester, DirectRequester};
use rabble::direct_requester::OpenAiCompatibleFallback;

#[derive(Debug, Deserialize)]
struct HookInput {
    tool_name: String,
    tool_input: ToolInput,
}

#[derive(Debug, Deserialize)]
struct ToolInput {
    file_path: String,
    #[serde(default)]
    old_string: String,
    #[serde(default)]
    content: String,
    #[serde(default)]
    new_string: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct HookOutput {
    #[serde(rename = "hookSpecificOutput")]
    hook_specific_output: HookSpecificOutput,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct HookSpecificOutput {
    #[serde(rename = "hookEventName")]
    hook_event_name: String,
    #[serde(rename = "permissionDecision")]
    permission_decision: String,
    #[serde(rename = "permissionDecisionReason")]
    permission_decision_reason: String,
}

#[derive(Clone)]
struct AppConfig {
    data_file: String,
    check_files: Vec<String>,
    openrouter_api_key: String,
    model: String,
    log_dir: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut data_file: Option<String> = None;
    let mut check_files: Vec<String> = Vec::new();
    let mut port: u16 = 7878;
    let mut model = "claude-sonnet-4-6".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--data-file" => {
                i += 1;
                if i < args.len() {
                    data_file = Some(args[i].clone());
                }
            }
            "--check" => {
                i += 1;
                if i < args.len() {
                    check_files.push(args[i].clone());
                }
            }
            "--port" => {
                i += 1;
                if i < args.len() {
                    port = args[i].parse().expect("Invalid port number");
                }
            }
            "--model" => {
                i += 1;
                if i < args.len() {
                    model = args[i].clone();
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let data_file = data_file.expect("--data-file argument is required");
    if check_files.is_empty() {
        eprintln!("At least one --check argument is required");
        std::process::exit(1);
    }

    if !Path::new(&data_file).exists() {
        eprintln!("Data file not found: {}", data_file);
        std::process::exit(1);
    }

    let openrouter_api_key = fs::read_to_string("/Volumes/V/Rabble/api_key.txt")
        .expect("Failed to read OpenRouter API key at /Volumes/V/Rabble/api_key.txt")
        .trim()
        .to_string();

    let project_root = env::current_dir().expect("Failed to get current directory");
    let log_dir = project_root
        .join("FrontendRust/zen/logs")
        .to_string_lossy()
        .into_owned();

    let config = Arc::new(AppConfig {
        data_file,
        check_files,
        openrouter_api_key,
        model,
        log_dir,
    });

    let app = Router::new()
        .route("/validate", post(validate_handler))
        .with_state(config);

    let addr = format!("127.0.0.1:{}", port);
    eprintln!("writecop server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn validate_handler(
    State(config): State<Arc<AppConfig>>,
    Json(input): Json<HookInput>,
) -> impl IntoResponse {
    eprintln!("DEBUG: Received validation request");

    let cwd = env::current_dir()
        .expect("Failed to get cwd")
        .to_string_lossy()
        .into_owned();
    let conv = ClaudeCliRequester::new(
        cwd,
        config.model.clone(),
        vec!["Read".to_string(), "Grep".to_string(), "Glob".to_string()],
    );
    let direct = OpenAiCompatibleFallback::openrouter(&config.openrouter_api_key);
    let result = validate_hook(&config, input, &conv, &direct).await;

    match result {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(err_response) => (StatusCode::OK, Json(err_response)),
    }
}

fn make_allow(reason: &str) -> HookOutput {
    HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "PreToolUse".to_string(),
            permission_decision: "allow".to_string(),
            permission_decision_reason: reason.to_string(),
        },
    }
}

fn make_deny(reason: String) -> HookOutput {
    HookOutput {
        hook_specific_output: HookSpecificOutput {
            hook_event_name: "PreToolUse".to_string(),
            permission_decision: "deny".to_string(),
            permission_decision_reason: reason,
        },
    }
}

async fn validate_hook(
    config: &AppConfig,
    input: HookInput,
    conv: &dyn ConversationRequester,
    direct: &dyn DirectRequester,
) -> Result<HookOutput, HookOutput> {
    eprintln!("DEBUG: Validating tool_name={}", input.tool_name);
    eprintln!("DEBUG: Validating file_path={}", input.tool_input.file_path);

    // Skip hook infrastructure files
    if input.tool_input.file_path.contains(".claude/hooks/") {
        eprintln!("DEBUG: Skipping hook infrastructure file");
        return Ok(make_allow("Hook infrastructure file"));
    }

    // Only check Rust files in migration
    let rust_file_pattern = Regex::new(r"FrontendRust/src/.*\.rs$").unwrap();
    if !rust_file_pattern.is_match(&input.tool_input.file_path) {
        eprintln!("DEBUG: Skipping non-Rust or non-migration file: {}", input.tool_input.file_path);
        return Ok(make_allow("Not a Rust migration file"));
    }

    eprintln!("DEBUG: File matches, continuing with hook checks");

    // Get edit details
    let (old_string, new_string, context) = if input.tool_name == "Edit" {
        (input.tool_input.old_string, input.tool_input.new_string, "EDIT")
    } else if input.tool_name == "Write" {
        (String::new(), input.tool_input.content, "WRITE")
    } else {
        return Ok(make_allow("Not an Edit or Write operation"));
    };

    // Read current file content
    let file_content = if Path::new(&input.tool_input.file_path).exists() {
        fs::read_to_string(&input.tool_input.file_path)
            .unwrap_or_else(|_| "(could not read file)".to_string())
    } else {
        "(new file)".to_string()
    };

    // Read data template
    let data_template =
        fs::read_to_string(&config.data_file).expect("Failed to read data file");

    // Substitute variables in data template
    let data_substituted = data_template
        .replace("{{file_path}}", &input.tool_input.file_path)
        .replace("{{context}}", context)
        .replace("{{old_string}}", &old_string)
        .replace("{{new_string}}", &new_string)
        .replace("{{file_content}}", &file_content);

    let log_dir = Path::new(&config.log_dir);

    eprintln!("DEBUG: Log dir: {:?}", log_dir);

    // Clear old logs
    if log_dir.exists() {
        for entry in fs::read_dir(log_dir).unwrap() {
            if let Ok(entry) = entry {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    eprintln!("DEBUG: Cleared old logs");

    // One session for all check files (created after early exits)
    let session_config = SessionConfig {
        model: config.model.clone(),
        log_dir: config.log_dir.clone(),
    };
    let mut session = Session::new(conv, direct, &session_config)
        .map_err(|e| make_deny(format!("Session error: {}", e)))?;

    let mut all_denials = Vec::new();

    // Loop through all check files
    for check_file in &config.check_files {
        eprintln!("DEBUG: Processing check file: {}", check_file);

        if !Path::new(check_file).exists() {
            eprintln!("Check file not found: {}", check_file);
            return Err(make_deny(format!("Check file not found: {}", check_file)));
        }

        let check_content =
            fs::read_to_string(check_file).expect("Failed to read check file");

        // Strip frontmatter to get instructions
        let instructions = strip_frontmatter(&check_content);

        // Combine instructions with data
        let prompt = format!("{}\n\n{}", instructions, data_substituted);

        eprintln!("DEBUG: Invoking Rabble Session (Claude CLI backend)...");

        let mut check_session = session
            .fork()
            .map_err(|e| make_deny(format!("Fork error: {}", e)))?;

        let decision_result = check_session.ask_json::<HookOutput>(&prompt).await;
        let (decision, reason) = match decision_result {
            Ok(obj) => (
                obj.hook_specific_output.permission_decision,
                obj.hook_specific_output.permission_decision_reason,
            ),
            Err(e) => ("unknown".into(), format!("Failed to parse response: {}", e)),
        };

        // Write log file
        let instruction_basename = Path::new(check_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        let log_file = log_dir.join(format!("{}.log", instruction_basename));

        eprintln!("DEBUG: Writing log to: {:?}", log_file);

        let log_content = format!(
            "=== Hook Invocation Log ===\n\
             Timestamp: {}\n\
             Model: {}\n\
             Instructions File: {}\n\
             Data File: {}\n\
             File Being Edited: {}\n\
             \n\
             === Prompt Sent to Claude ===\n\
             {}\n\
             \n\
             === Decision ===\n\
             Decision: {}\n\
             Reason: {}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            config.model,
            check_file,
            &config.data_file,
            input.tool_input.file_path,
            prompt,
            decision,
            reason,
        );

        let _ = fs::write(&log_file, log_content);
        eprintln!("DEBUG: Log written to {:?}", log_file);

        if decision == "deny" {
            all_denials.push(format!("[{}] {}", instruction_basename, reason));
        }
    }

    // Aggregate results
    if !all_denials.is_empty() {
        let combined_reason = all_denials.join("\n");
        Err(make_deny(combined_reason))
    } else {
        Ok(make_allow("All checks passed"))
    }
}

fn parse_frontmatter_model(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_frontmatter = false;
    let mut frontmatter_count = 0;

    for line in lines {
        if line.trim() == "---" {
            frontmatter_count += 1;
            if frontmatter_count == 1 {
                in_frontmatter = true;
            } else if frontmatter_count == 2 {
                break;
            }
            continue;
        }

        if in_frontmatter && line.starts_with("model:") {
            return Some(line.split(':').nth(1)?.trim().to_string());
        }
    }

    None
}

fn strip_frontmatter(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut frontmatter_count = 0;

    for line in lines {
        if line.trim() == "---" {
            frontmatter_count += 1;
            continue;
        }

        if frontmatter_count >= 2 {
            result.push(line);
        }
    }

    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    use anyhow::Result;
    use rabble::{ConversationRequester, DirectRequester, LlmLogger, StreamEvent};
    use rabble::testing::MockConvRequester;

    // ── Panic stubs (early-exit tests: LLM is never reached) ─────────────────

    struct PanicDirect;

    impl DirectRequester for PanicDirect {
        fn request(
            &self,
            _model: &str,
            _conversation_log: &[(String, String)],
            _json_schema: Option<&str>,
            _logger: Arc<dyn LlmLogger>,
            _path: &str,
        ) -> Result<String> {
            panic!("PanicDirect: should not be called in this test")
        }
    }

    struct PanicConv;

    impl ConversationRequester for PanicConv {
        fn create_session(&self) -> Result<String> {
            panic!("PanicConv: should not be called in this test")
        }

        fn request_stream(
            &self,
            _message: &str,
            _json_schema: Option<&str>,
            _session_id: Option<&str>,
            _fork: bool,
            _debug_logger: Arc<dyn LlmLogger>,
            _stderr_collector: Arc<Mutex<Vec<String>>>,
            _path: &str,
            _timeout_secs: u64,
        ) -> Result<Box<dyn Iterator<Item = Result<StreamEvent>>>> {
            panic!("PanicConv: should not be called in this test")
        }
    }

    // ── Test helpers ──────────────────────────────────────────────────────────

    fn dummy_config_with_log(log_dir: &str) -> AppConfig {
        AppConfig {
            data_file: "/nonexistent/data.txt".to_string(),
            check_files: vec![],
            openrouter_api_key: String::new(),
            model: "test-model".to_string(),
            log_dir: log_dir.to_string(),
        }
    }

    fn hooks_input() -> HookInput {
        HookInput {
            tool_name: "Edit".to_string(),
            tool_input: ToolInput {
                file_path: "/some/path/.claude/hooks/src/main.rs".to_string(),
                old_string: "old".to_string(),
                new_string: "new".to_string(),
                content: String::new(),
            },
        }
    }

    fn migration_edit_input(file_path: &str) -> HookInput {
        HookInput {
            tool_name: "Edit".to_string(),
            tool_input: ToolInput {
                file_path: file_path.to_string(),
                old_string: "old".to_string(),
                new_string: "new".to_string(),
                content: String::new(),
            },
        }
    }

    fn read_input(file_path: &str) -> HookInput {
        HookInput {
            tool_name: "Read".to_string(),
            tool_input: ToolInput {
                file_path: file_path.to_string(),
                old_string: String::new(),
                new_string: String::new(),
                content: String::new(),
            },
        }
    }

    const ALLOW_JSON: &str = r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow","permissionDecisionReason":"looks good"}}"#;
    const DENY_JSON: &str = r#"{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"test violation"}}"#;

    // ── Pure-function tests ───────────────────────────────────────────────────

    #[test]
    fn test_parse_frontmatter_model_present() {
        let content = "---\nmodel: claude-opus-4\n---\nsome instructions";
        assert_eq!(
            parse_frontmatter_model(content),
            Some("claude-opus-4".to_string())
        );
    }

    #[test]
    fn test_parse_frontmatter_model_missing() {
        let content = "---\ntitle: foo\n---\nsome instructions";
        assert_eq!(parse_frontmatter_model(content), None);
    }

    #[test]
    fn test_parse_frontmatter_model_no_frontmatter() {
        let content = "just plain content";
        assert_eq!(parse_frontmatter_model(content), None);
    }

    #[test]
    fn test_strip_frontmatter_removes_header() {
        let content = "---\nmodel: foo\n---\nactual instructions here";
        assert_eq!(strip_frontmatter(content), "actual instructions here");
    }

    #[test]
    fn test_strip_frontmatter_no_frontmatter() {
        let content = "just plain content\nno frontmatter";
        assert_eq!(strip_frontmatter(content), "");
    }

    #[test]
    fn test_strip_frontmatter_multiline_body() {
        let content = "---\nmodel: foo\n---\nline1\nline2\nline3";
        assert_eq!(strip_frontmatter(content), "line1\nline2\nline3");
    }

    // ── Early-exit tests (no LLM called) ─────────────────────────────────────

    #[tokio::test]
    async fn test_hooks_file_skipped() {
        let config = dummy_config_with_log("/tmp");
        let result = validate_hook(&config, hooks_input(), &PanicConv, &PanicDirect).await;
        let output = result.unwrap();
        assert_eq!(output.hook_specific_output.permission_decision, "allow");
    }

    #[tokio::test]
    async fn test_non_migration_file_skipped() {
        let config = dummy_config_with_log("/tmp");
        let input = migration_edit_input("/some/other/file.rs");
        let result = validate_hook(&config, input, &PanicConv, &PanicDirect).await;
        let output = result.unwrap();
        assert_eq!(output.hook_specific_output.permission_decision, "allow");
        assert_eq!(
            output.hook_specific_output.permission_decision_reason,
            "Not a Rust migration file"
        );
    }

    #[tokio::test]
    async fn test_non_edit_write_tool_skipped() {
        let config = dummy_config_with_log("/tmp");
        let input = read_input("/proj/FrontendRust/src/foo.rs");
        let result = validate_hook(&config, input, &PanicConv, &PanicDirect).await;
        let output = result.unwrap();
        assert_eq!(output.hook_specific_output.permission_decision, "allow");
        assert_eq!(
            output.hook_specific_output.permission_decision_reason,
            "Not an Edit or Write operation"
        );
    }

    // ── LLM-decision tests ────────────────────────────────────────────────────

    /// Test fixture: two separate temp dirs — one for check/data files, one for logs.
    /// The log dir must be separate because validate_hook clears it before processing.
    struct LlmTestFixture {
        _files_dir: tempfile::TempDir,
        _log_dir: tempfile::TempDir,
    }

    impl LlmTestFixture {
        fn new() -> Self {
            LlmTestFixture {
                _files_dir: tempfile::tempdir().unwrap(),
                _log_dir: tempfile::tempdir().unwrap(),
            }
        }

        fn make_config(&self, check_files: Vec<String>, data_file: String) -> AppConfig {
            AppConfig {
                data_file,
                check_files,
                openrouter_api_key: String::new(),
                model: "test-model".to_string(),
                log_dir: self._log_dir.path().to_string_lossy().into_owned(),
            }
        }

        fn write_check_file(&self, name: &str, body: &str) -> String {
            let path = self._files_dir.path().join(name);
            fs::write(&path, body).unwrap();
            path.to_string_lossy().into_owned()
        }

        fn write_data_file(&self) -> String {
            let path = self._files_dir.path().join("data.txt");
            fs::write(
                &path,
                "File: {{file_path}}\nContext: {{context}}\nOld: {{old_string}}\nNew: {{new_string}}",
            )
            .unwrap();
            path.to_string_lossy().into_owned()
        }
    }

    fn migration_edit() -> HookInput {
        migration_edit_input("/proj/FrontendRust/src/postparsing/foo.rs")
    }

    #[tokio::test]
    async fn test_deny_decision_propagated() {
        let fix = LlmTestFixture::new();
        let data_file = fix.write_data_file();
        let check_file = fix.write_check_file("check1.md", "Check: deny bad things");
        let config = fix.make_config(vec![check_file], data_file);

        // 1 check file → create_session("base") + fork_session → "fork1" + request_stream
        let mock = MockConvRequester::new(vec![
            ("", Some("fork1"), false, "fork1", DENY_JSON),
        ])
        .with_session_ids(vec!["base", "fork1"]);

        let result = validate_hook(&config, migration_edit(), &mock, &PanicDirect).await;
        let output = result.unwrap_err();
        assert_eq!(output.hook_specific_output.permission_decision, "deny");
        assert!(output.hook_specific_output.permission_decision_reason.contains("check1"));
    }

    #[tokio::test]
    async fn test_allow_decision_propagated() {
        let fix = LlmTestFixture::new();
        let data_file = fix.write_data_file();
        let check_file = fix.write_check_file("mycheck.md", "Check: allow good things");
        let config = fix.make_config(vec![check_file], data_file);

        let mock = MockConvRequester::new(vec![
            ("", Some("fork1"), false, "fork1", ALLOW_JSON),
        ])
        .with_session_ids(vec!["base", "fork1"]);

        let result = validate_hook(&config, migration_edit(), &mock, &PanicDirect).await;
        let output = result.unwrap();
        assert_eq!(output.hook_specific_output.permission_decision, "allow");
    }

    #[tokio::test]
    async fn test_two_check_files_both_allow() {
        let fix = LlmTestFixture::new();
        let data_file = fix.write_data_file();
        let check1 = fix.write_check_file("check1.md", "Check one");
        let check2 = fix.write_check_file("check2.md", "Check two");
        let config = fix.make_config(vec![check1, check2], data_file);

        // create_session → "base"; fork → "fork1"; fork → "fork2"
        let mock = MockConvRequester::new(vec![
            ("", Some("fork1"), false, "fork1", ALLOW_JSON),
            ("", Some("fork2"), false, "fork2", ALLOW_JSON),
        ])
        .with_session_ids(vec!["base", "fork1", "fork2"]);

        let result = validate_hook(&config, migration_edit(), &mock, &PanicDirect).await;
        let output = result.unwrap();
        assert_eq!(output.hook_specific_output.permission_decision, "allow");
    }

    #[tokio::test]
    async fn test_two_check_files_one_denies() {
        let fix = LlmTestFixture::new();
        let data_file = fix.write_data_file();
        let check1 = fix.write_check_file("alpha.md", "Check alpha");
        let check2 = fix.write_check_file("beta.md", "Check beta");
        let config = fix.make_config(vec![check1, check2], data_file);

        let mock = MockConvRequester::new(vec![
            ("", Some("fork1"), false, "fork1", DENY_JSON),
            ("", Some("fork2"), false, "fork2", ALLOW_JSON),
        ])
        .with_session_ids(vec!["base", "fork1", "fork2"]);

        let result = validate_hook(&config, migration_edit(), &mock, &PanicDirect).await;
        let output = result.unwrap_err();
        assert_eq!(output.hook_specific_output.permission_decision, "deny");
        assert!(output.hook_specific_output.permission_decision_reason.contains("alpha"));
    }
}
