use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use regex::Regex;

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

#[derive(Debug, Serialize, Deserialize)]
struct HookOutput {
    #[serde(rename = "hookSpecificOutput")]
    hook_specific_output: HookSpecificOutput,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut data_file: Option<String> = None;
    let mut check_files: Vec<String> = Vec::new();
    let mut port: u16 = 7878;

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

    let config = Arc::new(AppConfig {
        data_file,
        check_files,
        openrouter_api_key,
    });

    // Build the router
    let app = Router::new()
        .route("/validate", post(validate_handler))
        .with_state(config);

    // Start the server
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

    let result = validate_hook(&config, input);

    match result {
        Ok(response) => (StatusCode::OK, Json(response)),
        Err(err_response) => (StatusCode::OK, Json(err_response)),
    }
}

fn validate_hook(config: &AppConfig, input: HookInput) -> Result<HookOutput, HookOutput> {

    eprintln!("DEBUG: Validating tool_name={}", input.tool_name);
    eprintln!("DEBUG: Validating file_path={}", input.tool_input.file_path);

    // Skip hook infrastructure files
    if input.tool_input.file_path.contains(".claude/hooks/") {
        eprintln!("DEBUG: Skipping hook infrastructure file");
        return Ok(HookOutput {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
                permission_decision_reason: "Hook infrastructure file".to_string(),
            },
        });
    }

    // Only check Rust files in migration
    let rust_file_pattern = Regex::new(r"FrontendRust/src/.*\.rs$").unwrap();
    if !rust_file_pattern.is_match(&input.tool_input.file_path) {
        eprintln!("DEBUG: Skipping non-Rust or non-migration file: {}", input.tool_input.file_path);
        return Ok(HookOutput {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
                permission_decision_reason: "Not a Rust migration file".to_string(),
            },
        });
    }

    eprintln!("DEBUG: File matches, continuing with hook checks");

    // Get edit details
    let (old_string, new_string, context) = if input.tool_name == "Edit" {
        (input.tool_input.old_string, input.tool_input.new_string, "EDIT")
    } else if input.tool_name == "Write" {
        (String::new(), input.tool_input.content, "WRITE")
    } else {
        return Ok(HookOutput {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
                permission_decision_reason: "Not an Edit or Write operation".to_string(),
            },
        });
    };

    // Read current file content
    let file_content = if Path::new(&input.tool_input.file_path).exists() {
        fs::read_to_string(&input.tool_input.file_path).unwrap_or_else(|_| "(could not read file)".to_string())
    } else {
        "(new file)".to_string()
    };

    // Read data template
    let data_template = fs::read_to_string(&config.data_file).expect("Failed to read data file");

    // Substitute variables in data template
    let data_substituted = data_template
        .replace("{{file_path}}", &input.tool_input.file_path)
        .replace("{{context}}", context)
        .replace("{{old_string}}", &old_string)
        .replace("{{new_string}}", &new_string)
        .replace("{{file_content}}", &file_content);

    // Get project root - assume we're running from project root via cargo run
    let project_root = env::current_dir().expect("Failed to get current directory");
    let log_dir = project_root.join("FrontendRust/zen/logs");

    eprintln!("DEBUG: Project root: {:?}", project_root);
    eprintln!("DEBUG: Log dir: {:?}", log_dir);

    // Clear old logs
    if log_dir.exists() {
        for entry in fs::read_dir(&log_dir).unwrap() {
            if let Ok(entry) = entry {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    eprintln!("DEBUG: Cleared old logs");

    let mut all_denials = Vec::new();
    let mut all_results = Vec::new();

    // Loop through all check files
    for check_file in &config.check_files {
        eprintln!("DEBUG: Processing check file: {}", check_file);

        if !Path::new(check_file).exists() {
            eprintln!("Check file not found: {}", check_file);
            return Err(HookOutput {
                hook_specific_output: HookSpecificOutput {
                    hook_event_name: "PreToolUse".to_string(),
                    permission_decision: "deny".to_string(),
                    permission_decision_reason: format!("Check file not found: {}", check_file),
                },
            });
        }

        // Parse frontmatter to get model
        let check_content = fs::read_to_string(check_file).expect("Failed to read check file");
        let model = parse_frontmatter_model(&check_content).unwrap_or_else(|| "sonnet".to_string());

        eprintln!("DEBUG: Using model: {}", model);

        // Strip frontmatter
        let instructions = strip_frontmatter(&check_content);

        // Combine instructions with data
        let prompt = format!("{}\n\n{}", instructions, data_substituted);

        eprintln!("DEBUG: Invoking claude CLI...");

        // Invoke Claude CLI - pass prompt via stdin to avoid --allowedTools consuming it
        // Use a thread for stdin to avoid pipe deadlock (prompt may be large)
        eprintln!("DEBUG: Spawning claude CLI process, prompt length: {}", prompt.len());
        let mut child = Command::new("claude")
            .arg("-p")
            .arg("--model")
            .arg(&model)
            // .arg("--json-schema")
            // .arg(r#"{"type":"object","properties":{"hookSpecificOutput":{"type":"object","properties":{"hookEventName":{"type":"string","enum":["PreToolUse"]},"permissionDecision":{"type":"string","enum":["allow","deny","ask"]},"permissionDecisionReason":{"type":"string"}},"required":["hookEventName","permissionDecision","permissionDecisionReason"],"additionalProperties":false}},"required":["hookSpecificOutput"],"additionalProperties":false}"#)
            .arg("--no-session-persistence")
            .arg("--allowedTools")
            .arg("Read")
            .arg("Grep")
            .arg("Glob")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn claude CLI");

        eprintln!("DEBUG: Claude process spawned (pid {:?}), writing stdin on thread", child.id());

        // Write stdin on a separate thread to avoid deadlock with large prompts
        let stdin_handle = child.stdin.take();
        let prompt_bytes = prompt.as_bytes().to_vec();
        let stdin_thread = std::thread::spawn(move || {
            use std::io::Write;
            if let Some(mut stdin) = stdin_handle {
                eprintln!("DEBUG: stdin thread: writing {} bytes", prompt_bytes.len());
                let result = stdin.write_all(&prompt_bytes);
                eprintln!("DEBUG: stdin thread: write result: {:?}", result);
                // stdin closes when dropped here
            }
            eprintln!("DEBUG: stdin thread: done");
        });

        eprintln!("DEBUG: Waiting for claude output...");
        let output = child.wait_with_output().expect("Failed to wait for claude CLI");
        eprintln!("DEBUG: Claude exited with status: {:?}, stdout len: {}, stderr len: {}", output.status, output.stdout.len(), output.stderr.len());
        eprintln!("DEBUG: stdout raw bytes: {:?}", &output.stdout);
        eprintln!("DEBUG: stderr raw bytes: {:?}", &output.stderr);
        let _ = stdin_thread.join();

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        eprintln!("DEBUG: Claude CLI returned, result length: {}", result.len());

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
             Working Directory: {:?}\n\
             Project Root: {:?}\n\
             \n\
             === Prompt Sent to Claude ===\n\
             {}\n\
             \n\
             === Response from Claude ===\n\
             {}\n\
             \n\
             === Stderr from Claude ===\n\
             {}\n\
             \n\
             === Decision ===\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            model,
            check_file,
            &config.data_file,
            input.tool_input.file_path,
            env::current_dir().unwrap_or_default(),
            project_root,
            prompt,
            result,
            stderr
        );

        // Parse decision — 3-step cascade: raw, strip fences, OpenRouter
        let json_result = if serde_json::from_str::<HookOutput>(&result).is_ok() {
            result.clone()
        } else {
            let stripped = strip_code_fences(&result).to_string();
            if serde_json::from_str::<HookOutput>(&stripped).is_ok() {
                eprintln!("DEBUG: Parsed after stripping code fences");
                stripped
            } else {
                eprintln!("DEBUG: Sending to OpenRouter for JSON extraction...");
                extract_json_via_openrouter(&config.openrouter_api_key, &result)
                    .unwrap_or_else(|e| {
                        eprintln!("DEBUG: OpenRouter extraction failed: {}", e);
                        result.clone()
                    })
            }
        };
        let decision_obj: Result<HookOutput, _> = serde_json::from_str(&json_result);
        let (decision, reason) = if let Ok(obj) = decision_obj {
            (
                obj.hook_specific_output.permission_decision.clone(),
                obj.hook_specific_output.permission_decision_reason.clone(),
            )
        } else {
            ("unknown".to_string(), "Failed to parse response".to_string())
        };

        let final_log = format!("{}Decision: {}\nReason: {}\n", log_content, decision, reason);

        fs::write(&log_file, final_log).expect("Failed to write log file");
        eprintln!("DEBUG: Log written to {:?}", log_file);

        // Collect result
        all_results.push(result.clone());

        // Categorize decision
        if decision == "deny" {
            all_denials.push(format!("[{}] {}", instruction_basename, reason));
        }
    }

    // Aggregate results
    if !all_denials.is_empty() {
        let combined_reason = all_denials.join("\n");
        Err(HookOutput {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "deny".to_string(),
                permission_decision_reason: combined_reason,
            },
        })
    } else {
        Ok(HookOutput {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
                permission_decision_reason: "All checks passed".to_string(),
            },
        })
    }
}

