// Frontend (Valestrom) invocation
// Mirrors Coordinator/src/valestrom.vale

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Project directory declaration
/// Mirrors ProjectDirectoryDeclaration in build.vale lines 14-17
#[derive(Debug, Clone)]
pub struct ProjectDirectoryDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

/// Project Vale input declaration  
/// Mirrors ProjectValeInputDeclaration in build.vale lines 19-22
#[derive(Debug, Clone)]
pub struct ProjectValeInputDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

/// Project non-Vale input declaration
/// Mirrors ProjectNonValeInputDeclaration in build.vale lines 24-27
#[derive(Debug, Clone)]
pub struct ProjectNonValeInputDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

/// Invoke the Rust Frontend for parsing only (.vale -> .vpst)
/// This calls FrontendRust which mirrors the Scala parser
pub fn invoke_frontend_rust(
    frontend_rust_path: &Path,
    project_directories: &[ProjectDirectoryDeclaration],
    project_vale_inputs: &[ProjectValeInputDeclaration],
    output_dir: &Path,
) -> Result<(), String> {
    // Ensure output directories exist
    std::fs::create_dir_all(output_dir.join("vpst"))
        .map_err(|e| format!("Failed to create vpst directory: {}", e))?;
    
    // Build command line args for FrontendRust
    let mut command_line_args = Vec::new();
    command_line_args.push("--output_dir".to_string());
    command_line_args.push(output_dir.display().to_string());
    
    // Add all project directory inputs (e.g., stdlib)
    for declaration in project_directories {
        let resolved_path = declaration.path.canonicalize()
            .unwrap_or_else(|_| declaration.path.clone());
        command_line_args.push(format!("{}={}", declaration.project_name, resolved_path.display()));
    }
    
    // Add all project file inputs
    for declaration in project_vale_inputs {
        let resolved_path = declaration.path.canonicalize()
            .unwrap_or_else(|_| declaration.path.clone());
        command_line_args.push(format!("{}={}", declaration.project_name, resolved_path.display()));
    }
    
    // Run FrontendRust and wait for completion
    let status = Command::new(frontend_rust_path)
        .args(&command_line_args)
        .status()
        .map_err(|e| format!("Failed to run FrontendRust: {}", e))?;
    
    if !status.success() {
        return Err(format!("FrontendRust failed with exit code: {:?}", status.code()));
    }
    
    Ok(())
}

/// Invoke the frontend - uses FrontendRust for parsing, then Scala for post-parsing phases
/// This is the main entry point that coordinates both frontends
pub fn invoke_frontend(
    frontend_path: &Path,
    project_directories: &[ProjectDirectoryDeclaration],
    project_vale_inputs: &[ProjectValeInputDeclaration],
    project_non_vale_inputs: &[ProjectNonValeInputDeclaration],
    benchmark: bool,
    sanity_check: bool,
    verbose: bool,
    debug_output: bool,
    include_builtins: bool,
    output_vast: bool,
    output_vpst: bool,
    output_dir: &Path,
) -> Result<std::process::Child, String> {
    // First, use FrontendRust for parsing (.vale -> .vpst)
    // This replaces the Scala parsing pass
    if verbose {
        println!("Running FrontendRust for parsing...");
    }
    
    // Find FrontendRust binary
    // Compute compiler root from frontend_path
    // If frontend_path is /path/to/Sylvan/Frontend/Frontend.jar, compiler root is /path/to/Sylvan
    // If frontend_path is /path/to/Sylvan/CoordinatorRust/target/release/Frontend.jar, compiler root is /path/to/Sylvan
    let compiler_root = frontend_path
        .parent()
        .ok_or("Cannot determine frontend directory")?
        .parent()
        .ok_or("Cannot determine compiler root")?;
    
    // Check if we're in a build directory (target/release)
    let compiler_root = if compiler_root.ends_with("target") {
        compiler_root.parent().ok_or("Cannot determine compiler root from target directory")?
            .parent().ok_or("Cannot determine compiler root")?
    } else {
        compiler_root
    };
    
    let frontend_rust_path = compiler_root
        .join("FrontendRust")
        .join("target")
        .join("release")
        .join("frontend_rust");
    
    if !frontend_rust_path.exists() {
        return Err(format!(
            "FrontendRust binary not found at: {}\nPlease build it with: cd FrontendRust && cargo build --release --bin frontend_rust",
            frontend_rust_path.display()
        ));
    }
    
    // Run FrontendRust to generate .vpst files
    invoke_frontend_rust(&frontend_rust_path, project_directories, project_vale_inputs, output_dir)?;
    
    if verbose {
        println!("FrontendRust parsing complete, running Scala post-parsing phases...");
    }
    
    // Now run Scala frontend for post-parsing phases (typing, simplifying, etc.)
    // It will read the .vpst files and produce .vast files
    invoke_frontend_scala(
        frontend_path,
        project_directories,
        project_vale_inputs,
        project_non_vale_inputs,
        benchmark,
        sanity_check,
        verbose,
        debug_output,
        include_builtins,
        output_vast,
        output_vpst,
        output_dir,
    )
}

