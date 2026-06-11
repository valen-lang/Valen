// Build orchestration logic
// Mirrors Coordinator/src/build.vale

use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;

use crate::clang;
use crate::midas;
use crate::valestrom::{ProjectDirectoryDeclaration, ProjectNonValeInputDeclaration, ProjectValeInputDeclaration};

/// List all .vast files in the output directory
/// Mirrors list_vasts in build.vale lines 29-37
fn list_vasts(output_dir: &Path) -> Vec<PathBuf> {
    let mut vast_files = Vec::new();
    let vast_dir = output_dir.join("vast");
    
    if vast_dir.exists() && vast_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&vast_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name() {
                        if name.to_string_lossy().ends_with(".vast") {
                            vast_files.push(path);
                        }
                    }
                }
            }
        }
    }
    
    vast_files
}

/// Parse a flag value from command-line arguments
/// Helper function for flag parsing
fn get_flag_value(args: &[String], flag: &str, default: &str) -> String {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return args[i + 1].clone();
        }
    }
    default.to_string()
}

/// Check if a boolean flag is present
fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

/// Get boolean flag value
fn get_bool_flag(args: &[String], flag: &str, default: bool) -> bool {
    for i in 0..args.len() {
        if args[i] == flag {
            if i + 1 < args.len() {
                return args[i + 1] == "true";
            }
            return true; // Flag present without value means true
        }
    }
    default
}

/// Get integer flag value
fn get_int_flag(args: &[String], flag: &str, default: i32) -> i32 {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return args[i + 1].parse().unwrap_or(default);
        }
    }
    default
}

/// Get optional string flag value
fn get_optional_flag(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

/// Wait for a child process and print its output
fn print_and_join(mut child: process::Child) -> Result<i32, String> {
    // Print stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }
    
    // Print stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
            }
        }
    }
    
    // Wait for process to complete
    let status = child.wait()
        .map_err(|e| format!("Failed to wait for process: {}", e))?;
    
    Ok(status.code().unwrap_or(-1))
}