fn strip_code_fences(s: &str) -> &str {
    let s = s.trim();
    let s = s.strip_prefix("```json").or_else(|| s.strip_prefix("```")).unwrap_or(s);
    let s = s.strip_suffix("```").unwrap_or(s);
    s.trim()
}

fn extract_json_via_openrouter(api_key: &str, claude_response: &str) -> Result<String, String> {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "hookSpecificOutput": {
                "type": "object",
                "properties": {
                    "hookEventName": { "type": "string" },
                    "permissionDecision": { "type": "string", "enum": ["allow", "deny", "ask"] },
                    "permissionDecisionReason": { "type": "string" }
                },
                "required": ["hookEventName", "permissionDecision", "permissionDecisionReason"],
                "additionalProperties": false
            }
        },
        "required": ["hookSpecificOutput"],
        "additionalProperties": false
    });

    let messages = vec![
        serde_json::json!({
            "role": "user",
            "content": format!(
                "The following text contains a hook permission decision. It may already be valid JSON, or it may be JSON wrapped in markdown code fences, or plain text describing the decision. Your job is to return it in the required JSON schema format. **Do not re-evaluate the decision** — if the text already contains a clear allow/deny/ask decision, preserve it exactly. If the input is already conformant JSON, you may return it as-is.\n\n{}",
                claude_response
            )
        })
    ];

    let body = serde_json::json!({
        "model": "openai/gpt-oss-20b",
        "messages": messages,
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "hook_response",
                "strict": true,
                "schema": schema
            }
        }
    });

    eprintln!("DEBUG: Sending to OpenRouter for JSON extraction...");
    let api_key_owned = api_key.to_string();
    let body_str = body.to_string();
    let resp_json: serde_json::Value = std::thread::spawn(move || -> Result<serde_json::Value, String> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key_owned))
            .header("Content-Type", "application/json")
            .body(body_str)
            .send()
            .map_err(|e| format!("OpenRouter request failed: {}", e))?;
        resp.json::<serde_json::Value>()
            .map_err(|e| format!("Failed to parse OpenRouter response: {}", e))
    })
    .join()
    .map_err(|_| "OpenRouter thread panicked".to_string())?
    .map_err(|e| e)?;

    eprintln!("DEBUG: OpenRouter response: {:?}", resp_json);

    resp_json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Missing content in OpenRouter response: {:?}", resp_json))
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