/// Invoke the Scala Frontend (Valestrom) pass - used for post-parsing phases
/// Mirrors invoke_frontend in valestrom.vale lines 2-67, but modified to use .vpst input
pub fn invoke_frontend_scala(
    frontend_path: &Path,
    _project_directories: &[ProjectDirectoryDeclaration],
    _project_vale_inputs: &[ProjectValeInputDeclaration],
    _project_non_vale_inputs: &[ProjectNonValeInputDeclaration],
    benchmark: bool,
    sanity_check: bool,
    verbose: bool,
    debug_output: bool,
    include_builtins: bool,
    output_vast: bool,
    output_vpst: bool,
    output_dir: &Path,
) -> Result<std::process::Child, String> {
    // Mirrors valestrom.vale line 16
    let program = if cfg!(windows) { "java.exe" } else { "java" };

    // Mirrors valestrom.vale lines 18-21
    if !frontend_path.exists() {
        return Err(format!("Cannot find Frontend.jar at: {}", frontend_path.display()));
    }

    // Mirrors valestrom.vale lines 23-29
    let mut command_line_args = Vec::new();
    command_line_args.push("-cp".to_string());
    command_line_args.push(frontend_path.display().to_string());
    command_line_args.push("dev.vale.passmanager.PassManager".to_string());
    command_line_args.push("build".to_string());
    command_line_args.push("--output_dir".to_string());
    command_line_args.push(output_dir.display().to_string());

    // Mirrors valestrom.vale lines 31-55
    if benchmark {
        command_line_args.push("--benchmark".to_string());
    }
    if sanity_check {
        command_line_args.push("--sanity_check".to_string());
        command_line_args.push("true".to_string());
    }
    if verbose {
        command_line_args.push("--verbose".to_string());
    }
    if debug_output {
        command_line_args.push("--debug_output".to_string());
    }
    if !include_builtins {
        command_line_args.push("--include_builtins".to_string());
        command_line_args.push("false".to_string());
    }
    if !output_vast {
        command_line_args.push("--output_vast".to_string());
        command_line_args.push("false".to_string());
    }
    if !output_vpst {
        command_line_args.push("--output_vpst".to_string());
        command_line_args.push("false".to_string());
    }

    // MODIFICATION: Instead of passing original inputs, pass the vpst directory
    // The Scala PassManager will load .vpst files from this directory
    command_line_args.push("--input_vpst".to_string());
    command_line_args.push(output_dir.join("vpst").display().to_string());

    // Mirrors valestrom.vale lines 65-66
    let child = Command::new(program)
        .args(&command_line_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn frontend process: {}", e))?;

    Ok(child)
}

