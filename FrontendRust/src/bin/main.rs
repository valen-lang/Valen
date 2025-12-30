// CLI for FrontendRust - Vale parser
// Produces VPST (Vale Parsed Syntax Tree) files from Vale source
// Mirrors LexAndExplore.scala import-driven parsing logic

use frontend_rust::interner::Interner;
use frontend_rust::keywords::Keywords;
use frontend_rust::lexing::lex_and_explore::lex_and_explore;
use frontend_rust::utils::code_hierarchy::PackageCoordinate;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} --output_dir <dir> [project_name=path ...]", args[0]);
        process::exit(1);
    }

    let mut output_dir = None;
    let mut inputs = Vec::new();
    
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--output_dir" && i + 1 < args.len() {
            output_dir = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else if let Some((project, path_str)) = args[i].split_once('=') {
            inputs.push((project.to_string(), PathBuf::from(path_str)));
            i += 1;
        } else {
            i += 1;
        }
    }

    let output_dir = output_dir.expect("--output_dir required");
    
    // Create output directories
    let vpst_dir = output_dir.join("vpst");
    fs::create_dir_all(&vpst_dir).expect("Failed to create vpst directory");

    println!("FrontendRust: Parsing Vale source files...");
    
    // Initialize interner and keywords
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);

    // From PassManager.scala lines 136-143: Build inputs
    // DirectFilePathInput for files, ModulePathInput for directories
    let mut module_roots: HashMap<String, PathBuf> = HashMap::new();
    let mut direct_file_inputs: HashMap<Arc<PackageCoordinate>, PathBuf> = HashMap::new();
    let mut initial_packages: Vec<Arc<PackageCoordinate>> = Vec::new();
    
    for (project_name, input_path) in inputs {
        let module_str = {
            // Interner now has interior mutability
            interner.intern(&project_name)
        };
        
        if input_path.is_dir() {
            // From PassManager.scala line 142: ModulePathInput
            module_roots.insert(project_name.clone(), input_path);
        } else if input_path.extension().and_then(|s| s.to_str()) == Some("vale") {
            // From PassManager.scala line 137: DirectFilePathInput
            let package_coord = interner.intern_package_coordinate(PackageCoordinate {
                module: module_str.clone(),
                packages: vec![],
            });
            direct_file_inputs.insert(package_coord.clone(), input_path.clone());
            initial_packages.push(package_coord);
            
            // Also register parent directory as module root for resolving imports
            if let Some(parent) = input_path.parent() {
                module_roots.entry(project_name).or_insert_with(|| parent.to_path_buf());
            }
        }
    }
    
    // From PassManager.scala line 238: Always add PackageCoordinate.BUILTIN to packages
    // From CodeHierarchy.scala line 50: BUILTIN is the root package (empty module, empty packages)
    // This ensures builtin files (str.vale, print.vale, etc.) are always parsed
    if module_roots.contains_key("stdlib") {
        let stdlib_root_package = interner.intern_package_coordinate(PackageCoordinate {
            module: interner.intern("stdlib"),
            packages: vec![],
        });
        initial_packages.push(stdlib_root_package);
    }

    // From LexAndExplore.scala lines 58-146: Import-driven parsing loop
    lex_and_explore(
        interner,
        keywords,
        initial_packages,
        module_roots,
        direct_file_inputs,
        &vpst_dir,
    );

    println!("FrontendRust: Parsing complete!");
}
