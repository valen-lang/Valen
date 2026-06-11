// Frontend invocation
// Mirrors Coordinator/src/valestrom.vale

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct ProjectDirectoryDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProjectValeInputDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProjectNonValeInputDeclaration {
    pub project_name: String,
    pub path: PathBuf,
}

pub fn invoke_frontend(
    frontend_path: &Path,
    project_directories: &[ProjectDirectoryDeclaration],
    project_vale_inputs: &[ProjectValeInputDeclaration],
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
    if !frontend_path.exists() {
        return Err(format!("Cannot find frontend at: {}", frontend_path.display()));
    }

    let mut command_line_args: Vec<String> = Vec::new();
    command_line_args.push("build".to_string());
    command_line_args.push("--output_dir".to_string());
    command_line_args.push(output_dir.display().to_string());

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

    for declaration in project_directories {
        let resolved_path = declaration.path.canonicalize()
            .unwrap_or_else(|_| declaration.path.clone());
        command_line_args.push(format!("{}={}", declaration.project_name, resolved_path.display()));
    }

    for declaration in project_vale_inputs {
        let resolved_path = declaration.path.canonicalize()
            .unwrap_or_else(|_| declaration.path.clone());
        command_line_args.push(format!("{}={}", declaration.project_name, resolved_path.display()));
    }

    let child = Command::new(frontend_path)
        .args(&command_line_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn frontend process: {}", e))?;

    Ok(child)
}
/*
func invoke_frontend(
  frontend_path &Path,
  project_directories &List<ProjectDirectoryDeclaration>,
  project_vale_inputs &List<ProjectValeInputDeclaration>,
  project_non_vale_inputs &List<ProjectNonValeInputDeclaration>,
  benchmark bool,
  sanity_check bool,
  verbose bool,
  debug_output bool,
  include_builtins bool,
  output_vast bool,
  output_vpst bool,
  output_dir &Path)
Subprocess {
  program = if IsWindows() { "java.exe" } else { "java" };

  //frontend_path = frontend_dir./("Frontend.jar");
  if not frontend_path.exists() {
    panic("Cannot find Frontend.jar at: " + frontend_path.str());
  }

  command_line_args = List<str>();
  command_line_args.add("-cp");
  command_line_args.add(frontend_path.str());
  command_line_args.add("dev.vale.passmanager.PassManager");
  command_line_args.add("build");
  command_line_args.add("--output_dir");
  command_line_args.add(output_dir.str());

  if benchmark {
    command_line_args.add("--benchmark");
  }
  if sanity_check {
    command_line_args.add("--sanity_check");
    command_line_args.add("true");
  }
  if verbose {
    command_line_args.add("--verbose");
  }
  if debug_output {
    command_line_args.add("--debug_output");
  }
  if not include_builtins {
    command_line_args.add("--include_builtins");
    command_line_args.add("false");
  }
  if not output_vast {
    command_line_args.add("--output_vast");
    command_line_args.add("false");
  }
  if not output_vpst {
    command_line_args.add("--output_vpst");
    command_line_args.add("false");
  }

  project_directories.each((declaration) => {
    command_line_args.add(declaration.project_name + "=" + declaration.path.resolve().str());
  });

  project_vale_inputs.each((declaration) => {
    command_line_args.add(declaration.project_name + "=" + declaration.path.resolve().str());
  });

  x = (Subprocess(program, &command_line_args)).expect();
  return x;
}
*/