/// Main build function
/// Mirrors build_stuff in build.vale lines 39-632
pub fn build_stuff(compiler_dir: &Path, all_args: &[String]) {
    // Mirrors build.vale line 42
    let windows = cfg!(windows);

    // Skip first two args (program name and "build" command)
    let build_args = &all_args[2..];

    // Parse all the flags (mirrors build.vale lines 57-292)
    // In Vale this uses the flagger library, we'll parse manually
    
    // Mirrors build.vale lines 297-304: Frontend path
    let frontend_program_name = if windows { "frontend_rust.exe" } else { "frontend_rust" };
    let frontend_path = if let Some(override_path) = get_optional_flag(build_args, "--frontend_path_override") {
        let path = PathBuf::from(override_path);
        if !path.is_file() {
            eprintln!("Error: --frontend_path_override's value ({}) is not a file.", path.display());
            process::exit(1);
        }
        path
    } else {
        compiler_dir.join(frontend_program_name)
    };

    // Mirrors build.vale lines 306-314: Backend path
    let backend_program_name = if windows { "backend.exe" } else { "backend" };
    let backend_path = if let Some(override_path) = get_optional_flag(build_args, "--backend_path_override") {
        let path = PathBuf::from(override_path);
        if !path.is_file() {
            eprintln!("Error: --backend_path_override's value ({}) is not a file.", path.display());
            process::exit(1);
        }
        path
    } else {
        compiler_dir.join(backend_program_name)
    };

    // Mirrors build.vale lines 316-323: Builtins directory
    let builtins_dir = if let Some(override_path) = get_optional_flag(build_args, "--builtins_dir_override") {
        let path = PathBuf::from(override_path);
        if !path.is_dir() {
            eprintln!("Error: --builtins_dir_override's value ({}) is not a directory.", path.display());
            process::exit(1);
        }
        path
    } else {
        compiler_dir.join("builtins")
    };

    // Mirrors build.vale lines 325-326
    let maybe_clang_path_override = get_optional_flag(build_args, "--clang_override");
    let maybe_libc_path_override = get_optional_flag(build_args, "--libc_override");

    // Mirrors build.vale line 328
    let output_dir = PathBuf::from(get_flag_value(build_args, "--output_dir", "build"));

    // Mirrors build.vale lines 330-342: Parse boolean flags
    let benchmark = get_bool_flag(build_args, "--benchmark", false);
    let verbose = get_bool_flag(build_args, "--verbose", false);
    let debug_output = get_bool_flag(build_args, "--debug_output", false);
    let include_builtins = get_bool_flag(build_args, "--include_builtins", true);
    let output_vast = get_bool_flag(build_args, "--output_vast", true);
    let reuse_vast = get_bool_flag(build_args, "--reuse_vast", false);
    let run_backend = get_bool_flag(build_args, "--run_backend", true);
    let run_clang = get_bool_flag(build_args, "--run_clang", true);
    let sanity_check = get_bool_flag(build_args, "--sanity_check", true);
    let enable_replaying = get_bool_flag(build_args, "--enable_replaying", false);
    let enable_side_calling = get_bool_flag(build_args, "--enable_side_calling", false);
    let output_vpst = get_bool_flag(build_args, "--output_vpst", true);
    let no_std = get_bool_flag(build_args, "--no_std", false);

    // Mirrors build.vale lines 344-366: More flags
    let maybe_region_override = get_optional_flag(build_args, "--region_override");
    let maybe_opt_level = get_optional_flag(build_args, "--opt_level");
    let maybe_cpu = get_optional_flag(build_args, "--cpu");
    let executable_name = get_flag_value(build_args, "-o", "main");
    let flares = get_bool_flag(build_args, "--flares", false);
    let gen_heap = get_bool_flag(build_args, "--gen_heap", false);
    let census = get_bool_flag(build_args, "--census", false);
    let asan = get_bool_flag(build_args, "--asan", false);
    let verify = get_bool_flag(build_args, "--verify", false);
    let debug_symbols = has_flag(build_args, "-g");
    let llvm_ir = get_bool_flag(build_args, "--llvm_ir", false);
    let pic = get_bool_flag(build_args, "--pic", true);
    let opt_level = get_flag_value(build_args, "--opt_level", "O0");
    let pie = get_bool_flag(build_args, "--pie", true);
    let asm = get_bool_flag(build_args, "--asm", true);
    let replay_whitelist_extern = get_flag_value(build_args, "--replay_whitelist_extern", "");
    let print_mem_overhead = get_bool_flag(build_args, "--print_mem_overhead", false);
    let elide_checks_for_known_live = get_bool_flag(build_args, "--elide_checks_for_known_live", true);
    let elide_checks_for_regions = get_bool_flag(build_args, "--elide_checks_for_regions", true);
    let gen_size = get_int_flag(build_args, "--gen_size", 32);
    let use_atomic_rc = get_bool_flag(build_args, "--use_atomic_rc", false);
    let include_bounds_checks = get_bool_flag(build_args, "--include_bounds_checks", true);
    let force_all_known_live = get_bool_flag(build_args, "--force_all_known_live", false);

    // Mirrors build.vale lines 368-370
    if verbose {
        println!("Parsing command line inputs...");
    }

    // Mirrors build.vale lines 372-380: Initialize project declarations
    let mut project_directory_declarations = Vec::new();
    let mut project_vale_input_declarations = Vec::new();
    let mut project_non_vale_input_declarations = Vec::new();

    if !no_std {
        project_directory_declarations.push(ProjectDirectoryDeclaration {
            project_name: "stdlib".to_string(),
            path: compiler_dir.join("stdlib").join("src"),
        });
    }

    // Mirrors build.vale lines 383-407: Parse unrecognized inputs (project declarations)
    let mut i = 0;
    while i < build_args.len() {
        let arg = &build_args[i];
        
        // Skip recognized flags
        if arg.starts_with("--") || arg == "-g" || arg == "-o" {
            // Skip flag and its value (if it has one)
            if arg == "-o" || arg == "--output_dir" || arg == "--frontend_path_override" 
                || arg == "--backend_path_override" || arg == "--builtins_dir_override"
                || arg == "--clang_override" || arg == "--libc_override"
                || arg == "--region_override" || arg == "--opt_level" || arg == "--cpu"
                || arg == "--replay_whitelist_extern" || arg == "--gen_size" {
                i += 2; // Skip flag and value
                continue;
            } else if arg == "--benchmark" || arg == "--sanity_check" || arg == "--verbose"
                || arg == "--debug_output" || arg == "--include_builtins" || arg == "--output_vast"
                || arg == "--reuse_vast" || arg == "--run_backend" || arg == "--run_clang"
                || arg == "--enable_replaying" || arg == "--enable_side_calling" || arg == "--output_vpst"
                || arg == "--no_std" || arg == "--flares" || arg == "--gen_heap" || arg == "--census"
                || arg == "--asan" || arg == "--verify" || arg == "--llvm_ir" || arg == "--pic"
                || arg == "--pie" || arg == "--asm" || arg == "--print_mem_overhead"
                || arg == "--elide_checks_for_known_live" || arg == "--elide_checks_for_regions"
                || arg == "--use_atomic_rc" || arg == "--include_bounds_checks" || arg == "--force_all_known_live" {
                // Boolean flags - check if next arg is a value (true/false) or another flag
                if i + 1 < build_args.len() && (build_args[i + 1] == "true" || build_args[i + 1] == "false") {
                    i += 2; // Skip flag and value
                } else {
                    i += 1; // Skip flag only
                }
                continue;
            } else {
                i += 1; // Skip flag only
                continue;
            }
        }

        // Check for project=path format
        if let Some((project_name, path_str)) = arg.split_once('=') {
            let path = PathBuf::from(path_str);
            let resolved_path = path.canonicalize().unwrap_or(path.clone());
            
            if resolved_path.is_dir() {
                project_directory_declarations.push(ProjectDirectoryDeclaration {
                    project_name: project_name.to_string(),
                    path: resolved_path,
                });
            } else if resolved_path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.ends_with(".vale") || n.ends_with(".vpst"))
                .unwrap_or(false) {
                project_vale_input_declarations.push(ProjectValeInputDeclaration {
                    project_name: project_name.to_string(),
                    path: resolved_path,
                });
            } else {
                project_non_vale_input_declarations.push(ProjectNonValeInputDeclaration {
                    project_name: project_name.to_string(),
                    path: resolved_path,
                });
            }
        } else if !arg.starts_with("-") {
            eprintln!("Unrecognized input: {}", arg);
            process::exit(1);
        }
        
        i += 1;
    }

    // Mirrors build.vale lines 409-417: Validate paths
    if !frontend_path.exists() {
        eprintln!("Cannot find Frontend directory: {}", frontend_path.display());
        process::exit(1);
    }
    if !backend_path.exists() {
        eprintln!("Cannot find Backend directory: {}", backend_path.display());
        process::exit(1);
    }
    if !builtins_dir.exists() {
        eprintln!("Cannot find builtins directory: {}", builtins_dir.display());
        process::exit(1);
    }

    // Mirrors build.vale lines 419-453: Run frontend
    if verbose {
        println!("Invoking Frontend...");
    }

    if reuse_vast {
        if list_vasts(&output_dir).is_empty() {
            eprintln!("Error: --reuse_vast specified, but no .vast files found in {}.", output_dir.display());
            process::exit(1);
        }
    } else {
        if output_dir.exists() {
            println!("Deleting existing {}.", output_dir.display());
            if let Err(e) = fs::remove_dir_all(&output_dir) {
                eprintln!("Error removing old dir: {}", e);
                process::exit(1);
            }
            assert!(!output_dir.exists(), "Removing old dir {} failed!", output_dir.display());
        }
        
        if let Err(e) = fs::create_dir_all(&output_dir) {
            eprintln!("Error creating output directory: {}", e);
            process::exit(1);
        }

        let frontend_process = match crate::valestrom::invoke_frontend(
            &frontend_path,
            &project_directory_declarations,
            &project_vale_input_declarations,
            &project_non_vale_input_declarations,
            benchmark,
            sanity_check,
            verbose,
            debug_output,
            include_builtins,
            output_vast,
            output_vpst,
            &output_dir,
        ) {
            Ok(process) => process,
            Err(e) => {
                eprintln!("Error invoking frontend: {}", e);
                process::exit(1);
            }
        };

        println!("Running frontend...");
        let frontend_return_code = match print_and_join(frontend_process) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error waiting for frontend: {}", e);
                process::exit(1);
            }
        };
        
        if frontend_return_code != 0 {
            eprintln!("Frontend returned error code {}, aborting.", frontend_return_code);
            process::exit(frontend_return_code);
        }
    }

    // Mirrors build.vale lines 480-483: Check if should run backend
    if !run_backend {
        println!("Not running backend, stopping here.");
        return;
    }

    // Mirrors build.vale lines 485-522: Run backend
    if verbose {
        println!("Invoking Backend...");
    }

    let vast_files = list_vasts(&output_dir);

    let backend_process = match midas::invoke_backend(
        &backend_path,
        &vast_files,
        &output_dir,
        maybe_region_override.as_deref(),
        maybe_opt_level.as_deref(),
        maybe_cpu.as_deref(),
        &executable_name,
        flares,
        gen_heap,
        census,
        verify,
        &opt_level,
        llvm_ir,
        asm,
        enable_replaying,
        &replay_whitelist_extern,
        enable_side_calling,
        pic,
        print_mem_overhead,
        elide_checks_for_known_live,
        elide_checks_for_regions,
        use_atomic_rc,
        gen_size,
        force_all_known_live,
        include_bounds_checks,
    ) {
        Ok(process) => process,
        Err(e) => {
            eprintln!("Error invoking backend: {}", e);
            process::exit(1);
        }
    };

    println!("Running backend...");
    let backend_return_code = match print_and_join(backend_process) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error waiting for backend: {}", e);
            process::exit(1);
        }
    };
    
    if backend_return_code != 0 {
        eprintln!("Backend returned error code {}, aborting.", backend_return_code);
        process::exit(backend_return_code);
    }

    // Mirrors build.vale lines 524-527: Check if should run clang
    if !run_clang {
        println!("Not running clang, stopping here.");
        return;
    }

    // Mirrors build.vale lines 529-550: Collect clang inputs
    if verbose {
        println!("Collecting cc inputs...");
    }

    let mut clang_inputs = Vec::new();
    if windows {
        clang_inputs.push(output_dir.join("build.obj"));
    } else {
        clang_inputs.push(output_dir.join("build.o"));
    }

    // Add .c files from output directory
    if let Ok(entries) = fs::read_dir(&output_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().ends_with(".c") {
                        clang_inputs.push(path);
                    }
                }
            }
        }
    }

    // Add .c files from builtins directory
    if let Ok(entries) = fs::read_dir(&builtins_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().ends_with(".c") {
                        clang_inputs.push(path);
                    }
                }
            }
        }
    }

    // Mirrors build.vale lines 552-606: Collect native .c files from projects
    let mut project_names = HashSet::new();

    for vast_file in &vast_files {
        if let Some(file_name) = vast_file.file_name().and_then(|n| n.to_str()) {
            let package_coord_str = file_name.strip_suffix(".vast").unwrap_or(file_name);
            let package_coord_parts: Vec<&str> = package_coord_str.split('.').collect();
            
            if !package_coord_parts.is_empty() {
                let project_name = package_coord_parts[0];
                project_names.insert(project_name.to_string());

                // Find native files in project directories
                for declaration in &project_directory_declarations {
                    if declaration.project_name == project_name {
                        let mut package_native_dir = declaration.path.clone();
                        
                        // Navigate through package steps
                        for package_step in &package_coord_parts[1..] {
                            package_native_dir = package_native_dir.join(package_step);
                        }
                        
                        let possible_native_dir = package_native_dir.join("native");
                        if possible_native_dir.exists() && possible_native_dir.is_dir() {
                            if let Ok(entries) = fs::read_dir(&possible_native_dir) {
                                for entry in entries.flatten() {
                                    let path = entry.path();
                                    if path.is_file() {
                                        if let Some(name) = path.file_name() {
                                            if name.to_string_lossy().ends_with(".c") {
                                                clang_inputs.push(path);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let abi_dir = output_dir.join("abi");

    // Collect ABI-generated .c files
    for project_name in &project_names {
        let possible_generated_dir = abi_dir.join(project_name);
        if possible_generated_dir.exists() {
            if !possible_generated_dir.is_dir() {
                eprintln!("Generated dir is not a directory: {}", possible_generated_dir.display());
                process::exit(1);
            }

            if let Ok(entries) = fs::read_dir(&possible_generated_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name() {
                            if name.to_string_lossy().ends_with(".c") {
                                clang_inputs.push(path);
                            }
                        }
                    }
                }
            }
        }

        // Add non-Vale inputs for this project
        for declaration in &project_non_vale_input_declarations {
            if declaration.project_name == *project_name {
                clang_inputs.push(declaration.path.clone());
            }
        }
    }

    // Mirrors build.vale lines 608-631: Invoke clang
    if verbose {
        println!("Invoking cc...");
    }

    let clang_process = match clang::invoke_clang(
        windows,
        maybe_clang_path_override.as_deref(),
        maybe_libc_path_override.as_deref(),
        &clang_inputs,
        &executable_name,
        asan,
        debug_symbols,
        &output_dir,
        pic,
        pie,
    ) {
        Ok(process) => process,
        Err(e) => {
            eprintln!("Error invoking clang: {}", e);
            process::exit(1);
        }
    };

    println!("Running clang...");
    let clang_return_code = match print_and_join(clang_process) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error waiting for clang: {}", e);
            process::exit(1);
        }
    };
    
    if clang_return_code != 0 {
        eprintln!("clang returned error code {}, aborting.", clang_return_code);
        process::exit(clang_return_code);
    }
    
    if verbose {
        println!("Done!");
    }
}

